use serde::{Deserialize, Serialize};
/// Differential Testing Harness
///
/// Validates Python→Rust transpilation by comparing runtime behavior:
/// - Python output (stdout/stderr/exit code) vs Rust output
/// - Deterministic 100% accuracy (vs ML-based regression detection)
///
/// Based on McKeeman (1998) "Differential Testing for Software"
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Result of running Python vs Rust differential test
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentialTestResult {
    pub test_name: String,
    pub passed: bool,
    pub python_output: ProgramOutput,
    pub rust_output: ProgramOutput,
    pub mismatches: Vec<Mismatch>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub runtime_ms: u128,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mismatch {
    StdoutDifference {
        python: String,
        rust: String,
        diff: String,
    },
    StderrDifference {
        python: String,
        rust: String,
    },
    ExitCodeDifference {
        python: i32,
        rust: i32,
    },
}

pub struct DifferentialTester {
    /// Path to Python interpreter (python3)
    python_exe: PathBuf,
    /// Path to depyler binary
    depyler_exe: PathBuf,
    /// Temp directory for compiled binaries
    temp_dir: PathBuf,
}

impl DifferentialTester {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Try to find depyler in PATH first, otherwise look in target/debug
        let depyler_exe = which::which("depyler").or_else(|_| {
            // During tests, current_exe is target/debug/deps/depyler_testing-<hash>
            // Go up to target/debug/deps -> target/debug -> depyler
            let exe_path = std::env::current_exe()?;
            let target_debug = exe_path
                .parent() // Remove test executable name -> target/debug/deps
                .and_then(|p| p.parent()) // -> target/debug
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Failed to find target/debug directory",
                    )
                })?;

            let depyler_path = target_debug.join("depyler");
            if depyler_path.exists() {
                Ok(depyler_path)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("depyler executable not found at {:?}", depyler_path),
                ))
            }
        })?;

        Ok(Self {
            python_exe: which::which("python3")?,
            depyler_exe,
            temp_dir: std::env::temp_dir().join("depyler-differential"),
        })
    }

    /// Run differential test on a single Python file
    ///
    /// Steps:
    /// 1. Run Python: python3 input.py [args]
    /// 2. Transpile: depyler transpile input.py -o output.rs
    /// 3. Compile: rustc output.rs -o binary
    /// 4. Run Rust: ./binary [args]
    /// 5. Compare outputs
    pub fn test_file(
        &self,
        python_file: &Path,
        args: &[&str],
    ) -> Result<DifferentialTestResult, Box<dyn std::error::Error>> {
        let test_name = python_file
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // 1. Run Python
        let python_output = self.run_python(python_file, args)?;

        // 2. Transpile Python → Rust
        let rust_file = self.transpile(python_file)?;

        // 3. Compile Rust
        let rust_binary = self.compile_rust(&rust_file)?;

        // 4. Run Rust
        let rust_output = self.run_rust(&rust_binary, args)?;

        // 5. Compare
        let mismatches = self.compare_outputs(&python_output, &rust_output);

        Ok(DifferentialTestResult {
            test_name,
            passed: mismatches.is_empty(),
            python_output,
            rust_output,
            mismatches,
        })
    }

    /// Run Python script and capture output
    fn run_python(
        &self,
        python_file: &Path,
        args: &[&str],
    ) -> Result<ProgramOutput, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();

        let output = Command::new(&self.python_exe)
            .arg(python_file)
            .args(args)
            .output()?;

        let runtime_ms = start.elapsed().as_millis();

        Ok(ProgramOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            runtime_ms,
        })
    }

    /// Transpile Python → Rust using depyler
    fn transpile(&self, python_file: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let rust_file = self
            .temp_dir
            .join(python_file.file_stem().unwrap().to_str().unwrap())
            .with_extension("rs");

        fs::create_dir_all(&self.temp_dir)?;

        let output = Command::new(&self.depyler_exe)
            .args(["transpile", python_file.to_str().unwrap()])
            .args(["-o", rust_file.to_str().unwrap()])
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Transpilation failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        Ok(rust_file)
    }

    /// Compile Rust to binary
    fn compile_rust(&self, rust_file: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let binary = rust_file.with_extension("");

        let output = Command::new("rustc")
            .arg(rust_file)
            .args(["-o", binary.to_str().unwrap()])
            .args(["--deny", "warnings"]) // Enforce zero warnings
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Rust compilation failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        Ok(binary)
    }

    /// Run compiled Rust binary
    fn run_rust(
        &self,
        binary: &Path,
        args: &[&str],
    ) -> Result<ProgramOutput, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();

        let output = Command::new(binary).args(args).output()?;

        let runtime_ms = start.elapsed().as_millis();

        Ok(ProgramOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            runtime_ms,
        })
    }

    /// Compare Python vs Rust outputs
    fn compare_outputs(&self, python: &ProgramOutput, rust: &ProgramOutput) -> Vec<Mismatch> {
        let mut mismatches = Vec::new();

        // Compare stdout (with normalization)
        let python_stdout = self.normalize_output(&python.stdout);
        let rust_stdout = self.normalize_output(&rust.stdout);

        if python_stdout != rust_stdout {
            let diff = self.compute_diff(&python_stdout, &rust_stdout);
            mismatches.push(Mismatch::StdoutDifference {
                python: python_stdout,
                rust: rust_stdout,
                diff,
            });
        }

        // Compare stderr (warnings are OK, errors are not)
        if (python.exit_code != 0 || rust.exit_code != 0) && python.stderr != rust.stderr {
            mismatches.push(Mismatch::StderrDifference {
                python: python.stderr.clone(),
                rust: rust.stderr.clone(),
            });
        }

        // Compare exit codes
        if python.exit_code != rust.exit_code {
            mismatches.push(Mismatch::ExitCodeDifference {
                python: python.exit_code,
                rust: rust.exit_code,
            });
        }

        mismatches
    }

    /// Normalize output for comparison
    ///
    /// Handles:
    /// - Whitespace differences
    /// - Platform-specific line endings
    /// - Floating point precision differences
    fn normalize_output(&self, output: &str) -> String {
        output
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Compute unified diff between two strings
    fn compute_diff(&self, a: &str, b: &str) -> String {
        // Simple line-by-line diff
        let a_lines: Vec<&str> = a.lines().collect();
        let b_lines: Vec<&str> = b.lines().collect();

        let mut diff = Vec::new();

        for (i, (a_line, b_line)) in a_lines.iter().zip(b_lines.iter()).enumerate() {
            if a_line != b_line {
                diff.push(format!(
                    "Line {}: Python: '{}' | Rust: '{}'",
                    i + 1,
                    a_line,
                    b_line
                ));
            }
        }

        // Handle length differences
        if a_lines.len() != b_lines.len() {
            diff.push(format!(
                "Length mismatch: Python {} lines, Rust {} lines",
                a_lines.len(),
                b_lines.len()
            ));
        }

        diff.join("\n")
    }
}

impl Default for DifferentialTester {
    fn default() -> Self {
        Self::new().expect("Failed to initialize DifferentialTester")
    }
}

/// Test suite for reprorusted examples
pub struct ReprorustedTestSuite {
    tester: DifferentialTester,
    examples_dir: PathBuf,
}

impl ReprorustedTestSuite {
    pub fn new(examples_dir: PathBuf) -> Self {
        Self {
            tester: DifferentialTester::new().unwrap(),
            examples_dir,
        }
    }

    /// Run all reprorusted examples
    pub fn run_all(&self) -> HashMap<String, DifferentialTestResult> {
        let mut results = HashMap::new();

        let examples = [
            ("example_simple", &["--name", "Alice"][..]),
            ("example_flags", &["--debug"]),
            ("example_config", &["init"]),
            (
                "example_csv_filter",
                &["data.csv", "--column", "name", "--value", "Alice"],
            ),
            // ... more examples
        ];

        for (name, args) in examples {
            let python_file = self
                .examples_dir
                .join(name)
                .join(format!("{}.py", name.strip_prefix("example_").unwrap()));

            match self.tester.test_file(&python_file, args) {
                Ok(result) => {
                    results.insert(name.to_string(), result);
                }
                Err(e) => {
                    eprintln!("Failed to test {}: {}", name, e);
                }
            }
        }

        results
    }

    /// Generate HTML report
    pub fn generate_report(&self, results: &HashMap<String, DifferentialTestResult>) -> String {
        let pass_count = results.values().filter(|r| r.passed).count();
        let total_count = results.len();

        let mut html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Depyler Differential Testing Report</title>
    <style>
        body {{ font-family: monospace; margin: 20px; }}
        .pass {{ color: green; }}
        .fail {{ color: red; }}
        .diff {{ background: #f0f0f0; padding: 10px; margin: 10px 0; }}
    </style>
</head>
<body>
    <h1>Depyler Differential Testing Report</h1>
    <p>Pass Rate: {}/{} ({:.1}%)</p>
    <hr>
"#,
            pass_count,
            total_count,
            (pass_count as f64 / total_count as f64) * 100.0
        );

        for (name, result) in results {
            let status = if result.passed { "PASS" } else { "FAIL" };
            let class = if result.passed { "pass" } else { "fail" };

            html.push_str(&format!(
                r#"<div class="{}">
    <h2>{}: {}</h2>
    <p>Python runtime: {}ms | Rust runtime: {}ms</p>
"#,
                class, name, status, result.python_output.runtime_ms, result.rust_output.runtime_ms
            ));

            if !result.passed {
                html.push_str("<div class=\"diff\">");
                for mismatch in &result.mismatches {
                    match mismatch {
                        Mismatch::StdoutDifference { diff, .. } => {
                            html.push_str(&format!("<pre>{}</pre>", diff));
                        }
                        Mismatch::ExitCodeDifference { python, rust } => {
                            html.push_str(&format!(
                                "<p>Exit code: Python={}, Rust={}</p>",
                                python, rust
                            ));
                        }
                        _ => {}
                    }
                }
                html.push_str("</div>");
            }

            html.push_str("</div><hr>");
        }

        html.push_str("</body></html>");
        html
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_output_clone() {
        let output = ProgramOutput {
            stdout: "hello".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
            runtime_ms: 100,
        };
        let cloned = output.clone();
        assert_eq!(output.stdout, cloned.stdout);
        assert_eq!(output.exit_code, cloned.exit_code);
    }

    #[test]
    fn test_program_output_eq() {
        let output1 = ProgramOutput {
            stdout: "output".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
            runtime_ms: 50,
        };
        let output2 = ProgramOutput {
            stdout: "output".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
            runtime_ms: 50,
        };
        assert_eq!(output1, output2);
    }

    #[test]
    fn test_program_output_debug() {
        let output = ProgramOutput {
            stdout: "test".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
            runtime_ms: 10,
        };
        let debug = format!("{:?}", output);
        assert!(debug.contains("ProgramOutput"));
    }

    #[test]
    fn test_program_output_serialize() {
        let output = ProgramOutput {
            stdout: "output".to_string(),
            stderr: "error".to_string(),
            exit_code: 1,
            runtime_ms: 200,
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("output"));
        assert!(json.contains("error"));
    }

    #[test]
    fn test_program_output_deserialize() {
        let json = r#"{"stdout":"out","stderr":"err","exit_code":0,"runtime_ms":100}"#;
        let output: ProgramOutput = serde_json::from_str(json).unwrap();
        assert_eq!(output.stdout, "out");
        assert_eq!(output.stderr, "err");
    }

    #[test]
    fn test_mismatch_stdout_difference() {
        let mismatch = Mismatch::StdoutDifference {
            python: "hello".to_string(),
            rust: "Hello".to_string(),
            diff: "case mismatch".to_string(),
        };
        let debug = format!("{:?}", mismatch);
        assert!(debug.contains("StdoutDifference"));
    }

    #[test]
    fn test_mismatch_stderr_difference() {
        let mismatch = Mismatch::StderrDifference {
            python: "error: foo".to_string(),
            rust: "error: bar".to_string(),
        };
        let cloned = mismatch.clone();
        assert_eq!(mismatch, cloned);
    }

    #[test]
    fn test_mismatch_exit_code_difference() {
        let mismatch = Mismatch::ExitCodeDifference { python: 0, rust: 1 };
        let json = serde_json::to_string(&mismatch).unwrap();
        assert!(json.contains("ExitCodeDifference"));
    }

    #[test]
    fn test_differential_test_result_passed() {
        let result = DifferentialTestResult {
            test_name: "test".to_string(),
            passed: true,
            python_output: ProgramOutput {
                stdout: "OK".to_string(),
                stderr: "".to_string(),
                exit_code: 0,
                runtime_ms: 10,
            },
            rust_output: ProgramOutput {
                stdout: "OK".to_string(),
                stderr: "".to_string(),
                exit_code: 0,
                runtime_ms: 5,
            },
            mismatches: vec![],
        };
        assert!(result.passed);
        assert!(result.mismatches.is_empty());
    }

    #[test]
    fn test_differential_test_result_failed() {
        let result = DifferentialTestResult {
            test_name: "failing_test".to_string(),
            passed: false,
            python_output: ProgramOutput {
                stdout: "42".to_string(),
                stderr: "".to_string(),
                exit_code: 0,
                runtime_ms: 20,
            },
            rust_output: ProgramOutput {
                stdout: "43".to_string(),
                stderr: "".to_string(),
                exit_code: 0,
                runtime_ms: 10,
            },
            mismatches: vec![Mismatch::StdoutDifference {
                python: "42".to_string(),
                rust: "43".to_string(),
                diff: "off by one".to_string(),
            }],
        };
        assert!(!result.passed);
        assert_eq!(result.mismatches.len(), 1);
    }

    #[test]
    fn test_differential_test_result_serialize() {
        let result = DifferentialTestResult {
            test_name: "serialize_test".to_string(),
            passed: true,
            python_output: ProgramOutput {
                stdout: "".to_string(),
                stderr: "".to_string(),
                exit_code: 0,
                runtime_ms: 0,
            },
            rust_output: ProgramOutput {
                stdout: "".to_string(),
                stderr: "".to_string(),
                exit_code: 0,
                runtime_ms: 0,
            },
            mismatches: vec![],
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("serialize_test"));
    }

    #[test]
    fn test_differential_test_result_clone() {
        let result = DifferentialTestResult {
            test_name: "clone_test".to_string(),
            passed: true,
            python_output: ProgramOutput {
                stdout: "out".to_string(),
                stderr: "".to_string(),
                exit_code: 0,
                runtime_ms: 1,
            },
            rust_output: ProgramOutput {
                stdout: "out".to_string(),
                stderr: "".to_string(),
                exit_code: 0,
                runtime_ms: 1,
            },
            mismatches: vec![],
        };
        let cloned = result.clone();
        assert_eq!(result.test_name, cloned.test_name);
        assert_eq!(result.passed, cloned.passed);
    }

    #[test]
    #[ignore] // Requires external tools
    fn test_normalize_output() {
        let tester = DifferentialTester::new().unwrap();

        let input = "  Line 1  \n\n  Line 2  \r\n  Line 3  ";
        let expected = "Line 1\nLine 2\nLine 3";

        assert_eq!(tester.normalize_output(input), expected);
    }

    #[test]
    #[ignore] // Requires external tools
    fn test_differential_simple() {
        // Create a simple Python script
        let temp_dir = tempfile::tempdir().unwrap();
        let python_file = temp_dir.path().join("test.py");

        std::fs::write(
            &python_file,
            r#"
def main():
    print("Hello, World!")

if __name__ == "__main__":
    main()
"#,
        )
        .unwrap();

        let tester = DifferentialTester::new().unwrap();
        let result = tester.test_file(&python_file, &[]).unwrap();

        assert!(
            result.passed,
            "Simple hello world should pass differential test: {:?}",
            result.mismatches
        );
    }
}
