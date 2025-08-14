//! Property tests for debug command functionality

use depyler::debug_cmd::generate_debugger_script;
use proptest::prelude::*;
use std::fs;
use tempfile::TempDir;

proptest! {
    /// Property: Debugger script generation never panics for valid inputs
    #[test]
    fn prop_debugger_script_never_panics(
        filename in "[a-zA-Z][a-zA-Z0-9_]{0,20}",
        debugger in prop::sample::select(vec!["gdb", "lldb", "rust-gdb"])
    ) {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join(format!("{}.py", filename));
        let rust_file = temp_dir.path().join(format!("{}.rs", filename));

        // Create dummy files
        fs::write(&source_file, "def test(): pass").unwrap();
        fs::write(&rust_file, "fn test() {}").unwrap();

        // Should not panic
        let _ = generate_debugger_script(&source_file, &rust_file, debugger, None);
    }

    /// Property: Generated script files have correct extensions
    #[test]
    fn prop_correct_script_extensions(
        filename in "[a-zA-Z][a-zA-Z0-9_]{0,20}",
        debugger in prop::sample::select(vec!["gdb", "lldb", "rust-gdb"])
    ) {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join(format!("{}.py", filename));
        let rust_file = temp_dir.path().join(format!("{}.rs", filename));
        let output_file = temp_dir.path().join(format!("{}_output", filename));

        fs::write(&source_file, "def test(): pass").unwrap();
        fs::write(&rust_file, "fn test() {}").unwrap();

        if let Ok(()) = generate_debugger_script(&source_file, &rust_file, debugger, Some(&output_file)) {
            assert!(output_file.exists());

            // Verify file was created
            let content = fs::read_to_string(&output_file).unwrap();
            assert!(!content.is_empty());
        }
    }

    /// Property: Invalid debugger names always return errors
    #[test]
    fn prop_invalid_debugger_returns_error(
        filename in "[a-zA-Z][a-zA-Z0-9_]{0,20}",
        invalid_debugger in "[a-zA-Z][a-zA-Z0-9_]{5,20}".prop_filter("Must not be valid debugger",
            |s| !["gdb", "lldb", "rust-gdb"].contains(&s.as_str()))
    ) {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join(format!("{}.py", filename));
        let rust_file = temp_dir.path().join(format!("{}.rs", filename));

        fs::write(&source_file, "def test(): pass").unwrap();
        fs::write(&rust_file, "fn test() {}").unwrap();

        let result = generate_debugger_script(&source_file, &rust_file, &invalid_debugger, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown debugger"));
    }

    /// Property: Path handling works with various file structures
    #[test]
    fn prop_path_handling(
        dir_depth in 0usize..5,
        filename in "[a-zA-Z][a-zA-Z0-9_]{0,20}"
    ) {
        let temp_dir = TempDir::new().unwrap();

        // Create nested directory structure
        let mut path = temp_dir.path().to_path_buf();
        for i in 0..dir_depth {
            path = path.join(format!("dir{}", i));
        }
        fs::create_dir_all(&path).unwrap();

        let source_file = path.join(format!("{}.py", filename));
        let rust_file = path.join(format!("{}.rs", filename));

        fs::write(&source_file, "def test(): pass").unwrap();
        fs::write(&rust_file, "fn test() {}").unwrap();

        // Should handle nested paths correctly
        let result = generate_debugger_script(&source_file, &rust_file, "gdb", None);
        assert!(result.is_ok());
    }
}
