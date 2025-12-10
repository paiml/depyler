#![allow(non_snake_case)]
// DEPYLER-0306: Nested 2D Array Indexing Fix
// Tests for fixing malformed code generation in nested loops with 2D indexing
// Issue: range(len(matrix[i])) generated invalid syntax with block expressions

use depyler_core::DepylerPipeline;

// ========== Basic Nested 2D Indexing Tests ==========

#[test]
fn test_nested_2d_find_first_match() {
    let python_code = r#"
def find_first_match(matrix: list[list[int]], target: int) -> tuple[int, int]:
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                return (i, j)
    return (-1, -1)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT contain block expressions in range
    assert!(
        !rust_code.contains("for j in 0..{"),
        "Should NOT use block expression in range"
    );

    // Should use inline .get() expression
    assert!(
        rust_code.contains(".get(i as usize)"),
        "Should use inline .get() for indexing"
    );

    // Should have valid range expression - check for matrix.get() in the for loop context
    // Note: Formatting may split this across lines, so check semantically
    let has_valid_range = rust_code.contains("for j in 0..matrix")
        && rust_code.contains(".get(i as usize)");
    assert!(
        has_valid_range,
        "Should have valid range expression with matrix.get()\nGenerated:\n{}",
        rust_code
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_nested_2d_count_matches() {
    let python_code = r#"
def count_matches_in_matrix(matrix: list[list[int]], target: int) -> int:
    count = 0
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                count = count + 1
    return count
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT contain block expressions in range
    assert!(
        !rust_code.contains("for j in 0..{"),
        "Should NOT use block expression in range"
    );

    // Should use inline .get() expression
    assert!(
        rust_code.contains(".get(i as usize)"),
        "Should use inline .get() for indexing"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_nested_2d_sum_matrix() {
    let python_code = r#"
def sum_matrix(matrix: list[list[int]]) -> int:
    total = 0
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            total = total + matrix[i][j]
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT contain block expressions in range
    assert!(
        !rust_code.contains("for j in 0..{"),
        "Should NOT use block expression in range"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== 3D Indexing Tests ==========

#[test]
fn test_3d_nested_indexing() {
    let python_code = r#"
def sum_3d(cube: list[list[list[int]]]) -> int:
    total = 0
    for i in range(len(cube)):
        for j in range(len(cube[i])):
            for k in range(len(cube[i][j])):
                total = total + cube[i][j][k]
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT contain block expressions in range
    assert!(
        !rust_code.contains("for j in 0..{"),
        "Should NOT use block expression in j loop"
    );
    assert!(
        !rust_code.contains("for k in 0..{"),
        "Should NOT use block expression in k loop"
    );

    // Should use inline .get() expressions
    assert!(
        rust_code.contains(".get(i as usize)"),
        "Should use inline .get() for i indexing"
    );
    assert!(
        rust_code.contains(".get(j as usize)"),
        "Should use inline .get() for j indexing"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== Regression Tests ==========

#[test]
fn test_regression_diagonal_access_still_works() {
    // Matrix[i][i] should still work (doesn't use range expression)
    let python_code = r#"
def sum_diagonal(matrix: list[list[int]]) -> int:
    total = 0
    for i in range(len(matrix)):
        total = total + matrix[i][i]
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use inline .get() for simple variable indices
    assert!(
        rust_code.contains(".get(i as usize)"),
        "Should use inline .get() for simple variable index"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_regression_complex_index_still_uses_block() {
    // Complex expressions like arr[i + 1] should still use block with negative index handling
    let python_code = r#"
def access_next(arr: list[int], i: int) -> int:
    return arr[i + 1]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Complex expression (i + 1) should still use block for negative index handling
    // This ensures we didn't break the original functionality
    assert!(
        rust_code.contains("let base =") || rust_code.contains("let idx"),
        "Complex expressions should still use block with negative index handling"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_regression_negative_literal_index() {
    // Negative literals like arr[-1] should still work
    let python_code = r#"
def get_last(arr: list[int]) -> int:
    return arr[-1]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Negative literal should use special handling
    assert!(
        rust_code.contains("saturating_sub") || rust_code.contains("len()"),
        "Negative literal indices should use special handling"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== Integration Tests ==========

#[test]
fn test_matrix_transpose() {
    let python_code = r#"
def transpose(matrix: list[list[int]]) -> list[list[int]]:
    if len(matrix) == 0:
        return []
    rows = len(matrix)
    cols = len(matrix[0])
    result: list[list[int]] = []
    for j in range(cols):
        row: list[int] = []
        for i in range(rows):
            row.append(matrix[i][j])
        result.append(row)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT contain block expressions in range
    assert!(
        !rust_code.contains("for j in 0..{"),
        "Should NOT use block expression in j range"
    );
    assert!(
        !rust_code.contains("for i in 0..{"),
        "Should NOT use block expression in i range"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_matrix_multiply() {
    let python_code = r#"
def matrix_multiply(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    rows_a = len(a)
    cols_a = len(a[0])
    cols_b = len(b[0])
    result: list[list[int]] = []
    for i in range(rows_a):
        row: list[int] = []
        for j in range(cols_b):
            total = 0
            for k in range(cols_a):
                total = total + a[i][k] * b[k][j]
            row.append(total)
        result.append(row)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT contain block expressions in any range
    assert!(
        !rust_code.contains("for i in 0..{")
            && !rust_code.contains("for j in 0..{")
            && !rust_code.contains("for k in 0..{"),
        "Should NOT use block expressions in any range"
    );

    println!("Generated Rust code:\n{}", rust_code);
}
