//! Session 11 Batch 2: Set/dict operators, comprehensions, walrus, power
//!
//! Targets:
//! - expr_gen.rs:839 dict merge | operator
//! - expr_gen.rs:850 set operators (|, &, -, ^)
//! - expr_gen.rs:1183 power operator edge cases
//! - expr_gen.rs:10777 comprehension variants (ListComp, SetComp, DictComp, GeneratorExp)
//! - expr_gen.rs:10808 NamedExpr (walrus operator)
//! - direct_rules_convert.rs:1873 multi-generator comprehensions

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ===== Set operators =====

#[test]
fn test_s11b2_set_union() {
    let code = r#"
def unite(a: set, b: set) -> set:
    return a | b
"#;
    let result = transpile(code);
    assert!(result.contains("fn unite"), "Got: {}", result);
}

#[test]
fn test_s11b2_set_intersection() {
    let code = r#"
def common(a: set, b: set) -> set:
    return a & b
"#;
    let result = transpile(code);
    assert!(result.contains("fn common"), "Got: {}", result);
}

#[test]
fn test_s11b2_set_difference() {
    let code = r#"
def diff(a: set, b: set) -> set:
    return a - b
"#;
    let result = transpile(code);
    assert!(result.contains("fn diff"), "Got: {}", result);
}

#[test]
fn test_s11b2_set_symmetric_difference() {
    let code = r#"
def sym_diff(a: set, b: set) -> set:
    return a ^ b
"#;
    let result = transpile(code);
    assert!(result.contains("fn sym_diff"), "Got: {}", result);
}

// ===== Dict merge =====

#[test]
fn test_s11b2_dict_merge() {
    let code = r#"
def merge_dicts(a: dict, b: dict) -> dict:
    return a | b
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_dicts"), "Got: {}", result);
}

// ===== Power operator =====

#[test]
fn test_s11b2_power_int() {
    let code = r#"
def square(x: int) -> int:
    return x ** 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn square"), "Got: {}", result);
}

#[test]
fn test_s11b2_power_float() {
    let code = r#"
def cube_root(x: float) -> float:
    return x ** (1.0 / 3.0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn cube_root"), "Got: {}", result);
}

#[test]
fn test_s11b2_power_variable() {
    let code = r#"
def power(base: int, exp: int) -> int:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s11b2_power_literal_three() {
    let code = r#"
def cube(x: int) -> int:
    return x ** 3
"#;
    let result = transpile(code);
    assert!(result.contains("fn cube"), "Got: {}", result);
}

// ===== Comprehension variants =====

#[test]
fn test_s11b2_list_comp_simple() {
    let code = r#"
def doubles(items: list) -> list:
    return [x * 2 for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn doubles"), "Got: {}", result);
}

#[test]
fn test_s11b2_list_comp_filter() {
    let code = r#"
def positives(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn positives"), "Got: {}", result);
}

#[test]
fn test_s11b2_list_comp_nested() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

#[test]
fn test_s11b2_set_comp() {
    let code = r#"
def unique_abs(items: list) -> set:
    return {abs(x) for x in items}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_abs"), "Got: {}", result);
}

#[test]
fn test_s11b2_set_comp_filter() {
    let code = r#"
def positive_set(items: list) -> set:
    return {x for x in items if x > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_set"), "Got: {}", result);
}

#[test]
fn test_s11b2_dict_comp() {
    let code = r#"
def index_map(items: list) -> dict:
    return {i: v for i, v in enumerate(items)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn index_map"), "Got: {}", result);
}

#[test]
fn test_s11b2_dict_comp_filter() {
    let code = r#"
def positive_map(d: dict) -> dict:
    return {k: v for k, v in d.items() if v > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_map"), "Got: {}", result);
}

#[test]
fn test_s11b2_generator_in_sum() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_squares"), "Got: {}", result);
}

#[test]
fn test_s11b2_generator_in_any() {
    let code = r#"
def has_neg(items: list) -> bool:
    return any(x < 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_neg"), "Got: {}", result);
}

#[test]
fn test_s11b2_generator_in_all() {
    let code = r#"
def all_pos(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_pos"), "Got: {}", result);
}

#[test]
fn test_s11b2_generator_in_min() {
    let code = r#"
def min_abs(items: list) -> int:
    return min(abs(x) for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_abs"), "Got: {}", result);
}

#[test]
fn test_s11b2_generator_in_max() {
    let code = r#"
def max_abs(items: list) -> int:
    return max(abs(x) for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_abs"), "Got: {}", result);
}

// ===== Walrus operator =====

#[test]
fn test_s11b2_walrus_in_if() {
    let code = r#"
def check_len(text: str) -> bool:
    if (n := len(text)) > 10:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_len"), "Got: {}", result);
}

#[test]
fn test_s11b2_walrus_in_while() {
    let code = r#"
def process(items: list) -> int:
    total = 0
    idx = 0
    while (n := len(items)) > idx:
        total += items[idx]
        idx += 1
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

// ===== Chained comparisons =====

#[test]
fn test_s11b2_chained_lt() {
    let code = r#"
def in_range(x: int, lo: int, hi: int) -> bool:
    return lo <= x <= hi
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Got: {}", result);
}

#[test]
fn test_s11b2_chained_lt_lt() {
    let code = r#"
def strictly_between(x: int, a: int, b: int) -> bool:
    return a < x < b
"#;
    let result = transpile(code);
    assert!(result.contains("fn strictly_between"), "Got: {}", result);
}

// ===== Ternary expressions =====

#[test]
fn test_s11b2_ternary_basic() {
    let code = r#"
def absolute(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn absolute"), "Got: {}", result);
}

#[test]
fn test_s11b2_ternary_nested() {
    let code = r#"
def sign(x: int) -> int:
    return 1 if x > 0 else (0 if x == 0 else -1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sign"), "Got: {}", result);
}

#[test]
fn test_s11b2_ternary_in_assignment() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    result = lo if x < lo else (hi if x > hi else x)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"), "Got: {}", result);
}

// ===== Augmented assignments =====

#[test]
fn test_s11b2_augmented_mul() {
    let code = r#"
def factorial(n: int) -> int:
    result = 1
    for i in range(1, n + 1):
        result *= i
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn factorial"), "Got: {}", result);
}

#[test]
fn test_s11b2_augmented_floor_div() {
    let code = r#"
def halve(n: int) -> int:
    count = 0
    while n > 1:
        n //= 2
        count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn halve"), "Got: {}", result);
}

#[test]
fn test_s11b2_augmented_mod() {
    let code = r#"
def reduce_mod(x: int, m: int) -> int:
    x %= m
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn reduce_mod"), "Got: {}", result);
}

#[test]
fn test_s11b2_augmented_pow() {
    let code = r#"
def square_in_place(x: int) -> int:
    x **= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn square_in_place"), "Got: {}", result);
}

#[test]
fn test_s11b2_augmented_bitand() {
    let code = r#"
def mask(x: int, m: int) -> int:
    x &= m
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn mask"), "Got: {}", result);
}

#[test]
fn test_s11b2_augmented_bitor() {
    let code = r#"
def set_bits(x: int, bits: int) -> int:
    x |= bits
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_bits"), "Got: {}", result);
}

#[test]
fn test_s11b2_augmented_bitxor() {
    let code = r#"
def toggle_bits(x: int, bits: int) -> int:
    x ^= bits
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn toggle_bits"), "Got: {}", result);
}

#[test]
fn test_s11b2_augmented_lshift() {
    let code = r#"
def shift_left(x: int, n: int) -> int:
    x <<= n
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_left"), "Got: {}", result);
}

#[test]
fn test_s11b2_augmented_rshift() {
    let code = r#"
def shift_right(x: int, n: int) -> int:
    x >>= n
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_right"), "Got: {}", result);
}

// ===== Tuple return/unpack patterns =====

#[test]
fn test_s11b2_return_tuple() {
    let code = r#"
def divmod_manual(a: int, b: int) -> tuple:
    return (a // b, a % b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn divmod_manual"), "Got: {}", result);
}

#[test]
fn test_s11b2_tuple_swap() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
}

#[test]
fn test_s11b2_triple_unpack() {
    let code = r#"
def sum_triple(t: tuple) -> int:
    a, b, c = t
    return a + b + c
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_triple"), "Got: {}", result);
}
