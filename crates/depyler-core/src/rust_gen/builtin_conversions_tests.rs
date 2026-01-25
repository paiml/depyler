//! Comprehensive builtin_conversions tests
//!
//! These tests exercise the builtin_conversions.rs code paths through
//! integration tests via the transpilation pipeline.

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============================================================================
// LEN() FUNCTION TESTS
// ============================================================================

#[test]
fn test_len_list() {
    let code = transpile("def test():\n    items = [1, 2, 3]\n    return len(items)");
    assert!(code.contains("len()"));
}

#[test]
fn test_len_string() {
    let code = transpile("def test(s: str) -> int:\n    return len(s)");
    assert!(code.contains("len()"));
}

#[test]
fn test_len_dict() {
    assert!(transpile_ok("def test(d: dict) -> int:\n    return len(d)"));
}

#[test]
fn test_len_set() {
    assert!(transpile_ok("def test(s: set) -> int:\n    return len(s)"));
}

#[test]
fn test_len_in_expression() {
    let code = transpile("def test(items: list) -> bool:\n    return len(items) > 0");
    assert!(code.contains("len()"));
}

#[test]
fn test_len_in_comparison() {
    assert!(transpile_ok(
        "def test(a: list, b: list) -> bool:\n    return len(a) == len(b)"
    ));
}

#[test]
fn test_len_in_range() {
    assert!(transpile_ok(
        "def test(items: list):\n    for i in range(len(items)):\n        pass"
    ));
}

// ============================================================================
// INT() FUNCTION TESTS
// ============================================================================

#[test]
fn test_int_from_string_literal() {
    assert!(transpile_ok("def test() -> int:\n    return int('42')"));
}

#[test]
fn test_int_from_string_var() {
    assert!(transpile_ok("def test(s: str) -> int:\n    return int(s)"));
}

#[test]
fn test_int_from_float() {
    assert!(transpile_ok(
        "def test(x: float) -> int:\n    return int(x)"
    ));
}

#[test]
fn test_int_from_bool() {
    assert!(transpile_ok("def test(b: bool) -> int:\n    return int(b)"));
}

#[test]
fn test_int_with_base_16() {
    assert!(transpile_ok("def test() -> int:\n    return int('ff', 16)"));
}

#[test]
fn test_int_with_base_2() {
    assert!(transpile_ok(
        "def test() -> int:\n    return int('1010', 2)"
    ));
}

#[test]
fn test_int_with_base_8() {
    assert!(transpile_ok("def test() -> int:\n    return int('77', 8)"));
}

#[test]
fn test_int_from_expression() {
    assert!(transpile_ok(
        "def test(x: float) -> int:\n    return int(x * 2.5)"
    ));
}

#[test]
fn test_int_in_list_comprehension() {
    assert!(transpile_ok(
        "def test(items: list) -> list:\n    return [int(x) for x in items]"
    ));
}

#[test]
fn test_int_from_string_split() {
    assert!(transpile_ok(
        "def test(s: str) -> int:\n    return int(s.split()[0])"
    ));
}

// ============================================================================
// FLOAT() FUNCTION TESTS
// ============================================================================

#[test]
fn test_float_from_string_literal() {
    assert!(transpile_ok(
        "def test() -> float:\n    return float('3.14')"
    ));
}

#[test]
fn test_float_from_string_var() {
    assert!(transpile_ok(
        "def test(s: str) -> float:\n    return float(s)"
    ));
}

#[test]
fn test_float_from_int() {
    assert!(transpile_ok(
        "def test(n: int) -> float:\n    return float(n)"
    ));
}

#[test]
fn test_float_from_bool() {
    assert!(transpile_ok(
        "def test(b: bool) -> float:\n    return float(b)"
    ));
}

#[test]
fn test_float_from_expression() {
    assert!(transpile_ok(
        "def test(a: int, b: int) -> float:\n    return float(a + b)"
    ));
}

#[test]
fn test_float_in_arithmetic() {
    assert!(transpile_ok(
        "def test(n: int) -> float:\n    return float(n) / 2.0"
    ));
}

#[test]
fn test_float_from_string_strip() {
    assert!(transpile_ok(
        "def test(s: str) -> float:\n    return float(s.strip())"
    ));
}

// ============================================================================
// STR() FUNCTION TESTS
// ============================================================================

#[test]
fn test_str_from_int() {
    let code = transpile("def test(n: int) -> str:\n    return str(n)");
    assert!(code.contains("to_string"));
}

#[test]
fn test_str_from_float() {
    let code = transpile("def test(x: float) -> str:\n    return str(x)");
    assert!(code.contains("to_string"));
}

#[test]
fn test_str_from_bool() {
    let code = transpile("def test(b: bool) -> str:\n    return str(b)");
    assert!(code.contains("to_string"));
}

#[test]
fn test_str_from_list() {
    assert!(transpile_ok(
        "def test(items: list) -> str:\n    return str(items)"
    ));
}

#[test]
fn test_str_from_expression() {
    assert!(transpile_ok(
        "def test(a: int, b: int) -> str:\n    return str(a + b)"
    ));
}

#[test]
fn test_str_concatenation() {
    assert!(transpile_ok(
        "def test(n: int) -> str:\n    return 'value: ' + str(n)"
    ));
}

#[test]
fn test_str_in_list() {
    assert!(transpile_ok(
        "def test(items: list) -> list:\n    return [str(x) for x in items]"
    ));
}

// ============================================================================
// BOOL() FUNCTION TESTS
// ============================================================================

#[test]
fn test_bool_from_string() {
    assert!(transpile_ok(
        "def test(s: str) -> bool:\n    return bool(s)"
    ));
}

#[test]
fn test_bool_from_int() {
    assert!(transpile_ok(
        "def test(n: int) -> bool:\n    return bool(n)"
    ));
}

#[test]
fn test_bool_from_float() {
    assert!(transpile_ok(
        "def test(x: float) -> bool:\n    return bool(x)"
    ));
}

#[test]
fn test_bool_from_list() {
    assert!(transpile_ok(
        "def test(items: list) -> bool:\n    return bool(items)"
    ));
}

#[test]
fn test_bool_from_dict() {
    assert!(transpile_ok(
        "def test(d: dict) -> bool:\n    return bool(d)"
    ));
}

#[test]
fn test_bool_from_set() {
    assert!(transpile_ok(
        "def test(s: set) -> bool:\n    return bool(s)"
    ));
}

#[test]
fn test_bool_empty_string_literal() {
    assert!(transpile_ok("def test() -> bool:\n    return bool('')"));
}

#[test]
fn test_bool_non_empty_string_literal() {
    assert!(transpile_ok(
        "def test() -> bool:\n    return bool('hello')"
    ));
}

#[test]
fn test_bool_zero_int() {
    assert!(transpile_ok("def test() -> bool:\n    return bool(0)"));
}

#[test]
fn test_bool_non_zero_int() {
    assert!(transpile_ok("def test() -> bool:\n    return bool(42)"));
}

#[test]
fn test_bool_in_condition() {
    assert!(transpile_ok(
        "def test(items: list):\n    if bool(items):\n        pass"
    ));
}

// ============================================================================
// TYPE CONVERSION CHAINS
// ============================================================================

#[test]
fn test_int_to_str() {
    let code = transpile("def test(n: int) -> str:\n    return str(int(n))");
    assert!(code.contains("to_string"));
}

#[test]
fn test_str_to_int() {
    assert!(transpile_ok(
        "def test(s: str) -> int:\n    return int(str(s))"
    ));
}

#[test]
fn test_float_to_str() {
    let code = transpile("def test(x: float) -> str:\n    return str(float(x))");
    assert!(code.contains("to_string"));
}

#[test]
fn test_int_to_float_to_str() {
    assert!(transpile_ok(
        "def test(n: int) -> str:\n    return str(float(n))"
    ));
}

#[test]
fn test_str_to_int_to_float() {
    assert!(transpile_ok(
        "def test(s: str) -> float:\n    return float(int(s))"
    ));
}

#[test]
fn test_multiple_conversions() {
    assert!(transpile_ok(
        "def test(x: int) -> str:\n    return str(int(float(x)))"
    ));
}

// ============================================================================
// CONVERSIONS IN EXPRESSIONS
// ============================================================================

#[test]
fn test_int_in_arithmetic() {
    assert!(transpile_ok(
        "def test(s: str) -> int:\n    return int(s) + 10"
    ));
}

#[test]
fn test_float_in_multiplication() {
    assert!(transpile_ok(
        "def test(n: int) -> float:\n    return float(n) * 2.5"
    ));
}

#[test]
fn test_str_in_format() {
    assert!(transpile_ok(
        "def test(n: int) -> str:\n    return f'value: {str(n)}'"
    ));
}

#[test]
fn test_bool_in_and() {
    assert!(transpile_ok(
        "def test(a: list, b: str) -> bool:\n    return bool(a) and bool(b)"
    ));
}

#[test]
fn test_len_in_int() {
    assert!(transpile_ok(
        "def test(items: list) -> int:\n    return int(len(items))"
    ));
}

// ============================================================================
// CONVERSIONS WITH VARIABLES
// ============================================================================

#[test]
fn test_int_from_var_named_str() {
    assert!(transpile_ok(
        "def test():\n    value_str = '42'\n    return int(value_str)"
    ));
}

#[test]
fn test_int_from_var_named_text() {
    assert!(transpile_ok(
        "def test():\n    text = '42'\n    return int(text)"
    ));
}

#[test]
fn test_float_from_var_named_string() {
    assert!(transpile_ok(
        "def test():\n    string = '3.14'\n    return float(string)"
    ));
}

#[test]
fn test_bool_from_var_named_s() {
    assert!(transpile_ok("def test():\n    s = ''\n    return bool(s)"));
}

// ============================================================================
// CONVERSIONS IN FUNCTION CALLS
// ============================================================================

#[test]
fn test_str_as_arg() {
    assert!(transpile_ok(
        "def foo(s: str):\n    pass\n\ndef test(n: int):\n    foo(str(n))"
    ));
}

#[test]
fn test_int_as_arg() {
    assert!(transpile_ok(
        "def foo(n: int):\n    pass\n\ndef test(s: str):\n    foo(int(s))"
    ));
}

#[test]
fn test_float_as_arg() {
    assert!(transpile_ok(
        "def foo(x: float):\n    pass\n\ndef test(n: int):\n    foo(float(n))"
    ));
}

// ============================================================================
// CONVERSIONS IN DATA STRUCTURES
// ============================================================================

#[test]
fn test_str_in_list_literal() {
    assert!(transpile_ok(
        "def test(a: int, b: int) -> list:\n    return [str(a), str(b)]"
    ));
}

#[test]
fn test_int_in_dict_value() {
    assert!(transpile_ok(
        "def test(s: str) -> dict:\n    return {'value': int(s)}"
    ));
}

#[test]
fn test_float_in_tuple() {
    assert!(transpile_ok(
        "def test(a: int, b: int) -> tuple:\n    return (float(a), float(b))"
    ));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_int_negative_string() {
    assert!(transpile_ok("def test() -> int:\n    return int('-42')"));
}

#[test]
fn test_float_scientific_notation() {
    assert!(transpile_ok(
        "def test() -> float:\n    return float('1e10')"
    ));
}

#[test]
fn test_str_from_none() {
    assert!(transpile_ok("def test() -> str:\n    return str(None)"));
}

#[test]
fn test_len_empty_list() {
    let code = transpile("def test() -> int:\n    return len([])");
    assert!(code.contains("len()"));
}

#[test]
fn test_nested_len() {
    assert!(transpile_ok(
        "def test(items: list) -> int:\n    return len([len(x) for x in items])"
    ));
}

// ============================================================================
// CONVERSIONS WITH METHOD CALLS
// ============================================================================

#[test]
fn test_int_from_strip() {
    assert!(transpile_ok(
        "def test(s: str) -> int:\n    return int(s.strip())"
    ));
}

#[test]
fn test_float_from_replace() {
    assert!(transpile_ok(
        "def test(s: str) -> float:\n    return float(s.replace(',', '.'))"
    ));
}

#[test]
fn test_int_from_split_index() {
    assert!(transpile_ok(
        "def test(s: str) -> int:\n    return int(s.split()[0])"
    ));
}

#[test]
fn test_len_after_split() {
    let code = transpile("def test(s: str) -> int:\n    return len(s.split())");
    assert!(code.contains("len()"));
}

// ============================================================================
// CONVERSIONS IN LOOPS
// ============================================================================

#[test]
fn test_int_in_loop() {
    assert!(transpile_ok("def test(items: list):\n    total = 0\n    for item in items:\n        total += int(item)\n    return total"));
}

#[test]
fn test_str_in_loop() {
    assert!(transpile_ok("def test(items: list):\n    result = []\n    for item in items:\n        result.append(str(item))\n    return result"));
}

#[test]
fn test_len_in_loop_condition() {
    assert!(transpile_ok(
        "def test(items: list):\n    i = 0\n    while i < len(items):\n        i += 1"
    ));
}

// ============================================================================
// TRUTHINESS TESTS (bool() semantics)
// ============================================================================

#[test]
fn test_implicit_bool_string() {
    assert!(transpile_ok("def test(s: str):\n    if s:\n        pass"));
}

#[test]
fn test_implicit_bool_list() {
    assert!(transpile_ok(
        "def test(items: list):\n    if items:\n        pass"
    ));
}

#[test]
fn test_implicit_bool_int() {
    assert!(transpile_ok("def test(n: int):\n    if n:\n        pass"));
}

#[test]
fn test_not_bool_string() {
    assert!(transpile_ok(
        "def test(s: str) -> bool:\n    return not bool(s)"
    ));
}

#[test]
fn test_not_bool_list() {
    assert!(transpile_ok(
        "def test(items: list) -> bool:\n    return not bool(items)"
    ));
}
