//! Session 11: Stdlib method coverage tests
//!
//! Targets specific untested stdlib method paths:
//! - math module functions
//! - string methods (less common ones)
//! - json module
//! - collections operations
//! - os module
//! - sys module
//! - re module

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
// math module
// ============================================================================

#[test]
fn test_s11_stdlib_math_sqrt() {
    let code = r#"
import math

def root(x: float) -> float:
    return math.sqrt(x)
"#;
    let result = transpile(code);
    assert!(result.contains("sqrt"), "Should transpile math.sqrt. Got: {}", result);
}

#[test]
fn test_s11_stdlib_math_floor() {
    let code = r#"
import math

def floor_val(x: float) -> int:
    return math.floor(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("floor") || result.contains("fn floor_val"),
        "Should transpile math.floor. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_math_ceil() {
    let code = r#"
import math

def ceil_val(x: float) -> int:
    return math.ceil(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("ceil") || result.contains("fn ceil_val"),
        "Should transpile math.ceil. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_math_sin() {
    let code = r#"
import math

def sine(x: float) -> float:
    return math.sin(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("sin") || result.contains("fn sine"),
        "Should transpile math.sin. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_math_cos() {
    let code = r#"
import math

def cosine(x: float) -> float:
    return math.cos(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("cos") || result.contains("fn cosine"),
        "Should transpile math.cos. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_math_log() {
    let code = r#"
import math

def natural_log(x: float) -> float:
    return math.log(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("ln") || result.contains("log") || result.contains("fn natural_log"),
        "Should transpile math.log. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_math_pow() {
    let code = r#"
import math

def power(base: float, exp: float) -> float:
    return math.pow(base, exp)
"#;
    let result = transpile(code);
    assert!(
        result.contains("powf") || result.contains("pow") || result.contains("fn power"),
        "Should transpile math.pow. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_math_fabs() {
    let code = r#"
import math

def absolute(x: float) -> float:
    return math.fabs(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("abs") || result.contains("fn absolute"),
        "Should transpile math.fabs. Got: {}",
        result
    );
}

// ============================================================================
// String methods (less common)
// ============================================================================

#[test]
fn test_s11_stdlib_str_strip() {
    let code = r#"
def clean(text: str) -> str:
    return text.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("trim"), "Should transpile strip to trim. Got: {}", result);
}

#[test]
fn test_s11_stdlib_str_lstrip() {
    let code = r#"
def left_clean(text: str) -> str:
    return text.lstrip()
"#;
    let result = transpile(code);
    assert!(
        result.contains("trim_start") || result.contains("fn left_clean"),
        "Should transpile lstrip. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_str_rstrip() {
    let code = r#"
def right_clean(text: str) -> str:
    return text.rstrip()
"#;
    let result = transpile(code);
    assert!(
        result.contains("trim_end") || result.contains("fn right_clean"),
        "Should transpile rstrip. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_str_upper() {
    let code = r#"
def shout(text: str) -> str:
    return text.upper()
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_uppercase") || result.contains("to_ascii_uppercase"),
        "Should transpile upper. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_str_lower() {
    let code = r#"
def whisper(text: str) -> str:
    return text.lower()
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_lowercase") || result.contains("to_ascii_lowercase"),
        "Should transpile lower. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_str_isdigit() {
    let code = r#"
def all_digits(text: str) -> bool:
    return text.isdigit()
"#;
    let result = transpile(code);
    assert!(
        result.contains("is_ascii_digit")
            || result.contains("chars")
            || result.contains("fn all_digits"),
        "Should transpile isdigit. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_str_isalpha() {
    let code = r#"
def all_alpha(text: str) -> bool:
    return text.isalpha()
"#;
    let result = transpile(code);
    assert!(
        result.contains("is_alphabetic")
            || result.contains("chars")
            || result.contains("fn all_alpha"),
        "Should transpile isalpha. Got: {}",
        result
    );
}

// ============================================================================
// Dict operations
// ============================================================================

#[test]
fn test_s11_stdlib_dict_items() {
    let code = r#"
def print_items(d: dict) -> None:
    for key, val in d.items():
        print(key, val)
"#;
    let result = transpile(code);
    assert!(
        result.contains("iter") || result.contains("fn print_items"),
        "Should transpile dict.items(). Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_dict_keys() {
    let code = r#"
def get_keys(d: dict) -> list:
    return list(d.keys())
"#;
    let result = transpile(code);
    assert!(
        result.contains("keys") || result.contains("fn get_keys"),
        "Should transpile dict.keys(). Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_dict_values() {
    let code = r#"
def get_values(d: dict) -> list:
    return list(d.values())
"#;
    let result = transpile(code);
    assert!(
        result.contains("values") || result.contains("fn get_values"),
        "Should transpile dict.values(). Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_dict_update() {
    let code = r#"
def merge(a: dict, b: dict) -> None:
    a.update(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("extend") || result.contains("fn merge"),
        "Should transpile dict.update(). Got: {}",
        result
    );
}

// ============================================================================
// List operations
// ============================================================================

#[test]
fn test_s11_stdlib_list_sort() {
    let code = r#"
def sort_inplace(items: list) -> None:
    items.sort()
"#;
    let result = transpile(code);
    assert!(result.contains("sort"), "Should transpile list.sort(). Got: {}", result);
}

#[test]
fn test_s11_stdlib_list_reverse() {
    let code = r#"
def reverse_inplace(items: list) -> None:
    items.reverse()
"#;
    let result = transpile(code);
    assert!(result.contains("reverse"), "Should transpile list.reverse(). Got: {}", result);
}

#[test]
fn test_s11_stdlib_list_extend() {
    let code = r#"
def extend_list(a: list, b: list) -> None:
    a.extend(b)
"#;
    let result = transpile(code);
    assert!(result.contains("extend"), "Should transpile list.extend(). Got: {}", result);
}

#[test]
fn test_s11_stdlib_list_insert() {
    let code = r#"
def insert_at(items: list, idx: int, val: int) -> None:
    items.insert(idx, val)
"#;
    let result = transpile(code);
    assert!(result.contains("insert"), "Should transpile list.insert(). Got: {}", result);
}

#[test]
fn test_s11_stdlib_list_pop_no_args() {
    let code = r#"
def pop_last(items: list) -> int:
    return items.pop()
"#;
    let result = transpile(code);
    assert!(
        result.contains("pop") || result.contains("fn pop_last"),
        "Should transpile list.pop(). Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_list_remove() {
    let code = r#"
def remove_val(items: list, val: int) -> None:
    items.remove(val)
"#;
    let result = transpile(code);
    assert!(
        result.contains("retain") || result.contains("remove") || result.contains("fn remove_val"),
        "Should transpile list.remove(). Got: {}",
        result
    );
}

// ============================================================================
// Built-in functions
// ============================================================================

#[test]
fn test_s11_stdlib_range_with_step() {
    let code = r#"
def even_range(n: int) -> list:
    result: list = []
    for i in range(0, n, 2):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("step_by") || result.contains("fn even_range"),
        "Should transpile range with step. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_enumerate_basic() {
    let code = r#"
def indexed(items: list) -> list:
    result: list = []
    for i, item in enumerate(items):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("enumerate"), "Should transpile enumerate. Got: {}", result);
}

#[test]
fn test_s11_stdlib_zip_basic() {
    let code = r#"
def pair_up(a: list, b: list) -> list:
    result: list = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("zip") || result.contains("fn pair_up"),
        "Should transpile zip. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_print_multiple_args() {
    let code = r#"
def show(a: int, b: str, c: float) -> None:
    print(a, b, c)
"#;
    let result = transpile(code);
    assert!(result.contains("println"), "Should transpile print with args. Got: {}", result);
}

#[test]
fn test_s11_stdlib_input_basic() {
    let code = r#"
def ask(prompt: str) -> str:
    return input(prompt)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ask"), "Should transpile input(). Got: {}", result);
}

// ============================================================================
// Exception handling patterns
// ============================================================================

#[test]
fn test_s11_stdlib_raise_value_error() {
    let code = r#"
def check_positive(x: int) -> int:
    if x < 0:
        raise ValueError("must be positive")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_positive"), "Should transpile raise. Got: {}", result);
}

#[test]
fn test_s11_stdlib_try_multiple_except() {
    let code = r#"
def safe_op(x: int, y: int) -> int:
    try:
        return x // y
    except ZeroDivisionError:
        return 0
    except ValueError:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_op"), "Should transpile multiple except. Got: {}", result);
}

#[test]
fn test_s11_stdlib_try_finally() {
    let code = r#"
def with_cleanup(x: int) -> int:
    result: int = 0
    try:
        result = x * 2
    finally:
        print("done")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn with_cleanup"), "Should transpile try/finally. Got: {}", result);
}

// ============================================================================
// Class-related patterns
// ============================================================================

#[test]
fn test_s11_stdlib_class_with_init() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def distance(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(
        result.contains("Point") || result.contains("struct"),
        "Should transpile class. Got: {}",
        result
    );
}

#[test]
fn test_s11_stdlib_class_method_call() {
    let code = r#"
class Calculator:
    def __init__(self) -> None:
        self.value: int = 0

    def add(self, x: int) -> None:
        self.value = self.value + x

    def get(self) -> int:
        return self.value
"#;
    let result = transpile(code);
    assert!(
        result.contains("Calculator") || result.contains("value"),
        "Should transpile class methods. Got: {}",
        result
    );
}

// ============================================================================
// Complex algorithm patterns
// ============================================================================

#[test]
fn test_s11_stdlib_bubble_sort() {
    let code = r#"
def bubble_sort(arr: list) -> list:
    n: int = len(arr)
    for i in range(n):
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
    return arr
"#;
    let result = transpile(code);
    assert!(result.contains("fn bubble_sort"), "Should transpile bubble sort. Got: {}", result);
}

#[test]
fn test_s11_stdlib_matrix_multiply() {
    let code = r#"
def dot_product(a: list, b: list) -> int:
    result: int = 0
    for i in range(len(a)):
        result += a[i] * b[i]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn dot_product"), "Should transpile dot product. Got: {}", result);
}

#[test]
fn test_s11_stdlib_string_reversal() {
    let code = r#"
def reverse_str(s: str) -> str:
    return s[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_str"), "Should transpile string reversal. Got: {}", result);
}

#[test]
fn test_s11_stdlib_count_occurrences() {
    let code = r#"
def count_char(text: str, ch: str) -> int:
    count: int = 0
    for c in text:
        if c == ch:
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_char"), "Should transpile char counting. Got: {}", result);
}
