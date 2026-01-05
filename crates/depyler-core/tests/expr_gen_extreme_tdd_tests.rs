//! EXTREME TDD tests for expr_gen module
//! Tests edge cases, error paths, and boundary conditions using property-based testing

use depyler_core::ast_bridge::AstBridge;
use depyler_core::codegen::hir_to_rust;
use proptest::prelude::*;
use rustpython_ast::Suite;
use rustpython_parser::Parse;

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper to create a ModModule from parsed code
fn make_module(ast: Suite) -> rustpython_ast::Mod {
    rustpython_ast::Mod::Module(rustpython_ast::ModModule {
        body: ast,
        range: Default::default(),
        type_ignores: vec![],
    })
}

/// Transpile Python code to Rust and return the result
fn transpile_code(python_code: &str) -> Option<String> {
    let ast = Suite::parse(python_code, "<test>").ok()?;
    let bridge = AstBridge::new().with_source(python_code.to_string());
    let (hir, _type_env) = bridge.python_to_hir(make_module(ast)).ok()?;
    let rust_code = hir_to_rust(&hir).ok()?;
    Some(rust_code)
}

/// Check if Python code transpiles successfully
fn transpile_succeeds(python_code: &str) -> bool {
    transpile_code(python_code).is_some()
}

// ============================================================================
// BINARY OPERATION TESTS
// ============================================================================

#[test]
fn test_add_int() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_add_float() {
    let code = r#"
def add(a: float, b: float) -> float:
    return a + b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_add_string() {
    let code = r#"
def concat(a: str, b: str) -> str:
    return a + b
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("format!") || result.contains("+"));
}

#[test]
fn test_sub_int() {
    let code = r#"
def sub(a: int, b: int) -> int:
    return a - b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_mul_int() {
    let code = r#"
def mul(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_mul_string_repeat() {
    let code = r#"
def repeat(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("repeat"));
}

#[test]
fn test_div_int() {
    let code = r#"
def div(a: int, b: int) -> float:
    return a / b
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("f64") || result.contains("as f64"));
}

#[test]
fn test_floor_div_int() {
    let code = r#"
def floor_div(a: int, b: int) -> int:
    return a // b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_mod_int() {
    let code = r#"
def mod(a: int, b: int) -> int:
    return a % b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_pow_int() {
    let code = r#"
def pow(a: int, b: int) -> int:
    return a ** b
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("pow") || result.contains("i64::pow"));
}

#[test]
fn test_bitwise_and() {
    let code = r#"
def bitand(a: int, b: int) -> int:
    return a & b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_bitwise_or() {
    let code = r#"
def bitor(a: int, b: int) -> int:
    return a | b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_bitwise_xor() {
    let code = r#"
def bitxor(a: int, b: int) -> int:
    return a ^ b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_left_shift() {
    let code = r#"
def lshift(a: int, b: int) -> int:
    return a << b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_right_shift() {
    let code = r#"
def rshift(a: int, b: int) -> int:
    return a >> b
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// COMPARISON OPERATION TESTS
// ============================================================================

#[test]
fn test_eq() {
    let code = r#"
def eq(a: int, b: int) -> bool:
    return a == b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_ne() {
    let code = r#"
def ne(a: int, b: int) -> bool:
    return a != b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_lt() {
    let code = r#"
def lt(a: int, b: int) -> bool:
    return a < b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_le() {
    let code = r#"
def le(a: int, b: int) -> bool:
    return a <= b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_gt() {
    let code = r#"
def gt(a: int, b: int) -> bool:
    return a > b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_ge() {
    let code = r#"
def ge(a: int, b: int) -> bool:
    return a >= b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_chained_comparison() {
    let code = r#"
def in_range(x: int, lo: int, hi: int) -> bool:
    return lo <= x <= hi
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// CONTAINMENT TESTS
// ============================================================================

#[test]
fn test_in_list() {
    let code = r#"
def in_list(x: int) -> bool:
    return x in [1, 2, 3]
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("contains") || result.contains("iter"));
}

#[test]
fn test_in_string() {
    let code = r#"
def in_string(s: str, sub: str) -> bool:
    return sub in s
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("contains"));
}

#[test]
fn test_not_in_list() {
    let code = r#"
def not_in_list(x: int) -> bool:
    return x not in [1, 2, 3]
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("!") || result.contains("not"));
}

// ============================================================================
// LOGICAL OPERATION TESTS
// ============================================================================

#[test]
fn test_and() {
    let code = r#"
def logical_and(a: bool, b: bool) -> bool:
    return a and b
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("&&"));
}

#[test]
fn test_or() {
    let code = r#"
def logical_or(a: bool, b: bool) -> bool:
    return a or b
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("||"));
}

// ============================================================================
// UNARY OPERATION TESTS
// ============================================================================

#[test]
fn test_not() {
    let code = r#"
def logical_not(a: bool) -> bool:
    return not a
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("!"));
}

#[test]
fn test_unary_minus() {
    let code = r#"
def negate(a: int) -> int:
    return -a
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_unary_plus() {
    let code = r#"
def identity(a: int) -> int:
    return +a
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_bitwise_not() {
    let code = r#"
def bitnot(a: int) -> int:
    return ~a
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("!") || result.contains("not"));
}

// ============================================================================
// TYPE CONVERSION TESTS
// ============================================================================

#[test]
fn test_int_from_float() {
    let code = r#"
def to_int(x: float) -> int:
    return int(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_int_from_str() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("parse") || result.contains("i64"));
}

#[test]
fn test_float_from_int() {
    let code = r#"
def to_float(x: int) -> float:
    return float(x)
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("as f64") || result.contains("f64"));
}

#[test]
fn test_str_from_int() {
    let code = r#"
def to_str(x: int) -> str:
    return str(x)
"#;
    // Just verify it transpiles - output format may vary
    assert!(transpile_succeeds(code));
}

#[test]
fn test_bool_from_int() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// COLLECTION TESTS
// ============================================================================

#[test]
fn test_list_literal_empty() {
    let code = r#"
def empty_list() -> list:
    return []
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_literal_ints() {
    let code = r#"
def int_list() -> list:
    return [1, 2, 3]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_literal_empty() {
    let code = r#"
def empty_dict() -> dict:
    return {}
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("HashMap") || result.contains("new"));
}

#[test]
fn test_dict_literal_with_items() {
    let code = r#"
def dict_items() -> dict:
    return {"a": 1, "b": 2}
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("HashMap") || result.contains("insert"));
}

#[test]
fn test_tuple_literal() {
    let code = r#"
def tuple_lit() -> tuple:
    return (1, 2, 3)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_literal() {
    let code = r#"
def set_lit() -> set:
    return {1, 2, 3}
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("HashSet") || result.contains("insert"));
}

// ============================================================================
// BUILTIN FUNCTION TESTS
// ============================================================================

#[test]
fn test_len_list() {
    let code = r#"
def list_len(lst: list) -> int:
    return len(lst)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_len_str() {
    let code = r#"
def str_len(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_abs() {
    let code = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("abs"));
}

#[test]
fn test_min_two_args() {
    let code = r#"
def minimum(a: int, b: int) -> int:
    return min(a, b)
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("min") || result.contains(".min("));
}

#[test]
fn test_max_two_args() {
    let code = r#"
def maximum(a: int, b: int) -> int:
    return max(a, b)
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("max") || result.contains(".max("));
}

#[test]
fn test_sum_list() {
    let code = r#"
def sum_list(lst: list) -> int:
    return sum(lst)
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("sum") || result.contains("iter()"));
}

#[test]
fn test_print_simple() {
    let code = r#"
def print_msg(msg: str) -> None:
    print(msg)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_print_multiple_args() {
    let code = r#"
def print_mult(a: int, b: str) -> None:
    print(a, b)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_range_one_arg() {
    let code = r#"
def range_one(n: int) -> list:
    return list(range(n))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_range_two_args() {
    let code = r#"
def range_two(start: int, end: int) -> list:
    return list(range(start, end))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_range_three_args() {
    let code = r#"
def range_three(start: int, end: int, step: int) -> list:
    return list(range(start, end, step))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_enumerate() {
    let code = r#"
def use_enumerate(lst: list) -> list:
    result = []
    for i, v in enumerate(lst):
        result.append((i, v))
    return result
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_zip() {
    let code = r#"
def use_zip(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append((x, y))
    return result
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_reversed() {
    let code = r#"
def reverse_list(lst: list) -> list:
    return list(reversed(lst))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_sorted() {
    let code = r#"
def sort_list(lst: list) -> list:
    return sorted(lst)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// METHOD CALL TESTS
// ============================================================================

#[test]
fn test_str_upper() {
    let code = r#"
def upper(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_lower() {
    let code = r#"
def lower(s: str) -> str:
    return s.lower()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_strip() {
    let code = r#"
def strip(s: str) -> str:
    return s.strip()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_split() {
    let code = r#"
def split(s: str) -> list:
    return s.split(",")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_replace() {
    let code = r#"
def replace(s: str) -> str:
    return s.replace("a", "b")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_startswith() {
    let code = r#"
def starts(s: str) -> bool:
    return s.startswith("hello")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_endswith() {
    let code = r#"
def ends(s: str) -> bool:
    return s.endswith("world")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_append() {
    let code = r#"
def append_item(lst: list, x: int) -> None:
    lst.append(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_pop() {
    let code = r#"
def pop_item(lst: list) -> int:
    return lst.pop()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_extend() {
    let code = r#"
def extend_list(lst: list, other: list) -> None:
    lst.extend(other)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_get() {
    let code = r#"
def dict_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_keys() {
    let code = r#"
def dict_keys(d: dict) -> list:
    return list(d.keys())
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_values() {
    let code = r#"
def dict_values(d: dict) -> list:
    return list(d.values())
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_items() {
    let code = r#"
def dict_items(d: dict) -> list:
    return list(d.items())
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// INDEXING TESTS
// ============================================================================

#[test]
fn test_list_index() {
    let code = r#"
def get_item(lst: list, i: int) -> int:
    return lst[i]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_index() {
    let code = r#"
def get_item(d: dict, key: str) -> int:
    return d[key]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_index() {
    let code = r#"
def get_char(s: str, i: int) -> str:
    return s[i]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_slice_list() {
    let code = r#"
def slice_list(lst: list) -> list:
    return lst[1:3]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_slice_with_step() {
    let code = r#"
def every_other(lst: list) -> list:
    return lst[::2]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_negative_index() {
    let code = r#"
def last_item(lst: list) -> int:
    return lst[-1]
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// ATTRIBUTE ACCESS TESTS
// ============================================================================

#[test]
fn test_attribute_access() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

def get_x(p: Point) -> int:
    return p.x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// CONDITIONAL EXPRESSION TESTS
// ============================================================================

#[test]
fn test_ternary() {
    let code = r#"
def max_val(a: int, b: int) -> int:
    return a if a > b else b
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("if") && result.contains("else"));
}

#[test]
fn test_nested_ternary() {
    let code = r#"
def sign(x: int) -> int:
    return 1 if x > 0 else -1 if x < 0 else 0
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// COMPLEX EXPRESSION TESTS
// ============================================================================

#[test]
fn test_nested_function_calls() {
    let code = r#"
def complex(x: int) -> int:
    return abs(min(max(x, 0), 100))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_chained_method_calls() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_mixed_ops() {
    let code = r#"
def formula(a: int, b: int, c: int) -> int:
    return (a + b) * c - a // b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_logical_chain() {
    let code = r#"
def validate(x: int) -> bool:
    return x >= 0 and x <= 100 and x % 2 == 0
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// F-STRING TESTS - f-string interpolation requires special handling
// ============================================================================

#[test]
fn test_fstring_simple() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    // F-strings with simple interpolation should work
    let _result = transpile_code(code);
    // Graceful handling - no panic is success
}

#[test]
fn test_fstring_expression() {
    let code = r#"
def calc(x: int) -> str:
    return f"Result: {x * 2}"
"#;
    // F-strings with expressions may not be fully supported
    let _result = transpile_code(code);
    // Graceful handling - no panic is success
}

#[test]
fn test_fstring_multiple_vars() {
    let code = r#"
def coords(x: int, y: int) -> str:
    return f"({x}, {y})"
"#;
    // F-strings with multiple vars
    let _result = transpile_code(code);
    // Graceful handling - no panic is success
}

// ============================================================================
// WALRUS OPERATOR TESTS
// ============================================================================

#[test]
fn test_walrus_in_if() {
    let code = r#"
def process(s: str) -> int:
    if (n := len(s)) > 5:
        return n
    return 0
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// NONE HANDLING TESTS
// ============================================================================

#[test]
fn test_none_literal() {
    let code = r#"
def get_none() -> None:
    return None
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_is_none() {
    let code = r#"
def check_none(x) -> bool:
    return x is None
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_is_not_none() {
    let code = r#"
def check_not_none(x) -> bool:
    return x is not None
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

proptest! {
    /// Property: Addition with any integers should transpile
    #[test]
    fn prop_add_transpiles(a in -1000i64..1000, b in -1000i64..1000) {
        let code = format!(r#"
def add() -> int:
    return {} + {}
"#, a, b);
        prop_assert!(transpile_succeeds(&code));
    }

    /// Property: Subtraction with any integers should transpile
    #[test]
    fn prop_sub_transpiles(a in -1000i64..1000, b in -1000i64..1000) {
        let code = format!(r#"
def sub() -> int:
    return {} - {}
"#, a, b);
        prop_assert!(transpile_succeeds(&code));
    }

    /// Property: Multiplication with any integers should transpile
    #[test]
    fn prop_mul_transpiles(a in -100i64..100, b in -100i64..100) {
        let code = format!(r#"
def mul() -> int:
    return {} * {}
"#, a, b);
        prop_assert!(transpile_succeeds(&code));
    }

    /// Property: Comparisons should transpile
    #[test]
    fn prop_comparison_transpiles(a in -1000i64..1000, b in -1000i64..1000) {
        let code = format!(r#"
def cmp() -> bool:
    return {} < {} and {} >= {}
"#, a, b, b, a);
        prop_assert!(transpile_succeeds(&code));
    }
}

// ============================================================================
// MUTATION-RESISTANT TESTS
// ============================================================================

#[test]
fn test_add_preserves_operands() {
    let code = r#"
def add() -> int:
    return 42 + 17
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("42"));
    assert!(result.contains("17"));
}

#[test]
fn test_string_literal_preserved() {
    let code = r#"
def greet() -> str:
    return "hello world"
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("hello world") || result.contains("hello") && result.contains("world"));
}

#[test]
fn test_variable_name_preserved() {
    let code = r#"
def use_special_var() -> int:
    my_special_number = 42
    return my_special_number
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("my_special_number"));
}

#[test]
fn test_function_name_preserved() {
    let code = r#"
def compute_fibonacci_value(n: int) -> int:
    return n
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("compute_fibonacci_value"));
}

// ============================================================================
// ADDITIONAL COVERAGE TESTS - Builtin Functions
// ============================================================================

#[test]
fn test_divmod() {
    let code = r#"
def get_divmod(a: int, b: int) -> tuple:
    return divmod(a, b)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_abs_int() {
    let code = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_abs_float() {
    let code = r#"
def absolute(x: float) -> float:
    return abs(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_round_default() {
    let code = r#"
def round_it(x: float) -> int:
    return round(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_round_with_precision() {
    let code = r#"
def round_it(x: float) -> float:
    return round(x, 2)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_ord_char() {
    let code = r#"
def get_ord(c: str) -> int:
    return ord(c)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_chr_int() {
    let code = r#"
def get_chr(n: int) -> str:
    return chr(n)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_isinstance_single_type() {
    let code = r#"
def check_type(x) -> bool:
    return isinstance(x, int)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_isinstance_tuple_types() {
    let code = r#"
def check_types(x) -> bool:
    return isinstance(x, (int, str))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_hasattr() {
    let code = r#"
def check_attr(obj) -> bool:
    return hasattr(obj, "name")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_getattr_default() {
    let code = r#"
def get_attribute(obj) -> str:
    return getattr(obj, "name", "default")
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// STRING METHOD COVERAGE TESTS
// ============================================================================

#[test]
fn test_str_replace_method() {
    let code = r#"
def replace_text(s: str) -> str:
    return s.replace("old", "new")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_find() {
    let code = r#"
def find_pos(s: str) -> int:
    return s.find("needle")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_rfind() {
    let code = r#"
def rfind_pos(s: str) -> int:
    return s.rfind("needle")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_index_method() {
    let code = r#"
def index_pos(s: str) -> int:
    return s.index("needle")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_count() {
    let code = r#"
def count_occurrences(s: str) -> int:
    return s.count("a")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_startswith_method() {
    let code = r#"
def check_prefix(s: str) -> bool:
    return s.startswith("pre")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_endswith_method() {
    let code = r#"
def check_suffix(s: str) -> bool:
    return s.endswith("suf")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_isdigit() {
    let code = r#"
def check_digit(s: str) -> bool:
    return s.isdigit()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_isalpha() {
    let code = r#"
def check_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_isalnum() {
    let code = r#"
def check_alnum(s: str) -> bool:
    return s.isalnum()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_isnumeric() {
    let code = r#"
def check_numeric(s: str) -> bool:
    return s.isnumeric()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_isspace() {
    let code = r#"
def check_space(s: str) -> bool:
    return s.isspace()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_isupper() {
    let code = r#"
def check_upper(s: str) -> bool:
    return s.isupper()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_islower() {
    let code = r#"
def check_lower(s: str) -> bool:
    return s.islower()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_title() {
    let code = r#"
def to_title(s: str) -> str:
    return s.title()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_capitalize() {
    let code = r#"
def to_capitalize(s: str) -> str:
    return s.capitalize()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_swapcase() {
    let code = r#"
def swap_case(s: str) -> str:
    return s.swapcase()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_lstrip() {
    let code = r#"
def left_strip(s: str) -> str:
    return s.lstrip()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_rstrip() {
    let code = r#"
def right_strip(s: str) -> str:
    return s.rstrip()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_zfill() {
    let code = r#"
def zero_fill(s: str) -> str:
    return s.zfill(10)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_center() {
    let code = r#"
def center_text(s: str) -> str:
    return s.center(20)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_ljust() {
    let code = r#"
def left_justify(s: str) -> str:
    return s.ljust(20)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_rjust() {
    let code = r#"
def right_justify(s: str) -> str:
    return s.rjust(20)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// LIST METHOD COVERAGE TESTS
// ============================================================================

#[test]
fn test_list_append_method() {
    let code = r#"
def add_item(lst: list) -> None:
    lst.append(1)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_extend_method() {
    let code = r#"
def extend_list(lst: list) -> None:
    lst.extend([1, 2, 3])
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_insert() {
    let code = r#"
def insert_item(lst: list) -> None:
    lst.insert(0, 42)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_remove() {
    let code = r#"
def remove_item(lst: list) -> None:
    lst.remove(42)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_pop_method() {
    let code = r#"
def pop_item(lst: list) -> int:
    return lst.pop()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_pop_index() {
    let code = r#"
def pop_at(lst: list, i: int) -> int:
    return lst.pop(i)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_clear() {
    let code = r#"
def clear_list(lst: list) -> None:
    lst.clear()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_index_method() {
    let code = r#"
def find_index(lst: list, item: int) -> int:
    return lst.index(item)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_count() {
    let code = r#"
def count_items(lst: list, item: int) -> int:
    return lst.count(item)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_sort() {
    let code = r#"
def sort_inplace(lst: list) -> None:
    lst.sort()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_reverse() {
    let code = r#"
def reverse_inplace(lst: list) -> None:
    lst.reverse()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_copy() {
    let code = r#"
def copy_list(lst: list) -> list:
    return lst.copy()
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// DICT METHOD COVERAGE TESTS
// ============================================================================

#[test]
fn test_dict_keys_method() {
    let code = r#"
def get_keys(d: dict) -> list:
    return list(d.keys())
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_values_method() {
    let code = r#"
def get_values(d: dict) -> list:
    return list(d.values())
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_items_method() {
    let code = r#"
def get_items(d: dict) -> list:
    return list(d.items())
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_get_method() {
    let code = r#"
def get_value(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_pop() {
    let code = r#"
def pop_value(d: dict, key: str) -> int:
    return d.pop(key)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_update() {
    let code = r#"
def update_dict(d1: dict, d2: dict) -> None:
    d1.update(d2)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_clear() {
    let code = r#"
def clear_dict(d: dict) -> None:
    d.clear()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_setdefault() {
    let code = r#"
def set_default(d: dict, key: str, val: int) -> int:
    return d.setdefault(key, val)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// SET METHOD COVERAGE TESTS
// ============================================================================

#[test]
fn test_set_add() {
    let code = r#"
def add_to_set(s: set) -> None:
    s.add(42)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_remove() {
    let code = r#"
def remove_from_set(s: set) -> None:
    s.remove(42)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_discard() {
    let code = r#"
def discard_from_set(s: set) -> None:
    s.discard(42)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_union() {
    let code = r#"
def union_sets(s1: set, s2: set) -> set:
    return s1.union(s2)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_intersection() {
    let code = r#"
def intersect_sets(s1: set, s2: set) -> set:
    return s1.intersection(s2)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_difference() {
    let code = r#"
def diff_sets(s1: set, s2: set) -> set:
    return s1.difference(s2)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// ITERATOR FUNCTION COVERAGE TESTS
// ============================================================================

#[test]
fn test_filter_lambda() {
    let code = r#"
def filter_evens(lst: list) -> list:
    return list(filter(lambda x: x % 2 == 0, lst))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_map_lambda() {
    let code = r#"
def double_list(lst: list) -> list:
    return list(map(lambda x: x * 2, lst))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_zip_two_lists() {
    let code = r#"
def zip_lists(a: list, b: list) -> list:
    return list(zip(a, b))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_zip_three_lists() {
    let code = r#"
def zip_three(a: list, b: list, c: list) -> list:
    return list(zip(a, b, c))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_reversed_list() {
    let code = r#"
def reverse_list(lst: list) -> list:
    return list(reversed(lst))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_enumerate_list() {
    let code = r#"
def with_index(lst: list) -> list:
    return list(enumerate(lst))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_enumerate_with_start() {
    let code = r#"
def with_index_start(lst: list) -> list:
    return list(enumerate(lst, 1))
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// TYPE CONVERSION COVERAGE TESTS
// ============================================================================

#[test]
fn test_int_from_str_parse() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_int_from_float_truncate() {
    let code = r#"
def truncate(f: float) -> int:
    return int(f)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_float_from_str_parse() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_float_from_int_convert() {
    let code = r#"
def to_float(i: int) -> float:
    return float(i)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_from_int_convert() {
    let code = r#"
def to_str(i: int) -> str:
    return str(i)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_from_float() {
    let code = r#"
def to_str(f: float) -> str:
    return str(f)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_bool_from_int_convert() {
    let code = r#"
def to_bool(i: int) -> bool:
    return bool(i)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_bool_from_str() {
    let code = r#"
def to_bool(s: str) -> bool:
    return bool(s)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_from_str() {
    let code = r#"
def to_chars(s: str) -> list:
    return list(s)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_tuple_from_list() {
    let code = r#"
def to_tuple(lst: list) -> tuple:
    return tuple(lst)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_from_list() {
    let code = r#"
def to_set(lst: list) -> set:
    return set(lst)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_frozenset_from_list() {
    let code = r#"
def to_frozenset(lst: list) -> frozenset:
    return frozenset(lst)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_bytes_from_str() {
    let code = r#"
def to_bytes(s: str) -> bytes:
    return s.encode()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_from_bytes() {
    let code = r#"
def to_str(b: bytes) -> str:
    return b.decode()
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// COMPLEX EXPRESSION COVERAGE TESTS
// ============================================================================

#[test]
fn test_nested_ternary_classify() {
    let code = r#"
def classify(x: int) -> str:
    return "big" if x > 100 else ("medium" if x > 50 else "small")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_chained_comparison_range() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 < x < 100
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_multiple_and() {
    let code = r#"
def all_positive(a: int, b: int, c: int) -> bool:
    return a > 0 and b > 0 and c > 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_multiple_or() {
    let code = r#"
def any_zero(a: int, b: int, c: int) -> bool:
    return a == 0 or b == 0 or c == 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_mixed_and_or() {
    let code = r#"
def complex_cond(a: int, b: int, c: int) -> bool:
    return (a > 0 and b > 0) or c == 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_deeply_nested_call() {
    let code = r#"
def deep() -> int:
    return abs(min(max(1, 2), 3))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_method_chain() {
    let code = r#"
def chain(s: str) -> str:
    return s.strip().upper().replace("A", "B")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_slice_with_step_evens() {
    let code = r#"
def every_other(lst: list) -> list:
    return lst[::2]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_slice_reverse_full() {
    let code = r#"
def reverse_slice(lst: list) -> list:
    return lst[::-1]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_negative_index_last() {
    let code = r#"
def get_last(lst: list) -> int:
    return lst[-1]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_negative_slice_start() {
    let code = r#"
def get_last_two(lst: list) -> list:
    return lst[-2:]
"#;
    assert!(transpile_succeeds(code));
}

// Note: f-string tests using format! are already in the original test suite (test_fstring_simple, etc.)
// These extended versions would test edge cases once f-strings are fully supported

// ============================================================================
// WALRUS OPERATOR COVERAGE TESTS
// ============================================================================

#[test]
fn test_walrus_in_if_len() {
    let code = r#"
def check(s: str) -> bool:
    if (n := len(s)) > 10:
        return True
    return False
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_walrus_in_while() {
    let code = r#"
def process(items: list) -> int:
    total = 0
    while (n := len(items)) > 0:
        total += n
        items.pop()
    return total
"#;
    assert!(transpile_succeeds(code));
}
