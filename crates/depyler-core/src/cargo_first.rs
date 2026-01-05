//! Cargo-First Compilation Strategy (DEPYLER-CARGO-FIRST)
//!
//! Implements the "Jidoka" approach to verification: instead of running bare `rustc`,
//! we create ephemeral Cargo workspaces with proper dependency resolution.
//!
//! This eliminates false-positive compilation failures from missing external crates
//! (E0432 errors), allowing Hunt Mode to focus on true semantic defects.
//!
//! # Toyota Way Principles
//! - **Jidoka**: Automatically provide necessary resources (dependencies)
//! - **Poka-Yoke**: Fail-safe compilation that can't miss dependencies
//! - **Heijunka**: Standardized build environment for all outputs

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

/// Result of a Cargo check operation
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// Whether compilation succeeded
    pub success: bool,
    /// Compiler errors (if any)
    pub errors: Vec<CompilerError>,
    /// Compiler warnings
    pub warnings: Vec<CompilerWarning>,
    /// Raw stderr output
    pub stderr: String,
}

/// A structured compiler error from cargo check --message-format=json
#[derive(Debug, Clone)]
pub struct CompilerError {
    /// Error code (e.g., "E0308")
    pub code: Option<String>,
    /// Error message
    pub message: String,
    /// Primary span location
    pub span: Option<ErrorSpan>,
    /// Is this a true semantic error vs dependency error?
    pub is_semantic: bool,
}

/// A compiler warning
#[derive(Debug, Clone)]
pub struct CompilerWarning {
    pub code: Option<String>,
    pub message: String,
}

/// Source location of an error
#[derive(Debug, Clone)]
pub struct ErrorSpan {
    pub file: String,
    pub line_start: u32,
    pub line_end: u32,
    pub column_start: u32,
    pub column_end: u32,
}

/// Ephemeral Cargo workspace for compilation verification
///
/// Creates a temporary directory with:
/// - `Cargo.toml` with detected dependencies
/// - `src/lib.rs` with the Rust code to verify
///
/// # Example
/// ```ignore
/// let workspace = EphemeralWorkspace::new("my_module", rust_code, &deps)?;
/// let result = workspace.check()?;
/// if result.success {
///     println!("Code compiles!");
/// }
/// ```
pub struct EphemeralWorkspace {
    /// Temporary directory (auto-cleaned on drop)
    dir: TempDir,
    /// Module name
    #[allow(dead_code)]
    name: String,
    /// Path to the generated lib.rs
    lib_path: PathBuf,
}

impl EphemeralWorkspace {
    /// Create a new ephemeral workspace with the given Rust code
    ///
    /// # Arguments
    /// * `name` - Module name (used in Cargo.toml)
    /// * `rust_code` - The Rust source code to verify
    /// * `cargo_toml` - Pre-generated Cargo.toml content
    ///
    /// # Jidoka Principle
    /// Automatically sets up the build environment with all necessary dependencies,
    /// eliminating manual configuration errors.
    pub fn new(name: &str, rust_code: &str, cargo_toml: &str) -> Result<Self> {
        let dir = TempDir::new().context("Failed to create temp directory")?;
        let dir_path = dir.path();

        // Create src directory
        let src_dir = dir_path.join("src");
        std::fs::create_dir_all(&src_dir).context("Failed to create src directory")?;

        // Write Cargo.toml
        let cargo_path = dir_path.join("Cargo.toml");
        std::fs::write(&cargo_path, cargo_toml).context("Failed to write Cargo.toml")?;

        // Write src/lib.rs
        let lib_path = src_dir.join("lib.rs");
        std::fs::write(&lib_path, rust_code).context("Failed to write lib.rs")?;

        Ok(Self {
            dir,
            name: name.to_string(),
            lib_path,
        })
    }

    /// Get the path to the workspace directory
    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    /// Get the path to lib.rs
    pub fn lib_path(&self) -> &Path {
        &self.lib_path
    }

    /// Run `cargo check` and parse the results
    ///
    /// # Poka-Yoke Principle
    /// Uses `--message-format=json` for structured error parsing,
    /// making it impossible to miss or misinterpret compiler messages.
    ///
    /// # Coverage Mode Compatibility
    /// Clears LLVM coverage environment variables to prevent interference
    /// when running under cargo-llvm-cov. These vars cause compilation
    /// issues in spawned cargo subprocesses.
    pub fn check(&self) -> Result<CheckResult> {
        let output = Command::new("cargo")
            .arg("check")
            .arg("--message-format=json")
            .current_dir(self.dir.path())
            // Clear LLVM coverage environment to prevent interference with sub-cargo
            .env_remove("CARGO_LLVM_COV")
            .env_remove("CARGO_LLVM_COV_SHOW_ENV")
            .env_remove("CARGO_LLVM_COV_TARGET_DIR")
            .env_remove("LLVM_PROFILE_FILE")
            .env_remove("RUSTFLAGS")
            .env_remove("CARGO_INCREMENTAL")
            .env_remove("CARGO_BUILD_JOBS")
            .env_remove("CARGO_TARGET_DIR")
            .output()
            .context("Failed to run cargo check")?;

        self.parse_cargo_output(output)
    }

    /// Parse cargo check JSON output into structured results
    fn parse_cargo_output(&self, output: Output) -> Result<CheckResult> {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Parse JSON messages from stdout
        for line in stdout.lines() {
            if let Ok(msg) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(reason) = msg.get("reason").and_then(|r| r.as_str()) {
                    if reason == "compiler-message" {
                        if let Some(message) = msg.get("message") {
                            self.parse_compiler_message(message, &mut errors, &mut warnings);
                        }
                    }
                }
            }
        }

        Ok(CheckResult {
            success: output.status.success(),
            errors,
            warnings,
            stderr,
        })
    }

    /// Parse a single compiler message
    fn parse_compiler_message(
        &self,
        message: &serde_json::Value,
        errors: &mut Vec<CompilerError>,
        warnings: &mut Vec<CompilerWarning>,
    ) {
        let level = message.get("level").and_then(|l| l.as_str()).unwrap_or("");
        let msg_text = message
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .to_string();
        let code = message
            .get("code")
            .and_then(|c| c.get("code"))
            .and_then(|c| c.as_str())
            .map(|s| s.to_string());

        // Parse span if available
        let span = message
            .get("spans")
            .and_then(|s| s.as_array())
            .and_then(|arr| arr.first())
            .map(|s| ErrorSpan {
                file: s
                    .get("file_name")
                    .and_then(|f| f.as_str())
                    .unwrap_or("")
                    .to_string(),
                line_start: s.get("line_start").and_then(|l| l.as_u64()).unwrap_or(0) as u32,
                line_end: s.get("line_end").and_then(|l| l.as_u64()).unwrap_or(0) as u32,
                column_start: s
                    .get("column_start")
                    .and_then(|c| c.as_u64())
                    .unwrap_or(0) as u32,
                column_end: s.get("column_end").and_then(|c| c.as_u64()).unwrap_or(0) as u32,
            });

        match level {
            "error" => {
                // Determine if this is a semantic error vs dependency error
                let is_semantic = !Self::is_dependency_error(&code, &msg_text);
                errors.push(CompilerError {
                    code,
                    message: msg_text,
                    span,
                    is_semantic,
                });
            }
            "warning" => {
                warnings.push(CompilerWarning {
                    code,
                    message: msg_text,
                });
            }
            _ => {}
        }
    }

    /// Check if an error is a dependency-related error (not a true semantic issue)
    ///
    /// These errors are automatically resolved by Cargo-First approach:
    /// - E0432: unresolved import (missing crate)
    /// - E0433: failed to resolve (missing crate path)
    fn is_dependency_error(code: &Option<String>, message: &str) -> bool {
        match code.as_deref() {
            Some("E0432") => true, // unresolved import
            Some("E0433") => true, // failed to resolve
            Some("E0463") => true, // can't find crate
            _ => {
                // Also check message content for dependency hints
                message.contains("can't find crate")
                    || message.contains("unresolved import")
                    || message.contains("could not find")
            }
        }
    }
}

/// Compile Rust code using Cargo-First approach
///
/// This is the main entry point for verifying transpiled Rust code.
/// It automatically:
/// 1. Detects dependencies from `use` statements
/// 2. Generates an appropriate Cargo.toml
/// 3. Creates an ephemeral workspace
/// 4. Runs `cargo check` and returns structured results
///
/// # Arguments
/// * `name` - Module name for the workspace
/// * `rust_code` - The Rust source code to verify
/// * `cargo_toml` - Optional pre-generated Cargo.toml (uses minimal if None)
///
/// # Returns
/// Result with CheckResult containing success status and any errors
pub fn compile_with_cargo(
    name: &str,
    rust_code: &str,
    cargo_toml: Option<&str>,
) -> Result<CheckResult> {
    // Generate comprehensive Cargo.toml with common dependencies
    // that are used by the Python->Rust module mappings
    let default_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
# Serialization (json, pickle modules)
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"

# Regex (re module)
regex = "1.10"

# Random (random module)
rand = "0.8"

# Collections (itertools module)
itertools = "0.12"

# Date/time (datetime module)
chrono = "0.4"

# Async runtime (asyncio module)
tokio = {{ version = "1.0", features = ["full"] }}

# Lazy static (constants)
once_cell = "1.19"

# Testing
quickcheck = "1.0"

# Hashing (hashlib module)
sha2 = "0.10"
md-5 = "0.10"
hex = "0.4"

# Base64 encoding
base64 = "0.22"

# URL parsing (urllib module)
url = "2.5"

# Temp files
tempfile = "3.10"

# Argument parsing (argparse module)
clap = {{ version = "4.5", features = ["derive"] }}
"#,
        name.replace('-', "_")
    );

    let toml = cargo_toml.unwrap_or(&default_toml);
    let workspace = EphemeralWorkspace::new(name, rust_code, toml)?;
    workspace.check()
}

/// Quick check if Rust code compiles (returns Ok/Err)
///
/// Convenience wrapper that returns a simple Result<(), String>
/// for integration with existing verification flows.
pub fn quick_check(name: &str, rust_code: &str, cargo_toml: Option<&str>) -> Result<(), String> {
    match compile_with_cargo(name, rust_code, cargo_toml) {
        Ok(result) if result.success => Ok(()),
        Ok(result) => {
            let error_msg = result
                .errors
                .iter()
                .filter(|e| e.is_semantic) // Only report semantic errors
                .map(|e| format!("{}: {}", e.code.as_deref().unwrap_or("E????"), e.message))
                .collect::<Vec<_>>()
                .join("\n");
            if error_msg.is_empty() {
                // All errors were dependency-related (shouldn't happen with proper Cargo.toml)
                Err(result.stderr)
            } else {
                Err(error_msg)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === CheckResult tests ===

    #[test]
    fn test_check_result_success_fields() {
        let result = CheckResult {
            success: true,
            errors: vec![],
            warnings: vec![],
            stderr: String::new(),
        };
        assert!(result.success);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn test_check_result_with_errors() {
        let result = CheckResult {
            success: false,
            errors: vec![CompilerError {
                code: Some("E0308".to_string()),
                message: "mismatched types".to_string(),
                span: None,
                is_semantic: true,
            }],
            warnings: vec![],
            stderr: "error: mismatched types".to_string(),
        };
        assert!(!result.success);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code.as_deref(), Some("E0308"));
    }

    #[test]
    fn test_check_result_with_warnings() {
        let result = CheckResult {
            success: true,
            errors: vec![],
            warnings: vec![CompilerWarning {
                code: Some("unused_variables".to_string()),
                message: "unused variable `x`".to_string(),
            }],
            stderr: String::new(),
        };
        assert!(result.success);
        assert_eq!(result.warnings.len(), 1);
        assert!(result.warnings[0].message.contains("unused"));
    }

    #[test]
    fn test_check_result_clone() {
        let result = CheckResult {
            success: true,
            errors: vec![],
            warnings: vec![],
            stderr: "test".to_string(),
        };
        let cloned = result.clone();
        assert_eq!(cloned.success, result.success);
        assert_eq!(cloned.stderr, result.stderr);
    }

    #[test]
    fn test_check_result_debug() {
        let result = CheckResult {
            success: true,
            errors: vec![],
            warnings: vec![],
            stderr: String::new(),
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("CheckResult"));
        assert!(debug.contains("success"));
    }

    // === CompilerError tests ===

    #[test]
    fn test_compiler_error_all_fields() {
        let error = CompilerError {
            code: Some("E0308".to_string()),
            message: "mismatched types".to_string(),
            span: Some(ErrorSpan {
                file: "lib.rs".to_string(),
                line_start: 10,
                line_end: 10,
                column_start: 5,
                column_end: 15,
            }),
            is_semantic: true,
        };
        assert_eq!(error.code.as_deref(), Some("E0308"));
        assert!(error.message.contains("mismatched"));
        assert!(error.span.is_some());
        assert!(error.is_semantic);
    }

    #[test]
    fn test_compiler_error_no_code() {
        let error = CompilerError {
            code: None,
            message: "some error".to_string(),
            span: None,
            is_semantic: false,
        };
        assert!(error.code.is_none());
        assert!(!error.is_semantic);
    }

    #[test]
    fn test_compiler_error_clone() {
        let error = CompilerError {
            code: Some("E0001".to_string()),
            message: "test".to_string(),
            span: None,
            is_semantic: true,
        };
        let cloned = error.clone();
        assert_eq!(cloned.code, error.code);
        assert_eq!(cloned.message, error.message);
    }

    #[test]
    fn test_compiler_error_debug() {
        let error = CompilerError {
            code: Some("E0001".to_string()),
            message: "test".to_string(),
            span: None,
            is_semantic: true,
        };
        let debug = format!("{:?}", error);
        assert!(debug.contains("CompilerError"));
        assert!(debug.contains("E0001"));
    }

    // === CompilerWarning tests ===

    #[test]
    fn test_compiler_warning_fields() {
        let warning = CompilerWarning {
            code: Some("unused_variables".to_string()),
            message: "unused variable: `x`".to_string(),
        };
        assert_eq!(warning.code.as_deref(), Some("unused_variables"));
        assert!(warning.message.contains("unused"));
    }

    #[test]
    fn test_compiler_warning_no_code() {
        let warning = CompilerWarning {
            code: None,
            message: "warning without code".to_string(),
        };
        assert!(warning.code.is_none());
    }

    #[test]
    fn test_compiler_warning_clone() {
        let warning = CompilerWarning {
            code: Some("dead_code".to_string()),
            message: "unused".to_string(),
        };
        let cloned = warning.clone();
        assert_eq!(cloned.code, warning.code);
        assert_eq!(cloned.message, warning.message);
    }

    #[test]
    fn test_compiler_warning_debug() {
        let warning = CompilerWarning {
            code: Some("test".to_string()),
            message: "msg".to_string(),
        };
        let debug = format!("{:?}", warning);
        assert!(debug.contains("CompilerWarning"));
    }

    // === ErrorSpan tests ===

    #[test]
    fn test_error_span_fields() {
        let span = ErrorSpan {
            file: "src/main.rs".to_string(),
            line_start: 10,
            line_end: 15,
            column_start: 1,
            column_end: 20,
        };
        assert_eq!(span.file, "src/main.rs");
        assert_eq!(span.line_start, 10);
        assert_eq!(span.line_end, 15);
        assert_eq!(span.column_start, 1);
        assert_eq!(span.column_end, 20);
    }

    #[test]
    fn test_error_span_single_line() {
        let span = ErrorSpan {
            file: "lib.rs".to_string(),
            line_start: 5,
            line_end: 5,
            column_start: 10,
            column_end: 25,
        };
        assert_eq!(span.line_start, span.line_end);
    }

    #[test]
    fn test_error_span_clone() {
        let span = ErrorSpan {
            file: "test.rs".to_string(),
            line_start: 1,
            line_end: 2,
            column_start: 3,
            column_end: 4,
        };
        let cloned = span.clone();
        assert_eq!(cloned.file, span.file);
        assert_eq!(cloned.line_start, span.line_start);
    }

    #[test]
    fn test_error_span_debug() {
        let span = ErrorSpan {
            file: "test.rs".to_string(),
            line_start: 1,
            line_end: 1,
            column_start: 1,
            column_end: 10,
        };
        let debug = format!("{:?}", span);
        assert!(debug.contains("ErrorSpan"));
        assert!(debug.contains("test.rs"));
    }

    // === EphemeralWorkspace tests ===

    #[test]
    fn test_ephemeral_workspace_creates_files() {
        let rust_code = r#"
            pub fn hello() -> &'static str {
                "Hello, world!"
            }
        "#;
        let cargo_toml = r#"
            [package]
            name = "test_module"
            version = "0.1.0"
            edition = "2021"
        "#;

        let workspace = EphemeralWorkspace::new("test_module", rust_code, cargo_toml).unwrap();

        // Verify files were created
        assert!(workspace.path().join("Cargo.toml").exists());
        assert!(workspace.path().join("src/lib.rs").exists());

        // Verify content
        let lib_content = std::fs::read_to_string(workspace.lib_path()).unwrap();
        assert!(lib_content.contains("hello"));
    }

    #[test]
    fn test_ephemeral_workspace_path_accessors() {
        let rust_code = "pub fn test() {}";
        let cargo_toml = r#"
[package]
name = "test_paths"
version = "0.1.0"
edition = "2021"
"#;
        let workspace = EphemeralWorkspace::new("test_paths", rust_code, cargo_toml).unwrap();

        // path() returns temp dir
        assert!(workspace.path().exists());
        assert!(workspace.path().is_dir());

        // lib_path() returns lib.rs path
        assert!(workspace.lib_path().exists());
        assert!(workspace.lib_path().ends_with("lib.rs"));
    }

    #[test]
    fn test_ephemeral_workspace_with_special_name() {
        let rust_code = "pub fn test() {}";
        let cargo_toml = r#"
[package]
name = "test_special_name"
version = "0.1.0"
edition = "2021"
"#;
        let workspace =
            EphemeralWorkspace::new("test-special-name", rust_code, cargo_toml).unwrap();
        assert!(workspace.path().exists());
    }

    #[test]
    fn test_valid_code_compiles() {
        let rust_code = r#"
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        let cargo_toml = r#"
[package]
name = "test_valid"
version = "0.1.0"
edition = "2021"
"#;

        let workspace = EphemeralWorkspace::new("test_valid", rust_code, cargo_toml).unwrap();
        let result = workspace.check().unwrap();

        assert!(
            result.success,
            "Valid code should compile: {:?}",
            result.errors
        );
        assert!(result.errors.is_empty(), "Should have no errors");
    }

    #[test]
    fn test_invalid_code_fails() {
        let rust_code = r#"
            pub fn broken() -> i32 {
                "not an integer"  // Type mismatch
            }
        "#;
        let cargo_toml = r#"
[package]
name = "test_invalid"
version = "0.1.0"
edition = "2021"
"#;

        let workspace = EphemeralWorkspace::new("test_invalid", rust_code, cargo_toml).unwrap();
        let result = workspace.check().unwrap();

        assert!(!result.success, "Invalid code should fail");
        assert!(!result.errors.is_empty(), "Should have errors");
        // This is a semantic error, not dependency error
        assert!(
            result.errors.iter().any(|e| e.is_semantic),
            "Should be semantic error"
        );
    }

    #[test]
    fn test_with_serde_dependency() {
        let rust_code = r#"
            use serde::{Serialize, Deserialize};

            #[derive(Serialize, Deserialize)]
            pub struct Point {
                pub x: f64,
                pub y: f64,
            }
        "#;
        let cargo_toml = r#"
[package]
name = "test_serde"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
"#;

        let workspace = EphemeralWorkspace::new("test_serde", rust_code, cargo_toml).unwrap();
        let result = workspace.check().unwrap();

        assert!(
            result.success,
            "Code with serde should compile: {:?}",
            result.errors
        );
    }

    // === is_dependency_error tests ===

    #[test]
    fn test_is_dependency_error() {
        // E0432 is dependency error
        assert!(EphemeralWorkspace::is_dependency_error(
            &Some("E0432".to_string()),
            "unresolved import"
        ));

        // E0463 is dependency error
        assert!(EphemeralWorkspace::is_dependency_error(
            &Some("E0463".to_string()),
            "can't find crate"
        ));

        // E0308 (type mismatch) is NOT dependency error
        assert!(!EphemeralWorkspace::is_dependency_error(
            &Some("E0308".to_string()),
            "mismatched types"
        ));

        // E0599 (method not found) is NOT dependency error
        assert!(!EphemeralWorkspace::is_dependency_error(
            &Some("E0599".to_string()),
            "method not found"
        ));
    }

    #[test]
    fn test_is_dependency_error_e0433() {
        // E0433 - failed to resolve
        assert!(EphemeralWorkspace::is_dependency_error(
            &Some("E0433".to_string()),
            "failed to resolve"
        ));
    }

    #[test]
    fn test_is_dependency_error_message_based() {
        // Check message-based detection
        assert!(EphemeralWorkspace::is_dependency_error(
            &None,
            "can't find crate `foo`"
        ));

        assert!(EphemeralWorkspace::is_dependency_error(
            &None,
            "unresolved import `bar::baz`"
        ));

        assert!(EphemeralWorkspace::is_dependency_error(
            &None,
            "could not find `qux` in `xyz`"
        ));
    }

    #[test]
    fn test_is_dependency_error_not_dependency() {
        // Various non-dependency errors
        assert!(!EphemeralWorkspace::is_dependency_error(
            &Some("E0277".to_string()),
            "trait bound not satisfied"
        ));

        assert!(!EphemeralWorkspace::is_dependency_error(
            &Some("E0382".to_string()),
            "borrow of moved value"
        ));

        assert!(!EphemeralWorkspace::is_dependency_error(&None, "some other error message"));
    }

    #[test]
    fn test_is_dependency_error_none_code() {
        // With None code, relies on message
        assert!(!EphemeralWorkspace::is_dependency_error(&None, "type mismatch"));
    }

    // === compile_with_cargo tests ===

    #[test]
    fn test_compile_with_cargo_default_toml() {
        let rust_code = r#"
            pub fn simple() -> i32 {
                42
            }
        "#;
        let result = compile_with_cargo("test_default", rust_code, None).unwrap();
        assert!(result.success, "Simple code should compile with default toml");
    }

    #[test]
    fn test_compile_with_cargo_custom_toml() {
        let rust_code = "pub fn test() {}";
        let custom_toml = r#"
[package]
name = "custom_pkg"
version = "0.2.0"
edition = "2021"
"#;
        let result = compile_with_cargo("custom_pkg", rust_code, Some(custom_toml)).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_compile_with_cargo_name_with_dash() {
        // Test that names with dashes are converted to underscores
        let rust_code = "pub fn test() {}";
        let result = compile_with_cargo("my-test-module", rust_code, None).unwrap();
        assert!(result.success);
    }

    // === quick_check tests ===

    #[test]
    fn test_quick_check_success() {
        let rust_code = "pub fn hello() {}";
        let result = quick_check("test_quick_ok", rust_code, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_quick_check_semantic_error() {
        let rust_code = r#"
            pub fn broken() -> i32 {
                "string"  // type error
            }
        "#;
        let result = quick_check("test_quick_fail", rust_code, None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        // Should contain error code for semantic error
        assert!(err.contains("E0") || !err.is_empty());
    }

    #[test]
    fn test_quick_check_with_custom_toml() {
        let rust_code = "pub fn test() {}";
        let toml = r#"
[package]
name = "quick_custom"
version = "0.1.0"
edition = "2021"
"#;
        let result = quick_check("quick_custom", rust_code, Some(toml));
        assert!(result.is_ok());
    }

    // === Edge case tests ===

    #[test]
    fn test_empty_rust_code() {
        let rust_code = "";
        let result = compile_with_cargo("empty_code", rust_code, None).unwrap();
        // Empty code should compile (creates empty lib)
        assert!(result.success);
    }

    #[test]
    fn test_code_with_warnings() {
        let rust_code = r#"
            pub fn test() {
                let unused_var = 42;
            }
        "#;
        let result = compile_with_cargo("with_warnings", rust_code, None).unwrap();
        // Should still succeed (warnings don't fail check by default)
        assert!(result.success);
        // May have warnings
        // (Note: warning detection depends on cargo check behavior)
    }

    #[test]
    fn test_multiple_functions() {
        let rust_code = r#"
            pub fn add(a: i32, b: i32) -> i32 { a + b }
            pub fn sub(a: i32, b: i32) -> i32 { a - b }
            pub fn mul(a: i32, b: i32) -> i32 { a * b }
        "#;
        let result = compile_with_cargo("multi_fn", rust_code, None).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_with_struct() {
        let rust_code = r#"
            pub struct Point {
                pub x: f64,
                pub y: f64,
            }

            impl Point {
                pub fn new(x: f64, y: f64) -> Self {
                    Self { x, y }
                }
            }
        "#;
        let result = compile_with_cargo("with_struct", rust_code, None).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_with_enum() {
        let rust_code = r#"
            pub enum Status {
                Active,
                Inactive,
                Pending,
            }
        "#;
        let result = compile_with_cargo("with_enum", rust_code, None).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_with_generics() {
        let rust_code = r#"
            pub fn identity<T>(x: T) -> T {
                x
            }

            pub struct Container<T> {
                pub value: T,
            }
        "#;
        let result = compile_with_cargo("with_generics", rust_code, None).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_syntax_error() {
        let rust_code = "pub fn broken( { }"; // syntax error
        let result = compile_with_cargo("syntax_err", rust_code, None).unwrap();
        assert!(!result.success);
        assert!(!result.errors.is_empty());
    }
}
