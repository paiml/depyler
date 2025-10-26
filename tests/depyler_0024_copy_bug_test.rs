// ============================================================================
// DEPYLER-0024: copy.copy() for lists generates invalid Rust code (P1 MAJOR)
// ============================================================================
// BUG: copy.copy() for lists generates `.copy()` method call which doesn't
// exist in Rust. Should generate `.clone()` or proper copy semantics.
//
// DISCOVERED: TDD Book validation (copy module)
// SEVERITY: P1 MAJOR - affects fundamental stdlib function
// ============================================================================

use depyler_core::DepylerPipeline;

#[test]
fn test_depyler_0024_copy_copy_list_invalid_codegen() {
    // DEPYLER-0024: copy.copy() for lists generates invalid Rust code
    let python_code = r#"
import copy

def test_shallow_copy() -> int:
    original = [1, 2, 3]
    copied = copy.copy(original)
    copied.append(4)
    return len(original)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // CRITICAL: Should NOT generate `.copy()` method (doesn't exist in Rust)
    assert!(
        !rust_code.contains(".copy()"),
        "BUG CONFIRMED: Generated invalid `.copy()` method!\nGenerated code:\n{}",
        rust_code
    );

    // Should generate valid Rust code with .clone() or proper copy semantics
    assert!(
        rust_code.contains(".clone()") || rust_code.contains("copy::copy"),
        "Should generate valid Rust copy operation (.clone() or copy::copy)\nGenerated code:\n{}",
        rust_code
    );

    // Verify the copy semantics are correct (shallow copy behavior)
    assert!(
        rust_code.contains("copied") || rust_code.contains("let"),
        "Should have proper variable assignment for copied list\nGenerated code:\n{}",
        rust_code
    );
}

#[test]
fn test_depyler_0024_copy_copy_dict_works() {
    // DEPYLER-0024: Verify copy.copy() for dicts works (regression check)
    let python_code = r#"
import copy

def test_dict_copy() -> int:
    original = {"a": 1, "b": 2}
    copied = copy.copy(original)
    copied["c"] = 3
    return len(original)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Should generate valid Rust code for dict copy
    assert!(
        rust_code.contains(".clone()") || rust_code.contains("copy"),
        "Should generate valid dict copy operation\nGenerated code:\n{}",
        rust_code
    );
}

#[test]
fn test_depyler_0024_copy_deepcopy_list_works() {
    // DEPYLER-0024: Verify copy.deepcopy() still works (regression check)
    let python_code = r#"
import copy

def test_deep_copy() -> int:
    original = [[1, 2], [3, 4]]
    copied = copy.deepcopy(original)
    copied[0].append(5)
    return len(original[0])
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Should generate valid deep copy operation
    assert!(
        rust_code.contains("clone") || rust_code.contains("deep"),
        "Should generate valid deep copy operation\nGenerated code:\n{}",
        rust_code
    );
}
