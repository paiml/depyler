// DEPYLER-0303 Phase 2: Dictionary/HashMap Method Medium Wins Test
// Tests for:
// 1. Cow<str> over-complication → simple &str (1 error)
// 2. Option unwrapping with None checks (1 error)
// 3. Iterator reference cloning in for loops (2 errors)

use depyler_core::DepylerPipeline;

// ========== Cow<str> Fix Tests ==========

#[test]
fn test_has_key_uses_simple_str_not_cow() {
    let python_code = r#"
def has_key(d: dict[str, int], key: str) -> bool:
    return key in d
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT use Cow<'_, str>
    assert!(
        !rust_code.contains("Cow<"),
        "Should NOT use Cow for simple string parameter"
    );

    // Should use simple &str
    assert!(
        rust_code.contains("key: &"),
        "Should use &str for string parameter"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_param_not_escaping() {
    let python_code = r#"
def find_value(d: dict[str, int], search_key: str) -> int:
    for k, v in d.items():
        if k == search_key:
            return v
    return -1
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // String parameter used in comparison shouldn't use Cow
    assert!(
        !rust_code.contains("Cow<"),
        "Should NOT use Cow when string doesn't escape"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== Option Unwrapping Fix Tests ==========

#[test]
fn test_get_without_default_returns_option() {
    let python_code = r#"
def get_without_default(d: dict[str, int], key: str) -> int | None:
    result = d.get(key)
    if result is None:
        return None
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT unwrap_or_default when return type is Optional
    assert!(
        !rust_code.contains("unwrap_or_default()") || rust_code.contains("Option<"),
        "Should NOT unwrap when function returns Optional"
    );

    // Should have .cloned() to get Option
    assert!(
        rust_code.contains(".cloned()"),
        "Should use .cloned() to get Option"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_get_with_default_still_unwraps() {
    let python_code = r#"
def get_with_default(d: dict[str, int], key: str) -> int:
    return d.get(key, 0)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // With default value, should still unwrap
    assert!(
        rust_code.contains(".unwrap_or(") || rust_code.contains(".unwrap_or_default()"),
        "Should unwrap when default value provided"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_get_non_optional_return_unwraps() {
    let python_code = r#"
def get_or_zero(d: dict[str, int], key: str) -> int:
    result = d.get(key)
    if result is None:
        return 0
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Non-optional return should unwrap
    // Note: This may not be perfect yet, but should not break
    assert!(
        rust_code.contains("i32") && !rust_code.contains("Option<i32"),
        "Return type should be i32, not Option<i32>"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== Iterator Reference Cloning Fix Tests ==========

#[test]
fn test_for_loop_tuple_both_vars_used() {
    let python_code = r#"
def sum_dict(d: dict[str, int]) -> int:
    total = 0
    for k, v in d.items():
        total += v
        print(k)
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Both k and v are used, so pattern should be (k, v) not (_k, v)
    assert!(
        rust_code.contains("for (k, v)"),
        "Should use (k, v) when both variables are used"
    );

    // Should NOT have underscore prefix
    assert!(
        !rust_code.contains("for (_k, v)"),
        "Should NOT use (_k, v) when k is used"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_for_loop_key_used_in_index() {
    let python_code = r#"
def update_dict(d1: dict[str, int], d2: dict[str, int]) -> None:
    for k, v in d2.items():
        d1[k] = v
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // k is used in assignment target (d1[k]), so should be (k, v)
    assert!(
        rust_code.contains("for (k, v)"),
        "Should use (k, v) when k is used in assignment target"
    );

    // Should NOT have underscore prefix
    assert!(
        !rust_code.contains("for (_k,"),
        "Should NOT use _k when k is used in d1[k]"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_for_loop_only_value_used() {
    let python_code = r#"
def sum_values(d: dict[str, int]) -> int:
    total = 0
    for k, v in d.items():
        total += v
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // k is unused, v is used, so pattern should be (_k, v)
    assert!(
        rust_code.contains("for (_k, v)") || rust_code.contains("for (_, v)"),
        "Should use (_k, v) when k is unused"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_for_loop_items_clones_correctly() {
    let python_code = r#"
def copy_keys(d: dict[str, int]) -> list[str]:
    keys = []
    for k, v in d.items():
        keys.append(k)
    return keys
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .iter().map(...).collect() for items()
    assert!(
        rust_code.contains(".iter()") && rust_code.contains(".map("),
        "Should use .iter().map() for dict.items()"
    );

    // Should clone both k and v
    assert!(
        rust_code.contains("k.clone()") && rust_code.contains("v.clone()"),
        "Should clone both k and v in items()"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== Integration Tests ==========

#[test]
fn test_phase2_all_fixes_combined() {
    // Test all three fixes together
    let python_code = r#"
def merge_and_get(d1: dict[str, int], d2: dict[str, int], key: str) -> int | None:
    result = {}
    for k, v in d1.items():
        result[k] = v
    for k, v in d2.items():
        result[k] = v
    return result.get(key)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Fix #5: String param should not use Cow
    assert!(
        !rust_code.contains("Cow<"),
        "Should NOT use Cow for string parameter"
    );

    // Fix #3: Optional return should not unwrap unnecessarily
    assert!(
        rust_code.contains(".cloned()") && !rust_code.contains(".cloned().unwrap_or_default()"),
        "Should return Option directly, not unwrap"
    );

    // Fix #4: For loop should use (k, v) not (_k, v) when k is used
    assert!(
        rust_code.contains("for (k, v)"),
        "Should use (k, v) when both variables are used"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_regression_dict_operations_still_work() {
    // Ensure our fixes don't break existing functionality
    let python_code = r#"
def dict_ops(d: dict[str, int]) -> None:
    d["new_key"] = 42
    d.pop("old_key", None)
    if "check_key" in d:
        print(d["check_key"])
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Basic dict operations should still work
    assert!(
        rust_code.contains(".insert("),
        "Should still use .insert() for dict assignment"
    );
    assert!(
        rust_code.contains(".remove(") || rust_code.contains(".pop("),
        "Should still handle .pop() method"
    );
    assert!(
        rust_code.contains(".contains_key("),
        "Should still use .contains_key() for membership test"
    );

    println!("Generated Rust code:\n{}", rust_code);
}
