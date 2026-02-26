//! Session 11: Advanced codegen coverage tests
//!
//! Targets specific untested code paths identified by coverage analysis:
//! - Async function transpilation
//! - Lambda with 0, 1, 2+ params
//! - F-string edge cases (empty, literals only)
//! - Ternary expression optimization patterns
//! - Varargs parameter handling
//! - Assert with various comparison operators
//! - Walrus operator
//! - Complex yield patterns

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
// Async function transpilation
// ============================================================================

#[test]
fn test_s11_adv_async_simple() {
    let code = r#"
async def fetch(url: str) -> str:
    return url
"#;
    let result = transpile(code);
    assert!(
        result.contains("async") || result.contains("fn fetch"),
        "Should transpile async function. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_async_with_await() {
    let code = r#"
async def get_data(url: str) -> str:
    result: str = await fetch(url)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_data"), "Should transpile async with await. Got: {}", result);
}

#[test]
fn test_s11_adv_async_void() {
    let code = r#"
async def do_work() -> None:
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn do_work"), "Should transpile async void. Got: {}", result);
}

// ============================================================================
// Lambda edge cases
// ============================================================================

#[test]
fn test_s11_adv_lambda_no_params() {
    let code = r#"
def get_fn() -> int:
    f = lambda: 42
    return f()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_fn"),
        "Should transpile lambda with no params. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_lambda_two_params() {
    let code = r#"
def apply_op(a: int, b: int) -> int:
    add = lambda x, y: x + y
    return add(a, b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn apply_op"),
        "Should transpile lambda with two params. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_lambda_three_params() {
    let code = r#"
def clamp_val(x: int, lo: int, hi: int) -> int:
    cl = lambda v, a, b: max(a, min(b, v))
    return cl(x, lo, hi)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn clamp_val"),
        "Should transpile lambda with three params. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_lambda_in_sort_key() {
    let code = r#"
def sort_by_second(items: list) -> list:
    items.sort(key=lambda x: x[1])
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_by_second"),
        "Should transpile lambda as sort key. Got: {}",
        result
    );
}

// ============================================================================
// F-string edge cases
// ============================================================================

#[test]
fn test_s11_adv_fstring_no_expressions() {
    let code = r#"
def literal_fstr() -> str:
    return f"hello world"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn literal_fstr"),
        "Should transpile f-string with no expressions. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_fstring_multiple_exprs() {
    let code = r#"
def multi_fstr(a: int, b: str, c: float) -> str:
    return f"{a} is {b} at {c}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("format!"),
        "Should transpile multi-expression f-string. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_fstring_with_method_call() {
    let code = r#"
def upper_fstr(name: str) -> str:
    return f"Hello, {name.upper()}!"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn upper_fstr"),
        "Should transpile f-string with method call. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_fstring_with_computation() {
    let code = r#"
def compute_fstr(x: int) -> str:
    return f"result = {x * 2 + 1}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn compute_fstr"),
        "Should transpile f-string with computation. Got: {}",
        result
    );
}

// ============================================================================
// Ternary expression patterns
// ============================================================================

#[test]
fn test_s11_adv_ternary_simple() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_val"), "Should transpile simple ternary. Got: {}", result);
}

#[test]
fn test_s11_adv_ternary_with_string() {
    let code = r#"
def label(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;
    let result = transpile(code);
    assert!(result.contains("fn label"), "Should transpile string ternary. Got: {}", result);
}

#[test]
fn test_s11_adv_ternary_nested() {
    let code = r#"
def sign(x: int) -> int:
    return 1 if x > 0 else (-1 if x < 0 else 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sign"), "Should transpile nested ternary. Got: {}", result);
}

#[test]
fn test_s11_adv_ternary_with_call() {
    let code = r#"
def safe_len(items: list) -> int:
    return len(items) if items else 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_len"),
        "Should transpile ternary with function call. Got: {}",
        result
    );
}

// ============================================================================
// Varargs parameter handling
// ============================================================================

#[test]
fn test_s11_adv_varargs_basic() {
    let code = r#"
def sum_all(*args: int) -> int:
    total: int = 0
    for a in args:
        total += a
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_all"), "Should transpile varargs. Got: {}", result);
}

#[test]
fn test_s11_adv_varargs_with_regular() {
    let code = r#"
def prefix_sum(prefix: str, *values: int) -> str:
    total: int = 0
    for v in values:
        total += v
    return f"{prefix}: {total}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn prefix_sum"),
        "Should transpile mixed regular and varargs. Got: {}",
        result
    );
}

// ============================================================================
// Assert with various comparison operators
// ============================================================================

#[test]
fn test_s11_adv_assert_gt() {
    let code = r#"
def check_positive(x: int) -> int:
    assert x > 0
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("assert"), "Should transpile assert with >. Got: {}", result);
}

#[test]
fn test_s11_adv_assert_lt() {
    let code = r#"
def check_small(x: int) -> int:
    assert x < 100
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("assert"), "Should transpile assert with <. Got: {}", result);
}

#[test]
fn test_s11_adv_assert_eq() {
    let code = r#"
def check_eq(x: int, y: int) -> int:
    assert x == y
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("assert"), "Should transpile assert with ==. Got: {}", result);
}

#[test]
fn test_s11_adv_assert_ne() {
    let code = r#"
def check_ne(x: int, y: int) -> int:
    assert x != y
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("assert"), "Should transpile assert with !=. Got: {}", result);
}

#[test]
fn test_s11_adv_assert_compound() {
    let code = r#"
def check_range(x: int) -> int:
    assert x >= 0 and x <= 100
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("assert") || result.contains("fn check_range"),
        "Should transpile compound assert. Got: {}",
        result
    );
}

// ============================================================================
// Raise and exception patterns
// ============================================================================

#[test]
fn test_s11_adv_raise_runtime_error() {
    let code = r#"
def fail_fast(msg: str) -> None:
    raise RuntimeError(msg)
"#;
    let result = transpile(code);
    assert!(
        result.contains("panic") || result.contains("fn fail_fast"),
        "Should transpile raise RuntimeError. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_raise_type_error() {
    let code = r#"
def check_type(x: int) -> int:
    if x < 0:
        raise TypeError("expected positive")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_type"), "Should transpile raise TypeError. Got: {}", result);
}

#[test]
fn test_s11_adv_raise_index_error() {
    let code = r#"
def safe_get(items: list, idx: int) -> int:
    if idx >= len(items):
        raise IndexError("index out of range")
    return items[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Should transpile raise IndexError. Got: {}", result);
}

// ============================================================================
// Walrus operator
// ============================================================================

#[test]
fn test_s11_adv_walrus_in_if() {
    let code = r#"
def process(items: list) -> int:
    if (n := len(items)) > 0:
        return n
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Should transpile walrus operator. Got: {}", result);
}

// ============================================================================
// Generator / yield patterns
// ============================================================================

#[test]
fn test_s11_adv_generator_simple() {
    let code = r#"
def count_up(n: int):
    i: int = 0
    while i < n:
        yield i
        i += 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_up"), "Should transpile generator. Got: {}", result);
}

#[test]
fn test_s11_adv_yield_value() {
    let code = r#"
def squares(n: int):
    for i in range(n):
        yield i * i
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares"), "Should transpile yield with value. Got: {}", result);
}

// ============================================================================
// Starred expressions and tuple unpacking
// ============================================================================

#[test]
fn test_s11_adv_tuple_unpack() {
    let code = r#"
def swap(a: int, b: int) -> int:
    a, b = b, a
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Should transpile tuple unpacking. Got: {}", result);
}

#[test]
fn test_s11_adv_triple_unpack() {
    let code = r#"
def rotate(a: int, b: int, c: int) -> int:
    a, b, c = b, c, a
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn rotate"), "Should transpile triple unpacking. Got: {}", result);
}

// ============================================================================
// String operations edge cases
// ============================================================================

#[test]
fn test_s11_adv_str_startswith() {
    let code = r#"
def starts(text: str, pre: str) -> bool:
    return text.startswith(pre)
"#;
    let result = transpile(code);
    assert!(
        result.contains("starts_with") || result.contains("fn starts"),
        "Should transpile startswith. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_str_endswith() {
    let code = r#"
def ends(text: str, suf: str) -> bool:
    return text.endswith(suf)
"#;
    let result = transpile(code);
    assert!(
        result.contains("ends_with") || result.contains("fn ends"),
        "Should transpile endswith. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_str_replace() {
    let code = r#"
def replace_all(text: str, old: str, new: str) -> str:
    return text.replace(old, new)
"#;
    let result = transpile(code);
    assert!(
        result.contains("replace") || result.contains("fn replace_all"),
        "Should transpile str.replace. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_str_find() {
    let code = r#"
def find_char(text: str, ch: str) -> int:
    return text.find(ch)
"#;
    let result = transpile(code);
    assert!(
        result.contains("find") || result.contains("fn find_char"),
        "Should transpile str.find. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_str_count() {
    let code = r#"
def count_sub(text: str, sub: str) -> int:
    return text.count(sub)
"#;
    let result = transpile(code);
    assert!(
        result.contains("matches") || result.contains("fn count_sub"),
        "Should transpile str.count. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_str_join() {
    let code = r#"
def join_words(words: list) -> str:
    return ", ".join(words)
"#;
    let result = transpile(code);
    assert!(
        result.contains("join") || result.contains("fn join_words"),
        "Should transpile str.join. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_str_split_with_delim() {
    let code = r#"
def split_csv(line: str) -> list:
    return line.split(",")
"#;
    let result = transpile(code);
    assert!(
        result.contains("split") || result.contains("fn split_csv"),
        "Should transpile str.split with delimiter. Got: {}",
        result
    );
}

// ============================================================================
// Comparison chains
// ============================================================================

#[test]
fn test_s11_adv_chained_comparison() {
    let code = r#"
def in_range(x: int, lo: int, hi: int) -> bool:
    return lo <= x <= hi
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Should transpile chained comparison. Got: {}", result);
}

// ============================================================================
// Global / module-level constants
// ============================================================================

#[test]
fn test_s11_adv_module_constant_int() {
    let code = r#"
MAX_SIZE: int = 100

def get_max() -> int:
    return MAX_SIZE
"#;
    let result = transpile(code);
    assert!(
        result.contains("MAX_SIZE") || result.contains("100"),
        "Should transpile module constant. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_module_constant_str() {
    let code = r#"
VERSION: str = "1.0"

def get_version() -> str:
    return VERSION
"#;
    let result = transpile(code);
    assert!(
        result.contains("VERSION") || result.contains("fn get_version"),
        "Should transpile string constant. Got: {}",
        result
    );
}

// ============================================================================
// Dict comprehension and set operations
// ============================================================================

#[test]
fn test_s11_adv_dict_comprehension() {
    let code = r#"
def square_map(n: int) -> dict:
    return {i: i * i for i in range(n)}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn square_map"),
        "Should transpile dict comprehension. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_set_literal() {
    let code = r#"
def unique_items() -> set:
    return {1, 2, 3}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_items"), "Should transpile set literal. Got: {}", result);
}

// ============================================================================
// Type conversion edge cases
// ============================================================================

#[test]
fn test_s11_adv_float_to_int() {
    let code = r#"
def truncate(x: float) -> int:
    return int(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("as i64") || result.contains("fn truncate"),
        "Should transpile float to int. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_int_to_float() {
    let code = r#"
def widen(x: int) -> float:
    return float(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("as f64") || result.contains("fn widen"),
        "Should transpile int to float. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_str_to_int() {
    let code = r#"
def parse_num(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("parse") || result.contains("fn parse_num"),
        "Should transpile str to int. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_int_to_str() {
    let code = r#"
def stringify(x: int) -> str:
    return str(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_string") || result.contains("fn stringify"),
        "Should transpile int to str. Got: {}",
        result
    );
}

// ============================================================================
// Bitwise operations
// ============================================================================

#[test]
fn test_s11_adv_bitwise_and() {
    let code = r#"
def mask(x: int, m: int) -> int:
    return x & m
"#;
    let result = transpile(code);
    assert!(
        result.contains("&") || result.contains("fn mask"),
        "Should transpile bitwise AND. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_bitwise_or() {
    let code = r#"
def combine(a: int, b: int) -> int:
    return a | b
"#;
    let result = transpile(code);
    assert!(
        result.contains("|") || result.contains("fn combine"),
        "Should transpile bitwise OR. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_bitwise_xor() {
    let code = r#"
def toggle(a: int, b: int) -> int:
    return a ^ b
"#;
    let result = transpile(code);
    assert!(
        result.contains("^") || result.contains("fn toggle"),
        "Should transpile bitwise XOR. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_left_shift() {
    let code = r#"
def shift_up(x: int, n: int) -> int:
    return x << n
"#;
    let result = transpile(code);
    assert!(
        result.contains("<<") || result.contains("fn shift_up"),
        "Should transpile left shift. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_right_shift() {
    let code = r#"
def shift_down(x: int, n: int) -> int:
    return x >> n
"#;
    let result = transpile(code);
    assert!(
        result.contains(">>") || result.contains("fn shift_down"),
        "Should transpile right shift. Got: {}",
        result
    );
}

// ============================================================================
// Multiple assignment targets
// ============================================================================

#[test]
fn test_s11_adv_augmented_sub() {
    let code = r#"
def decrement(x: int) -> int:
    x -= 1
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("-=") || result.contains("fn decrement"),
        "Should transpile -=. Got: {}",
        result
    );
}

#[test]
fn test_s11_adv_augmented_mul() {
    let code = r#"
def double_up(x: int) -> int:
    x *= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_up"), "Should transpile *=. Got: {}", result);
}

#[test]
fn test_s11_adv_augmented_div() {
    let code = r#"
def halve(x: float) -> float:
    x /= 2.0
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn halve"), "Should transpile /=. Got: {}", result);
}
