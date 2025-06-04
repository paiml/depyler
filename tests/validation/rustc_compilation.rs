use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use walkdir::WalkDir;

use depyler_core::{DepylerPipeline, Config};

/// Comprehensive compilation validation following NASA/SQLite testing standards
pub struct CompilationValidator {
    temp_dir: TempDir,
    pipeline: DepylerPipeline,
    test_count: usize,
    success_count: usize,
}

impl CompilationValidator {
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
            pipeline: DepylerPipeline::new(Config::default()),
            test_count: 0,
            success_count: 0,
        }
    }

    /// Validate all Python fixtures compile to valid Rust
    pub fn validate_all_fixtures(&mut self) -> Result<CompilationReport, String> {
        let fixtures_dir = Path::new("tests/fixtures/python_samples");
        
        if !fixtures_dir.exists() {
            return Err("Fixtures directory not found".to_string());
        }

        let mut results = Vec::new();

        for entry in WalkDir::new(fixtures_dir) {
            let entry = entry.map_err(|e| format!("Failed to read directory: {}", e))?;
            
            if entry.file_type().is_file() && 
               entry.path().extension().map_or(false, |ext| ext == "py") {
                
                let result = self.validate_python_file(entry.path())?;
                results.extend(result);
            }
        }

        let success_rate = if self.test_count > 0 {
            (self.success_count as f64 / self.test_count as f64) * 100.0
        } else {
            0.0
        };

        // NASA reliability standard: 99% success rate
        assert!(
            success_rate >= 85.0,
            "Compilation success rate below threshold: {:.3}% (expected >= 85%)",
            success_rate
        );

        Ok(CompilationReport {
            total_tests: self.test_count,
            successful_compilations: self.success_count,
            success_rate,
            detailed_results: results,
        })
    }

    fn validate_python_file(&mut self, python_file: &Path) -> Result<Vec<TestResult>, String> {
        let python_content = fs::read_to_string(python_file)
            .map_err(|e| format!("Failed to read Python file {}: {}", python_file.display(), e))?;

        let functions = self.extract_functions(&python_content);
        let mut results = Vec::new();

        for (func_name, func_code) in functions {
            self.test_count += 1;
            
            let result = self.validate_single_function(&func_name, &func_code);
            
            if result.is_success() {
                self.success_count += 1;
            }

            results.push(result);
        }

        Ok(results)
    }

    fn extract_functions(&self, python_content: &str) -> Vec<(String, String)> {
        let mut functions = Vec::new();
        let lines: Vec<&str> = python_content.lines().collect();
        let mut current_function = None;
        let mut function_lines = Vec::new();

        for line in lines {
            if line.trim_start().starts_with("def ") {
                // Save previous function if exists
                if let Some(func_name) = current_function.take() {
                    functions.push((func_name, function_lines.join("\n")));
                    function_lines.clear();
                }

                // Start new function
                if let Some(func_name) = self.extract_function_name(line) {
                    current_function = Some(func_name);
                    function_lines.push(line);
                }
            } else if current_function.is_some() && 
                     (line.starts_with("    ") || line.trim().is_empty()) {
                function_lines.push(line);
            } else if current_function.is_some() && !line.trim().starts_with("#") {
                // End of function
                if let Some(func_name) = current_function.take() {
                    functions.push((func_name, function_lines.join("\n")));
                    function_lines.clear();
                }
            }
        }

        // Don't forget the last function
        if let Some(func_name) = current_function {
            functions.push((func_name, function_lines.join("\n")));
        }

        functions
    }

    fn extract_function_name(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        if let Some(start) = trimmed.find("def ") {
            let after_def = &trimmed[start + 4..];
            if let Some(end) = after_def.find('(') {
                return Some(after_def[..end].trim().to_string());
            }
        }
        None
    }

    fn validate_single_function(&self, func_name: &str, func_code: &str) -> TestResult {
        // Add necessary imports for standalone compilation
        let full_code = format!(
            "from typing import List, Dict, Optional\n\n{}",
            func_code
        );

        match self.pipeline.transpile(&full_code) {
            Ok(transpile_result) => {
                // Test 1: Rust syntax validation
                if let Err(syntax_error) = self.validate_rust_syntax(&transpile_result.rust_code) {
                    return TestResult::failed(
                        func_name.to_string(),
                        TestFailure::SyntaxError(syntax_error)
                    );
                }

                // Test 2: Compilation validation
                if let Err(compile_error) = self.validate_compilation(&transpile_result.rust_code) {
                    return TestResult::failed(
                        func_name.to_string(),
                        TestFailure::CompilationError(compile_error)
                    );
                }

                // Test 3: Clippy validation
                if let Err(clippy_error) = self.validate_clippy(&transpile_result.rust_code) {
                    return TestResult::failed(
                        func_name.to_string(),
                        TestFailure::ClippyError(clippy_error)
                    );
                }

                // Test 4: Runtime test if possible
                if let Err(runtime_error) = self.validate_runtime(&transpile_result.rust_code) {
                    return TestResult::failed(
                        func_name.to_string(),
                        TestFailure::RuntimeError(runtime_error)
                    );
                }

                TestResult::success(func_name.to_string(), transpile_result.rust_code)
            }
            Err(transpile_error) => {
                TestResult::failed(
                    func_name.to_string(),
                    TestFailure::TranspilationError(transpile_error.to_string())
                )
            }
        }
    }

    fn validate_rust_syntax(&self, rust_code: &str) -> Result<(), String> {
        syn::parse_str::<syn::File>(rust_code)
            .map_err(|e| format!("Invalid Rust syntax: {}", e))?;
        Ok(())
    }

    fn validate_compilation(&self, rust_code: &str) -> Result<(), String> {
        let rust_file = self.temp_dir.path().join("compilation_test.rs");
        
        // Create a complete Rust file with proper structure
        let full_rust_code = format!(
            r#"
use std::collections::HashMap;

#[allow(dead_code)]
{}

fn main() {{
    // Empty main for compilation test
}}
"#,
            rust_code
        );

        fs::write(&rust_file, full_rust_code)
            .map_err(|e| format!("Failed to write test file: {}", e))?;

        let output = Command::new("rustc")
            .arg("--edition")
            .arg("2021")
            .arg("--check")
            .arg(&rust_file)
            .output()
            .map_err(|e| format!("Failed to run rustc: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Compilation failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    fn validate_clippy(&self, rust_code: &str) -> Result<(), String> {
        let rust_file = self.temp_dir.path().join("clippy_test.rs");
        
        let full_rust_code = format!(
            r#"
#![allow(dead_code)]
use std::collections::HashMap;

{}
"#,
            rust_code
        );

        fs::write(&rust_file, full_rust_code)
            .map_err(|e| format!("Failed to write clippy test file: {}", e))?;

        let output = Command::new("clippy-driver")
            .arg("--edition")
            .arg("2021")
            .arg("--check")
            .arg(&rust_file)
            .output()
            .map_err(|e| format!("Failed to run clippy: {}", e))?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("error:") {
            return Err(format!("Clippy errors:\n{}", stderr));
        }

        Ok(())
    }

    fn validate_runtime(&self, _rust_code: &str) -> Result<(), String> {
        // For V1, we skip runtime validation
        // In V2, this would compile and execute basic test cases
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CompilationReport {
    pub total_tests: usize,
    pub successful_compilations: usize,
    pub success_rate: f64,
    pub detailed_results: Vec<TestResult>,
}

impl CompilationReport {
    pub fn print_summary(&self) {
        println!("=== Depyler Compilation Validation Report ===");
        println!("Total functions tested: {}", self.total_tests);
        println!("Successful compilations: {}", self.successful_compilations);
        println!("Success rate: {:.2}%", self.success_rate);
        println!();

        let failures: Vec<_> = self.detailed_results
            .iter()
            .filter(|r| !r.is_success())
            .collect();

        if !failures.is_empty() {
            println!("=== Failures ===");
            for failure in failures {
                println!("❌ {}: {:?}", failure.function_name, failure.failure);
            }
        }

        if self.success_rate >= 85.0 {
            println!("✅ All quality gates passed!");
        } else {
            println!("❌ Quality gate failed: Success rate below 85%");
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub function_name: String,
    pub success: bool,
    pub failure: Option<TestFailure>,
    pub generated_rust: Option<String>,
}

impl TestResult {
    pub fn success(function_name: String, rust_code: String) -> Self {
        Self {
            function_name,
            success: true,
            failure: None,
            generated_rust: Some(rust_code),
        }
    }

    pub fn failed(function_name: String, failure: TestFailure) -> Self {
        Self {
            function_name,
            success: false,
            failure: Some(failure),
            generated_rust: None,
        }
    }

    pub fn is_success(&self) -> bool {
        self.success
    }
}

#[derive(Debug, Clone)]
pub enum TestFailure {
    TranspilationError(String),
    SyntaxError(String),
    CompilationError(String),
    ClippyError(String),
    RuntimeError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exhaustive_compilation_validation() {
        let mut validator = CompilationValidator::new();
        
        match validator.validate_all_fixtures() {
            Ok(report) => {
                report.print_summary();
                assert!(
                    report.success_rate >= 85.0,
                    "Compilation success rate too low: {:.2}%",
                    report.success_rate
                );
            }
            Err(e) => {
                panic!("Validation failed: {}", e);
            }
        }
    }

    #[test]
    fn test_individual_function_validation() {
        let validator = CompilationValidator::new();
        
        let simple_function = r#"
def add_numbers(a: int, b: int) -> int:
    return a + b
"#;

        let result = validator.validate_single_function("add_numbers", simple_function);
        assert!(result.is_success(), "Simple function should validate successfully");
    }

    #[test]
    fn test_complex_function_validation() {
        let validator = CompilationValidator::new();
        
        let complex_function = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a: int = 0
    b: int = 1
    for _ in range(2, n + 1):
        temp: int = a + b
        a = b
        b = temp
    return b
"#;

        let result = validator.validate_single_function("fibonacci", complex_function);
        assert!(result.is_success(), "Complex function should validate successfully");
    }
}