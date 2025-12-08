//! Transpiler runner module (Phase 2).
//!
//! Executes depyler transpile on all Python files in the corpus.

use crate::config::CorpusConfig;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use walkdir::WalkDir;

/// Result of transpiling a single file.
#[derive(Debug, Clone)]
pub struct TranspilationResult {
    /// The source Python file.
    pub python_file: PathBuf,
    /// The generated Rust file (if successful).
    pub rust_file: Option<PathBuf>,
    /// The generated Cargo.toml directory (if Cargo-First).
    pub cargo_dir: Option<PathBuf>,
    /// Whether transpilation succeeded.
    pub success: bool,
    /// Error message if failed.
    pub error: Option<String>,
    /// Transpilation duration.
    pub duration: Duration,
}

/// Batch transpiler for corpus analysis.
pub struct TranspileRunner<'a> {
    config: &'a CorpusConfig,
}

impl<'a> TranspileRunner<'a> {
    /// Create a new transpile runner.
    pub fn new(config: &'a CorpusConfig) -> Self {
        Self { config }
    }

    /// Run transpilation on all Python files in the corpus.
    pub fn run(&self) -> Result<Vec<TranspilationResult>> {
        let python_files = self.find_python_files()?;
        let mut results = Vec::with_capacity(python_files.len());

        for py_file in python_files {
            let result = self.transpile_file(&py_file)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Find all Python files matching the include/exclude patterns.
    pub fn find_python_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.config.corpus_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("py") {
                continue;
            }

            // Check exclude patterns
            let path_str = path.to_string_lossy();
            let excluded = self.config.exclude_patterns.iter().any(|pattern| {
                // Simple pattern matching (could use glob crate for full support)
                if pattern.contains("__pycache__") && path_str.contains("__pycache__") {
                    return true;
                }
                if pattern.contains("test_") && path.file_name().is_some_and(|n| {
                    n.to_string_lossy().starts_with("test_")
                }) {
                    return true;
                }
                if pattern.contains("__init__") && path.file_name().is_some_and(|n| {
                    n.to_string_lossy() == "__init__.py"
                }) {
                    return true;
                }
                false
            });

            if !excluded {
                files.push(path.to_path_buf());
            }
        }

        files.sort();
        Ok(files)
    }

    /// Transpile a single Python file.
    pub fn transpile_file(&self, py_file: &Path) -> Result<TranspilationResult> {
        let start = Instant::now();

        let dir = py_file.parent().unwrap_or(Path::new("."));
        let stem = py_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let rs_file = dir.join(format!("{stem}.rs"));

        let depyler = self.config.depyler_binary();

        let output = Command::new(&depyler)
            .arg("transpile")
            .arg(py_file)
            .arg("-o")
            .arg(&rs_file)
            .output()
            .with_context(|| format!("Failed to run depyler on {}", py_file.display()))?;

        let duration = start.elapsed();
        let success = output.status.success() && rs_file.exists();

        // Check for Cargo.toml (Cargo-First)
        let cargo_toml = dir.join("Cargo.toml");
        let cargo_dir = if cargo_toml.exists() {
            Some(dir.to_path_buf())
        } else {
            None
        };

        Ok(TranspilationResult {
            python_file: py_file.to_path_buf(),
            rust_file: if success { Some(rs_file) } else { None },
            cargo_dir,
            success,
            error: if success {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
            duration,
        })
    }

    /// Get summary statistics.
    pub fn summarize(results: &[TranspilationResult]) -> TranspileSummary {
        let total = results.len();
        let success = results.iter().filter(|r| r.success).count();
        let with_cargo = results.iter().filter(|r| r.cargo_dir.is_some()).count();
        let total_duration: Duration = results.iter().map(|r| r.duration).sum();

        TranspileSummary {
            total,
            success,
            failed: total - success,
            with_cargo,
            total_duration,
            success_rate: if total > 0 {
                (success as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        }
    }
}

/// Summary of transpilation results.
#[derive(Debug, Clone)]
pub struct TranspileSummary {
    /// Total files processed.
    pub total: usize,
    /// Successfully transpiled.
    pub success: usize,
    /// Failed transpilation.
    pub failed: usize,
    /// Files with Cargo.toml generated (Cargo-First).
    pub with_cargo: usize,
    /// Total duration.
    pub total_duration: Duration,
    /// Success rate percentage.
    pub success_rate: f64,
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
    fn test_find_python_files_empty_dir() {
        let dir = TempDir::new().unwrap();
        let config = create_test_config(dir.path());
        let runner = TranspileRunner::new(&config);

        let files = runner.find_python_files().unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_find_python_files_with_files() {
        let dir = TempDir::new().unwrap();

        // Create Python files
        std::fs::write(dir.path().join("example.py"), "print('hello')").unwrap();
        std::fs::write(dir.path().join("another.py"), "x = 1").unwrap();

        let config = create_test_config(dir.path());
        let runner = TranspileRunner::new(&config);

        let files = runner.find_python_files().unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_find_python_files_excludes_test() {
        let dir = TempDir::new().unwrap();

        std::fs::write(dir.path().join("example.py"), "print('hello')").unwrap();
        std::fs::write(dir.path().join("test_example.py"), "assert True").unwrap();

        let config = create_test_config(dir.path());
        let runner = TranspileRunner::new(&config);

        let files = runner.find_python_files().unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("example.py"));
    }

    #[test]
    fn test_find_python_files_excludes_init() {
        let dir = TempDir::new().unwrap();

        std::fs::write(dir.path().join("example.py"), "print('hello')").unwrap();
        std::fs::write(dir.path().join("__init__.py"), "").unwrap();

        let config = create_test_config(dir.path());
        let runner = TranspileRunner::new(&config);

        let files = runner.find_python_files().unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_summarize_empty() {
        let summary = TranspileRunner::summarize(&[]);
        assert_eq!(summary.total, 0);
        assert_eq!(summary.success_rate, 0.0);
    }

    #[test]
    fn test_summarize_with_results() {
        let results = vec![
            TranspilationResult {
                python_file: PathBuf::from("a.py"),
                rust_file: Some(PathBuf::from("a.rs")),
                cargo_dir: Some(PathBuf::from(".")),
                success: true,
                error: None,
                duration: Duration::from_millis(100),
            },
            TranspilationResult {
                python_file: PathBuf::from("b.py"),
                rust_file: None,
                cargo_dir: None,
                success: false,
                error: Some("parse error".to_string()),
                duration: Duration::from_millis(50),
            },
        ];

        let summary = TranspileRunner::summarize(&results);
        assert_eq!(summary.total, 2);
        assert_eq!(summary.success, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.with_cargo, 1);
        assert_eq!(summary.success_rate, 50.0);
    }
}
