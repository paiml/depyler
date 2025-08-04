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

    let default_output = PathBuf::from(format!("{}.{}", 
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
    println!("3. Debug with GDB:");
    println!("   rust-gdb target/debug/your_program");
    println!("   (gdb) source script.gdb");
    println!();
    println!("4. Debug with LLDB:");
    println!("   rust-lldb target/debug/your_program");
    println!("   (lldb) command source script.lldb");
    println!();
    println!("5. Set breakpoints:");
    println!("   - In original Python function names");
    println!("   - Line numbers map to Rust code");
    println!();
    println!("6. View variables:");
    println!("   - Python variables retain their names");
    println!("   - Use 'info locals' (gdb) or 'frame variable' (lldb)");
}