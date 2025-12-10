//! Batch compilation and error collection
//!
//! Handles parallel compilation of Python examples through the transpiler
//! and collects structured error information.
//!
//! Uses Cargo-First compilation strategy (DEPYLER-CARGO-FIRST) to ensure
//! proper dependency resolution for all generated Rust code.
//!
//! DEPYLER-CACHE-001: O(1) caching support via SqliteCache integration.

use super::cache::{
    CacheConfig, CacheEntry, CompilationStatus, SqliteCache, TranspilationCacheKey,
};
use super::reporter::progress_bar;
use super::state::DisplayMode;
use anyhow::Result;
use depyler_core::cargo_first;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// A single compilation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationError {
    /// Rust error code (e.g., E0599, E0308)
    pub code: String,
    /// Error message
    pub message: String,
    /// File where error occurred
    pub file: PathBuf,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
}

/// Result of compiling a single example
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// Source Python file
    pub source_file: PathBuf,
    /// Whether compilation succeeded
    pub success: bool,
    /// Compilation errors (if any)
    pub errors: Vec<CompilationError>,
    /// Generated Rust file (if transpilation succeeded)
    pub rust_file: Option<PathBuf>,
}

/// Batch compiler for Python examples
///
/// DEPYLER-CACHE-001: Optionally integrates with SqliteCache for O(1) lookups.
pub struct BatchCompiler {
    /// Directory containing examples
    input_dir: PathBuf,
    /// Number of parallel jobs
    parallel_jobs: usize,
    /// Optional compilation cache (DEPYLER-CACHE-001)
    cache: Option<SqliteCache>,
    /// Cache configuration
    cache_config: CacheConfig,
    /// Display mode for progress output
    display_mode: DisplayMode,
}

impl BatchCompiler {
    /// Create a new batch compiler
    pub fn new(input_dir: &Path) -> Self {
        let cache_config = CacheConfig::default();
        Self {
            input_dir: input_dir.to_path_buf(),
            parallel_jobs: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
            cache: None,
            cache_config,
            display_mode: DisplayMode::default(),
        }
    }

    /// Set display mode for progress output
    pub fn with_display_mode(mut self, display_mode: DisplayMode) -> Self {
        self.display_mode = display_mode;
        self
    }

    /// Set number of parallel jobs
    pub fn with_parallel_jobs(mut self, jobs: usize) -> Self {
        self.parallel_jobs = jobs;
        self
    }

    /// Enable caching with the given configuration (DEPYLER-CACHE-001)
    pub fn with_cache(mut self, config: CacheConfig) -> Result<Self> {
        self.cache_config = config.clone();
        self.cache = Some(SqliteCache::open(config)?);
        Ok(self)
    }

    /// Enable caching with default configuration
    pub fn with_default_cache(self) -> Result<Self> {
        self.with_cache(CacheConfig::default())
    }

    /// Get cache statistics (if caching is enabled)
    pub fn cache_stats(&self) -> Option<super::cache::CacheStats> {
        self.cache.as_ref().and_then(|c| c.stats().ok())
    }

    /// Compile all Python files in the input directory
    pub async fn compile_all(&self) -> Result<Vec<CompilationResult>> {
        let mut results = Vec::new();

        // Find all Python files
        let python_files = self.find_python_files()?;
        let total = python_files.len();

        // Show initial compilation message
        if matches!(self.display_mode, DisplayMode::Rich) {
            println!("Compiling {} files...", total);
        }

        // Compile each file with progress output
        for (i, py_file) in python_files.iter().enumerate() {
            let result = self.compile_one(py_file).await?;

            // Output progress based on display mode
            self.report_compile_progress(i + 1, total, py_file, &result);

            results.push(result);
        }

        // Clear line and show completion
        if matches!(self.display_mode, DisplayMode::Rich) {
            println!();
        }

        Ok(results)
    }

    /// Report progress during compilation
    fn report_compile_progress(
        &self,
        current: usize,
        total: usize,
        py_file: &Path,
        result: &CompilationResult,
    ) {
        let filename = py_file
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or_default();
        let status = if result.success { "✓" } else { "✗" };

        match self.display_mode {
            DisplayMode::Rich => {
                let bar = progress_bar(current, total, 20);
                let pct = if total > 0 {
                    (current as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                // Use carriage return for in-place update
                print!(
                    "\r[{}] {:3.0}% {} {:40}",
                    bar,
                    pct,
                    status,
                    truncate_filename(&filename, 40)
                );
                let _ = std::io::stdout().flush();
            }
            DisplayMode::Minimal => {
                // Only output on completion or every 10%
                if current == total || (current * 10 / total) > ((current - 1) * 10 / total) {
                    println!(
                        "[{}/{}] Compiling... {}% complete",
                        current,
                        total,
                        current * 100 / total.max(1)
                    );
                }
            }
            DisplayMode::Json | DisplayMode::Silent => {}
        }
    }

    /// Find all Python files in input directory
    fn find_python_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if self.input_dir.is_file() {
            if self.input_dir.extension().is_some_and(|e| e == "py") {
                files.push(self.input_dir.clone());
            }
        } else if self.input_dir.is_dir() {
            self.find_python_files_recursive(&self.input_dir, &mut files)?;
        }

        Ok(files)
    }

    /// Recursively find Python files
    fn find_python_files_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Skip __pycache__ and hidden directories
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if !name.starts_with('.') && name != "__pycache__" {
                    self.find_python_files_recursive(&path, files)?;
                }
            } else if path.extension().is_some_and(|e| e == "py") {
                files.push(path);
            }
        }

        Ok(())
    }

    /// Compile a single Python file
    ///
    /// DEPYLER-CACHE-001: Checks cache first for O(1) lookup.
    async fn compile_one(&self, py_file: &Path) -> Result<CompilationResult> {
        let start = Instant::now();

        // Step 0: Check cache (DEPYLER-CACHE-001)
        if let Some(ref cache) = self.cache {
            if let Some(cached_result) = self.check_cache(cache, py_file)? {
                return Ok(cached_result);
            }
        }

        // Step 1: Transpile Python to Rust
        let transpile_result = self.transpile(py_file).await;

        match transpile_result {
            Ok(rust_file) => {
                // Read the generated Rust code for caching
                let rust_code = std::fs::read_to_string(&rust_file).unwrap_or_default();

                // Step 2: Compile Rust
                let compile_result = self.compile_rust(&rust_file).await;

                let result = match compile_result {
                    Ok(()) => CompilationResult {
                        source_file: py_file.to_path_buf(),
                        success: true,
                        errors: vec![],
                        rust_file: Some(rust_file),
                    },
                    Err(errors) => CompilationResult {
                        source_file: py_file.to_path_buf(),
                        success: false,
                        errors,
                        rust_file: Some(rust_file),
                    },
                };

                // Step 3: Store in cache (DEPYLER-CACHE-001)
                if let Some(ref cache) = self.cache {
                    let elapsed = start.elapsed().as_millis() as u64;
                    let _ = self.store_in_cache(cache, py_file, &rust_code, &result, elapsed);
                }

                Ok(result)
            }
            Err(e) => {
                // Transpilation failed
                let result = CompilationResult {
                    source_file: py_file.to_path_buf(),
                    success: false,
                    errors: vec![CompilationError {
                        code: "TRANSPILE".to_string(),
                        message: e.to_string(),
                        file: py_file.to_path_buf(),
                        line: 0,
                        column: 0,
                    }],
                    rust_file: None,
                };

                // Store failure in cache too
                if let Some(ref cache) = self.cache {
                    let elapsed = start.elapsed().as_millis() as u64;
                    let _ = self.store_in_cache(cache, py_file, "", &result, elapsed);
                }

                Ok(result)
            }
        }
    }

    /// Check cache for a previously compiled result (DEPYLER-CACHE-001)
    fn check_cache(
        &self,
        cache: &SqliteCache,
        py_file: &Path,
    ) -> Result<Option<CompilationResult>> {
        // Read Python source
        let source = std::fs::read_to_string(py_file)?;
        let key = TranspilationCacheKey::compute(&source, &self.cache_config);

        // Lookup in cache
        if let Ok(Some(entry)) = cache.lookup(&key) {
            // Load the cached Rust code
            if let Ok(rust_code) = cache.load_rust_code(&entry) {
                // Write rust code to expected location
                let rust_file = py_file.with_extension("rs");
                std::fs::write(&rust_file, &rust_code)?;

                // Return cached result
                let errors: Vec<CompilationError> = entry
                    .error_messages
                    .iter()
                    .map(|msg| CompilationError {
                        code: "CACHED".to_string(),
                        message: msg.clone(),
                        file: rust_file.clone(),
                        line: 0,
                        column: 0,
                    })
                    .collect();

                return Ok(Some(CompilationResult {
                    source_file: py_file.to_path_buf(),
                    success: entry.status == CompilationStatus::Success,
                    errors,
                    rust_file: Some(rust_file),
                }));
            }
        }

        Ok(None)
    }

    /// Store compilation result in cache (DEPYLER-CACHE-001)
    fn store_in_cache(
        &self,
        cache: &SqliteCache,
        py_file: &Path,
        rust_code: &str,
        result: &CompilationResult,
        elapsed_ms: u64,
    ) -> Result<()> {
        let source = std::fs::read_to_string(py_file)?;
        let key = TranspilationCacheKey::compute(&source, &self.cache_config);

        let entry = CacheEntry {
            rust_code_blob: String::new(), // Will be set by store
            cargo_toml_blob: String::new(),
            dependencies: vec![],
            status: if result.success {
                CompilationStatus::Success
            } else {
                CompilationStatus::Failure
            },
            error_messages: result.errors.iter().map(|e| e.message.clone()).collect(),
            created_at: 0,
            last_accessed_at: 0,
            transpilation_time_ms: elapsed_ms,
        };

        cache.store(&key, rust_code, "", entry)?;
        Ok(())
    }

    /// Transpile Python to Rust
    async fn transpile(&self, py_file: &Path) -> Result<PathBuf> {
        use std::process::Command;

        let output_file = py_file.with_extension("rs");

        let output = Command::new("depyler")
            .arg("transpile")
            .arg(py_file)
            .arg("-o")
            .arg(&output_file)
            .output()?;

        if output.status.success() {
            Ok(output_file)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Transpilation failed: {}", stderr)
        }
    }

    /// Compile Rust code
    ///
    /// Uses cargo build when Cargo.toml exists (to resolve external dependencies),
    /// otherwise falls back to rustc for simple standalone files.
    async fn compile_rust(
        &self,
        rust_file: &Path,
    ) -> std::result::Result<(), Vec<CompilationError>> {
        // Check if Cargo.toml exists in the same directory
        let parent_dir = rust_file.parent().unwrap_or(rust_file);
        let cargo_toml = parent_dir.join("Cargo.toml");

        if cargo_toml.exists() {
            // Use cargo build for projects with dependencies
            self.compile_with_cargo(parent_dir, rust_file).await
        } else {
            // Use rustc for standalone files
            self.compile_with_rustc(rust_file).await
        }
    }

    /// Compile using cargo build (for projects with Cargo.toml)
    async fn compile_with_cargo(
        &self,
        project_dir: &Path,
        rust_file: &Path,
    ) -> std::result::Result<(), Vec<CompilationError>> {
        use std::process::Command;

        let output = Command::new("cargo")
            .arg("build")
            .arg("--message-format=short")
            .current_dir(project_dir)
            .env("RUSTFLAGS", "-D warnings")
            .output()
            .map_err(|e| {
                vec![CompilationError {
                    code: "IO".to_string(),
                    message: e.to_string(),
                    file: rust_file.to_path_buf(),
                    line: 0,
                    column: 0,
                }]
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let errors = self.parse_rustc_errors(&stderr, rust_file);
            Err(errors)
        }
    }

    /// Compile using Cargo-First approach (DEPYLER-CARGO-FIRST)
    ///
    /// Uses ephemeral Cargo workspace for accurate verification with
    /// proper dependency resolution. This eliminates false-positive
    /// "missing crate" errors that plagued bare rustc.
    async fn compile_with_rustc(
        &self,
        rust_file: &Path,
    ) -> std::result::Result<(), Vec<CompilationError>> {
        // Read the Rust source code
        let rust_code = std::fs::read_to_string(rust_file).map_err(|e| {
            vec![CompilationError {
                code: "IO".to_string(),
                message: e.to_string(),
                file: rust_file.to_path_buf(),
                line: 0,
                column: 0,
            }]
        })?;

        // Use Cargo-First compilation strategy
        let name = rust_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("verification_target");

        match cargo_first::compile_with_cargo(name, &rust_code, None) {
            Ok(result) if result.success => Ok(()),
            Ok(result) => {
                // Convert CompilerError to CompilationError
                let errors = result
                    .errors
                    .into_iter()
                    .filter(|e| e.is_semantic) // Only report semantic errors
                    .map(|e| CompilationError {
                        code: e.code.unwrap_or_else(|| "E????".to_string()),
                        message: e.message,
                        file: rust_file.to_path_buf(),
                        line: e.span.as_ref().map(|s| s.line_start as usize).unwrap_or(0),
                        column: e
                            .span
                            .as_ref()
                            .map(|s| s.column_start as usize)
                            .unwrap_or(0),
                    })
                    .collect::<Vec<_>>();

                if errors.is_empty() {
                    // All errors were dependency-related (shouldn't happen with Cargo-First)
                    Err(vec![CompilationError {
                        code: "CARGO".to_string(),
                        message: result.stderr,
                        file: rust_file.to_path_buf(),
                        line: 0,
                        column: 0,
                    }])
                } else {
                    Err(errors)
                }
            }
            Err(e) => Err(vec![CompilationError {
                code: "CARGO".to_string(),
                message: e.to_string(),
                file: rust_file.to_path_buf(),
                line: 0,
                column: 0,
            }]),
        }
    }

    /// Parse rustc error output into structured errors
    fn parse_rustc_errors(&self, stderr: &str, file: &Path) -> Vec<CompilationError> {
        let mut errors = Vec::new();

        // Parse rustc JSON output or text output
        for line in stderr.lines() {
            if let Some(error) = self.parse_error_line(line, file) {
                errors.push(error);
            }
        }

        // If no structured errors found, create a generic one
        if errors.is_empty() && !stderr.is_empty() {
            errors.push(CompilationError {
                code: "UNKNOWN".to_string(),
                message: stderr.to_string(),
                file: file.to_path_buf(),
                line: 0,
                column: 0,
            });
        }

        errors
    }

    /// Parse a single error line
    fn parse_error_line(&self, line: &str, file: &Path) -> Option<CompilationError> {
        // Look for patterns like "error[E0599]:"
        if let Some(start) = line.find("error[E") {
            let code_start = start + 6;
            if let Some(code_end) = line[code_start..].find(']') {
                let code = line[code_start..code_start + code_end].to_string();
                let message = line[code_start + code_end + 2..].trim().to_string();

                return Some(CompilationError {
                    code,
                    message,
                    file: file.to_path_buf(),
                    line: 0,
                    column: 0,
                });
            }
        }

        None
    }
}

/// Truncate filename for display
fn truncate_filename(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("...{}", &s[s.len() - max_len + 3..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_batch_compiler_new() {
        let compiler = BatchCompiler::new(Path::new("/tmp/examples"));
        assert_eq!(compiler.input_dir, PathBuf::from("/tmp/examples"));
    }

    #[test]
    fn test_parse_error_line() {
        let compiler = BatchCompiler::new(Path::new("/tmp"));
        let line = "error[E0599]: no method named `foo` found";
        let file = Path::new("test.rs");

        let error = compiler.parse_error_line(line, file);
        assert!(error.is_some());

        let error = error.unwrap();
        assert_eq!(error.code, "E0599");
        assert!(error.message.contains("no method"));
    }

    #[tokio::test]
    async fn test_find_python_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create some Python files
        std::fs::write(temp_dir.path().join("test1.py"), "print('hello')").unwrap();
        std::fs::write(temp_dir.path().join("test2.py"), "print('world')").unwrap();
        std::fs::write(temp_dir.path().join("not_python.txt"), "text").unwrap();

        let compiler = BatchCompiler::new(temp_dir.path());
        let files = compiler.find_python_files().unwrap();

        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|f| f.extension().unwrap() == "py"));
    }
}
