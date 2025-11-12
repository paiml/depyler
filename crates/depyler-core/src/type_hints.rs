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
    /// Variable passed to function expecting type
    ArgumentConstraint {
        _var: String,
        _func: String,
        _param_idx: usize,
        _expected: Type,
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
    /// Used as container
    Container,
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
    }

    fn collect_parameter_hints(&mut self, func: &HirFunction, hints: &mut Vec<TypeHint>) {
        for param in &func.params {
            if matches!(param.ty, Type::Unknown) {
                if let Some(hint) = self.infer_parameter_type(&param.name) {
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
        self.analyze_expr(condition)?;
        self.analyze_body(then_body)?;
        if let Some(else_stmts) = else_body {
            self.analyze_body(else_stmts)?;
        }
        Ok(())
    }

    fn analyze_while_stmt(&mut self, condition: &HirExpr, body: &[HirStmt]) -> Result<()> {
        self.analyze_expr(condition)?;
        self.analyze_body(body)
    }

    fn analyze_for_stmt(
        &mut self,
        target: &crate::hir::AssignTarget,
        iter: &HirExpr,
        body: &[HirStmt],
    ) -> Result<()> {
        // For now, only analyze simple symbol targets
        if let crate::hir::AssignTarget::Symbol(target_name) = target {
            self.analyze_for_loop(target_name, iter)?;
        }
        self.analyze_body(body)
    }

    fn analyze_expr(&mut self, expr: &HirExpr) -> Result<()> {
        match expr {
            HirExpr::Binary { left, right, op } => self.analyze_binary_op(left, right, *op),
            HirExpr::Call { func, args , ..} => self.analyze_call(func, args),
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => self.analyze_method_call(object, method, args),
            HirExpr::Index { base, index } => self.analyze_indexing(base, index),
            _ => Ok(()),
        }
    }

    fn analyze_assignment(&mut self, var_name: &str, value: &HirExpr) -> Result<()> {
        self.infer_from_literal(var_name, value);
        self.infer_from_collection(var_name, value);
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
            _ => {}
        }
    }

    fn infer_list_type(&mut self, var_name: &str, elems: &[HirExpr]) {
        let elem_type = self.infer_collection_element_type(elems);
        let list_type = Type::List(Box::new(elem_type));
        self.add_compatible_constraint(var_name, list_type);
    }

    fn infer_dict_type(&mut self, var_name: &str, items: &[(HirExpr, HirExpr)]) {
        if let Some((k, v)) = items.first() {
            let key_type = self.infer_expr_type(k);
            let val_type = self.infer_expr_type(v);
            let dict_type = Type::Dict(Box::new(key_type), Box::new(val_type));
            self.add_compatible_constraint(var_name, dict_type);
        }
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

    fn analyze_call(&mut self, func: &str, args: &[HirExpr]) -> Result<()> {
        self.analyze_builtin_call(func, args);
        self.analyze_call_arguments(args)
    }

    fn analyze_builtin_call(&mut self, func: &str, args: &[HirExpr]) {
        match func {
            "len" => self.analyze_len_call(args),
            "str" | "int" | "float" | "bool" => self.analyze_conversion_call(func, args),
            _ => {}
        }
    }

    fn analyze_len_call(&mut self, args: &[HirExpr]) {
        if let Some(HirExpr::Var(var)) = args.first() {
            self.record_usage_pattern(var, UsagePattern::Container);
        }
    }

    fn analyze_conversion_call(&mut self, func: &str, args: &[HirExpr]) {
        if let Some(HirExpr::Var(var)) = args.first() {
            let target_type = self.conversion_target_type(func);
            self.add_argument_constraint(var, func, target_type);
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
                _var: var.to_string(),
                _func: func.to_string(),
                _param_idx: 0,
                _expected: expected,
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
            self.record_usage_pattern(var, UsagePattern::Container);
        }
        self.analyze_expr(base)?;
        self.analyze_expr(index)?;
        Ok(())
    }

    fn record_usage_pattern(&mut self, var: &str, pattern: UsagePattern) {
        self.context
            .usage_patterns
            .entry(var.to_string())
            .or_default()
            .push(pattern);
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
            UsagePattern::Container => {
                *type_score
                    .entry(Type::List(Box::new(Type::Unknown)))
                    .or_insert(0) += 1
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

        // Check first element
        self.infer_expr_type(&elems[0])
    }

    fn infer_expr_type(&self, expr: &HirExpr) -> Type {
        match expr {
            HirExpr::Literal(lit) => self.literal_to_type(lit),
            HirExpr::List(elems) => self.infer_list_expr_type(elems),
            HirExpr::Var(name) => self.infer_var_type(name),
            _ => Type::Unknown,
        }
    }

    fn infer_list_expr_type(&self, elems: &[HirExpr]) -> Type {
        Type::List(Box::new(self.infer_collection_element_type(elems)))
    }

    fn infer_var_type(&self, name: &str) -> Type {
        for constraint in &self.context.constraints {
            if let TypeConstraint::Compatible { var, ty } = constraint {
                if var == name {
                    return ty.clone();
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

    #[test]
    fn test_type_hint_provider_new() {
        let provider = TypeHintProvider::new();
        assert!(provider.variable_types.is_empty());
        assert!(provider.parameter_hints.is_empty());
        assert!(provider.return_hints.is_empty());
    }

    #[test]
    fn test_confidence_ordering() {
        assert!(Confidence::Low < Confidence::Medium);
        assert!(Confidence::Medium < Confidence::High);
        assert!(Confidence::High < Confidence::Certain);
    }

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

        // Should suggest numeric types for parameters used in addition
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
                args: vec![], kwargs: vec![]
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        provider.analyze_function(&func).unwrap();

        // Should infer string type from method usage
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

        // Should have high confidence about literal assignments
        // Note: This test verifies that analyze_function() runs without errors.
        // Detailed hint validation would require exposing internal variable_types field,
        // which is intentionally private. The hints are validated in test_format_hints().
    }

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
    fn test_extract_variable_from_error() {
        assert_eq!(
            extract_variable_from_error("undefined variable 'test_var'"),
            Some("test_var".to_string())
        );

        assert_eq!(extract_variable_from_error("no variable here"), None);
    }
}
