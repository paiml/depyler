use crate::hir::{HirExpr, HirFunction, HirStmt, Literal, Type};
use std::collections::HashSet;

/// Analyzes string usage patterns to determine optimal string types
#[derive(Debug, Default)]
pub struct StringOptimizer {
    /// String literals that are only read, never mutated
    read_only_strings: HashSet<String>,
    /// String parameters that are never mutated
    immutable_params: HashSet<String>,
    /// Strings that are returned from functions
    returned_strings: HashSet<String>,
    /// Strings used in multiple contexts (may need Cow)
    mixed_usage_strings: HashSet<String>,
}

/// Optimal string representation based on usage analysis
#[derive(Debug, Clone, PartialEq)]
pub enum OptimalStringType {
    /// Use &'static str for string literals that are never mutated
    StaticStr,
    /// Use &str for borrowed string parameters
    BorrowedStr { lifetime: Option<String> },
    /// Use String for owned, mutable strings
    OwnedString,
    /// Use Cow<'static, str> for mixed usage patterns
    CowStr,
}

impl StringOptimizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyze a function to determine optimal string types
    pub fn analyze_function(&mut self, func: &HirFunction) {
        // Track string parameters
        for (param_name, param_type) in &func.params {
            if matches!(param_type, Type::String) {
                self.immutable_params.insert(param_name.clone());
            }
        }

        // Analyze function body
        for stmt in &func.body {
            self.analyze_stmt(stmt);
        }

        // Parameters that are mutated are not immutable
        for param in self.immutable_params.clone() {
            if !self.is_immutable(&param) {
                self.immutable_params.remove(&param);
            }
        }
    }

    /// Get the optimal string type for a given context
    pub fn get_optimal_type(&self, context: &StringContext) -> OptimalStringType {
        match context {
            StringContext::Literal(s) => {
                if self.returned_strings.contains(s) || self.mixed_usage_strings.contains(s) {
                    OptimalStringType::CowStr
                } else if self.is_read_only(s) {
                    OptimalStringType::StaticStr
                } else {
                    OptimalStringType::OwnedString
                }
            }
            StringContext::Parameter(name) => {
                if self.immutable_params.contains(name) {
                    OptimalStringType::BorrowedStr {
                        lifetime: Some("'a".to_string()),
                    }
                } else if self.mixed_usage_strings.contains(name) {
                    OptimalStringType::CowStr
                } else {
                    OptimalStringType::OwnedString
                }
            }
            StringContext::Return => OptimalStringType::OwnedString,
            StringContext::Concatenation => OptimalStringType::OwnedString,
        }
    }

    fn analyze_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign { target, value } => {
                // Track mutations
                if let HirExpr::Var(name) = target {
                    self.immutable_params.remove(name);
                }
                self.analyze_expr(value, false);
            }
            HirStmt::Return(Some(expr)) => {
                self.analyze_expr(expr, true);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expr(condition, false);
                for stmt in then_body {
                    self.analyze_stmt(stmt);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        self.analyze_stmt(stmt);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.analyze_expr(condition, false);
                for stmt in body {
                    self.analyze_stmt(stmt);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.analyze_expr(iter, false);
                for stmt in body {
                    self.analyze_stmt(stmt);
                }
            }
            HirStmt::Expr(expr) => {
                self.analyze_expr(expr, false);
            }
            _ => {}
        }
    }

    fn analyze_expr(&mut self, expr: &HirExpr, is_returned: bool) {
        match expr {
            HirExpr::Literal(Literal::String(s)) => {
                if is_returned {
                    self.returned_strings.insert(s.clone());
                } else {
                    self.read_only_strings.insert(s.clone());
                }
            }
            HirExpr::Var(name) => {
                if is_returned && self.immutable_params.contains(name) {
                    // Parameter is returned, might need Cow
                    self.mixed_usage_strings.insert(name.clone());
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.analyze_expr(left, false);
                self.analyze_expr(right, false);
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    self.analyze_expr(arg, false);
                }
            }
            HirExpr::List(elts) | HirExpr::Tuple(elts) => {
                for elt in elts {
                    self.analyze_expr(elt, is_returned);
                }
            }
            HirExpr::Dict(items) => {
                for (k, v) in items {
                    self.analyze_expr(k, false);
                    self.analyze_expr(v, is_returned);
                }
            }
            _ => {}
        }
    }

    fn is_read_only(&self, s: &str) -> bool {
        self.read_only_strings.contains(s) && !self.returned_strings.contains(s)
    }

    fn is_immutable(&self, param: &str) -> bool {
        self.immutable_params.contains(param)
    }
}

/// Context in which a string is being used
#[derive(Debug, Clone)]
pub enum StringContext {
    /// String literal in source code
    Literal(String),
    /// Function parameter
    Parameter(String),
    /// Return value
    Return,
    /// String concatenation operation
    Concatenation,
}

/// Generates optimized string code based on usage
pub fn generate_optimized_string(
    optimizer: &StringOptimizer,
    context: &StringContext,
) -> String {
    match optimizer.get_optimal_type(context) {
        OptimalStringType::StaticStr => {
            // For static strings, we don't need .to_string()
            match context {
                StringContext::Literal(s) => format!("\"{}\"", s),
                _ => {
                    // Fallback to owned string for non-literal contexts
                    eprintln!("Warning: StaticStr type used for non-literal context");
                    format!("{}.to_string()", context)
                }
            }
        }
        OptimalStringType::BorrowedStr { .. } => {
            // Parameters should be borrowed
            match context {
                StringContext::Parameter(name) => format!("&{}", name),
                _ => {
                    // Fallback to owned string for non-parameter contexts
                    eprintln!("Warning: BorrowedStr type used for non-parameter context");
                    format!("{}.to_string()", context)
                }
            }
        }
        OptimalStringType::OwnedString => {
            // Need full String ownership
            match context {
                StringContext::Literal(s) => format!("\"{}\".to_string()", s),
                StringContext::Parameter(name) => name.clone(),
                _ => "String::new()".to_string(),
            }
        }
        OptimalStringType::CowStr => {
            // Use Cow for flexible ownership
            match context {
                StringContext::Literal(s) => format!("Cow::Borrowed(\"{}\")", s),
                StringContext::Parameter(name) => format!("Cow::Borrowed({})", name),
                _ => "Cow::Owned(String::new())".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;

    #[test]
    fn test_read_only_string_optimization() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![],
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        let context = StringContext::Literal("hello".to_string());
        assert_eq!(
            optimizer.get_optimal_type(&context),
            OptimalStringType::StaticStr
        );
    }

    #[test]
    fn test_returned_string_needs_ownership() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![],
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String(
                "result".to_string(),
            ))))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        let context = StringContext::Literal("result".to_string());
        // Returned strings might need Cow for flexibility
        assert_eq!(
            optimizer.get_optimal_type(&context),
            OptimalStringType::CowStr
        );
    }

    #[test]
    fn test_immutable_parameter_borrowing() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![("s".to_string(), Type::String)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("s".to_string())],
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        let context = StringContext::Parameter("s".to_string());
        assert_eq!(
            optimizer.get_optimal_type(&context),
            OptimalStringType::BorrowedStr {
                lifetime: Some("'a".to_string())
            }
        );
    }

    #[test]
    fn test_generate_optimized_string_code() {
        let optimizer = StringOptimizer::new();

        // Test static string generation
        let code = generate_optimized_string(
            &optimizer,
            &StringContext::Literal("hello".to_string()),
        );
        assert!(code == "\"hello\".to_string()" || code == "\"hello\"");
    }
}