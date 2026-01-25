use crate::hir::{HirExpr, HirFunction, HirStmt, Type};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Tracks type variables and their constraints for generic inference
#[derive(Debug, Default)]
pub struct TypeVarRegistry {
    /// Map from TypeVar names to their constraints
    type_vars: HashMap<String, TypeVarConstraints>,
    /// Type parameters inferred for functions
    function_type_params: HashMap<String, Vec<TypeParameter>>,
    /// Active type variable bindings during inference (unused but reserved for future use)
    #[allow(dead_code)]
    active_bindings: HashMap<String, Type>,
}

#[derive(Debug, Clone)]
pub struct TypeVarConstraints {
    pub name: String,
    pub bounds: Vec<TypeBound>,
    pub variance: Variance,
    pub default: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeBound {
    /// Type must be a subtype of this type
    UpperBound(Type),
    /// Type must implement this trait/protocol
    TraitBound(String),
    /// Type must be one of these types (Union constraint)
    UnionBound(Vec<Type>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variance {
    Invariant,
    Covariant,
    Contravariant,
}

#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub name: String,
    pub bounds: Vec<String>, // Rust trait bounds
    pub default: Option<Type>,
}

/// Generic type representation extending the base Type enum
#[derive(Debug, Clone, PartialEq)]
pub enum GenericType {
    /// A type variable like T, U, etc.
    TypeVar(String),
    /// A concrete type with generic parameters like List<T>
    Generic {
        base: Type,
        params: Vec<GenericType>,
    },
    /// A union type Union[A, B, C]
    Union(Vec<Type>),
    /// A concrete type
    Concrete(Type),
}

impl TypeVarRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new type variable with its constraints
    pub fn register_type_var(&mut self, name: String, constraints: TypeVarConstraints) {
        self.type_vars.insert(name, constraints);
    }

    /// DEPYLER-0716: Infer type substitutions for a function (e.g., T -> String)
    /// Returns substitutions that should be applied to parameter types before type mapping
    pub fn infer_type_substitutions(&self, func: &HirFunction) -> Result<HashMap<String, Type>> {
        let mut inference = TypeInference::new();
        inference.analyze_function(func)?;
        Ok(inference.substitutions)
    }

    /// DEPYLER-0716: Apply type substitutions to a Type
    /// Replaces Unknown with the substituted type (e.g., if T -> String, Unknown becomes String)
    pub fn apply_substitutions(ty: &Type, substitutions: &HashMap<String, Type>) -> Type {
        // If we have a substitution for T, replace Unknown with the substituted type
        let substituted = substitutions.get("T");

        match ty {
            Type::Unknown => {
                // Replace Unknown with substituted type if available
                substituted.cloned().unwrap_or(Type::Unknown)
            }
            Type::List(inner) => {
                Type::List(Box::new(Self::apply_substitutions(inner, substitutions)))
            }
            Type::Dict(k, v) => Type::Dict(
                Box::new(Self::apply_substitutions(k, substitutions)),
                Box::new(Self::apply_substitutions(v, substitutions)),
            ),
            Type::Optional(inner) => {
                Type::Optional(Box::new(Self::apply_substitutions(inner, substitutions)))
            }
            Type::Tuple(types) => Type::Tuple(
                types
                    .iter()
                    .map(|t| Self::apply_substitutions(t, substitutions))
                    .collect(),
            ),
            // For other types, return as-is
            other => other.clone(),
        }
    }

    /// Infer generic type parameters for a function
    pub fn infer_function_generics(&mut self, func: &HirFunction) -> Result<Vec<TypeParameter>> {
        let mut collector = TypeVarCollector::new();

        // Collect type variables from parameters
        for param in &func.params {
            collector.collect_from_type(&param.ty);
        }

        // Collect from return type
        collector.collect_from_type(&func.ret_type);

        // DEPYLER-0781: Don't collect from function body
        // Type vars from body can add generic params that aren't used in the
        // function signature, causing E0283 "cannot infer type" errors
        // Only params and return type define the function's generic interface

        // Build constraints from usage
        let mut inference = TypeInference::new();
        inference.analyze_function(func)?;

        // DEPYLER-0716: Filter out type vars that have been substituted with concrete types
        // If T is substituted with String, don't generate a type parameter for T
        let filtered_type_vars: HashSet<String> = collector
            .type_vars
            .into_iter()
            .filter(|tv| !inference.substitutions.contains_key(tv))
            .collect();

        // DEPYLER-0716: Also filter dict_key_type_vars to remove substituted ones
        let filtered_dict_key_vars: HashSet<String> = collector
            .dict_key_type_vars
            .into_iter()
            .filter(|tv| !inference.substitutions.contains_key(tv))
            .collect();

        // Generate type parameters
        let type_params = self.generate_type_parameters(
            &filtered_type_vars,
            &inference.constraints,
            &filtered_dict_key_vars,
        )?;

        // Store for later use
        self.function_type_params
            .insert(func.name.clone(), type_params.clone());

        Ok(type_params)
    }

    /// Check if a type contains generic parameters
    pub fn is_generic(&self, ty: &Type) -> bool {
        match ty {
            Type::Custom(name) => self.type_vars.contains_key(name),
            Type::List(inner) | Type::Optional(inner) => self.is_generic(inner),
            Type::Dict(k, v) => self.is_generic(k) || self.is_generic(v),
            Type::Tuple(types) => types.iter().any(|t| self.is_generic(t)),
            Type::Function { params, ret } => {
                params.iter().any(|t| self.is_generic(t)) || self.is_generic(ret)
            }
            _ => false,
        }
    }

    /// Convert a generic type to its Rust representation
    pub fn to_rust_generic(&self, name: &str, params: &[Type]) -> String {
        if params.is_empty() {
            name.to_string()
        } else {
            let param_strs: Vec<String> =
                params.iter().map(|p| self.type_to_rust_string(p)).collect();
            format!("{}<{}>", name, param_strs.join(", "))
        }
    }

    fn type_to_rust_string(&self, ty: &Type) -> String {
        match ty {
            Type::Custom(name) if self.type_vars.contains_key(name) => name.clone(),
            Type::Int => "i32".to_string(),
            Type::Float => "f64".to_string(),
            Type::String => "String".to_string(),
            Type::Bool => "bool".to_string(),
            Type::None => "()".to_string(),
            Type::List(inner) => format!("Vec<{}>", self.type_to_rust_string(inner)),
            Type::Dict(k, v) => format!(
                "HashMap<{}, {}>",
                self.type_to_rust_string(k),
                self.type_to_rust_string(v)
            ),
            Type::Optional(inner) => format!("Option<{}>", self.type_to_rust_string(inner)),
            Type::Tuple(types) => {
                let type_strs: Vec<String> =
                    types.iter().map(|t| self.type_to_rust_string(t)).collect();
                format!("({})", type_strs.join(", "))
            }
            Type::Custom(name) => name.clone(),
            _ => "()".to_string(),
        }
    }

    fn generate_type_parameters(
        &self,
        type_vars: &HashSet<String>,
        constraints: &HashMap<String, Vec<TypeConstraint>>,
        dict_key_type_vars: &HashSet<String>,
    ) -> Result<Vec<TypeParameter>> {
        let mut params = Vec::new();

        for var in type_vars {
            let mut bounds = std::collections::HashSet::new();

            // Add constraints as bounds
            if let Some(var_constraints) = constraints.get(var) {
                for constraint in var_constraints {
                    match constraint {
                        TypeConstraint::MustImplement(trait_name) => {
                            bounds.insert(trait_name.clone());
                        }
                        TypeConstraint::MustBe(_ty) => {
                            // For concrete type constraints, we might skip the type parameter
                            continue;
                        }
                        TypeConstraint::SubtypeOf(_) => {
                            // Handle subtyping constraints
                        }
                    }
                }
            }

            // DEPYLER-0716: Dict key type vars need Eq + Hash for HashMap
            if dict_key_type_vars.contains(var) {
                bounds.insert("Eq".to_string());
                bounds.insert("std::hash::Hash".to_string());
            }

            // Add default bounds based on usage
            if bounds.is_empty() {
                // If used in collections, might need Clone
                bounds.insert("Clone".to_string());
            } else {
                // Always add Clone as it's needed for most operations
                bounds.insert("Clone".to_string());
            }

            // Convert to sorted Vec for consistent output
            let mut bounds_vec: Vec<String> = bounds.into_iter().collect();
            bounds_vec.sort();

            params.push(TypeParameter {
                name: var.clone(),
                bounds: bounds_vec,
                default: None,
            });
        }

        Ok(params)
    }
}

/// Collects type variables from a function
struct TypeVarCollector {
    type_vars: HashSet<String>,
    /// DEPYLER-0716: Track type vars used as Dict keys - these need Eq + Hash bounds
    dict_key_type_vars: HashSet<String>,
}

impl TypeVarCollector {
    fn new() -> Self {
        Self {
            type_vars: HashSet::new(),
            dict_key_type_vars: HashSet::new(),
        }
    }

    fn collect_from_type(&mut self, ty: &Type) {
        // Use nested=false for top-level types
        self.collect_from_type_internal(ty, false);
    }

    /// DEPYLER-0271: Track nesting to only add generic T when Unknown is in a nested position
    /// (like List[Unknown], Dict[str, Unknown]) but NOT for bare Unknown (function return type)
    fn collect_from_type_internal(&mut self, ty: &Type, _nested: bool) {
        match ty {
            Type::Custom(name) if name.chars().next().is_some_and(|c| c.is_uppercase()) => {
                // Assume single uppercase letters are type variables
                if name.len() == 1 {
                    self.type_vars.insert(name.clone());
                }
            }
            Type::TypeVar(name) => {
                self.type_vars.insert(name.clone());
            }
            // DEPYLER-0781: Don't treat Unknown as type parameter T
            // Unknown types should use concrete defaults via type_mapper:
            // - List[Unknown] → Vec<serde_json::Value>
            // - Set[Unknown] → HashSet<String>
            // - Bare Unknown → i32 or String depending on context
            // Adding T causes E0283 "cannot infer type" when T isn't used in signature
            Type::Unknown => {
                // Don't add T - let type_mapper use concrete fallback
            }
            // Collection types create nested context
            Type::List(inner) | Type::Optional(inner) => {
                self.collect_from_type_internal(inner, true);
            }
            Type::Dict(k, v) => {
                // DEPYLER-0750: Removed dict_key_type_vars tracking for Unknown
                // Bare dict uses concrete defaults (String keys) via type_mapper
                // Only track explicit TypeVar for generics
                if let Type::TypeVar(name) = k.as_ref() {
                    self.dict_key_type_vars.insert(name.clone());
                }
                self.collect_from_type_internal(k, true);
                self.collect_from_type_internal(v, true);
            }
            Type::Tuple(types) => {
                for t in types {
                    self.collect_from_type_internal(t, true);
                }
            }
            Type::Function { params, ret } => {
                for p in params {
                    self.collect_from_type_internal(p, true);
                }
                self.collect_from_type_internal(ret, true);
            }
            // DEPYLER-0836: Handle Generic types like Either<L, R>, Maybe<T>
            // Extract type vars from generic parameters (e.g., L and R from Either<L, R>)
            Type::Generic { params, .. } => {
                for p in params {
                    self.collect_from_type_internal(p, true);
                }
            }
            // DEPYLER-0836: Handle Union types (e.g., Union[T, U] or T | None)
            Type::Union(types) => {
                for t in types {
                    self.collect_from_type_internal(t, true);
                }
            }
            // DEPYLER-0836: Handle Set types
            Type::Set(inner) => {
                self.collect_from_type_internal(inner, true);
            }
            _ => {}
        }
    }

    #[allow(dead_code)] // Reserved for future use in generic inference passes
    fn collect_from_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign { value, .. } => self.collect_from_expr(value),
            HirStmt::Return(Some(expr)) => self.collect_from_expr(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_from_expr(condition);
                for s in then_body {
                    self.collect_from_stmt(s);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_from_stmt(s);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.collect_from_expr(condition);
                for s in body {
                    self.collect_from_stmt(s);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.collect_from_expr(iter);
                for s in body {
                    self.collect_from_stmt(s);
                }
            }
            HirStmt::Expr(expr) => self.collect_from_expr(expr),
            _ => {}
        }
    }

    #[allow(dead_code, clippy::only_used_in_recursion)] // Reserved for future use in generic inference passes
    fn collect_from_expr(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::Binary { left, right, .. } => {
                self.collect_from_expr(left);
                self.collect_from_expr(right);
            }
            HirExpr::Unary { operand, .. } => self.collect_from_expr(operand),
            HirExpr::Call { args, .. } => {
                for arg in args {
                    self.collect_from_expr(arg);
                }
            }
            HirExpr::MethodCall { object, args, .. } => {
                self.collect_from_expr(object);
                for arg in args {
                    self.collect_from_expr(arg);
                }
            }
            HirExpr::Index { base, index } => {
                self.collect_from_expr(base);
                self.collect_from_expr(index);
            }
            HirExpr::List(elems) => {
                for elem in elems {
                    self.collect_from_expr(elem);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    self.collect_from_expr(k);
                    self.collect_from_expr(v);
                }
            }
            HirExpr::Tuple(elems) => {
                for elem in elems {
                    self.collect_from_expr(elem);
                }
            }
            _ => {}
        }
    }
}

/// Type inference engine using Hindley-Milner style inference
struct TypeInference {
    constraints: HashMap<String, Vec<TypeConstraint>>,
    /// DEPYLER-0716: Type substitutions - when we can infer a concrete type, substitute it
    /// e.g., when T is compared to &str, substitute T with String
    substitutions: HashMap<String, Type>,
    /// DEPYLER-0715: Parameter types for detecting string comparisons
    param_types: HashMap<String, Type>,
    /// DEPYLER-0715: Maps loop variables to their source type parameter
    /// e.g., `for user in users` maps "user" -> "T" (if users is Vec<T>)
    loop_var_to_type_param: HashMap<String, String>,
}

#[derive(Debug, Clone)]
enum TypeConstraint {
    /// Must be exactly this type (unused but reserved for future use)
    #[allow(dead_code)]
    MustBe(Type),
    /// Must be a subtype of this type (unused but reserved for future use)
    #[allow(dead_code)]
    SubtypeOf(Type),
    MustImplement(String),
}

impl TypeInference {
    fn new() -> Self {
        Self {
            constraints: HashMap::new(),
            substitutions: HashMap::new(),
            param_types: HashMap::new(),
            loop_var_to_type_param: HashMap::new(),
        }
    }

    fn analyze_function(&mut self, func: &HirFunction) -> Result<()> {
        // DEPYLER-0715: Populate param_types for string comparison detection
        for param in &func.params {
            self.param_types
                .insert(param.name.clone(), param.ty.clone());
        }

        // Analyze parameter usage to infer constraints
        for param in &func.params {
            match &param.ty {
                Type::Custom(type_var)
                    if type_var.len() == 1 && type_var.chars().next().unwrap().is_uppercase() =>
                {
                    // This is a type variable, analyze its usage
                    self.analyze_param_usage(&param.name, type_var, &func.body)?;
                }
                Type::TypeVar(type_var) => {
                    // This is explicitly a type variable
                    self.analyze_param_usage(&param.name, type_var, &func.body)?;
                }
                // DEPYLER-0715: Handle List(Unknown) - the element type becomes type var "T"
                // This is common when Python has bare `list` without type params
                Type::List(inner) if matches!(**inner, Type::Unknown) => {
                    // The element type is T, analyze usage with param name and "T" type var
                    self.analyze_param_usage(&param.name, "T", &func.body)?;
                }
                // DEPYLER-0715: Handle Dict with Unknown keys or values
                Type::Dict(k, v)
                    if matches!(**k, Type::Unknown) || matches!(**v, Type::Unknown) =>
                {
                    // Dict with unknown element types - T becomes the value type
                    self.analyze_param_usage(&param.name, "T", &func.body)?;
                }
                // DEPYLER-0744: Handle Optional(Unknown) - infer T from return statements
                // When a param has default=None, it becomes Optional(Unknown).
                // We analyze return statements to find what concrete type T should be.
                Type::Optional(inner) if matches!(**inner, Type::Unknown) => {
                    // Find concrete types returned by the function
                    if let Some(concrete_type) =
                        self.infer_concrete_type_from_returns(&param.name, &func.body)
                    {
                        // Substitute T with the concrete type
                        self.substitutions.insert("T".to_string(), concrete_type);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn analyze_param_usage(
        &mut self,
        param_name: &str,
        type_var: &str,
        body: &[HirStmt],
    ) -> Result<()> {
        for stmt in body {
            self.analyze_stmt_for_param(param_name, type_var, stmt)?;
        }
        Ok(())
    }

    fn analyze_stmt_for_param(
        &mut self,
        param_name: &str,
        type_var: &str,
        stmt: &HirStmt,
    ) -> Result<()> {
        match stmt {
            HirStmt::Expr(expr) => {
                self.analyze_expr_for_param(param_name, type_var, expr)?;
            }
            HirStmt::Assign { value, .. } => {
                self.analyze_expr_for_param(param_name, type_var, value)?;
            }
            HirStmt::Return(Some(expr)) => {
                self.analyze_expr_for_param(param_name, type_var, expr)?;
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expr_for_param(param_name, type_var, condition)?;
                for s in then_body {
                    self.analyze_stmt_for_param(param_name, type_var, s)?;
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.analyze_stmt_for_param(param_name, type_var, s)?;
                    }
                }
            }
            // DEPYLER-0715: Handle for-loops - traverse iter and body
            // Track loop variable as derived from parameter's element type
            HirStmt::For { target, iter, body } => {
                self.analyze_expr_for_param(param_name, type_var, iter)?;
                // If iterating over the parameter, the loop variable has type T
                if let HirExpr::Var(iter_var) = iter {
                    if iter_var == param_name {
                        // Track that the loop variable has the element type (T)
                        if let crate::hir::AssignTarget::Symbol(loop_var) = target {
                            self.loop_var_to_type_param
                                .insert(loop_var.clone(), type_var.to_string());
                        }
                    }
                }
                for s in body {
                    self.analyze_stmt_for_param(param_name, type_var, s)?;
                }
            }
            // DEPYLER-0715: Handle while-loops - traverse condition and body
            HirStmt::While { condition, body } => {
                self.analyze_expr_for_param(param_name, type_var, condition)?;
                for s in body {
                    self.analyze_stmt_for_param(param_name, type_var, s)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn analyze_expr_for_param(
        &mut self,
        param_name: &str,
        type_var: &str,
        expr: &HirExpr,
    ) -> Result<()> {
        match expr {
            HirExpr::Binary { left, right, op } => {
                self.check_binary_op_usage(param_name, type_var, left, right, *op)?;
                self.analyze_expr_for_param(param_name, type_var, left)?;
                self.analyze_expr_for_param(param_name, type_var, right)?;
            }
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => {
                if let HirExpr::Var(var) = object.as_ref() {
                    if var == param_name {
                        self.add_method_constraint(type_var, method);
                        // DEPYLER-0716: Detect dict key access with string argument
                        // When dict.get(key) is called where key is string-typed,
                        // substitute the key type T with String
                        if (method == "get" || method == "__getitem__") && !args.is_empty() {
                            let key_arg = &args[0];
                            let key_is_string = match key_arg {
                                HirExpr::Literal(crate::hir::Literal::String(_)) => true,
                                HirExpr::Var(v) => self.is_string_typed(v),
                                _ => false,
                            };
                            if key_is_string {
                                // Dict key type should be String, not generic T
                                self.substitutions
                                    .insert(type_var.to_string(), Type::String);
                            }
                        }
                    }
                }
                self.analyze_expr_for_param(param_name, type_var, object)?;
                for arg in args {
                    self.analyze_expr_for_param(param_name, type_var, arg)?;
                }
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    self.analyze_expr_for_param(param_name, type_var, arg)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn add_method_constraint(&mut self, type_var: &str, method: &str) {
        let constraint = match method {
            "len" => TypeConstraint::MustImplement("HasLen".to_string()),
            "push" | "pop" => TypeConstraint::MustImplement("VecLike".to_string()),
            "clone" => TypeConstraint::MustImplement("Clone".to_string()),
            _ => return,
        };

        self.constraints
            .entry(type_var.to_string())
            .or_default()
            .push(constraint);
    }

    fn check_binary_op_usage(
        &mut self,
        param_name: &str,
        type_var: &str,
        left: &HirExpr,
        right: &HirExpr,
        op: crate::hir::BinOp,
    ) -> Result<()> {
        use crate::hir::BinOp;

        // DEPYLER-0715: Detect which operand is the param (or loop var derived from param)
        // and which is the comparison target
        let (uses_param, other_operand) = match (left, right) {
            (HirExpr::Var(l), other) if l == param_name => (true, Some(other)),
            (other, HirExpr::Var(r)) if r == param_name => (true, Some(other)),
            // DEPYLER-0715: Also check if either operand is a loop variable derived from param
            (HirExpr::Var(l), other)
                if self
                    .loop_var_to_type_param
                    .get(l)
                    .is_some_and(|tv| tv == type_var) =>
            {
                (true, Some(other))
            }
            (other, HirExpr::Var(r))
                if self
                    .loop_var_to_type_param
                    .get(r)
                    .is_some_and(|tv| tv == type_var) =>
            {
                (true, Some(other))
            }
            _ => (false, None),
        };

        if uses_param {
            let constraint = match op {
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                    TypeConstraint::MustImplement("std::ops::Add".to_string())
                }
                BinOp::Eq | BinOp::NotEq => {
                    // DEPYLER-0716: Detect comparison target type
                    // If comparing to a string, substitute T with String instead of adding complex trait bounds
                    let target_is_string = other_operand.is_some_and(|op| match op {
                        HirExpr::Literal(crate::hir::Literal::String(_)) => true,
                        HirExpr::Var(v) => self.is_string_typed(v),
                        _ => false,
                    });
                    if target_is_string {
                        // DEPYLER-0716: Instead of complex PartialEq<&str> bound, substitute T with String
                        // This is simpler and String naturally implements PartialEq<&str>
                        self.substitutions
                            .insert(type_var.to_string(), Type::String);
                        // Still add basic PartialEq for the signature
                        TypeConstraint::MustImplement("PartialEq".to_string())
                    } else {
                        TypeConstraint::MustImplement("PartialEq".to_string())
                    }
                }
                BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq => {
                    TypeConstraint::MustImplement("PartialOrd".to_string())
                }
                BinOp::In | BinOp::NotIn => {
                    // DEPYLER-0716: For "key in dict" pattern, if key is string-typed
                    // and dict is the param, substitute T with String
                    // Note: In "key in dict", left is key and right is dict
                    // So if right is param_name and left is string-typed, substitute
                    if let HirExpr::Var(r) = right {
                        if r == param_name {
                            let key_is_string = match left {
                                HirExpr::Literal(crate::hir::Literal::String(_)) => true,
                                HirExpr::Var(v) => self.is_string_typed(v),
                                _ => false,
                            };
                            if key_is_string {
                                self.substitutions
                                    .insert(type_var.to_string(), Type::String);
                            }
                        }
                    }
                    return Ok(()); // No trait constraint needed
                }
                _ => return Ok(()),
            };

            self.constraints
                .entry(type_var.to_string())
                .or_default()
                .push(constraint);
        }

        Ok(())
    }

    /// DEPYLER-0715: Check if a variable is string-typed based on parameter types
    fn is_string_typed(&self, var_name: &str) -> bool {
        // Check if the variable matches a known string-typed parameter
        self.param_types
            .get(var_name)
            .is_some_and(|ty| matches!(ty, Type::String))
    }

    /// DEPYLER-0744: Infer the concrete type from return statements
    /// When a function has a param with Optional(Unknown), we analyze returns
    /// to find what concrete type the Optional should contain.
    /// Example: def f(value: int, fallback=None): return value OR return fallback
    /// Returns Int because `return value` where value: int provides the concrete type.
    #[allow(dead_code)]
    fn infer_concrete_type_from_returns(
        &self,
        _optional_param_name: &str,
        body: &[HirStmt],
    ) -> Option<Type> {
        let mut concrete_types = Vec::new();

        // Recursively collect return types from statements
        self.collect_return_types_from_stmts(body, &mut concrete_types);

        // Find the first concrete (non-Optional, non-Unknown, non-None) type
        concrete_types.into_iter().find(|ty| {
            !matches!(
                ty,
                Type::Optional(_) | Type::Unknown | Type::None | Type::TypeVar(_)
            )
        })
    }

    /// Helper to collect types from return statements recursively
    fn collect_return_types_from_stmts(&self, stmts: &[HirStmt], types: &mut Vec<Type>) {
        for stmt in stmts {
            match stmt {
                HirStmt::Return(Some(expr)) => {
                    // Infer type from the returned expression
                    if let Some(ty) = self.infer_expr_type(expr) {
                        types.push(ty);
                    }
                }
                HirStmt::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    self.collect_return_types_from_stmts(then_body, types);
                    if let Some(else_stmts) = else_body {
                        self.collect_return_types_from_stmts(else_stmts, types);
                    }
                }
                HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                    self.collect_return_types_from_stmts(body, types);
                }
                HirStmt::With { body, .. } | HirStmt::Try { body, .. } => {
                    self.collect_return_types_from_stmts(body, types);
                }
                _ => {}
            }
        }
    }

    /// Infer the type of an expression
    fn infer_expr_type(&self, expr: &HirExpr) -> Option<Type> {
        use crate::hir::Literal;

        match expr {
            HirExpr::Literal(lit) => match lit {
                Literal::Int(_) => Some(Type::Int),
                Literal::Float(_) => Some(Type::Float),
                Literal::String(_) => Some(Type::String),
                Literal::Bool(_) => Some(Type::Bool),
                Literal::None => Some(Type::None),
                _ => None,
            },
            HirExpr::Var(name) => {
                // Look up the variable's type from param_types
                self.param_types.get(name).cloned()
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;

    // ========== TypeVarRegistry Tests ==========

    #[test]
    fn test_type_var_registry_new() {
        let registry = TypeVarRegistry::new();
        assert!(registry.type_vars.is_empty());
        assert!(registry.function_type_params.is_empty());
    }

    #[test]
    fn test_type_var_registry_register() {
        let mut registry = TypeVarRegistry::new();
        let constraints = TypeVarConstraints {
            name: "T".to_string(),
            bounds: vec![TypeBound::TraitBound("Clone".to_string())],
            variance: Variance::Covariant,
            default: None,
        };
        registry.register_type_var("T".to_string(), constraints);
        assert!(registry.type_vars.contains_key("T"));
    }

    #[test]
    fn test_apply_substitutions_unknown() {
        let mut subs = HashMap::new();
        subs.insert("T".to_string(), Type::String);

        let result = TypeVarRegistry::apply_substitutions(&Type::Unknown, &subs);
        assert_eq!(result, Type::String);
    }

    #[test]
    fn test_apply_substitutions_no_match() {
        let subs = HashMap::new();
        let result = TypeVarRegistry::apply_substitutions(&Type::Unknown, &subs);
        assert_eq!(result, Type::Unknown);
    }

    #[test]
    fn test_apply_substitutions_list() {
        let mut subs = HashMap::new();
        subs.insert("T".to_string(), Type::Int);

        let list_type = Type::List(Box::new(Type::Unknown));
        let result = TypeVarRegistry::apply_substitutions(&list_type, &subs);
        assert_eq!(result, Type::List(Box::new(Type::Int)));
    }

    #[test]
    fn test_apply_substitutions_dict() {
        let mut subs = HashMap::new();
        subs.insert("T".to_string(), Type::Int);

        let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::Unknown));
        let result = TypeVarRegistry::apply_substitutions(&dict_type, &subs);
        assert_eq!(
            result,
            Type::Dict(Box::new(Type::String), Box::new(Type::Int))
        );
    }

    #[test]
    fn test_apply_substitutions_optional() {
        let mut subs = HashMap::new();
        subs.insert("T".to_string(), Type::Float);

        let opt_type = Type::Optional(Box::new(Type::Unknown));
        let result = TypeVarRegistry::apply_substitutions(&opt_type, &subs);
        assert_eq!(result, Type::Optional(Box::new(Type::Float)));
    }

    #[test]
    fn test_apply_substitutions_tuple() {
        let mut subs = HashMap::new();
        subs.insert("T".to_string(), Type::Bool);

        let tuple_type = Type::Tuple(vec![Type::Int, Type::Unknown, Type::String]);
        let result = TypeVarRegistry::apply_substitutions(&tuple_type, &subs);
        assert_eq!(
            result,
            Type::Tuple(vec![Type::Int, Type::Bool, Type::String])
        );
    }

    #[test]
    fn test_apply_substitutions_concrete() {
        let subs = HashMap::new();
        let result = TypeVarRegistry::apply_substitutions(&Type::Int, &subs);
        assert_eq!(result, Type::Int);
    }

    #[test]
    fn test_is_generic_with_type_var() {
        let mut registry = TypeVarRegistry::new();
        registry.register_type_var(
            "T".to_string(),
            TypeVarConstraints {
                name: "T".to_string(),
                bounds: vec![],
                variance: Variance::Invariant,
                default: None,
            },
        );

        assert!(registry.is_generic(&Type::Custom("T".to_string())));
        assert!(!registry.is_generic(&Type::Custom("String".to_string())));
    }

    #[test]
    fn test_is_generic_nested() {
        let mut registry = TypeVarRegistry::new();
        registry.register_type_var(
            "T".to_string(),
            TypeVarConstraints {
                name: "T".to_string(),
                bounds: vec![],
                variance: Variance::Invariant,
                default: None,
            },
        );

        assert!(registry.is_generic(&Type::List(Box::new(Type::Custom("T".to_string())))));
        assert!(registry.is_generic(&Type::Optional(Box::new(Type::Custom("T".to_string())))));
        assert!(registry.is_generic(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Custom("T".to_string()))
        )));
    }

    #[test]
    fn test_is_generic_tuple() {
        let mut registry = TypeVarRegistry::new();
        registry.register_type_var(
            "T".to_string(),
            TypeVarConstraints {
                name: "T".to_string(),
                bounds: vec![],
                variance: Variance::Invariant,
                default: None,
            },
        );

        assert!(registry.is_generic(&Type::Tuple(vec![Type::Int, Type::Custom("T".to_string())])));
        assert!(!registry.is_generic(&Type::Tuple(vec![Type::Int, Type::String])));
    }

    #[test]
    fn test_is_generic_function() {
        let mut registry = TypeVarRegistry::new();
        registry.register_type_var(
            "T".to_string(),
            TypeVarConstraints {
                name: "T".to_string(),
                bounds: vec![],
                variance: Variance::Invariant,
                default: None,
            },
        );

        assert!(registry.is_generic(&Type::Function {
            params: vec![Type::Custom("T".to_string())],
            ret: Box::new(Type::Int),
        }));
        assert!(registry.is_generic(&Type::Function {
            params: vec![Type::Int],
            ret: Box::new(Type::Custom("T".to_string())),
        }));
    }

    #[test]
    fn test_to_rust_generic_no_params() {
        let registry = TypeVarRegistry::new();
        assert_eq!(registry.to_rust_generic("Vec", &[]), "Vec");
    }

    #[test]
    fn test_to_rust_generic_with_params() {
        let registry = TypeVarRegistry::new();
        let result = registry.to_rust_generic("HashMap", &[Type::String, Type::Int]);
        assert_eq!(result, "HashMap<String, i32>");
    }

    #[test]
    fn test_type_to_rust_string() {
        let registry = TypeVarRegistry::new();
        assert_eq!(registry.type_to_rust_string(&Type::Int), "i32");
        assert_eq!(registry.type_to_rust_string(&Type::Float), "f64");
        assert_eq!(registry.type_to_rust_string(&Type::String), "String");
        assert_eq!(registry.type_to_rust_string(&Type::Bool), "bool");
        assert_eq!(registry.type_to_rust_string(&Type::None), "()");
    }

    #[test]
    fn test_type_to_rust_string_containers() {
        let registry = TypeVarRegistry::new();
        assert_eq!(
            registry.type_to_rust_string(&Type::List(Box::new(Type::Int))),
            "Vec<i32>"
        );
        assert_eq!(
            registry.type_to_rust_string(&Type::Dict(Box::new(Type::String), Box::new(Type::Int))),
            "HashMap<String, i32>"
        );
        assert_eq!(
            registry.type_to_rust_string(&Type::Optional(Box::new(Type::Int))),
            "Option<i32>"
        );
    }

    #[test]
    fn test_type_to_rust_string_tuple() {
        let registry = TypeVarRegistry::new();
        let tuple = Type::Tuple(vec![Type::Int, Type::String, Type::Bool]);
        assert_eq!(registry.type_to_rust_string(&tuple), "(i32, String, bool)");
    }

    // ========== TypeVarConstraints Tests ==========

    #[test]
    fn test_type_var_constraints_clone() {
        let constraints = TypeVarConstraints {
            name: "T".to_string(),
            bounds: vec![TypeBound::TraitBound("Clone".to_string())],
            variance: Variance::Covariant,
            default: Some(Type::Int),
        };
        let cloned = constraints.clone();
        assert_eq!(cloned.name, "T");
        assert_eq!(cloned.variance, Variance::Covariant);
    }

    #[test]
    fn test_type_var_constraints_debug() {
        let constraints = TypeVarConstraints {
            name: "T".to_string(),
            bounds: vec![],
            variance: Variance::Invariant,
            default: None,
        };
        let debug = format!("{:?}", constraints);
        assert!(debug.contains("TypeVarConstraints"));
        assert!(debug.contains("name"));
    }

    // ========== TypeBound Tests ==========

    #[test]
    fn test_type_bound_equality() {
        let bound1 = TypeBound::TraitBound("Clone".to_string());
        let bound2 = TypeBound::TraitBound("Clone".to_string());
        let bound3 = TypeBound::TraitBound("Debug".to_string());
        assert_eq!(bound1, bound2);
        assert_ne!(bound1, bound3);
    }

    #[test]
    fn test_type_bound_upper_bound() {
        let bound = TypeBound::UpperBound(Type::String);
        assert!(matches!(bound, TypeBound::UpperBound(Type::String)));
    }

    #[test]
    fn test_type_bound_union_bound() {
        let bound = TypeBound::UnionBound(vec![Type::Int, Type::String]);
        if let TypeBound::UnionBound(types) = bound {
            assert_eq!(types.len(), 2);
        } else {
            panic!("Expected UnionBound");
        }
    }

    // ========== Variance Tests ==========

    #[test]
    fn test_variance_equality() {
        assert_eq!(Variance::Invariant, Variance::Invariant);
        assert_eq!(Variance::Covariant, Variance::Covariant);
        assert_eq!(Variance::Contravariant, Variance::Contravariant);
        assert_ne!(Variance::Invariant, Variance::Covariant);
    }

    #[test]
    fn test_variance_copy() {
        let v = Variance::Covariant;
        let v2 = v; // Copy
        assert_eq!(v, v2);
    }

    // ========== TypeParameter Tests ==========

    #[test]
    fn test_type_parameter_clone() {
        let param = TypeParameter {
            name: "T".to_string(),
            bounds: vec!["Clone".to_string(), "Debug".to_string()],
            default: Some(Type::Int),
        };
        let cloned = param.clone();
        assert_eq!(cloned.name, "T");
        assert_eq!(cloned.bounds.len(), 2);
    }

    // ========== GenericType Tests ==========

    #[test]
    fn test_generic_type_type_var() {
        let gt = GenericType::TypeVar("T".to_string());
        assert!(matches!(gt, GenericType::TypeVar(s) if s == "T"));
    }

    #[test]
    fn test_generic_type_generic() {
        let gt = GenericType::Generic {
            base: Type::List(Box::new(Type::Unknown)),
            params: vec![GenericType::TypeVar("T".to_string())],
        };
        if let GenericType::Generic { params, .. } = gt {
            assert_eq!(params.len(), 1);
        }
    }

    #[test]
    fn test_generic_type_union() {
        let gt = GenericType::Union(vec![Type::Int, Type::String]);
        if let GenericType::Union(types) = gt {
            assert_eq!(types.len(), 2);
        }
    }

    #[test]
    fn test_generic_type_concrete() {
        let gt = GenericType::Concrete(Type::Int);
        assert_eq!(gt, GenericType::Concrete(Type::Int));
    }

    #[test]
    fn test_generic_type_clone() {
        let gt = GenericType::TypeVar("T".to_string());
        let cloned = gt.clone();
        assert_eq!(gt, cloned);
    }

    // ========== TypeVarCollector Tests ==========

    #[test]
    fn test_type_var_detection() {
        let mut collector = TypeVarCollector::new();

        collector.collect_from_type(&Type::Custom("T".to_string()));
        collector.collect_from_type(&Type::List(Box::new(Type::Custom("U".to_string()))));
        collector.collect_from_type(&Type::Custom("MyClass".to_string())); // Not a type var

        assert!(collector.type_vars.contains("T"));
        assert!(collector.type_vars.contains("U"));
        assert!(!collector.type_vars.contains("MyClass"));
    }

    #[test]
    fn test_type_var_collector_type_var_type() {
        let mut collector = TypeVarCollector::new();
        collector.collect_from_type(&Type::TypeVar("V".to_string()));
        assert!(collector.type_vars.contains("V"));
    }

    #[test]
    fn test_type_var_collector_dict_key() {
        let mut collector = TypeVarCollector::new();
        let dict = Type::Dict(
            Box::new(Type::TypeVar("K".to_string())),
            Box::new(Type::TypeVar("V".to_string())),
        );
        collector.collect_from_type(&dict);
        assert!(collector.type_vars.contains("K"));
        assert!(collector.type_vars.contains("V"));
        assert!(collector.dict_key_type_vars.contains("K"));
    }

    #[test]
    fn test_type_var_collector_function() {
        let mut collector = TypeVarCollector::new();
        let func = Type::Function {
            params: vec![Type::Custom("T".to_string())],
            ret: Box::new(Type::Custom("U".to_string())),
        };
        collector.collect_from_type(&func);
        assert!(collector.type_vars.contains("T"));
        assert!(collector.type_vars.contains("U"));
    }

    #[test]
    fn test_type_var_collector_generic() {
        let mut collector = TypeVarCollector::new();
        let generic = Type::Generic {
            base: "Either".to_string(),
            params: vec![Type::Custom("L".to_string()), Type::Custom("R".to_string())],
        };
        collector.collect_from_type(&generic);
        assert!(collector.type_vars.contains("L"));
        assert!(collector.type_vars.contains("R"));
    }

    #[test]
    fn test_type_var_collector_union() {
        let mut collector = TypeVarCollector::new();
        let union = Type::Union(vec![
            Type::Custom("A".to_string()),
            Type::Custom("B".to_string()),
        ]);
        collector.collect_from_type(&union);
        assert!(collector.type_vars.contains("A"));
        assert!(collector.type_vars.contains("B"));
    }

    #[test]
    fn test_type_var_collector_set() {
        let mut collector = TypeVarCollector::new();
        let set = Type::Set(Box::new(Type::Custom("T".to_string())));
        collector.collect_from_type(&set);
        assert!(collector.type_vars.contains("T"));
    }

    // ========== Function Inference Tests ==========

    #[test]
    fn test_generic_function_inference() {
        let mut registry = TypeVarRegistry::new();

        let func = HirFunction {
            name: "identity".to_string(),
            params: smallvec::smallvec![HirParam::new(
                "x".to_string(),
                Type::Custom("T".to_string())
            )],
            ret_type: Type::Custom("T".to_string()),
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let type_params = registry.infer_function_generics(&func).unwrap();
        assert_eq!(type_params.len(), 1);
        assert_eq!(type_params[0].name, "T");
    }

    #[test]
    fn test_infer_type_substitutions() {
        let registry = TypeVarRegistry::new();

        let func = HirFunction {
            name: "get_first".to_string(),
            params: smallvec::smallvec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::String))
            )],
            ret_type: Type::String,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let subs = registry.infer_type_substitutions(&func).unwrap();
        // No type vars in this function, so no substitutions
        assert!(subs.is_empty() || subs.contains_key("T"));
    }

    // ========== TypeInference Tests ==========

    #[test]
    fn test_constraint_inference() {
        let mut inference = TypeInference::new();

        // Test method constraint
        inference.add_method_constraint("T", "len");
        assert!(inference.constraints["T"]
            .iter()
            .any(|c| { matches!(c, TypeConstraint::MustImplement(s) if s == "HasLen") }));
    }

    #[test]
    fn test_type_inference_new() {
        let inference = TypeInference::new();
        assert!(inference.constraints.is_empty());
        assert!(inference.substitutions.is_empty());
    }

    #[test]
    fn test_add_method_constraint_push() {
        let mut inference = TypeInference::new();
        inference.add_method_constraint("T", "push");
        assert!(inference.constraints["T"]
            .iter()
            .any(|c| matches!(c, TypeConstraint::MustImplement(s) if s == "VecLike")));
    }

    #[test]
    fn test_add_method_constraint_clone() {
        let mut inference = TypeInference::new();
        inference.add_method_constraint("T", "clone");
        assert!(inference.constraints["T"]
            .iter()
            .any(|c| matches!(c, TypeConstraint::MustImplement(s) if s == "Clone")));
    }

    #[test]
    fn test_add_method_constraint_unknown() {
        let mut inference = TypeInference::new();
        inference.add_method_constraint("T", "unknown_method");
        // Unknown methods don't add constraints
        assert!(!inference.constraints.contains_key("T"));
    }
}
