use anyhow::Result;
use colored::Colorize;
use depyler_annotations::{AnnotationParser, AnnotationValidator};
use depyler_core::DepylerPipeline;
use depyler_quality::QualityAnalyzer;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use rustpython_parser::{parse, Mode};
use std::fs;

#[cfg(test)]
#[path = "interactive_tests.rs"]
mod tests;

pub struct InteractiveSession {
    pipeline: DepylerPipeline,
    #[allow(dead_code)]
    annotation_parser: AnnotationParser,
    #[allow(dead_code)]
    annotation_validator: AnnotationValidator,
    quality_analyzer: QualityAnalyzer,
}

/// Represents a suggested annotation for improving transpilation
///
/// # Examples
///
/// ```rust
/// use depyler::interactive::{AnnotationSuggestion, SuggestionType, ImpactLevel};
///
/// let suggestion = AnnotationSuggestion {
///     line: 10,
///     function_name: "process_data".to_string(),
///     suggestion_type: SuggestionType::Performance,
///     annotation: "# @depyler: optimize = \"aggressive\"".to_string(),
///     reason: "Nested loops detected".to_string(),
///     impact: ImpactLevel::High,
/// };
///
/// assert_eq!(suggestion.line, 10);
/// assert_eq!(suggestion.impact, ImpactLevel::High);
/// ```
#[derive(Debug, Clone)]
pub struct AnnotationSuggestion {
    pub line: usize,
    pub function_name: String,
    pub suggestion_type: SuggestionType,
    pub annotation: String,
    pub reason: String,
    pub impact: ImpactLevel,
}

/// Types of annotation suggestions
///
/// # Examples
///
/// ```rust
/// use depyler::interactive::SuggestionType;
///
/// let perf = SuggestionType::Performance;
/// let safety = SuggestionType::Safety;
///
/// // Pattern matching on suggestion types
/// match perf {
///     SuggestionType::Performance => println!("Optimize for speed"),
///     SuggestionType::Memory => println!("Optimize for memory"),
///     _ => println!("Other optimization"),
/// }
/// ```
#[derive(Debug, Clone)]
pub enum SuggestionType {
    Performance,
    Safety,
    TypeStrategy,
    ErrorHandling,
    Concurrency,
    Memory,
}

/// Impact level of an annotation suggestion
///
/// Ordered from Low to High for easy sorting and filtering.
///
/// # Examples
///
/// ```rust
/// use depyler::interactive::ImpactLevel;
///
/// assert!(ImpactLevel::Low < ImpactLevel::Medium);
/// assert!(ImpactLevel::Medium < ImpactLevel::High);
///
/// let mut impacts = vec![ImpactLevel::Medium, ImpactLevel::High, ImpactLevel::Low];
/// impacts.sort();
/// assert_eq!(impacts, vec![ImpactLevel::Low, ImpactLevel::Medium, ImpactLevel::High]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
}

impl Default for InteractiveSession {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveSession {
    /// Create a new interactive transpilation session
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler::interactive::InteractiveSession;
    ///
    /// let session = InteractiveSession::new();
    /// // Session is ready for interactive transpilation
    /// ```
    pub fn new() -> Self {
        Self {
            pipeline: DepylerPipeline::new(),
            annotation_parser: AnnotationParser::new(),
            annotation_validator: AnnotationValidator::new(),
            quality_analyzer: QualityAnalyzer::new(),
        }
    }

    /// Run an interactive transpilation session
    ///
    /// Attempts to transpile the given Python file, providing interactive
    /// feedback and annotation suggestions when transpilation fails.
    ///
    /// # Arguments
    ///
    /// * `input_file` - Path to the Python source file
    /// * `annotate_mode` - Whether to suggest annotations for optimization
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use depyler::interactive::InteractiveSession;
    ///
    /// let mut session = InteractiveSession::new();
    /// // Run without annotation suggestions
    /// session.run("example.py", false).unwrap();
    ///
    /// // Run with annotation suggestions
    /// session.run("complex.py", true).unwrap();
    /// ```
    pub fn run(&mut self, input_file: &str, annotate_mode: bool) -> Result<()> {
        let python_source = fs::read_to_string(input_file)?;

        println!(
            "{}",
            "Depyler Interactive Transpilation Session"
                .bright_blue()
                .bold()
        );
        println!(
            "{}",
            "════════════════════════════════════════".bright_blue()
        );
        println!();

        // Initial transpilation attempt
        match self.attempt_transpilation(&python_source) {
            Ok((rust_code, warnings)) => {
                self.handle_successful_transpilation(&rust_code, warnings)?;

                if annotate_mode {
                    self.suggest_improvements(&python_source, &rust_code)?;
                }
            }
            Err(e) => {
                println!("{} Initial transpilation failed: {}", "❌".red(), e);
                println!();

                if annotate_mode {
                    self.run_annotation_workflow(&python_source, input_file)?;
                } else {
                    println!("💡 Tip: Run with --annotate to get annotation suggestions");
                }
            }
        }

        Ok(())
    }

    fn attempt_transpilation(&self, python_source: &str) -> Result<(String, Vec<String>)> {
        let mut warnings = Vec::new();

        // Parse and transpile
        let rust_code = self.pipeline.transpile(python_source)?;

        // Check for potential issues
        if rust_code.contains("unsafe") {
            warnings.push("Generated code contains unsafe blocks".to_string());
        }

        if rust_code.contains("panic!") {
            warnings.push("Generated code may panic".to_string());
        }

        Ok((rust_code, warnings))
    }

    fn handle_successful_transpilation(
        &self,
        rust_code: &str,
        warnings: Vec<String>,
    ) -> Result<()> {
        println!("{} Transpilation successful!", "✅".green());
        println!("Generated {} lines of Rust code", rust_code.lines().count());

        if !warnings.is_empty() {
            println!("\n{} Warnings:", "⚠️".yellow());
            for warning in warnings {
                println!("  • {warning}");
            }
        }

        // Ask if user wants to see the code
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to see the generated Rust code?")
            .default(false)
            .interact()?
        {
            println!("\n{}", "Generated Rust Code:".bright_green());
            println!("{}", "─".repeat(60));
            println!("{rust_code}");
            println!("{}", "─".repeat(60));
        }

        Ok(())
    }

    fn run_annotation_workflow(&mut self, python_source: &str, input_file: &str) -> Result<()> {
        println!(
            "{} Analyzing code for annotation opportunities...",
            "🔍".bright_cyan()
        );

        // Generate suggestions
        let suggestions = self.generate_annotation_suggestions(python_source)?;

        if suggestions.is_empty() {
            println!("No specific annotation suggestions found.");
            return Ok(());
        }

        println!(
            "\n{} Found {} annotation opportunities:",
            "📝".bright_yellow(),
            suggestions.len()
        );

        // Display suggestions
        for (i, suggestion) in suggestions.iter().enumerate() {
            self.display_suggestion(i + 1, suggestion);
        }

        // Interactive selection
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select annotations to apply (Space to select, Enter to confirm)")
            .items(
                &suggestions
                    .iter()
                    .map(|s| format!("{}: {}", s.function_name, s.annotation))
                    .collect::<Vec<_>>(),
            )
            .interact()?;

        if selections.is_empty() {
            println!("No annotations selected.");
            return Ok(());
        }

        // Apply selected annotations
        let mut modified_source = python_source.to_string();
        for &idx in selections.iter().rev() {
            modified_source = self.apply_annotation(&modified_source, &suggestions[idx])?;
        }

        // Show diff
        println!(
            "\n{} Modified source with annotations:",
            "📄".bright_green()
        );
        self.show_diff(python_source, &modified_source)?;

        // Confirm save
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Save modified file?")
            .default(true)
            .interact()?
        {
            let backup_file = format!("{input_file}.backup");
            fs::copy(input_file, &backup_file)?;
            fs::write(input_file, &modified_source)?;
            println!("✅ File saved. Backup created at: {backup_file}");

            // Retry transpilation
            println!("\n🔄 Retrying transpilation with annotations...");
            match self.attempt_transpilation(&modified_source) {
                Ok((rust_code, warnings)) => {
                    self.handle_successful_transpilation(&rust_code, warnings)?;
                }
                Err(e) => {
                    println!("❌ Transpilation still failed: {e}");
                    println!("💡 Additional manual modifications may be required.");
                }
            }
        }

        Ok(())
    }

    fn generate_annotation_suggestions(
        &self,
        python_source: &str,
    ) -> Result<Vec<AnnotationSuggestion>> {
        let mut suggestions = Vec::new();

        // Parse Python to analyze structure
        let ast = parse(python_source, Mode::Module, "<input>")?;
        let hir = depyler_core::ast_bridge::AstBridge::new()
            .with_source(python_source.to_string())
            .python_to_hir(ast)?;

        // Analyze each function
        for func in &hir.functions {
            // Analyze complexity for optimization suggestions
            let complexity = self.calculate_complexity(&func.body);

            // Performance suggestions based on patterns
            if self.has_nested_loops(&func.body) {
                suggestions.push(AnnotationSuggestion {
                    line: self.find_function_line(python_source, &func.name),
                    function_name: func.name.clone(),
                    suggestion_type: SuggestionType::Performance,
                    annotation: "# @depyler: optimization_level = \"aggressive\"".to_string(),
                    reason: "Nested loops detected - aggressive optimization recommended"
                        .to_string(),
                    impact: ImpactLevel::High,
                });

                suggestions.push(AnnotationSuggestion {
                    line: self.find_function_line(python_source, &func.name),
                    function_name: func.name.clone(),
                    suggestion_type: SuggestionType::Performance,
                    annotation: "# @depyler: optimization_hint = \"vectorize\"".to_string(),
                    reason: "Consider SIMD vectorization for better performance".to_string(),
                    impact: ImpactLevel::High,
                });
            } else if self.has_loops(&func.body) && self.has_simple_numeric_loop(&func.body) {
                suggestions.push(AnnotationSuggestion {
                    line: self.find_function_line(python_source, &func.name),
                    function_name: func.name.clone(),
                    suggestion_type: SuggestionType::Performance,
                    annotation: "# @depyler: optimization_hint = \"unroll_loops[4]\"".to_string(),
                    reason: "Simple numeric loop detected - unrolling can improve performance"
                        .to_string(),
                    impact: ImpactLevel::Medium,
                });
            }

            // Memory and ownership suggestions
            if self.has_large_collections(func) {
                if self.is_collection_modified(func) {
                    suggestions.push(AnnotationSuggestion {
                        line: self.find_function_line(python_source, &func.name),
                        function_name: func.name.clone(),
                        suggestion_type: SuggestionType::Memory,
                        annotation: "# @depyler: ownership = \"owned\"".to_string(),
                        reason: "Function modifies collections - owned values required".to_string(),
                        impact: ImpactLevel::Medium,
                    });
                } else {
                    suggestions.push(AnnotationSuggestion {
                        line: self.find_function_line(python_source, &func.name),
                        function_name: func.name.clone(),
                        suggestion_type: SuggestionType::Memory,
                        annotation: "# @depyler: ownership = \"borrowed\"".to_string(),
                        reason: "Function only reads collections - borrowing reduces memory usage"
                            .to_string(),
                        impact: ImpactLevel::Medium,
                    });
                }

                // Suggest appropriate collection type
                if self.has_frequent_lookups(func) {
                    suggestions.push(AnnotationSuggestion {
                        line: self.find_function_line(python_source, &func.name),
                        function_name: func.name.clone(),
                        suggestion_type: SuggestionType::Memory,
                        annotation: "# @depyler: hash_strategy = \"fnv\"".to_string(),
                        reason: "Frequent lookups detected - FNV hash is faster for small keys"
                            .to_string(),
                        impact: ImpactLevel::Medium,
                    });
                }
            }

            // String strategy suggestions
            if self.has_string_operations(func) {
                if self.has_string_concatenation(&func.body) {
                    suggestions.push(AnnotationSuggestion {
                        line: self.find_function_line(python_source, &func.name),
                        function_name: func.name.clone(),
                        suggestion_type: SuggestionType::TypeStrategy,
                        annotation: "# @depyler: string_strategy = \"always_owned\"".to_string(),
                        reason: "String concatenation detected - owned strings needed".to_string(),
                        impact: ImpactLevel::Low,
                    });
                } else if self.only_reads_strings(func) {
                    suggestions.push(AnnotationSuggestion {
                        line: self.find_function_line(python_source, &func.name),
                        function_name: func.name.clone(),
                        suggestion_type: SuggestionType::TypeStrategy,
                        annotation: "# @depyler: string_strategy = \"zero_copy\"".to_string(),
                        reason: "Function only reads strings - zero-copy improves performance"
                            .to_string(),
                        impact: ImpactLevel::Medium,
                    });
                }
            }

            // Safety and error handling suggestions
            if self.has_array_access(func) || self.has_dict_access(&func.body) {
                suggestions.push(AnnotationSuggestion {
                    line: self.find_function_line(python_source, &func.name),
                    function_name: func.name.clone(),
                    suggestion_type: SuggestionType::Safety,
                    annotation: "# @depyler: bounds_checking = \"explicit\"".to_string(),
                    reason: "Collection access detected - explicit bounds checking prevents panics"
                        .to_string(),
                    impact: ImpactLevel::High,
                });

                suggestions.push(AnnotationSuggestion {
                    line: self.find_function_line(python_source, &func.name),
                    function_name: func.name.clone(),
                    suggestion_type: SuggestionType::ErrorHandling,
                    annotation: "# @depyler: panic_behavior = \"convert_to_result\"".to_string(),
                    reason: "Potential panic points detected - convert to Result for safety"
                        .to_string(),
                    impact: ImpactLevel::High,
                });
            }

            // Concurrency suggestions
            if complexity > 50 && !self.has_shared_state(func) {
                suggestions.push(AnnotationSuggestion {
                    line: self.find_function_line(python_source, &func.name),
                    function_name: func.name.clone(),
                    suggestion_type: SuggestionType::Concurrency,
                    annotation: "# @depyler: thread_safety = \"send_sync\"".to_string(),
                    reason: "Complex pure function - can be safely parallelized".to_string(),
                    impact: ImpactLevel::Medium,
                });
            }

            // Performance critical marking
            if complexity > 30 || self.has_nested_loops(&func.body) {
                suggestions.push(AnnotationSuggestion {
                    line: self.find_function_line(python_source, &func.name),
                    function_name: func.name.clone(),
                    suggestion_type: SuggestionType::Performance,
                    annotation: "# @depyler: performance_critical = \"true\"".to_string(),
                    reason: "High complexity function - mark as performance critical".to_string(),
                    impact: ImpactLevel::High,
                });
            }

            // Error strategy based on return type
            if matches!(func.ret_type, depyler_core::hir::Type::Optional(_)) {
                suggestions.push(AnnotationSuggestion {
                    line: self.find_function_line(python_source, &func.name),
                    function_name: func.name.clone(),
                    suggestion_type: SuggestionType::ErrorHandling,
                    annotation: "# @depyler: error_strategy = \"result_type\"".to_string(),
                    reason:
                        "Optional return type - consider using Result for better error handling"
                            .to_string(),
                    impact: ImpactLevel::Low,
                });
            }
        }

        // Sort suggestions by impact and line number
        suggestions.sort_by(|a, b| b.impact.cmp(&a.impact).then_with(|| a.line.cmp(&b.line)));

        Ok(suggestions)
    }

    fn display_suggestion(&self, idx: usize, suggestion: &AnnotationSuggestion) {
        let impact_color = match suggestion.impact {
            ImpactLevel::High => "red",
            ImpactLevel::Medium => "yellow",
            ImpactLevel::Low => "green",
        };

        let type_icon = match suggestion.suggestion_type {
            SuggestionType::Performance => "⚡",
            SuggestionType::Safety => "🛡️",
            SuggestionType::TypeStrategy => "📊",
            SuggestionType::ErrorHandling => "❗",
            SuggestionType::Concurrency => "🔄",
            SuggestionType::Memory => "💾",
        };

        println!(
            "\n{}. {} {} - Function: {}",
            idx,
            type_icon,
            format!("{:?}", suggestion.suggestion_type).bright_cyan(),
            suggestion.function_name.bright_yellow()
        );
        println!("   Annotation: {}", suggestion.annotation.bright_green());
        println!("   Reason: {}", suggestion.reason);
        println!(
            "   Impact: {}",
            format!("{:?}", suggestion.impact).color(impact_color)
        );
    }

    fn apply_annotation(&self, source: &str, suggestion: &AnnotationSuggestion) -> Result<String> {
        let lines: Vec<&str> = source.lines().collect();
        let mut result = Vec::new();
        let mut applied = false;

        for line in lines {
            if !applied && line.contains(&format!("def {}", suggestion.function_name)) {
                // Insert annotation before function definition
                result.push(suggestion.annotation.as_str());
                applied = true;
            }
            result.push(line);
        }

        Ok(result.join("\n"))
    }

    fn show_diff(&self, original: &str, modified: &str) -> Result<()> {
        let original_lines: Vec<&str> = original.lines().collect();
        let modified_lines: Vec<&str> = modified.lines().collect();

        println!("\n{}", "Diff:".bright_yellow());
        println!("{}", "─".repeat(60));

        for (i, (orig, modif)) in original_lines.iter().zip(modified_lines.iter()).enumerate() {
            if orig != modif {
                println!("{} {}: {}", "-".red(), i + 1, orig);
                println!("{} {}: {}", "+".green(), i + 1, modif);
            }
        }

        // Handle added lines at the end
        if modified_lines.len() > original_lines.len() {
            for (i, line) in modified_lines[original_lines.len()..].iter().enumerate() {
                println!("{} {}: {}", "+".green(), original_lines.len() + i + 1, line);
            }
        }

        println!("{}", "─".repeat(60));

        Ok(())
    }

    fn suggest_improvements(&self, python_source: &str, rust_code: &str) -> Result<()> {
        println!(
            "\n{} Analyzing for potential improvements...",
            "🔍".bright_cyan()
        );

        // Quality analysis
        let ast = parse(python_source, Mode::Module, "<input>")?;
        let hir = depyler_core::ast_bridge::AstBridge::new()
            .with_source(python_source.to_string())
            .python_to_hir(ast)?;
        let quality_report = self.quality_analyzer.analyze_quality(&hir.functions)?;

        if quality_report.overall_status == depyler_quality::QualityStatus::Passed {
            println!("✅ Code quality meets all standards!");
        } else {
            println!(
                "\n{} Quality improvement suggestions:",
                "💡".bright_yellow()
            );

            for failed_gate in &quality_report.gates_failed {
                println!(
                    "  • {}: {}",
                    failed_gate.gate_name, failed_gate.actual_value
                );
            }
        }

        // Performance suggestions
        if rust_code.lines().count() > 100 {
            println!("\n{} Performance suggestions:", "⚡".bright_yellow());
            println!("  • Consider adding: # @depyler: optimization_level = \"aggressive\"");
            println!("  • For hot paths, add: # @depyler: performance_critical = \"true\"");
        }

        Ok(())
    }

    // Helper functions for analysis
    #[allow(clippy::only_used_in_recursion)]
    fn has_loops(&self, stmts: &[depyler_core::hir::HirStmt]) -> bool {
        use depyler_core::hir::HirStmt;
        stmts.iter().any(|stmt| match stmt {
            HirStmt::For { .. } | HirStmt::While { .. } => true,
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => self.has_loops(then_body) || else_body.as_ref().is_some_and(|e| self.has_loops(e)),
            _ => false,
        })
    }

    fn has_nested_loops(&self, stmts: &[depyler_core::hir::HirStmt]) -> bool {
        use depyler_core::hir::HirStmt;
        stmts.iter().any(|stmt| match stmt {
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => self.has_loops(body),
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                self.has_nested_loops(then_body)
                    || else_body.as_ref().is_some_and(|e| self.has_nested_loops(e))
            }
            _ => false,
        })
    }

    fn has_simple_numeric_loop(&self, stmts: &[depyler_core::hir::HirStmt]) -> bool {
        use depyler_core::hir::{HirExpr, HirStmt};
        stmts.iter().any(|stmt| match stmt {
            HirStmt::For { iter, body, .. } => {
                // Check if iterating over range
                matches!(iter, HirExpr::Call { func, .. } if func == "range") &&
                // Check if body is simple (no nested loops)
                !self.has_loops(body)
            }
            _ => false,
        })
    }

    fn has_large_collections(&self, func: &depyler_core::hir::HirFunction) -> bool {
        use depyler_core::hir::Type;
        func.params
            .iter()
            .any(|param| matches!(param.ty, Type::List(_) | Type::Dict(_, _)))
    }

    fn is_collection_modified(&self, func: &depyler_core::hir::HirFunction) -> bool {
        // Check if function modifies collections (simplified - checks for common patterns)
        self.has_modification_patterns(&func.body)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn has_modification_patterns(&self, stmts: &[depyler_core::hir::HirStmt]) -> bool {
        use depyler_core::hir::{HirExpr, HirStmt};
        stmts.iter().any(|stmt| match stmt {
            HirStmt::Expr(HirExpr::Call { func, .. }) => {
                matches!(
                    func.as_str(),
                    "append" | "extend" | "insert" | "remove" | "pop" | "clear"
                )
            }
            HirStmt::Expr(_) => false,
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                self.has_modification_patterns(then_body)
                    || else_body
                        .as_ref()
                        .is_some_and(|e| self.has_modification_patterns(e))
            }
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
                self.has_modification_patterns(body)
            }
            _ => false,
        })
    }

    fn has_frequent_lookups(&self, func: &depyler_core::hir::HirFunction) -> bool {
        // Check for dict/set lookups in loops
        self.has_lookup_in_loop(&func.body)
    }

    fn has_lookup_in_loop(&self, stmts: &[depyler_core::hir::HirStmt]) -> bool {
        use depyler_core::hir::HirStmt;
        stmts.iter().any(|stmt| match stmt {
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => self.has_dict_access(body),
            _ => false,
        })
    }

    fn has_string_operations(&self, func: &depyler_core::hir::HirFunction) -> bool {
        use depyler_core::hir::Type;
        func.params.iter().any(|param| matches!(param.ty, Type::String))
            || matches!(func.ret_type, Type::String)
    }

    fn has_string_concatenation(&self, stmts: &[depyler_core::hir::HirStmt]) -> bool {
        use depyler_core::hir::HirStmt;
        stmts.iter().any(|stmt| match stmt {
            HirStmt::Assign { value, .. } => self.has_string_concat_expr(value),
            HirStmt::Return(Some(expr)) => self.has_string_concat_expr(expr),
            _ => false,
        })
    }

    fn has_string_concat_expr(&self, expr: &depyler_core::hir::HirExpr) -> bool {
        use depyler_core::hir::BinOp;
        match expr {
            depyler_core::hir::HirExpr::Binary {
                op: BinOp::Add,
                left,
                right,
            } => {
                // Check if either operand is a string
                self.is_string_expr(left) || self.is_string_expr(right)
            }
            _ => false,
        }
    }

    fn is_string_expr(&self, expr: &depyler_core::hir::HirExpr) -> bool {
        use depyler_core::hir::{HirExpr, Literal};
        matches!(expr, HirExpr::Literal(Literal::String(_)) | HirExpr::Var(_))
    }

    fn only_reads_strings(&self, func: &depyler_core::hir::HirFunction) -> bool {
        !self.has_string_concatenation(&func.body) && !self.has_modification_patterns(&func.body)
    }

    fn has_array_access(&self, func: &depyler_core::hir::HirFunction) -> bool {
        self.has_index_access(&func.body)
    }

    fn has_index_access(&self, stmts: &[depyler_core::hir::HirStmt]) -> bool {
        use depyler_core::hir::HirStmt;
        stmts.iter().any(|stmt| match stmt {
            HirStmt::Assign { value, .. } => self.has_index_expr(value),
            HirStmt::Return(Some(expr)) => self.has_index_expr(expr),
            HirStmt::Expr(expr) => self.has_index_expr(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.has_index_expr(condition)
                    || self.has_index_access(then_body)
                    || else_body.as_ref().is_some_and(|e| self.has_index_access(e))
            }
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => self.has_index_access(body),
            _ => false,
        })
    }

    fn has_index_expr(&self, expr: &depyler_core::hir::HirExpr) -> bool {
        use depyler_core::hir::HirExpr;
        matches!(expr, HirExpr::Index { .. })
    }

    fn has_dict_access(&self, stmts: &[depyler_core::hir::HirStmt]) -> bool {
        self.has_index_access(stmts) // Simplified - dict access uses same index syntax
    }

    fn has_shared_state(&self, _func: &depyler_core::hir::HirFunction) -> bool {
        // Check if function accesses global state or has side effects
        // Simplified check - would need more sophisticated analysis
        false
    }

    #[allow(clippy::only_used_in_recursion)]
    fn calculate_complexity(&self, stmts: &[depyler_core::hir::HirStmt]) -> u32 {
        use depyler_core::hir::HirStmt;
        stmts
            .iter()
            .map(|stmt| match stmt {
                HirStmt::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    1 + self.calculate_complexity(then_body)
                        + else_body
                            .as_ref()
                            .map_or(0, |e| self.calculate_complexity(e))
                }
                HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
                    3 + self.calculate_complexity(body)
                }
                // Match statements not yet supported in HIR
                _ => 1,
            })
            .sum()
    }

    fn find_function_line(&self, source: &str, func_name: &str) -> usize {
        source
            .lines()
            .enumerate()
            .find(|(_, line)| line.contains(&format!("def {func_name}")))
            .map(|(i, _)| i + 1)
            .unwrap_or(0)
    }
}

/// Run an interactive transpilation session with a new session instance
///
/// This is a convenience function that creates a new `InteractiveSession`
/// and runs it with the specified parameters.
///
/// # Arguments
///
/// * `input_file` - Path to the Python source file to transpile
/// * `annotate_mode` - Whether to provide annotation suggestions
///
/// # Examples
///
/// ```rust,no_run
/// use depyler::interactive::run_interactive_session;
///
/// // Run interactive session on a Python file
/// run_interactive_session("script.py", true).unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The input file cannot be read
/// - The Python code cannot be parsed
/// - User interaction fails (e.g., no terminal available)
pub fn run_interactive_session(input_file: &str, annotate_mode: bool) -> Result<()> {
    let mut session = InteractiveSession::new();
    session.run(input_file, annotate_mode)
}
