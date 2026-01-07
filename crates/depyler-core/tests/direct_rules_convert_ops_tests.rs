//! EXTREME TDD: Tests for direct_rules_convert operator functions
//! Coverage: convert_binop, convert_arithmetic_op, convert_comparison_op, convert_logical_op

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

// ============ Arithmetic operators ============

#[test]
fn test_op_add_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a + b"));
}

#[test]
fn test_op_add_float() {
    assert!(transpile_ok("def f(a: float, b: float) -> float: return a + b"));
}

#[test]
fn test_op_add_string() {
    assert!(transpile_ok("def f(a: str, b: str) -> str: return a + b"));
}

#[test]
fn test_op_sub_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a - b"));
}

#[test]
fn test_op_sub_float() {
    assert!(transpile_ok("def f(a: float, b: float) -> float: return a - b"));
}

#[test]
fn test_op_mul_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a * b"));
}

#[test]
fn test_op_mul_float() {
    assert!(transpile_ok("def f(a: float, b: float) -> float: return a * b"));
}

#[test]
fn test_op_mul_string_int() {
    assert!(transpile_ok("def f(s: str, n: int) -> str: return s * n"));
}

#[test]
fn test_op_div_float() {
    assert!(transpile_ok("def f(a: float, b: float) -> float: return a / b"));
}

#[test]
fn test_op_floordiv() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a // b"));
}

#[test]
fn test_op_mod() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a % b"));
}

#[test]
fn test_op_pow_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a ** b"));
}

#[test]
fn test_op_pow_float() {
    assert!(transpile_ok("def f(a: float, b: float) -> float: return a ** b"));
}

#[test]
fn test_op_matmul() {
    // Matrix multiplication - may or may not be supported
    let code = "def f(a: list, b: list) -> list: return a @ b";
    let _ = transpile(code); // Just ensure no panic
}

// ============ Comparison operators ============

#[test]
fn test_op_eq_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a == b"));
}

#[test]
fn test_op_eq_str() {
    assert!(transpile_ok("def f(a: str, b: str) -> bool: return a == b"));
}

#[test]
fn test_op_eq_bool() {
    assert!(transpile_ok("def f(a: bool, b: bool) -> bool: return a == b"));
}

#[test]
fn test_op_ne_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a != b"));
}

#[test]
fn test_op_lt_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a < b"));
}

#[test]
fn test_op_lt_float() {
    assert!(transpile_ok("def f(a: float, b: float) -> bool: return a < b"));
}

#[test]
fn test_op_lt_str() {
    assert!(transpile_ok("def f(a: str, b: str) -> bool: return a < b"));
}

#[test]
fn test_op_le_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a <= b"));
}

#[test]
fn test_op_gt_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a > b"));
}

#[test]
fn test_op_ge_int() {
    assert!(transpile_ok("def f(a: int, b: int) -> bool: return a >= b"));
}

#[test]
fn test_op_chained_comparison() {
    assert!(transpile_ok("def f(x: int) -> bool: return 0 < x < 10"));
}

#[test]
fn test_op_chained_eq() {
    assert!(transpile_ok("def f(a: int, b: int, c: int) -> bool: return a == b == c"));
}

// ============ Logical operators ============

#[test]
fn test_op_and_bool() {
    assert!(transpile_ok("def f(a: bool, b: bool) -> bool: return a and b"));
}

#[test]
fn test_op_or_bool() {
    assert!(transpile_ok("def f(a: bool, b: bool) -> bool: return a or b"));
}

#[test]
fn test_op_not_bool() {
    assert!(transpile_ok("def f(a: bool) -> bool: return not a"));
}

#[test]
fn test_op_and_short_circuit() {
    let code = r#"
def f(x: int) -> bool:
    return x > 0 and x < 10
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_or_short_circuit() {
    let code = r#"
def f(x: int) -> bool:
    return x < 0 or x > 10
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_complex_logical() {
    let code = r#"
def f(a: bool, b: bool, c: bool) -> bool:
    return (a and b) or (not c)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_nested_logical() {
    let code = r#"
def f(x: int, y: int) -> bool:
    return (x > 0 and y > 0) or (x < 0 and y < 0)
"#;
    assert!(transpile_ok(code));
}

// ============ Bitwise operators ============

#[test]
fn test_op_bitand() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a & b"));
}

#[test]
fn test_op_bitor() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a | b"));
}

#[test]
fn test_op_bitxor() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a ^ b"));
}

#[test]
fn test_op_bitnot() {
    assert!(transpile_ok("def f(a: int) -> int: return ~a"));
}

#[test]
fn test_op_lshift() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a << b"));
}

#[test]
fn test_op_rshift() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return a >> b"));
}

#[test]
fn test_op_bitwise_complex() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return (a & b) | (b ^ c)
"#;
    assert!(transpile_ok(code));
}

// ============ Membership operators ============

#[test]
fn test_op_in_list() {
    assert!(transpile_ok("def f(x: int, items: list) -> bool: return x in items"));
}

#[test]
fn test_op_in_str() {
    assert!(transpile_ok("def f(c: str, s: str) -> bool: return c in s"));
}

#[test]
fn test_op_in_dict() {
    assert!(transpile_ok("def f(k: str, d: dict) -> bool: return k in d"));
}

#[test]
fn test_op_not_in_list() {
    assert!(transpile_ok("def f(x: int, items: list) -> bool: return x not in items"));
}

#[test]
fn test_op_not_in_str() {
    assert!(transpile_ok("def f(c: str, s: str) -> bool: return c not in s"));
}

// ============ Identity operators ============

#[test]
fn test_op_is_none() {
    assert!(transpile_ok("def f(x: int) -> bool: return x is None"));
}

#[test]
fn test_op_is_not_none() {
    assert!(transpile_ok("def f(x: int) -> bool: return x is not None"));
}

// ============ Unary operators ============

#[test]
fn test_op_unary_neg_int() {
    assert!(transpile_ok("def f(x: int) -> int: return -x"));
}

#[test]
fn test_op_unary_neg_float() {
    assert!(transpile_ok("def f(x: float) -> float: return -x"));
}

#[test]
fn test_op_unary_pos() {
    assert!(transpile_ok("def f(x: int) -> int: return +x"));
}

// ============ Augmented assignment operators ============

#[test]
fn test_op_aug_add() {
    let code = r#"
def f(x: int) -> int:
    x += 1
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_sub() {
    let code = r#"
def f(x: int) -> int:
    x -= 1
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_mul() {
    let code = r#"
def f(x: int) -> int:
    x *= 2
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_div() {
    let code = r#"
def f(x: float) -> float:
    x /= 2.0
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_floordiv() {
    let code = r#"
def f(x: int) -> int:
    x //= 2
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_mod() {
    let code = r#"
def f(x: int) -> int:
    x %= 10
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_pow() {
    let code = r#"
def f(x: int) -> int:
    x **= 2
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_bitand() {
    let code = r#"
def f(x: int) -> int:
    x &= 0xFF
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_bitor() {
    let code = r#"
def f(x: int) -> int:
    x |= 0x01
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_bitxor() {
    let code = r#"
def f(x: int) -> int:
    x ^= 0xFF
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_lshift() {
    let code = r#"
def f(x: int) -> int:
    x <<= 1
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_aug_rshift() {
    let code = r#"
def f(x: int) -> int:
    x >>= 1
    return x
"#;
    assert!(transpile_ok(code));
}

// ============ Operator precedence ============

#[test]
fn test_op_precedence_arith() {
    assert!(transpile_ok("def f(a: int, b: int, c: int) -> int: return a + b * c"));
}

#[test]
fn test_op_precedence_parens() {
    assert!(transpile_ok("def f(a: int, b: int, c: int) -> int: return (a + b) * c"));
}

#[test]
fn test_op_precedence_mixed() {
    let code = r#"
def f(a: int, b: int, c: int) -> bool:
    return a + b > c and b - a < c
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_op_precedence_complex() {
    let code = r#"
def f(a: int, b: int, c: int, d: int) -> int:
    return a + b * c - d // 2 + a % b
"#;
    assert!(transpile_ok(code));
}

// ============ Edge cases ============

#[test]
fn test_op_divide_by_literal() {
    assert!(transpile_ok("def f(x: int) -> float: return x / 2.0"));
}

#[test]
fn test_op_power_of_two() {
    assert!(transpile_ok("def f(n: int) -> int: return 2 ** n"));
}

#[test]
fn test_op_string_repeat() {
    assert!(transpile_ok("def f(s: str) -> str: return s * 3"));
}

#[test]
fn test_op_list_concat() {
    assert!(transpile_ok("def f(a: list, b: list) -> list: return a + b"));
}

#[test]
fn test_op_negation_chain() {
    assert!(transpile_ok("def f(x: int) -> int: return --x"));
}

#[test]
fn test_op_not_chain() {
    assert!(transpile_ok("def f(b: bool) -> bool: return not not b"));
}
