// DEPYLER-0834: Variable Scope Hoisting for If/For Blocks
// RED PHASE TESTS - These tests should FAIL until implementation is complete
//
// Problem: In Python, variables defined inside if/for blocks escape to outer scope.
// Currently, Rust codegen only hoists variables assigned in BOTH branches,
// but variables assigned in ANY branch that are used AFTER the block should be hoisted.
//
// Example E0425 Error Pattern:
// ```python
// if condition:
//     var = value
// # var used here - E0425 in Rust: cannot find value `var` in this scope
// print(var)
// ```

use depyler_core::DepylerPipeline;

/// Helper to compile-check generated Rust code
fn compiles_successfully(rust_code: &str) -> bool {
    use std::io::Write;
    use std::process::Command;

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let rust_file = temp_dir.path().join("test_scope.rs");
    let mut file = std::fs::File::create(&rust_file).expect("Failed to create file");
    writeln!(file, "{}", rust_code).expect("Failed to write file");
    drop(file);

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "--emit=metadata", "-o"])
        .arg(temp_dir.path().join("test_scope.rmeta"))
        .arg(&rust_file)
        .output()
        .expect("Failed to run rustc");

    output.status.success()
}

/// Helper to check for E0425 specifically
fn has_e0425_error(rust_code: &str) -> bool {
    use std::io::Write;
    use std::process::Command;

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let rust_file = temp_dir.path().join("test_scope.rs");
    let mut file = std::fs::File::create(&rust_file).expect("Failed to create file");
    writeln!(file, "{}", rust_code).expect("Failed to write file");
    drop(file);

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "--emit=metadata", "-o"])
        .arg(temp_dir.path().join("test_scope.rmeta"))
        .arg(&rust_file)
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.contains("E0425") || stderr.contains("cannot find value")
}

// ============================================================================
// SECTION 1: Basic If-Only (No Else) Scope Hoisting
// ============================================================================

/// Test: Variable assigned in if-only block, used after block
/// Python: variable escapes if scope
/// Expected: Rust declares variable before if, initializes with Default
#[test]
fn test_depyler_0834_if_only_var_used_after() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_data(condition: bool) -> str:
    if condition:
        result = "found"
    return result
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    // Generated code should compile without E0425
    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error: cannot find value 'result'\n\nGenerated code:\n{}",
        rust_code
    );

    // Variable should be declared before the if block
    assert!(
        rust_code.contains("let mut result") || rust_code.contains("let result"),
        "Variable 'result' should be declared before if block\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Multiple variables assigned in if-only block, all used after
#[test]
fn test_depyler_0834_if_only_multiple_vars_used_after() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def calculate(flag: bool) -> int:
    if flag:
        x = 10
        y = 20
    return x + y
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Variable assigned in if-only block, used in subsequent if condition
#[test]
fn test_depyler_0834_if_only_var_used_in_later_condition() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_section(name: str) -> str:
    if name:
        current_section = name
    if current_section:
        return current_section
    return ""
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'current_section'\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 2: If-Else with Asymmetric Variables
// ============================================================================

/// Test: Variable only assigned in then branch, used after if-else
#[test]
fn test_depyler_0834_if_else_then_only_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_value(flag: bool) -> int:
    if flag:
        value = 42
    else:
        pass
    return value
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'value'\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Variable only assigned in else branch, used after if-else
#[test]
fn test_depyler_0834_if_else_else_only_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_fallback(flag: bool) -> str:
    if flag:
        pass
    else:
        fallback = "default"
    return fallback
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'fallback'\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 3: For Loop Scope Hoisting
// ============================================================================

/// Test: Variable assigned in for loop, used after loop
#[test]
fn test_depyler_0834_for_loop_var_used_after() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def find_last(items: list) -> int:
    for item in items:
        last = item
    return last
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'last'\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Variable assigned conditionally inside for loop, used after
#[test]
fn test_depyler_0834_for_loop_conditional_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def find_match(items: list, target: str) -> str:
    for item in items:
        if item == target:
            found = item
    return found
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'found'\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 4: While Loop Scope Hoisting
// ============================================================================

/// Test: Variable assigned in while loop, used after loop
#[test]
fn test_depyler_0834_while_loop_var_used_after() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_until(limit: int) -> int:
    i = 0
    while i < limit:
        processed = i * 2
        i = i + 1
    return processed
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'processed'\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 5: Nested Block Scope Hoisting
// ============================================================================

/// Test: Variable assigned in nested if inside for, used after for
#[test]
fn test_depyler_0834_nested_if_in_for_var_used_after() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def find_positive(items: list) -> int:
    for item in items:
        if item > 0:
            result = item
            break
    return result
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'result'\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Variable assigned in deeply nested blocks, used at function level
#[test]
fn test_depyler_0834_deeply_nested_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def deep_search(matrix: list, target: int) -> int:
    for row in matrix:
        for item in row:
            if item == target:
                found_value = item
    return found_value
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'found_value'\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 6: Real-World Patterns from Corpus
// ============================================================================

/// Test: Settings loader pattern - variable assigned in if, used in later if
/// Based on example_settings_loader E0425 error
#[test]
fn test_depyler_0834_settings_loader_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def parse_ini(lines: list) -> dict:
    result = {}
    for line in lines:
        if line.startswith("["):
            section_name = line
            current_section = section_name
        if current_section:
            result[current_section] = line
    return result
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error (settings loader pattern)\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Template engine pattern - variable assigned in if, used after
/// Based on example_template_engine E0425 error
#[test]
fn test_depyler_0834_template_engine_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def parse_block(line: str) -> str:
    if "for" in line:
        block_type = "for"
        block_var = "item"
        block_iterable = "items"
    return block_type + block_var + block_iterable
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error (template engine pattern)\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: CSV dialect pattern - variable assigned in for loop if, used after
/// Based on example_csv_dialect E0425 error
#[test]
fn test_depyler_0834_csv_dialect_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def detect_dialect(data: str) -> str:
    for line in data.split("\n"):
        if "," in line:
            lines = line
    return lines
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error (csv dialect pattern)\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 7: Edge Cases
// ============================================================================

/// Test: Variable with same name reassigned at different scopes
#[test]
fn test_depyler_0834_variable_reassignment_scopes() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_scope(flag: bool) -> int:
    x = 0
    if flag:
        x = 10
        y = x
    return y
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    // x is already declared, but y needs hoisting
    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'y'\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Variable used only if block was executed (common Python pattern)
#[test]
fn test_depyler_0834_optional_variable_access() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def maybe_get(data: list, index: int) -> int:
    if index < len(data):
        result = data[index]
    if result:
        return result
    return 0
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0425_error(&rust_code),
        "Generated code has E0425 error for 'result'\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 8: Compilation Verification
// ============================================================================

/// Meta-test: Ensure all generated code actually compiles
/// SLOW: Requires rustc compilation for each pattern
#[test]
#[ignore = "slow: requires rustc compilation for each pattern"]
fn test_depyler_0834_all_patterns_compile() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        // (name, python_code)
        (
            "if_only_basic",
            r#"
def f(cond: bool) -> str:
    if cond:
        x = "a"
    return x
"#,
        ),
        (
            "if_else_asymmetric",
            r#"
def f(cond: bool) -> int:
    if cond:
        x = 1
    else:
        pass
    return x
"#,
        ),
        (
            "for_loop_escape",
            r#"
from typing import List
def f(items: List[int]) -> int:
    for i in items:
        x = i
    return x
"#,
        ),
        (
            "while_loop_escape",
            r#"
def f(n: int) -> int:
    i = 0
    while i < n:
        x = i
        i = i + 1
    return x
"#,
        ),
    ];

    let mut failures = Vec::new();

    for (name, python_code) in test_cases {
        let result = pipeline.transpile(python_code);
        if let Ok(rust_code) = result {
            if !compiles_successfully(&rust_code) {
                failures.push((name, rust_code));
            }
        } else {
            failures.push((name, format!("Transpilation failed: {:?}", result.err())));
        }
    }

    assert!(
        failures.is_empty(),
        "The following test cases failed to compile:\n{}",
        failures
            .iter()
            .map(|(name, code)| format!("=== {} ===\n{}", name, code))
            .collect::<Vec<_>>()
            .join("\n\n")
    );
}
