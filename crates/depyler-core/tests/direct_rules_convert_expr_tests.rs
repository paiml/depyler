//! EXTREME TDD: Tests for direct_rules_convert expression functions
//! Coverage: convert_expr, convert_expr_with_context, convert_expr_with_param_types

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    DepylerPipeline::new().transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

fn transpile_contains(code: &str, needle: &str) -> bool {
    transpile(code).map(|s| s.contains(needle)).unwrap_or(false)
}

// ============ Literal expression tests ============

#[test]
fn test_expr_int_literal() {
    assert!(transpile_ok("def f() -> int:\n    return 42"));
}

#[test]
fn test_expr_negative_int() {
    assert!(transpile_ok("def f() -> int:\n    return -42"));
}

#[test]
fn test_expr_zero() {
    assert!(transpile_ok("def f() -> int:\n    return 0"));
}

#[test]
fn test_expr_large_int() {
    assert!(transpile_ok("def f() -> int:\n    return 9999999999"));
}

#[test]
fn test_expr_float_literal() {
    assert!(transpile_ok("def f() -> float:\n    return 3.14159"));
}

#[test]
fn test_expr_negative_float() {
    assert!(transpile_ok("def f() -> float:\n    return -2.5"));
}

#[test]
fn test_expr_scientific_float() {
    assert!(transpile_ok("def f() -> float:\n    return 1e10"));
}

#[test]
fn test_expr_string_literal() {
    assert!(transpile_ok("def f() -> str:\n    return \"hello\""));
}

#[test]
fn test_expr_empty_string() {
    assert!(transpile_ok("def f() -> str:\n    return \"\""));
}

#[test]
fn test_expr_string_with_escape() {
    assert!(transpile_ok("def f() -> str:\n    return \"hello\\nworld\""));
}

#[test]
fn test_expr_bool_true() {
    assert!(transpile_ok("def f() -> bool:\n    return True"));
}

#[test]
fn test_expr_bool_false() {
    assert!(transpile_ok("def f() -> bool:\n    return False"));
}

#[test]
fn test_expr_none() {
    assert!(transpile_ok("def f() -> None:\n    return None"));
}

// ============ Binary expression tests ============

#[test]
fn test_expr_add() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a + b"));
}

#[test]
fn test_expr_sub() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a - b"));
}

#[test]
fn test_expr_mul() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a * b"));
}

#[test]
fn test_expr_div() {
    assert!(transpile_ok("def f(a: float, b: float) -> float:\n    return a / b"));
}

#[test]
fn test_expr_floor_div() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a // b"));
}

#[test]
fn test_expr_mod() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a % b"));
}

#[test]
fn test_expr_pow() {
    assert!(transpile_ok("def f(a: float, b: float) -> float:\n    return a ** b"));
}

#[test]
fn test_expr_eq() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool:\n    return a == b"));
}

#[test]
fn test_expr_ne() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool:\n    return a != b"));
}

#[test]
fn test_expr_lt() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool:\n    return a < b"));
}

#[test]
fn test_expr_le() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool:\n    return a <= b"));
}

#[test]
fn test_expr_gt() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool:\n    return a > b"));
}

#[test]
fn test_expr_ge() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool:\n    return a >= b"));
}

#[test]
fn test_expr_and() {
    assert!(transpile_ok("def f(a: bool, b: bool) -> bool:\n    return a and b"));
}

#[test]
fn test_expr_or() {
    assert!(transpile_ok("def f(a: bool, b: bool) -> bool:\n    return a or b"));
}

#[test]
fn test_expr_bitand() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a & b"));
}

#[test]
fn test_expr_bitor() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a | b"));
}

#[test]
fn test_expr_bitxor() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a ^ b"));
}

#[test]
fn test_expr_lshift() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a << b"));
}

#[test]
fn test_expr_rshift() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a >> b"));
}

// ============ Unary expression tests ============

#[test]
fn test_expr_unary_neg() {
    assert!(transpile_ok("def f(x: int) -> int:\n    return -x"));
}

#[test]
fn test_expr_unary_not() {
    assert!(transpile_ok("def f(x: bool) -> bool:\n    return not x"));
}

#[test]
fn test_expr_unary_bitnot() {
    assert!(transpile_ok("def f(x: int) -> int:\n    return ~x"));
}

// ============ Call expression tests ============

#[test]
fn test_expr_call_len() {
    assert!(transpile_ok("def f(items: list) -> int:\n    return len(items)"));
}

#[test]
fn test_expr_call_str() {
    assert!(transpile_ok("def f(x: int) -> str:\n    return str(x)"));
}

#[test]
fn test_expr_call_int() {
    assert!(transpile_ok("def f(s: str) -> int:\n    return int(s)"));
}

#[test]
fn test_expr_call_float() {
    assert!(transpile_ok("def f(x: int) -> float:\n    return float(x)"));
}

#[test]
fn test_expr_call_abs() {
    assert!(transpile_ok("def f(x: int) -> int:\n    return abs(x)"));
}

#[test]
fn test_expr_call_min() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return min(a, b)"));
}

#[test]
fn test_expr_call_max() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return max(a, b)"));
}

#[test]
fn test_expr_call_sum() {
    assert!(transpile_ok("def f(items: list) -> int:\n    return sum(items)"));
}

#[test]
fn test_expr_call_range() {
    let code = r#"
def f(n: int) -> list:
    return list(range(n))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_call_sorted() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return sorted(items)"));
}

#[test]
fn test_expr_call_reversed() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return list(reversed(items))"));
}

// ============ Method call expression tests ============

#[test]
fn test_expr_method_str_upper() {
    assert!(transpile_ok("def f(s: str) -> str:\n    return s.upper()"));
}

#[test]
fn test_expr_method_str_lower() {
    assert!(transpile_ok("def f(s: str) -> str:\n    return s.lower()"));
}

#[test]
fn test_expr_method_str_strip() {
    assert!(transpile_ok("def f(s: str) -> str:\n    return s.strip()"));
}

#[test]
fn test_expr_method_str_split() {
    assert!(transpile_ok("def f(s: str) -> list:\n    return s.split()"));
}

#[test]
fn test_expr_method_str_join() {
    assert!(transpile_ok("def f(items: list) -> str:\n    return \",\".join(items)"));
}

#[test]
fn test_expr_method_str_replace() {
    assert!(transpile_ok("def f(s: str) -> str:\n    return s.replace(\"a\", \"b\")"));
}

#[test]
fn test_expr_method_str_startswith() {
    assert!(transpile_ok("def f(s: str) -> bool:\n    return s.startswith(\"hello\")"));
}

#[test]
fn test_expr_method_str_endswith() {
    assert!(transpile_ok("def f(s: str) -> bool:\n    return s.endswith(\"world\")"));
}

#[test]
fn test_expr_method_list_append() {
    let code = r#"
def f(items: list, x: int) -> list:
    items.append(x)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_method_list_pop() {
    let code = r#"
def f(items: list) -> int:
    return items.pop()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_method_list_extend() {
    let code = r#"
def f(a: list, b: list) -> list:
    a.extend(b)
    return a
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_method_dict_get() {
    assert!(transpile_ok("def f(d: dict, k: str) -> int:\n    return d.get(k, 0)"));
}

#[test]
fn test_expr_method_dict_keys() {
    assert!(transpile_ok("def f(d: dict) -> list:\n    return list(d.keys())"));
}

#[test]
fn test_expr_method_dict_values() {
    assert!(transpile_ok("def f(d: dict) -> list:\n    return list(d.values())"));
}

#[test]
fn test_expr_method_dict_items() {
    assert!(transpile_ok("def f(d: dict) -> list:\n    return list(d.items())"));
}

// ============ Subscript expression tests ============

#[test]
fn test_expr_subscript_list() {
    assert!(transpile_ok("def f(items: list) -> int:\n    return items[0]"));
}

#[test]
fn test_expr_subscript_dict() {
    assert!(transpile_ok("def f(d: dict) -> int:\n    return d[\"key\"]"));
}

#[test]
fn test_expr_subscript_string() {
    assert!(transpile_ok("def f(s: str) -> str:\n    return s[0]"));
}

#[test]
fn test_expr_subscript_negative() {
    assert!(transpile_ok("def f(items: list) -> int:\n    return items[-1]"));
}

#[test]
fn test_expr_subscript_variable() {
    assert!(transpile_ok("def f(items: list, i: int) -> int:\n    return items[i]"));
}

// ============ Slice expression tests ============

#[test]
fn test_expr_slice_start() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return items[1:]"));
}

#[test]
fn test_expr_slice_end() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return items[:3]"));
}

#[test]
fn test_expr_slice_both() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return items[1:3]"));
}

#[test]
fn test_expr_slice_step() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return items[::2]"));
}

#[test]
fn test_expr_slice_reverse() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return items[::-1]"));
}

// ============ Attribute expression tests ============

#[test]
fn test_expr_attr_self() {
    let code = r#"
class Point:
    x: int
    def get_x(self) -> int:
        return self.x
"#;
    assert!(transpile_ok(code));
}

// ============ List/Dict/Set literal tests ============

#[test]
fn test_expr_list_empty() {
    assert!(transpile_ok("def f() -> list:\n    return []"));
}

#[test]
fn test_expr_list_ints() {
    assert!(transpile_ok("def f() -> list:\n    return [1, 2, 3]"));
}

#[test]
fn test_expr_list_mixed() {
    assert!(transpile_ok("def f() -> list:\n    return [1, \"two\", 3.0]"));
}

#[test]
fn test_expr_dict_empty() {
    assert!(transpile_ok("def f() -> dict:\n    return {}"));
}

#[test]
fn test_expr_dict_simple() {
    assert!(transpile_ok("def f() -> dict:\n    return {\"a\": 1, \"b\": 2}"));
}

#[test]
fn test_expr_set_literal() {
    assert!(transpile_ok("def f() -> set:\n    return {1, 2, 3}"));
}

#[test]
fn test_expr_tuple_literal() {
    assert!(transpile_ok("def f() -> tuple:\n    return (1, 2, 3)"));
}

// ============ Comprehension tests ============

#[test]
fn test_expr_list_comp_simple() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return [x * 2 for x in items]"));
}

#[test]
fn test_expr_list_comp_with_if() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return [x for x in items if x > 0]"));
}

#[test]
fn test_expr_dict_comp() {
    assert!(transpile_ok("def f(items: list) -> dict:\n    return {x: x * 2 for x in items}"));
}

#[test]
fn test_expr_set_comp() {
    assert!(transpile_ok("def f(items: list) -> set:\n    return {x * 2 for x in items}"));
}

// ============ Ternary expression tests ============

#[test]
fn test_expr_ternary_simple() {
    assert!(transpile_ok("def f(x: int) -> int:\n    return x if x > 0 else -x"));
}

#[test]
fn test_expr_ternary_nested() {
    let code = r#"
def f(x: int) -> str:
    return "positive" if x > 0 else ("zero" if x == 0 else "negative")
"#;
    assert!(transpile_ok(code));
}

// ============ Lambda expression tests ============

#[test]
fn test_expr_lambda_simple() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: x * -1)
"#;
    assert!(transpile_ok(code));
}

// ============ Complex expression tests ============

#[test]
fn test_expr_chained_comparison() {
    assert!(transpile_ok("def f(x: int) -> bool:\n    return 0 < x < 10"));
}

#[test]
fn test_expr_complex_arithmetic() {
    assert!(transpile_ok("def f(a: int, b: int, c: int) -> int:\n    return (a + b) * c - (a - b) // c"));
}

#[test]
fn test_expr_nested_calls() {
    assert!(transpile_ok("def f(s: str) -> str:\n    return s.upper().strip().lower()"));
}

#[test]
fn test_expr_in_operator() {
    assert!(transpile_ok("def f(x: int, items: list) -> bool:\n    return x in items"));
}

#[test]
fn test_expr_not_in_operator() {
    assert!(transpile_ok("def f(x: int, items: list) -> bool:\n    return x not in items"));
}
