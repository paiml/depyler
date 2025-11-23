// DEPYLER-0303 Phase 1: Dictionary/HashMap Method Quick Wins Test
// Tests for:
// 1. &&str vs &str fixes (contains_key, remove)
// 2. Immutable HashMap parameters (mut keyword for insert, clear)

use depyler_core::DepylerPipeline;

#[test]
fn test_dict_insert_adds_mut() {
    let python_code = r#"
def add_entry(d: dict[str, int], key: str, value: int) -> dict[str, int]:
    d[key] = value
    return d
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should add mut keyword to d parameter
    assert!(
        rust_code.contains("mut d: HashMap<String, i32>"),
        "Should contain 'mut d: HashMap<String, i32>'"
    );
    assert!(
        rust_code.contains(".insert("),
        "Should contain .insert() call"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_dict_clear_adds_mut() {
    let python_code = r#"
def clear_dict(d: dict[str, int]) -> dict[str, int]:
    d.clear()
    return d
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should add mut keyword to d parameter
    assert!(
        rust_code.contains("mut d: HashMap<String, i32>"),
        "Should contain 'mut d: HashMap<String, i32>'"
    );
    assert!(
        rust_code.contains(".clear()"),
        "Should contain .clear() call"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_dict_pop_removes_double_ref() {
    let python_code = r#"
def pop_entry(d: dict[str, int], key: str) -> int:
    return d.pop(key, -1)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT have &&key (double reference)
    assert!(
        !rust_code.contains("&&key") && !rust_code.contains("& & key"),
        "Should NOT contain &&key double reference"
    );

    // Should have single reference or no reference
    // (depends on whether key is already &str or String parameter)
    assert!(
        rust_code.contains(".remove(key)") || rust_code.contains(".remove(&key)"),
        "Should contain .remove(key) or .remove(&key)"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_dict_contains_key_no_double_ref() {
    let python_code = r#"
def has_key(d: dict[str, int], key: str) -> bool:
    return key in d
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT have &&key (double reference)
    assert!(
        !rust_code.contains("&&key") && !rust_code.contains("& & key"),
        "Should NOT contain &&key double reference"
    );

    // DEPYLER-0449: Changed from .contains_key() to .get().is_some() for serde_json::Value compatibility
    // Accept both patterns as valid (they're semantically equivalent)
    let has_contains_key = rust_code.contains(".contains_key(key)") || rust_code.contains(".contains_key(&key)");
    let has_get_is_some = rust_code.contains(".get(&key).is_some()") || rust_code.contains(".get(key).is_some()");

    assert!(
        has_contains_key || has_get_is_some,
        "Should contain .contains_key() or .get().is_some() pattern.\nGenerated code:\n{}",
        rust_code
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_dict_combined_mutations() {
    // Test multiple mutating operations
    let python_code = r#"
def modify_dict(d: dict[str, int], k1: str, k2: str) -> dict[str, int]:
    d[k1] = 10
    if k2 in d:
        d.pop(k2)
    return d
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should add mut keyword
    assert!(
        rust_code.contains("mut d: HashMap<String, i32>"),
        "Should contain 'mut d: HashMap<String, i32>'"
    );

    // Should NOT have &&k2
    assert!(
        !rust_code.contains("&&k2") && !rust_code.contains("& & k2"),
        "Should NOT contain &&k2 double reference"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_dict_no_mut_when_not_mutated() {
    // Parameter should NOT have mut if not mutated
    let python_code = r#"
def get_value(d: dict[str, int], key: str) -> int:
    return d.get(key, 0)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT add mut keyword (read-only access)
    // Should contain HashMap type in signature
    assert!(
        rust_code.contains("HashMap"),
        "Should contain HashMap in signature"
    );

    // Should NOT have mut d
    assert!(
        !rust_code.contains("mut d:") && !rust_code.contains("mut d :"),
        "Should NOT contain 'mut d' for read-only parameter"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_dict_remove_in_conditional() {
    // Test that mut is detected even when method call is in conditional
    let python_code = r#"
def remove_if_exists(d: dict[str, int], key: str) -> dict[str, int]:
    if key in d:
        d.pop(key)
    return d
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should add mut keyword (mutation happens in if body)
    assert!(
        rust_code.contains("mut d: HashMap<String, i32>"),
        "Should contain 'mut d: HashMap<String, i32>'"
    );

    // Should NOT have &&key in contains_key or remove
    assert!(
        !rust_code.contains("&&key") && !rust_code.contains("& & key"),
        "Should NOT contain &&key double reference"
    );

    println!("Generated Rust code:\n{}", rust_code);
}
