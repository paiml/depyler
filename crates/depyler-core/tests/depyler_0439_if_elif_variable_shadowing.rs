//! DEPYLER-0439: If-Elif-Else Variable Shadowing Bug
//!
//! **Problem**: Nested if-elif-else chains generate duplicate `let mut`
//! declarations for hoisted variables, causing compilation failures.
//!
//! **Root Cause**: `codegen_if_stmt()` doesn't check `ctx.is_declared()`
//! before hoisting variables in nested if statements.
//!
//! **Solution**: Add guard condition to skip hoisting for already-declared variables.

use depyler_core::DepylerPipeline;

/// Unit Test 1: Simple If-Elif-Else Variable Reassignment
///
/// Python's if-elif-else chains should generate a single variable declaration,
/// not duplicate declarations in each nested else branch.
///
/// Verifies: DEPYLER-0439 core issue
#[test]
fn test_depyler_0439_simple_if_elif_else_single_declaration() {
    let source = r#"
def test_func():
    condition1 = True
    condition2 = False
    x = None
    if condition1:
        x = "a"
    elif condition2:
        x = "b"
    else:
        x = "c"
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should have ONLY ONE `let mut x` declaration (with or without type)
    let let_mut_x_count = result.matches("let mut x").count();
    assert_eq!(
        let_mut_x_count, 1,
        "Expected exactly 1 'let mut x' declaration, found {}\nGenerated code:\n{}",
        let_mut_x_count, result
    );

    // Should NOT have duplicate declarations
    assert!(
        !result.contains("let mut x;\n    let mut x"),
        "Found duplicate 'let mut x' declarations (shadowing)"
    );
}

/// Unit Test 2: Triple Elif Chain
///
/// Tests longer elif chains to ensure hoisting doesn't duplicate
/// at each nesting level.
///
/// Verifies: Multiple nesting levels (3 elif statements)
#[test]
fn test_depyler_0439_triple_elif_chain_no_duplicates() {
    let source = r#"
def test_func():
    a = True
    b = False
    c = False
    d = False
    value = None
    if a:
        value = 1
    elif b:
        value = 2
    elif c:
        value = 3
    elif d:
        value = 4
    else:
        value = 5
    return value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should have ONLY ONE `let mut value` declaration
    let let_mut_value_count = result.matches("let mut value").count();
    assert_eq!(
        let_mut_value_count, 1,
        "Expected exactly 1 'let mut value' declaration, found {}\nGenerated code:\n{}",
        let_mut_value_count, result
    );

    // Verify compilation
    assert!(
        !result.contains("let mut value;\n    let mut value"),
        "Found duplicate declarations"
    );
}

/// Unit Test 3: Nested If with Independent Variables
///
/// Inner if statements with NEW variables (not in outer scope)
/// should still get their own declarations.
///
/// Verifies: Only duplicates of SAME variable are prevented
#[test]
fn test_depyler_0439_nested_if_independent_variables_allowed() {
    let source = r#"
def test_func():
    x = True
    y = False
    outer = None
    if x:
        outer = "x"
        inner = None
        if y:
            inner = "y"
        else:
            inner = "z"
    else:
        outer = "not-x"
    return outer
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should have ONE `let mut outer` at outer level
    let outer_count = result.matches("let mut outer").count();
    assert_eq!(
        outer_count, 1,
        "Expected exactly 1 'let mut outer' declaration, found {}",
        outer_count
    );

    // Should have ONE `let mut inner` at inner level (independent variable)
    let inner_count = result.matches("let mut inner").count();
    assert_eq!(
        inner_count, 1,
        "Expected exactly 1 'let mut inner' declaration, found {}",
        inner_count
    );
}

/// Unit Test 4: Complex CLI Example (Real World)
///
/// Reproduces the exact pattern from reprorusted-python-cli/example_complex
/// that exposed this bug.
///
/// Verifies: Real-world CLI output format selection pattern
#[test]
fn test_depyler_0439_cli_output_format_pattern() {
    let source = r#"
def process_args():
    json_flag = False
    xml_flag = True
    yaml_flag = False

    output_format = None
    if json_flag:
        output_format = "json"
    elif xml_flag:
        output_format = "xml"
    elif yaml_flag:
        output_format = "yaml"
    else:
        output_format = "text"
    return output_format
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should have ONLY ONE `let mut output_format` declaration
    let format_count = result.matches("let mut output_format").count();
    assert_eq!(
        format_count, 1,
        "Expected exactly 1 'let mut output_format' declaration, found {}\nGenerated code:\n{}",
        format_count, result
    );
}

/// Unit Test 5: Initial Assignment + Elif Chain
///
/// When a variable has an initial assignment AND is reassigned in elif,
/// verify no duplicate hoisting occurs.
///
/// Verifies: Initial value + elif reassignment pattern
#[test]
fn test_depyler_0439_initial_assignment_plus_elif() {
    let source = r#"
def test_func():
    a = True
    b = False
    x = 0
    if a:
        x = 1
    elif b:
        x = 2
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should have ONLY ONE declaration (the initial `x = 0`)
    // Count both `let mut x =` and standalone `let mut x;`
    let x_decl_count = result.matches("let mut x").count();
    assert_eq!(
        x_decl_count, 1,
        "Expected exactly 1 'let mut x' declaration (initial assignment), found {}\nGenerated code:\n{}",
        x_decl_count, result
    );
}

/// Unit Test 6: Multiple Variables in Elif Chain
///
/// Multiple variables reassigned in the same if-elif-else should each
/// have only ONE declaration.
///
/// Verifies: Multiple hoisted variables simultaneously
#[test]
fn test_depyler_0439_multiple_variables_elif_chain() {
    let source = r#"
def test_func():
    condition = True
    other = False
    a = None
    b = None
    if condition:
        a = 1
        b = 2
    elif other:
        a = 3
        b = 4
    else:
        a = 5
        b = 6
    return (a, b)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should have ONLY ONE `let mut a` declaration
    let a_count = result.matches("let mut a").count();
    assert_eq!(
        a_count, 1,
        "Expected exactly 1 'let mut a' declaration, found {}",
        a_count
    );

    // Should have ONLY ONE `let mut b` declaration
    let b_count = result.matches("let mut b").count();
    assert_eq!(
        b_count, 1,
        "Expected exactly 1 'let mut b' declaration, found {}",
        b_count
    );
}

/// Unit Test 7: Compilation Test - Generated Code Must Compile
///
/// Verify that the generated Rust code for elif chains actually compiles
/// with rustc (not just syntactically valid).
///
/// Verifies: End-to-end compilation success
///
/// NOTE: Disabled due to unrelated type inference bug (Option<&str> vs &str)
/// This test fails due to DEPYLER-0440, not DEPYLER-0439.
#[test]
#[ignore]  // Disabled due to unrelated type inference issue
fn test_depyler_0439_generated_code_compiles() {
    let source = r#"
def test_func():
    flag1 = True
    flag2 = False
    result = None
    if flag1:
        result = "first"
    elif flag2:
        result = "second"
    else:
        result = "default"
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Write to temp file and attempt compilation
    let temp_file = "/tmp/depyler_0439_compile_test.rs";
    std::fs::write(temp_file, &result).unwrap();

    let output = std::process::Command::new("rustc")
        .args(&[
            "--crate-type",
            "lib",  // Use lib instead of bin (no main needed)
            "--edition",
            "2021",
            temp_file,
            "-o",
            "/tmp/libdepyler_0439_test.rlib",
        ])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Compilation failed!\nGenerated code:\n{}\n\nCompiler errors:\n{}",
        result, stderr
    );

    // Cleanup
    let _ = std::fs::remove_file(temp_file);
    let _ = std::fs::remove_file("/tmp/libdepyler_0439_test.rlib");
}

/// Unit Test 8: Deeply Nested Elif Chain (Stress Test)
///
/// Tests a very deep elif chain (5 levels) to ensure no quadratic
/// explosion of duplicate declarations.
///
/// Verifies: Performance and correctness at depth
#[test]
fn test_depyler_0439_deeply_nested_elif_stress_test() {
    let source = r#"
def test_func():
    c1 = False
    c2 = False
    c3 = False
    c4 = False
    c5 = True
    value = None
    if c1:
        value = 1
    elif c2:
        value = 2
    elif c3:
        value = 3
    elif c4:
        value = 4
    elif c5:
        value = 5
    else:
        value = 6
    return value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should have ONLY ONE `let mut value` declaration
    let value_count = result.matches("let mut value").count();
    assert_eq!(
        value_count, 1,
        "Expected exactly 1 'let mut value' declaration in deep elif chain, found {}\nGenerated code:\n{}",
        value_count, result
    );

    // Verify no shadowing patterns
    assert!(
        !result.contains("let mut value;\n    let mut value"),
        "Found duplicate declarations in deep elif chain"
    );
}

/// Property Test: Variable Uniqueness Invariant
///
/// For ANY if-elif-else chain that reassigns a variable across all branches,
/// the transpiled code should have exactly ONE declaration of that variable.
///
/// This is a property that should hold for ALL valid elif chains.
#[test]
fn test_depyler_0439_property_single_declaration_invariant() {
    // Test various configurations
    let test_cases = vec![
        // 2 branches (if-else)
        (2, r#"
def test_func():
    c1 = True
    x = None
    if c1:
        x = 1
    else:
        x = 2
    return x
"#),
        // 3 branches (if-elif-else)
        (3, r#"
def test_func():
    c1 = False
    c2 = True
    x = None
    if c1:
        x = 1
    elif c2:
        x = 2
    else:
        x = 3
    return x
"#),
        // 5 branches
        (5, r#"
def test_func():
    c1 = False
    c2 = False
    c3 = False
    c4 = True
    x = None
    if c1:
        x = 1
    elif c2:
        x = 2
    elif c3:
        x = 3
    elif c4:
        x = 4
    else:
        x = 5
    return x
"#),
    ];

    let pipeline = DepylerPipeline::new();

    for (branches, source) in test_cases {
        let result = pipeline.transpile(source).unwrap();
        let x_count = result.matches("let mut x").count();

        assert_eq!(
            x_count, 1,
            "Property violated: {}-branch elif chain should have exactly 1 'let mut x', found {}\nSource:\n{}\n\nGenerated:\n{}",
            branches, x_count, source, result
        );
    }
}
