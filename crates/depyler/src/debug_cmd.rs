//! Debug-related commands for Depyler

use anyhow::Result;
use depyler_core::debug::{DebuggerIntegration, DebuggerType, SourceMap};
use std::fs;
use std::path::{Path, PathBuf};

/// Generate a debugger initialization script
pub fn generate_debugger_script(
    source_file: &Path,
    rust_file: &Path,
    debugger: &str,
    output: Option<&Path>,
) -> Result<()> {
    // Create a mock source map (in real implementation, this would be loaded from transpilation)
    let source_map = SourceMap {
        source_file: source_file.to_path_buf(),
        target_file: rust_file.to_path_buf(),
        mappings: vec![],
        function_map: std::collections::HashMap::new(),
    };

    let debugger_type = match debugger.to_lowercase().as_str() {
        "gdb" => DebuggerType::Gdb,
        "rust-gdb" => DebuggerType::RustGdb,
        "lldb" => DebuggerType::Lldb,
        _ => return Err(anyhow::anyhow!("Unknown debugger: {}", debugger)),
    };

    let integration = DebuggerIntegration::new(debugger_type);
    let script = integration.generate_init_script(&source_map);

    let default_output = PathBuf::from(format!(
        "{}.{}",
        rust_file.file_stem().unwrap().to_string_lossy(),
        match debugger_type {
            DebuggerType::Gdb | DebuggerType::RustGdb => "gdb",
            DebuggerType::Lldb => "lldb",
        }
    ));
    let output_path = output.unwrap_or(&default_output);

    fs::write(output_path, script)?;
    println!("✅ Generated debugger script: {}", output_path.display());

    Ok(())
}

/// Launch interactive debugger with spydecy
pub fn launch_spydecy_debugger(source_file: &Path, visualize: bool) -> Result<()> {
    use std::process::Command;

    println!("🐛 Launching spydecy interactive debugger...");
    println!("   Source: {}", source_file.display());

    let mut cmd = Command::new("spydecy");
    cmd.arg("debug").arg(source_file);

    if visualize {
        cmd.arg("--visualize");
        println!("   Visualization: enabled");
    }

    let status = cmd.status()?;

    if status.success() {
        println!("✅ Debugger session completed");
        Ok(())
    } else {
        Err(anyhow::anyhow!("Debugger exited with error code: {:?}", status.code()))
    }
}

/// Print debugging tips
pub fn print_debugging_tips() {
    println!("🐛 Depyler Debugging Guide");
    println!("==========================");
    println!();
    println!("1. Transpile with debug info:");
    println!("   depyler transpile script.py --debug");
    println!();
    println!("2. Generate source map:");
    println!("   depyler transpile script.py --source-map");
    println!();
    println!("3. Interactive debugging with spydecy:");
    println!("   depyler debug --spydecy script.py");
    println!("   depyler debug --spydecy script.py --visualize");
    println!();
    println!("4. Debug with GDB:");
    println!("   rust-gdb target/debug/your_program");
    println!("   (gdb) source script.gdb");
    println!();
    println!("5. Debug with LLDB:");
    println!("   rust-lldb target/debug/your_program");
    println!("   (lldb) command source script.lldb");
    println!();
    println!("6. Set breakpoints:");
    println!("   - In original Python function names");
    println!("   - Line numbers map to Rust code");
    println!();
    println!("7. View variables:");
    println!("   - Python variables retain their names");
    println!("   - Use 'info locals' (gdb) or 'frame variable' (lldb)");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Test debugger script generation for different debuggers
    #[test]
    fn test_generate_debugger_script() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("test.py");
        let rust_file = temp_dir.path().join("test.rs");

        // Create dummy files
        fs::write(&source_file, "def test(): pass").unwrap();
        fs::write(&rust_file, "fn test() {}").unwrap();

        // Test GDB script generation
        let gdb_output = temp_dir.path().join("test.gdb");
        let result = generate_debugger_script(&source_file, &rust_file, "gdb", Some(&gdb_output));
        assert!(result.is_ok());
        assert!(gdb_output.exists());

        // Test LLDB script generation
        let lldb_output = temp_dir.path().join("test.lldb");
        let result = generate_debugger_script(&source_file, &rust_file, "lldb", Some(&lldb_output));
        assert!(result.is_ok());
        assert!(lldb_output.exists());

        // Test rust-gdb
        let result = generate_debugger_script(&source_file, &rust_file, "rust-gdb", None);
        assert!(result.is_ok());
    }

    /// Test error handling for unknown debuggers
    #[test]
    fn test_unknown_debugger_error() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("test.py");
        let rust_file = temp_dir.path().join("test.rs");

        let result = generate_debugger_script(&source_file, &rust_file, "unknown-debugger", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown debugger"));
    }

    /// Test default output path generation
    #[test]
    fn test_default_output_path() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("my_script.py");
        let rust_file = temp_dir.path().join("my_script.rs");

        fs::write(&source_file, "def test(): pass").unwrap();
        fs::write(&rust_file, "fn test() {}").unwrap();

        // Without specifying output, should create default file
        let result = generate_debugger_script(&source_file, &rust_file, "gdb", None);
        assert!(result.is_ok());

        // Check default file exists
        let default_gdb = temp_dir.path().join("my_script.gdb");
        assert!(default_gdb.exists() || PathBuf::from("my_script.gdb").exists());
    }

    /// Test debugger tips printing (just ensure it doesn't panic)
    #[test]
    fn test_print_debugging_tips() {
        // Just ensure the function runs without panicking
        print_debugging_tips();
    }
}

/// Doctests for public API
///
/// # Example
/// ```no_run
/// use depyler::debug_cmd::{generate_debugger_script, print_debugging_tips};
/// use std::path::Path;
///
/// // Generate a GDB script for debugging
/// let source = Path::new("my_script.py");
/// let rust = Path::new("my_script.rs");
/// generate_debugger_script(source, rust, "gdb", None).unwrap();
///
/// // Print debugging tips
/// print_debugging_tips();
/// ```
pub mod examples {}
