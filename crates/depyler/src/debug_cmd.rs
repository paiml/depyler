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
        rust_file.file_stem().expect("file has stem").to_string_lossy(),
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
        Err(anyhow::anyhow!(
            "Debugger exited with error code: {:?}",
            status.code()
        ))
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

    #[test]
    fn test_debugger_case_insensitive() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("case.py");
        let rust_file = temp_dir.path().join("case.rs");

        fs::write(&source_file, "def foo(): pass").unwrap();
        fs::write(&rust_file, "fn foo() {}").unwrap();

        // Test uppercase
        let gdb_upper = temp_dir.path().join("upper.gdb");
        let result = generate_debugger_script(&source_file, &rust_file, "GDB", Some(&gdb_upper));
        assert!(result.is_ok());

        // Test mixed case
        let lldb_mixed = temp_dir.path().join("mixed.lldb");
        let result = generate_debugger_script(&source_file, &rust_file, "LlDb", Some(&lldb_mixed));
        assert!(result.is_ok());

        // Test rust-gdb mixed case
        let result = generate_debugger_script(&source_file, &rust_file, "Rust-GDB", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gdb_script_content() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("content.py");
        let rust_file = temp_dir.path().join("content.rs");
        let gdb_output = temp_dir.path().join("content.gdb");

        fs::write(&source_file, "def test_func(): pass").unwrap();
        fs::write(&rust_file, "fn test_func() {}").unwrap();

        let result = generate_debugger_script(&source_file, &rust_file, "gdb", Some(&gdb_output));
        assert!(result.is_ok());

        let content = fs::read_to_string(&gdb_output).unwrap();
        // GDB script should have some content
        assert!(!content.is_empty());
    }

    #[test]
    fn test_lldb_script_content() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("lldb_content.py");
        let rust_file = temp_dir.path().join("lldb_content.rs");
        let lldb_output = temp_dir.path().join("lldb_content.lldb");

        fs::write(&source_file, "def another(): pass").unwrap();
        fs::write(&rust_file, "fn another() {}").unwrap();

        let result = generate_debugger_script(&source_file, &rust_file, "lldb", Some(&lldb_output));
        assert!(result.is_ok());

        let content = fs::read_to_string(&lldb_output).unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_rust_gdb_script_default_extension() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("rustgdb.py");
        let rust_file = temp_dir.path().join("rustgdb.rs");

        fs::write(&source_file, "def rust_debug(): pass").unwrap();
        fs::write(&rust_file, "fn rust_debug() {}").unwrap();

        // rust-gdb should produce .gdb extension
        let result = generate_debugger_script(&source_file, &rust_file, "rust-gdb", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_spydecy_not_installed() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("spy.py");

        fs::write(&source_file, "def spy(): pass").unwrap();

        // spydecy likely not installed, should return error
        let result = launch_spydecy_debugger(&source_file, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_spydecy_with_visualize() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("viz.py");

        fs::write(&source_file, "def visualize(): pass").unwrap();

        // spydecy likely not installed, should return error
        let result = launch_spydecy_debugger(&source_file, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_debugger_name() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("empty.py");
        let rust_file = temp_dir.path().join("empty.rs");

        let result = generate_debugger_script(&source_file, &rust_file, "", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_various_unknown_debuggers() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("unknown.py");
        let rust_file = temp_dir.path().join("unknown.rs");

        let unknown_debuggers = vec!["windbg", "ollydbg", "ida", "x64dbg", "radare2"];
        for dbg in unknown_debuggers {
            let result = generate_debugger_script(&source_file, &rust_file, dbg, None);
            assert!(result.is_err(), "Expected error for debugger: {}", dbg);
        }
    }

    #[test]
    fn test_paths_with_spaces() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("file with spaces.py");
        let rust_file = temp_dir.path().join("file with spaces.rs");
        let output = temp_dir.path().join("output with spaces.gdb");

        fs::write(&source_file, "def spaced(): pass").unwrap();
        fs::write(&rust_file, "fn spaced() {}").unwrap();

        let result = generate_debugger_script(&source_file, &rust_file, "gdb", Some(&output));
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_special_characters_in_filename() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("test_file-v2.0.py");
        let rust_file = temp_dir.path().join("test_file-v2.0.rs");

        fs::write(&source_file, "def version(): pass").unwrap();
        fs::write(&rust_file, "fn version() {}").unwrap();

        let result = generate_debugger_script(&source_file, &rust_file, "gdb", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_all_debugger_types_produce_output() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("alldbg.py");
        let rust_file = temp_dir.path().join("alldbg.rs");

        fs::write(&source_file, "def all_debug(): pass").unwrap();
        fs::write(&rust_file, "fn all_debug() {}").unwrap();

        let debuggers = vec![("gdb", "gdb"), ("lldb", "lldb"), ("rust-gdb", "gdb")];
        for (dbg, ext) in debuggers {
            let output = temp_dir.path().join(format!("alldbg_{}.{}", dbg, ext));
            let result = generate_debugger_script(&source_file, &rust_file, dbg, Some(&output));
            assert!(result.is_ok(), "Failed for debugger: {}", dbg);
            assert!(output.exists(), "Output not created for: {}", dbg);
        }
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
