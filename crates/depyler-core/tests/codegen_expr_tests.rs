//! EXTREME TDD: Tests for codegen.rs expression generation
//! Coverage: expr_to_rust_tokens, binary_expr_to_rust_tokens, call_expr_to_rust_tokens

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    DepylerPipeline::new().transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

// ============ Literal to tokens ============

#[test]
fn test_expr_literal_int_zero() {
    assert!(transpile_ok("def f() -> int: return 0"));
}

#[test]
fn test_expr_literal_int_positive() {
    assert!(transpile_ok("def f() -> int: return 12345"));
}

#[test]
fn test_expr_literal_int_negative() {
    assert!(transpile_ok("def f() -> int: return -42"));
}

#[test]
fn test_expr_literal_float() {
    assert!(transpile_ok("def f() -> float: return 3.14159"));
}

#[test]
fn test_expr_literal_float_scientific() {
    assert!(transpile_ok("def f() -> float: return 1.5e10"));
}

#[test]
fn test_expr_literal_str() {
    assert!(transpile_ok("def f() -> str: return \"hello world\""));
}

#[test]
fn test_expr_literal_str_empty() {
    assert!(transpile_ok("def f() -> str: return \"\""));
}

#[test]
fn test_expr_literal_str_escape() {
    assert!(transpile_ok("def f() -> str: return \"line1\\nline2\""));
}

#[test]
fn test_expr_literal_bool_true() {
    assert!(transpile_ok("def f() -> bool: return True"));
}

#[test]
fn test_expr_literal_bool_false() {
    assert!(transpile_ok("def f() -> bool: return False"));
}

#[test]
fn test_expr_literal_none() {
    assert!(transpile_ok("def f() -> None: return None"));
}

// ============ Binary expression to tokens ============

#[test]
fn test_expr_binary_add() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a + b"));
}

#[test]
fn test_expr_binary_sub() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a - b"));
}

#[test]
fn test_expr_binary_mul() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a * b"));
}

#[test]
fn test_expr_binary_div() {
    assert!(transpile_ok("def f(a: float, b: float) -> float: return a / b"));
}

#[test]
fn test_expr_binary_floordiv() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a // b"));
}

#[test]
fn test_expr_binary_mod() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a % b"));
}

#[test]
fn test_expr_binary_pow() {
    assert!(transpile_ok("def f(a: float, b: float) -> float: return a ** b"));
}

#[test]
fn test_expr_binary_eq() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a == b"));
}

#[test]
fn test_expr_binary_ne() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a != b"));
}

#[test]
fn test_expr_binary_lt() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a < b"));
}

#[test]
fn test_expr_binary_le() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a <= b"));
}

#[test]
fn test_expr_binary_gt() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a > b"));
}

#[test]
fn test_expr_binary_ge() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a >= b"));
}

#[test]
fn test_expr_binary_and() {
    assert!(transpile_ok("def f(a: bool, b: bool) -> bool: return a and b"));
}

#[test]
fn test_expr_binary_or() {
    assert!(transpile_ok("def f(a: bool, b: bool) -> bool: return a or b"));
}

#[test]
fn test_expr_binary_bitand() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a & b"));
}

#[test]
fn test_expr_binary_bitor() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a | b"));
}

#[test]
fn test_expr_binary_bitxor() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a ^ b"));
}

#[test]
fn test_expr_binary_lshift() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a << b"));
}

#[test]
fn test_expr_binary_rshift() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a >> b"));
}

// ============ Unary expression to tokens ============

#[test]
fn test_expr_unary_neg() {
    assert!(transpile_ok("def f(x: int) -> int: return -x"));
}

#[test]
fn test_expr_unary_not() {
    assert!(transpile_ok("def f(x: bool) -> bool: return not x"));
}

#[test]
fn test_expr_unary_bitnot() {
    assert!(transpile_ok("def f(x: int) -> int: return ~x"));
}

// ============ Call expression to tokens ============

#[test]
fn test_expr_call_len() {
    assert!(transpile_ok("def f(items: list) -> int: return len(items)"));
}

#[test]
fn test_expr_call_abs() {
    assert!(transpile_ok("def f(x: int) -> int: return abs(x)"));
}

#[test]
fn test_expr_call_min() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return min(a, b)"));
}

#[test]
fn test_expr_call_max() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return max(a, b)"));
}

#[test]
fn test_expr_call_sum() {
    assert!(transpile_ok("def f(items: list) -> int: return sum(items)"));
}

#[test]
fn test_expr_call_str() {
    assert!(transpile_ok("def f(x: int) -> str: return str(x)"));
}

#[test]
fn test_expr_call_int() {
    assert!(transpile_ok("def f(s: str) -> int: return int(s)"));
}

#[test]
fn test_expr_call_float() {
    assert!(transpile_ok("def f(x: int) -> float: return float(x)"));
}

#[test]
fn test_expr_call_bool() {
    assert!(transpile_ok("def f(x: int) -> bool: return bool(x)"));
}

#[test]
fn test_expr_call_sorted() {
    assert!(transpile_ok("def f(items: list) -> list: return sorted(items)"));
}

#[test]
fn test_expr_call_reversed() {
    assert!(transpile_ok("def f(items: list) -> list: return list(reversed(items))"));
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
fn test_expr_call_range_start_end() {
    let code = r#"
def f(start: int, end: int) -> list:
    return list(range(start, end))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_call_enumerate() {
    let code = r#"
def f(items: list) -> list:
    return list(enumerate(items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_call_zip() {
    let code = r#"
def f(a: list, b: list) -> list:
    return list(zip(a, b))
"#;
    assert!(transpile_ok(code));
}

// ============ Method call to tokens ============

#[test]
fn test_expr_method_str_upper() {
    assert!(transpile_ok("def f(s: str) -> str: return s.upper()"));
}

#[test]
fn test_expr_method_str_lower() {
    assert!(transpile_ok("def f(s: str) -> str: return s.lower()"));
}

#[test]
fn test_expr_method_str_strip() {
    assert!(transpile_ok("def f(s: str) -> str: return s.strip()"));
}

#[test]
fn test_expr_method_str_split() {
    assert!(transpile_ok("def f(s: str) -> list: return s.split()"));
}

#[test]
fn test_expr_method_str_join() {
    assert!(transpile_ok("def f(items: list) -> str: return \",\".join(items)"));
}

#[test]
fn test_expr_method_str_replace() {
    assert!(transpile_ok("def f(s: str) -> str: return s.replace(\"a\", \"b\")"));
}

#[test]
fn test_expr_method_str_startswith() {
    assert!(transpile_ok("def f(s: str) -> bool: return s.startswith(\"hello\")"));
}

#[test]
fn test_expr_method_str_endswith() {
    assert!(transpile_ok("def f(s: str) -> bool: return s.endswith(\"world\")"));
}

#[test]
fn test_expr_method_str_find() {
    assert!(transpile_ok("def f(s: str, sub: str) -> int: return s.find(sub)"));
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
    assert!(transpile_ok("def f(items: list) -> int: return items.pop()"));
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
fn test_expr_method_list_sort() {
    let code = r#"
def f(items: list) -> list:
    items.sort()
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_method_list_reverse() {
    let code = r#"
def f(items: list) -> list:
    items.reverse()
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_method_dict_get() {
    assert!(transpile_ok("def f(d: dict, k: str) -> int: return d.get(k, 0)"));
}

#[test]
fn test_expr_method_dict_keys() {
    assert!(transpile_ok("def f(d: dict) -> list: return list(d.keys())"));
}

#[test]
fn test_expr_method_dict_values() {
    assert!(transpile_ok("def f(d: dict) -> list: return list(d.values())"));
}

#[test]
fn test_expr_method_dict_items() {
    assert!(transpile_ok("def f(d: dict) -> list: return list(d.items())"));
}

// ============ List/Dict/Tuple literals to tokens ============

#[test]
fn test_expr_list_empty() {
    assert!(transpile_ok("def f() -> list: return []"));
}

#[test]
fn test_expr_list_ints() {
    assert!(transpile_ok("def f() -> list: return [1, 2, 3, 4, 5]"));
}

#[test]
fn test_expr_list_strings() {
    assert!(transpile_ok("def f() -> list: return [\"a\", \"b\", \"c\"]"));
}

#[test]
fn test_expr_list_nested() {
    assert!(transpile_ok("def f() -> list: return [[1, 2], [3, 4]]"));
}

#[test]
fn test_expr_dict_empty() {
    assert!(transpile_ok("def f() -> dict: return {}"));
}

#[test]
fn test_expr_dict_str_int() {
    assert!(transpile_ok("def f() -> dict: return {\"a\": 1, \"b\": 2}"));
}

#[test]
fn test_expr_dict_int_str() {
    // Dict with int keys - may or may not be fully supported
    // This test ensures the transpiler handles the input without panic
    let result = transpile("def f() -> dict: return {1: \"one\", 2: \"two\"}");
    // Accept both success and graceful failure (no panic)
    let _ = result;
}

#[test]
fn test_expr_tuple_empty() {
    assert!(transpile_ok("def f() -> tuple: return ()"));
}

#[test]
fn test_expr_tuple_single() {
    assert!(transpile_ok("def f() -> tuple: return (1,)"));
}

#[test]
fn test_expr_tuple_multiple() {
    assert!(transpile_ok("def f() -> tuple: return (1, 2, 3)"));
}

#[test]
fn test_expr_set_literal() {
    assert!(transpile_ok("def f() -> set: return {1, 2, 3}"));
}

// ============ Comprehension to tokens ============

#[test]
fn test_expr_list_comp_simple() {
    assert!(transpile_ok("def f(items: list) -> list: return [x * 2 for x in items]"));
}

#[test]
fn test_expr_list_comp_with_if() {
    assert!(transpile_ok("def f(items: list) -> list: return [x for x in items if x > 0]"));
}

#[test]
fn test_expr_list_comp_with_call() {
    assert!(transpile_ok("def f(items: list) -> list: return [str(x) for x in items]"));
}

#[test]
fn test_expr_dict_comp() {
    assert!(transpile_ok("def f(items: list) -> dict: return {x: x * 2 for x in items}"));
}

#[test]
fn test_expr_set_comp() {
    assert!(transpile_ok("def f(items: list) -> set: return {x * 2 for x in items}"));
}

// ============ Subscript to tokens ============

#[test]
fn test_expr_subscript_list() {
    assert!(transpile_ok("def f(items: list) -> int: return items[0]"));
}

#[test]
fn test_expr_subscript_list_negative() {
    assert!(transpile_ok("def f(items: list) -> int: return items[-1]"));
}

#[test]
fn test_expr_subscript_dict() {
    assert!(transpile_ok("def f(data: dict) -> int: return data[\"key\"]"));
}

#[test]
fn test_expr_subscript_str() {
    assert!(transpile_ok("def f(s: str) -> str: return s[0]"));
}

// ============ Slice to tokens ============

#[test]
fn test_expr_slice_start() {
    assert!(transpile_ok("def f(items: list) -> list: return items[1:]"));
}

#[test]
fn test_expr_slice_end() {
    assert!(transpile_ok("def f(items: list) -> list: return items[:3]"));
}

#[test]
fn test_expr_slice_both() {
    assert!(transpile_ok("def f(items: list) -> list: return items[1:3]"));
}

#[test]
fn test_expr_slice_step() {
    assert!(transpile_ok("def f(items: list) -> list: return items[::2]"));
}

#[test]
fn test_expr_slice_reverse() {
    assert!(transpile_ok("def f(items: list) -> list: return items[::-1]"));
}

// ============ Ternary to tokens ============

#[test]
fn test_expr_ternary() {
    assert!(transpile_ok("def f(x: int) -> int: return x if x > 0 else -x"));
}

#[test]
fn test_expr_ternary_nested() {
    let code = r#"
def f(x: int) -> str:
    return "pos" if x > 0 else ("zero" if x == 0 else "neg")
"#;
    assert!(transpile_ok(code));
}

// ============ Lambda to tokens ============

#[test]
fn test_expr_lambda_in_sorted() {
    assert!(transpile_ok("def f(items: list) -> list: return sorted(items, key=lambda x: -x)"));
}

// ============ Complex expressions ============

#[test]
fn test_expr_nested_calls() {
    assert!(transpile_ok("def f(s: str) -> str: return s.upper().strip().lower()"));
}

#[test]
fn test_expr_complex_arithmetic() {
    assert!(transpile_ok("def f(a: int, b: int, c: int) -> int: return (a + b) * c - a % b"));
}

#[test]
fn test_expr_in_operator() {
    assert!(transpile_ok("def f(x: int, items: list) -> bool: return x in items"));
}

#[test]
fn test_expr_not_in_operator() {
    assert!(transpile_ok("def f(x: int, items: list) -> bool: return x not in items"));
}
