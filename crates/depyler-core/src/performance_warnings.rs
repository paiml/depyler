/// Performance warning system for identifying inefficient patterns
use crate::hir::{BinOp, HirExpr, HirFunction, HirProgram, HirStmt, Type};
use colored::Colorize;

/// Performance analyzer that identifies potentially inefficient patterns
pub struct PerformanceAnalyzer {
    /// Collected warnings
    warnings: Vec<PerformanceWarning>,
    /// Configuration
    config: PerformanceConfig,
    /// Loop depth tracking
    current_loop_depth: usize,
}

#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Warn about string concatenation in loops
    pub warn_string_concat: bool,
    /// Warn about unnecessary allocations
    pub warn_allocations: bool,
    /// Warn about inefficient algorithms
    pub warn_algorithms: bool,
    /// Warn about repeated computations
    pub warn_repeated_computation: bool,
    /// Maximum loop depth before warning
    pub max_loop_depth: usize,
    /// Minimum list size to warn about O(n²) operations
    pub quadratic_threshold: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            warn_string_concat: true,
            warn_allocations: true,
            warn_algorithms: true,
            warn_repeated_computation: true,
            max_loop_depth: 3,
            quadratic_threshold: 100,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceWarning {
    /// Type of performance issue
    pub category: WarningCategory,
    /// Severity of the issue
    pub severity: WarningSeverity,
    /// Brief description
    pub message: String,
    /// Detailed explanation
    pub explanation: String,
    /// Suggested fix
    pub suggestion: String,
    /// Estimated impact
    pub impact: PerformanceImpact,
    /// Source location
    pub location: Option<Location>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarningCategory {
    /// String operations
    StringPerformance,
    /// Memory allocations
    MemoryAllocation,
    /// Algorithm complexity
    AlgorithmComplexity,
    /// Repeated computation
    RedundantComputation,
    /// I/O operations
    IoPerformance,
    /// Collection usage
    CollectionUsage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WarningSeverity {
    /// Minor performance impact
    Low,
    /// Noticeable performance impact
    Medium,
    /// Significant performance impact
    High,
    /// Severe performance impact
    Critical,
}

#[derive(Debug, Clone)]
pub struct PerformanceImpact {
    /// Complexity class (O(n), O(n²), etc.)
    pub complexity: String,
    /// Whether impact scales with input size
    pub scales_with_input: bool,
    /// Whether it's in a hot path (loop)
    pub in_hot_path: bool,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub function: String,
    pub line: usize,
    pub in_loop: bool,
    pub loop_depth: usize,
}

impl PerformanceAnalyzer {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            warnings: Vec::new(),
            config,
            current_loop_depth: 0,
        }
    }

    /// Analyze a program for performance issues
    pub fn analyze_program(&mut self, program: &HirProgram) -> Vec<PerformanceWarning> {
        self.warnings.clear();

        for func in &program.functions {
            self.analyze_function(func);
        }

        // Sort by severity
        self.warnings.sort_by(|a, b| b.severity.cmp(&a.severity));

        self.warnings.clone()
    }

    fn analyze_function(&mut self, func: &HirFunction) {
        self.current_loop_depth = 0;

        // Check for common performance antipatterns
        self.check_function_level_issues(func);

        // Analyze function body
        for (idx, stmt) in func.body.iter().enumerate() {
            self.analyze_stmt(stmt, func, idx);
        }
    }

    fn check_function_level_issues(&mut self, func: &HirFunction) {
        // Check for large parameter passing
        for param in &func.params {
            if self.is_large_type(&param.ty) && !self.is_reference_type(&param.ty) {
                self.add_warning(PerformanceWarning {
                    category: WarningCategory::MemoryAllocation,
                    severity: WarningSeverity::Medium,
                    message: format!("Large value '{}' passed by copy", param.name),
                    explanation: "Passing large values by copy is inefficient".to_string(),
                    suggestion:
                        "Consider passing by reference (&) or using Box/Arc for large types"
                            .to_string(),
                    impact: PerformanceImpact {
                        complexity: "O(n)".to_string(),
                        scales_with_input: true,
                        in_hot_path: false,
                    },
                    location: Some(Location {
                        function: func.name.clone(),
                        line: 0,
                        in_loop: false,
                        loop_depth: 0,
                    }),
                });
            }
        }
    }

    fn analyze_stmt(&mut self, stmt: &HirStmt, func: &HirFunction, line: usize) {
        match stmt {
            HirStmt::For {
                target: _,
                iter,
                body,
            } => {
                self.analyze_for_loop(iter, body, func, line);
            }
            HirStmt::While { condition: _, body } => {
                self.analyze_while_loop(body, func, line);
            }
            HirStmt::Assign {
                target: _, value, ..
            } => {
                self.analyze_assignment(value, func, line);
            }
            HirStmt::Expr(expr) => {
                self.analyze_expr(expr, func, line);
            }
            _ => {}
        }
    }

    fn analyze_for_loop(
        &mut self,
        iter: &HirExpr,
        body: &[HirStmt],
        func: &HirFunction,
        line: usize,
    ) {
        self.current_loop_depth += 1;

        self.check_loop_depth_violation(func, line);
        self.check_iteration_pattern(iter, func, line);

        for inner_stmt in body {
            self.analyze_stmt(inner_stmt, func, line);
        }

        self.current_loop_depth -= 1;
    }

    fn analyze_while_loop(&mut self, body: &[HirStmt], func: &HirFunction, line: usize) {
        self.current_loop_depth += 1;

        for inner_stmt in body {
            self.analyze_stmt(inner_stmt, func, line);
        }

        self.current_loop_depth -= 1;
    }

    fn analyze_assignment(&mut self, value: &HirExpr, func: &HirFunction, line: usize) {
        self.analyze_expr(value, func, line);

        if self.current_loop_depth > 0 && self.is_string_concatenation(value) {
            self.warn_string_concat_in_loop(func, line);
        }
    }

    fn check_loop_depth_violation(&mut self, func: &HirFunction, line: usize) {
        if self.current_loop_depth > self.config.max_loop_depth {
            self.add_warning(PerformanceWarning {
                category: WarningCategory::AlgorithmComplexity,
                severity: WarningSeverity::High,
                message: format!("Deeply nested loops (depth: {})", self.current_loop_depth),
                explanation: "Deeply nested loops can lead to exponential time complexity"
                    .to_string(),
                suggestion:
                    "Consider refactoring to reduce nesting or use more efficient algorithms"
                        .to_string(),
                impact: PerformanceImpact {
                    complexity: format!("O(n^{})", self.current_loop_depth),
                    scales_with_input: true,
                    in_hot_path: true,
                },
                location: Some(Location {
                    function: func.name.clone(),
                    line,
                    in_loop: true,
                    loop_depth: self.current_loop_depth,
                }),
            });
        }
    }

    fn warn_string_concat_in_loop(&mut self, func: &HirFunction, line: usize) {
        self.add_warning(PerformanceWarning {
            category: WarningCategory::StringPerformance,
            severity: WarningSeverity::High,
            message: "String concatenation in loop".to_string(),
            explanation: "String concatenation in loops creates many intermediate strings"
                .to_string(),
            suggestion: "Use String::with_capacity() and push_str(), or collect into a String"
                .to_string(),
            impact: PerformanceImpact {
                complexity: "O(n²)".to_string(),
                scales_with_input: true,
                in_hot_path: true,
            },
            location: Some(Location {
                function: func.name.clone(),
                line,
                in_loop: true,
                loop_depth: self.current_loop_depth,
            }),
        });
    }

    fn analyze_expr(&mut self, expr: &HirExpr, func: &HirFunction, line: usize) {
        match expr {
            HirExpr::Binary { left, right, op } => {
                self.analyze_binary_expr(left, right, op, func, line);
            }
            HirExpr::Call { func: fname, args } => {
                self.analyze_function_call(fname, args, func, line);
            }
            HirExpr::MethodCall {
                object,
                method,
                args,
            } => {
                self.analyze_method_call(object, method, args, func, line);
            }
            HirExpr::List(items) => {
                self.analyze_list_expr(items, func, line);
            }
            _ => {}
        }
    }

    fn analyze_binary_expr(
        &mut self,
        left: &HirExpr,
        right: &HirExpr,
        op: &BinOp,
        func: &HirFunction,
        line: usize,
    ) {
        self.analyze_expr(left, func, line);
        self.analyze_expr(right, func, line);

        if matches!(op, BinOp::Pow) && self.current_loop_depth > 0 {
            self.warn_power_in_loop(func, line);
        }
    }

    fn analyze_function_call(
        &mut self,
        fname: &str,
        args: &[HirExpr],
        func: &HirFunction,
        line: usize,
    ) {
        if self.current_loop_depth > 0 && self.is_expensive_function(fname) {
            self.warn_expensive_function_in_loop(fname, func, line);
        }

        self.check_function_call_patterns(fname, args, func, line);

        for arg in args {
            self.analyze_expr(arg, func, line);
        }
    }

    fn analyze_method_call(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
        func: &HirFunction,
        line: usize,
    ) {
        self.analyze_expr(object, func, line);
        self.check_method_patterns(object, method, args, func, line);

        for arg in args {
            self.analyze_expr(arg, func, line);
        }
    }

    fn analyze_list_expr(&mut self, items: &[HirExpr], func: &HirFunction, line: usize) {
        if self.current_loop_depth > 0 && items.len() > 10 {
            self.warn_large_list_in_loop(func, line);
        }

        for item in items {
            self.analyze_expr(item, func, line);
        }
    }

    fn warn_power_in_loop(&mut self, func: &HirFunction, line: usize) {
        self.add_warning(PerformanceWarning {
            category: WarningCategory::RedundantComputation,
            severity: WarningSeverity::Medium,
            message: "Power operation in loop".to_string(),
            explanation: "Power operations are computationally expensive".to_string(),
            suggestion: "Consider caching the result if the value doesn't change".to_string(),
            impact: PerformanceImpact {
                complexity: "O(log n) per operation".to_string(),
                scales_with_input: false,
                in_hot_path: true,
            },
            location: Some(Location {
                function: func.name.clone(),
                line,
                in_loop: true,
                loop_depth: self.current_loop_depth,
            }),
        });
    }

    fn warn_expensive_function_in_loop(&mut self, fname: &str, func: &HirFunction, line: usize) {
        self.add_warning(PerformanceWarning {
            category: WarningCategory::RedundantComputation,
            severity: WarningSeverity::Medium,
            message: format!("Expensive function '{}' called in loop", fname),
            explanation: "Calling expensive functions repeatedly can impact performance"
                .to_string(),
            suggestion: "Cache the result if the inputs don't change".to_string(),
            impact: PerformanceImpact {
                complexity: "Depends on function".to_string(),
                scales_with_input: true,
                in_hot_path: true,
            },
            location: Some(Location {
                function: func.name.clone(),
                line,
                in_loop: true,
                loop_depth: self.current_loop_depth,
            }),
        });
    }

    fn warn_large_list_in_loop(&mut self, func: &HirFunction, line: usize) {
        self.add_warning(PerformanceWarning {
            category: WarningCategory::MemoryAllocation,
            severity: WarningSeverity::Medium,
            message: "Large list created in loop".to_string(),
            explanation: "Creating large collections in loops causes repeated allocations"
                .to_string(),
            suggestion: "Move the list creation outside the loop or use a pre-allocated buffer"
                .to_string(),
            impact: PerformanceImpact {
                complexity: "O(n) allocations".to_string(),
                scales_with_input: true,
                in_hot_path: true,
            },
            location: Some(Location {
                function: func.name.clone(),
                line,
                in_loop: true,
                loop_depth: self.current_loop_depth,
            }),
        });
    }

    fn check_iteration_pattern(&mut self, iter: &HirExpr, func: &HirFunction, line: usize) {
        // Check for range(len(x)) antipattern
        if let HirExpr::Call { func: fname, args } = iter {
            if fname == "range" && !args.is_empty() {
                if let HirExpr::Call {
                    func: inner_func, ..
                } = &args[0]
                {
                    if inner_func == "len" {
                        self.add_warning(PerformanceWarning {
                            category: WarningCategory::CollectionUsage,
                            severity: WarningSeverity::Low,
                            message: "Using range(len(x)) instead of enumerate".to_string(),
                            explanation: "This pattern is less efficient and less idiomatic"
                                .to_string(),
                            suggestion: "Use enumerate() to get both index and value".to_string(),
                            impact: PerformanceImpact {
                                complexity: "O(1) overhead".to_string(),
                                scales_with_input: false,
                                in_hot_path: true,
                            },
                            location: Some(Location {
                                function: func.name.clone(),
                                line,
                                in_loop: false,
                                loop_depth: self.current_loop_depth,
                            }),
                        });
                    }
                }
            }
        }
    }

    fn check_function_call_patterns(
        &mut self,
        fname: &str,
        args: &[HirExpr],
        func: &HirFunction,
        line: usize,
    ) {
        // Check for repeated sorting
        if fname == "sorted" && self.current_loop_depth > 0 {
            self.add_warning(PerformanceWarning {
                category: WarningCategory::AlgorithmComplexity,
                severity: WarningSeverity::High,
                message: "Sorting inside a loop".to_string(),
                explanation: "Sorting has O(n log n) complexity and shouldn't be repeated"
                    .to_string(),
                suggestion: "Sort once before the loop or maintain sorted order".to_string(),
                impact: PerformanceImpact {
                    complexity: "O(n² log n)".to_string(),
                    scales_with_input: true,
                    in_hot_path: true,
                },
                location: Some(Location {
                    function: func.name.clone(),
                    line,
                    in_loop: true,
                    loop_depth: self.current_loop_depth,
                }),
            });
        }

        // Check for sum/min/max on same collection multiple times
        if ["sum", "min", "max"].contains(&fname) && !args.is_empty() {
            // This would need more context to detect repeated calls on same data
            // For now, just warn if in nested loops
            if self.current_loop_depth > 1 {
                self.add_warning(PerformanceWarning {
                    category: WarningCategory::RedundantComputation,
                    severity: WarningSeverity::Medium,
                    message: format!("Aggregate function '{}' in nested loop", fname),
                    explanation: "Computing aggregates repeatedly is inefficient".to_string(),
                    suggestion: "Compute once and cache the result".to_string(),
                    impact: PerformanceImpact {
                        complexity: "O(n) per call".to_string(),
                        scales_with_input: true,
                        in_hot_path: true,
                    },
                    location: Some(Location {
                        function: func.name.clone(),
                        line,
                        in_loop: true,
                        loop_depth: self.current_loop_depth,
                    }),
                });
            }
        }
    }

    fn check_method_patterns(
        &mut self,
        _object: &HirExpr,
        method: &str,
        _args: &[HirExpr],
        func: &HirFunction,
        line: usize,
    ) {
        if self.current_loop_depth == 0 {
            return;
        }

        match method {
            "append" => self.warn_append_in_loop(func, line),
            "remove" if self.current_loop_depth > 1 => self.warn_remove_in_nested_loop(func, line),
            "index" | "count" => self.warn_linear_search_in_loop(method, func, line),
            _ => {}
        }
    }

    fn warn_append_in_loop(&mut self, func: &HirFunction, line: usize) {
        self.add_warning(PerformanceWarning {
            category: WarningCategory::CollectionUsage,
            severity: WarningSeverity::Low,
            message: "Multiple append calls in loop".to_string(),
            explanation: "Multiple append operations can be less efficient than extend".to_string(),
            suggestion: "Consider collecting items and using extend() once".to_string(),
            impact: PerformanceImpact {
                complexity: "O(1) amortized, but more calls".to_string(),
                scales_with_input: true,
                in_hot_path: true,
            },
            location: Some(Location {
                function: func.name.clone(),
                line,
                in_loop: true,
                loop_depth: self.current_loop_depth,
            }),
        });
    }

    fn warn_remove_in_nested_loop(&mut self, func: &HirFunction, line: usize) {
        self.add_warning(PerformanceWarning {
            category: WarningCategory::AlgorithmComplexity,
            severity: WarningSeverity::Critical,
            message: "List remove() in nested loop".to_string(),
            explanation: "remove() is O(n) and in nested loops becomes O(n²) or worse".to_string(),
            suggestion: "Use a set for O(1) removal or filter to create a new list".to_string(),
            impact: PerformanceImpact {
                complexity: format!("O(n^{})", self.current_loop_depth + 1),
                scales_with_input: true,
                in_hot_path: true,
            },
            location: Some(Location {
                function: func.name.clone(),
                line,
                in_loop: true,
                loop_depth: self.current_loop_depth,
            }),
        });
    }

    fn warn_linear_search_in_loop(&mut self, method: &str, func: &HirFunction, line: usize) {
        self.add_warning(PerformanceWarning {
            category: WarningCategory::AlgorithmComplexity,
            severity: WarningSeverity::Medium,
            message: format!("Linear search method '{}' in loop", method),
            explanation: "Linear search in loops can lead to quadratic complexity".to_string(),
            suggestion: "Consider using a HashMap/HashSet for O(1) lookups".to_string(),
            impact: PerformanceImpact {
                complexity: "O(n²)".to_string(),
                scales_with_input: true,
                in_hot_path: true,
            },
            location: Some(Location {
                function: func.name.clone(),
                line,
                in_loop: true,
                loop_depth: self.current_loop_depth,
            }),
        });
    }

    // Helper methods

    fn is_large_type(&self, ty: &Type) -> bool {
        match ty {
            Type::List(_) | Type::Dict(_, _) | Type::String => true,
            Type::Custom(name) => {
                // Assume custom types might be large
                !["i32", "i64", "f32", "f64", "bool", "char"].contains(&name.as_str())
            }
            _ => false,
        }
    }

    fn is_reference_type(&self, _ty: &Type) -> bool {
        // In the current HIR, we don't track references explicitly
        // This would need enhancement to properly detect &T types
        false
    }

    fn is_string_concatenation(&self, expr: &HirExpr) -> bool {
        if let HirExpr::Binary {
            op: BinOp::Add,
            left,
            right,
        } = expr
        {
            // Check if either operand might be a string
            matches!(left.as_ref(), HirExpr::Var(_)) || matches!(right.as_ref(), HirExpr::Var(_))
        } else {
            false
        }
    }

    fn is_expensive_function(&self, fname: &str) -> bool {
        // List of known expensive functions
        let expensive = [
            "sorted", "sort", "reverse", "compile", "eval", "exec", "deepcopy", "copy", "hash",
            "checksum",
        ];
        expensive.contains(&fname)
    }

    fn add_warning(&mut self, warning: PerformanceWarning) {
        self.warnings.push(warning);
    }

    /// Format warnings for display
    pub fn format_warnings(&self, warnings: &[PerformanceWarning]) -> String {
        if warnings.is_empty() {
            return "✅ No performance warnings found!\n".green().to_string();
        }

        let mut output = String::new();
        self.append_header(&mut output);
        self.append_warning_details(&mut output, warnings);
        self.append_summary(&mut output, warnings);
        output
    }

    fn append_header(&self, output: &mut String) {
        output.push_str(&format!("\n{}\n", "Performance Warnings".bold().yellow()));
        output.push_str(&format!("{}\n\n", "═".repeat(50)));
    }

    fn append_warning_details(&self, output: &mut String, warnings: &[PerformanceWarning]) {
        for (idx, warning) in warnings.iter().enumerate() {
            self.append_single_warning(output, idx, warning);
        }
    }

    fn append_single_warning(&self, output: &mut String, idx: usize, warning: &PerformanceWarning) {
        self.append_warning_header(output, idx, warning);
        self.append_warning_location(output, warning);
        self.append_warning_impact(output, warning);
        self.append_warning_explanation(output, warning);
        self.append_warning_suggestion(output, warning);
        output.push('\n');
    }

    fn append_warning_header(&self, output: &mut String, idx: usize, warning: &PerformanceWarning) {
        let severity_color = self.get_severity_color(warning.severity);
        output.push_str(&format!(
            "{} {} {}\n",
            format!("[{}]", idx + 1).dimmed(),
            format!("[{:?}]", warning.severity)
                .color(severity_color)
                .bold(),
            warning.message.bold()
        ));
    }

    fn append_warning_location(&self, output: &mut String, warning: &PerformanceWarning) {
        if let Some(loc) = &warning.location {
            let loop_info = self.format_loop_info(loc);
            output.push_str(&format!(
                "   {} {}, line {}{}\n",
                "Location:".dimmed(),
                loc.function,
                loc.line,
                loop_info
            ));
        }
    }

    fn append_warning_impact(&self, output: &mut String, warning: &PerformanceWarning) {
        output.push_str(&format!(
            "   {} Complexity: {}, Scales: {}, Hot path: {}\n",
            "Impact:".dimmed(),
            warning.impact.complexity.yellow(),
            self.format_yes_no(warning.impact.scales_with_input),
            self.format_yes_no(warning.impact.in_hot_path)
        ));
    }

    fn append_warning_explanation(&self, output: &mut String, warning: &PerformanceWarning) {
        output.push_str(&format!("   {} {}\n", "Why:".dimmed(), warning.explanation));
    }

    fn append_warning_suggestion(&self, output: &mut String, warning: &PerformanceWarning) {
        output.push_str(&format!(
            "   {} {}\n",
            "Fix:".green(),
            warning.suggestion.green()
        ));
    }

    fn append_summary(&self, output: &mut String, warnings: &[PerformanceWarning]) {
        let (critical, high) = self.count_severity_levels(warnings);

        output.push_str(&format!(
            "{} Found {} warnings ({} critical, {} high severity)\n",
            "Summary:".bold(),
            warnings.len(),
            critical,
            high
        ));

        if critical > 0 || high > 0 {
            self.append_critical_warning_notice(output);
        }
    }

    fn get_severity_color(&self, severity: WarningSeverity) -> &'static str {
        match severity {
            WarningSeverity::Critical => "red",
            WarningSeverity::High => "bright red",
            WarningSeverity::Medium => "yellow",
            WarningSeverity::Low => "bright yellow",
        }
    }

    fn format_loop_info(&self, loc: &Location) -> String {
        if loc.in_loop {
            format!(" (in loop, depth: {})", loc.loop_depth)
                .red()
                .to_string()
        } else {
            String::new()
        }
    }

    fn format_yes_no(&self, value: bool) -> colored::ColoredString {
        if value {
            "Yes".red()
        } else {
            "No".green()
        }
    }

    fn count_severity_levels(&self, warnings: &[PerformanceWarning]) -> (usize, usize) {
        let critical = warnings
            .iter()
            .filter(|w| w.severity == WarningSeverity::Critical)
            .count();
        let high = warnings
            .iter()
            .filter(|w| w.severity == WarningSeverity::High)
            .count();
        (critical, high)
    }

    fn append_critical_warning_notice(&self, output: &mut String) {
        output.push_str(
            &"⚠️  Address critical and high severity warnings for better performance\n"
                .red()
                .to_string(),
        );
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

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();
        assert!(config.warn_string_concat);
        assert!(config.warn_allocations);
        assert_eq!(config.max_loop_depth, 3);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(WarningSeverity::Critical > WarningSeverity::High);
        assert!(WarningSeverity::High > WarningSeverity::Medium);
        assert!(WarningSeverity::Medium > WarningSeverity::Low);
    }

    #[test]
    fn test_string_concat_in_loop_detection() {
        let body = vec![HirStmt::For {
            target: "i".to_string(),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
            },
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("s".to_string()),
                value: HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("s".to_string())),
                    right: Box::new(HirExpr::Var("i".to_string())),
                },
                type_annotation: None,
            }],
        }];

        let func = create_test_function("test", body);
        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let mut analyzer = PerformanceAnalyzer::new(PerformanceConfig::default());
        let warnings = analyzer.analyze_program(&program);

        assert!(!warnings.is_empty());
        assert!(warnings
            .iter()
            .any(|w| w.category == WarningCategory::StringPerformance));
    }

    #[test]
    fn test_nested_loop_detection() {
        let inner_loop = HirStmt::For {
            target: "j".to_string(),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Var("j".to_string()))],
        };

        let body = vec![HirStmt::For {
            target: "i".to_string(),
            iter: HirExpr::Var("items".to_string()),
            body: vec![inner_loop],
        }];

        let func = create_test_function("test", body);
        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let mut analyzer = PerformanceAnalyzer::new(PerformanceConfig {
            max_loop_depth: 2,
            ..Default::default()
        });
        let warnings = analyzer.analyze_program(&program);

        assert!(warnings.is_empty()); // Depth 2 is within limit
    }

    #[test]
    fn test_expensive_function_in_loop() {
        let body = vec![HirStmt::For {
            target: "item".to_string(),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("s".to_string()),
                value: HirExpr::Call {
                    func: "sorted".to_string(),
                    args: vec![HirExpr::Var("data".to_string())],
                },
                type_annotation: None,
            }],
        }];

        let func = create_test_function("test", body);
        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let mut analyzer = PerformanceAnalyzer::new(PerformanceConfig::default());
        let warnings = analyzer.analyze_program(&program);

        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| {
            w.category == WarningCategory::AlgorithmComplexity
                || w.category == WarningCategory::RedundantComputation
        }));
    }
}
