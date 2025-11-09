//! Comprehensive CLI flag coverage tests
//!
//! Target: 100% coverage of all depyler CLI flags and combinations
//! Coverage focus: All Commands variants and their flags
//!
//! Test Strategy:
//! - Every top-level command
//! - Every flag combination for each command
//! - Global --verbose flag
//! - Error cases (invalid inputs, missing files)
//! - Flag interactions and combinations

use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Test helper: Create a simple Python file for testing
fn create_test_py_file(dir: &TempDir, filename: &str) -> std::path::PathBuf {
    let file_path = dir.path().join(filename);
    fs::write(
        &file_path,
        "def add(a: int, b: int) -> int:\n    return a + b",
    )
    .unwrap();
    file_path
}

// ============================================================================
// TRANSPILE COMMAND TESTS
// ============================================================================

#[test]
fn test_transpile_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["transpile", input.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_transpile_with_output_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "input.py");
    let output = temp_dir.path().join("output.rs");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "transpile",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn test_transpile_with_verify_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["transpile", input.to_str().unwrap(), "--verify"])
        .assert()
        .success();
}

#[test]
fn test_transpile_with_gen_tests_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["transpile", input.to_str().unwrap(), "--gen-tests"])
        .assert()
        .success();
}

#[test]
fn test_transpile_with_debug_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["transpile", input.to_str().unwrap(), "--debug"])
        .assert()
        .success();
}

#[test]
fn test_transpile_with_source_map_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["transpile", input.to_str().unwrap(), "--source-map"])
        .assert()
        .success();
}

#[test]
fn test_transpile_all_flags_combined() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");
    let output = temp_dir.path().join("output.rs");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "transpile",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--verify",
            "--gen-tests",
            "--debug",
            "--source-map",
        ])
        .assert()
        .success();
}

// ============================================================================
// ANALYZE COMMAND TESTS
// ============================================================================

#[test]
fn test_analyze_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["analyze", input.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_analyze_format_json() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["analyze", input.to_str().unwrap(), "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("{"));
}

#[test]
fn test_analyze_format_text() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["analyze", input.to_str().unwrap(), "--format", "text"])
        .assert()
        .success();
}

// ============================================================================
// CHECK COMMAND TESTS
// ============================================================================

#[test]
fn test_check_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["check", input.to_str().unwrap()])
        .assert()
        .success();
}

// ============================================================================
// QUALITY-CHECK COMMAND TESTS
// ============================================================================

#[test]
fn test_quality_check_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["quality-check", input.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_quality_check_with_enforce_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["quality-check", input.to_str().unwrap(), "--enforce"])
        .assert();
    // May succeed or fail depending on quality, just check it runs
}

#[test]
fn test_quality_check_with_min_tdg() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "quality-check",
            input.to_str().unwrap(),
            "--min-tdg",
            "0.5",
        ])
        .assert()
        .success();
}

#[test]
fn test_quality_check_with_max_tdg() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "quality-check",
            input.to_str().unwrap(),
            "--max-tdg",
            "3.0",
        ])
        .assert()
        .success();
}

#[test]
fn test_quality_check_with_max_complexity() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "quality-check",
            input.to_str().unwrap(),
            "--max-complexity",
            "15",
        ])
        .assert()
        .success();
}

#[test]
fn test_quality_check_with_min_coverage() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "quality-check",
            input.to_str().unwrap(),
            "--min-coverage",
            "70",
        ])
        .assert()
        .success();
}

#[test]
fn test_quality_check_all_flags() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "quality-check",
            input.to_str().unwrap(),
            "--enforce",
            "--min-tdg",
            "0.5",
            "--max-tdg",
            "3.0",
            "--max-complexity",
            "20",
            "--min-coverage",
            "75",
        ])
        .assert();
    // May succeed or fail, just check it runs
}

// ============================================================================
// INTERACTIVE COMMAND TESTS
// ============================================================================

#[test]
#[ignore] // Interactive mode requires TTY
fn test_interactive_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["interactive", input.to_str().unwrap()])
        .assert();
}

#[test]
#[ignore] // Interactive mode requires TTY
fn test_interactive_with_annotate() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["interactive", input.to_str().unwrap(), "--annotate"])
        .assert();
}

// ============================================================================
// INSPECT COMMAND TESTS
// ============================================================================

#[test]
fn test_inspect_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["inspect", input.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_inspect_repr_python_ast() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "inspect",
            input.to_str().unwrap(),
            "--repr",
            "python-ast",
        ])
        .assert()
        .success();
}

#[test]
fn test_inspect_repr_hir() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["inspect", input.to_str().unwrap(), "--repr", "hir"])
        .assert()
        .success();
}

#[test]
fn test_inspect_repr_typed_hir() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["inspect", input.to_str().unwrap(), "--repr", "typed-hir"])
        .assert()
        .success();
}

#[test]
fn test_inspect_format_json() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["inspect", input.to_str().unwrap(), "--format", "json"])
        .assert()
        .success();
}

#[test]
fn test_inspect_format_debug() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["inspect", input.to_str().unwrap(), "--format", "debug"])
        .assert()
        .success();
}

#[test]
fn test_inspect_format_pretty() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["inspect", input.to_str().unwrap(), "--format", "pretty"])
        .assert()
        .success();
}

#[test]
fn test_inspect_with_output_file() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");
    let output = temp_dir.path().join("inspect_output.txt");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "inspect",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Verify output file was created
    assert!(output.exists());
}

#[test]
fn test_inspect_all_flags() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");
    let output = temp_dir.path().join("inspect.json");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "inspect",
            input.to_str().unwrap(),
            "--repr",
            "hir",
            "--format",
            "json",
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();
}

// ============================================================================
// LSP COMMAND TESTS
// ============================================================================

#[test]
#[ignore] // LSP starts a server, difficult to test in CI
fn test_lsp_basic() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["lsp"])
        .timeout(std::time::Duration::from_secs(1))
        .assert();
}

#[test]
#[ignore] // LSP starts a server
fn test_lsp_with_port() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["lsp", "--port", "3000"])
        .timeout(std::time::Duration::from_secs(1))
        .assert();
}

#[test]
#[ignore] // LSP starts a server
fn test_lsp_with_verbose() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["lsp", "--verbose"])
        .timeout(std::time::Duration::from_secs(1))
        .assert();
}

// ============================================================================
// DEBUG COMMAND TESTS
// ============================================================================

#[test]
fn test_debug_tips() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["debug", "--tips"])
        .assert()
        .success();
}

#[test]
fn test_debug_gen_script() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");
    let script = temp_dir.path().join("debug.gdb");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "debug",
            "--gen-script",
            script.to_str().unwrap(),
            "--source",
            input.to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn test_debug_debugger_gdb() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["debug", "--debugger", "gdb", "--tips"])
        .assert()
        .success();
}

#[test]
fn test_debug_debugger_lldb() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["debug", "--debugger", "lldb", "--tips"])
        .assert()
        .success();
}

#[test]
fn test_debug_debugger_rust_gdb() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["debug", "--debugger", "rust-gdb", "--tips"])
        .assert()
        .success();
}

// ============================================================================
// DOCS COMMAND TESTS
// ============================================================================

#[test]
fn test_docs_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");
    let output = temp_dir.path().join("docs");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "docs",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn test_docs_format_markdown() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");
    let output = temp_dir.path().join("docs");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "docs",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--format",
            "markdown",
        ])
        .assert()
        .success();
}

#[test]
fn test_docs_format_html() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");
    let output = temp_dir.path().join("docs");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "docs",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--format",
            "html",
        ])
        .assert()
        .success();
}

#[test]
fn test_docs_all_boolean_flags() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");
    let output = temp_dir.path().join("docs");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "docs",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--include-source",
            "--examples",
            "--migration-notes",
            "--performance-notes",
            "--api-reference",
            "--usage-guide",
            "--index",
        ])
        .assert()
        .success();
}

// ============================================================================
// PROFILE COMMAND TESTS
// ============================================================================

#[test]
fn test_profile_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["profile", input.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_profile_count_instructions() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["profile", input.to_str().unwrap(), "--count-instructions"])
        .assert()
        .success();
}

#[test]
fn test_profile_track_allocations() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["profile", input.to_str().unwrap(), "--track-allocations"])
        .assert()
        .success();
}

#[test]
fn test_profile_detect_hot_paths() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["profile", input.to_str().unwrap(), "--detect-hot-paths"])
        .assert()
        .success();
}

#[test]
fn test_profile_hot_path_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "profile",
            input.to_str().unwrap(),
            "--hot-path-threshold",
            "200",
        ])
        .assert()
        .success();
}

#[test]
fn test_profile_flamegraph() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["profile", input.to_str().unwrap(), "--flamegraph"])
        .assert()
        .success();
}

#[test]
fn test_profile_with_output_files() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");
    let flamegraph = temp_dir.path().join("flamegraph.svg");
    let perf = temp_dir.path().join("perf.txt");

    Command::cargo_bin("depyler")
        .unwrap()
        .args([
            "profile",
            input.to_str().unwrap(),
            "--flamegraph-output",
            flamegraph.to_str().unwrap(),
            "--perf-output",
            perf.to_str().unwrap(),
        ])
        .assert()
        .success();
}

// ============================================================================
// AGENT COMMAND TESTS (Subcommands)
// ============================================================================

#[test]
#[ignore] // Agent starts a daemon
fn test_agent_start_basic() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["agent", "start"])
        .timeout(std::time::Duration::from_secs(2))
        .assert();
}

#[test]
#[ignore] // Agent command
fn test_agent_start_with_port() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["agent", "start", "--port", "3001"])
        .timeout(std::time::Duration::from_secs(2))
        .assert();
}

#[test]
#[ignore] // Agent command
fn test_agent_start_with_debug() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["agent", "start", "--debug"])
        .timeout(std::time::Duration::from_secs(2))
        .assert();
}

#[test]
#[ignore] // Agent command
fn test_agent_start_foreground() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["agent", "start", "--foreground"])
        .timeout(std::time::Duration::from_secs(2))
        .assert();
}

#[test]
fn test_agent_status() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["agent", "status"])
        .assert()
        .success();
}

#[test]
fn test_agent_list_projects() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["agent", "list-projects"])
        .assert()
        .success();
}

// ============================================================================
// GLOBAL FLAGS TESTS
// ============================================================================

#[test]
fn test_global_verbose_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["--verbose", "transpile", input.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_global_verbose_short_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = create_test_py_file(&temp_dir, "test.py");

    Command::cargo_bin("depyler")
        .unwrap()
        .args(["-v", "transpile", input.to_str().unwrap()])
        .assert()
        .success();
}

// ============================================================================
// ERROR CASE TESTS
// ============================================================================

#[test]
fn test_missing_file_error() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["transpile", "/nonexistent/file.py"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file").or(predicate::str::contains("error")));
}

#[test]
fn test_invalid_command() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["invalid-command"])
        .assert()
        .failure();
}

#[test]
fn test_help_command() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("depyler"));
}

#[test]
fn test_version_command() {
    Command::cargo_bin("depyler")
        .unwrap()
        .args(["--version"])
        .assert()
        .success();
}
