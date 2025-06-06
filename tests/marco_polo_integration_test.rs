#[cfg(test)]
mod marco_polo_tests {
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn test_marco_polo_simple_transpilation() {
        // Check if the example file exists
        let workspace_root = env!("CARGO_MANIFEST_DIR");
        let example_path = Path::new(workspace_root)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples/marco_polo_cli/marco_polo_simple.py");
        assert!(
            example_path.exists(),
            "Marco Polo simple example should exist at {:?}",
            example_path
        );

        // Try to transpile it
        let output = Command::new("cargo")
            .args(&["run", "--", "transpile", example_path.to_str().unwrap()])
            .output()
            .expect("Failed to run transpilation");

        // Check if transpilation succeeded
        assert!(
            output.status.success(),
            "Transpilation should succeed for marco_polo_simple.py"
        );

        // Verify output contains expected elements
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("📄 Source:") || stdout.contains("Source:"), 
               "Expected source information in output, got: {}", stdout);
    }

    #[test]
    fn test_marco_polo_annotations() {
        let workspace_root = env!("CARGO_MANIFEST_DIR");
        let example_path = Path::new(workspace_root)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples/marco_polo_cli/marco_polo_simple.py");
        let content = std::fs::read_to_string(&example_path).expect("Should read example file");

        // Verify the example contains Depyler annotations
        assert!(
            content.contains("@depyler:"),
            "Example should include Depyler annotations"
        );
        assert!(
            content.contains("optimization_level"),
            "Should have optimization annotations"
        );
        assert!(
            content.contains("string_strategy"),
            "Should have string strategy annotations"
        );
    }

    #[test]
    fn test_marco_polo_rust_project() {
        let workspace_root = env!("CARGO_MANIFEST_DIR");
        let project_dir = Path::new(workspace_root)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples/marco_polo_cli");
        let cargo_path = project_dir.join("Cargo.toml");
        assert!(
            cargo_path.exists(),
            "Marco Polo Rust project should exist at {:?}",
            cargo_path
        );

        // Verify the Rust project builds
        let output = Command::new("cargo")
            .args(&["check"])
            .current_dir(&project_dir)
            .output()
            .expect("Failed to check Rust project");

        assert!(
            output.status.success(),
            "Marco Polo Rust project should compile successfully"
        );
    }
}
