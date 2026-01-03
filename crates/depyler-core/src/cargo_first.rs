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

        assert!(result.success, "Valid code should compile: {:?}", result.errors);
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
}
