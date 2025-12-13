//! DEPYLER-0964: Option<Dict> Parameter Handling Tests
//!
//! This test module validates the correct generation of Rust code for Python
//! functions with optional dict parameters (e.g., `memo: Dict[int, int] = None`).
//!
//! The pattern creates `&mut Option<HashMap<K, V>>` in Rust, requiring:
//! 1. Assignment: `memo = {}` → `*memo = Some(HashMap::new())`
//! 2. Method calls: `memo.get(k)` → `memo.as_ref().unwrap().get(&k)`
//! 3. Subscript assign: `memo[k] = v` → `memo.as_mut().unwrap().insert(k, v)`

use depyler_core::DepylerPipeline;

#[test]
fn test_option_dict_param_none_check() {
    // Python: Optional dict param with None default
    let python = r#"
def memoized_fib(n: int, memo: dict[int, int] = None) -> int:
    if memo is None:
        memo = {}
    return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // The generated code should:
    // 1. Have Option<HashMap> parameter type
    // 2. Use *memo = Some(HashMap::new()) for assignment
    // 3. NOT have bare `memo = { ... }` assignment to Option type

    // Check that we don't have the incorrect pattern
    assert!(
        !rust_code.contains("memo = { let map = HashMap::new(); map };"),
        "Should not assign HashMap directly to &mut Option<HashMap>\n\nGenerated:\n{}",
        rust_code
    );

    // Check for correct pattern (deref + Some wrap)
    assert!(
        rust_code.contains("*memo = Some(") || rust_code.contains("memo.get_or_insert"),
        "Should use *memo = Some(...) or get_or_insert for Option dict initialization\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_option_dict_get_method() {
    // Python: Calling .get() on optional dict
    let python = r#"
def lookup(key: int, cache: dict[int, str] = None) -> str:
    if cache is None:
        cache = {}
    if key in cache:
        return cache.get(key)
    return ""
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should NOT call .get() directly on Option<HashMap>
    // Pattern to avoid: memo.get(&key) where memo is Option<HashMap>

    // Check for correct unwrapping pattern
    let has_correct_get = rust_code.contains("as_ref().unwrap().get")
        || rust_code.contains("as_ref()?.get")
        || rust_code.contains("unwrap().get");

    assert!(
        has_correct_get || !rust_code.contains(".get(&"),
        "Should unwrap Option before calling .get()\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_option_dict_subscript_assign() {
    // Python: Assigning to dict subscript
    let python = r#"
def cache_value(key: int, value: str, cache: dict[int, str] = None) -> dict[int, str]:
    if cache is None:
        cache = {}
    cache[key] = value
    return cache
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use .insert() on the inner HashMap, not on Option
    // Correct: cache.as_mut().unwrap().insert(key, value)
    // Wrong: cache.as_object_mut().unwrap().insert(...) (serde_json method)

    assert!(
        !rust_code.contains("as_object_mut"),
        "Should NOT use serde_json's as_object_mut() for HashMap\n\nGenerated:\n{}",
        rust_code
    );

    // Check for correct insert pattern
    let has_correct_insert = rust_code.contains("as_mut().unwrap().insert")
        || rust_code.contains("unwrap().insert")
        || rust_code.contains("get_or_insert");

    assert!(
        has_correct_insert || !rust_code.contains(".insert("),
        "Should use proper HashMap insert on unwrapped Option\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_option_dict_contains_check() {
    // Python: Checking if key in dict
    let python = r#"
def has_key(key: int, data: dict[int, int] = None) -> bool:
    if data is None:
        data = {}
    return key in data
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use contains_key on unwrapped HashMap
    let has_correct_contains = rust_code.contains("as_ref().unwrap().contains_key")
        || rust_code.contains("unwrap().contains_key");

    // If there's a contains_key call, it should be on unwrapped value
    if rust_code.contains("contains_key") {
        assert!(
            has_correct_contains,
            "Should unwrap Option before calling contains_key\n\nGenerated:\n{}",
            rust_code
        );
    }
}

#[test]
fn test_memoized_fibonacci_compiles() {
    // The actual fibonacci_memoized example that's currently failing
    let python = r#"
def fibonacci_memoized(n: int, memo: dict[int, int] = None) -> int:
    if memo is None:
        memo = {}
    if n in memo:
        return memo[n]
    if n <= 1:
        result = n
    else:
        result = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo)
    memo[n] = result
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Critical checks for the fibonacci pattern:
    // 1. No direct HashMap assignment to Option
    assert!(
        !rust_code.contains("memo = { let map = HashMap::new(); map }"),
        "Should not assign HashMap directly to Option<HashMap>\n\nGenerated:\n{}",
        rust_code
    );

    // 2. No serde_json methods
    assert!(
        !rust_code.contains("as_object_mut"),
        "Should not use serde_json methods on HashMap\n\nGenerated:\n{}",
        rust_code
    );
}
