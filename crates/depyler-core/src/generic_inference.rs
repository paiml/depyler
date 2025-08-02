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

    /// Infer generic type parameters for a function
    pub fn infer_function_generics(&mut self, func: &HirFunction) -> Result<Vec<TypeParameter>> {
        let mut collector = TypeVarCollector::new();

        // Collect type variables from parameters
        for (_, param_type) in &func.params {
            collector.collect_from_type(param_type);
        }

        // Collect from return type
        collector.collect_from_type(&func.ret_type);

        // Collect from function body
        for stmt in &func.body {
            collector.collect_from_stmt(stmt);
        }

        // Build constraints from usage
        let mut inference = TypeInference::new();
        inference.analyze_function(func)?;

        // Generate type parameters
        let type_params =
            self.generate_type_parameters(&collector.type_vars, &inference.constraints)?;

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
}

impl TypeVarCollector {
    fn new() -> Self {
        Self {
            type_vars: HashSet::new(),
        }
    }

    fn collect_from_type(&mut self, ty: &Type) {
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
            Type::List(inner) | Type::Optional(inner) => self.collect_from_type(inner),
            Type::Dict(k, v) => {
                self.collect_from_type(k);
                self.collect_from_type(v);
            }
            Type::Tuple(types) => {
                for t in types {
                    self.collect_from_type(t);
                }
            }
            Type::Function { params, ret } => {
                for p in params {
                    self.collect_from_type(p);
                }
                self.collect_from_type(ret);
            }
            _ => {}
        }
    }

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

    #[allow(clippy::only_used_in_recursion)]
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
    /// Type substitutions for unification (unused but reserved for future use)
    #[allow(dead_code)]
    substitutions: HashMap<String, Type>,
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
        }
    }

    fn analyze_function(&mut self, func: &HirFunction) -> Result<()> {
        // Analyze parameter usage to infer constraints
        for (param_name, param_type) in &func.params {
            match param_type {
                Type::Custom(type_var)
                    if type_var.len() == 1 && type_var.chars().next().unwrap().is_uppercase() =>
                {
                    // This is a type variable, analyze its usage
                    self.analyze_param_usage(param_name, type_var, &func.body)?;
                }
                Type::TypeVar(type_var) => {
                    // This is explicitly a type variable
                    self.analyze_param_usage(param_name, type_var, &func.body)?;
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

        let uses_param = match (left, right) {
            (HirExpr::Var(l), _) if l == param_name => true,
            (_, HirExpr::Var(r)) if r == param_name => true,
            _ => false,
        };

        if uses_param {
            let constraint = match op {
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                    TypeConstraint::MustImplement("std::ops::Add".to_string())
                }
                BinOp::Eq | BinOp::NotEq => TypeConstraint::MustImplement("PartialEq".to_string()),
                BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq => {
                    TypeConstraint::MustImplement("PartialOrd".to_string())
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;

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
    fn test_generic_function_inference() {
        let mut registry = TypeVarRegistry::new();

        let func = HirFunction {
            name: "identity".to_string(),
            params: smallvec::smallvec![("x".to_string(), Type::Custom("T".to_string()))],
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
    fn test_constraint_inference() {
        let mut inference = TypeInference::new();

        // Test method constraint
        inference.add_method_constraint("T", "len");
        assert!(inference.constraints["T"]
            .iter()
            .any(|c| { matches!(c, TypeConstraint::MustImplement(s) if s == "HasLen") }));
    }
}
