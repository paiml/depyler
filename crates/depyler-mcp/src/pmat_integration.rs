use pmcp::Error as McpError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PmatQualityReport {
    pub score: f64,
    pub passes: bool,
    pub violations: Vec<QualityViolation>,
    pub suggestions: Vec<String>,
    pub metrics: QualityMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityViolation {
    pub severity: ViolationSeverity,
    pub rule: String,
    pub message: String,
    pub location: Option<CodeLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViolationSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLocation {
    pub file: String,
    pub line: usize,
    pub column: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub cyclomatic_complexity: f64,
    pub cognitive_complexity: f64,
    pub test_coverage: f64,
    pub documentation_coverage: f64,
    pub type_safety_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: u8,
    pub tags: Vec<String>,
    pub dependencies: Vec<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Blocked,
}

pub struct PmatIntegration {
    tasks: Vec<TodoTask>,
}

impl PmatIntegration {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub async fn check_transpiled_code(
        &self,
        rust_code: &str,
        _project_path: Option<&Path>,
    ) -> Result<PmatQualityReport, McpError> {
        info!("Running pmat quality check on transpiled Rust code");

        // Use pmat's rust-ast feature to analyze the code
        let _syntax_valid = if let Ok(file) = syn::parse_str::<syn::File>(rust_code) {
            // Analyze the parsed AST using pmat patterns
            let metrics = self.analyze_rust_ast(&file);
            let violations = self.check_violations(rust_code, &metrics);
            let score = self.calculate_score(&metrics, &violations, true);
            let suggestions = self.generate_suggestions(&metrics, &violations);

            let report = PmatQualityReport {
                score,
                passes: score >= 70.0,
                violations,
                suggestions,
                metrics,
            };

            debug!(
                "Quality check complete. Score: {}, Passes: {}",
                report.score, report.passes
            );
            return Ok(report);
        } else {
            false
        };

        // Fallback for invalid syntax
        let metrics = QualityMetrics {
            cyclomatic_complexity: 999.0,
            cognitive_complexity: 999.0,
            test_coverage: 0.0,
            documentation_coverage: 0.0,
            type_safety_score: 0.0,
        };

        let violations = vec![QualityViolation {
            severity: ViolationSeverity::Error,
            rule: "invalid_syntax".to_string(),
            message: "Failed to parse Rust code".to_string(),
            location: None,
        }];

        Ok(PmatQualityReport {
            score: 0.0,
            passes: false,
            violations,
            suggestions: vec!["Fix syntax errors in the generated Rust code".to_string()],
            metrics,
        })
    }

    pub async fn create_transpilation_tasks(
        &mut self,
        python_files: Vec<String>,
    ) -> Result<Vec<TodoTask>, McpError> {
        info!(
            "Creating transpilation tasks for {} files",
            python_files.len()
        );

        let mut tasks = Vec::new();

        for (idx, file) in python_files.iter().enumerate() {
            let task = TodoTask {
                id: format!("transpile_{}", idx),
                title: format!("Transpile {}", file),
                description: format!("Convert Python file {} to Rust", file),
                status: TaskStatus::Pending,
                priority: self.calculate_priority(file),
                tags: vec!["transpilation".to_string(), "python-to-rust".to_string()],
                dependencies: Vec::new(),
                metadata: serde_json::json!({
                    "source_file": file,
                    "target_language": "rust",
                }),
            };

            tasks.push(task.clone());
            self.tasks.push(task);
        }

        // Add quality check tasks
        for (idx, file) in python_files.iter().enumerate() {
            let task = TodoTask {
                id: format!("quality_check_{}", idx),
                title: format!("Quality check for {}", file),
                description: "Verify transpiled Rust code meets pmat standards".to_string(),
                status: TaskStatus::Pending,
                priority: 2,
                tags: vec!["quality".to_string(), "pmat".to_string()],
                dependencies: vec![format!("transpile_{}", idx)],
                metadata: serde_json::json!({
                    "source_file": file,
                    "check_type": "pmat_quality",
                }),
            };

            tasks.push(task.clone());
            self.tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn update_task_status(
        &mut self,
        task_id: &str,
        status: TaskStatus,
    ) -> Result<(), McpError> {
        for task in &mut self.tasks {
            if task.id == task_id {
                task.status = status;
                return Ok(());
            }
        }
        Err(McpError::invalid_params(format!(
            "Task {} not found",
            task_id
        )))
    }

    pub async fn get_pending_tasks(&self) -> Vec<TodoTask> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .cloned()
            .collect()
    }

    fn analyze_rust_ast(&self, file: &syn::File) -> QualityMetrics {
        let mut test_count = 0;
        let mut doc_count = 0;
        let mut function_count = 0;
        let mut complexity = 0.0;

        for item in &file.items {
            match item {
                syn::Item::Fn(func) => {
                    function_count += 1;

                    // Check for test functions
                    for attr in &func.attrs {
                        if attr.path().is_ident("test") {
                            test_count += 1;
                        }
                        // Check for doc comments
                        if attr.path().is_ident("doc") {
                            doc_count += 1;
                        }
                    }

                    // Analyze function complexity
                    complexity += self.calculate_function_complexity(func);
                }
                syn::Item::Mod(module) => {
                    // Check for test modules
                    for attr in &module.attrs {
                        if attr.path().is_ident("cfg") {
                            test_count += 1;
                        }
                    }
                }
                _ => {}
            }
        }

        let function_count_f = function_count as f64;

        QualityMetrics {
            cyclomatic_complexity: if function_count > 0 {
                complexity / function_count_f
            } else {
                0.0
            },
            cognitive_complexity: if function_count > 0 {
                (complexity * 1.5) / function_count_f
            } else {
                0.0
            },
            test_coverage: if function_count > 0 {
                (test_count as f64 / function_count_f) * 100.0
            } else {
                0.0
            },
            documentation_coverage: if function_count > 0 {
                (doc_count as f64 / function_count_f) * 100.0
            } else {
                0.0
            },
            type_safety_score: 85.0, // Rust has strong type safety by default
        }
    }

    fn calculate_function_complexity(&self, func: &syn::ItemFn) -> f64 {
        // Simple complexity estimation based on control flow
        let body_str = quote::quote!(#func).to_string();
        let mut complexity = 1.0; // Base complexity

        // Count control flow keywords
        complexity += body_str.matches(" if ").count() as f64;
        complexity += body_str.matches(" match ").count() as f64 * 2.0;
        complexity += body_str.matches(" for ").count() as f64 * 2.0;
        complexity += body_str.matches(" while ").count() as f64 * 2.0;
        complexity += body_str.matches(" loop ").count() as f64 * 2.0;
        complexity += body_str.matches(" ? ").count() as f64 * 0.5; // Error propagation

        complexity
    }

    fn check_violations(&self, rust_code: &str, metrics: &QualityMetrics) -> Vec<QualityViolation> {
        let mut violations = Vec::new();

        // Check for missing tests
        if metrics.test_coverage < 30.0 {
            violations.push(QualityViolation {
                severity: ViolationSeverity::Warning,
                rule: "missing_tests".to_string(),
                message: "Transpiled code lacks test coverage".to_string(),
                location: None,
            });
        }

        // Check for missing documentation
        if metrics.documentation_coverage < 40.0 {
            violations.push(QualityViolation {
                severity: ViolationSeverity::Info,
                rule: "missing_docs".to_string(),
                message: "Consider adding documentation to transpiled code".to_string(),
                location: None,
            });
        }

        // Check for high complexity
        if metrics.cyclomatic_complexity > 10.0 {
            violations.push(QualityViolation {
                severity: ViolationSeverity::Warning,
                rule: "high_complexity".to_string(),
                message: "Transpiled code has high cyclomatic complexity".to_string(),
                location: None,
            });
        }

        // Check for unsafe code
        if rust_code.contains("unsafe ") {
            violations.push(QualityViolation {
                severity: ViolationSeverity::Error,
                rule: "unsafe_code".to_string(),
                message: "Transpiled code contains unsafe blocks".to_string(),
                location: None,
            });
        }

        violations
    }

    fn calculate_score(
        &self,
        metrics: &QualityMetrics,
        violations: &[QualityViolation],
        syntax_valid: bool,
    ) -> f64 {
        let mut score = 100.0;

        if !syntax_valid {
            score -= 50.0;
        }

        // Deduct for violations
        for violation in violations {
            match violation.severity {
                ViolationSeverity::Error => score -= 20.0,
                ViolationSeverity::Warning => score -= 10.0,
                ViolationSeverity::Info => score -= 5.0,
            }
        }

        // Factor in metrics
        score -= 10.0 - metrics.cyclomatic_complexity.min(10.0);
        score += metrics.test_coverage * 0.1;
        score += metrics.documentation_coverage * 0.05;
        score += metrics.type_safety_score * 0.1;

        score.clamp(0.0, 100.0)
    }

    fn generate_suggestions(
        &self,
        metrics: &QualityMetrics,
        violations: &[QualityViolation],
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        if metrics.test_coverage < 50.0 {
            suggestions.push("Add unit tests for transpiled functions".to_string());
        }

        if metrics.documentation_coverage < 60.0 {
            suggestions
                .push("Add documentation comments to public functions and modules".to_string());
        }

        if metrics.cyclomatic_complexity > 8.0 {
            suggestions.push(
                "Consider refactoring complex functions into smaller, more focused ones"
                    .to_string(),
            );
        }

        if metrics.type_safety_score < 80.0 {
            suggestions.push("Add explicit type annotations to improve type safety".to_string());
        }

        for violation in violations {
            if violation.severity == ViolationSeverity::Error {
                suggestions.push(format!("Fix critical issue: {}", violation.message));
            }
        }

        suggestions
    }

    fn calculate_priority(&self, file: &str) -> u8 {
        // Prioritize based on file importance
        if file.contains("main") || file.contains("lib") {
            1
        } else if file.contains("test") {
            3
        } else {
            2
        }
    }
}

impl Default for PmatIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quality_check() {
        let integration = PmatIntegration::new();

        let rust_code = r#"
            /// A simple function
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            
            #[cfg(test)]
            mod tests {
                use super::*;
                
                #[test]
                fn test_add() {
                    assert_eq!(add(2, 2), 4);
                }
            }
        "#;

        let report = integration
            .check_transpiled_code(rust_code, None)
            .await
            .unwrap();
        assert!(report.passes);
        assert!(report.score > 70.0);
    }

    #[tokio::test]
    async fn test_task_creation() {
        let mut integration = PmatIntegration::new();

        let files = vec![
            "main.py".to_string(),
            "utils.py".to_string(),
            "test_main.py".to_string(),
        ];

        let tasks = integration.create_transpilation_tasks(files).await.unwrap();
        assert_eq!(tasks.len(), 6); // 3 transpile + 3 quality check tasks
    }
}
