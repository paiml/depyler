#![allow(non_snake_case)]
// DEPYLER-0303 Phase 3: Dictionary/HashMap Method Final Wins Test
// Tests for:
// 1. zip iterator ownership → .into_iter() for owned collections (1 error)
// 2. dict merge operator | → .extend() (1 error)
// 3. sum type inference → remove redundant .collect().iter() (1 error)

use depyler_core::DepylerPipeline;

// ========== Fix #6: zip iterator ownership ==========

#[test]
fn test_zip_uses_into_iter_for_owned_vecs() {
    let python_code = r#"
def create_from_lists(keys: list[str], values: list[int]) -> dict[str, int]:
    return dict(zip(keys, values))
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .into_iter() for owned collections
    assert!(
        rust_code.contains(".into_iter()"),
        "Should use .into_iter() for owned collections in zip()"
    );

    // Should NOT use .iter() which yields references
    assert!(
        !rust_code.contains("keys.iter().zip"),
        "Should NOT use .iter() for owned collections"
    );

    // Verify it compiles by checking for owned type
    // The result should be HashMap<String, i32>, not HashMap<&String, &i32>
    // Check the function definition specifically, not the entire file (which includes DepylerValue runtime)
    let fn_start = rust_code
        .find("fn create_from_lists")
        .expect("Function not found");
    let fn_end = rust_code[fn_start..]
        .find('}')
        .map(|i| fn_start + i)
        .unwrap_or(rust_code.len());
    let fn_code = &rust_code[fn_start..fn_end];
    assert!(
        !fn_code.contains("&&"),
        "Should not have double references (&&) from .iter() in function body"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_zip_with_both_owned_params() {
    let python_code = r#"
def pair_lists(a: list[str], b: list[int]) -> list[tuple[str, int]]:
    return list(zip(a, b))
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .into_iter() for both parameters
    assert!(
        rust_code.contains(".into_iter()"),
        "Should use .into_iter() for owned parameters"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== Fix #7: dict merge operator | ==========

#[test]
fn test_dict_merge_operator_translates_to_extend() {
    let python_code = r#"
def merge_with_pipe(d1: dict[str, int], d2: dict[str, int]) -> dict[str, int]:
    return d1 | d2
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should translate | operator to extend or clone+extend pattern
    // Note: Exact pattern depends on implementation, but should NOT use | operator
    assert!(
        !rust_code.contains("d1 | d2"),
        "Should NOT use | operator (not supported in Rust HashMap)"
    );

    // Should have some form of merge operation
    // This might be .extend(), .clone(), or a combination
    assert!(
        rust_code.contains(".extend(") || rust_code.contains(".clone("),
        "Should use .extend() or clone+extend pattern for dict merge"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== Fix #8: sum type inference ==========

#[test]
fn test_sum_on_dict_values_no_redundant_collect() {
    let python_code = r#"
def average_values(d: dict[str, int]) -> float:
    if len(d) == 0:
        return 0.0
    return sum(d.values()) / len(d)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT have redundant .collect().iter() pattern
    assert!(
        !rust_code.contains(".collect::<Vec<_>>().iter()"),
        "Should NOT have redundant .collect().iter() pattern"
    );

    // Should use .values() directly with .sum (note: may have turbofish .sum::<f64>())
    assert!(
        rust_code.contains(".values()") && rust_code.contains(".sum"),
        "Should use .values() and .sum together"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_sum_on_list_direct() {
    let python_code = r#"
def sum_list(nums: list[int]) -> int:
    return sum(nums)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .iter().sum directly, not .collect().iter().sum
    assert!(
        rust_code.contains(".iter()") && rust_code.contains(".sum"),
        "Should use .iter().sum directly"
    );

    assert!(
        !rust_code.contains(".collect::<Vec<_>>().iter()"),
        "Should NOT have redundant .collect().iter()"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== Integration Tests ==========

#[test]
fn test_phase3_all_fixes_combined() {
    // Test all three fixes together
    let python_code = r#"
def process_lists(keys: list[str], values: list[int]) -> float:
    d = dict(zip(keys, values))
    if len(d) == 0:
        return 0.0
    return sum(d.values()) / len(d)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Fix #6: zip should use .into_iter()
    assert!(
        rust_code.contains(".into_iter()"),
        "Should use .into_iter() for zip()"
    );

    // Fix #8: sum should not have redundant .collect().iter()
    assert!(
        !rust_code.contains(".collect::<Vec<_>>().iter().sum()"),
        "Should NOT have redundant .collect().iter() in sum()"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_regression_zip_with_literals() {
    // Ensure zip works with list literals too
    let python_code = r#"
def zip_literals() -> dict[str, int]:
    return dict(zip(["a", "b", "c"], [1, 2, 3]))
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // List literals are also owned, should use .into_iter()
    assert!(
        rust_code.contains(".into_iter()"),
        "Should use .into_iter() for list literals in zip()"
    );

    println!("Generated Rust code:\n{}", rust_code);
}
