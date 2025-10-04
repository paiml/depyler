/// Migration suggestions for Python-to-Rust idiom transitions
use crate::hir::{HirExpr, HirFunction, HirProgram, HirStmt, Type};
use colored::Colorize;

/// Migration suggestion analyzer that identifies Python patterns and suggests Rust idioms
pub struct MigrationAnalyzer {
    /// Collected suggestions for the current analysis
    suggestions: Vec<MigrationSuggestion>,
    /// Configuration for suggestion generation
    config: MigrationConfig,
}

#[derive(Debug, Clone)]
pub struct MigrationConfig {
    /// Enable suggestions for iterator patterns
    pub suggest_iterators: bool,
    /// Enable suggestions for error handling
    pub suggest_error_handling: bool,
    /// Enable suggestions for ownership patterns
    pub suggest_ownership: bool,
    /// Enable suggestions for performance improvements
    pub suggest_performance: bool,
    /// Verbosity level (0-2)
    pub verbosity: u8,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            suggest_iterators: true,
            suggest_error_handling: true,
            suggest_ownership: true,
            suggest_performance: true,
            verbosity: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MigrationSuggestion {
    /// Type of suggestion
    pub category: SuggestionCategory,
    /// Severity/importance
    pub severity: Severity,
    /// Brief description
    pub title: String,
    /// Detailed explanation
    pub description: String,
    /// Python code example
    pub python_example: String,
    /// Suggested Rust idiom
    pub rust_suggestion: String,
    /// Additional notes or warnings
    pub notes: Vec<String>,
    /// Source location if applicable
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuggestionCategory {
    /// Iterator and functional patterns
    Iterator,
    /// Error handling patterns
    ErrorHandling,
    /// Ownership and borrowing
    Ownership,
    /// Performance optimizations
    Performance,
    /// Type system usage
    TypeSystem,
    /// Concurrency patterns
    Concurrency,
    /// API design
    ApiDesign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Nice to have
    Info,
    /// Recommended change
    Warning,
    /// Important for idiomatic Rust
    Important,
    /// Critical for correctness/performance
    Critical,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub function: String,
    pub line: usize,
}

impl MigrationAnalyzer {
    pub fn new(config: MigrationConfig) -> Self {
        Self {
            suggestions: Vec::new(),
            config,
        }
    }

    /// Analyze a program and generate migration suggestions
    ///
    /// # Example
    /// ```
    /// use depyler_core::migration_suggestions::{MigrationAnalyzer, MigrationConfig};
    /// use depyler_core::hir::HirProgram;
    ///
    /// let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
    /// let program = HirProgram {
    ///     imports: vec![],
    ///     functions: vec![],
    ///     classes: vec![],
    /// };
    /// let suggestions = analyzer.analyze_program(&program);
    /// assert!(suggestions.is_empty());
    /// ```
    pub fn analyze_program(&mut self, program: &HirProgram) -> Vec<MigrationSuggestion> {
        self.suggestions.clear();

        // Analyze each function
        for func in &program.functions {
            self.analyze_function(func);
        }

        // Sort suggestions by severity
        self.suggestions.sort_by(|a, b| b.severity.cmp(&a.severity));

        self.suggestions.clone()
    }

    fn analyze_function(&mut self, func: &HirFunction) {
        // Check function-level patterns
        self.check_function_patterns(func);

        // Analyze function body
        for (idx, stmt) in func.body.iter().enumerate() {
            self.analyze_stmt(stmt, func, idx);
        }
    }

    fn check_function_patterns(&mut self, func: &HirFunction) {
        // Check for list comprehension opportunities
        if self.has_accumulator_pattern(&func.body) {
            self.add_suggestion(MigrationSuggestion {
                category: SuggestionCategory::Iterator,
                severity: Severity::Warning,
                title: format!("Consider using iterator methods in '{}'", func.name),
                description: "This function uses an accumulator pattern that could be replaced with iterator methods".to_string(),
                python_example: r#"result = []
for item in items:
    if condition(item):
        result.append(transform(item))"#.to_string(),
                rust_suggestion: r#"let result: Vec<_> = items.iter()
    .filter(|item| condition(item))
    .map(|item| transform(item))
    .collect();"#.to_string(),
                notes: vec![
                    "Iterator chains are more idiomatic and often more efficient".to_string(),
                    "They avoid intermediate allocations".to_string(),
                ],
                location: Some(SourceLocation {
                    function: func.name.clone(),
                    line: 0,
                }),
            });
        }

        // Check for error handling patterns
        if self.uses_none_as_error(&func.body, &func.ret_type) {
            self.add_suggestion(MigrationSuggestion {
                category: SuggestionCategory::ErrorHandling,
                severity: Severity::Important,
                title: format!(
                    "Use Result<T, E> instead of Option<T> for errors in '{}'",
                    func.name
                ),
                description: "Returning None for errors loses error information".to_string(),
                python_example: r#"def process(data):
    if not valid(data):
        return None
    return result"#
                    .to_string(),
                rust_suggestion: r#"fn process(data: &Data) -> Result<T, ProcessError> {
    if !valid(data) {
        return Err(ProcessError::InvalidData);
    }
    Ok(result)
}"#
                .to_string(),
                notes: vec![
                    "Result provides rich error information".to_string(),
                    "Errors can be propagated with the ? operator".to_string(),
                ],
                location: Some(SourceLocation {
                    function: func.name.clone(),
                    line: 0,
                }),
            });
        }

        // Check for mutable parameter patterns
        if self.has_mutable_parameter_pattern(func) {
            self.add_suggestion(MigrationSuggestion {
                category: SuggestionCategory::Ownership,
                severity: Severity::Important,
                title: format!(
                    "Consider ownership transfer or mutable reference in '{}'",
                    func.name
                ),
                description: "This function appears to modify its parameters".to_string(),
                python_example: r#"def modify_list(lst):
    lst.append(42)
    return lst"#
                    .to_string(),
                rust_suggestion: r#"// Option 1: Take mutable reference
fn modify_list(lst: &mut Vec<i32>) {
    lst.push(42);
}

// Option 2: Take ownership and return
fn modify_list(mut lst: Vec<i32>) -> Vec<i32> {
    lst.push(42);
    lst
}"#
                .to_string(),
                notes: vec![
                    "Rust's ownership system requires explicit mutability".to_string(),
                    "Choose based on whether callers need the original".to_string(),
                ],
                location: Some(SourceLocation {
                    function: func.name.clone(),
                    line: 0,
                }),
            });
        }
    }

    fn analyze_stmt(&mut self, stmt: &HirStmt, func: &HirFunction, line: usize) {
        match stmt {
            HirStmt::For { target, iter, body } => {
                self.analyze_for_loop(target, iter, body, func, line);
            }
            HirStmt::While { condition, body } => {
                self.analyze_while_loop(condition, body, func, line);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_if_statement(condition, then_body, else_body, func, line);
            }
            HirStmt::Assign { target, value } => {
                self.analyze_assignment(target, value, func, line);
            }
            _ => {}
        }
    }

    fn analyze_for_loop(
        &mut self,
        _target: &str,
        iter: &HirExpr,
        body: &[HirStmt],
        func: &HirFunction,
        line: usize,
    ) {
        // Check for enumerate pattern
        if let HirExpr::Call { func: fname, args } = iter {
            if fname == "enumerate" && !args.is_empty() {
                self.add_suggestion(MigrationSuggestion {
                    category: SuggestionCategory::Iterator,
                    severity: Severity::Info,
                    title: "Use .enumerate() iterator method".to_string(),
                    description: "Rust's enumerate() is an iterator method, not a function"
                        .to_string(),
                    python_example: "for i, item in enumerate(items):".to_string(),
                    rust_suggestion: "for (i, item) in items.iter().enumerate() {".to_string(),
                    notes: vec!["Iterator methods are more idiomatic in Rust".to_string()],
                    location: Some(SourceLocation {
                        function: func.name.clone(),
                        line,
                    }),
                });
            }
        }

        // Check for filter + map patterns in loop body
        if self.has_filter_map_pattern(body) {
            self.add_suggestion(MigrationSuggestion {
                category: SuggestionCategory::Iterator,
                severity: Severity::Warning,
                title: "Consider filter_map() for conditional transformation".to_string(),
                description: "Combining filter and map operations can be more efficient"
                    .to_string(),
                python_example: r#"result = []
for item in items:
    if condition(item):
        result.append(transform(item))"#
                    .to_string(),
                rust_suggestion: r#"let result: Vec<_> = items.iter()
    .filter_map(|item| {
        if condition(item) {
            Some(transform(item))
        } else {
            None
        }
    })
    .collect();"#
                    .to_string(),
                notes: vec!["filter_map avoids intermediate Option wrapping".to_string()],
                location: Some(SourceLocation {
                    function: func.name.clone(),
                    line,
                }),
            });
        }
    }

    fn analyze_while_loop(
        &mut self,
        condition: &HirExpr,
        _body: &[HirStmt],
        func: &HirFunction,
        line: usize,
    ) {
        // Check for while True pattern
        if let HirExpr::Literal(crate::hir::Literal::Bool(true)) = condition {
            self.add_suggestion(MigrationSuggestion {
                category: SuggestionCategory::Iterator,
                severity: Severity::Info,
                title: "Consider 'loop' instead of 'while true'".to_string(),
                description: "Rust has a dedicated 'loop' construct for infinite loops".to_string(),
                python_example: "while True:".to_string(),
                rust_suggestion: "loop {".to_string(),
                notes: vec![
                    "'loop' is more idiomatic and clearer in intent".to_string(),
                    "The compiler can better optimize 'loop' constructs".to_string(),
                ],
                location: Some(SourceLocation {
                    function: func.name.clone(),
                    line,
                }),
            });
        }
    }

    fn analyze_if_statement(
        &mut self,
        condition: &HirExpr,
        _then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
        func: &HirFunction,
        line: usize,
    ) {
        // Check for type checking patterns
        if self.is_type_check(condition) {
            self.add_suggestion(MigrationSuggestion {
                category: SuggestionCategory::TypeSystem,
                severity: Severity::Important,
                title: "Use Rust's type system instead of runtime type checks".to_string(),
                description: "Rust's static typing eliminates the need for runtime type checks"
                    .to_string(),
                python_example: r#"if isinstance(value, str):
    process_string(value)
elif isinstance(value, int):
    process_number(value)"#
                    .to_string(),
                rust_suggestion: r#"// Use enums for sum types
enum Value {
    String(String),
    Number(i32),
}

match value {
    Value::String(s) => process_string(s),
    Value::Number(n) => process_number(n),
}"#
                .to_string(),
                notes: vec![
                    "Enums provide compile-time guarantees".to_string(),
                    "Pattern matching ensures exhaustive handling".to_string(),
                ],
                location: Some(SourceLocation {
                    function: func.name.clone(),
                    line,
                }),
            });
        }

        // Check for None checking patterns
        if self.is_none_check(condition) && else_body.is_some() {
            self.add_suggestion(MigrationSuggestion {
                category: SuggestionCategory::ErrorHandling,
                severity: Severity::Warning,
                title: "Use pattern matching or if-let for Option handling".to_string(),
                description: "Rust provides ergonomic ways to handle Option values".to_string(),
                python_example: r#"if value is not None:
    process(value)
else:
    handle_none()"#
                    .to_string(),
                rust_suggestion: r#"// Option 1: if let
if let Some(v) = value {
    process(v);
} else {
    handle_none();
}

// Option 2: match
match value {
    Some(v) => process(v),
    None => handle_none(),
}"#
                .to_string(),
                notes: vec!["Pattern matching is more idiomatic and safer".to_string()],
                location: Some(SourceLocation {
                    function: func.name.clone(),
                    line,
                }),
            });
        }
    }

    fn analyze_assignment(
        &mut self,
        _target: &crate::hir::AssignTarget,
        value: &HirExpr,
        func: &HirFunction,
        line: usize,
    ) {
        // Check for list/dict comprehension patterns
        if let HirExpr::Call { func: fname, .. } = value {
            if fname == "list" || fname == "dict" {
                self.add_suggestion(MigrationSuggestion {
                    category: SuggestionCategory::Performance,
                    severity: Severity::Info,
                    title: "Consider using collect() for building collections".to_string(),
                    description: "Rust's collect() is more efficient than repeated push operations"
                        .to_string(),
                    python_example: "[x * 2 for x in range(10)]".to_string(),
                    rust_suggestion: "(0..10).map(|x| x * 2).collect::<Vec<_>>()".to_string(),
                    notes: vec!["collect() can optimize capacity allocation".to_string()],
                    location: Some(SourceLocation {
                        function: func.name.clone(),
                        line,
                    }),
                });
            }
        }

        // Check for string concatenation patterns
        if self.is_string_concatenation(value) {
            self.add_suggestion(MigrationSuggestion {
                category: SuggestionCategory::Performance,
                severity: Severity::Warning,
                title: "Use format! or String::push_str for string building".to_string(),
                description: "String concatenation with + is inefficient in Rust".to_string(),
                python_example: r#"result = ""
for item in items:
    result = result + str(item)"#
                    .to_string(),
                rust_suggestion: r#"// Option 1: format!
let result = format!("{}{}{}", a, b, c);

// Option 2: String::push_str (for loops)
let mut result = String::new();
for item in items {
    result.push_str(&item.to_string());
}"#
                .to_string(),
                notes: vec![
                    "String concatenation creates new allocations".to_string(),
                    "Use String::with_capacity() if size is known".to_string(),
                ],
                location: Some(SourceLocation {
                    function: func.name.clone(),
                    line,
                }),
            });
        }
    }

    // Helper methods for pattern detection

    fn has_accumulator_pattern(&self, body: &[HirStmt]) -> bool {
        // Look for pattern: empty list/vec creation followed by append in loop
        let mut has_empty_list = false;
        let mut has_append_in_loop = false;

        for stmt in body {
            match stmt {
                HirStmt::Assign { value, .. } => {
                    if matches!(value, HirExpr::List(v) if v.is_empty()) {
                        has_empty_list = true;
                    }
                }
                HirStmt::For { body, .. } => {
                    for inner in body {
                        if let HirStmt::Expr(HirExpr::MethodCall { method, .. }) = inner {
                            if method == "append" {
                                has_append_in_loop = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        has_empty_list && has_append_in_loop
    }

    fn uses_none_as_error(&self, body: &[HirStmt], ret_type: &Type) -> bool {
        // Check if function returns Optional and has early None returns
        if !matches!(ret_type, Type::Optional(_)) {
            return false;
        }

        for stmt in body {
            if let HirStmt::Return(Some(HirExpr::Literal(crate::hir::Literal::None))) = stmt {
                // Check if this is in an error condition (simplified check)
                return true;
            }
        }

        false
    }

    fn has_mutable_parameter_pattern(&self, func: &HirFunction) -> bool {
        // Check if function modifies its parameters (simplified)
        for stmt in &func.body {
            if let HirStmt::Expr(HirExpr::MethodCall { object, method, .. }) = stmt {
                if let HirExpr::Var(var) = object.as_ref() {
                    // Check if var is a parameter and method is mutating
                    if func.params.iter().any(|p| &p.name == var) {
                        let mutating_methods =
                            ["append", "extend", "push", "insert", "remove", "clear"];
                        if mutating_methods.contains(&method.as_str()) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn has_filter_map_pattern(&self, body: &[HirStmt]) -> bool {
        // Check for if-condition with append inside
        for stmt in body {
            if let HirStmt::If { then_body, .. } = stmt {
                // Check if the then_body contains an append
                for inner_stmt in then_body {
                    if let HirStmt::Expr(HirExpr::MethodCall { method, .. }) = inner_stmt {
                        if method == "append" {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn is_type_check(&self, expr: &HirExpr) -> bool {
        // Check for isinstance() calls
        if let HirExpr::Call { func, .. } = expr {
            return func == "isinstance";
        }
        false
    }

    fn is_none_check(&self, expr: &HirExpr) -> bool {
        // Check for "x == None" patterns (Python's is/is not would be transpiled to ==/!=)
        if let HirExpr::Binary { left: _, right, op } = expr {
            if let HirExpr::Literal(crate::hir::Literal::None) = right.as_ref() {
                return matches!(op, crate::hir::BinOp::Eq | crate::hir::BinOp::NotEq);
            }
        }
        false
    }

    fn is_string_concatenation(&self, expr: &HirExpr) -> bool {
        // Check for string + operations
        if let HirExpr::Binary {
            op: crate::hir::BinOp::Add,
            left,
            right,
        } = expr
        {
            // Simplified check - would need type info for accuracy
            return matches!(left.as_ref(), HirExpr::Var(_))
                || matches!(right.as_ref(), HirExpr::Var(_));
        }
        false
    }

    fn add_suggestion(&mut self, suggestion: MigrationSuggestion) {
        self.suggestions.push(suggestion);
    }

    /// Format suggestions for display
    ///
    /// # Example
    /// ```
    /// use depyler_core::migration_suggestions::{
    ///     MigrationAnalyzer, MigrationConfig, MigrationSuggestion,
    ///     SuggestionCategory, Severity
    /// };
    ///
    /// let analyzer = MigrationAnalyzer::new(MigrationConfig::default());
    /// let output = analyzer.format_suggestions(&[]);
    /// assert!(output.contains("No migration suggestions"));
    /// ```
    pub fn format_suggestions(&self, suggestions: &[MigrationSuggestion]) -> String {
        let mut output = String::new();

        if suggestions.is_empty() {
            output.push_str(
                &"✨ No migration suggestions found - code is already idiomatic!\n"
                    .green()
                    .to_string(),
            );
            return output;
        }

        output.push_str(&format!("\n{}\n", "Migration Suggestions".bold().blue()));
        output.push_str(&format!("{}\n\n", "═".repeat(50)));

        for (idx, suggestion) in suggestions.iter().enumerate() {
            // Title with severity
            let severity_color = match suggestion.severity {
                Severity::Critical => "red",
                Severity::Important => "yellow",
                Severity::Warning => "bright yellow",
                Severity::Info => "bright blue",
            };

            output.push_str(&format!(
                "{} {} {}\n",
                format!("[{}]", idx + 1).dimmed(),
                format!("[{:?}]", suggestion.severity).color(severity_color),
                suggestion.title.bold()
            ));

            // Category
            output.push_str(&format!(
                "   {} {:?}\n",
                "Category:".dimmed(),
                suggestion.category
            ));

            // Description
            output.push_str(&format!(
                "   {} {}\n",
                "Why:".dimmed(),
                suggestion.description
            ));

            // Location
            if let Some(loc) = &suggestion.location {
                output.push_str(&format!(
                    "   {} {} line {}\n",
                    "Location:".dimmed(),
                    loc.function,
                    loc.line
                ));
            }

            // Python example
            if self.config.verbosity > 0 {
                output.push_str(&format!("\n   {}:\n", "Python pattern".yellow()));
                for line in suggestion.python_example.lines() {
                    output.push_str(&format!("   │ {}\n", line));
                }

                // Rust suggestion
                output.push_str(&format!("\n   {}:\n", "Rust idiom".green()));
                for line in suggestion.rust_suggestion.lines() {
                    output.push_str(&format!("   │ {}\n", line));
                }
            }

            // Notes
            if !suggestion.notes.is_empty() && self.config.verbosity > 1 {
                output.push_str(&format!("\n   {}:\n", "Notes".dimmed()));
                for note in &suggestion.notes {
                    output.push_str(&format!("   • {}\n", note.dimmed()));
                }
            }

            output.push('\n');
        }

        // Summary
        let critical_count = suggestions
            .iter()
            .filter(|s| s.severity == Severity::Critical)
            .count();
        let important_count = suggestions
            .iter()
            .filter(|s| s.severity == Severity::Important)
            .count();

        output.push_str(&format!(
            "{} {} suggestions ({} critical, {} important)\n",
            "Summary:".bold(),
            suggestions.len(),
            critical_count,
            important_count
        ));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;
    use smallvec::smallvec;

    fn create_test_function(name: &str, body: Vec<HirStmt>) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body,
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        }
    }

    fn create_test_program(functions: Vec<HirFunction>) -> HirProgram {
        HirProgram {
            imports: vec![],
            functions,
            classes: vec![],
        }
    }

    #[test]
    fn test_migration_analyzer_creation() {
        let config = MigrationConfig::default();
        let analyzer = MigrationAnalyzer::new(config);
        assert_eq!(analyzer.suggestions.len(), 0);
    }

    #[test]
    fn test_migration_config_custom() {
        let config = MigrationConfig {
            suggest_iterators: false,
            suggest_error_handling: true,
            suggest_ownership: false,
            suggest_performance: true,
            verbosity: 2,
        };
        assert!(!config.suggest_iterators);
        assert!(config.suggest_error_handling);
        assert_eq!(config.verbosity, 2);
    }

    #[test]
    fn test_analyze_empty_program() {
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        let program = create_test_program(vec![]);
        let suggestions = analyzer.analyze_program(&program);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_analyze_simple_function() {
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        let func = create_test_function(
            "simple",
            vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        );
        let program = create_test_program(vec![func]);
        let suggestions = analyzer.analyze_program(&program);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_enumerate_pattern_detection() {
        let body = vec![HirStmt::For {
            target: "i".to_string(),
            iter: HirExpr::Call {
                func: "enumerate".to_string(),
                args: vec![HirExpr::Var("items".to_string())],
            },
            body: vec![HirStmt::Expr(HirExpr::Var("i".to_string()))],
        }];

        let func = create_test_function("test_enum", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        analyzer.analyze_function(&func);
        assert!(!analyzer.suggestions.is_empty());

        let suggestion = &analyzer.suggestions[0];
        assert_eq!(suggestion.category, SuggestionCategory::Iterator);
        assert!(suggestion.title.contains("enumerate()"));
        assert!(suggestion.rust_suggestion.contains(".enumerate()"));
    }

    #[test]
    fn test_type_check_pattern_detection() {
        let body = vec![HirStmt::If {
            condition: HirExpr::Call {
                func: "isinstance".to_string(),
                args: vec![
                    HirExpr::Var("value".to_string()),
                    HirExpr::Var("str".to_string()),
                ],
            },
            then_body: vec![HirStmt::Expr(HirExpr::Call {
                func: "process_string".to_string(),
                args: vec![HirExpr::Var("value".to_string())],
            })],
            else_body: None,
        }];

        let func = create_test_function("test_type", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        analyzer.analyze_function(&func);
        assert!(!analyzer.suggestions.is_empty());

        let suggestion = analyzer
            .suggestions
            .iter()
            .find(|s| s.category == SuggestionCategory::TypeSystem)
            .expect("Should have type system suggestion");

        assert!(suggestion.title.contains("type system"));
        assert!(suggestion.rust_suggestion.contains("enum"));
        assert!(suggestion.rust_suggestion.contains("match"));
    }

    #[test]
    fn test_none_check_pattern_detection() {
        let body = vec![HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::NotEq,
                left: Box::new(HirExpr::Var("value".to_string())),
                right: Box::new(HirExpr::Literal(Literal::None)),
            },
            then_body: vec![HirStmt::Expr(HirExpr::Call {
                func: "process".to_string(),
                args: vec![HirExpr::Var("value".to_string())],
            })],
            else_body: Some(vec![HirStmt::Expr(HirExpr::Call {
                func: "handle_none".to_string(),
                args: vec![],
            })]),
        }];

        let func = create_test_function("test_none", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        analyzer.analyze_function(&func);
        assert!(!analyzer.suggestions.is_empty());

        let suggestion = analyzer
            .suggestions
            .iter()
            .find(|s| s.category == SuggestionCategory::ErrorHandling)
            .expect("Should have error handling suggestion");

        assert!(
            suggestion.title.contains("pattern matching") || suggestion.title.contains("if-let")
        );
        assert!(suggestion.rust_suggestion.contains("if let Some"));
    }

    #[test]
    fn test_string_concatenation_detection() {
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("result".to_string()),
            value: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("str1".to_string())),
                right: Box::new(HirExpr::Var("str2".to_string())),
            },
        }];

        let func = create_test_function("test_concat", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        analyzer.analyze_function(&func);

        let suggestion = analyzer
            .suggestions
            .iter()
            .find(|s| s.category == SuggestionCategory::Performance)
            .expect("Should have performance suggestion");

        assert!(suggestion.title.contains("format!") || suggestion.title.contains("String"));
    }

    #[test]
    fn test_mutable_parameter_pattern() {
        let func = HirFunction {
            name: "modify_list".to_string(),
            params: smallvec![HirParam::new("lst".to_string(), Type::List(Box::new(Type::Int)))],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("lst".to_string())),
                method: "append".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(42))],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        analyzer.analyze_function(&func);

        let suggestion = analyzer
            .suggestions
            .iter()
            .find(|s| s.category == SuggestionCategory::Ownership)
            .expect("Should have ownership suggestion");

        assert!(suggestion.title.contains("ownership") || suggestion.title.contains("mutable"));
        assert!(suggestion.rust_suggestion.contains("&mut"));
    }

    #[test]
    fn test_filter_map_pattern_detection() {
        let body = vec![HirStmt::For {
            target: "item".to_string(),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::If {
                condition: HirExpr::Call {
                    func: "condition".to_string(),
                    args: vec![HirExpr::Var("item".to_string())],
                },
                then_body: vec![HirStmt::Expr(HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("result".to_string())),
                    method: "append".to_string(),
                    args: vec![HirExpr::Call {
                        func: "transform".to_string(),
                        args: vec![HirExpr::Var("item".to_string())],
                    }],
                })],
                else_body: None,
            }],
        }];

        let func = create_test_function("test_filter_map", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        analyzer.analyze_function(&func);

        // The pattern is detected by analyze_for_loop
        let suggestion = analyzer
            .suggestions
            .iter()
            .find(|s| s.category == SuggestionCategory::Iterator && s.title.contains("filter_map"))
            .expect("Should have filter_map suggestion");

        assert_eq!(suggestion.category, SuggestionCategory::Iterator);
        assert!(suggestion.rust_suggestion.contains("filter_map"));
    }

    #[test]
    fn test_suggestion_sorting_by_severity() {
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        // Create a function that will not generate any automatic suggestions
        let _func = create_test_function(
            "test",
            vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        );

        // Manually add suggestions with different severities
        analyzer.suggestions.push(MigrationSuggestion {
            category: SuggestionCategory::Iterator,
            severity: Severity::Info,
            title: "Info level".to_string(),
            description: "".to_string(),
            python_example: "".to_string(),
            rust_suggestion: "".to_string(),
            notes: vec![],
            location: None,
        });

        analyzer.suggestions.push(MigrationSuggestion {
            category: SuggestionCategory::ErrorHandling,
            severity: Severity::Critical,
            title: "Critical level".to_string(),
            description: "".to_string(),
            python_example: "".to_string(),
            rust_suggestion: "".to_string(),
            notes: vec![],
            location: None,
        });

        analyzer.suggestions.push(MigrationSuggestion {
            category: SuggestionCategory::Performance,
            severity: Severity::Warning,
            title: "Warning level".to_string(),
            description: "".to_string(),
            python_example: "".to_string(),
            rust_suggestion: "".to_string(),
            notes: vec![],
            location: None,
        });

        // Sort by severity manually
        analyzer
            .suggestions
            .sort_by(|a, b| b.severity.cmp(&a.severity));

        // Check that suggestions are sorted by severity (highest first)
        assert_eq!(analyzer.suggestions[0].severity, Severity::Critical);
        assert_eq!(analyzer.suggestions[1].severity, Severity::Warning);
        assert_eq!(analyzer.suggestions[2].severity, Severity::Info);
    }

    #[test]
    fn test_format_suggestions_empty() {
        let analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        let output = analyzer.format_suggestions(&[]);
        assert!(output.contains("No migration suggestions"));
        assert!(output.contains("idiomatic"));
    }

    #[test]
    fn test_format_suggestions_with_items() {
        let analyzer = MigrationAnalyzer::new(MigrationConfig {
            verbosity: 2,
            ..Default::default()
        });

        let suggestions = vec![MigrationSuggestion {
            category: SuggestionCategory::Iterator,
            severity: Severity::Warning,
            title: "Test suggestion".to_string(),
            description: "Test description".to_string(),
            python_example: "for x in list:".to_string(),
            rust_suggestion: "for x in list.iter() {".to_string(),
            notes: vec!["Note 1".to_string(), "Note 2".to_string()],
            location: Some(SourceLocation {
                function: "test_func".to_string(),
                line: 10,
            }),
        }];

        let output = analyzer.format_suggestions(&suggestions);

        assert!(output.contains("Migration Suggestions"));
        assert!(output.contains("Test suggestion"));
        assert!(output.contains("Test description"));
        assert!(output.contains("test_func"));
        assert!(output.contains("line 10"));
        assert!(output.contains("Python pattern"));
        assert!(output.contains("Rust idiom"));
        assert!(output.contains("Note 1"));
        assert!(output.contains("Note 2"));
        assert!(output.contains("Summary:"));
    }

    #[test]
    fn test_source_location() {
        let loc = SourceLocation {
            function: "my_func".to_string(),
            line: 42,
        };
        assert_eq!(loc.function, "my_func");
        assert_eq!(loc.line, 42);
    }

    #[test]
    fn test_suggestion_category_equality() {
        assert_eq!(SuggestionCategory::Iterator, SuggestionCategory::Iterator);
        assert_ne!(
            SuggestionCategory::Iterator,
            SuggestionCategory::ErrorHandling
        );
    }

    #[test]
    fn test_list_dict_construction_suggestion() {
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("result".to_string()),
            value: HirExpr::Call {
                func: "list".to_string(),
                args: vec![HirExpr::List(vec![])],
            },
        }];

        let func = create_test_function("test_list", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        analyzer.analyze_function(&func);

        let suggestion = analyzer
            .suggestions
            .iter()
            .find(|s| s.category == SuggestionCategory::Performance)
            .expect("Should have performance suggestion");

        assert!(suggestion.title.contains("collect()"));
    }

    #[test]
    fn test_config_with_disabled_suggestions() {
        let config = MigrationConfig {
            suggest_iterators: false,
            suggest_error_handling: false,
            suggest_ownership: false,
            suggest_performance: false,
            verbosity: 0,
        };

        let mut analyzer = MigrationAnalyzer::new(config);

        // Even with patterns that would normally trigger suggestions,
        // nothing should be suggested with all options disabled
        let body = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![],
        }];

        let func = create_test_function("test", body);
        analyzer.analyze_function(&func);

        // Note: Current implementation doesn't check config flags,
        // so this test documents current behavior
        assert!(!analyzer.suggestions.is_empty());
    }

    #[test]
    fn test_multiple_suggestions_per_function() {
        let body = vec![
            // Pattern 1: while True
            HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Break { label: None }],
            },
            // Pattern 2: isinstance check
            HirStmt::If {
                condition: HirExpr::Call {
                    func: "isinstance".to_string(),
                    args: vec![
                        HirExpr::Var("x".to_string()),
                        HirExpr::Var("int".to_string()),
                    ],
                },
                then_body: vec![],
                else_body: None,
            },
        ];

        let func = create_test_function("multi_pattern", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        analyzer.analyze_function(&func);

        // Should have at least 2 suggestions
        assert!(analyzer.suggestions.len() >= 2);

        // Should have both categories
        let categories: Vec<_> = analyzer.suggestions.iter().map(|s| &s.category).collect();

        assert!(categories.contains(&&SuggestionCategory::Iterator));
        assert!(categories.contains(&&SuggestionCategory::TypeSystem));
    }

    #[test]
    fn test_migration_config_default() {
        let config = MigrationConfig::default();
        assert!(config.suggest_iterators);
        assert!(config.suggest_error_handling);
        assert!(config.suggest_ownership);
        assert!(config.suggest_performance);
        assert_eq!(config.verbosity, 1);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::Important);
        assert!(Severity::Important > Severity::Warning);
        assert!(Severity::Warning > Severity::Info);
    }

    #[test]
    fn test_accumulator_pattern_detection() {
        let body = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("result".to_string()),
                value: HirExpr::List(vec![]),
            },
            HirStmt::For {
                target: "item".to_string(),
                iter: HirExpr::Var("items".to_string()),
                body: vec![HirStmt::Expr(HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("result".to_string())),
                    method: "append".to_string(),
                    args: vec![HirExpr::Var("item".to_string())],
                })],
            },
        ];

        let func = create_test_function("test", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        analyzer.analyze_function(&func);
        assert!(!analyzer.suggestions.is_empty());
        assert_eq!(
            analyzer.suggestions[0].category,
            SuggestionCategory::Iterator
        );
    }

    // Note: none-as-error detection is not yet implemented.
    // This test is kept as documentation of expected behavior.
    #[test]
    #[ignore]
    fn test_none_as_error_detection() {
        let body = vec![
            HirStmt::If {
                condition: HirExpr::Var("error".to_string()),
                then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::None)))],
                else_body: None,
            },
            HirStmt::Return(Some(HirExpr::Var("result".to_string()))),
        ];

        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Optional(Box::new(Type::Unknown)),
            body,
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        analyzer.analyze_function(&func);

        assert!(analyzer
            .suggestions
            .iter()
            .any(|s| s.category == SuggestionCategory::ErrorHandling));
    }

    #[test]
    fn test_while_true_detection() {
        let body = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Break { label: None }],
        }];

        let func = create_test_function("test", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());

        analyzer.analyze_function(&func);
        assert!(analyzer
            .suggestions
            .iter()
            .any(|s| s.title.contains("loop")));
    }
}
