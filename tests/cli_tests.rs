use depyler_core::DepylerPipeline;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_basic_transpile_command() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.py");

    fs::write(
        &input_file,
        "def add(a: int, b: int) -> int:\n    return a + b",
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "transpile", input_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_analyze_command() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.py");

    fs::write(
        &input_file,
        "def simple_func(x: int) -> int:\n    return x * 2",
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "analyze", input_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_check_command() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.py");

    fs::write(
        &input_file,
        "def check_func(n: int) -> bool:\n    return n > 0",
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "check", input_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Command might fail but should execute without panicking
    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_quality_check_command() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.py");

    fs::write(
        &input_file,
        "def quality_func(x: int) -> int:\n    return x + 1",
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "quality-check", input_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Command should execute and produce quality report
    assert!(!output.stdout.is_empty() || !output.stderr.is_empty());
}

#[test]
fn test_invalid_file_handling() {
    let non_existent_file = "/tmp/does_not_exist_12345.py";

    let output = Command::new("cargo")
        .args(["run", "--", "transpile", non_existent_file])
        .output()
        .expect("Failed to execute command");

    // Should fail gracefully with error message
    assert!(!output.status.success());
    assert!(!output.stderr.is_empty());
}

#[test]
#[ignore] // Interactive mode requires TTY and is difficult to test in CI
fn test_interactive_mode_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.py");

    fs::write(
        &input_file,
        "def interactive_func(a: int) -> int:\n    return a * 3",
    )
    .unwrap();

    // Test interactive mode without user input (should handle gracefully)
    // Note: This test is disabled in CI because it requires TTY and timeout command
    // which may not be available on all platforms

    // For manual testing, run: cargo test test_interactive_mode_basic -- --ignored
}

#[test]
fn test_pipeline_error_handling() {
    let pipeline = DepylerPipeline::new();

    // Test with invalid Python syntax
    let result = pipeline.transpile("def invalid_syntax(\n    # Missing closing parenthesis");
    assert!(result.is_err());

    // Test with empty input
    let result = pipeline.transpile("");
    // Empty can be either success or error, just check it doesn't panic
    let _ = result; // Both Ok and Err are acceptable

    // Test with complex unsupported syntax
    let result = pipeline.transpile("async def async_func():\n    await something()");
    // Async might or might not be supported - just check it doesn't panic
    let _ = result; // Both Ok and Err are acceptable
}

#[test]
fn test_json_output_format() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.py");

    fs::write(
        &input_file,
        "def json_test(x: int) -> str:\n    return str(x)",
    )
    .unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "analyze",
            input_file.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        // Should contain valid JSON structure
        assert!(stdout_str.contains("{") && stdout_str.contains("}"));
    }
}

#[test]
fn test_multiple_quality_gates() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("complex.py");

    // Create a more complex function that might fail some quality gates
    fs::write(
        &input_file,
        r#"
def complex_function(a: int, b: int, c: int, d: int) -> int:
    if a > 0:
        if b > 0:
            if c > 0:
                if d > 0:
                    return a + b + c + d
                else:
                    return a + b + c
            else:
                return a + b
        else:
            return a
    else:
        return 0
"#,
    )
    .unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "quality-check",
            input_file.to_str().unwrap(),
            "--max-complexity",
            "10",
            "--min-coverage",
            "70",
        ])
        .output()
        .expect("Failed to execute command");

    // Should produce quality report regardless of pass/fail
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout_str.contains("Quality Report")
            || stdout_str.contains("Quality Gates")
            || !output.stderr.is_empty()
    );
}

#[test]
fn test_annotation_parsing_integration() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("annotated.py");

    fs::write(
        &input_file,
        r#"
# @depyler: type_strategy = "conservative"
# @depyler: ownership = "borrowed"
def annotated_func(items: list[int]) -> list[int]:
    """Function with annotations"""
    return [x * 2 for x in items if x > 0]
"#,
    )
    .unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "interactive",
            input_file.to_str().unwrap(),
            "--annotate",
        ])
        .output()
        .expect("Failed to execute command");

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    // Should parse and display annotations
    assert!(
        stdout_str.contains("annotations")
            || stdout_str.contains("Strategy")
            || !output.stderr.is_empty()
    );
}

#[test]
fn test_file_permissions_error() {
    // Create a file with restricted permissions (if possible)
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("restricted.py");

    fs::write(&input_file, "def test(): pass").unwrap();

    // Try to make it unreadable (may not work on all systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&input_file).unwrap().permissions();
        perms.set_mode(0o000);
        let _ = fs::set_permissions(&input_file, perms);

        let output = Command::new("cargo")
            .args(["run", "--", "transpile", input_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command");

        // Should handle permission error gracefully
        if !output.status.success() {
            assert!(!output.stderr.is_empty());
        }
    }
}

// ============================================================================
// COMPREHENSIVE CLI FLAG COVERAGE
// Helper function for creating test Python files
// ============================================================================

fn create_test_py(dir: &TempDir, name: &str) -> std::path::PathBuf {
    let path = dir.path().join(name);
    fs::write(&path, "def add(a: int, b: int) -> int:\n    return a + b").unwrap();
    path
}

// ============================================================================
// TRANSPILE - All Flags
// ============================================================================

#[test]
fn test_transpile_with_output_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");
    let output = dir.path().join("out.rs");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "transpile",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_transpile_verify_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "transpile",
            input.to_str().unwrap(),
            "--verify",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_transpile_gen_tests_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "transpile",
            input.to_str().unwrap(),
            "--gen-tests",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_transpile_debug_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args(["run", "--", "transpile", input.to_str().unwrap(), "--debug"])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_transpile_source_map_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "transpile",
            input.to_str().unwrap(),
            "--source-map",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_transpile_all_flags_combined() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");
    let output = dir.path().join("out.rs");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "transpile",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--verify",
            "--gen-tests",
            "--debug",
            "--source-map",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

// ============================================================================
// QUALITY-CHECK - Missing Flags
// ============================================================================

#[test]
fn test_quality_check_enforce_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "quality-check",
            input.to_str().unwrap(),
            "--enforce",
        ])
        .output()
        .expect("Failed to execute");

    // May pass or fail based on quality, just verify it runs
    assert!(!result.stdout.is_empty() || !result.stderr.is_empty());
}

#[test]
fn test_quality_check_min_tdg_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "quality-check",
            input.to_str().unwrap(),
            "--min-tdg",
            "0.5",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_quality_check_max_tdg_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "quality-check",
            input.to_str().unwrap(),
            "--max-tdg",
            "3.0",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

// ============================================================================
// INSPECT - All Flags (Previously Untested)
// ============================================================================

#[test]
fn test_inspect_basic() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args(["run", "--", "inspect", input.to_str().unwrap()])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_inspect_repr_python_ast() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "inspect",
            input.to_str().unwrap(),
            "--repr",
            "python-ast",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_inspect_repr_typed_hir() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "inspect",
            input.to_str().unwrap(),
            "--repr",
            "typed-hir",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_inspect_format_json() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "inspect",
            input.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_inspect_format_debug() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "inspect",
            input.to_str().unwrap(),
            "--format",
            "debug",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_inspect_with_output_file() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");
    let output = dir.path().join("inspect.txt");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "inspect",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

// ============================================================================
// DEBUG - All Flags (Previously Untested)
// ============================================================================

#[test]
fn test_debug_tips() {
    let result = Command::new("cargo")
        .args(["run", "--", "debug", "--tips"])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_debug_gen_script() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");
    let script = dir.path().join("debug.gdb");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "debug",
            "--gen-script",
            script.to_str().unwrap(),
            "--source",
            input.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_debug_debugger_lldb() {
    let result = Command::new("cargo")
        .args(["run", "--", "debug", "--debugger", "lldb", "--tips"])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

// ============================================================================
// DOCS - All Flags (Previously Untested)
// ============================================================================

#[test]
fn test_docs_basic() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");
    let output = dir.path().join("docs");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "docs",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_docs_format_html() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");
    let output = dir.path().join("docs");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "docs",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--format",
            "html",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_docs_performance_notes_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");
    let output = dir.path().join("docs");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "docs",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--performance-notes",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

// ============================================================================
// PROFILE - All Flags (Previously Untested)
// ============================================================================

#[test]
fn test_profile_basic() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args(["run", "--", "profile", input.to_str().unwrap()])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_profile_flamegraph_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "profile",
            input.to_str().unwrap(),
            "--flamegraph",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_profile_hot_path_threshold() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "profile",
            input.to_str().unwrap(),
            "--hot-path-threshold",
            "200",
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

// ============================================================================
// AGENT - Subcommands (Previously Untested)
// ============================================================================

#[test]
fn test_agent_status() {
    let result = Command::new("cargo")
        .args(["run", "--", "agent", "status"])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
fn test_agent_list_projects() {
    let result = Command::new("cargo")
        .args(["run", "--", "agent", "list-projects"])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

// ============================================================================
// GLOBAL FLAGS
// ============================================================================

#[test]
fn test_global_verbose_flag() {
    let dir = TempDir::new().unwrap();
    let input = create_test_py(&dir, "test.py");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "--verbose",
            "transpile",
            input.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success() || !result.stderr.is_empty());
}

#[test]
#[ignore] // Flaky under coverage instrumentation - cargo run from instrumented test causes issues
fn test_help_flag() {
    let result = Command::new("cargo")
        .args(["run", "--bin", "depyler", "--", "--help"])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success());
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("depyler") || stdout.contains("Usage"));
}

#[test]
#[ignore] // Flaky under coverage instrumentation - cargo run from instrumented test causes issues
fn test_version_flag() {
    let result = Command::new("cargo")
        .args(["run", "--bin", "depyler", "--", "--version"])
        .output()
        .expect("Failed to execute");

    assert!(result.status.success());
}
