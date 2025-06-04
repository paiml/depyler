use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

use depyler_core::{DepylerPipeline, Config, TranspileResult};
use depyler_verify::{PropertyVerifier, VerificationResult};

#[derive(Debug)]
pub struct TranspilationTestHarness {
    temp_dir: TempDir,
    pipeline: DepylerPipeline,
    verifier: PropertyVerifier,
}

impl TranspilationTestHarness {
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
            pipeline: DepylerPipeline::new(Config::default()),
            verifier: PropertyVerifier::new(),
        }
    }

    pub fn test_transpilation(&self, python_source: &str, expected_rust: &str) -> Result<(), String> {
        // 1. Transpile Python to Rust
        let result = self.pipeline.transpile(python_source)
            .map_err(|e| format!("Transpilation failed: {}", e))?;

        // 2. Verify generated Rust compiles
        self.verify_rust_compiles(&result.rust_code)?;

        // 3. Verify generated code passes Clippy
        self.verify_clippy_passes(&result.rust_code)?;

        // 4. Compare with expected output (if provided)
        if !expected_rust.is_empty() {
            self.compare_outputs(&result.rust_code, expected_rust)?;
        }

        // 5. Run property verification
        self.verify_properties(&result)?;

        Ok(())
    }

    fn verify_rust_compiles(&self, rust_code: &str) -> Result<(), String> {
        let rust_file = self.temp_dir.path().join("test.rs");
        fs::write(&rust_file, rust_code)
            .map_err(|e| format!("Failed to write Rust file: {}", e))?;

        let output = Command::new("rustc")
            .arg("--check")
            .arg(&rust_file)
            .output()
            .map_err(|e| format!("Failed to run rustc: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Generated Rust code does not compile:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    fn verify_clippy_passes(&self, rust_code: &str) -> Result<(), String> {
        let rust_file = self.temp_dir.path().join("clippy_test.rs");
        fs::write(&rust_file, format!("#![allow(dead_code)]\n{}", rust_code))
            .map_err(|e| format!("Failed to write Rust file: {}", e))?;

        let output = Command::new("clippy-driver")
            .arg("--check")
            .arg(&rust_file)
            .output()
            .map_err(|e| format!("Failed to run clippy: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("error:") {
                return Err(format!("Generated Rust code has Clippy errors:\n{}", stderr));
            }
        }

        Ok(())
    }

    fn compare_outputs(&self, actual: &str, expected: &str) -> Result<(), String> {
        // Normalize whitespace for comparison
        let normalize = |s: &str| s.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        let actual_normalized = normalize(actual);
        let expected_normalized = normalize(expected);

        if actual_normalized != expected_normalized {
            return Err(format!(
                "Generated Rust does not match expected:\nActual:\n{}\nExpected:\n{}",
                actual, expected
            ));
        }

        Ok(())
    }

    fn verify_properties(&self, result: &TranspileResult) -> Result<(), String> {
        let verification_results = self.verifier.verify(&result.hir);
        
        for result in verification_results {
            if let VerificationResult::Failed(msg) = result {
                return Err(format!("Property verification failed: {}", msg));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function_transpilation() {
        let harness = TranspilationTestHarness::new();
        
        let python_code = r#"
def add_numbers(a: int, b: int) -> int:
    return a + b
"#;

        let expected_rust = r#"
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        harness.test_transpilation(python_code, expected_rust)
            .expect("Simple function transpilation should succeed");
    }

    #[test]
    fn test_list_operations() {
        let harness = TranspilationTestHarness::new();
        
        let python_code = r#"
from typing import List

def sum_list(numbers: List[int]) -> int:
    total: int = 0
    for n in numbers:
        total += n
    return total
"#;

        harness.test_transpilation(python_code, "")
            .expect("List operations should transpile successfully");
    }

    #[test]
    fn test_conditional_logic() {
        let harness = TranspilationTestHarness::new();
        
        let python_code = r#"
def classify_number(n: int) -> str:
    if n > 0:
        return "positive"
    elif n < 0:
        return "negative"
    else:
        return "zero"
"#;

        harness.test_transpilation(python_code, "")
            .expect("Conditional logic should transpile successfully");
    }
}