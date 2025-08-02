use crate::hir::{HirExpr, HirFunction, HirStmt, Type};
use std::collections::{HashMap, HashSet};

/// Tracks how parameters are used within a function to infer borrowing patterns
#[derive(Debug, Default)]
pub struct BorrowingContext {
    /// Parameters that are mutated in the function
    mutated_params: HashSet<String>,
    /// Parameters that escape (are returned or stored)
    escaping_params: HashSet<String>,
    /// Parameters that are only read
    read_only_params: HashSet<String>,
    /// Parameters used in loops (may need special handling)
    loop_used_params: HashSet<String>,
}

/// Analysis result for a single parameter
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowingPattern {
    /// Parameter should be taken by value (moved)
    Owned,
    /// Parameter can be borrowed immutably
    Borrowed,
    /// Parameter needs mutable borrow
    MutableBorrow,
}

impl BorrowingContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyze a function to determine parameter borrowing patterns
    pub fn analyze_function(&mut self, func: &HirFunction) {
        // First pass: identify all parameters
        for (param_name, _) in &func.params {
            self.read_only_params.insert(param_name.clone());
        }

        // Analyze function body
        for stmt in &func.body {
            self.analyze_stmt(stmt);
        }

        // Remove read-only classification from mutated or escaping params
        for param in &self.mutated_params {
            self.read_only_params.remove(param);
        }
        for param in &self.escaping_params {
            self.read_only_params.remove(param);
        }
    }

    /// Get the borrowing pattern for a specific parameter
    pub fn get_pattern(&self, param_name: &str, param_type: &Type) -> BorrowingPattern {
        if self.escaping_params.contains(param_name) {
            // Parameters that escape must be owned
            BorrowingPattern::Owned
        } else if self.mutated_params.contains(param_name) {
            // Mutated parameters need mutable borrow
            BorrowingPattern::MutableBorrow
        } else if self.is_copyable(param_type) {
            // Small copyable types should be passed by value
            BorrowingPattern::Owned
        } else {
            // Everything else can be borrowed
            BorrowingPattern::Borrowed
        }
    }

    /// Generate Rust parameter signature based on borrowing pattern
    pub fn generate_param_signature(&self, param_name: &str, param_type: &Type) -> String {
        let pattern = self.get_pattern(param_name, param_type);
        let type_str = self.type_to_rust_string(param_type);
        
        match pattern {
            BorrowingPattern::Owned => format!("{}: {}", param_name, type_str),
            BorrowingPattern::Borrowed => format!("{}: &{}", param_name, type_str),
            BorrowingPattern::MutableBorrow => format!("{}: &mut {}", param_name, type_str),
        }
    }

    fn analyze_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign { target, value } => {
                // Check if we're assigning to a parameter
                if let HirExpr::Var(name) = target {
                    self.mutated_params.insert(name.clone());
                }
                // Check if we're assigning a parameter (escaping)
                self.check_escaping_expr(value);
                self.analyze_expr(value);
            }
            HirStmt::Return(Some(expr)) => {
                // Parameters in return statements are escaping
                self.check_escaping_expr(expr);
                self.analyze_expr(expr);
            }
            HirStmt::Expr(expr) => {
                self.analyze_expr(expr);
            }
            HirStmt::If { test, then_body, else_body } => {
                self.analyze_expr(test);
                for stmt in then_body {
                    self.analyze_stmt(stmt);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        self.analyze_stmt(stmt);
                    }
                }
            }
            HirStmt::While { test, body } => {
                self.analyze_expr(test);
                // Mark parameters used in loops
                self.mark_loop_params(body);
                for stmt in body {
                    self.analyze_stmt(stmt);
                }
            }
            HirStmt::For { target: _, iter, body } => {
                self.analyze_expr(iter);
                // Mark parameters used in loops
                self.mark_loop_params(body);
                for stmt in body {
                    self.analyze_stmt(stmt);
                }
            }
            _ => {}
        }
    }

    fn analyze_expr(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::BinOp { op: _, left, right } => {
                self.analyze_expr(left);
                self.analyze_expr(right);
            }
            HirExpr::UnaryOp { op: _, operand } => {
                self.analyze_expr(operand);
            }
            HirExpr::Call { func: _, args } => {
                for arg in args {
                    self.analyze_expr(arg);
                }
            }
            HirExpr::List(elts) => {
                for elt in elts {
                    self.analyze_expr(elt);
                }
            }
            HirExpr::Dict(items) => {
                for (k, v) in items {
                    self.analyze_expr(k);
                    self.analyze_expr(v);
                }
            }
            HirExpr::Tuple(elts) => {
                for elt in elts {
                    self.analyze_expr(elt);
                }
            }
            HirExpr::Index { base, index } => {
                self.analyze_expr(base);
                self.analyze_expr(index);
            }
            _ => {}
        }
    }

    fn check_escaping_expr(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::Var(name) => {
                // Direct return of parameter
                self.escaping_params.insert(name.clone());
            }
            HirExpr::List(elts) | HirExpr::Tuple(elts) => {
                // Parameters in collections that are returned
                for elt in elts {
                    if let HirExpr::Var(name) = elt {
                        self.escaping_params.insert(name.clone());
                    }
                }
            }
            _ => {}
        }
    }

    fn mark_loop_params(&mut self, body: &[HirStmt]) {
        for stmt in body {
            self.find_params_in_stmt(stmt);
        }
    }

    fn find_params_in_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Expr(expr) => self.find_params_in_expr(expr),
            HirStmt::Assign { value, .. } => self.find_params_in_expr(value),
            _ => {}
        }
    }

    fn find_params_in_expr(&mut self, expr: &HirExpr) {
        if let HirExpr::Var(name) = expr {
            if self.read_only_params.contains(name) 
                || self.mutated_params.contains(name) 
                || self.escaping_params.contains(name) {
                self.loop_used_params.insert(name.clone());
            }
        }
    }

    fn is_copyable(&self, ty: &Type) -> bool {
        matches!(
            ty,
            Type::Int | Type::Float | Type::Bool | Type::None
        )
    }

    fn type_to_rust_string(&self, ty: &Type) -> String {
        match ty {
            Type::Unknown => "serde_json::Value".to_string(),
            Type::Int => "i32".to_string(),
            Type::Float => "f64".to_string(),
            Type::String => "String".to_string(),
            Type::Bool => "bool".to_string(),
            Type::None => "()".to_string(),
            Type::List(inner) => format!("Vec<{}>", self.type_to_rust_string(inner)),
            Type::Dict(k, v) => format!("HashMap<{}, {}>", 
                self.type_to_rust_string(k), 
                self.type_to_rust_string(v)
            ),
            Type::Tuple(types) => {
                if types.is_empty() {
                    "()".to_string()
                } else {
                    let type_strs: Vec<String> = types
                        .iter()
                        .map(|t| self.type_to_rust_string(t))
                        .collect();
                    format!("({})", type_strs.join(", "))
                }
            }
            Type::Optional(inner) => format!("Option<{}>", self.type_to_rust_string(inner)),
            Type::Function { .. } => "/* function */".to_string(),
            Type::Custom(name) => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, Literal};

    #[test]
    fn test_read_only_parameter() {
        let mut ctx = BorrowingContext::new();
        
        let func = HirFunction {
            name: "test".to_string(),
            params: vec![("x".to_string(), Type::String)],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Return(Some(HirExpr::Call {
                    func: "len".to_string(),
                    args: vec![HirExpr::Var("x".to_string())],
                }))
            ],
        };

        ctx.analyze_function(&func);
        assert_eq!(
            ctx.get_pattern("x", &Type::String),
            BorrowingPattern::Borrowed
        );
    }

    #[test]
    fn test_mutated_parameter() {
        let mut ctx = BorrowingContext::new();
        
        let func = HirFunction {
            name: "test".to_string(),
            params: vec![("x".to_string(), Type::List(Box::new(Type::Int)))],
            ret_type: Type::None,
            body: vec![
                HirStmt::Expr(HirExpr::Call {
                    func: "append".to_string(),
                    args: vec![
                        HirExpr::Var("x".to_string()),
                        HirExpr::Literal(Literal::Int(42)),
                    ],
                })
            ],
        };

        ctx.analyze_function(&func);
        // Note: This is a simplified test. In reality, we'd need to track
        // method calls that mutate the receiver.
    }

    #[test]
    fn test_escaping_parameter() {
        let mut ctx = BorrowingContext::new();
        
        let func = HirFunction {
            name: "test".to_string(),
            params: vec![("x".to_string(), Type::String)],
            ret_type: Type::String,
            body: vec![
                HirStmt::Return(Some(HirExpr::Var("x".to_string())))
            ],
        };

        ctx.analyze_function(&func);
        assert_eq!(
            ctx.get_pattern("x", &Type::String),
            BorrowingPattern::Owned
        );
    }

    #[test]
    fn test_copyable_parameter() {
        let mut ctx = BorrowingContext::new();
        
        let func = HirFunction {
            name: "test".to_string(),
            params: vec![("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Return(Some(HirExpr::BinOp {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("x".to_string())),
                    right: Box::new(HirExpr::Literal(Literal::Int(1))),
                }))
            ],
        };

        ctx.analyze_function(&func);
        assert_eq!(
            ctx.get_pattern("x", &Type::Int),
            BorrowingPattern::Owned
        );
    }

    #[test]
    fn test_generate_param_signature() {
        let ctx = BorrowingContext::new();
        
        // Test borrowed string
        let mut ctx_borrow = BorrowingContext::new();
        ctx_borrow.read_only_params.insert("s".to_string());
        assert_eq!(
            ctx_borrow.generate_param_signature("s", &Type::String),
            "s: &String"
        );
        
        // Test owned int
        assert_eq!(
            ctx.generate_param_signature("n", &Type::Int),
            "n: i32"
        );
    }
}