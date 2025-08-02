use crate::hir::{HirExpr, HirFunction, HirStmt, Literal, Type};
use std::collections::{HashMap, HashSet};

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
    /// String literal frequency counter for interning decisions
    string_literal_count: HashMap<String, usize>,
    /// Strings that should be interned due to frequent use
    interned_strings: HashSet<String>,
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
                // Track mutations - target is already a String (Symbol)
                if self.immutable_params.contains(target) {
                    self.immutable_params.remove(target);
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
                // Count string literal occurrences
                *self.string_literal_count.entry(s.clone()).or_insert(0) += 1;
                
                // Check if this string should be interned (used more than 3 times)
                if self.string_literal_count.get(s).copied().unwrap_or(0) > 3 {
                    self.interned_strings.insert(s.clone());
                }
                
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
            HirExpr::Binary { op, left, right } => {
                // String concatenation needs owned strings
                if matches!(op, crate::hir::BinOp::Add) {
                    // Check if either side is a string
                    if self.is_string_expr(left) || self.is_string_expr(right) {
                        // Mark both sides as needing ownership for concatenation
                        self.mark_as_owned(left);
                        self.mark_as_owned(right);
                    }
                }
                self.analyze_expr(left, false);
                self.analyze_expr(right, false);
            }
            HirExpr::Call { func, args } => {
                // Analyze method calls that might mutate strings
                if self.is_mutating_method(func) && !args.is_empty() {
                    if let HirExpr::Var(name) = &args[0] {
                        self.immutable_params.remove(name);
                    }
                }
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

    /// Check if an expression is a string type
    fn is_string_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(name) => self.immutable_params.contains(name),
            HirExpr::Call { func, .. } => {
                // Common string-returning functions
                matches!(func.as_str(), "str" | "format" | "to_string" | "join")
            }
            _ => false,
        }
    }

    /// Mark a string expression as needing ownership
    fn mark_as_owned(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::Literal(Literal::String(s)) => {
                self.read_only_strings.remove(s);
            }
            HirExpr::Var(name) => {
                self.immutable_params.remove(name);
            }
            _ => {}
        }
    }

    /// Check if a method call mutates the string
    fn is_mutating_method(&self, method: &str) -> bool {
        matches!(
            method,
            "push_str" | "push" | "insert" | "insert_str" | "replace_range" | "clear" | "truncate"
        )
    }

    /// Check if a string literal should be interned
    pub fn should_intern(&self, s: &str) -> bool {
        self.interned_strings.contains(s)
    }

    /// Get interned string name for a literal
    pub fn get_interned_name(&self, s: &str) -> Option<String> {
        if self.should_intern(s) {
            // Generate a constant name from the string content
            let name = s
                .chars()
                .map(|c| match c {
                    'a'..='z' | 'A'..='Z' | '0'..='9' => c.to_ascii_uppercase(),
                    _ => '_',
                })
                .collect::<String>();
            Some(format!("STR_{}", if name.is_empty() { "EMPTY" } else { &name }))
        } else {
            None
        }
    }

    /// Generate interned string constants
    pub fn generate_interned_constants(&self) -> Vec<String> {
        let mut constants = Vec::new();
        for s in &self.interned_strings {
            if let Some(name) = self.get_interned_name(s) {
                constants.push(format!(
                    "const {}: &'static str = \"{}\";",
                    name,
                    escape_string(s)
                ));
            }
        }
        constants
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

impl std::fmt::Display for StringContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringContext::Literal(s) => write!(f, "\"{}\"", s),
            StringContext::Parameter(name) => write!(f, "{}", name),
            StringContext::Return => write!(f, "<return>"),
            StringContext::Concatenation => write!(f, "<concat>"),
        }
    }
}

/// Generates optimized string code based on usage
pub fn generate_optimized_string(optimizer: &StringOptimizer, context: &StringContext) -> String {
    match optimizer.get_optimal_type(context) {
        OptimalStringType::StaticStr => {
            // For static strings, we don't need .to_string()
            match context {
                StringContext::Literal(s) => format!("\"{}\"", escape_string(s)),
                _ => {
                    // Fallback to owned string for non-literal contexts
                    format!("{}.to_string()", context)
                }
            }
        }
        OptimalStringType::BorrowedStr { .. } => {
            // Parameters should be borrowed
            match context {
                StringContext::Parameter(name) => name.clone(), // Already &str in function signature
                StringContext::Literal(s) => format!("\"{}\"", escape_string(s)), // Literals can be &'static str
                _ => {
                    format!("{}.as_str()", context)
                }
            }
        }
        OptimalStringType::OwnedString => {
            // Need full String ownership
            match context {
                StringContext::Literal(s) => format!("\"{}\".to_string()", escape_string(s)),
                StringContext::Parameter(name) => format!("{}.to_string()", name),
                StringContext::Concatenation => "String::new()".to_string(),
                _ => "String::new()".to_string(),
            }
        }
        OptimalStringType::CowStr => {
            // Use Cow for flexible ownership
            match context {
                StringContext::Literal(s) => format!("Cow::Borrowed(\"{}\")", escape_string(s)),
                StringContext::Parameter(name) => format!("Cow::Borrowed({})", name),
                StringContext::Concatenation => "Cow::Owned(String::new())".to_string(),
                _ => "Cow::Owned(String::new())".to_string(),
            }
        }
    }
}

/// Escape a string for use in Rust string literals
fn escape_string(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '"' => vec!['\\', '"'],
            '\\' => vec!['\\', '\\'],
            '\n' => vec!['\\', 'n'],
            '\r' => vec!['\\', 'r'],
            '\t' => vec!['\\', 't'],
            c => vec![c],
        })
        .collect()
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
            params: vec![].into(),
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
            params: vec![].into(),
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
            params: vec![("s".to_string(), Type::String)].into(),
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
        let code =
            generate_optimized_string(&optimizer, &StringContext::Literal("hello".to_string()));
        assert!(code == "\"hello\".to_string()" || code == "\"hello\"");
    }
}
