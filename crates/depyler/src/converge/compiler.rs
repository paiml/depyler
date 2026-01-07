//! Batch compilation and error collection
//!
//! Simplified implementation focused on testable pure functions.

use super::state::DisplayMode;
use anyhow::Result;
use depyler_core::cargo_first;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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
pub struct BatchCompiler {
    /// Directory containing examples
    input_dir: PathBuf,
    /// Number of parallel jobs
    parallel_jobs: usize,
    /// Display mode for progress output
    display_mode: DisplayMode,
}

impl BatchCompiler {
    /// Create a new batch compiler
    pub fn new(input_dir: &Path) -> Self {
        Self {
            input_dir: input_dir.to_path_buf(),
            parallel_jobs: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
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

    /// Compile all Python files in the input directory
    pub async fn compile_all(&self) -> Result<Vec<CompilationResult>> {
        let python_files = find_python_files(&self.input_dir)?;
        let total = python_files.len();
        let mut results = Vec::with_capacity(total);

        if matches!(self.display_mode, DisplayMode::Rich) {
            println!("Compiling {} files...", total);
        }

        for py_file in python_files {
            let result = compile_single_file(&py_file).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Parse rustc error output into structured errors (testable pure function)
    pub fn parse_rustc_errors(&self, stderr: &str, file: &Path) -> Vec<CompilationError> {
        parse_rustc_errors(stderr, file)
    }

    /// Parse a single error line (testable pure function)
    pub fn parse_error_line(&self, line: &str, file: &Path) -> Option<CompilationError> {
        parse_error_line(line, file)
    }
}

// ============================================================================
// Pure Functions (testable)
// ============================================================================

/// Truncate filename for display
pub fn truncate_filename(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("...{}", &s[s.len() - max_len + 3..])
    }
}

/// Parse rustc error output into structured errors
pub fn parse_rustc_errors(stderr: &str, file: &Path) -> Vec<CompilationError> {
    let mut errors = Vec::new();

    for line in stderr.lines() {
        if let Some(error) = parse_error_line(line, file) {
            errors.push(error);
        }
    }

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
pub fn parse_error_line(line: &str, file: &Path) -> Option<CompilationError> {
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

/// Find all Python files in a directory
pub fn find_python_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if dir.is_file() {
        if dir.extension().is_some_and(|e| e == "py") {
            files.push(dir.to_path_buf());
        }
    } else if dir.is_dir() {
        find_python_files_recursive(dir, &mut files)?;
    }

    Ok(files)
}

fn find_python_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if !name.starts_with('.') && name != "__pycache__" {
                find_python_files_recursive(&path, files)?;
            }
        } else if path.extension().is_some_and(|e| e == "py") {
            files.push(path);
        }
    }
    Ok(())
}

/// Create a compilation result for a transpilation failure
pub fn make_transpile_failure(py_file: &Path, error_message: &str) -> CompilationResult {
    CompilationResult {
        source_file: py_file.to_path_buf(),
        success: false,
        errors: vec![CompilationError {
            code: "TRANSPILE".to_string(),
            message: error_message.to_string(),
            file: py_file.to_path_buf(),
            line: 0,
            column: 0,
        }],
        rust_file: None,
    }
}

/// Create a successful compilation result
pub fn make_success_result(py_file: &Path, rust_file: PathBuf) -> CompilationResult {
    CompilationResult {
        source_file: py_file.to_path_buf(),
        success: true,
        errors: vec![],
        rust_file: Some(rust_file),
    }
}

/// Create a compilation failure result
pub fn make_compile_failure(
    py_file: &Path,
    rust_file: PathBuf,
    errors: Vec<CompilationError>,
) -> CompilationResult {
    CompilationResult {
        source_file: py_file.to_path_buf(),
        success: false,
        errors,
        rust_file: Some(rust_file),
    }
}

/// Convert cargo-first errors to CompilationErrors
pub fn convert_cargo_errors(
    cargo_errors: Vec<depyler_core::cargo_first::CompilerError>,
    rust_file: &Path,
) -> Vec<CompilationError> {
    cargo_errors
        .into_iter()
        .filter(|e| e.is_semantic)
        .map(|e| {
            let (line, column) = match &e.span {
                Some(s) => (s.line_start as usize, s.column_start as usize),
                None => (0, 0),
            };
            CompilationError {
                code: e.code.unwrap_or_else(|| "E????".to_string()),
                message: e.message,
                file: rust_file.to_path_buf(),
                line,
                column,
            }
        })
        .collect()
}

/// Compile a single file using cargo-first
pub async fn compile_single_file(py_file: &Path) -> Result<CompilationResult> {
    use std::process::Command;

    let output_file = py_file.with_extension("rs");

    let output = Command::new("depyler")
        .arg("transpile")
        .arg(py_file)
        .arg("-o")
        .arg(&output_file)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Ok(make_transpile_failure(py_file, &stderr));
    }

    let rust_code = std::fs::read_to_string(&output_file).unwrap_or_default();
    let name = py_file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("verification_target");

    match cargo_first::compile_with_cargo(name, &rust_code, None) {
        Ok(result) if result.success => Ok(make_success_result(py_file, output_file)),
        Ok(result) => {
            let errors = convert_cargo_errors(result.errors, &output_file);
            Ok(make_compile_failure(py_file, output_file, errors))
        }
        Err(e) => Ok(CompilationResult {
            source_file: py_file.to_path_buf(),
            success: false,
            errors: vec![CompilationError {
                code: "CARGO".to_string(),
                message: e.to_string(),
                file: py_file.to_path_buf(),
                line: 0,
                column: 0,
            }],
            rust_file: None,
        }),
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
    fn test_parse_error_line_basic() {
        let line = "error[E0599]: no method named `foo` found";
        let file = Path::new("test.rs");
        let error = parse_error_line(line, file);
        assert!(error.is_some());
        let error = error.unwrap();
        assert_eq!(error.code, "E0599");
        assert!(error.message.contains("no method"));
    }

    #[test]
    fn test_find_python_files_in_dir() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("test1.py"), "print('hello')").unwrap();
        std::fs::write(temp_dir.path().join("test2.py"), "print('world')").unwrap();
        std::fs::write(temp_dir.path().join("not_python.txt"), "text").unwrap();

        let files = find_python_files(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|f| f.extension().unwrap() == "py"));
    }

    #[test]
    fn test_truncate_filename_short() {
        let result = truncate_filename("test.rs", 20);
        assert!(result.len() >= 8);
        assert!(result.starts_with("test.rs"));
    }

    #[test]
    fn test_truncate_filename_exact() {
        let result = truncate_filename("test.rs", 7);
        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_truncate_filename_long() {
        let result = truncate_filename("very_long_filename_that_exceeds_limit.rs", 20);
        assert!(result.starts_with("..."));
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn test_truncate_filename_empty() {
        let result = truncate_filename("", 10);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_with_display_mode() {
        let compiler = BatchCompiler::new(Path::new("/tmp"))
            .with_display_mode(DisplayMode::Minimal);
        assert!(matches!(compiler.display_mode, DisplayMode::Minimal));
    }

    #[test]
    fn test_with_display_mode_json() {
        let compiler = BatchCompiler::new(Path::new("/tmp"))
            .with_display_mode(DisplayMode::Json);
        assert!(matches!(compiler.display_mode, DisplayMode::Json));
    }

    #[test]
    fn test_with_display_mode_silent() {
        let compiler = BatchCompiler::new(Path::new("/tmp"))
            .with_display_mode(DisplayMode::Silent);
        assert!(matches!(compiler.display_mode, DisplayMode::Silent));
    }

    #[test]
    fn test_with_display_mode_rich() {
        let compiler = BatchCompiler::new(Path::new("/tmp"))
            .with_display_mode(DisplayMode::Rich);
        assert!(matches!(compiler.display_mode, DisplayMode::Rich));
    }

    #[test]
    fn test_with_parallel_jobs() {
        let compiler = BatchCompiler::new(Path::new("/tmp"))
            .with_parallel_jobs(8);
        assert_eq!(compiler.parallel_jobs, 8);
    }

    #[test]
    fn test_with_parallel_jobs_single() {
        let compiler = BatchCompiler::new(Path::new("/tmp"))
            .with_parallel_jobs(1);
        assert_eq!(compiler.parallel_jobs, 1);
    }

    #[test]
    fn test_with_parallel_jobs_large() {
        let compiler = BatchCompiler::new(Path::new("/tmp"))
            .with_parallel_jobs(256);
        assert_eq!(compiler.parallel_jobs, 256);
    }

    #[test]
    fn test_compilation_error_struct() {
        let error = CompilationError {
            code: "E0599".to_string(),
            message: "no method found".to_string(),
            file: PathBuf::from("test.rs"),
            line: 10,
            column: 5,
        };
        assert_eq!(error.code, "E0599");
        assert_eq!(error.message, "no method found");
        assert_eq!(error.file, PathBuf::from("test.rs"));
        assert_eq!(error.line, 10);
        assert_eq!(error.column, 5);
    }

    #[test]
    fn test_compilation_error_clone() {
        let error = CompilationError {
            code: "E0308".to_string(),
            message: "mismatched types".to_string(),
            file: PathBuf::from("main.rs"),
            line: 20,
            column: 10,
        };
        let cloned = error.clone();
        assert_eq!(error.code, cloned.code);
        assert_eq!(error.message, cloned.message);
        assert_eq!(error.file, cloned.file);
    }

    #[test]
    fn test_compilation_error_debug() {
        let error = CompilationError {
            code: "E0599".to_string(),
            message: "test".to_string(),
            file: PathBuf::from("test.rs"),
            line: 1,
            column: 1,
        };
        let debug = format!("{:?}", error);
        assert!(debug.contains("E0599"));
        assert!(debug.contains("test.rs"));
    }

    #[test]
    fn test_compilation_result_struct() {
        let result = CompilationResult {
            source_file: PathBuf::from("test.py"),
            success: true,
            errors: vec![],
            rust_file: Some(PathBuf::from("test.rs")),
        };
        assert!(result.success);
        assert!(result.errors.is_empty());
        assert!(result.rust_file.is_some());
    }

    #[test]
    fn test_compilation_result_failure() {
        let result = CompilationResult {
            source_file: PathBuf::from("test.py"),
            success: false,
            errors: vec![CompilationError {
                code: "E0599".to_string(),
                message: "error".to_string(),
                file: PathBuf::from("test.rs"),
                line: 1,
                column: 1,
            }],
            rust_file: None,
        };
        assert!(!result.success);
        assert_eq!(result.errors.len(), 1);
        assert!(result.rust_file.is_none());
    }

    #[test]
    fn test_compilation_result_clone() {
        let result = CompilationResult {
            source_file: PathBuf::from("test.py"),
            success: true,
            errors: vec![],
            rust_file: Some(PathBuf::from("test.rs")),
        };
        let cloned = result.clone();
        assert_eq!(result.source_file, cloned.source_file);
        assert_eq!(result.success, cloned.success);
    }

    #[test]
    fn test_parse_error_line_no_error() {
        let line = "warning: unused variable";
        let file = Path::new("test.rs");
        let error = parse_error_line(line, file);
        assert!(error.is_none());
    }

    #[test]
    fn test_parse_error_line_various_codes() {
        let file = Path::new("test.rs");
        let test_cases = [
            ("error[E0308]: mismatched types", "E0308"),
            ("error[E0382]: use of moved value", "E0382"),
            ("error[E0425]: cannot find value", "E0425"),
            ("error[E0277]: trait not implemented", "E0277"),
        ];

        for (line, expected_code) in test_cases {
            let error = parse_error_line(line, file);
            assert!(error.is_some(), "Should parse: {}", line);
            assert_eq!(error.unwrap().code, expected_code);
        }
    }

    #[test]
    fn test_parse_rustc_errors_empty() {
        let file = Path::new("test.rs");
        let errors = parse_rustc_errors("", file);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_parse_rustc_errors_unparseable() {
        let file = Path::new("test.rs");
        let stderr = "Some random error message without error code";
        let errors = parse_rustc_errors(stderr, file);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, "UNKNOWN");
    }

    #[test]
    fn test_parse_rustc_errors_multiple() {
        let file = Path::new("test.rs");
        let stderr = "error[E0599]: no method\nerror[E0308]: mismatched types";
        let errors = parse_rustc_errors(stderr, file);
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].code, "E0599");
        assert_eq!(errors[1].code, "E0308");
    }

    #[test]
    fn test_find_python_files_nested() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        std::fs::create_dir_all(&sub_dir).unwrap();

        std::fs::write(temp_dir.path().join("test1.py"), "print('hello')").unwrap();
        std::fs::write(sub_dir.join("test2.py"), "print('world')").unwrap();

        let files = find_python_files(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_find_python_files_skip_pycache() {
        let temp_dir = TempDir::new().unwrap();
        let pycache = temp_dir.path().join("__pycache__");
        std::fs::create_dir_all(&pycache).unwrap();

        std::fs::write(temp_dir.path().join("test.py"), "print('hello')").unwrap();
        std::fs::write(pycache.join("cached.py"), "cached").unwrap();

        let files = find_python_files(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].file_name().unwrap().to_str().unwrap() == "test.py");
    }

    #[test]
    fn test_find_python_files_skip_hidden() {
        let temp_dir = TempDir::new().unwrap();
        let hidden = temp_dir.path().join(".hidden");
        std::fs::create_dir_all(&hidden).unwrap();

        std::fs::write(temp_dir.path().join("test.py"), "print('hello')").unwrap();
        std::fs::write(hidden.join("hidden.py"), "hidden").unwrap();

        let files = find_python_files(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_find_python_files_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let py_file = temp_dir.path().join("single.py");
        std::fs::write(&py_file, "print('single')").unwrap();

        let files = find_python_files(&py_file).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], py_file);
    }

    #[test]
    fn test_find_python_files_non_python_file() {
        let temp_dir = TempDir::new().unwrap();
        let txt_file = temp_dir.path().join("test.txt");
        std::fs::write(&txt_file, "not python").unwrap();

        let files = find_python_files(&txt_file).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_batch_compiler_default_parallel_jobs() {
        let compiler = BatchCompiler::new(Path::new("/tmp"));
        assert!(compiler.parallel_jobs > 0);
    }

    #[test]
    fn test_batch_compiler_chained_builders() {
        let compiler = BatchCompiler::new(Path::new("/tmp"))
            .with_parallel_jobs(4)
            .with_display_mode(DisplayMode::Minimal);

        assert_eq!(compiler.parallel_jobs, 4);
        assert!(matches!(compiler.display_mode, DisplayMode::Minimal));
    }

    #[test]
    fn test_compilation_error_serialization() {
        let error = CompilationError {
            code: "E0599".to_string(),
            message: "no method found".to_string(),
            file: PathBuf::from("test.rs"),
            line: 10,
            column: 5,
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("E0599"));
        assert!(json.contains("no method found"));

        let deserialized: CompilationError = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.code, error.code);
        assert_eq!(deserialized.message, error.message);
    }

    #[test]
    fn test_truncate_filename_unicode() {
        let result = truncate_filename("файл.py", 20);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_truncate_filename_very_short_limit() {
        let result = truncate_filename("verylongfilename.py", 5);
        assert!(result.len() == 5);
    }

    #[test]
    fn test_make_transpile_failure() {
        let py_file = Path::new("test.py");
        let result = make_transpile_failure(py_file, "Failed to parse");
        assert!(!result.success);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code, "TRANSPILE");
        assert!(result.errors[0].message.contains("Failed to parse"));
        assert!(result.rust_file.is_none());
    }

    #[test]
    fn test_make_success_result() {
        let py_file = Path::new("test.py");
        let rs_file = PathBuf::from("test.rs");
        let result = make_success_result(py_file, rs_file.clone());
        assert!(result.success);
        assert!(result.errors.is_empty());
        assert_eq!(result.rust_file, Some(rs_file));
    }

    #[test]
    fn test_make_compile_failure() {
        let py_file = Path::new("test.py");
        let rs_file = PathBuf::from("test.rs");
        let errors = vec![CompilationError {
            code: "E0599".to_string(),
            message: "no method found".to_string(),
            file: rs_file.clone(),
            line: 10,
            column: 5,
        }];
        let result = make_compile_failure(py_file, rs_file.clone(), errors);
        assert!(!result.success);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.rust_file, Some(rs_file));
    }

    #[test]
    fn test_make_compile_failure_empty_errors() {
        let py_file = Path::new("test.py");
        let rs_file = PathBuf::from("test.rs");
        let result = make_compile_failure(py_file, rs_file.clone(), vec![]);
        assert!(!result.success);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_convert_cargo_errors_empty() {
        let rust_file = Path::new("test.rs");
        let result = convert_cargo_errors(vec![], rust_file);
        assert!(result.is_empty());
    }

    #[test]
    fn test_convert_cargo_errors_filters_non_semantic() {
        use depyler_core::cargo_first::CompilerError;
        let rust_file = Path::new("test.rs");
        let cargo_errors = vec![
            CompilerError {
                message: "semantic error".to_string(),
                code: Some("E0599".to_string()),
                span: None,
                is_semantic: true,
            },
            CompilerError {
                message: "non-semantic error".to_string(),
                code: Some("E0000".to_string()),
                span: None,
                is_semantic: false,
            },
        ];
        let result = convert_cargo_errors(cargo_errors, rust_file);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "E0599");
    }

    #[test]
    fn test_convert_cargo_errors_with_span() {
        use depyler_core::cargo_first::{CompilerError, ErrorSpan};
        let rust_file = Path::new("test.rs");
        let cargo_errors = vec![CompilerError {
            message: "error with span".to_string(),
            code: Some("E0308".to_string()),
            span: Some(ErrorSpan {
                file: "test.rs".to_string(),
                line_start: 10,
                line_end: 10,
                column_start: 5,
                column_end: 10,
            }),
            is_semantic: true,
        }];
        let result = convert_cargo_errors(cargo_errors, rust_file);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].line, 10);
        assert_eq!(result[0].column, 5);
    }

    #[test]
    fn test_convert_cargo_errors_no_code() {
        use depyler_core::cargo_first::CompilerError;
        let rust_file = Path::new("test.rs");
        let cargo_errors = vec![CompilerError {
            message: "error without code".to_string(),
            code: None,
            span: None,
            is_semantic: true,
        }];
        let result = convert_cargo_errors(cargo_errors, rust_file);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "E????");
    }
}
