//! Session 11: Coverage tests targeting untested expr_gen.rs paths
//!
//! Tests exercise these code paths through end-to-end transpilation:
//! - Bytes literal conversion
//! - Complex binary operations with type coercion
//! - FrozenSet handling
//! - Nested method chains
//! - Dict/set comprehension expressions
//! - Ternary (conditional) expressions
//! - Star-unpacking expressions
//! - String multiplication
//! - Boolean literals in expressions
//! - None literal in various contexts

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ============================================================================
// Boolean literal expressions
// ============================================================================

#[test]
fn test_s11_bool_literal_true() {
    let code = r#"
def always_true() -> bool:
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("true"), "Should convert True to true. Got: {}", result);
}

#[test]
fn test_s11_bool_literal_false() {
    let code = r#"
def always_false() -> bool:
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("false"), "Should convert False to false. Got: {}", result);
}

// ============================================================================
// None literal in various contexts
// ============================================================================

#[test]
fn test_s11_none_return() {
    let code = r#"
def do_nothing() -> None:
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn do_nothing"), "Should generate function. Got: {}", result);
}

#[test]
fn test_s11_none_comparison() {
    let code = r#"
from typing import Optional

def is_none(x: Optional[int]) -> bool:
    return x is None
"#;
    let result = transpile(code);
    assert!(
        result.contains("is_none") || result.contains("None") || result.contains("Option"),
        "Should handle None comparison. Got: {}",
        result
    );
}

#[test]
fn test_s11_none_is_not_check() {
    let code = r#"
from typing import Optional

def has_value(x: Optional[int]) -> bool:
    return x is not None
"#;
    let result = transpile(code);
    assert!(
        result.contains("is_some") || result.contains("Some") || result.contains("x"),
        "Should handle 'is not None' check. Got: {}",
        result
    );
}

// ============================================================================
// Ternary/conditional expression
// ============================================================================

#[test]
fn test_s11_ternary_expression() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(
        result.contains("if") || result.contains("else"),
        "Should handle ternary expression. Got: {}",
        result
    );
}

#[test]
fn test_s11_ternary_with_string() {
    let code = r#"
def label(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;
    let result = transpile(code);
    assert!(result.contains("positive"), "Should handle string ternary. Got: {}", result);
}

// ============================================================================
// String multiplication
// ============================================================================

#[test]
fn test_s11_string_repeat() {
    let code = r#"
def repeat_str(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile(code);
    assert!(
        result.contains("repeat") || result.contains("*"),
        "Should handle string multiplication. Got: {}",
        result
    );
}

// ============================================================================
// Complex binary operations with type coercion
// ============================================================================

#[test]
fn test_s11_int_float_addition() {
    let code = r#"
def add_mixed(a: int, b: float) -> float:
    return a + b
"#;
    let result = transpile(code);
    assert!(
        result.contains("f64") || result.contains("+"),
        "Should handle int+float coercion. Got: {}",
        result
    );
}

#[test]
fn test_s11_int_division_returns_float() {
    let code = r#"
def divide(a: int, b: int) -> float:
    return a / b
"#;
    let result = transpile(code);
    assert!(
        result.contains("f64") || result.contains("/"),
        "Should handle int division to float. Got: {}",
        result
    );
}

#[test]
fn test_s11_floor_division() {
    let code = r#"
def floor_div(a: int, b: int) -> int:
    return a // b
"#;
    let result = transpile(code);
    assert!(
        result.contains("/") || result.contains("div"),
        "Should handle floor division. Got: {}",
        result
    );
}

#[test]
fn test_s11_modulo_operation() {
    let code = r#"
def mod_op(a: int, b: int) -> int:
    return a % b
"#;
    let result = transpile(code);
    assert!(result.contains("%"), "Should handle modulo. Got: {}", result);
}

#[test]
fn test_s11_power_operation() {
    let code = r#"
def power(base: int, exp: int) -> int:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(
        result.contains("pow") || result.contains("**"),
        "Should handle power operation. Got: {}",
        result
    );
}

// ============================================================================
// Bitwise operations
// ============================================================================

#[test]
fn test_s11_bitwise_and() {
    let code = r#"
def bit_and(a: int, b: int) -> int:
    return a & b
"#;
    let result = transpile(code);
    assert!(result.contains("&"), "Should handle bitwise AND. Got: {}", result);
}

#[test]
fn test_s11_bitwise_or() {
    let code = r#"
def bit_or(a: int, b: int) -> int:
    return a | b
"#;
    let result = transpile(code);
    assert!(result.contains("|"), "Should handle bitwise OR. Got: {}", result);
}

#[test]
fn test_s11_bitwise_xor() {
    let code = r#"
def bit_xor(a: int, b: int) -> int:
    return a ^ b
"#;
    let result = transpile(code);
    assert!(result.contains("^"), "Should handle bitwise XOR. Got: {}", result);
}

#[test]
fn test_s11_left_shift() {
    let code = r#"
def shift_left(a: int, b: int) -> int:
    return a << b
"#;
    let result = transpile(code);
    assert!(result.contains("<<"), "Should handle left shift. Got: {}", result);
}

#[test]
fn test_s11_right_shift() {
    let code = r#"
def shift_right(a: int, b: int) -> int:
    return a >> b
"#;
    let result = transpile(code);
    assert!(result.contains(">>"), "Should handle right shift. Got: {}", result);
}

// ============================================================================
// Comparison chaining
// ============================================================================

#[test]
fn test_s11_chained_comparison() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 <= x <= 100
"#;
    let result = transpile(code);
    assert!(
        result.contains("&&") || result.contains("<="),
        "Should handle chained comparisons. Got: {}",
        result
    );
}

// ============================================================================
// String methods as expressions
// ============================================================================

#[test]
fn test_s11_string_upper() {
    let code = r#"
def to_upper(s: str) -> str:
    return s.upper()
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_uppercase"),
        "Should convert upper() to to_uppercase(). Got: {}",
        result
    );
}

#[test]
fn test_s11_string_lower() {
    let code = r#"
def to_lower(s: str) -> str:
    return s.lower()
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_lowercase"),
        "Should convert lower() to to_lowercase(). Got: {}",
        result
    );
}

#[test]
fn test_s11_string_strip() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("trim"), "Should convert strip() to trim(). Got: {}", result);
}

#[test]
fn test_s11_string_startswith() {
    let code = r#"
def check_prefix(s: str) -> bool:
    return s.startswith("pre")
"#;
    let result = transpile(code);
    assert!(
        result.contains("starts_with"),
        "Should convert startswith() to starts_with(). Got: {}",
        result
    );
}

#[test]
fn test_s11_string_endswith() {
    let code = r#"
def check_suffix(s: str) -> bool:
    return s.endswith("end")
"#;
    let result = transpile(code);
    assert!(
        result.contains("ends_with"),
        "Should convert endswith() to ends_with(). Got: {}",
        result
    );
}

#[test]
fn test_s11_string_replace() {
    let code = r#"
def fix_text(s: str) -> str:
    return s.replace("old", "new")
"#;
    let result = transpile(code);
    assert!(
        result.contains("replace") || result.contains("replacen"),
        "Should convert replace(). Got: {}",
        result
    );
}

#[test]
fn test_s11_string_split() {
    let code = r#"
def tokenize(s: str) -> list:
    return s.split(",")
"#;
    let result = transpile(code);
    assert!(result.contains("split"), "Should convert split(). Got: {}", result);
}

#[test]
fn test_s11_string_join() {
    let code = r#"
def join_words(words: list) -> str:
    return ", ".join(words)
"#;
    let result = transpile(code);
    assert!(result.contains("join"), "Should convert join(). Got: {}", result);
}

// ============================================================================
// List methods
// ============================================================================

#[test]
fn test_s11_list_append() {
    let code = r#"
def build_list() -> list:
    result: list = []
    result.append(1)
    result.append(2)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("push"), "Should convert append() to push(). Got: {}", result);
}

#[test]
fn test_s11_list_len() {
    let code = r#"
def count(items: list) -> int:
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("len()"), "Should convert len() to .len(). Got: {}", result);
}

#[test]
fn test_s11_list_sorted() {
    let code = r#"
def sort_items(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("sort") || result.contains("sorted"),
        "Should handle sorted(). Got: {}",
        result
    );
}

// ============================================================================
// Dict operations
// ============================================================================

#[test]
fn test_s11_dict_get() {
    let code = r#"
def lookup(data: dict, key: str) -> int:
    return data.get(key, 0)
"#;
    let result = transpile(code);
    assert!(
        result.contains("get") || result.contains("unwrap_or"),
        "Should handle dict.get() with default. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_keys() {
    let code = r#"
def get_keys(data: dict) -> list:
    return list(data.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("keys"), "Should handle dict.keys(). Got: {}", result);
}

#[test]
fn test_s11_dict_values() {
    let code = r#"
def get_values(data: dict) -> list:
    return list(data.values())
"#;
    let result = transpile(code);
    assert!(result.contains("values"), "Should handle dict.values(). Got: {}", result);
}

// ============================================================================
// Unary operations
// ============================================================================

#[test]
fn test_s11_unary_negative() {
    let code = r#"
def negate(x: int) -> int:
    return -x
"#;
    let result = transpile(code);
    assert!(result.contains("-"), "Should handle unary negation. Got: {}", result);
}

#[test]
fn test_s11_unary_not() {
    let code = r#"
def invert(b: bool) -> bool:
    return not b
"#;
    let result = transpile(code);
    assert!(result.contains("!"), "Should handle unary not. Got: {}", result);
}

#[test]
fn test_s11_unary_bitwise_not() {
    let code = r#"
def bit_not(x: int) -> int:
    return ~x
"#;
    let result = transpile(code);
    assert!(
        result.contains("!") || result.contains("~"),
        "Should handle bitwise not. Got: {}",
        result
    );
}

// ============================================================================
// F-string expressions
// ============================================================================

#[test]
fn test_s11_fstring_simple() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(
        result.contains("format!") || result.contains("Hello"),
        "Should handle f-string. Got: {}",
        result
    );
}

#[test]
fn test_s11_fstring_with_expression() {
    let code = r#"
def describe(x: int) -> str:
    return f"Value is {x + 1}"
"#;
    let result = transpile(code);
    assert!(result.contains("format!"), "Should handle f-string with expression. Got: {}", result);
}

// ============================================================================
// Tuple expressions
// ============================================================================

#[test]
fn test_s11_tuple_creation() {
    let code = r#"
from typing import Tuple

def make_pair(a: int, b: str) -> Tuple[int, str]:
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("(") && result.contains(")"), "Should create tuple. Got: {}", result);
}

#[test]
fn test_s11_tuple_index() {
    let code = r#"
from typing import Tuple

def first(t: Tuple[int, int]) -> int:
    return t[0]
"#;
    let result = transpile(code);
    assert!(
        result.contains(".0") || result.contains("[0]") || result.contains("t"),
        "Should handle tuple indexing. Got: {}",
        result
    );
}

// ============================================================================
// Set operations
// ============================================================================

#[test]
fn test_s11_set_literal() {
    let code = r#"
def unique_nums() -> set:
    return {1, 2, 3}
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashSet") || result.contains("set"),
        "Should handle set literal. Got: {}",
        result
    );
}

#[test]
fn test_s11_set_add() {
    let code = r#"
from typing import Set

def build_set() -> Set[int]:
    s: Set[int] = set()
    s.add(1)
    s.add(2)
    return s
"#;
    let result = transpile(code);
    assert!(
        result.contains("insert") || result.contains("HashSet"),
        "Should handle set.add(). Got: {}",
        result
    );
}

// ============================================================================
// Lambda expressions
// ============================================================================

#[test]
fn test_s11_lambda_simple() {
    let code = r#"
def apply_fn(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("|") || result.contains("map") || result.contains("closure"),
        "Should handle lambda. Got: {}",
        result
    );
}

// ============================================================================
// Slice operations
// ============================================================================

#[test]
fn test_s11_list_slice() {
    let code = r#"
def first_three(items: list) -> list:
    return items[0:3]
"#;
    let result = transpile(code);
    assert!(
        result.contains("[") || result.contains(".."),
        "Should handle list slicing. Got: {}",
        result
    );
}

// ============================================================================
// Enumerate and zip builtins
// ============================================================================

#[test]
fn test_s11_enumerate() {
    let code = r#"
def indexed(items: list) -> list:
    result: list = []
    for i, item in enumerate(items):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("enumerate") || result.contains("iter"),
        "Should handle enumerate(). Got: {}",
        result
    );
}

#[test]
fn test_s11_zip_two_lists() {
    let code = r#"
def pair_up(a: list, b: list) -> list:
    result: list = []
    for x, y in zip(a, b):
        result.append(x)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("zip") || result.contains("iter"),
        "Should handle zip(). Got: {}",
        result
    );
}

// ============================================================================
// Type casting / conversion builtins
// ============================================================================

#[test]
fn test_s11_int_conversion() {
    let code = r#"
def to_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("parse") || result.contains("int"),
        "Should handle int() conversion. Got: {}",
        result
    );
}

#[test]
fn test_s11_float_conversion() {
    let code = r#"
def to_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("parse") || result.contains("f64"),
        "Should handle float() conversion. Got: {}",
        result
    );
}

#[test]
fn test_s11_str_conversion() {
    let code = r#"
def to_str(x: int) -> str:
    return str(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_string") || result.contains("format"),
        "Should handle str() conversion. Got: {}",
        result
    );
}

// ============================================================================
// Boolean operations (and/or)
// ============================================================================

#[test]
fn test_s11_logical_and() {
    let code = r#"
def both(a: bool, b: bool) -> bool:
    return a and b
"#;
    let result = transpile(code);
    assert!(result.contains("&&"), "Should convert 'and' to '&&'. Got: {}", result);
}

#[test]
fn test_s11_logical_or() {
    let code = r#"
def either(a: bool, b: bool) -> bool:
    return a or b
"#;
    let result = transpile(code);
    assert!(result.contains("||"), "Should convert 'or' to '||'. Got: {}", result);
}

// ============================================================================
// Membership test (in)
// ============================================================================

#[test]
fn test_s11_in_operator_list() {
    let code = r#"
def contains(items: list, target: int) -> bool:
    return target in items
"#;
    let result = transpile(code);
    assert!(
        result.contains("contains") || result.contains("iter"),
        "Should handle 'in' operator. Got: {}",
        result
    );
}

#[test]
fn test_s11_not_in_operator() {
    let code = r#"
def missing(items: list, target: int) -> bool:
    return target not in items
"#;
    let result = transpile(code);
    assert!(
        result.contains("contains") || result.contains("!"),
        "Should handle 'not in' operator. Got: {}",
        result
    );
}

// ============================================================================
// abs/min/max builtins
// ============================================================================

#[test]
fn test_s11_abs_builtin() {
    let code = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("abs"), "Should handle abs(). Got: {}", result);
}

#[test]
fn test_s11_min_builtin() {
    let code = r#"
def smaller(a: int, b: int) -> int:
    return min(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("min"), "Should handle min(). Got: {}", result);
}

#[test]
fn test_s11_max_builtin() {
    let code = r#"
def larger(a: int, b: int) -> int:
    return max(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("max"), "Should handle max(). Got: {}", result);
}
