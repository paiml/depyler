//! Compilation verifier module (Phase 3).
//!
//! Verifies that transpiled Rust code compiles successfully.

use crate::config::CorpusConfig;
use crate::transpiler::TranspilationResult;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

/// Result of compiling a single file.
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// The Rust file being compiled.
    pub rust_file: PathBuf,
    /// The original Python file.
    pub python_file: PathBuf,
    /// Whether compilation succeeded.
    pub success: bool,
    /// Compiler exit code.
    pub exit_code: Option<i32>,
    /// Compiler stderr output.
    pub stderr: Option<String>,
    /// Compiler stdout output.
    pub stdout: Option<String>,
    /// Compilation duration.
    pub duration: Duration,
    /// Whether Cargo-First was used.
    pub cargo_first: bool,
}

/// Compilation verifier for transpiled Rust files.
pub struct CompilationVerifier<'a> {
    #[allow(dead_code)]
    config: &'a CorpusConfig,
}

impl<'a> CompilationVerifier<'a> {
    /// Create a new compilation verifier.
    pub fn new(config: &'a CorpusConfig) -> Self {
        Self { config }
    }

    /// Verify compilation for all transpiled files.
    pub fn verify(
        &self,
        transpile_results: &[TranspilationResult],
    ) -> Result<Vec<CompilationResult>> {
        let mut results = Vec::new();

        for tr in transpile_results {
            if !tr.success {
                continue; // Skip files that failed to transpile
            }

            let result = if let Some(ref cargo_dir) = tr.cargo_dir {
                self.verify_with_cargo(cargo_dir, &tr.python_file)?
            } else if let Some(ref rs_file) = tr.rust_file {
                self.verify_with_rustc(rs_file, &tr.python_file)?
            } else {
                continue;
            };

            results.push(result);
        }

        Ok(results)
    }

    /// Verify using cargo build (Cargo-First path).
    pub fn verify_with_cargo(
        &self,
        cargo_dir: &Path,
        python_file: &Path,
    ) -> Result<CompilationResult> {
        let start = Instant::now();

        let output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(cargo_dir)
            .output()
            .with_context(|| format!("Failed to run cargo build in {}", cargo_dir.display()))?;

        let duration = start.elapsed();
        let success = output.status.success();

        // Find the .rs file in the directory
        let rust_file = self.find_rs_file(cargo_dir)?;

        Ok(CompilationResult {
            rust_file,
            python_file: python_file.to_path_buf(),
            success,
            exit_code: output.status.code(),
            stderr: Some(String::from_utf8_lossy(&output.stderr).to_string()),
            stdout: Some(String::from_utf8_lossy(&output.stdout).to_string()),
            duration,
            cargo_first: true,
        })
    }

    /// Verify using rustc directly (fallback path).
    pub fn verify_with_rustc(
        &self,
        rs_file: &Path,
        python_file: &Path,
    ) -> Result<CompilationResult> {
        let start = Instant::now();

        // Create a unique temp dir for output (rustc doesn't support -o /dev/null for libs)
        // Use thread ID for uniqueness to avoid races between parallel tests
        let temp_dir = std::env::temp_dir().join(format!(
            "depyler_verify_{}_{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::create_dir_all(&temp_dir);

        let output = Command::new("rustc")
            .arg("--crate-type")
            .arg("lib")
            .arg("--edition")
            .arg("2021")
            .arg(rs_file)
            .arg("--out-dir")
            .arg(&temp_dir)
            // Clear MAKEFLAGS to avoid jobserver issues in tests
            .env_remove("MAKEFLAGS")
            .output()
            .with_context(|| format!("Failed to run rustc on {}", rs_file.display()))?;

        // Clean up temp dir
        let _ = std::fs::remove_dir_all(&temp_dir);

        let duration = start.elapsed();
        let success = output.status.success();

        Ok(CompilationResult {
            rust_file: rs_file.to_path_buf(),
            python_file: python_file.to_path_buf(),
            success,
            exit_code: output.status.code(),
            stderr: Some(String::from_utf8_lossy(&output.stderr).to_string()),
            stdout: Some(String::from_utf8_lossy(&output.stdout).to_string()),
            duration,
            cargo_first: false,
        })
    }

    fn find_rs_file(&self, dir: &Path) -> Result<PathBuf> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                return Ok(path);
            }
        }
        // Return a placeholder if no .rs file found
        Ok(dir.join("unknown.rs"))
    }

    /// Get summary statistics.
    pub fn summarize(results: &[CompilationResult]) -> CompileSummary {
        let total = results.len();
        let success = results.iter().filter(|r| r.success).count();
        let cargo_first = results.iter().filter(|r| r.cargo_first).count();
        let total_duration: Duration = results.iter().map(|r| r.duration).sum();

        CompileSummary {
            total,
            success,
            failed: total - success,
            cargo_first_count: cargo_first,
            rustc_count: total - cargo_first,
            total_duration,
            single_shot_rate: if total > 0 {
                (success as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        }
    }
}

/// Summary of compilation results.
#[derive(Debug, Clone)]
pub struct CompileSummary {
    /// Total files verified.
    pub total: usize,
    /// Successfully compiled.
    pub success: usize,
    /// Failed compilation.
    pub failed: usize,
    /// Files compiled with Cargo-First.
    pub cargo_first_count: usize,
    /// Files compiled with rustc directly.
    pub rustc_count: usize,
    /// Total duration.
    pub total_duration: Duration,
    /// Single-shot compilation rate.
    pub single_shot_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_config(path: &Path) -> CorpusConfig {
        CorpusConfig::default()
            .with_corpus_path(path.to_path_buf())
            .with_skip_clean(true)
    }

    #[test]
    fn test_verify_with_rustc_valid_code() {
        let dir = TempDir::new().unwrap();
        let rs_file = dir.path().join("valid.rs");
        std::fs::write(&rs_file, "pub fn hello() {}").unwrap();

        let config = create_test_config(dir.path());
        let verifier = CompilationVerifier::new(&config);

        let result = verifier
            .verify_with_rustc(&rs_file, &PathBuf::from("test.py"))
            .unwrap();

        // Debug output for flaky coverage builds
        if !result.success {
            eprintln!("rustc stderr: {:?}", result.stderr);
            eprintln!("rustc exit code: {:?}", result.exit_code);
        }
        assert!(
            result.success,
            "rustc should compile valid code: {:?}",
            result.stderr
        );
        assert!(!result.cargo_first);
    }

    #[test]
    fn test_verify_with_rustc_invalid_code() {
        let dir = TempDir::new().unwrap();
        let rs_file = dir.path().join("invalid.rs");
        std::fs::write(&rs_file, "fn invalid { }").unwrap();

        let config = create_test_config(dir.path());
        let verifier = CompilationVerifier::new(&config);

        let result = verifier
            .verify_with_rustc(&rs_file, &PathBuf::from("test.py"))
            .unwrap();

        assert!(!result.success);
        assert!(result.stderr.is_some());
    }

    #[test]
    fn test_summarize_empty() {
        let summary = CompilationVerifier::summarize(&[]);
        assert_eq!(summary.total, 0);
        assert_eq!(summary.single_shot_rate, 0.0);
    }

    #[test]
    fn test_summarize_with_results() {
        let results = vec![
            CompilationResult {
                rust_file: PathBuf::from("a.rs"),
                python_file: PathBuf::from("a.py"),
                success: true,
                exit_code: Some(0),
                stderr: None,
                stdout: None,
                duration: Duration::from_millis(100),
                cargo_first: true,
            },
            CompilationResult {
                rust_file: PathBuf::from("b.rs"),
                python_file: PathBuf::from("b.py"),
                success: false,
                exit_code: Some(1),
                stderr: Some("error".to_string()),
                stdout: None,
                duration: Duration::from_millis(50),
                cargo_first: false,
            },
        ];

        let summary = CompilationVerifier::summarize(&results);
        assert_eq!(summary.total, 2);
        assert_eq!(summary.success, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.cargo_first_count, 1);
        assert_eq!(summary.rustc_count, 1);
        assert_eq!(summary.single_shot_rate, 50.0);
    }
}
