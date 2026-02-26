//! DEPYLER-99MODE-S10: Integration tests targeting stmt_gen_complex.rs coverage gaps
//!
//! Tests for try/except patterns, nested functions, variable hoisting, and
//! complex control flow through the transpilation pipeline.

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

// ===== Try/Except Patterns =====

#[test]
fn test_s10_try_except_basic() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_parse"));
    // Should handle the try/except pattern
    assert!(result.contains("parse") || result.contains("unwrap_or"));
}

#[test]
fn test_s10_try_except_with_binding() {
    let code = r#"
def handle_error(s: str) -> str:
    try:
        x = int(s)
        return str(x)
    except ValueError as e:
        return str(e)
"#;
    let result = transpile(code);
    assert!(result.contains("fn handle_error"));
    // Exception binding should be present
    assert!(result.contains("Err"));
}

#[test]
fn test_s10_try_except_finally() {
    let code = r#"
def with_cleanup(x: int) -> int:
    result = 0
    try:
        result = x * 2
    except Exception:
        result = -1
    finally:
        print(result)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn with_cleanup"));
    assert!(result.contains("println!") || result.contains("print"));
}

#[test]
fn test_s10_try_except_multiple_handlers() {
    let code = r#"
def multi_handler(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;
    let result = transpile(code);
    assert!(result.contains("fn multi_handler"));
    // Should handle multiple exception types
}

#[test]
fn test_s10_try_except_negative_literal() {
    let code = r#"
def parse_with_default(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_with_default"));
    // Should use unwrap_or(-1) for negative default
    assert!(result.contains("-1") || result.contains("- 1"));
}

#[test]
fn test_s10_try_except_string_default() {
    let code = r#"
def parse_name(s: str) -> str:
    try:
        return s.strip()
    except Exception:
        return "unknown"
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_name"));
    assert!(result.contains("unknown"));
}

#[test]
fn test_s10_try_except_variable_hoisting() {
    let code = r#"
def hoisted_vars(x: int) -> int:
    try:
        result = x * 2
    except Exception:
        result = 0
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn hoisted_vars"));
    assert!(result.contains("result"));
}

#[test]
fn test_s10_try_finally_no_except() {
    let code = r#"
def cleanup_only(x: int) -> int:
    result = 0
    try:
        result = x + 1
    finally:
        print("done")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn cleanup_only"));
    assert!(result.contains("done"));
}

// ===== Nested Function Patterns =====

#[test]
fn test_s10_nested_function_basic() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"));
    assert!(result.contains("inner"));
    // Should generate as closure
    assert!(result.contains("move |") || result.contains("let inner"));
}

#[test]
fn test_s10_nested_function_with_capture() {
    let code = r#"
def make_adder(n: int) -> int:
    def add(x: int) -> int:
        return x + n
    return add(5)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_adder"));
    assert!(result.contains("add"));
}

#[test]
fn test_s10_nested_function_recursive() {
    let code = r#"
def factorial_wrapper(n: int) -> int:
    def factorial(x: int) -> int:
        if x <= 1:
            return 1
        return x * factorial(x - 1)
    return factorial(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn factorial_wrapper"));
    // Recursive nested functions should be generated as fn, not closure
    assert!(result.contains("fn factorial"));
}

#[test]
fn test_s10_nested_function_string_param() {
    let code = r#"
def process(items: list) -> list:
    def transform(s: str) -> str:
        return s.upper()
    return [transform(x) for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"));
    assert!(result.contains("transform"));
}

#[test]
fn test_s10_nested_function_list_param() {
    let code = r#"
def sum_all(data: list) -> int:
    def helper(nums: list) -> int:
        total = 0
        for n in nums:
            total = total + n
        return total
    return helper(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_all"));
    assert!(result.contains("helper"));
}

#[test]
fn test_s10_nested_function_no_return_type() {
    let code = r#"
def run_tasks(items: list):
    def process(item):
        print(item)
    for item in items:
        process(item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn run_tasks"));
    assert!(result.contains("process"));
}

#[test]
fn test_s10_multiple_nested_functions() {
    let code = r#"
def pipeline(x: int) -> int:
    def double(n: int) -> int:
        return n * 2
    def add_one(n: int) -> int:
        return n + 1
    return add_one(double(x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn pipeline"));
    assert!(result.contains("double"));
    assert!(result.contains("add_one"));
}

// ===== Floor Division + ZeroDivisionError =====

#[test]
fn test_s10_try_floor_div_zero() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_div"));
    // Should generate zero-division check
    assert!(result.contains("== 0") || result.contains("ZeroDivision"));
}

// ===== Try/Except with Class/Exception =====

#[test]
fn test_s10_try_with_raise() {
    let code = r#"
def validate(x: int) -> int:
    try:
        if x < 0:
            raise ValueError("negative")
        return x
    except ValueError as e:
        print(e)
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"));
    assert!(result.contains("negative"));
}

// ===== While True with Break =====

#[test]
fn test_s10_while_true_break_pattern() {
    let code = r#"
def read_until_done(x: int) -> int:
    result = 0
    while True:
        if x <= 0:
            break
        result = result + x
        x = x - 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_until_done"));
    assert!(result.contains("loop") || result.contains("while"));
    assert!(result.contains("break"));
}

// ===== Complex Nested Try/Except =====

#[test]
fn test_s10_nested_try_except() {
    let code = r#"
def nested_error_handling(s: str) -> int:
    try:
        try:
            return int(s)
        except ValueError:
            return -1
    except Exception:
        return -2
"#;
    let result = transpile(code);
    assert!(result.contains("fn nested_error_handling"));
}

// ===== With Statement Patterns =====

#[test]
fn test_s10_with_statement_basic() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"));
}

#[test]
fn test_s10_with_statement_write() {
    let code = r#"
def write_file(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"));
    assert!(result.contains("write"));
}

// ===== Return Type Inference =====

#[test]
fn test_s10_return_type_bool() {
    let code = r#"
def is_valid(x: int) -> bool:
    try:
        return x > 0
    except Exception:
        return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid"));
    assert!(result.contains("bool"));
}

#[test]
fn test_s10_return_type_float() {
    let code = r#"
def safe_sqrt(x: float) -> float:
    try:
        return x ** 0.5
    except ValueError:
        return 0.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_sqrt"));
    assert!(result.contains("f64"));
}

// ===== For Loop Patterns =====

#[test]
fn test_s10_for_with_enumerate() {
    let code = r#"
def indexed_sum(items: list) -> int:
    total = 0
    for i, val in enumerate(items):
        total = total + i + val
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed_sum"));
    assert!(result.contains("enumerate"));
}

#[test]
fn test_s10_for_with_range_step() {
    let code = r#"
def skip_sum(n: int) -> int:
    total = 0
    for i in range(0, n, 2):
        total = total + i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_sum"));
    assert!(result.contains("step_by") || result.contains("range"));
}

// ===== Assert Statement =====

#[test]
fn test_s10_assert_basic() {
    let code = r#"
def checked_add(a: int, b: int) -> int:
    assert a >= 0
    assert b >= 0
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn checked_add"));
    assert!(result.contains("assert!") || result.contains("assert"));
}

#[test]
fn test_s10_assert_with_message() {
    let code = r#"
def positive_only(x: int) -> int:
    assert x > 0, "must be positive"
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_only"));
    assert!(result.contains("must be positive"));
}

// ===== Class/ADT Patterns =====

#[test]
fn test_s10_simple_class() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def magnitude(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("struct Point") || result.contains("Point"));
    assert!(result.contains("magnitude"));
}

#[test]
fn test_s10_class_with_method() {
    let code = r#"
class Counter:
    def __init__(self, start: int):
        self.count = start

    def increment(self) -> int:
        self.count = self.count + 1
        return self.count
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"));
    assert!(result.contains("increment"));
}

// ===== Complex Expression Patterns =====

#[test]
fn test_s10_walrus_like_assign_in_while() {
    let code = r#"
def accumulate(items: list) -> int:
    total = 0
    i = 0
    while i < len(items):
        total = total + items[i]
        i = i + 1
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn accumulate"));
    assert!(result.contains("while"));
}

#[test]
fn test_s10_multiple_return_paths() {
    let code = r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"));
    assert!(result.contains("positive"));
    assert!(result.contains("negative"));
    assert!(result.contains("zero"));
}

// ===== Dict/List Comprehension Inside Try =====

#[test]
fn test_s10_comprehension_in_function() {
    let code = r#"
def squares(n: int) -> list:
    return [i * i for i in range(n)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares"));
    assert!(result.contains("map") || result.contains("iter"));
}

#[test]
fn test_s10_dict_comprehension() {
    let code = r#"
def make_lookup(keys: list, values: list) -> dict:
    return {k: v for k, v in zip(keys, values)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_lookup"));
    assert!(result.contains("zip") || result.contains("HashMap"));
}

// ===== String Operations =====

#[test]
fn test_s10_string_format_f_string() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"));
    assert!(result.contains("format!") || result.contains("Hello"));
}

#[test]
fn test_s10_string_join() {
    let code = r#"
def join_words(words: list) -> str:
    return ", ".join(words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_words"));
    assert!(result.contains("join"));
}

// ===== Global/Module Level =====

#[test]
fn test_s10_module_level_constant() {
    let code = r#"
MAX_SIZE = 100

def check_size(n: int) -> bool:
    return n <= MAX_SIZE
"#;
    let result = transpile(code);
    assert!(result.contains("MAX_SIZE"));
    assert!(result.contains("100"));
}

// ===== Tuple Return =====

#[test]
fn test_s10_tuple_return() {
    let code = r#"
def divmod_func(a: int, b: int) -> tuple:
    return (a // b, a % b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn divmod_func"));
}

// ===== Default Parameters =====

#[test]
fn test_s10_default_parameter() {
    let code = r#"
def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"));
}

// ===== Exception Patterns =====

#[test]
fn test_s10_try_with_bool_return() {
    let code = r#"
def try_parse_bool(s: str) -> bool:
    try:
        return int(s) > 0
    except ValueError:
        return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn try_parse_bool"));
    assert!(result.contains("bool"));
}

#[test]
fn test_s10_try_except_float_default() {
    let code = r#"
def safe_float(s: str) -> float:
    try:
        return float(s)
    except ValueError:
        return 0.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_float"));
}

// ===== Continue in Loop =====

#[test]
fn test_s10_continue_in_for() {
    let code = r#"
def sum_positives(items: list) -> int:
    total = 0
    for x in items:
        if x <= 0:
            continue
        total = total + x
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_positives"));
    assert!(result.contains("continue"));
}

// ===== Pass Statement =====

#[test]
fn test_s10_pass_in_except() {
    let code = r#"
def ignore_errors(x: int) -> int:
    try:
        return x * 2
    except Exception:
        pass
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn ignore_errors"));
}

// ===== Lambda =====

#[test]
fn test_s10_lambda_in_sort() {
    let code = r#"
def sort_by_second(items: list) -> list:
    items.sort(key=lambda x: x[1])
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_second"));
    assert!(result.contains("sort") || result.contains("sort_by"));
}

// ===== Type Annotations =====

#[test]
fn test_s10_optional_return_type() {
    let code = r#"
from typing import Optional

def find_item(items: list, target: int) -> Optional[int]:
    for item in items:
        if item == target:
            return item
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_item"));
    assert!(result.contains("Option") || result.contains("None"));
}

// ===== Chained Method Calls =====

#[test]
fn test_s10_chained_string_methods() {
    let code = r#"
def normalize(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"));
    assert!(result.contains("trim") || result.contains("strip"));
    assert!(result.contains("to_lowercase") || result.contains("lower"));
    assert!(result.contains("replace"));
}

// ===== Global Variable Reference =====

#[test]
fn test_s10_global_constant_reference() {
    let code = r#"
PI = 3.14159

def circle_area(r: float) -> float:
    return PI * r * r
"#;
    let result = transpile(code);
    assert!(result.contains("PI"));
    assert!(result.contains("fn circle_area"));
}
