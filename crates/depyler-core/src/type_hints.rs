use crate::error_reporting::EnhancedError;
use crate::hir::{HirExpr, HirFunction, HirStmt, Type};
use anyhow::Result;
use colored::Colorize;
use std::collections::HashMap;

/// Type inference hints provider
pub struct TypeHintProvider {
    /// Variable type annotations discovered
    variable_types: HashMap<String, TypeHint>,
    /// Function parameter hints
    parameter_hints: HashMap<String, Vec<TypeHint>>,
    /// Return type hints
    return_hints: HashMap<String, TypeHint>,
    /// Active inference context
    context: InferenceContext,
}

#[derive(Debug, Clone)]
pub struct TypeHint {
    pub suggested_type: Type,
    pub confidence: Confidence,
    pub reason: String,
    pub source_location: Option<(usize, usize)>,
    pub target: HintTarget,
}

#[derive(Debug, Clone)]
pub enum HintTarget {
    Parameter(String),
    Return,
    Variable(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    Low,
    Medium,
    High,
    Certain,
}

#[derive(Debug, Default)]
struct InferenceContext {
    /// Current function being analyzed
    current_function: Option<String>,
    /// Type constraints collected
    constraints: Vec<TypeConstraint>,
    /// Usage patterns
    usage_patterns: HashMap<String, Vec<UsagePattern>>,
    /// Loop variable sources (DEPYLER-0451 Phase 1c)
    /// Maps loop variable → iterable variable (e.g., "item" → "items")
    loop_var_sources: HashMap<String, String>,
    /// DEPYLER-0531: Variables assigned from parameters, indexing, or dict operations
    /// These should NOT default to List<String> even if they have Container pattern
    non_list_variables: std::collections::HashSet<String>,
}

#[derive(Debug, Clone)]
enum TypeConstraint {
    /// Variable must be compatible with type
    Compatible { var: String, ty: Type },
    /// Variable used in operation requiring specific type
    #[allow(dead_code)]
    OperatorConstraint {
        var: String,
        op: String,
        required: Type,
    },
    /// Variable passed to function expecting type (DEPYLER-0492: stdlib signatures)
    ArgumentConstraint {
        var: String,
        func: String,
        /// Parameter index (reserved for future multi-param signature matching)
        _param_idx: usize,
        expected: Type,
    },
    /// Variable returned from function
    ReturnConstraint { var: String, ty: Type },
}

#[derive(Debug, Clone)]
enum UsagePattern {
    /// Used as iterator
    Iterator,
    /// Used with numeric operators
    Numeric,
    /// Used with string methods
    StringLike,
    /// Used as container (list-like, integer indexing)
    Container,
    /// DEPYLER-0552: Used as dictionary (string-keyed access)
    DictAccess,
    /// Used as callable
    #[allow(dead_code)]
    Callable,
}

impl Default for TypeHintProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeHintProvider {
    pub fn new() -> Self {
        Self {
            variable_types: HashMap::new(),
            parameter_hints: HashMap::new(),
            return_hints: HashMap::new(),
            context: InferenceContext::default(),
        }
    }

    /// Analyze a function and generate type hints
    pub fn analyze_function(&mut self, func: &HirFunction) -> Result<Vec<TypeHint>> {
        self.initialize_context(func);
        self.analyze_body(&func.body)?;

        let mut hints = Vec::new();
        self.collect_parameter_hints(func, &mut hints);
        self.collect_return_hint(func, &mut hints);
        self.collect_variable_hints(&mut hints);

        Ok(hints)
    }

    fn initialize_context(&mut self, func: &HirFunction) {
        self.context.current_function = Some(func.name.clone());
        self.context.constraints.clear();
        self.context.usage_patterns.clear();
        self.context.loop_var_sources.clear();

        // DEPYLER-0265: Add explicit parameter types as constraints
        // This allows infer_var_type to find parameter types when analyzing
        // assignments like `longest = words[0]` where `words: list[str]`
        for param in &func.params {
            if !matches!(param.ty, Type::Unknown) {
                self.context.constraints.push(TypeConstraint::Compatible {
                    var: param.name.clone(),
                    ty: param.ty.clone(),
                });
            }
        }
    }

    fn collect_parameter_hints(&mut self, func: &HirFunction, hints: &mut Vec<TypeHint>) {
        for param in &func.params {
            if matches!(param.ty, Type::Unknown) {
                // DEPYLER-0492: Infer type from default value first (highest confidence)
                if let Some(hint) = self.infer_from_default(&param.name, &param.default) {
                    hints.push(hint.clone());
                    self.parameter_hints
                        .entry(func.name.clone())
                        .or_default()
                        .push(hint);
                } else if let Some(hint) = self.infer_parameter_type(&param.name) {
                    hints.push(hint.clone());
                    self.parameter_hints
                        .entry(func.name.clone())
                        .or_default()
                        .push(hint);
                }
            }
        }
    }

    fn collect_return_hint(&mut self, func: &HirFunction, hints: &mut Vec<TypeHint>) {
        if matches!(func.ret_type, Type::Unknown) {
            if let Some(hint) = self.infer_return_type(&func.name) {
                hints.push(hint.clone());
                self.return_hints.insert(func.name.clone(), hint);
            }
        }
    }

    fn collect_variable_hints(&mut self, hints: &mut Vec<TypeHint>) {
        for (var_name, patterns) in &self.context.usage_patterns {
            if let Some(hint) = self.infer_variable_type(var_name, patterns) {
                hints.push(hint.clone());
                self.variable_types.insert(var_name.clone(), hint);
            }
        }
    }

    /// Generate enhanced error with type hints
    pub fn enhance_error(&self, error: &mut EnhancedError, context: &str) {
        self.add_variable_hint_to_error(error, context);
        self.add_function_hints_to_error(error);
    }

    fn add_variable_hint_to_error(&self, error: &mut EnhancedError, context: &str) {
        if let Some(var_match) = extract_variable_from_error(context) {
            if let Some(hint) = self.variable_types.get(&var_match) {
                self.add_type_suggestion(error, &var_match, hint);
                self.add_type_note(error, hint);
            }
        }
    }

    fn add_type_suggestion(&self, error: &mut EnhancedError, var_name: &str, hint: &TypeHint) {
        error.suggestion = Some(format!(
            "Consider adding type annotation: {}: {}",
            var_name,
            self.type_to_annotation(&hint.suggested_type)
        ));
    }

    fn add_type_note(&self, error: &mut EnhancedError, hint: &TypeHint) {
        error.notes.push(format!(
            "Type inference suggests '{}' based on {} (confidence: {:?})",
            self.type_to_annotation(&hint.suggested_type),
            hint.reason,
            hint.confidence
        ));
    }

    fn add_function_hints_to_error(&self, error: &mut EnhancedError) {
        if let Some(func_name) = &self.context.current_function {
            if let Some(param_hints) = self.parameter_hints.get(func_name) {
                for hint in param_hints {
                    error.notes.push(format!(
                        "Parameter type hint: {} ({})",
                        self.type_to_annotation(&hint.suggested_type),
                        hint.reason
                    ));
                }
            }
        }
    }

    /// Format type hints for display
    pub fn format_hints(&self, hints: &[TypeHint]) -> String {
        hints
            .iter()
            .map(|hint| self.format_single_hint(hint))
            .collect()
    }

    fn format_single_hint(&self, hint: &TypeHint) -> String {
        let mut output = String::new();
        let confidence_color = self.get_confidence_color(hint.confidence);
        let target_str = self.format_target(&hint.target);

        output.push_str(&format!(
            "{} {} for {} {} ({})\n",
            "Hint:".bright_blue(),
            self.type_to_annotation(&hint.suggested_type)
                .color(confidence_color),
            target_str,
            format!("[{:?}]", hint.confidence).dimmed(),
            hint.reason.italic()
        ));

        self.append_location_if_present(&mut output, hint.source_location);
        output
    }

    fn get_confidence_color(&self, confidence: Confidence) -> &'static str {
        match confidence {
            Confidence::Certain => "green",
            Confidence::High => "bright green",
            Confidence::Medium => "yellow",
            Confidence::Low => "bright yellow",
        }
    }

    fn format_target(&self, target: &HintTarget) -> String {
        match target {
            HintTarget::Parameter(name) => format!("parameter '{}'", name),
            HintTarget::Return => "return type".to_string(),
            HintTarget::Variable(name) => format!("variable '{}'", name),
        }
    }

    fn append_location_if_present(&self, output: &mut String, location: Option<(usize, usize)>) {
        if let Some((line, col)) = location {
            output.push_str(&format!(
                "     {} line {}, column {}\n",
                "at".dimmed(),
                line,
                col
            ));
        }
    }

    fn analyze_body(&mut self, body: &[HirStmt]) -> Result<()> {
        for stmt in body {
            self.analyze_stmt(stmt)?;
        }
        Ok(())
    }

    fn analyze_stmt(&mut self, stmt: &HirStmt) -> Result<()> {
        match stmt {
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol(var_name),
                value,
                ..
            } => self.analyze_assignment(var_name, value),
            HirStmt::Assign { .. } => Ok(()),
            HirStmt::Return(Some(expr)) => self.analyze_return(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => self.analyze_if_stmt(condition, then_body, else_body),
            HirStmt::While { condition, body } => self.analyze_while_stmt(condition, body),
            HirStmt::For { target, iter, body } => self.analyze_for_stmt(target, iter, body),
            // DEPYLER-0436: Analyze Try blocks to infer types from exception handlers
            HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => self.analyze_try_stmt(body, handlers, finalbody),
            // DEPYLER-0432: Analyze With statements to infer types from context (e.g., open(filepath))
            HirStmt::With { context, body, .. } => self.analyze_with_stmt(context, body),
            HirStmt::Expr(expr) => self.analyze_expr(expr),
            _ => Ok(()),
        }
    }

    fn analyze_if_stmt(
        &mut self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
    ) -> Result<()> {
        // DEPYLER-0432: If condition is a simple variable, infer bool type
        self.infer_bool_from_condition(condition);
        self.analyze_expr(condition)?;
        self.analyze_body(then_body)?;
        if let Some(else_stmts) = else_body {
            self.analyze_body(else_stmts)?;
        }
        Ok(())
    }

    fn analyze_while_stmt(&mut self, condition: &HirExpr, body: &[HirStmt]) -> Result<()> {
        // DEPYLER-0432: If condition is a simple variable, infer bool type
        self.infer_bool_from_condition(condition);
        self.analyze_expr(condition)?;
        self.analyze_body(body)
    }

    /// DEPYLER-0432: Infer bool type for variables used directly in conditions
    fn infer_bool_from_condition(&mut self, condition: &HirExpr) {
        if let HirExpr::Var(var) = condition {
            // Variable used directly as condition → likely bool
            // Add multiple constraints for higher confidence (need 4+ for High confidence)
            self.context.constraints.push(TypeConstraint::Compatible {
                var: var.to_string(),
                ty: Type::Bool,
            });
            self.context.constraints.push(TypeConstraint::Compatible {
                var: var.to_string(),
                ty: Type::Bool,
            });
        }
    }

    fn analyze_for_stmt(
        &mut self,
        target: &crate::hir::AssignTarget,
        iter: &HirExpr,
        body: &[HirStmt],
    ) -> Result<()> {
        // DEPYLER-0451 Phase 1c: Track loop variable sources for back-propagation
        // For simple symbol targets: for item in items: ...
        if let crate::hir::AssignTarget::Symbol(target_name) = target {
            // Track the loop variable source
            if let HirExpr::Var(iter_var) = iter {
                self.context
                    .loop_var_sources
                    .insert(target_name.clone(), iter_var.clone());
            }

            self.analyze_for_loop(target_name, iter)?;
        }

        // Analyze the loop body (this will collect constraints on loop variables)
        self.analyze_body(body)?;

        // DEPYLER-0451 Phase 1c: Back-propagate element types to collection types
        // After analyzing the body, we know how loop variables are used
        // Apply those constraints to the iterable parameters
        if let crate::hir::AssignTarget::Symbol(target_name) = target {
            self.back_propagate_element_constraints(target_name)?;
        }

        Ok(())
    }

    fn analyze_try_stmt(
        &mut self,
        body: &[HirStmt],
        handlers: &[crate::hir::ExceptHandler],
        finalbody: &Option<Vec<HirStmt>>,
    ) -> Result<()> {
        // Analyze the try body
        self.analyze_body(body)?;

        // Analyze each exception handler body
        for handler in handlers {
            self.analyze_body(&handler.body)?;
        }

        // Analyze the finally body if present
        if let Some(final_stmts) = finalbody {
            self.analyze_body(final_stmts)?;
        }

        Ok(())
    }

    /// DEPYLER-0432: Analyze with statement context expressions
    fn analyze_with_stmt(&mut self, context: &HirExpr, body: &[HirStmt]) -> Result<()> {
        // Analyze the context expression (e.g., open(filepath))
        self.analyze_expr(context)?;
        // Analyze the body
        self.analyze_body(body)
    }

    fn analyze_expr(&mut self, expr: &HirExpr) -> Result<()> {
        match expr {
            HirExpr::Binary { left, right, op } => self.analyze_binary_op(left, right, *op),
            HirExpr::Call { func, args, .. } => self.analyze_call(func, args),
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => self.analyze_method_call(object, method, args),
            HirExpr::Index { base, index } => self.analyze_indexing(base, index),
            // DEPYLER-0492: Slicing operations imply list/vector type
            HirExpr::Slice { base, .. } => self.analyze_slicing(base),
            // DEPYLER-0451 Phase 1b: F-string type inference
            HirExpr::FString { parts } => self.analyze_fstring(parts),
            _ => Ok(()),
        }
    }

    /// DEPYLER-0451 Phase 1b: Infer String type for variables used in f-strings
    fn analyze_fstring(&mut self, parts: &[crate::hir::FStringPart]) -> Result<()> {
        use crate::hir::FStringPart;

        for part in parts {
            if let FStringPart::Expr(expr) = part {
                // Variables in f-strings should be string-like (can be formatted)
                if let HirExpr::Var(var) = expr.as_ref() {
                    // Add multiple constraints for higher confidence
                    self.add_compatible_constraint(var, Type::String);
                    self.add_compatible_constraint(var, Type::String);
                    self.record_usage_pattern(var, UsagePattern::StringLike);
                }
                // Recursively analyze nested expressions
                self.analyze_expr(expr)?;
            }
        }
        Ok(())
    }

    fn analyze_assignment(&mut self, var_name: &str, value: &HirExpr) -> Result<()> {
        self.infer_from_literal(var_name, value);
        self.infer_from_collection(var_name, value);

        // DEPYLER-0531: Track variables assigned from non-list sources
        // These should NOT default to List<String> even if they have Container pattern
        match value {
            // Variable assigned from another variable (might be a parameter or dict)
            HirExpr::Var(_) => {
                self.context.non_list_variables.insert(var_name.to_string());
            }
            // Variable assigned from indexing (e.g., value = config["key"])
            // DEPYLER-0265: Infer element type from base list/dict type
            HirExpr::Index { base, .. } => {
                self.context.non_list_variables.insert(var_name.to_string());
                // If base is a variable, try to get its type and extract element type
                if let HirExpr::Var(base_name) = base.as_ref() {
                    let base_type = self.infer_var_type(base_name);
                    if let Some(elem_type) = self.extract_element_type(&base_type) {
                        self.add_compatible_constraint(var_name, elem_type);
                    }
                }
            }
            // Variable assigned from dict literal
            HirExpr::Dict(_) => {
                self.context.non_list_variables.insert(var_name.to_string());
            }
            // Variable assigned from attribute access (e.g., obj.value)
            HirExpr::Attribute { .. } => {
                self.context.non_list_variables.insert(var_name.to_string());
            }
            // DEPYLER-0532: Handle method calls that return known types
            HirExpr::MethodCall { object, method, .. } => {
                // Check for module method calls with known return types
                if let HirExpr::Var(module_name) = object.as_ref() {
                    let module_method_type = match (module_name.as_str(), method.as_str()) {
                        // Regex methods that return lists
                        ("re", "findall") | ("regex", "findall") => {
                            Some(Type::List(Box::new(Type::String)))
                        }
                        ("re", "split") | ("regex", "split") => {
                            Some(Type::List(Box::new(Type::String)))
                        }
                        // JSON methods
                        ("json", "loads") | ("json", "load") => {
                            Some(Type::Custom("serde_json::Value".to_string()))
                        }
                        ("json", "dumps") => Some(Type::String),
                        _ => None,
                    };
                    if let Some(ty) = module_method_type {
                        self.add_compatible_constraint(var_name, ty);
                    }
                }
            }
            _ => {}
        }

        self.analyze_expr(value)
    }

    fn infer_from_literal(&mut self, var_name: &str, value: &HirExpr) {
        if let HirExpr::Literal(lit) = value {
            let ty = self.literal_to_type(lit);
            self.add_compatible_constraint(var_name, ty);
        }
    }

    fn literal_to_type(&self, lit: &crate::hir::Literal) -> Type {
        match lit {
            crate::hir::Literal::Int(_) => Type::Int,
            crate::hir::Literal::Float(_) => Type::Float,
            crate::hir::Literal::String(_) => Type::String,
            crate::hir::Literal::Bytes(_) => Type::Custom("bytes".to_string()),
            crate::hir::Literal::Bool(_) => Type::Bool,
            crate::hir::Literal::None => Type::None,
        }
    }

    fn add_compatible_constraint(&mut self, var_name: &str, ty: Type) {
        self.context.constraints.push(TypeConstraint::Compatible {
            var: var_name.to_string(),
            ty,
        });
    }

    fn infer_from_collection(&mut self, var_name: &str, value: &HirExpr) {
        match value {
            HirExpr::List(elems) => self.infer_list_type(var_name, elems),
            HirExpr::Dict(items) => self.infer_dict_type(var_name, items),
            HirExpr::Set(elems) => self.infer_set_type(var_name, elems),
            HirExpr::Tuple(elems) => self.infer_tuple_type(var_name, elems),
            _ => {}
        }
    }

    fn infer_list_type(&mut self, var_name: &str, elems: &[HirExpr]) {
        let elem_type = self.infer_collection_element_type(elems);
        let list_type = Type::List(Box::new(elem_type));
        self.add_compatible_constraint(var_name, list_type);
    }

    fn infer_dict_type(&mut self, var_name: &str, items: &[(HirExpr, HirExpr)]) {
        if items.is_empty() {
            return;
        }

        // DEPYLER-0740: Check if any value is None
        let has_none_value = items
            .iter()
            .any(|(_, v)| matches!(v, HirExpr::Literal(crate::hir::Literal::None)));

        // Get key type from first item
        let key_type = self.infer_expr_type(&items[0].0);

        // Get value type from first non-None value
        let base_val_type = items
            .iter()
            .filter(|(_, v)| !matches!(v, HirExpr::Literal(crate::hir::Literal::None)))
            .map(|(_, v)| self.infer_expr_type(v))
            .find(|t| !matches!(t, Type::None | Type::Unknown))
            .unwrap_or_else(|| self.infer_expr_type(&items[0].1));

        // If any value is None, wrap value type in Option
        let val_type = if has_none_value && !matches!(base_val_type, Type::None) {
            Type::Optional(Box::new(base_val_type))
        } else {
            base_val_type
        };

        let dict_type = Type::Dict(Box::new(key_type), Box::new(val_type));
        self.add_compatible_constraint(var_name, dict_type);
    }

    // DEPYLER-0742: Infer set type for variable assignment, handling None values
    fn infer_set_type(&mut self, var_name: &str, elems: &[HirExpr]) {
        if elems.is_empty() {
            return;
        }

        // Check if any element is None
        let has_none = elems
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(crate::hir::Literal::None)));

        // Get element type from first non-None element
        let base_elem_type = elems
            .iter()
            .filter(|e| !matches!(e, HirExpr::Literal(crate::hir::Literal::None)))
            .map(|e| self.infer_expr_type(e))
            .find(|t| !matches!(t, Type::None | Type::Unknown))
            .unwrap_or_else(|| self.infer_expr_type(&elems[0]));

        // If any element is None, wrap element type in Option
        let elem_type = if has_none && !matches!(base_elem_type, Type::None) {
            Type::Optional(Box::new(base_elem_type))
        } else {
            base_elem_type
        };

        let set_type = Type::Set(Box::new(elem_type));
        self.add_compatible_constraint(var_name, set_type);
    }

    // DEPYLER-0743: Infer tuple type for variable assignment, handling None values
    fn infer_tuple_type(&mut self, var_name: &str, elems: &[HirExpr]) {
        let elem_types: Vec<Type> = elems
            .iter()
            .map(|e| {
                let ty = self.infer_expr_type(e);
                // For None elements in tuple, use Option<()>
                if matches!(ty, Type::None) {
                    Type::Optional(Box::new(Type::Custom("()".to_string())))
                } else {
                    ty
                }
            })
            .collect();
        let tuple_type = Type::Tuple(elem_types);
        self.add_compatible_constraint(var_name, tuple_type);
    }

    fn analyze_return(&mut self, expr: &HirExpr) -> Result<()> {
        let return_type = self.infer_expr_type(expr);
        if let Some(func_name) = &self.context.current_function {
            self.context
                .constraints
                .push(TypeConstraint::ReturnConstraint {
                    var: func_name.clone(),
                    ty: return_type,
                });
        }
        self.analyze_expr(expr)?;
        Ok(())
    }

    fn analyze_for_loop(&mut self, target: &str, iter: &HirExpr) -> Result<()> {
        // Record iterator usage
        if let HirExpr::Var(var_name) = iter {
            self.record_usage_pattern(var_name, UsagePattern::Iterator);
        }

        // Infer loop variable type
        let elem_type = match iter {
            HirExpr::Call { func, .. } if func == "range" => Type::Int,
            HirExpr::Var(var_name) => {
                // Try to infer from existing constraints
                self.infer_iterator_element_type(var_name)
            }
            _ => Type::Unknown,
        };

        if !matches!(elem_type, Type::Unknown) {
            self.context.constraints.push(TypeConstraint::Compatible {
                var: target.to_string(),
                ty: elem_type,
            });
        }

        self.analyze_expr(iter)?;
        Ok(())
    }

    /// DEPYLER-0451 Phase 1c: Back-propagate element type constraints to collection types
    /// Example: `for item in items: total += item`
    /// - `item` gets Int constraint from arithmetic
    /// - Back-propagate: `items` should be &[Int]
    fn back_propagate_element_constraints(&mut self, loop_var: &str) -> Result<()> {
        // Find the source collection for this loop variable
        let source_collection = match self.context.loop_var_sources.get(loop_var) {
            Some(src) => src.clone(),
            None => return Ok(()), // No source tracked
        };

        // Collect all constraints on the loop variable
        let mut loop_var_constraints: Vec<Type> = self
            .context
            .constraints
            .iter()
            .filter_map(|constraint| match constraint {
                TypeConstraint::Compatible { var, ty } if var == loop_var => Some(ty.clone()),
                _ => None,
            })
            .collect();

        // DEPYLER-0451: Also infer from usage patterns if no explicit constraints
        if loop_var_constraints.is_empty() {
            if let Some(patterns) = self.context.usage_patterns.get(loop_var) {
                for pattern in patterns {
                    match pattern {
                        UsagePattern::Numeric => loop_var_constraints.push(Type::Int),
                        UsagePattern::StringLike => loop_var_constraints.push(Type::String),
                        _ => {}
                    }
                }
            }
        }

        // Back-propagate each element type constraint to the collection
        for elem_type in loop_var_constraints {
            // Element type → Collection type mapping
            let collection_type = match elem_type {
                Type::Int => Type::List(Box::new(Type::Int)),
                Type::Float => Type::List(Box::new(Type::Float)),
                Type::String => Type::List(Box::new(Type::String)),
                Type::Bool => Type::List(Box::new(Type::Bool)),
                other => Type::List(Box::new(other)),
            };

            // Add multiple constraints for Certain confidence (need 6+ votes)
            // This ensures typed collections beat generic List(Unknown) from Iterator pattern
            for _ in 0..4 {
                self.context.constraints.push(TypeConstraint::Compatible {
                    var: source_collection.clone(),
                    ty: collection_type.clone(),
                });
            }
        }

        Ok(())
    }

    fn analyze_binary_op(
        &mut self,
        left: &HirExpr,
        right: &HirExpr,
        op: crate::hir::BinOp,
    ) -> Result<()> {
        use crate::hir::BinOp;

        // Record numeric usage patterns
        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                // DEPYLER-0451: Stronger type inference for arithmetic with literals
                // If one operand is an integer literal, infer the variable as Int
                self.infer_int_from_arithmetic(left, right);

                if let HirExpr::Var(var) = left {
                    self.record_usage_pattern(var, UsagePattern::Numeric);
                }
                if let HirExpr::Var(var) = right {
                    self.record_usage_pattern(var, UsagePattern::Numeric);
                }
            }
            _ => {}
        }

        self.analyze_expr(left)?;
        self.analyze_expr(right)?;
        Ok(())
    }

    /// DEPYLER-0451: Infer Int type when variable is used in arithmetic with integer literal
    fn infer_int_from_arithmetic(&mut self, left: &HirExpr, right: &HirExpr) {
        use crate::hir::Literal;

        // Check if left is var and right is int literal (e.g., x + 1)
        if let (HirExpr::Var(var), HirExpr::Literal(Literal::Int(_))) = (left, right) {
            // Add multiple constraints for higher confidence
            self.add_compatible_constraint(var, Type::Int);
            self.add_compatible_constraint(var, Type::Int);
        }

        // Check if right is var and left is int literal (e.g., 1 + x)
        if let (HirExpr::Literal(Literal::Int(_)), HirExpr::Var(var)) = (left, right) {
            // Add multiple constraints for higher confidence
            self.add_compatible_constraint(var, Type::Int);
            self.add_compatible_constraint(var, Type::Int);
        }
    }

    fn analyze_call(&mut self, func: &str, args: &[HirExpr]) -> Result<()> {
        self.analyze_builtin_call(func, args);
        self.analyze_call_arguments(args)
    }

    fn analyze_builtin_call(&mut self, func: &str, args: &[HirExpr]) {
        match func {
            "len" => self.analyze_len_call(args),
            "str" | "int" | "float" | "bool" => self.analyze_conversion_call(func, args),
            "open" => self.analyze_open_call(args),
            _ => {}
        }
    }

    /// DEPYLER-0432: Detect open(filepath) - filepath should be &str
    fn analyze_open_call(&mut self, args: &[HirExpr]) {
        if let Some(HirExpr::Var(var)) = args.first() {
            // open(filepath) means filepath is a file path (String/&str)
            self.context.constraints.push(TypeConstraint::Compatible {
                var: var.to_string(),
                ty: Type::String,
            });
            // Record string-like pattern for stronger evidence
            self.record_usage_pattern(var, UsagePattern::StringLike);
        }
    }

    fn analyze_len_call(&mut self, args: &[HirExpr]) {
        if let Some(HirExpr::Var(var)) = args.first() {
            self.record_usage_pattern(var, UsagePattern::Container);
        }
    }

    fn analyze_conversion_call(&mut self, func: &str, args: &[HirExpr]) {
        if let Some(HirExpr::Var(var)) = args.first() {
            // DEPYLER-0436: int(value) means value is a string being parsed
            // This is the argparse validator pattern: def validator(value): int(value)
            if func == "int" {
                // Add evidence that this variable is a String (will map to &str)
                self.context.constraints.push(TypeConstraint::Compatible {
                    var: var.to_string(),
                    ty: Type::String,
                });
                // Also record string-like usage pattern for stronger evidence
                self.record_usage_pattern(var, UsagePattern::StringLike);
            } else {
                let target_type = self.conversion_target_type(func);
                self.add_argument_constraint(var, func, target_type);
            }
        }
    }

    fn conversion_target_type(&self, func: &str) -> Type {
        match func {
            "str" => Type::String,
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            _ => Type::Unknown,
        }
    }

    fn add_argument_constraint(&mut self, var: &str, func: &str, expected: Type) {
        self.context
            .constraints
            .push(TypeConstraint::ArgumentConstraint {
                var: var.to_string(),
                func: func.to_string(),
                _param_idx: 0,
                expected,
            });
    }

    fn analyze_call_arguments(&mut self, args: &[HirExpr]) -> Result<()> {
        for arg in args {
            self.analyze_expr(arg)?;
        }
        Ok(())
    }

    fn analyze_method_call(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<()> {
        if let HirExpr::Var(var) = object {
            // DEPYLER-0492: subprocess.run() expects first arg to be List[str]
            if var == "subprocess" && method == "run" {
                if let Some(HirExpr::Var(cmd_var)) = args.first() {
                    // subprocess.run(cmd) -> cmd should be Vec<String>
                    // Use ArgumentConstraint for high-confidence stdlib signature
                    self.context
                        .constraints
                        .push(TypeConstraint::ArgumentConstraint {
                            var: cmd_var.to_string(),
                            func: "subprocess.run".to_string(),
                            _param_idx: 0,
                            expected: Type::List(Box::new(Type::String)),
                        });
                    self.record_usage_pattern(cmd_var, UsagePattern::Container);
                }
            }

            // String methods
            if [
                "upper",
                "lower",
                "strip",
                "split",
                "replace",
                "startswith",
                "endswith",
            ]
            .contains(&method)
            {
                self.record_usage_pattern(var, UsagePattern::StringLike);
                self.context.constraints.push(TypeConstraint::Compatible {
                    var: var.to_string(),
                    ty: Type::String,
                });
            }

            // List methods
            if ["append", "extend", "pop", "remove", "clear", "sort"].contains(&method) {
                self.record_usage_pattern(var, UsagePattern::Container);
            }
        }

        self.analyze_expr(object)?;
        for arg in args {
            self.analyze_expr(arg)?;
        }
        Ok(())
    }

    fn analyze_indexing(&mut self, base: &HirExpr, index: &HirExpr) -> Result<()> {
        if let HirExpr::Var(var) = base {
            // DEPYLER-1040b: Check ONLY for actual evidence (string literal or fstring)
            // NOT variable names - per directive: "If you write if var_name.contains(...), you are fired"
            // Removed: DEPYLER-0552 heuristics based on variable naming patterns
            let is_string_key = matches!(
                index,
                HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::FString { .. }
            );

            if is_string_key {
                // Dict access: info["path"] → HashMap<String, Value>
                // Only when we have 100% proof (string literal index)
                self.record_usage_pattern(var, UsagePattern::DictAccess);
            } else {
                // DEPYLER-1040b: Without proof, assume DepylerValue (container)
                // "If you have 99% proof, use DepylerValue"
                self.record_usage_pattern(var, UsagePattern::Container);
            }
        }
        self.analyze_expr(base)?;
        self.analyze_expr(index)?;
        Ok(())
    }

    /// DEPYLER-0492: Slicing operations (items[1:]) imply list/vector type
    fn analyze_slicing(&mut self, base: &HirExpr) -> Result<()> {
        if let HirExpr::Var(var) = base {
            self.record_usage_pattern(var, UsagePattern::Container);
        }
        self.analyze_expr(base)?;
        Ok(())
    }

    fn record_usage_pattern(&mut self, var: &str, pattern: UsagePattern) {
        self.context
            .usage_patterns
            .entry(var.to_string())
            .or_default()
            .push(pattern);
    }

    /// DEPYLER-0492: Infer parameter type from default value (Certain confidence)
    fn infer_from_default(&self, param_name: &str, default: &Option<HirExpr>) -> Option<TypeHint> {
        let default_expr = default.as_ref()?;

        let inferred_type = match default_expr {
            HirExpr::Literal(lit) => match lit {
                crate::hir::Literal::Bool(_) => Type::Bool,
                crate::hir::Literal::Int(_) => Type::Int,
                crate::hir::Literal::Float(_) => Type::Float,
                crate::hir::Literal::String(_) => Type::String,
                crate::hir::Literal::None => return None, // None doesn't give type info
                _ => return None,                         // Other literals not yet supported
            },
            _ => return None, // Complex defaults not yet supported
        };

        Some(TypeHint {
            suggested_type: inferred_type,
            confidence: Confidence::Certain, // Default values are certain
            reason: "inferred from default value".to_string(),
            source_location: None,
            target: HintTarget::Parameter(param_name.to_string()),
        })
    }

    fn infer_parameter_type(&self, param_name: &str) -> Option<TypeHint> {
        let mut type_votes: HashMap<Type, (u32, Vec<String>)> = HashMap::new();

        self.collect_constraint_evidence(param_name, &mut type_votes);
        self.collect_pattern_evidence(param_name, &mut type_votes);

        self.build_type_hint_from_votes(param_name, type_votes)
    }

    fn collect_constraint_evidence(
        &self,
        param_name: &str,
        type_votes: &mut HashMap<Type, (u32, Vec<String>)>,
    ) {
        for constraint in &self.context.constraints {
            match constraint {
                TypeConstraint::Compatible { var, ty } if var == param_name => {
                    self.add_compatible_evidence(ty, type_votes);
                }
                TypeConstraint::OperatorConstraint { var, op, required } if var == param_name => {
                    self.add_operator_evidence(op, required, type_votes);
                }
                // DEPYLER-0492: High-confidence stdlib function signatures
                TypeConstraint::ArgumentConstraint {
                    var,
                    func,
                    expected,
                    ..
                } if var == param_name => {
                    self.add_argument_evidence(func, expected, type_votes);
                }
                _ => {}
            }
        }
    }

    fn add_compatible_evidence(
        &self,
        ty: &Type,
        type_votes: &mut HashMap<Type, (u32, Vec<String>)>,
    ) {
        let (count, reasons) = type_votes.entry(ty.clone()).or_default();
        *count += 2;
        reasons.push("direct assignment".to_string());
    }

    fn add_operator_evidence(
        &self,
        op: &str,
        required: &Type,
        type_votes: &mut HashMap<Type, (u32, Vec<String>)>,
    ) {
        let (count, reasons) = type_votes.entry(required.clone()).or_default();
        *count += 1;
        reasons.push(format!("used with {} operator", op));
    }

    /// DEPYLER-0492: High-confidence evidence from stdlib function signatures
    /// Score of 5 gives Confidence::High, ensuring parameter types are inferred
    fn add_argument_evidence(
        &self,
        func: &str,
        expected: &Type,
        type_votes: &mut HashMap<Type, (u32, Vec<String>)>,
    ) {
        let (count, reasons) = type_votes.entry(expected.clone()).or_default();
        *count += 5; // High confidence (score ≥ 4)
        reasons.push(format!("stdlib function {} signature", func));
    }

    fn collect_pattern_evidence(
        &self,
        param_name: &str,
        type_votes: &mut HashMap<Type, (u32, Vec<String>)>,
    ) {
        if let Some(patterns) = self.context.usage_patterns.get(param_name) {
            for pattern in patterns {
                self.add_pattern_evidence(pattern, type_votes);
            }
        }
    }

    fn add_pattern_evidence(
        &self,
        pattern: &UsagePattern,
        type_votes: &mut HashMap<Type, (u32, Vec<String>)>,
    ) {
        match pattern {
            UsagePattern::Numeric => self.add_numeric_evidence(type_votes),
            UsagePattern::StringLike => self.add_string_evidence(type_votes),
            UsagePattern::Iterator => self.add_iterator_evidence(type_votes),
            // DEPYLER-0492: Container pattern from integer indexing/slicing
            UsagePattern::Container => self.add_container_evidence(type_votes),
            // DEPYLER-0552: Dict pattern from string-keyed access
            UsagePattern::DictAccess => self.add_dict_access_evidence(type_votes),
            _ => {}
        }
    }

    fn add_numeric_evidence(&self, type_votes: &mut HashMap<Type, (u32, Vec<String>)>) {
        let (count, reasons) = type_votes.entry(Type::Int).or_default();
        *count += 1;
        reasons.push("numeric operations".to_string());
    }

    fn add_string_evidence(&self, type_votes: &mut HashMap<Type, (u32, Vec<String>)>) {
        let (count, reasons) = type_votes.entry(Type::String).or_default();
        *count += 2;
        reasons.push("string methods".to_string());
    }

    fn add_iterator_evidence(&self, type_votes: &mut HashMap<Type, (u32, Vec<String>)>) {
        let (count, reasons) = type_votes
            .entry(Type::List(Box::new(Type::Unknown)))
            .or_default();
        *count += 1;
        reasons.push("used as iterator".to_string());
    }

    /// DEPYLER-0492: High-confidence evidence from integer indexing/slicing operations
    fn add_container_evidence(&self, type_votes: &mut HashMap<Type, (u32, Vec<String>)>) {
        let (count, reasons) = type_votes
            .entry(Type::List(Box::new(Type::Unknown)))
            .or_default();
        *count += 4; // High confidence - indexing strongly implies list type
        reasons.push("indexing/slicing operation".to_string());
    }

    /// DEPYLER-0552: High-confidence evidence from string-keyed access (dict access)
    fn add_dict_access_evidence(&self, type_votes: &mut HashMap<Type, (u32, Vec<String>)>) {
        let (count, reasons) = type_votes
            .entry(Type::Dict(
                Box::new(Type::String),
                Box::new(Type::Custom("serde_json::Value".to_string())),
            ))
            .or_default();
        *count += 5; // Higher confidence than list - string keys are definitive dict access
        reasons.push("string-keyed access (dict)".to_string());
    }

    fn build_type_hint_from_votes(
        &self,
        param_name: &str,
        type_votes: HashMap<Type, (u32, Vec<String>)>,
    ) -> Option<TypeHint> {
        let (suggested_type, (score, reasons)) = type_votes
            .into_iter()
            .max_by_key(|(_, (count, _))| *count)?;

        let confidence = self.score_to_confidence(score);

        Some(TypeHint {
            suggested_type,
            confidence,
            reason: reasons.join(", "),
            source_location: None,
            target: HintTarget::Parameter(param_name.to_string()),
        })
    }

    fn score_to_confidence(&self, score: u32) -> Confidence {
        match score {
            0..=1 => Confidence::Low,
            2..=3 => Confidence::Medium,
            4..=5 => Confidence::High,
            _ => Confidence::Certain,
        }
    }

    fn infer_return_type(&self, func_name: &str) -> Option<TypeHint> {
        let return_types = self.collect_return_types(func_name);
        self.build_return_type_hint(return_types)
    }

    fn collect_return_types(&self, func_name: &str) -> HashMap<Type, Vec<String>> {
        let mut return_types: HashMap<Type, Vec<String>> = HashMap::new();

        for constraint in &self.context.constraints {
            if let TypeConstraint::ReturnConstraint { var, ty } = constraint {
                if var == func_name && !matches!(ty, Type::Unknown) {
                    return_types
                        .entry(ty.clone())
                        .or_default()
                        .push("explicit return".to_string());
                }
            }
        }

        return_types
    }

    fn build_return_type_hint(&self, return_types: HashMap<Type, Vec<String>>) -> Option<TypeHint> {
        let (suggested_type, reasons) = return_types
            .into_iter()
            .max_by_key(|(_, reasons)| reasons.len())?;

        let confidence = self.return_confidence(&reasons);

        Some(TypeHint {
            suggested_type,
            confidence,
            reason: reasons.join(", "),
            source_location: None,
            target: HintTarget::Return,
        })
    }

    fn return_confidence(&self, reasons: &[String]) -> Confidence {
        if reasons.len() > 1 {
            Confidence::High
        } else {
            Confidence::Medium
        }
    }

    fn infer_variable_type(&self, var_name: &str, patterns: &[UsagePattern]) -> Option<TypeHint> {
        let type_score = self.score_variable_patterns(patterns);
        self.build_variable_hint(var_name, type_score)
    }

    fn score_variable_patterns(&self, patterns: &[UsagePattern]) -> HashMap<Type, u32> {
        let mut type_score = HashMap::new();

        for pattern in patterns {
            self.update_type_score(pattern, &mut type_score);
        }

        type_score
    }

    fn update_type_score(&self, pattern: &UsagePattern, type_score: &mut HashMap<Type, u32>) {
        match pattern {
            UsagePattern::Numeric => *type_score.entry(Type::Int).or_insert(0) += 1,
            UsagePattern::StringLike => *type_score.entry(Type::String).or_insert(0) += 2,
            // DEPYLER-0492: Integer indexing/slicing strongly implies list type (High confidence)
            UsagePattern::Container => {
                *type_score
                    .entry(Type::List(Box::new(Type::Unknown)))
                    .or_insert(0) += 4 // High confidence (was 1)
            }
            // DEPYLER-0552: String-keyed access strongly implies dict type (Higher confidence)
            UsagePattern::DictAccess => {
                *type_score
                    .entry(Type::Dict(
                        Box::new(Type::String),
                        Box::new(Type::Custom("serde_json::Value".to_string())),
                    ))
                    .or_insert(0) += 5 // Higher confidence than list
            }
            _ => {}
        }
    }

    fn build_variable_hint(
        &self,
        var_name: &str,
        type_score: HashMap<Type, u32>,
    ) -> Option<TypeHint> {
        let (suggested_type, score) = type_score.into_iter().max_by_key(|(_, score)| *score)?;

        let confidence = self.variable_confidence(score);

        Some(TypeHint {
            suggested_type,
            confidence,
            reason: "usage patterns suggest this type".to_string(),
            source_location: None,
            target: HintTarget::Variable(var_name.to_string()),
        })
    }

    fn variable_confidence(&self, score: u32) -> Confidence {
        if score > 2 {
            Confidence::High
        } else {
            Confidence::Medium
        }
    }

    fn infer_collection_element_type(&self, elems: &[HirExpr]) -> Type {
        if elems.is_empty() {
            return Type::Unknown;
        }

        // DEPYLER-0739: Check if any element is None
        let has_none = elems
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(crate::hir::Literal::None)));

        // DEPYLER-0741: Check if this is a list of dicts and ANY dict has None values
        // If so, all dicts should have Option<V> value type for consistency
        let any_dict_has_none = elems.iter().any(|e| {
            if let HirExpr::Dict(items) = e {
                items
                    .iter()
                    .any(|(_, v)| matches!(v, HirExpr::Literal(crate::hir::Literal::None)))
            } else {
                false
            }
        });

        // Find first non-None element type
        let base_type = elems
            .iter()
            .filter(|e| !matches!(e, HirExpr::Literal(crate::hir::Literal::None)))
            .map(|e| self.infer_expr_type(e))
            .find(|t| !matches!(t, Type::None | Type::Unknown))
            .unwrap_or_else(|| self.infer_expr_type(&elems[0]));

        // DEPYLER-0741: If list of dicts and any dict has None value,
        // modify the dict value type to be Optional
        // Also need to find the real value type from a dict with non-None values
        if any_dict_has_none {
            // Find a dict with a non-None value to determine the real value type
            let real_value_type = elems
                .iter()
                .filter_map(|e| {
                    if let HirExpr::Dict(items) = e {
                        items
                            .iter()
                            .filter(|(_, v)| {
                                !matches!(v, HirExpr::Literal(crate::hir::Literal::None))
                            })
                            .map(|(_, v)| self.infer_expr_type(v))
                            .find(|t| !matches!(t, Type::None | Type::Unknown))
                    } else {
                        None
                    }
                })
                .next();

            // Get key type from first dict
            let key_type = elems
                .iter()
                .filter_map(|e| {
                    if let HirExpr::Dict(items) = e {
                        items.first().map(|(k, _)| self.infer_expr_type(k))
                    } else {
                        None
                    }
                })
                .next()
                .unwrap_or(Type::Unknown);

            if let Some(val_type) = real_value_type {
                return Type::Dict(Box::new(key_type), Box::new(Type::Optional(Box::new(val_type))));
            } else if let Type::Dict(k, v) = base_type {
                // Fallback: wrap value type in Option if not already
                let opt_v = if matches!(v.as_ref(), Type::Optional(_)) {
                    v
                } else {
                    Box::new(Type::Optional(v))
                };
                return Type::Dict(k, opt_v);
            }
        }

        // If list contains None, wrap element type in Option
        if has_none && !matches!(base_type, Type::None) {
            Type::Optional(Box::new(base_type))
        } else {
            base_type
        }
    }

    fn infer_expr_type(&self, expr: &HirExpr) -> Type {
        match expr {
            HirExpr::Literal(lit) => self.literal_to_type(lit),
            HirExpr::List(elems) => self.infer_list_expr_type(elems),
            HirExpr::Dict(items) => self.infer_dict_expr_type(items),
            HirExpr::Set(elems) => self.infer_set_expr_type(elems),
            HirExpr::Tuple(elems) => self.infer_tuple_expr_type(elems),
            HirExpr::Var(name) => self.infer_var_type(name),
            _ => Type::Unknown,
        }
    }

    fn infer_list_expr_type(&self, elems: &[HirExpr]) -> Type {
        Type::List(Box::new(self.infer_collection_element_type(elems)))
    }

    // DEPYLER-0742: Infer set type, handling None values
    fn infer_set_expr_type(&self, elems: &[HirExpr]) -> Type {
        if elems.is_empty() {
            return Type::Set(Box::new(Type::Unknown));
        }

        // Check if any element is None
        let has_none = elems
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(crate::hir::Literal::None)));

        // Get element type from first non-None element
        let base_elem_type = elems
            .iter()
            .filter(|e| !matches!(e, HirExpr::Literal(crate::hir::Literal::None)))
            .map(|e| self.infer_expr_type(e))
            .find(|t| !matches!(t, Type::None | Type::Unknown))
            .unwrap_or_else(|| self.infer_expr_type(&elems[0]));

        // If any element is None, wrap element type in Option
        let elem_type = if has_none && !matches!(base_elem_type, Type::None) {
            Type::Optional(Box::new(base_elem_type))
        } else {
            base_elem_type
        };

        Type::Set(Box::new(elem_type))
    }

    // DEPYLER-0743: Infer tuple type, handling None values in individual positions
    fn infer_tuple_expr_type(&self, elems: &[HirExpr]) -> Type {
        let elem_types: Vec<Type> = elems
            .iter()
            .map(|e| {
                let ty = self.infer_expr_type(e);
                // DEPYLER-0743: For None elements in tuple, use Option<()>
                // because None as a value needs Option type, and () is the
                // simplest inner type when we don't have more context
                if matches!(ty, Type::None) {
                    Type::Optional(Box::new(Type::Custom("()".to_string())))
                } else {
                    ty
                }
            })
            .collect();
        Type::Tuple(elem_types)
    }

    // DEPYLER-0740: Infer dict type, handling None values
    // DEPYLER-1051: Check for heterogeneous value types → DepylerValue
    fn infer_dict_expr_type(&self, items: &[(HirExpr, HirExpr)]) -> Type {
        if items.is_empty() {
            return Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown));
        }

        // Check if any value is None
        let has_none_value = items
            .iter()
            .any(|(_, v)| matches!(v, HirExpr::Literal(crate::hir::Literal::None)));

        // Get key type from first item
        let key_type = self.infer_expr_type(&items[0].0);

        // DEPYLER-1051: Collect all non-None value types to check for heterogeneity
        let value_types: Vec<Type> = items
            .iter()
            .filter(|(_, v)| !matches!(v, HirExpr::Literal(crate::hir::Literal::None)))
            .map(|(_, v)| self.infer_expr_type(v))
            .filter(|t| !matches!(t, Type::None | Type::Unknown))
            .collect();

        // Check if all value types are the same (homogeneous)
        let is_homogeneous = if value_types.is_empty() {
            true
        } else {
            let first = &value_types[0];
            value_types.iter().skip(1).all(|t| t == first)
        };

        let base_val_type = if !is_homogeneous {
            // DEPYLER-1051: Mixed types → Unknown (maps to DepylerValue)
            Type::Unknown
        } else if let Some(first) = value_types.first() {
            first.clone()
        } else {
            self.infer_expr_type(&items[0].1)
        };

        // If any value is None, wrap value type in Option (only for homogeneous types)
        let val_type = if has_none_value && !matches!(base_val_type, Type::None | Type::Unknown) {
            Type::Optional(Box::new(base_val_type))
        } else {
            base_val_type
        };

        Type::Dict(Box::new(key_type), Box::new(val_type))
    }

    fn infer_var_type(&self, name: &str) -> Type {
        // DEPYLER-0531: First check explicit Compatible constraints
        // This takes priority because Container/Iterator patterns only tell us
        // a variable can be indexed/iterated, not its actual type. A dict or
        // serde_json::Value can also be indexed, so we shouldn't assume List.
        for constraint in &self.context.constraints {
            if let TypeConstraint::Compatible { var, ty } = constraint {
                if var == name {
                    return ty.clone();
                }
            }
        }

        // DEPYLER-0531: Skip List inference for variables assigned from non-list sources
        // (parameters, indexing, dicts, attribute access)
        if self.context.non_list_variables.contains(name) {
            return Type::Unknown;
        }

        // DEPYLER-0519/0531: Then check usage patterns for Container/Iterator
        // This is lower priority because f-string analysis adds String constraints
        // to ANY variable used in formatting, even lists. But if we see the
        // variable used with iteration or len() AND no explicit constraint,
        // it's likely a list.
        //
        // Note: Container pattern (from indexing/len) could be dict OR list,
        // but without explicit constraints, we default to list since that's
        // more common in Python code being transpiled.
        if let Some(patterns) = self.context.usage_patterns.get(name) {
            for pattern in patterns {
                match pattern {
                    // DEPYLER-0552: Dict access takes priority
                    UsagePattern::DictAccess => {
                        return Type::Dict(
                            Box::new(Type::String),
                            Box::new(Type::Custom("serde_json::Value".to_string())),
                        );
                    }
                    UsagePattern::Container | UsagePattern::Iterator => {
                        // Both patterns suggest a collection type
                        return Type::List(Box::new(Type::String));
                    }
                    _ => {}
                }
            }
        }

        Type::Unknown
    }

    fn infer_iterator_element_type(&self, var_name: &str) -> Type {
        for constraint in &self.context.constraints {
            if let TypeConstraint::Compatible { var, ty } = constraint {
                if var == var_name {
                    if let Some(elem_type) = self.extract_element_type(ty) {
                        return elem_type;
                    }
                }
            }
        }
        Type::Unknown
    }

    fn extract_element_type(&self, ty: &Type) -> Option<Type> {
        match ty {
            Type::List(elem) => Some((**elem).clone()),
            Type::String => Some(Type::String),
            _ => None,
        }
    }

    fn type_to_annotation(&self, ty: &Type) -> String {
        type_to_annotation_inner(ty)
    }
}

fn type_to_annotation_inner(ty: &Type) -> String {
    match ty {
        Type::Int | Type::Float | Type::String | Type::Bool | Type::None => {
            simple_type_annotation(ty)
        }
        Type::List(elem) => format_list_annotation(elem),
        Type::Dict(k, v) => format_dict_annotation(k, v),
        Type::Optional(inner) => format_optional_annotation(inner),
        Type::Tuple(types) => format_tuple_annotation(types),
        Type::Custom(name) => name.clone(),
        _ => "Any".to_string(),
    }
}

fn simple_type_annotation(ty: &Type) -> String {
    match ty {
        Type::Int => "int".to_string(),
        Type::Float => "float".to_string(),
        Type::String => "str".to_string(),
        Type::Bool => "bool".to_string(),
        Type::None => "None".to_string(),
        _ => "Any".to_string(),
    }
}

fn format_list_annotation(elem: &Type) -> String {
    format!("list[{}]", type_to_annotation_inner(elem))
}

fn format_dict_annotation(k: &Type, v: &Type) -> String {
    format!(
        "dict[{}, {}]",
        type_to_annotation_inner(k),
        type_to_annotation_inner(v)
    )
}

fn format_optional_annotation(inner: &Type) -> String {
    format!("Optional[{}]", type_to_annotation_inner(inner))
}

fn format_tuple_annotation(types: &[Type]) -> String {
    let type_strs: Vec<String> = types.iter().map(type_to_annotation_inner).collect();
    format!("tuple[{}]", type_strs.join(", "))
}

fn extract_variable_from_error(context: &str) -> Option<String> {
    // Simple pattern matching to extract variable names from error context
    if let Some(start) = context.find("variable '") {
        let after_quote = &context[start + 10..];
        if let Some(end) = after_quote.find('\'') {
            return Some(after_quote[..end].to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;
    use smallvec::smallvec;

    // ============================================================
    // Constructor and Default tests
    // ============================================================

    #[test]
    fn test_type_hint_provider_new() {
        let provider = TypeHintProvider::new();
        assert!(provider.variable_types.is_empty());
        assert!(provider.parameter_hints.is_empty());
        assert!(provider.return_hints.is_empty());
    }

    #[test]
    fn test_type_hint_provider_default() {
        let provider = TypeHintProvider::default();
        assert!(provider.variable_types.is_empty());
    }

    // ============================================================
    // Confidence enum tests
    // ============================================================

    #[test]
    fn test_confidence_ordering() {
        assert!(Confidence::Low < Confidence::Medium);
        assert!(Confidence::Medium < Confidence::High);
        assert!(Confidence::High < Confidence::Certain);
    }

    #[test]
    fn test_confidence_equality() {
        assert_eq!(Confidence::Low, Confidence::Low);
        assert_eq!(Confidence::Certain, Confidence::Certain);
        assert_ne!(Confidence::Low, Confidence::High);
    }

    #[test]
    fn test_confidence_clone() {
        let c = Confidence::High;
        let c2 = c;
        assert_eq!(c, c2);
    }

    #[test]
    fn test_confidence_debug() {
        let debug_str = format!("{:?}", Confidence::Medium);
        assert_eq!(debug_str, "Medium");
    }

    // ============================================================
    // HintTarget enum tests
    // ============================================================

    #[test]
    fn test_hint_target_parameter() {
        let target = HintTarget::Parameter("x".to_string());
        assert!(matches!(target, HintTarget::Parameter(_)));
    }

    #[test]
    fn test_hint_target_return() {
        let target = HintTarget::Return;
        assert!(matches!(target, HintTarget::Return));
    }

    #[test]
    fn test_hint_target_variable() {
        let target = HintTarget::Variable("y".to_string());
        assert!(matches!(target, HintTarget::Variable(_)));
    }

    #[test]
    fn test_hint_target_clone() {
        let target = HintTarget::Parameter("foo".to_string());
        let cloned = target.clone();
        assert!(matches!(cloned, HintTarget::Parameter(s) if s == "foo"));
    }

    // ============================================================
    // TypeHint struct tests
    // ============================================================

    #[test]
    fn test_type_hint_creation() {
        let hint = TypeHint {
            suggested_type: Type::Int,
            confidence: Confidence::High,
            reason: "test reason".to_string(),
            source_location: Some((10, 5)),
            target: HintTarget::Parameter("x".to_string()),
        };
        assert!(matches!(hint.suggested_type, Type::Int));
        assert_eq!(hint.confidence, Confidence::High);
        assert_eq!(hint.reason, "test reason");
        assert_eq!(hint.source_location, Some((10, 5)));
    }

    #[test]
    fn test_type_hint_clone() {
        let hint = TypeHint {
            suggested_type: Type::String,
            confidence: Confidence::Low,
            reason: "clone test".to_string(),
            source_location: None,
            target: HintTarget::Return,
        };
        let cloned = hint.clone();
        assert!(matches!(cloned.suggested_type, Type::String));
        assert_eq!(cloned.confidence, Confidence::Low);
    }

    // ============================================================
    // Function analysis tests
    // ============================================================

    #[test]
    fn test_analyze_simple_function() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "add_numbers".to_string(),
            params: smallvec![
                HirParam::new("a".to_string(), Type::Unknown),
                HirParam::new("b".to_string(), Type::Unknown),
            ],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let hints = provider.analyze_function(&func).unwrap();
        assert!(!hints.is_empty());

        let param_hints = provider.parameter_hints.get("add_numbers").unwrap();
        assert!(param_hints
            .iter()
            .any(|h| matches!(h.suggested_type, Type::Int)));
    }

    #[test]
    fn test_string_method_inference() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "process_text".to_string(),
            params: smallvec![HirParam::new("text".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "upper".to_string(),
                args: vec![],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();

        let param_hints = provider.parameter_hints.get("process_text").unwrap();
        assert!(param_hints
            .iter()
            .any(|h| matches!(h.suggested_type, Type::String)));
    }

    #[test]
    fn test_literal_assignment_inference() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "test_literals".to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Literal(Literal::Int(42)),
                    type_annotation: None,
                },
                HirStmt::Assign {
                    target: AssignTarget::Symbol("y".to_string()),
                    value: HirExpr::Literal(Literal::String("hello".to_string())),
                    type_annotation: None,
                },
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let _hints = provider.analyze_function(&func).unwrap();
    }

    #[test]
    fn test_analyze_empty_function() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "empty".to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let hints = provider.analyze_function(&func).unwrap();
        assert!(hints.is_empty());
    }

    #[test]
    fn test_analyze_function_with_known_types() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "typed".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let hints = provider.analyze_function(&func).unwrap();
        // Known types should not generate hints
        assert!(hints.is_empty() || hints.iter().all(|h| !matches!(h.target, HintTarget::Parameter(_))));
    }

    // ============================================================
    // Format hints tests
    // ============================================================

    #[test]
    fn test_format_hints() {
        let provider = TypeHintProvider::new();

        let hints = vec![
            TypeHint {
                suggested_type: Type::Int,
                confidence: Confidence::High,
                reason: "numeric operations".to_string(),
                source_location: Some((10, 5)),
                target: HintTarget::Parameter("x".to_string()),
            },
            TypeHint {
                suggested_type: Type::String,
                confidence: Confidence::Medium,
                reason: "string methods".to_string(),
                source_location: None,
                target: HintTarget::Variable("text".to_string()),
            },
        ];

        let formatted = provider.format_hints(&hints);
        assert!(formatted.contains("Hint:"));
        assert!(formatted.contains("numeric operations"));
        assert!(formatted.contains("line 10, column 5"));
    }

    #[test]
    fn test_format_hints_empty() {
        let provider = TypeHintProvider::new();
        let hints: Vec<TypeHint> = vec![];
        let formatted = provider.format_hints(&hints);
        assert!(formatted.is_empty());
    }

    #[test]
    fn test_format_hints_return_target() {
        let provider = TypeHintProvider::new();

        let hints = vec![TypeHint {
            suggested_type: Type::Bool,
            confidence: Confidence::Certain,
            reason: "return value".to_string(),
            source_location: None,
            target: HintTarget::Return,
        }];

        let formatted = provider.format_hints(&hints);
        assert!(formatted.contains("return type"));
    }

    // ============================================================
    // Type annotation formatting tests
    // ============================================================

    #[test]
    fn test_type_to_annotation_primitives() {
        assert_eq!(type_to_annotation_inner(&Type::Int), "int");
        assert_eq!(type_to_annotation_inner(&Type::Float), "float");
        assert_eq!(type_to_annotation_inner(&Type::String), "str");
        assert_eq!(type_to_annotation_inner(&Type::Bool), "bool");
        assert_eq!(type_to_annotation_inner(&Type::None), "None");
    }

    #[test]
    fn test_type_to_annotation_list() {
        let list_type = Type::List(Box::new(Type::Int));
        assert_eq!(type_to_annotation_inner(&list_type), "list[int]");
    }

    #[test]
    fn test_type_to_annotation_dict() {
        let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        assert_eq!(type_to_annotation_inner(&dict_type), "dict[str, int]");
    }

    #[test]
    fn test_type_to_annotation_optional() {
        let opt_type = Type::Optional(Box::new(Type::String));
        assert_eq!(type_to_annotation_inner(&opt_type), "Optional[str]");
    }

    #[test]
    fn test_type_to_annotation_tuple() {
        let tuple_type = Type::Tuple(vec![Type::Int, Type::String, Type::Bool]);
        assert_eq!(type_to_annotation_inner(&tuple_type), "tuple[int, str, bool]");
    }

    #[test]
    fn test_type_to_annotation_custom() {
        let custom = Type::Custom("MyClass".to_string());
        assert_eq!(type_to_annotation_inner(&custom), "MyClass");
    }

    #[test]
    fn test_type_to_annotation_unknown() {
        assert_eq!(type_to_annotation_inner(&Type::Unknown), "Any");
    }

    #[test]
    fn test_type_to_annotation_nested() {
        let nested = Type::List(Box::new(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int),
        )));
        assert_eq!(type_to_annotation_inner(&nested), "list[dict[str, int]]");
    }

    // ============================================================
    // Variable extraction tests
    // ============================================================

    #[test]
    fn test_extract_variable_from_error() {
        assert_eq!(
            extract_variable_from_error("undefined variable 'test_var'"),
            Some("test_var".to_string())
        );
        assert_eq!(extract_variable_from_error("no variable here"), None);
    }

    #[test]
    fn test_extract_variable_from_error_complex() {
        assert_eq!(
            extract_variable_from_error("Error: cannot find variable 'my_value' in scope"),
            Some("my_value".to_string())
        );
    }

    #[test]
    fn test_extract_variable_from_error_empty() {
        assert_eq!(extract_variable_from_error(""), None);
    }

    #[test]
    fn test_extract_variable_no_closing_quote() {
        assert_eq!(extract_variable_from_error("variable 'incomplete"), None);
    }

    // ============================================================
    // Literal type inference tests
    // ============================================================

    #[test]
    fn test_literal_to_type_int() {
        let provider = TypeHintProvider::new();
        let ty = provider.literal_to_type(&Literal::Int(42));
        assert!(matches!(ty, Type::Int));
    }

    #[test]
    fn test_literal_to_type_float() {
        let provider = TypeHintProvider::new();
        let ty = provider.literal_to_type(&Literal::Float(3.15));
        assert!(matches!(ty, Type::Float));
    }

    #[test]
    fn test_literal_to_type_string() {
        let provider = TypeHintProvider::new();
        let ty = provider.literal_to_type(&Literal::String("hello".to_string()));
        assert!(matches!(ty, Type::String));
    }

    #[test]
    fn test_literal_to_type_bool() {
        let provider = TypeHintProvider::new();
        let ty = provider.literal_to_type(&Literal::Bool(true));
        assert!(matches!(ty, Type::Bool));
    }

    #[test]
    fn test_literal_to_type_none() {
        let provider = TypeHintProvider::new();
        let ty = provider.literal_to_type(&Literal::None);
        assert!(matches!(ty, Type::None));
    }

    #[test]
    fn test_literal_to_type_bytes() {
        let provider = TypeHintProvider::new();
        let ty = provider.literal_to_type(&Literal::Bytes(vec![1, 2, 3]));
        assert!(matches!(ty, Type::Custom(s) if s == "bytes"));
    }

    // ============================================================
    // Score to confidence tests
    // ============================================================

    #[test]
    fn test_score_to_confidence_low() {
        let provider = TypeHintProvider::new();
        assert_eq!(provider.score_to_confidence(0), Confidence::Low);
        assert_eq!(provider.score_to_confidence(1), Confidence::Low);
    }

    #[test]
    fn test_score_to_confidence_medium() {
        let provider = TypeHintProvider::new();
        assert_eq!(provider.score_to_confidence(2), Confidence::Medium);
        assert_eq!(provider.score_to_confidence(3), Confidence::Medium);
    }

    #[test]
    fn test_score_to_confidence_high() {
        let provider = TypeHintProvider::new();
        assert_eq!(provider.score_to_confidence(4), Confidence::High);
        assert_eq!(provider.score_to_confidence(5), Confidence::High);
    }

    #[test]
    fn test_score_to_confidence_certain() {
        let provider = TypeHintProvider::new();
        assert_eq!(provider.score_to_confidence(6), Confidence::Certain);
        assert_eq!(provider.score_to_confidence(100), Confidence::Certain);
    }

    // ============================================================
    // Statement analysis tests
    // ============================================================

    #[test]
    fn test_analyze_if_statement() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "test_if".to_string(),
            params: smallvec![HirParam::new("flag".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::If {
                condition: HirExpr::Var("flag".to_string()),
                then_body: vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(1)))],
                else_body: Some(vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(2)))]),
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let hints = provider.analyze_function(&func).unwrap();
        // Should infer bool from condition usage
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::Bool)));
    }

    #[test]
    fn test_analyze_while_statement() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "test_while".to_string(),
            params: smallvec![HirParam::new("condition".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::While {
                condition: HirExpr::Var("condition".to_string()),
                body: vec![],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let hints = provider.analyze_function(&func).unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::Bool)));
    }

    #[test]
    fn test_analyze_for_range_statement() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "test_for".to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(10))],
                    kwargs: vec![],
                },
                body: vec![],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let _hints = provider.analyze_function(&func).unwrap();
        // Loop variable from range should be Int
    }

    #[test]
    fn test_analyze_return_statement() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "get_number".to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let hints = provider.analyze_function(&func).unwrap();
        assert!(hints.iter().any(|h| matches!(h.target, HintTarget::Return)));
    }

    // ============================================================
    // Binary operation analysis tests
    // ============================================================

    #[test]
    fn test_analyze_subtraction() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "subtract".to_string(),
            params: smallvec![
                HirParam::new("a".to_string(), Type::Unknown),
                HirParam::new("b".to_string(), Type::Unknown),
            ],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::Binary {
                op: BinOp::Sub,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("subtract").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::Int)));
    }

    #[test]
    fn test_analyze_multiplication() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "multiply".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("multiply").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::Int)));
    }

    #[test]
    fn test_analyze_division() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "divide".to_string(),
            params: smallvec![HirParam::new("n".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::Binary {
                op: BinOp::Div,
                left: Box::new(HirExpr::Var("n".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("divide").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::Int)));
    }

    // ============================================================
    // Method call analysis tests
    // ============================================================

    #[test]
    fn test_analyze_lower_method() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "to_lower".to_string(),
            params: smallvec![HirParam::new("s".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("s".to_string())),
                method: "lower".to_string(),
                args: vec![],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("to_lower").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::String)));
    }

    #[test]
    fn test_analyze_strip_method() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "clean".to_string(),
            params: smallvec![HirParam::new("text".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "strip".to_string(),
                args: vec![],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("clean").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::String)));
    }

    #[test]
    fn test_analyze_split_method() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "tokenize".to_string(),
            params: smallvec![HirParam::new("line".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("line".to_string())),
                method: "split".to_string(),
                args: vec![HirExpr::Literal(Literal::String(" ".to_string()))],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("tokenize").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::String)));
    }

    #[test]
    fn test_analyze_append_method() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "add_item".to_string(),
            params: smallvec![HirParam::new("items".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("items".to_string())),
                method: "append".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("add_item").unwrap();
        // Should infer list type from append
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::List(_))));
    }

    // ============================================================
    // Builtin call analysis tests
    // ============================================================

    #[test]
    fn test_analyze_len_call() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "get_length".to_string(),
            params: smallvec![HirParam::new("data".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("data".to_string())],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("get_length").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::List(_))));
    }

    #[test]
    fn test_analyze_open_call() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "read_file".to_string(),
            params: smallvec![HirParam::new("path".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Var("path".to_string())],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("read_file").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::String)));
    }

    #[test]
    fn test_analyze_int_conversion() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "parse_int".to_string(),
            params: smallvec![HirParam::new("value".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "int".to_string(),
                args: vec![HirExpr::Var("value".to_string())],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("parse_int").unwrap();
        // int(value) means value is a string
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::String)));
    }

    // ============================================================
    // Indexing analysis tests
    // ============================================================

    #[test]
    fn test_analyze_integer_indexing() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "get_first".to_string(),
            params: smallvec![HirParam::new("items".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::Index {
                base: Box::new(HirExpr::Var("items".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("get_first").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::List(_))));
    }

    #[test]
    fn test_analyze_string_key_indexing() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "get_value".to_string(),
            params: smallvec![HirParam::new("config".to_string(), Type::Unknown)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::Index {
                base: Box::new(HirExpr::Var("config".to_string())),
                index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();
        let hints = provider.parameter_hints.get("get_value").unwrap();
        assert!(hints.iter().any(|h| matches!(h.suggested_type, Type::Dict(_, _))));
    }

    // ============================================================
    // Collection type inference tests
    // ============================================================

    #[test]
    fn test_infer_list_assignment() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "make_list".to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("nums".to_string()),
                value: HirExpr::List(vec![
                    HirExpr::Literal(Literal::Int(1)),
                    HirExpr::Literal(Literal::Int(2)),
                ]),
                type_annotation: None,
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let _hints = provider.analyze_function(&func).unwrap();
    }

    #[test]
    fn test_infer_dict_assignment() {
        let mut provider = TypeHintProvider::new();

        let func = HirFunction {
            name: "make_dict".to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("data".to_string()),
                value: HirExpr::Dict(vec![(
                    HirExpr::Literal(Literal::String("key".to_string())),
                    HirExpr::Literal(Literal::Int(42)),
                )]),
                type_annotation: None,
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let _hints = provider.analyze_function(&func).unwrap();
    }

    // ============================================================
    // Default value inference tests
    // ============================================================

    #[test]
    fn test_infer_from_default_int() {
        let provider = TypeHintProvider::new();
        let hint = provider.infer_from_default("x", &Some(HirExpr::Literal(Literal::Int(42))));
        assert!(hint.is_some());
        let h = hint.unwrap();
        assert!(matches!(h.suggested_type, Type::Int));
        assert_eq!(h.confidence, Confidence::Certain);
    }

    #[test]
    fn test_infer_from_default_bool() {
        let provider = TypeHintProvider::new();
        let hint = provider.infer_from_default("flag", &Some(HirExpr::Literal(Literal::Bool(false))));
        assert!(hint.is_some());
        let h = hint.unwrap();
        assert!(matches!(h.suggested_type, Type::Bool));
    }

    #[test]
    fn test_infer_from_default_string() {
        let provider = TypeHintProvider::new();
        let hint = provider.infer_from_default("name", &Some(HirExpr::Literal(Literal::String("default".to_string()))));
        assert!(hint.is_some());
        let h = hint.unwrap();
        assert!(matches!(h.suggested_type, Type::String));
    }

    #[test]
    fn test_infer_from_default_none() {
        let provider = TypeHintProvider::new();
        let hint = provider.infer_from_default("opt", &Some(HirExpr::Literal(Literal::None)));
        assert!(hint.is_none()); // None doesn't give type info
    }

    #[test]
    fn test_infer_from_default_missing() {
        let provider = TypeHintProvider::new();
        let hint = provider.infer_from_default("x", &None);
        assert!(hint.is_none());
    }

    // ============================================================
    // Expression type inference tests
    // ============================================================

    #[test]
    fn test_infer_expr_type_literal() {
        let provider = TypeHintProvider::new();
        let ty = provider.infer_expr_type(&HirExpr::Literal(Literal::Float(1.0)));
        assert!(matches!(ty, Type::Float));
    }

    #[test]
    fn test_infer_expr_type_list() {
        let provider = TypeHintProvider::new();
        let ty = provider.infer_expr_type(&HirExpr::List(vec![
            HirExpr::Literal(Literal::String("a".to_string())),
        ]));
        assert!(matches!(ty, Type::List(_)));
    }

    #[test]
    fn test_infer_expr_type_dict() {
        let provider = TypeHintProvider::new();
        let ty = provider.infer_expr_type(&HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("k".to_string())),
            HirExpr::Literal(Literal::Int(1)),
        )]));
        assert!(matches!(ty, Type::Dict(_, _)));
    }

    #[test]
    fn test_infer_expr_type_tuple() {
        let provider = TypeHintProvider::new();
        let ty = provider.infer_expr_type(&HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("hello".to_string())),
        ]));
        match ty {
            Type::Tuple(types) => {
                assert_eq!(types.len(), 2);
                assert!(matches!(types[0], Type::Int));
                assert!(matches!(types[1], Type::String));
            }
            _ => panic!("Expected Tuple type"),
        }
    }

    #[test]
    fn test_infer_expr_type_unknown_var() {
        let provider = TypeHintProvider::new();
        let ty = provider.infer_expr_type(&HirExpr::Var("unknown".to_string()));
        assert!(matches!(ty, Type::Unknown));
    }

    // ============================================================
    // Collection element type tests with None
    // ============================================================

    #[test]
    fn test_infer_collection_with_none() {
        let provider = TypeHintProvider::new();
        let ty = provider.infer_collection_element_type(&[
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::None),
        ]);
        // Should be Optional[Int]
        assert!(matches!(ty, Type::Optional(_)));
    }

    #[test]
    fn test_infer_collection_all_same_type() {
        let provider = TypeHintProvider::new();
        let ty = provider.infer_collection_element_type(&[
            HirExpr::Literal(Literal::String("a".to_string())),
            HirExpr::Literal(Literal::String("b".to_string())),
        ]);
        assert!(matches!(ty, Type::String));
    }

    #[test]
    fn test_infer_collection_empty() {
        let provider = TypeHintProvider::new();
        let ty = provider.infer_collection_element_type(&[]);
        assert!(matches!(ty, Type::Unknown));
    }

    // ============================================================
    // Helper function tests
    // ============================================================

    #[test]
    fn test_simple_type_annotation() {
        assert_eq!(simple_type_annotation(&Type::Int), "int");
        assert_eq!(simple_type_annotation(&Type::Float), "float");
        assert_eq!(simple_type_annotation(&Type::String), "str");
        assert_eq!(simple_type_annotation(&Type::Bool), "bool");
        assert_eq!(simple_type_annotation(&Type::None), "None");
        assert_eq!(simple_type_annotation(&Type::Unknown), "Any");
    }

    #[test]
    fn test_format_list_annotation() {
        let result = format_list_annotation(&Type::Int);
        assert_eq!(result, "list[int]");
    }

    #[test]
    fn test_format_dict_annotation() {
        let result = format_dict_annotation(&Type::String, &Type::Int);
        assert_eq!(result, "dict[str, int]");
    }

    #[test]
    fn test_format_optional_annotation() {
        let result = format_optional_annotation(&Type::String);
        assert_eq!(result, "Optional[str]");
    }

    #[test]
    fn test_format_tuple_annotation() {
        let result = format_tuple_annotation(&[Type::Int, Type::Bool]);
        assert_eq!(result, "tuple[int, bool]");
    }

    #[test]
    fn test_format_tuple_annotation_empty() {
        let result = format_tuple_annotation(&[]);
        assert_eq!(result, "tuple[]");
    }

    // ============================================================
    // Confidence color tests
    // ============================================================

    #[test]
    fn test_get_confidence_color() {
        let provider = TypeHintProvider::new();
        assert_eq!(provider.get_confidence_color(Confidence::Certain), "green");
        assert_eq!(provider.get_confidence_color(Confidence::High), "bright green");
        assert_eq!(provider.get_confidence_color(Confidence::Medium), "yellow");
        assert_eq!(provider.get_confidence_color(Confidence::Low), "bright yellow");
    }

    // ============================================================
    // Target formatting tests
    // ============================================================

    #[test]
    fn test_format_target_parameter() {
        let provider = TypeHintProvider::new();
        let result = provider.format_target(&HintTarget::Parameter("x".to_string()));
        assert_eq!(result, "parameter 'x'");
    }

    #[test]
    fn test_format_target_return() {
        let provider = TypeHintProvider::new();
        let result = provider.format_target(&HintTarget::Return);
        assert_eq!(result, "return type");
    }

    #[test]
    fn test_format_target_variable() {
        let provider = TypeHintProvider::new();
        let result = provider.format_target(&HintTarget::Variable("y".to_string()));
        assert_eq!(result, "variable 'y'");
    }

    // === DEPYLER-COVERAGE-95: Additional tests for untested internal types ===

    #[test]
    fn test_type_constraint_compatible_debug() {
        let constraint = TypeConstraint::Compatible {
            var: "x".to_string(),
            ty: Type::Int,
        };
        let debug = format!("{:?}", constraint);
        assert!(debug.contains("Compatible"));
        assert!(debug.contains("x"));
    }

    #[test]
    fn test_type_constraint_compatible_clone() {
        let constraint = TypeConstraint::Compatible {
            var: "value".to_string(),
            ty: Type::String,
        };
        let cloned = constraint.clone();
        if let TypeConstraint::Compatible { var, ty } = cloned {
            assert_eq!(var, "value");
            assert_eq!(ty, Type::String);
        } else {
            panic!("Expected Compatible variant");
        }
    }

    #[test]
    fn test_type_constraint_argument_debug() {
        let constraint = TypeConstraint::ArgumentConstraint {
            var: "arg".to_string(),
            func: "process".to_string(),
            _param_idx: 0,
            expected: Type::Float,
        };
        let debug = format!("{:?}", constraint);
        assert!(debug.contains("ArgumentConstraint"));
        assert!(debug.contains("arg"));
        assert!(debug.contains("process"));
    }

    #[test]
    fn test_type_constraint_argument_clone() {
        let constraint = TypeConstraint::ArgumentConstraint {
            var: "data".to_string(),
            func: "len".to_string(),
            _param_idx: 1,
            expected: Type::List(Box::new(Type::Int)),
        };
        let cloned = constraint.clone();
        if let TypeConstraint::ArgumentConstraint { var, func, _param_idx, expected } = cloned {
            assert_eq!(var, "data");
            assert_eq!(func, "len");
            assert_eq!(_param_idx, 1);
            assert!(matches!(expected, Type::List(_)));
        } else {
            panic!("Expected ArgumentConstraint variant");
        }
    }

    #[test]
    fn test_type_constraint_return_debug() {
        let constraint = TypeConstraint::ReturnConstraint {
            var: "result".to_string(),
            ty: Type::Bool,
        };
        let debug = format!("{:?}", constraint);
        assert!(debug.contains("ReturnConstraint"));
        assert!(debug.contains("result"));
    }

    #[test]
    fn test_type_constraint_return_clone() {
        let constraint = TypeConstraint::ReturnConstraint {
            var: "output".to_string(),
            ty: Type::String,
        };
        let cloned = constraint.clone();
        if let TypeConstraint::ReturnConstraint { var, ty } = cloned {
            assert_eq!(var, "output");
            assert_eq!(ty, Type::String);
        } else {
            panic!("Expected ReturnConstraint variant");
        }
    }

    #[test]
    fn test_usage_pattern_iterator_debug() {
        let pattern = UsagePattern::Iterator;
        let debug = format!("{:?}", pattern);
        assert!(debug.contains("Iterator"));
    }

    #[test]
    fn test_usage_pattern_numeric_clone() {
        let pattern = UsagePattern::Numeric;
        let cloned = pattern.clone();
        assert!(matches!(cloned, UsagePattern::Numeric));
    }

    #[test]
    fn test_usage_pattern_string_like() {
        let pattern = UsagePattern::StringLike;
        let debug = format!("{:?}", pattern);
        assert!(debug.contains("StringLike"));
    }

    #[test]
    fn test_usage_pattern_container() {
        let pattern = UsagePattern::Container;
        let cloned = pattern.clone();
        assert!(matches!(cloned, UsagePattern::Container));
    }

    #[test]
    fn test_usage_pattern_dict_access() {
        let pattern = UsagePattern::DictAccess;
        let debug = format!("{:?}", pattern);
        assert!(debug.contains("DictAccess"));
    }

    #[test]
    fn test_usage_pattern_all_variants() {
        // Verify all variants can be created and debugged
        let patterns = [
            UsagePattern::Iterator,
            UsagePattern::Numeric,
            UsagePattern::StringLike,
            UsagePattern::Container,
            UsagePattern::DictAccess,
            UsagePattern::Callable,
        ];
        for pattern in patterns {
            let debug = format!("{:?}", pattern);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_inference_context_default() {
        let context: InferenceContext = Default::default();
        assert!(context.current_function.is_none());
        assert!(context.constraints.is_empty());
        assert!(context.usage_patterns.is_empty());
        assert!(context.loop_var_sources.is_empty());
        assert!(context.non_list_variables.is_empty());
    }

    #[test]
    fn test_inference_context_debug() {
        let context = InferenceContext::default();
        let debug = format!("{:?}", context);
        assert!(debug.contains("InferenceContext"));
    }

    #[test]
    fn test_hint_target_debug_all_variants() {
        let targets = [
            HintTarget::Parameter("arg".to_string()),
            HintTarget::Return,
            HintTarget::Variable("x".to_string()),
        ];
        for target in targets {
            let debug = format!("{:?}", target);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_confidence_copy() {
        let c1 = Confidence::High;
        let c2 = c1; // Copy trait
        assert_eq!(c1, c2);
        // Can still use c1 after copy
        let _c3 = c1;
    }

    #[test]
    fn test_confidence_ord_full() {
        // Test full ordering chain
        assert!(Confidence::Certain > Confidence::High);
        assert!(Confidence::High > Confidence::Medium);
        assert!(Confidence::Medium > Confidence::Low);
        assert!(Confidence::Certain >= Confidence::Certain);
        assert!(Confidence::Low <= Confidence::Medium);
    }

    #[test]
    fn test_type_hint_debug() {
        let hint = TypeHint {
            suggested_type: Type::Int,
            confidence: Confidence::High,
            reason: "Used in arithmetic".to_string(),
            source_location: Some((10, 5)),
            target: HintTarget::Variable("x".to_string()),
        };
        let debug = format!("{:?}", hint);
        assert!(debug.contains("TypeHint"));
        assert!(debug.contains("Int"));
        assert!(debug.contains("High"));
    }

    #[test]
    fn test_type_hint_with_no_location() {
        let hint = TypeHint {
            suggested_type: Type::String,
            confidence: Confidence::Medium,
            reason: "Inferred from method".to_string(),
            source_location: None,
            target: HintTarget::Return,
        };
        assert!(hint.source_location.is_none());
        assert_eq!(hint.confidence, Confidence::Medium);
    }

    #[test]
    fn test_type_hint_provider_fields() {
        let provider = TypeHintProvider::new();
        // Verify initial state
        assert!(provider.variable_types.is_empty());
        assert!(provider.parameter_hints.is_empty());
        assert!(provider.return_hints.is_empty());
    }
}
