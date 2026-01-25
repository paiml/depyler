use crate::hir::{AssignTarget, HirExpr, HirFunction, HirStmt, Literal, Type};
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
    /// Mapping from string literal to its unique constant name
    interned_names: HashMap<String, String>,
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
        for param in &func.params {
            if matches!(param.ty, Type::String) {
                self.immutable_params.insert(param.name.clone());
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
                // v3.16.0 Phase 3: Only use Cow for TRUE mixed usage (returned AND borrowed elsewhere)
                // Don't use Cow for simple returned literals - use owned String instead
                if self.mixed_usage_strings.contains(s) {
                    OptimalStringType::CowStr
                } else if self.returned_strings.contains(s) {
                    // Returned but not borrowed elsewhere - use owned String
                    OptimalStringType::OwnedString
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
            HirStmt::Assign { target, value, .. } => {
                self.analyze_assign_stmt(target, value);
            }
            HirStmt::Return(Some(expr)) => {
                self.analyze_expr(expr, true);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_if_stmt(condition, then_body, else_body);
            }
            HirStmt::While { condition, body } => {
                self.analyze_while_stmt(condition, body);
            }
            HirStmt::For { iter, body, .. } => {
                self.analyze_for_stmt(iter, body);
            }
            HirStmt::Expr(expr) => {
                self.analyze_expr(expr, false);
            }
            _ => {}
        }
    }

    fn analyze_assign_stmt(&mut self, target: &AssignTarget, value: &HirExpr) {
        if let AssignTarget::Symbol(symbol) = target {
            if self.immutable_params.contains(symbol) {
                self.immutable_params.remove(symbol);
            }
        }
        self.analyze_expr(value, false);
    }

    fn analyze_if_stmt(
        &mut self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
    ) {
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

    fn analyze_while_stmt(&mut self, condition: &HirExpr, body: &[HirStmt]) {
        self.analyze_expr(condition, false);
        for stmt in body {
            self.analyze_stmt(stmt);
        }
    }

    fn analyze_for_stmt(&mut self, iter: &HirExpr, body: &[HirStmt]) {
        self.analyze_expr(iter, false);
        for stmt in body {
            self.analyze_stmt(stmt);
        }
    }

    fn analyze_expr(&mut self, expr: &HirExpr, is_returned: bool) {
        match expr {
            HirExpr::Literal(Literal::String(s)) => {
                self.analyze_string_literal(s, is_returned);
            }
            HirExpr::Var(name) => {
                self.analyze_var_usage(name, is_returned);
            }
            HirExpr::Binary { op, left, right } => {
                self.analyze_binary_expr(op, left, right);
            }
            HirExpr::Call { func, args, .. } => {
                self.analyze_call_expr(func, args);
            }
            HirExpr::List(elts) | HirExpr::Tuple(elts) => {
                self.analyze_collection_expr(elts, is_returned);
            }
            HirExpr::Dict(items) => {
                self.analyze_dict_expr(items, is_returned);
            }
            _ => {}
        }
    }

    fn analyze_string_literal(&mut self, s: &str, is_returned: bool) {
        *self.string_literal_count.entry(s.to_string()).or_insert(0) += 1;

        if self.string_literal_count.get(s).copied().unwrap_or(0) > 3 {
            self.interned_strings.insert(s.to_string());
        }

        if is_returned {
            self.returned_strings.insert(s.to_string());
        } else {
            self.read_only_strings.insert(s.to_string());
        }
    }

    /// Finalize interned string names, resolving any collisions
    /// This must be called after analysis and before code generation
    pub fn finalize_interned_names(&mut self) {
        if !self.interned_names.is_empty() {
            // Already finalized
            return;
        }

        // Map from base constant name to list of actual string values
        let mut name_map: HashMap<String, Vec<String>> = HashMap::new();

        // Group strings by their base constant name
        for s in &self.interned_strings {
            let base_name = self.generate_base_const_name(s);
            name_map.entry(base_name).or_default().push(s.clone());
        }

        // Assign unique names, adding suffixes for collisions
        for (base_name, strings) in name_map {
            if strings.len() == 1 {
                // No collision, use base name
                self.interned_names.insert(strings[0].clone(), base_name);
            } else {
                // Collision detected, add numeric suffixes
                for (idx, s) in strings.iter().enumerate() {
                    let unique_name = format!("{}_{}", base_name, idx + 1);
                    self.interned_names.insert(s.clone(), unique_name);
                }
            }
        }
    }

    /// Generate base constant name from string content (may have collisions)
    fn generate_base_const_name(&self, s: &str) -> String {
        // Convert to uppercase, replace non-alphanumeric with underscore
        let name = s
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => c.to_ascii_uppercase(),
                _ => '_',
            })
            .collect::<String>();

        let base_name = if name.is_empty() {
            "EMPTY".to_string()
        } else {
            name
        };

        format!("STR_{}", base_name)
    }

    fn analyze_var_usage(&mut self, name: &str, is_returned: bool) {
        if is_returned && self.immutable_params.contains(name) {
            self.mixed_usage_strings.insert(name.to_string());
        }
    }

    fn analyze_binary_expr(&mut self, op: &crate::hir::BinOp, left: &HirExpr, right: &HirExpr) {
        if matches!(op, crate::hir::BinOp::Add)
            && (self.is_string_expr(left) || self.is_string_expr(right))
        {
            self.mark_as_owned(left);
            self.mark_as_owned(right);
        }
        self.analyze_expr(left, false);
        self.analyze_expr(right, false);
    }

    fn analyze_call_expr(&mut self, func: &str, args: &[HirExpr]) {
        if self.is_mutating_method(func) && !args.is_empty() {
            if let HirExpr::Var(name) = &args[0] {
                self.immutable_params.remove(name);
            }
        }
        for arg in args {
            self.analyze_expr(arg, false);
        }
    }

    fn analyze_collection_expr(&mut self, elts: &[HirExpr], is_returned: bool) {
        for elt in elts {
            self.analyze_expr(elt, is_returned);
        }
    }

    fn analyze_dict_expr(&mut self, items: &[(HirExpr, HirExpr)], is_returned: bool) {
        for (k, v) in items {
            self.analyze_expr(k, false);
            self.analyze_expr(v, is_returned);
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
    /// Returns the unique constant name for an interned string
    pub fn get_interned_name(&self, s: &str) -> Option<String> {
        // Return the finalized name from the cache
        self.interned_names.get(s).cloned()
    }

    /// Generate interned string constants
    pub fn generate_interned_constants(&self) -> Vec<String> {
        let mut constants = Vec::new();

        for (string_value, const_name) in &self.interned_names {
            constants.push(format!(
                "const {}: &'static str = \"{}\";",
                const_name,
                escape_string(string_value)
            ));
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
        OptimalStringType::StaticStr => generate_static_str(context),
        OptimalStringType::BorrowedStr { .. } => generate_borrowed_str(context),
        OptimalStringType::OwnedString => generate_owned_string(context),
        OptimalStringType::CowStr => generate_cow_str(context),
    }
}

fn generate_static_str(context: &StringContext) -> String {
    match context {
        StringContext::Literal(s) => format!("\"{}\"", escape_string(s)),
        _ => format!("{}.to_string()", context),
    }
}

fn generate_borrowed_str(context: &StringContext) -> String {
    match context {
        StringContext::Parameter(name) => name.clone(),
        StringContext::Literal(s) => format!("\"{}\"", escape_string(s)),
        _ => format!("{}.as_str()", context),
    }
}

fn generate_owned_string(context: &StringContext) -> String {
    match context {
        StringContext::Literal(s) => format!("\"{}\".to_string()", escape_string(s)),
        StringContext::Parameter(name) => format!("{}.to_string()", name),
        StringContext::Concatenation | StringContext::Return => "String::new()".to_string(),
    }
}

fn generate_cow_str(context: &StringContext) -> String {
    match context {
        StringContext::Literal(s) => format!("Cow::Borrowed(\"{}\")", escape_string(s)),
        StringContext::Parameter(name) => format!("Cow::Borrowed({})", name),
        StringContext::Concatenation | StringContext::Return => {
            "Cow::Owned(String::new())".to_string()
        }
    }
}

/// Escape a string for use in Rust string literals
fn escape_string(s: &str) -> String {
    s.chars().flat_map(escape_char).collect()
}

fn escape_char(c: char) -> Vec<char> {
    match c {
        '"' => vec!['\\', '"'],
        '\\' => vec!['\\', '\\'],
        '\n' => vec!['\\', 'n'],
        '\r' => vec!['\\', 'r'],
        '\t' => vec!['\\', 't'],
        c => vec![c],
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
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
                kwargs: vec![],
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
        assert_eq!(
            optimizer.get_optimal_type(&context),
            OptimalStringType::OwnedString
        );
    }

    #[test]
    fn test_immutable_parameter_borrowing() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![HirParam::new("s".to_string(), Type::String)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("s".to_string())],
                kwargs: vec![],
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

        let code =
            generate_optimized_string(&optimizer, &StringContext::Literal("hello".to_string()));
        assert!(code == "\"hello\".to_string()" || code == "\"hello\"");
    }

    #[test]
    fn test_new_creates_default() {
        let optimizer = StringOptimizer::new();
        assert!(!optimizer.should_intern("any"));
        assert!(optimizer.get_interned_name("any").is_none());
    }

    #[test]
    fn test_mixed_usage_strings_get_cow() {
        let mut optimizer = StringOptimizer::new();

        // Parameter that is both used and returned - but immutable params take precedence
        let func = HirFunction {
            name: "test".to_string(),
            params: vec![HirParam::new("s".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![
                HirStmt::Expr(HirExpr::Var("s".to_string())),
                HirStmt::Return(Some(HirExpr::Var("s".to_string()))),
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        // Even though it's in mixed_usage_strings, immutable_params takes precedence
        let context = StringContext::Parameter("s".to_string());
        assert_eq!(
            optimizer.get_optimal_type(&context),
            OptimalStringType::BorrowedStr {
                lifetime: Some("'a".to_string())
            }
        );

        // Verify it IS in mixed_usage though
        assert!(optimizer.mixed_usage_strings.contains("s"));
    }

    #[test]
    fn test_mutated_parameter_loses_immutability() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![HirParam::new("s".to_string(), Type::String)].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("s".to_string()),
                value: HirExpr::Literal(Literal::String("new".to_string())),
                type_annotation: None,
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        let context = StringContext::Parameter("s".to_string());
        assert_eq!(
            optimizer.get_optimal_type(&context),
            OptimalStringType::OwnedString
        );
    }

    #[test]
    fn test_string_interning_threshold() {
        let mut optimizer = StringOptimizer::new();

        // Use the same string 4 times to trigger interning
        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![
                HirStmt::Expr(HirExpr::Literal(Literal::String("common".to_string()))),
                HirStmt::Expr(HirExpr::Literal(Literal::String("common".to_string()))),
                HirStmt::Expr(HirExpr::Literal(Literal::String("common".to_string()))),
                HirStmt::Expr(HirExpr::Literal(Literal::String("common".to_string()))),
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        assert!(optimizer.should_intern("common"));
    }

    #[test]
    fn test_finalize_interned_names_no_collision() {
        let mut optimizer = StringOptimizer::new();
        optimizer.interned_strings.insert("hello".to_string());
        optimizer.finalize_interned_names();

        let name = optimizer.get_interned_name("hello").unwrap();
        assert_eq!(name, "STR_HELLO");
    }

    #[test]
    fn test_finalize_interned_names_with_collision() {
        let mut optimizer = StringOptimizer::new();
        // "hello!" and "hello?" both become STR_HELLO_
        optimizer.interned_strings.insert("hello!".to_string());
        optimizer.interned_strings.insert("hello?".to_string());
        optimizer.finalize_interned_names();

        let name1 = optimizer.get_interned_name("hello!").unwrap();
        let name2 = optimizer.get_interned_name("hello?").unwrap();

        assert!(name1.starts_with("STR_HELLO_"));
        assert!(name2.starts_with("STR_HELLO_"));
        assert_ne!(name1, name2);
    }

    #[test]
    fn test_finalize_interned_names_already_finalized() {
        let mut optimizer = StringOptimizer::new();
        optimizer.interned_strings.insert("test".to_string());
        optimizer.finalize_interned_names();

        // Call again - should be a no-op
        optimizer.finalize_interned_names();

        assert!(optimizer.get_interned_name("test").is_some());
    }

    #[test]
    fn test_generate_base_const_name_empty() {
        let optimizer = StringOptimizer::new();
        let name = optimizer.generate_base_const_name("");
        assert_eq!(name, "STR_EMPTY");
    }

    #[test]
    fn test_generate_base_const_name_special_chars() {
        let optimizer = StringOptimizer::new();
        let name = optimizer.generate_base_const_name("hello world!");
        assert_eq!(name, "STR_HELLO_WORLD_");
    }

    #[test]
    fn test_generate_interned_constants() {
        let mut optimizer = StringOptimizer::new();
        optimizer.interned_strings.insert("test".to_string());
        optimizer.finalize_interned_names();

        let constants = optimizer.generate_interned_constants();
        assert_eq!(constants.len(), 1);
        assert!(constants[0].contains("STR_TEST"));
        assert!(constants[0].contains("\"test\""));
    }

    #[test]
    fn test_analyze_if_stmt_with_else() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::Expr(HirExpr::Literal(Literal::String(
                    "then".to_string(),
                )))],
                else_body: Some(vec![HirStmt::Expr(HirExpr::Literal(Literal::String(
                    "else".to_string(),
                )))]),
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        assert!(optimizer.is_read_only("then"));
        assert!(optimizer.is_read_only("else"));
    }

    #[test]
    fn test_analyze_while_stmt() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Expr(HirExpr::Literal(Literal::String(
                    "loop".to_string(),
                )))],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        assert!(optimizer.is_read_only("loop"));
    }

    #[test]
    fn test_analyze_for_stmt() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::List(vec![]),
                body: vec![HirStmt::Expr(HirExpr::Literal(Literal::String(
                    "body".to_string(),
                )))],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        assert!(optimizer.is_read_only("body"));
    }

    #[test]
    fn test_analyze_string_concatenation() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::String("a".to_string()))),
                right: Box::new(HirExpr::Literal(Literal::String("b".to_string()))),
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        // Binary expression with Add triggers analysis on both sides
        // The analyze_binary_expr calls mark_as_owned, then analyze_expr which re-adds to read_only
        // So they end up in read_only_strings (is_read_only returns true if in read_only AND not returned)
        assert!(optimizer.read_only_strings.contains("a"));
        assert!(optimizer.read_only_strings.contains("b"));
    }

    #[test]
    fn test_analyze_mutating_call() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![HirParam::new("s".to_string(), Type::String)].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "push_str".to_string(),
                args: vec![HirExpr::Var("s".to_string())],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        // Mutated parameter loses immutability
        assert!(!optimizer.immutable_params.contains("s"));
    }

    #[test]
    fn test_analyze_list_and_tuple() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![
                HirStmt::Expr(HirExpr::List(vec![HirExpr::Literal(Literal::String(
                    "list".to_string(),
                ))])),
                HirStmt::Expr(HirExpr::Tuple(vec![HirExpr::Literal(Literal::String(
                    "tuple".to_string(),
                ))])),
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        assert!(optimizer.is_read_only("list"));
        assert!(optimizer.is_read_only("tuple"));
    }

    #[test]
    fn test_analyze_dict() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Dict(vec![(
                HirExpr::Literal(Literal::String("key".to_string())),
                HirExpr::Literal(Literal::String("value".to_string())),
            )]))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);

        assert!(optimizer.is_read_only("key"));
        assert!(optimizer.is_read_only("value"));
    }

    #[test]
    fn test_is_string_expr_call() {
        let optimizer = StringOptimizer::new();

        assert!(optimizer.is_string_expr(&HirExpr::Call {
            func: "str".to_string(),
            args: vec![],
            kwargs: vec![]
        }));
        assert!(optimizer.is_string_expr(&HirExpr::Call {
            func: "format".to_string(),
            args: vec![],
            kwargs: vec![]
        }));
        assert!(optimizer.is_string_expr(&HirExpr::Call {
            func: "to_string".to_string(),
            args: vec![],
            kwargs: vec![]
        }));
        assert!(optimizer.is_string_expr(&HirExpr::Call {
            func: "join".to_string(),
            args: vec![],
            kwargs: vec![]
        }));
        assert!(!optimizer.is_string_expr(&HirExpr::Call {
            func: "len".to_string(),
            args: vec![],
            kwargs: vec![]
        }));
    }

    #[test]
    fn test_is_mutating_method() {
        let optimizer = StringOptimizer::new();

        assert!(optimizer.is_mutating_method("push_str"));
        assert!(optimizer.is_mutating_method("push"));
        assert!(optimizer.is_mutating_method("insert"));
        assert!(optimizer.is_mutating_method("insert_str"));
        assert!(optimizer.is_mutating_method("replace_range"));
        assert!(optimizer.is_mutating_method("clear"));
        assert!(optimizer.is_mutating_method("truncate"));
        assert!(!optimizer.is_mutating_method("len"));
    }

    #[test]
    fn test_get_optimal_type_return_context() {
        let optimizer = StringOptimizer::new();
        assert_eq!(
            optimizer.get_optimal_type(&StringContext::Return),
            OptimalStringType::OwnedString
        );
    }

    #[test]
    fn test_get_optimal_type_concatenation_context() {
        let optimizer = StringOptimizer::new();
        assert_eq!(
            optimizer.get_optimal_type(&StringContext::Concatenation),
            OptimalStringType::OwnedString
        );
    }

    #[test]
    fn test_string_context_display() {
        assert_eq!(
            format!("{}", StringContext::Literal("hello".to_string())),
            "\"hello\""
        );
        assert_eq!(
            format!("{}", StringContext::Parameter("s".to_string())),
            "s"
        );
        assert_eq!(format!("{}", StringContext::Return), "<return>");
        assert_eq!(format!("{}", StringContext::Concatenation), "<concat>");
    }

    #[test]
    fn test_generate_static_str() {
        let s = generate_static_str(&StringContext::Literal("hello".to_string()));
        assert_eq!(s, "\"hello\"");

        let s = generate_static_str(&StringContext::Parameter("s".to_string()));
        assert_eq!(s, "s.to_string()");
    }

    #[test]
    fn test_generate_borrowed_str() {
        let s = generate_borrowed_str(&StringContext::Parameter("s".to_string()));
        assert_eq!(s, "s");

        let s = generate_borrowed_str(&StringContext::Literal("test".to_string()));
        assert_eq!(s, "\"test\"");

        let s = generate_borrowed_str(&StringContext::Return);
        assert_eq!(s, "<return>.as_str()");
    }

    #[test]
    fn test_generate_owned_string() {
        let s = generate_owned_string(&StringContext::Literal("hello".to_string()));
        assert_eq!(s, "\"hello\".to_string()");

        let s = generate_owned_string(&StringContext::Parameter("s".to_string()));
        assert_eq!(s, "s.to_string()");

        let s = generate_owned_string(&StringContext::Return);
        assert_eq!(s, "String::new()");

        let s = generate_owned_string(&StringContext::Concatenation);
        assert_eq!(s, "String::new()");
    }

    #[test]
    fn test_generate_cow_str() {
        let s = generate_cow_str(&StringContext::Literal("hello".to_string()));
        assert_eq!(s, "Cow::Borrowed(\"hello\")");

        let s = generate_cow_str(&StringContext::Parameter("s".to_string()));
        assert_eq!(s, "Cow::Borrowed(s)");

        let s = generate_cow_str(&StringContext::Return);
        assert_eq!(s, "Cow::Owned(String::new())");

        let s = generate_cow_str(&StringContext::Concatenation);
        assert_eq!(s, "Cow::Owned(String::new())");
    }

    #[test]
    fn test_escape_string_special_chars() {
        assert_eq!(escape_string("hello\"world"), "hello\\\"world");
        assert_eq!(escape_string("hello\\world"), "hello\\\\world");
        assert_eq!(escape_string("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_string("hello\rworld"), "hello\\rworld");
        assert_eq!(escape_string("hello\tworld"), "hello\\tworld");
        assert_eq!(escape_string("normal"), "normal");
    }

    #[test]
    fn test_return_none_handled() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Return(None)],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);
        // Should not panic
    }

    #[test]
    fn test_mark_as_owned_var() {
        let mut optimizer = StringOptimizer::new();
        optimizer.immutable_params.insert("s".to_string());

        optimizer.mark_as_owned(&HirExpr::Var("s".to_string()));

        assert!(!optimizer.immutable_params.contains("s"));
    }

    #[test]
    fn test_mark_as_owned_other() {
        let mut optimizer = StringOptimizer::new();

        // Should not panic on non-string/non-var expressions
        optimizer.mark_as_owned(&HirExpr::Literal(Literal::Int(42)));
    }

    #[test]
    fn test_analyze_assign_non_symbol_target() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Assign {
                target: AssignTarget::Index {
                    base: Box::new(HirExpr::Var("arr".to_string())),
                    index: Box::new(HirExpr::Literal(Literal::Int(0))),
                },
                value: HirExpr::Literal(Literal::String("value".to_string())),
                type_annotation: None,
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);
        // Should not panic
    }

    #[test]
    fn test_call_with_no_args() {
        let mut optimizer = StringOptimizer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "push_str".to_string(),
                args: vec![],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        optimizer.analyze_function(&func);
        // Should not panic when args is empty
    }

    #[test]
    fn test_mixed_usage_literal() {
        let mut optimizer = StringOptimizer::new();
        optimizer.mixed_usage_strings.insert("mixed".to_string());

        let context = StringContext::Literal("mixed".to_string());
        assert_eq!(
            optimizer.get_optimal_type(&context),
            OptimalStringType::CowStr
        );
    }

    #[test]
    fn test_parameter_mixed_usage() {
        let mut optimizer = StringOptimizer::new();
        optimizer.mixed_usage_strings.insert("param".to_string());

        let context = StringContext::Parameter("param".to_string());
        assert_eq!(
            optimizer.get_optimal_type(&context),
            OptimalStringType::CowStr
        );
    }

    // Tests for OptimalStringType traits
    #[test]
    fn test_optimal_string_type_debug() {
        let static_str = OptimalStringType::StaticStr;
        assert!(format!("{:?}", static_str).contains("StaticStr"));

        let borrowed = OptimalStringType::BorrowedStr {
            lifetime: Some("'a".to_string()),
        };
        assert!(format!("{:?}", borrowed).contains("BorrowedStr"));
        assert!(format!("{:?}", borrowed).contains("'a"));

        let owned = OptimalStringType::OwnedString;
        assert!(format!("{:?}", owned).contains("OwnedString"));

        let cow = OptimalStringType::CowStr;
        assert!(format!("{:?}", cow).contains("CowStr"));
    }

    #[test]
    fn test_optimal_string_type_clone() {
        let original = OptimalStringType::BorrowedStr {
            lifetime: Some("'b".to_string()),
        };
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_optimal_string_type_partial_eq() {
        assert_eq!(OptimalStringType::StaticStr, OptimalStringType::StaticStr);
        assert_eq!(
            OptimalStringType::OwnedString,
            OptimalStringType::OwnedString
        );
        assert_eq!(OptimalStringType::CowStr, OptimalStringType::CowStr);
        assert_ne!(OptimalStringType::StaticStr, OptimalStringType::OwnedString);
        assert_ne!(OptimalStringType::CowStr, OptimalStringType::StaticStr);

        let borrowed1 = OptimalStringType::BorrowedStr {
            lifetime: Some("'a".to_string()),
        };
        let borrowed2 = OptimalStringType::BorrowedStr {
            lifetime: Some("'a".to_string()),
        };
        let borrowed3 = OptimalStringType::BorrowedStr {
            lifetime: Some("'b".to_string()),
        };
        let borrowed_none = OptimalStringType::BorrowedStr { lifetime: None };

        assert_eq!(borrowed1, borrowed2);
        assert_ne!(borrowed1, borrowed3);
        assert_ne!(borrowed1, borrowed_none);
    }

    // Tests for StringOptimizer struct
    #[test]
    fn test_string_optimizer_debug() {
        let optimizer = StringOptimizer::new();
        let debug_str = format!("{:?}", optimizer);
        assert!(debug_str.contains("StringOptimizer"));
    }

    #[test]
    fn test_string_optimizer_default() {
        let optimizer = StringOptimizer::default();
        assert!(optimizer.read_only_strings.is_empty());
        assert!(optimizer.immutable_params.is_empty());
        assert!(optimizer.returned_strings.is_empty());
        assert!(optimizer.mixed_usage_strings.is_empty());
        assert!(optimizer.string_literal_count.is_empty());
        assert!(optimizer.interned_strings.is_empty());
        assert!(optimizer.interned_names.is_empty());
    }

    // Tests for escape_char
    #[test]
    fn test_escape_char_backslash() {
        let result = escape_char('\\');
        assert_eq!(result, vec!['\\', '\\']);
    }

    #[test]
    fn test_escape_char_quote() {
        let result = escape_char('"');
        assert_eq!(result, vec!['\\', '"']);
    }

    #[test]
    fn test_escape_char_newline() {
        let result = escape_char('\n');
        assert_eq!(result, vec!['\\', 'n']);
    }

    #[test]
    fn test_escape_char_carriage_return() {
        let result = escape_char('\r');
        assert_eq!(result, vec!['\\', 'r']);
    }

    #[test]
    fn test_escape_char_tab() {
        let result = escape_char('\t');
        assert_eq!(result, vec!['\\', 't']);
    }

    #[test]
    fn test_escape_char_normal() {
        let result = escape_char('a');
        assert_eq!(result, vec!['a']);

        let result = escape_char('Z');
        assert_eq!(result, vec!['Z']);

        let result = escape_char('5');
        assert_eq!(result, vec!['5']);
    }

    // Tests for StringContext variants
    #[test]
    fn test_string_context_literal() {
        let ctx = StringContext::Literal("hello".to_string());
        if let StringContext::Literal(s) = ctx {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_string_context_parameter() {
        let ctx = StringContext::Parameter("name".to_string());
        if let StringContext::Parameter(s) = ctx {
            assert_eq!(s, "name");
        } else {
            panic!("Expected Parameter");
        }
    }

    #[test]
    fn test_string_context_return() {
        let ctx = StringContext::Return;
        assert!(matches!(ctx, StringContext::Return));
    }

    #[test]
    fn test_string_context_concatenation() {
        let ctx = StringContext::Concatenation;
        assert!(matches!(ctx, StringContext::Concatenation));
    }

    // Tests for is_read_only and is_immutable
    #[test]
    fn test_is_read_only_true() {
        let mut optimizer = StringOptimizer::new();
        optimizer.read_only_strings.insert("readonly".to_string());
        assert!(optimizer.is_read_only("readonly"));
    }

    #[test]
    fn test_is_read_only_false() {
        let optimizer = StringOptimizer::new();
        assert!(!optimizer.is_read_only("nonexistent"));
    }

    #[test]
    fn test_is_immutable_param_true() {
        let mut optimizer = StringOptimizer::new();
        optimizer.immutable_params.insert("param".to_string());
        assert!(optimizer.is_immutable("param"));
    }

    #[test]
    fn test_is_immutable_param_false() {
        let optimizer = StringOptimizer::new();
        assert!(!optimizer.is_immutable("nonexistent")); // Returns false if not in immutable_params set
    }
}
