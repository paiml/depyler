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
    // DEPYLER-0763: Returns (project_dir, is_binary) - libraries have no main()
    pb.set_message("Creating Cargo project...");
    let (project_dir, is_binary) = create_cargo_project(input, &rust_code, &dependencies)?;
    pb.inc(1);

    // Step 3: Build project (binary or library)
    pb.set_message(if is_binary { "Building binary..." } else { "Building library..." });
    let cargo_profile = profile.unwrap_or("release");
    build_cargo_project(&project_dir, cargo_profile)?;
    pb.inc(1);

    // Step 4: Copy binary to output location (only for binaries)
    // DEPYLER-0763: Libraries don't produce executables - skip finalize for them
    pb.set_message("Finalizing...");
    let result_path = if is_binary {
        finalize_binary(&project_dir, input, output, cargo_profile)?
    } else {
        // Library: just return the project directory since there's no binary
        // The .rlib is in target/release/lib<name>.rlib but we don't need to copy it
        project_dir.clone()
    };
    pb.inc(1);

    pb.finish_with_message(if is_binary {
        "✅ Compilation complete!"
    } else {
        "✅ Library compilation complete!"
    });
    Ok(result_path)
}

/// Create a Cargo project with the transpiled Rust code
///
/// DEPYLER-0384: Now accepts dependencies for automatic Cargo.toml generation
/// DEPYLER-0763: Returns (project_dir, is_binary) - is_binary is true if code has main()
///
/// Complexity: 4 (within ≤10 target)
fn create_cargo_project(
    input: &Path,
    rust_code: &str,
    dependencies: &[depyler_core::cargo_toml_gen::Dependency],
) -> Result<(PathBuf, bool)> {
    let project_name = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let temp_dir = std::env::temp_dir();
    let project_dir = temp_dir.join(format!("depyler_{}", project_name));

    // Create project structure
    // DEPYLER-0763: Clean existing src directory to avoid stale files
    // (e.g., leftover main.rs when switching to lib.rs)
    let src_dir = project_dir.join("src");
    if src_dir.exists() {
        fs::remove_dir_all(&src_dir).ok(); // Ignore errors - might not exist
    }
    fs::create_dir_all(&src_dir).context("Failed to create src directory")?;

    // DEPYLER-0763: Check if code has fn main() to determine crate type
    // Libraries (no main) should be compiled as --crate-type lib to avoid E0601
    // CLIs with argparse/main functions should be compiled as binaries
    let has_main = rust_code.contains("fn main()") || rust_code.contains("pub fn main()");
    let (rs_filename, cargo_toml) = if has_main {
        // Binary: uses [[bin]] section
        let toml = depyler_core::cargo_toml_gen::generate_cargo_toml(
            project_name,
            "src/main.rs",
            dependencies,
        );
        ("src/main.rs", toml)
    } else {
        // Library: uses [lib] section - avoids E0601 "main function not found"
        // Must use generate_cargo_toml_lib directly (not _auto which only does lib for test_*)
        let toml = depyler_core::cargo_toml_gen::generate_cargo_toml_lib(
            project_name,
            "src/lib.rs",
            dependencies,
        );
        ("src/lib.rs", toml)
    };
    fs::write(project_dir.join("Cargo.toml"), cargo_toml).context("Failed to write Cargo.toml")?;

    // Write source file (main.rs or lib.rs based on crate type)
    fs::write(project_dir.join(rs_filename), rust_code)
        .with_context(|| format!("Failed to write {}", rs_filename))?;

    // DEPYLER-0763: Return whether this is a binary (has main) so caller knows what to finalize
    Ok((project_dir, has_main))
}

/// Build the Cargo project
///
/// DEPYLER-0380-FIX: Explicitly set target-dir to avoid inheriting parent project's
/// .cargo/config.toml target-dir setting which would cause builds to go to wrong location.
///
/// Complexity: 3 (within ≤10 target)
fn build_cargo_project(project_dir: &Path, profile: &str) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--manifest-path")
        .arg(project_dir.join("Cargo.toml"))
        // Explicitly set target directory to project's own target dir
        // This prevents inheriting the parent project's .cargo/config.toml target-dir
        .arg("--target-dir")
        .arg(project_dir.join("target"));

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
    fn test_create_cargo_project_binary() {
        let rust_code = r#"fn main() { println!("test"); }"#;
        let temp = TempDir::new().unwrap();
        let input = temp.path().join("test.py");
        fs::write(&input, "").unwrap();

        // DEPYLER-0384: Empty dependencies list for basic test
        let dependencies = vec![];
        // DEPYLER-0763: Now returns (project_dir, is_binary)
        let (project_dir, is_binary) = create_cargo_project(&input, rust_code, &dependencies).unwrap();

        assert!(is_binary, "Code with fn main() should be detected as binary");
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
    fn test_create_cargo_project_library() {
        // DEPYLER-0763: Test library detection (no main function)
        let rust_code = r#"pub fn greet(name: &str) -> String { format!("Hello, {}!", name) }"#;
        let temp = TempDir::new().unwrap();
        let input = temp.path().join("mylib.py");
        fs::write(&input, "").unwrap();

        let dependencies = vec![];
        let (project_dir, is_binary) = create_cargo_project(&input, rust_code, &dependencies).unwrap();

        assert!(!is_binary, "Code without fn main() should be detected as library");
        assert!(project_dir.join("Cargo.toml").exists());
        assert!(project_dir.join("src/lib.rs").exists());
        assert!(!project_dir.join("src/main.rs").exists(), "Library should not have main.rs");

        // Verify Cargo.toml has [lib] section instead of [[bin]]
        let cargo_content = fs::read_to_string(project_dir.join("Cargo.toml")).unwrap();
        assert!(cargo_content.contains("[lib]"), "Library should have [lib] section");
        assert!(!cargo_content.contains("[[bin]]"), "Library should not have [[bin]] section");
    }

    #[test]
    fn test_compile_nonexistent_file() {
        let result =
            compile_python_to_binary(Path::new("/nonexistent/file.py"), None, Some("release"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
