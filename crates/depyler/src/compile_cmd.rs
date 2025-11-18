//! DEPYLER-0380: Compile Command Implementation
//!
//! **EXTREME TDD - GREEN Phase**
//!
//! Single-shot Python-to-Rust compilation:
//! 1. Transpile Python → Rust
//! 2. Create Cargo project structure
//! 3. Build executable binary
//! 4. Return path to binary
//!
//! Complexity: ≤10 per function
//! TDG Score: A (≤2.0)
//! Coverage: ≥85%

use anyhow::{Context, Result};
use depyler_core::DepylerPipeline;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Compile a Python script to a standalone Rust binary
///
/// # Arguments
/// * `input` - Path to Python file
/// * `output` - Optional output binary path (defaults to input name without extension)
/// * `profile` - Cargo profile (release, debug, etc.)
///
/// # Returns
/// Path to the compiled binary
///
/// Complexity: 8 (within ≤10 target)
pub fn compile_python_to_binary(
    input: &Path,
    output: Option<&Path>,
    profile: Option<&str>,
) -> Result<PathBuf> {
    // Validate input exists
    if !input.exists() {
        anyhow::bail!("Input file not found: {}", input.display());
    }

    // Set up progress bar
    let pb = ProgressBar::new(4);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░ "),
    );

    // Step 1: Transpile Python → Rust (DEPYLER-0384: with dependency tracking)
    pb.set_message("Transpiling Python to Rust...");
    let python_code = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input.display()))?;

    let pipeline = DepylerPipeline::new();
    let (rust_code, dependencies) = pipeline
        .transpile_with_dependencies(&python_code)
        .context("Failed to transpile Python to Rust")?;
    pb.inc(1);

    // Step 2: Create Cargo project (DEPYLER-0384: with automatic dependencies)
    pb.set_message("Creating Cargo project...");
    let project_dir = create_cargo_project(input, &rust_code, &dependencies)?;
    pb.inc(1);

    // Step 3: Build binary
    pb.set_message("Building binary...");
    let cargo_profile = profile.unwrap_or("release");
    build_cargo_project(&project_dir, cargo_profile)?;
    pb.inc(1);

    // Step 4: Copy binary to output location
    pb.set_message("Finalizing...");
    let binary_path = finalize_binary(&project_dir, input, output, cargo_profile)?;
    pb.inc(1);

    pb.finish_with_message("✅ Compilation complete!");
    Ok(binary_path)
}

/// Create a Cargo project with the transpiled Rust code
///
/// DEPYLER-0384: Now accepts dependencies for automatic Cargo.toml generation
///
/// Complexity: 3 (within ≤10 target)
fn create_cargo_project(
    input: &Path,
    rust_code: &str,
    dependencies: &[depyler_core::cargo_toml_gen::Dependency],
) -> Result<PathBuf> {
    let project_name = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let temp_dir = std::env::temp_dir();
    let project_dir = temp_dir.join(format!("depyler_{}", project_name));

    // Create project structure
    fs::create_dir_all(project_dir.join("src")).context("Failed to create src directory")?;

    // DEPYLER-0384: Generate Cargo.toml with automatic dependencies
    let cargo_toml = depyler_core::cargo_toml_gen::generate_cargo_toml(project_name, dependencies);
    fs::write(project_dir.join("Cargo.toml"), cargo_toml)
        .context("Failed to write Cargo.toml")?;

    // Write main.rs
    fs::write(project_dir.join("src/main.rs"), rust_code).context("Failed to write main.rs")?;

    Ok(project_dir)
}

/// Build the Cargo project
///
/// Complexity: 2 (within ≤10 target)
fn build_cargo_project(project_dir: &Path, profile: &str) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--manifest-path")
        .arg(project_dir.join("Cargo.toml"));

    if profile == "release" {
        cmd.arg("--release");
    }

    let output = cmd.output().context("Failed to run cargo build")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Cargo build failed:\n{}", stderr);
    }

    Ok(())
}

/// Copy the built binary to the final location
///
/// Complexity: 4 (within ≤10 target)
fn finalize_binary(
    project_dir: &Path,
    input: &Path,
    output: Option<&Path>,
    profile: &str,
) -> Result<PathBuf> {
    let project_name = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    // Determine binary location in target directory
    let profile_dir = if profile == "release" {
        "release"
    } else {
        "debug"
    };
    let binary_name = if cfg!(windows) {
        format!("{}.exe", project_name)
    } else {
        project_name.to_string()
    };
    let built_binary = project_dir
        .join("target")
        .join(profile_dir)
        .join(&binary_name);

    // Determine output location
    let output_path = if let Some(out) = output {
        out.to_path_buf()
    } else {
        input.with_file_name(&binary_name)
    };

    // Copy binary
    fs::copy(&built_binary, &output_path).with_context(|| {
        format!(
            "Failed to copy binary from {} to {}",
            built_binary.display(),
            output_path.display()
        )
    })?;

    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&output_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&output_path, perms)?;
    }

    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_create_cargo_project() {
        let rust_code = r#"fn main() { println!("test"); }"#;
        let temp = TempDir::new().unwrap();
        let input = temp.path().join("test.py");
        fs::write(&input, "").unwrap();

        // DEPYLER-0384: Empty dependencies list for basic test
        let dependencies = vec![];
        let project_dir = create_cargo_project(&input, rust_code, &dependencies).unwrap();

        assert!(project_dir.join("Cargo.toml").exists());
        assert!(project_dir.join("src/main.rs").exists());

        let main_content = fs::read_to_string(project_dir.join("src/main.rs")).unwrap();
        assert!(main_content.contains("test"));

        // DEPYLER-0384: Verify Cargo.toml has package section
        let cargo_content = fs::read_to_string(project_dir.join("Cargo.toml")).unwrap();
        assert!(cargo_content.contains("[package]"));
        assert!(cargo_content.contains("name = \"test\""));
    }

    #[test]
    fn test_compile_nonexistent_file() {
        let result = compile_python_to_binary(
            Path::new("/nonexistent/file.py"),
            None,
            Some("release"),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
