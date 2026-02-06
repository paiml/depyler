//! Session 11: Deep coverage tests for expr_gen.rs
//!
//! Targets the #2 coverage bottleneck (68% covered, 3575 missed regions):
//! - Floor division with sign correction
//! - Dict merge operator
//! - Complex indexing patterns
//! - Slice operations (start/stop/step)
//! - Decimal/Fraction constructors
//! - Complex binary operations
//! - Unary operations
//! - Comprehension expressions
//! - Walrus operator in conditions
//! - Complex call patterns

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

// ============================================================================
// Floor division edge cases
// ============================================================================

#[test]
fn test_s11_expr_floor_div_basic() {
    let code = r#"
def floor_divide(a: int, b: int) -> int:
    return a // b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn floor_divide"),
        "Should transpile floor division. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_floor_div_float() {
    let code = r#"
def floor_divide_float(a: float, b: float) -> float:
    return a // b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn floor_divide_float"),
        "Should transpile float floor division. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_floor_div_negative() {
    let code = r#"
def negative_floor(a: int, b: int) -> int:
    x = -7 // 2
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn negative_floor"),
        "Should transpile negative floor division. Got: {}",
        result
    );
}

// ============================================================================
// Power operator edge cases
// ============================================================================

#[test]
fn test_s11_expr_power_int() {
    let code = r#"
def power_int(base: int, exp: int) -> int:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn power_int"),
        "Should transpile int power. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_power_float() {
    let code = r#"
def power_float(base: float, exp: float) -> float:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn power_float"),
        "Should transpile float power. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_power_literal_2() {
    let code = r#"
def square(x: int) -> int:
    return x ** 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn square"),
        "Should transpile square. Got: {}",
        result
    );
}

// ============================================================================
// Modulo operator
// ============================================================================

#[test]
fn test_s11_expr_modulo() {
    let code = r#"
def mod_op(a: int, b: int) -> int:
    return a % b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn mod_op"),
        "Should transpile modulo. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_string_format_percent() {
    let code = r#"
def fmt_str(name: str) -> str:
    return "Hello %s" % name
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fmt_str"),
        "Should transpile string format with %. Got: {}",
        result
    );
}

// ============================================================================
// Bitwise operations
// ============================================================================

#[test]
fn test_s11_expr_bitwise_and() {
    let code = r#"
def bit_and(a: int, b: int) -> int:
    return a & b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn bit_and"),
        "Should transpile bitwise AND. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_bitwise_or() {
    let code = r#"
def bit_or(a: int, b: int) -> int:
    return a | b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn bit_or"),
        "Should transpile bitwise OR. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_bitwise_xor() {
    let code = r#"
def bit_xor(a: int, b: int) -> int:
    return a ^ b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn bit_xor"),
        "Should transpile bitwise XOR. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_bitwise_not() {
    let code = r#"
def bit_not(a: int) -> int:
    return ~a
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn bit_not"),
        "Should transpile bitwise NOT. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_left_shift() {
    let code = r#"
def shift_left(a: int, n: int) -> int:
    return a << n
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn shift_left"),
        "Should transpile left shift. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_right_shift() {
    let code = r#"
def shift_right(a: int, n: int) -> int:
    return a >> n
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn shift_right"),
        "Should transpile right shift. Got: {}",
        result
    );
}

// ============================================================================
// Complex indexing patterns
// ============================================================================

#[test]
fn test_s11_expr_negative_index() {
    let code = r#"
def get_last(items: list) -> int:
    return items[-1]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_last"),
        "Should transpile negative index. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_nested_index() {
    let code = r#"
def get_nested(matrix: list, i: int, j: int) -> int:
    return matrix[i][j]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_nested"),
        "Should transpile nested index. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_string_index() {
    let code = r#"
def get_char(s: str, i: int) -> str:
    return s[i]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_char"),
        "Should transpile string index. Got: {}",
        result
    );
}

// ============================================================================
// Slice operations
// ============================================================================

#[test]
fn test_s11_expr_slice_start_stop() {
    let code = r#"
def sublist(items: list) -> list:
    return items[1:3]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sublist"),
        "Should transpile slice start:stop. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_slice_start_only() {
    let code = r#"
def from_index(items: list) -> list:
    return items[2:]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn from_index"),
        "Should transpile slice from start. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_slice_stop_only() {
    let code = r#"
def up_to(items: list) -> list:
    return items[:3]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn up_to"),
        "Should transpile slice to stop. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_slice_with_step() {
    let code = r#"
def every_other(items: list) -> list:
    return items[::2]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn every_other"),
        "Should transpile slice with step. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_slice_reverse() {
    let code = r#"
def reverse_list(items: list) -> list:
    return items[::-1]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn reverse_list"),
        "Should transpile reverse slice. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_string_slice() {
    let code = r#"
def substr(s: str) -> str:
    return s[1:4]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn substr"),
        "Should transpile string slice. Got: {}",
        result
    );
}

// ============================================================================
// Unary operations
// ============================================================================

#[test]
fn test_s11_expr_unary_neg_int() {
    let code = r#"
def negate(x: int) -> int:
    return -x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn negate"),
        "Should transpile unary negation. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_unary_neg_float() {
    let code = r#"
def negate_float(x: float) -> float:
    return -x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn negate_float"),
        "Should transpile float negation. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_unary_not() {
    let code = r#"
def logical_not(x: bool) -> bool:
    return not x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn logical_not"),
        "Should transpile logical not. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_unary_pos() {
    let code = r#"
def positive(x: int) -> int:
    return +x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn positive"),
        "Should transpile unary positive. Got: {}",
        result
    );
}

// ============================================================================
// Comprehension edge cases
// ============================================================================

#[test]
fn test_s11_expr_set_comprehension() {
    let code = r#"
def unique_squares(n: int) -> set:
    return {x * x for x in range(n)}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn unique_squares"),
        "Should transpile set comprehension. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_dict_comprehension() {
    let code = r#"
def index_map(items: list) -> dict:
    return {i: v for i, v in enumerate(items)}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn index_map"),
        "Should transpile dict comprehension. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_nested_comprehension() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn flatten"),
        "Should transpile nested comprehension. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_comprehension_with_condition() {
    let code = r#"
def evens(n: int) -> list:
    return [x for x in range(n) if x % 2 == 0]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn evens"),
        "Should transpile filtered comprehension. Got: {}",
        result
    );
}

// ============================================================================
// Walrus operator (named expressions)
// ============================================================================

#[test]
fn test_s11_expr_walrus_in_while() {
    let code = r#"
def read_chunks(data: list) -> list:
    result: list = []
    i = 0
    while (chunk := data[i:i+3]) and i < len(data):
        result.append(chunk)
        i += 3
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_chunks"),
        "Should transpile walrus in while. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_walrus_in_if() {
    let code = r#"
def find_match(items: list) -> int:
    if (n := len(items)) > 5:
        return n
    return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_match"),
        "Should transpile walrus in if. Got: {}",
        result
    );
}

// ============================================================================
// Complex boolean expressions
// ============================================================================

#[test]
fn test_s11_expr_chained_comparison() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 < x < 100
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn in_range"),
        "Should transpile chained comparison. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_complex_boolean_and() {
    let code = r#"
def check_all(a: bool, b: bool, c: bool) -> bool:
    return a and b and c
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_all"),
        "Should transpile chained AND. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_complex_boolean_or() {
    let code = r#"
def check_any(a: bool, b: bool, c: bool) -> bool:
    return a or b or c
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_any"),
        "Should transpile chained OR. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_mixed_boolean() {
    let code = r#"
def mixed_logic(a: bool, b: bool, c: bool) -> bool:
    return (a and b) or (not c)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn mixed_logic"),
        "Should transpile mixed boolean. Got: {}",
        result
    );
}

// ============================================================================
// String containment
// ============================================================================

#[test]
fn test_s11_expr_string_in() {
    let code = r#"
def has_substr(s: str, sub: str) -> bool:
    return sub in s
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn has_substr"),
        "Should transpile string in. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_string_not_in() {
    let code = r#"
def no_substr(s: str, sub: str) -> bool:
    return sub not in s
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn no_substr"),
        "Should transpile string not in. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_list_in() {
    let code = r#"
def contains_item(items: list, val: int) -> bool:
    return val in items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn contains_item"),
        "Should transpile list in. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_dict_in() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn has_key"),
        "Should transpile dict in. Got: {}",
        result
    );
}

// ============================================================================
// Ternary / conditional expressions
// ============================================================================

#[test]
fn test_s11_expr_ternary_with_call() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn abs_val"),
        "Should transpile ternary abs. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_ternary_nested() {
    let code = r#"
def classify(x: int) -> str:
    return "positive" if x > 0 else ("zero" if x == 0 else "negative")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn classify"),
        "Should transpile nested ternary. Got: {}",
        result
    );
}

// ============================================================================
// F-string edge cases
// ============================================================================

#[test]
fn test_s11_expr_fstring_format_spec() {
    let code = r#"
def format_float(x: float) -> str:
    return f"{x:.2f}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn format_float"),
        "Should transpile f-string format spec. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_fstring_padding() {
    let code = r#"
def pad_num(x: int) -> str:
    return f"{x:>10}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pad_num"),
        "Should transpile f-string padding. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_fstring_hex() {
    let code = r#"
def to_hex(x: int) -> str:
    return f"{x:#x}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_hex"),
        "Should transpile f-string hex. Got: {}",
        result
    );
}

// ============================================================================
// Lambda expressions
// ============================================================================

#[test]
fn test_s11_expr_lambda_no_args() {
    let code = r#"
def make_const():
    f = lambda: 42
    return f()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_const"),
        "Should transpile lambda no args. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_lambda_two_args() {
    let code = r#"
def make_adder():
    f = lambda x, y: x + y
    return f(3, 4)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_adder"),
        "Should transpile lambda two args. Got: {}",
        result
    );
}

// ============================================================================
// Type conversion expressions
// ============================================================================

#[test]
fn test_s11_expr_list_constructor() {
    let code = r#"
def to_list(s: str) -> list:
    return list(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_list"),
        "Should transpile list() constructor. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_set_constructor() {
    let code = r#"
def to_set(items: list) -> set:
    return set(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_set"),
        "Should transpile set() constructor. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_tuple_constructor() {
    let code = r#"
def to_tuple(items: list) -> tuple:
    return tuple(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_tuple"),
        "Should transpile tuple() constructor. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_dict_constructor() {
    let code = r#"
def make_dict() -> dict:
    pairs = [("a", 1), ("b", 2)]
    return dict(pairs)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_dict"),
        "Should transpile dict() constructor. Got: {}",
        result
    );
}

// ============================================================================
// Augmented assignment edge cases
// ============================================================================

#[test]
fn test_s11_expr_augmented_mul() {
    let code = r#"
def double_val(x: int) -> int:
    x *= 2
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn double_val"),
        "Should transpile *=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_augmented_div() {
    let code = r#"
def halve(x: float) -> float:
    x /= 2.0
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn halve"),
        "Should transpile /=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_augmented_floor_div() {
    let code = r#"
def floor_halve(x: int) -> int:
    x //= 2
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn floor_halve"),
        "Should transpile //=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_augmented_mod() {
    let code = r#"
def mod_assign(x: int) -> int:
    x %= 3
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn mod_assign"),
        "Should transpile %=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_augmented_pow() {
    let code = r#"
def pow_assign(x: int) -> int:
    x **= 3
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pow_assign"),
        "Should transpile **=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_augmented_bitand() {
    let code = r#"
def and_assign(x: int) -> int:
    x &= 0xFF
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn and_assign"),
        "Should transpile &=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_augmented_bitor() {
    let code = r#"
def or_assign(x: int) -> int:
    x |= 0x0F
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn or_assign"),
        "Should transpile |=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_augmented_bitxor() {
    let code = r#"
def xor_assign(x: int) -> int:
    x ^= 0xFF
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn xor_assign"),
        "Should transpile ^=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_augmented_lshift() {
    let code = r#"
def shift_left_assign(x: int) -> int:
    x <<= 2
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn shift_left_assign"),
        "Should transpile <<=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_augmented_rshift() {
    let code = r#"
def shift_right_assign(x: int) -> int:
    x >>= 2
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn shift_right_assign"),
        "Should transpile >>=. Got: {}",
        result
    );
}

// ============================================================================
// Generator expressions
// ============================================================================

#[test]
fn test_s11_expr_generator_in_sum() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sum_squares"),
        "Should transpile generator in sum. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_generator_in_any() {
    let code = r#"
def has_positive(items: list) -> bool:
    return any(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn has_positive"),
        "Should transpile generator in any. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_generator_in_all() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn all_positive"),
        "Should transpile generator in all. Got: {}",
        result
    );
}

// ============================================================================
// Starred expressions
// ============================================================================

#[test]
fn test_s11_expr_star_unpack_assign() {
    let code = r#"
def first_and_rest(items: list) -> int:
    first = items[0]
    rest = items[1:]
    return first
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn first_and_rest"),
        "Should transpile star unpack. Got: {}",
        result
    );
}

// ============================================================================
// None comparisons
// ============================================================================

#[test]
fn test_s11_expr_is_none() {
    let code = r#"
def check_none(x) -> bool:
    return x is None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_none"),
        "Should transpile is None. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_is_not_none() {
    let code = r#"
def check_not_none(x) -> bool:
    return x is not None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_not_none"),
        "Should transpile is not None. Got: {}",
        result
    );
}

// ============================================================================
// String multiplication
// ============================================================================

#[test]
fn test_s11_expr_string_multiply() {
    let code = r#"
def repeat_str(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn repeat_str"),
        "Should transpile string multiply. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_string_multiply_literal() {
    let code = r#"
def dashes() -> str:
    return "-" * 40
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn dashes"),
        "Should transpile literal string multiply. Got: {}",
        result
    );
}

// ============================================================================
// Comparison with different types
// ============================================================================

#[test]
fn test_s11_expr_compare_strings() {
    let code = r#"
def str_equal(a: str, b: str) -> bool:
    return a == b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn str_equal"),
        "Should transpile string comparison. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_compare_greater_equal() {
    let code = r#"
def gte(a: int, b: int) -> bool:
    return a >= b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn gte"),
        "Should transpile >=. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_compare_less_equal() {
    let code = r#"
def lte(a: int, b: int) -> bool:
    return a <= b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn lte"),
        "Should transpile <=. Got: {}",
        result
    );
}
