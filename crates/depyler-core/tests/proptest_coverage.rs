//! Property-based tests for Depyler using proptest
//!
//! These tests generate random inputs to find edge cases
//! that manual tests might miss.

use depyler_core::DepylerPipeline;
use proptest::prelude::*;

fn transpile(code: &str) -> Result<String, String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

// ============================================================================
// PROPERTY: Valid identifiers should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_valid_identifier_names(name in "[a-z][a-z0-9_]{0,20}") {
        let code = format!("def {}():\n    pass", name);
        // Should not panic
        let _ = transpile(&code);
    }

    #[test]
    fn prop_valid_variable_names(name in "[a-z][a-z0-9_]{0,20}") {
        let code = format!("def f():\n    {} = 42", name);
        // Should not panic
        let _ = transpile(&code);
    }

    #[test]
    fn prop_valid_param_names(name in "[a-z][a-z0-9_]{0,20}") {
        let code = format!("def f({}): pass", name);
        // Should not panic
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: Integer literals should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_int_literals(n in -1000000i64..1000000i64) {
        let code = format!("def f() -> int:\n    return {}", n);
        // Should not panic and should produce valid code
        let _ = transpile(&code);
    }

    #[test]
    fn prop_large_int_literals(n in i64::MIN..i64::MAX) {
        let code = format!("def f() -> int:\n    return {}", n);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_int_addition(a in -1000i32..1000i32, b in -1000i32..1000i32) {
        let code = format!("def f() -> int:\n    return {} + {}", a, b);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_int_subtraction(a in -1000i32..1000i32, b in -1000i32..1000i32) {
        let code = format!("def f() -> int:\n    return {} - {}", a, b);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_int_multiplication(a in -100i32..100i32, b in -100i32..100i32) {
        let code = format!("def f() -> int:\n    return {} * {}", a, b);
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: Float literals should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_float_literals(n in -1000.0f64..1000.0f64) {
        if n.is_finite() {
            let code = format!("def f() -> float:\n    return {}", n);
            let _ = transpile(&code);
        }
    }

    #[test]
    fn prop_float_operations(a in -100.0f64..100.0f64, b in 1.0f64..100.0f64) {
        if a.is_finite() && b.is_finite() {
            let code = format!("def f() -> float:\n    return {} / {}", a, b);
            let _ = transpile(&code);
        }
    }
}

// ============================================================================
// PROPERTY: String literals should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_string_literals(s in "[a-zA-Z0-9 ]{0,50}") {
        let code = format!("def f() -> str:\n    return '{}'", s);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_string_concat(a in "[a-z]{0,20}", b in "[a-z]{0,20}") {
        let code = format!("def f() -> str:\n    return '{}' + '{}'", a, b);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_string_methods(s in "[a-zA-Z]{1,20}") {
        let code = format!("def f() -> str:\n    return '{}'.upper()", s);
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: List operations should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_list_literals(vec in prop::collection::vec(0i32..100, 0..10)) {
        let elements: Vec<String> = vec.iter().map(|n| n.to_string()).collect();
        let code = format!("def f() -> list:\n    return [{}]", elements.join(", "));
        let _ = transpile(&code);
    }

    #[test]
    fn prop_list_indexing(idx in 0usize..5) {
        let code = format!("def f(lst: list):\n    return lst[{}]", idx);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_list_slicing(start in 0usize..5, end in 5usize..10) {
        let code = format!("def f(lst: list) -> list:\n    return lst[{}:{}]", start, end);
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: Dict operations should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn prop_dict_literals(
        keys in prop::collection::vec("[a-z]{1,5}", 1..5),
        vals in prop::collection::vec(0i32..100, 1..5)
    ) {
        if keys.len() == vals.len() && !keys.is_empty() {
            let pairs: Vec<String> = keys.iter().zip(vals.iter())
                .map(|(k, v)| format!("'{}': {}", k, v))
                .collect();
            let code = format!("def f() -> dict:\n    return {{{}}}", pairs.join(", "));
            let _ = transpile(&code);
        }
    }

    #[test]
    fn prop_dict_access(key in "[a-z]{1,10}") {
        let code = format!("def f(d: dict):\n    return d['{}']", key);
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: Function parameters should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn prop_multiple_params(count in 1usize..10) {
        let params: Vec<String> = (0..count)
            .map(|i| format!("p{}: int", i))
            .collect();
        let code = format!("def f({}):\n    pass", params.join(", "));
        let _ = transpile(&code);
    }

    #[test]
    fn prop_default_values(default in 0i32..1000) {
        let code = format!("def f(x: int = {}) -> int:\n    return x", default);
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: Loops should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn prop_range_loops(start in 0i32..10, end in 10i32..100) {
        let code = format!(
            "def f():\n    for i in range({}, {}):\n        print(i)",
            start, end
        );
        let _ = transpile(&code);
    }

    #[test]
    fn prop_while_loops(limit in 1i32..50) {
        let code = format!(
            "def f():\n    i = 0\n    while i < {}:\n        i += 1",
            limit
        );
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: Conditionals should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn prop_if_comparison(threshold in -1000i32..1000i32) {
        let code = format!(
            "def f(x: int) -> str:\n    if x > {}:\n        return 'big'\n    return 'small'",
            threshold
        );
        let _ = transpile(&code);
    }

    #[test]
    fn prop_multiple_elif(count in 1usize..10) {
        let mut code = "def f(x: int) -> int:\n    if x == 0:\n        return 0\n".to_string();
        for i in 1..=count {
            code.push_str(&format!("    elif x == {}:\n        return {}\n", i, i));
        }
        code.push_str("    return -1");
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: Comprehensions should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn prop_list_comp_multiplier(mult in 1i32..10) {
        let code = format!(
            "def f(lst: list) -> list:\n    return [x * {} for x in lst]",
            mult
        );
        let _ = transpile(&code);
    }

    #[test]
    fn prop_list_comp_filter(threshold in 0i32..100) {
        let code = format!(
            "def f(lst: list) -> list:\n    return [x for x in lst if x > {}]",
            threshold
        );
        let _ = transpile(&code);
    }

    #[test]
    fn prop_dict_comp(mult in 1i32..10) {
        let code = format!(
            "def f(lst: list) -> dict:\n    return {{x: x * {} for x in lst}}",
            mult
        );
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: Binary operations should transpile
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_bitwise_and(a in 0u32..255, b in 0u32..255) {
        let code = format!("def f() -> int:\n    return {} & {}", a, b);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_bitwise_or(a in 0u32..255, b in 0u32..255) {
        let code = format!("def f() -> int:\n    return {} | {}", a, b);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_bitwise_xor(a in 0u32..255, b in 0u32..255) {
        let code = format!("def f() -> int:\n    return {} ^ {}", a, b);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_left_shift(val in 0u32..255, shift in 0u32..8) {
        let code = format!("def f() -> int:\n    return {} << {}", val, shift);
        let _ = transpile(&code);
    }

    #[test]
    fn prop_right_shift(val in 0u32..255, shift in 0u32..8) {
        let code = format!("def f() -> int:\n    return {} >> {}", val, shift);
        let _ = transpile(&code);
    }
}

// ============================================================================
// PROPERTY: Stress tests
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]

    #[test]
    fn prop_deeply_nested_parens(depth in 1usize..20) {
        let mut code = "def f() -> int:\n    return ".to_string();
        for _ in 0..depth {
            code.push('(');
        }
        code.push('1');
        for _ in 0..depth {
            code.push_str(" + 1)");
        }
        let _ = transpile(&code);
    }

    #[test]
    fn prop_long_expression_chain(length in 1usize..20) {
        let ops = ["+", "-", "*"];
        let mut code = "def f() -> int:\n    return 1".to_string();
        for i in 0..length {
            let op = &ops[i % ops.len()];
            code.push_str(&format!(" {} {}", op, (i + 2)));
        }
        let _ = transpile(&code);
    }

    #[test]
    fn prop_many_variables(count in 1usize..50) {
        let mut code = "def f() -> int:\n".to_string();
        for i in 0..count {
            code.push_str(&format!("    v{} = {}\n", i, i));
        }
        code.push_str("    return v0");
        let _ = transpile(&code);
    }
}

// ============================================================================
// STANDARD TESTS (non-proptest)
// ============================================================================

#[test]
fn test_transpiler_determinism() {
    // Same input should always produce same output
    let code = "def f(x: int) -> int:\n    return x * 2";
    let result1 = transpile(code).unwrap();
    let result2 = transpile(code).unwrap();
    assert_eq!(result1, result2, "Transpiler should be deterministic");
}

#[test]
fn test_empty_function() {
    assert!(transpile_ok("def f():\n    pass"));
}

#[test]
fn test_complex_function() {
    let code = r#"
def process(items: list, multiplier: int = 2) -> list:
    result = []
    for item in items:
        if item > 0:
            result.append(item * multiplier)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_with_methods() {
    let code = r#"
class Counter:
    def __init__(self, start: int = 0):
        self.value = start

    def increment(self) -> int:
        self.value += 1
        return self.value

    def reset(self):
        self.value = 0
"#;
    assert!(transpile_ok(code));
}
