//! Coverage tests for rust_gen/stdlib_method_gen/builtin_functions.rs
//!
//! DEPYLER-99MODE-001: Targets builtin_functions.rs (1,305 lines)
//! Covers: all/any, divmod, enumerate, zip, reversed, sorted,
//! sum, round, abs, min, max, pow, hex/bin/oct, chr/ord.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// all() / any()
// ============================================================================

#[test]
fn test_builtin_all_true() {
    let code = r#"
def f() -> bool:
    return all([True, True, True])
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_all_list() {
    let code = r#"
def f(items: list) -> bool:
    return all(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_any_list() {
    let code = r#"
def f(items: list) -> bool:
    return any(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_any_false() {
    let code = r#"
def f() -> bool:
    return any([False, False, False])
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// enumerate()
// ============================================================================

#[test]
fn test_builtin_enumerate_basic() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for idx, val in enumerate(items):
        total += idx
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// zip()
// ============================================================================

#[test]
fn test_builtin_zip_two() {
    let code = r#"
def f(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// reversed()
// ============================================================================

#[test]
fn test_builtin_reversed() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// sorted()
// ============================================================================

#[test]
fn test_builtin_sorted_basic() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_sorted_with_key() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// sum()
// ============================================================================

#[test]
fn test_builtin_sum_basic() {
    let code = r#"
def f(items: list) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// round()
// ============================================================================

#[test]
fn test_builtin_round_basic() {
    let code = r#"
def f(x: float) -> int:
    return round(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// abs()
// ============================================================================

#[test]
fn test_builtin_abs_int() {
    let code = r#"
def f(x: int) -> int:
    return abs(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_abs_float() {
    let code = r#"
def f(x: float) -> float:
    return abs(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// min() / max()
// ============================================================================

#[test]
fn test_builtin_min_list() {
    let code = r#"
def f(items: list) -> int:
    return min(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_max_list() {
    let code = r#"
def f(items: list) -> int:
    return max(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_min_args() {
    let code = r#"
def f(a: int, b: int) -> int:
    return min(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_max_args() {
    let code = r#"
def f(a: int, b: int) -> int:
    return max(a, b)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// pow()
// ============================================================================

#[test]
fn test_builtin_pow_basic() {
    let code = r#"
def f(base: int, exp: int) -> int:
    return pow(base, exp)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// hex() / bin() / oct()
// ============================================================================

#[test]
fn test_builtin_hex() {
    let code = r#"
def f(n: int) -> str:
    return hex(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_bin() {
    let code = r#"
def f(n: int) -> str:
    return bin(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_oct() {
    let code = r#"
def f(n: int) -> str:
    return oct(n)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// chr() / ord()
// ============================================================================

#[test]
fn test_builtin_chr() {
    let code = r#"
def f(n: int) -> str:
    return chr(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_ord() {
    let code = r#"
def f(c: str) -> int:
    return ord(c)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// isinstance()
// ============================================================================

#[test]
fn test_builtin_isinstance() {
    let code = r#"
def f(x: int) -> bool:
    return isinstance(x, int)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// len()
// ============================================================================

#[test]
fn test_builtin_len_list() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_len_str() {
    let code = r#"
def f(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_len_dict() {
    let code = r#"
def f(d: dict) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type conversions
// ============================================================================

#[test]
fn test_builtin_int_from_str() {
    let code = r#"
def f(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_str_from_int() {
    let code = r#"
def f(n: int) -> str:
    return str(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_float_from_str() {
    let code = r#"
def f(s: str) -> float:
    return float(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_bool_from_int() {
    let code = r#"
def f(n: int) -> bool:
    return bool(n)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// print()
// ============================================================================

#[test]
fn test_builtin_print_basic() {
    let code = r#"
def f(x: int):
    print(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_print_multiple() {
    let code = r#"
def f(a: int, b: str):
    print(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_print_end() {
    let code = r#"
def f():
    print("hello", end="")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_print_sep() {
    let code = r#"
def f():
    print("a", "b", "c", sep=", ")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// filter() / map()
// ============================================================================

#[test]
fn test_builtin_filter() {
    let code = r#"
def f(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_map() {
    let code = r#"
def f(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    assert!(transpile_ok(code));
}
