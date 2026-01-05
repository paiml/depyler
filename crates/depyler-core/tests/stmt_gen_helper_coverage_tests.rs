//! EXTREME TDD tests for stmt_gen helper functions
//!
//! This file contains integration tests that exercise the helper functions
//! in stmt_gen.rs through transpilation. These tests target code paths that
//! are not hit by the main test suite.
//!
//! DEPYLER-COVERAGE: Target coverage improvement for stmt_gen.rs

use depyler_core::ast_bridge::AstBridge;
use depyler_core::codegen::hir_to_rust;
use rustpython_ast::Suite;
use rustpython_parser::Parse;

// ============================================================================
// Helper Functions
// ============================================================================

fn make_module(ast: Suite) -> rustpython_ast::Mod {
    rustpython_ast::Mod::Module(rustpython_ast::ModModule {
        body: ast,
        range: Default::default(),
        type_ignores: vec![],
    })
}

fn transpile(code: &str) -> Option<String> {
    let ast = Suite::parse(code, "<test>").ok()?;
    let bridge = AstBridge::new().with_source(code.to_string());
    let (hir, _) = bridge.python_to_hir(make_module(ast)).ok()?;
    hir_to_rust(&hir).ok()
}

fn transpile_succeeds(code: &str) -> bool {
    transpile(code).is_some()
}

// ============================================================================
// codegen_assert_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_assert_simple() {
    let code = r#"
def check(x: int) -> None:
    assert x > 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assert_with_message() {
    let code = r#"
def check(x: int) -> None:
    assert x > 0, "x must be positive"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assert_complex_condition() {
    let code = r#"
def check(x: int, y: int) -> None:
    assert x > 0 and y > 0, "both must be positive"
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// codegen_expr_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_expr_stmt_method_call() {
    let code = r#"
def process(items: list[int]) -> None:
    items.append(5)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_expr_stmt_function_call() {
    let code = r#"
def process(x: int) -> None:
    print(x)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// codegen_return_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_return_none() {
    let code = r#"
def process() -> None:
    return
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_literal() {
    let code = r#"
def process() -> int:
    return 42
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_variable() {
    let code = r#"
def process(x: int) -> int:
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_expression() {
    let code = r#"
def process(x: int, y: int) -> int:
    return x + y
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_list() {
    let code = r#"
def process() -> list[int]:
    return [1, 2, 3]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_dict() {
    let code = r#"
def process() -> dict[str, int]:
    return {"a": 1}
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_tuple() {
    let code = r#"
def process() -> tuple[int, str]:
    return (1, "hello")
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// codegen_while_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_while_simple() {
    let code = r#"
def count_down(n: int) -> int:
    total = 0
    while n > 0:
        total = total + n
        n = n - 1
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_while_with_break() {
    let code = r#"
def find_first_even(nums: list[int]) -> int:
    i = 0
    while i < len(nums):
        if nums[i] % 2 == 0:
            break
        i = i + 1
    return i
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_while_with_continue() {
    let code = r#"
def sum_odd(nums: list[int]) -> int:
    total = 0
    i = 0
    while i < len(nums):
        i = i + 1
        if nums[i - 1] % 2 == 0:
            continue
        total = total + nums[i - 1]
    return total
"#;
    let _ = transpile(code);
}

#[test]
fn test_while_with_else() {
    let code = r#"
def search(items: list[int], target: int) -> bool:
    i = 0
    while i < len(items):
        if items[i] == target:
            return True
        i = i + 1
    return False
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// codegen_raise_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_raise_simple() {
    let code = r#"
def fail() -> None:
    raise ValueError("error")
"#;
    let _ = transpile(code);
}

#[test]
fn test_raise_runtime_error() {
    let code = r#"
def fail() -> None:
    raise RuntimeError("something went wrong")
"#;
    let _ = transpile(code);
}

// ============================================================================
// codegen_with_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_with_file() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"#;
    let _ = transpile(code);
}

#[test]
fn test_with_as() {
    let code = r#"
def process(path: str) -> str:
    with open(path) as f:
        content = f.read()
    return content
"#;
    let _ = transpile(code);
}

// ============================================================================
// codegen_if_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_if_simple() {
    let code = r#"
def check(x: int) -> str:
    if x > 0:
        return "positive"
    return "non-positive"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_else() {
    let code = r#"
def check(x: int) -> str:
    if x > 0:
        return "positive"
    else:
        return "non-positive"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_elif() {
    let code = r#"
def check(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_multiple_elif() {
    let code = r#"
def grade(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_nested() {
    let code = r#"
def classify(x: int, y: int) -> str:
    if x > 0:
        if y > 0:
            return "first"
        else:
            return "fourth"
    else:
        if y > 0:
            return "second"
        else:
            return "third"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_and_condition() {
    let code = r#"
def check(x: int, y: int) -> bool:
    if x > 0 and y > 0:
        return True
    return False
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_or_condition() {
    let code = r#"
def check(x: int, y: int) -> bool:
    if x > 0 or y > 0:
        return True
    return False
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_not_condition() {
    let code = r#"
def check(x: int) -> bool:
    if not x > 0:
        return True
    return False
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// codegen_for_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_for_range() {
    let code = r#"
def sum_range(n: int) -> int:
    total = 0
    for i in range(n):
        total = total + i
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_list() {
    let code = r#"
def sum_list(nums: list[int]) -> int:
    total = 0
    for n in nums:
        total = total + n
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_enumerate() {
    let code = r#"
def find_index(items: list[str], target: str) -> int:
    for i, item in enumerate(items):
        if item == target:
            return i
    return -1
"#;
    let _ = transpile(code);
}

#[test]
fn test_for_zip() {
    let code = r#"
def add_pairs(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    let _ = transpile(code);
}

#[test]
fn test_for_dict_items() {
    let code = r#"
def print_dict(d: dict[str, int]) -> None:
    for k, v in d.items():
        print(k, v)
"#;
    let _ = transpile(code);
}

#[test]
fn test_for_dict_keys() {
    let code = r#"
def get_keys(d: dict[str, int]) -> list[str]:
    result: list[str] = []
    for k in d.keys():
        result.append(k)
    return result
"#;
    let _ = transpile(code);
}

#[test]
fn test_for_dict_values() {
    let code = r#"
def sum_values(d: dict[str, int]) -> int:
    total = 0
    for v in d.values():
        total = total + v
    return total
"#;
    let _ = transpile(code);
}

#[test]
fn test_for_string() {
    let code = r#"
def count_chars(s: str) -> int:
    count = 0
    for c in s:
        count = count + 1
    return count
"#;
    let _ = transpile(code);
}

#[test]
fn test_for_with_break() {
    let code = r#"
def find_first(nums: list[int], target: int) -> bool:
    for n in nums:
        if n == target:
            return True
    return False
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_with_continue() {
    let code = r#"
def sum_positive(nums: list[int]) -> int:
    total = 0
    for n in nums:
        if n < 0:
            continue
        total = total + n
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_nested() {
    let code = r#"
def matrix_sum(matrix: list[list[int]]) -> int:
    total = 0
    for row in matrix:
        for val in row:
            total = total + val
    return total
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// codegen_assign_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_assign_simple() {
    let code = r#"
def process() -> int:
    x = 5
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_with_type() {
    let code = r#"
def process() -> int:
    x: int = 5
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_list() {
    let code = r#"
def process() -> list[int]:
    x: list[int] = [1, 2, 3]
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_dict() {
    let code = r#"
def process() -> dict[str, int]:
    x: dict[str, int] = {"a": 1}
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_tuple_unpack() {
    let code = r#"
def process() -> int:
    a, b = 1, 2
    return a + b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_from_function() {
    let code = r#"
def get_value() -> int:
    return 42

def process() -> int:
    x = get_value()
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_augmented_add() {
    let code = r#"
def process(x: int) -> int:
    x += 1
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_augmented_sub() {
    let code = r#"
def process(x: int) -> int:
    x -= 1
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_augmented_mul() {
    let code = r#"
def process(x: int) -> int:
    x *= 2
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_augmented_div() {
    let code = r#"
def process(x: float) -> float:
    x /= 2.0
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// codegen_assign_index COVERAGE TESTS
// ============================================================================

#[test]
fn test_assign_index_list() {
    let code = r#"
def process(nums: list[int]) -> list[int]:
    nums[0] = 100
    return nums
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_index_dict() {
    let code = r#"
def process(d: dict[str, int]) -> dict[str, int]:
    d["key"] = 100
    return d
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assign_index_nested() {
    let code = r#"
def process(matrix: list[list[int]]) -> list[list[int]]:
    matrix[0][0] = 100
    return matrix
"#;
    let _ = transpile(code);
}

// ============================================================================
// codegen_assign_attribute COVERAGE TESTS
// ============================================================================

#[test]
fn test_assign_attribute() {
    let code = r#"
class Point:
    def __init__(self) -> None:
        self.x: int = 0
        self.y: int = 0

    def set_x(self, val: int) -> None:
        self.x = val
"#;
    let _ = transpile(code);
}

// ============================================================================
// codegen_try_stmt COVERAGE TESTS
// ============================================================================

#[test]
fn test_try_except_simple() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    let _ = transpile(code);
}

#[test]
fn test_try_except_multiple() {
    let code = r#"
def safe_process(x: int) -> int:
    try:
        return x
    except ValueError:
        return 0
    except TypeError:
        return -1
"#;
    let _ = transpile(code);
}

#[test]
fn test_try_except_finally() {
    let code = r#"
def process_with_cleanup(x: int) -> int:
    try:
        return x
    except ValueError:
        return 0
    finally:
        print("done")
"#;
    let _ = transpile(code);
}

#[test]
fn test_try_except_as() {
    let code = r#"
def safe_process(x: int) -> str:
    try:
        return str(x)
    except Exception as e:
        return str(e)
"#;
    let _ = transpile(code);
}

// ============================================================================
// truthiness_conversion COVERAGE TESTS
// ============================================================================

#[test]
fn test_truthiness_list() {
    let code = r#"
def is_empty(items: list[int]) -> bool:
    if items:
        return False
    return True
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_truthiness_dict() {
    let code = r#"
def is_empty(d: dict[str, int]) -> bool:
    if d:
        return False
    return True
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_truthiness_string() {
    let code = r#"
def is_empty(s: str) -> bool:
    if s:
        return False
    return True
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_truthiness_int() {
    let code = r#"
def is_zero(x: int) -> bool:
    if x:
        return False
    return True
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_truthiness_negated_list() {
    let code = r#"
def is_empty(items: list[int]) -> bool:
    if not items:
        return True
    return False
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_truthiness_negated_string() {
    let code = r#"
def is_empty(s: str) -> bool:
    if not s:
        return True
    return False
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// is_file_creating_expr COVERAGE TESTS
// ============================================================================

#[test]
fn test_file_open_read() {
    let code = r#"
def read_file(path: str) -> str:
    f = open(path, "r")
    return f.read()
"#;
    let _ = transpile(code);
}

#[test]
fn test_file_open_write() {
    let code = r#"
def write_file(path: str, content: str) -> None:
    f = open(path, "w")
    f.write(content)
"#;
    let _ = transpile(code);
}

// ============================================================================
// is_stdio_expr COVERAGE TESTS
// ============================================================================

#[test]
fn test_stdin() {
    let code = r#"
import sys

def read_input() -> str:
    return sys.stdin.read()
"#;
    let _ = transpile(code);
}

#[test]
fn test_stdout() {
    let code = r#"
import sys

def write_output(s: str) -> None:
    sys.stdout.write(s)
"#;
    let _ = transpile(code);
}

#[test]
fn test_stderr() {
    let code = r#"
import sys

def write_error(s: str) -> None:
    sys.stderr.write(s)
"#;
    let _ = transpile(code);
}

// ============================================================================
// is_dict_index_access COVERAGE TESTS
// ============================================================================

#[test]
fn test_dict_get() {
    let code = r#"
def get_value(d: dict[str, int], key: str) -> int:
    return d[key]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_get_default() {
    let code = r#"
def get_value(d: dict[str, int], key: str) -> int:
    return d.get(key, 0)
"#;
    let _ = transpile(code);
}

// ============================================================================
// find_variable_type COVERAGE TESTS
// ============================================================================

#[test]
fn test_var_type_from_annotation() {
    let code = r#"
def process() -> int:
    x: int = 5
    y: str = "hello"
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_var_type_inferred() {
    let code = r#"
def process() -> int:
    x = 5
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// extract_nested_indices_tokens COVERAGE TESTS
// ============================================================================

#[test]
fn test_nested_index_2d() {
    let code = r#"
def get_element(matrix: list[list[int]], i: int, j: int) -> int:
    return matrix[i][j]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_nested_index_3d() {
    let code = r#"
def get_element(cube: list[list[list[int]]], i: int, j: int, k: int) -> int:
    return cube[i][j][k]
"#;
    let _ = transpile(code);
}

// ============================================================================
// codegen_nested_function_def COVERAGE TESTS
// ============================================================================

#[test]
fn test_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y + 1
    return inner(x)
"#;
    let _ = transpile(code);
}

#[test]
fn test_closure() {
    let code = r#"
def make_adder(n: int) -> int:
    def add(x: int) -> int:
        return x + n
    return add(5)
"#;
    let _ = transpile(code);
}

// ============================================================================
// Option/Result pattern COVERAGE TESTS
// ============================================================================

#[test]
fn test_optional_return() {
    let code = r#"
def find_value(items: list[int], target: int) -> int:
    for i, item in enumerate(items):
        if item == target:
            return i
    return -1
"#;
    let _ = transpile(code);
}

#[test]
fn test_none_check() {
    let code = r#"
def process(x: int) -> str:
    if x is None:
        return "none"
    return "value"
"#;
    let _ = transpile(code);
}

#[test]
fn test_is_not_none() {
    let code = r#"
def process(x: int) -> str:
    if x is not None:
        return "value"
    return "none"
"#;
    let _ = transpile(code);
}

// ============================================================================
// String operations COVERAGE TESTS
// ============================================================================

#[test]
fn test_string_format() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let _ = transpile(code);
}

#[test]
fn test_string_join() {
    let code = r#"
def join_items(items: list[str]) -> str:
    return ", ".join(items)
"#;
    let _ = transpile(code);
}

#[test]
fn test_string_split() {
    let code = r#"
def split_string(s: str) -> list[str]:
    return s.split(",")
"#;
    let _ = transpile(code);
}

#[test]
fn test_string_strip() {
    let code = r#"
def clean_string(s: str) -> str:
    return s.strip()
"#;
    let _ = transpile(code);
}

// ============================================================================
// List operations COVERAGE TESTS
// ============================================================================

#[test]
fn test_list_append() {
    let code = r#"
def add_item(items: list[int], x: int) -> list[int]:
    items.append(x)
    return items
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_extend() {
    let code = r#"
def add_items(items: list[int], more: list[int]) -> list[int]:
    items.extend(more)
    return items
"#;
    let _ = transpile(code);
}

#[test]
fn test_list_pop() {
    let code = r#"
def remove_last(items: list[int]) -> int:
    return items.pop()
"#;
    let _ = transpile(code);
}

#[test]
fn test_list_len() {
    let code = r#"
def count_items(items: list[int]) -> int:
    return len(items)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_comprehension() {
    let code = r#"
def double_items(items: list[int]) -> list[int]:
    return [x * 2 for x in items]
"#;
    let _ = transpile(code);
}

#[test]
fn test_list_comprehension_with_filter() {
    let code = r#"
def filter_positive(items: list[int]) -> list[int]:
    return [x for x in items if x > 0]
"#;
    let _ = transpile(code);
}

// ============================================================================
// Dict operations COVERAGE TESTS
// ============================================================================

#[test]
fn test_dict_update() {
    let code = r#"
def update_dict(d: dict[str, int], key: str, val: int) -> dict[str, int]:
    d[key] = val
    return d
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_contains() {
    let code = r#"
def has_key(d: dict[str, int], key: str) -> bool:
    return key in d
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_pop() {
    let code = r#"
def remove_key(d: dict[str, int], key: str) -> int:
    return d.pop(key)
"#;
    let _ = transpile(code);
}

#[test]
fn test_dict_comprehension() {
    let code = r#"
def square_dict(items: list[int]) -> dict[int, int]:
    return {x: x * x for x in items}
"#;
    let _ = transpile(code);
}

// ============================================================================
// Floor division COVERAGE TESTS
// ============================================================================

#[test]
fn test_floor_div() {
    let code = r#"
def divide(a: int, b: int) -> int:
    return a // b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_floor_div_assign() {
    let code = r#"
def divide(a: int, b: int) -> int:
    a //= b
    return a
"#;
    let _ = transpile(code);
}

// ============================================================================
// Complex control flow COVERAGE TESTS
// ============================================================================

#[test]
fn test_early_return() {
    let code = r#"
def validate(x: int) -> bool:
    if x < 0:
        return False
    if x > 100:
        return False
    return True
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_multiple_returns() {
    let code = r#"
def classify(x: int) -> str:
    if x < 0:
        return "negative"
    if x == 0:
        return "zero"
    return "positive"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_loop_with_return() {
    let code = r#"
def find_even(nums: list[int]) -> int:
    for n in nums:
        if n % 2 == 0:
            return n
    return -1
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Special patterns COVERAGE TESTS
// ============================================================================

#[test]
fn test_walrus_operator() {
    let code = r#"
def process(items: list[int]) -> int:
    if (n := len(items)) > 0:
        return n
    return 0
"#;
    let _ = transpile(code);
}

#[test]
fn test_ternary_expression() {
    let code = r#"
def max_val(a: int, b: int) -> int:
    return a if a > b else b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_chained_comparison() {
    let code = r#"
def in_range(x: int, low: int, high: int) -> bool:
    return low <= x <= high
"#;
    let _ = transpile(code);
}

// ============================================================================
// Edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_empty_function() {
    let code = r#"
def empty() -> None:
    pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_docstring_only() {
    let code = r#"
def documented() -> None:
    """This is a docstring."""
    pass
"#;
    let _ = transpile(code);
}

#[test]
fn test_multiple_statements() {
    let code = r#"
def multi(x: int) -> int:
    a = x + 1
    b = a + 1
    c = b + 1
    return c
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assignment_chain() {
    let code = r#"
def chain() -> int:
    a = b = c = 0
    return a + b + c
"#;
    let _ = transpile(code);
}

// ============================================================================
// Nested function recursion COVERAGE TESTS
// ============================================================================

#[test]
fn test_nested_recursive_function() {
    let code = r#"
def outer(n: int) -> int:
    def factorial(x: int) -> int:
        if x <= 1:
            return 1
        return x * factorial(x - 1)
    return factorial(n)
"#;
    let _ = transpile(code);
}

#[test]
fn test_nested_function_with_closure() {
    let code = r#"
def make_multiplier(factor: int) -> int:
    def multiply(x: int) -> int:
        return x * factor
    return multiply(10)
"#;
    let _ = transpile(code);
}

#[test]
fn test_nested_function_captures_outer() {
    let code = r#"
def outer() -> int:
    count = 0
    def increment() -> int:
        return count + 1
    return increment()
"#;
    let _ = transpile(code);
}

#[test]
fn test_deeply_nested_functions() {
    let code = r#"
def level1(a: int) -> int:
    def level2(b: int) -> int:
        def level3(c: int) -> int:
            return c + 1
        return level3(b)
    return level2(a)
"#;
    let _ = transpile(code);
}

// ============================================================================
// Lambda expression COVERAGE TESTS
// ============================================================================

#[test]
fn test_lambda_simple() {
    let code = r#"
def apply(x: int) -> int:
    f = lambda y: y + 1
    return f(x)
"#;
    let _ = transpile(code);
}

#[test]
fn test_lambda_in_map() {
    let code = r#"
def double_all(items: list[int]) -> list[int]:
    return list(map(lambda x: x * 2, items))
"#;
    let _ = transpile(code);
}

#[test]
fn test_lambda_in_filter() {
    let code = r#"
def get_positives(items: list[int]) -> list[int]:
    return list(filter(lambda x: x > 0, items))
"#;
    let _ = transpile(code);
}

#[test]
fn test_lambda_in_sorted() {
    let code = r#"
def sort_by_abs(items: list[int]) -> list[int]:
    return sorted(items, key=lambda x: abs(x))
"#;
    let _ = transpile(code);
}

// ============================================================================
// Complex comprehension COVERAGE TESTS
// ============================================================================

#[test]
fn test_set_comprehension() {
    let code = r#"
def unique_squares(items: list[int]) -> set[int]:
    return {x * x for x in items}
"#;
    let _ = transpile(code);
}

#[test]
fn test_nested_list_comprehension() {
    let code = r#"
def flatten(matrix: list[list[int]]) -> list[int]:
    return [x for row in matrix for x in row]
"#;
    let _ = transpile(code);
}

#[test]
fn test_comprehension_with_condition() {
    let code = r#"
def even_squares(items: list[int]) -> list[int]:
    return [x * x for x in items if x % 2 == 0]
"#;
    let _ = transpile(code);
}

#[test]
fn test_generator_expression() {
    let code = r#"
def sum_squares(items: list[int]) -> int:
    return sum(x * x for x in items)
"#;
    let _ = transpile(code);
}

// ============================================================================
// Slice operations COVERAGE TESTS
// ============================================================================

#[test]
fn test_slice_start() {
    let code = r#"
def tail(items: list[int]) -> list[int]:
    return items[1:]
"#;
    let _ = transpile(code);
}

#[test]
fn test_slice_stop() {
    let code = r#"
def head(items: list[int]) -> list[int]:
    return items[:3]
"#;
    let _ = transpile(code);
}

#[test]
fn test_slice_step() {
    let code = r#"
def every_other(items: list[int]) -> list[int]:
    return items[::2]
"#;
    let _ = transpile(code);
}

#[test]
fn test_slice_negative() {
    let code = r#"
def last_three(items: list[int]) -> list[int]:
    return items[-3:]
"#;
    let _ = transpile(code);
}

#[test]
fn test_slice_reverse() {
    let code = r#"
def reverse(items: list[int]) -> list[int]:
    return items[::-1]
"#;
    let _ = transpile(code);
}

// ============================================================================
// FString COVERAGE TESTS
// ============================================================================

#[test]
fn test_fstring_simple() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello {name}"
"#;
    let _ = transpile(code);
}

#[test]
fn test_fstring_expression() {
    let code = r#"
def show_sum(a: int, b: int) -> str:
    return f"{a} + {b} = {a + b}"
"#;
    let _ = transpile(code);
}

#[test]
fn test_fstring_nested() {
    let code = r#"
def format_items(items: list[int]) -> str:
    return f"Items: {len(items)}"
"#;
    let _ = transpile(code);
}

// ============================================================================
// Yield/Generator COVERAGE TESTS
// ============================================================================

#[test]
fn test_yield_simple() {
    let code = r#"
def count_up(n: int) -> int:
    for i in range(n):
        yield i
"#;
    let _ = transpile(code);
}

#[test]
fn test_yield_expression() {
    let code = r#"
def squares(n: int) -> int:
    for i in range(n):
        yield i * i
"#;
    let _ = transpile(code);
}

// ============================================================================
// Await/Async COVERAGE TESTS
// ============================================================================

#[test]
fn test_async_function() {
    let code = r#"
async def fetch(url: str) -> str:
    return url
"#;
    let _ = transpile(code);
}

#[test]
fn test_await_expression() {
    let code = r#"
async def process() -> str:
    result = await fetch("http://example.com")
    return result
"#;
    let _ = transpile(code);
}

// ============================================================================
// NamedExpr (Walrus) COVERAGE TESTS
// ============================================================================

#[test]
fn test_walrus_in_if() {
    let code = r#"
def process(items: list[int]) -> int:
    if (n := len(items)) > 0:
        return n
    return 0
"#;
    let _ = transpile(code);
}

#[test]
fn test_walrus_in_while() {
    let code = r#"
def read_chunks(data: list[int]) -> int:
    total = 0
    i = 0
    while (chunk := data[i] if i < len(data) else 0) > 0:
        total = total + chunk
        i = i + 1
    return total
"#;
    let _ = transpile(code);
}

#[test]
fn test_walrus_in_list_comp() {
    let code = r#"
def process(items: list[int]) -> list[int]:
    return [y for x in items if (y := x * 2) > 0]
"#;
    let _ = transpile(code);
}

// ============================================================================
// Borrow expression COVERAGE TESTS
// ============================================================================

#[test]
fn test_borrow_variable() {
    let code = r#"
def process(items: list[int]) -> int:
    return len(items)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Complex if-else patterns COVERAGE TESTS
// ============================================================================

#[test]
fn test_if_in_method_call() {
    let code = r#"
def process(x: int) -> str:
    result = "positive" if x > 0 else "non-positive"
    return result
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_nested_ternary() {
    let code = r#"
def classify(x: int) -> str:
    return "positive" if x > 0 else ("negative" if x < 0 else "zero")
"#;
    let _ = transpile(code);
}

// ============================================================================
// Dict comprehension COVERAGE TESTS
// ============================================================================

#[test]
fn test_dict_comprehension_simple() {
    let code = r#"
def square_dict(items: list[int]) -> dict[int, int]:
    return {x: x * x for x in items}
"#;
    let _ = transpile(code);
}

#[test]
fn test_dict_comprehension_with_filter() {
    let code = r#"
def positive_squares(items: list[int]) -> dict[int, int]:
    return {x: x * x for x in items if x > 0}
"#;
    let _ = transpile(code);
}

// ============================================================================
// Try/Except edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_try_with_orelse() {
    let code = r#"
def safe_process(x: int) -> int:
    try:
        result = x * 2
    except ValueError:
        result = 0
    else:
        result = result + 1
    return result
"#;
    let _ = transpile(code);
}

#[test]
fn test_try_nested() {
    let code = r#"
def complex_process(x: int) -> int:
    try:
        try:
            return x * 2
        except ValueError:
            return 0
    except TypeError:
        return -1
"#;
    let _ = transpile(code);
}

#[test]
fn test_raise_from() {
    let code = r#"
def process(x: int) -> int:
    try:
        return x
    except ValueError as e:
        raise RuntimeError("failed") from e
"#;
    let _ = transpile(code);
}

// ============================================================================
// With statement edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_with_multiple_items() {
    let code = r#"
def copy_file(src: str, dst: str) -> None:
    with open(src, "r") as f1, open(dst, "w") as f2:
        f2.write(f1.read())
"#;
    let _ = transpile(code);
}

// ============================================================================
// For loop edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_for_with_else() {
    let code = r#"
def find_target(items: list[int], target: int) -> int:
    for i, item in enumerate(items):
        if item == target:
            return i
    else:
        return -1
"#;
    let _ = transpile(code);
}

#[test]
fn test_for_range_step() {
    let code = r#"
def sum_even_indices(items: list[int]) -> int:
    total = 0
    for i in range(0, len(items), 2):
        total = total + items[i]
    return total
"#;
    let _ = transpile(code);
}

#[test]
fn test_for_range_negative_step() {
    let code = r#"
def reverse_sum(n: int) -> int:
    total = 0
    for i in range(n, 0, -1):
        total = total + i
    return total
"#;
    let _ = transpile(code);
}

// ============================================================================
// Augmented assignment edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_augassign_floordiv() {
    let code = r#"
def halve(x: int) -> int:
    x //= 2
    return x
"#;
    let _ = transpile(code);
}

#[test]
fn test_augassign_mod() {
    let code = r#"
def mod_ten(x: int) -> int:
    x %= 10
    return x
"#;
    let _ = transpile(code);
}

#[test]
fn test_augassign_pow() {
    let code = r#"
def square(x: int) -> int:
    x **= 2
    return x
"#;
    let _ = transpile(code);
}

#[test]
fn test_augassign_bitand() {
    let code = r#"
def mask(x: int) -> int:
    x &= 0xFF
    return x
"#;
    let _ = transpile(code);
}

#[test]
fn test_augassign_bitor() {
    let code = r#"
def set_bits(x: int) -> int:
    x |= 0x01
    return x
"#;
    let _ = transpile(code);
}

#[test]
fn test_augassign_bitxor() {
    let code = r#"
def toggle_bit(x: int) -> int:
    x ^= 0x01
    return x
"#;
    let _ = transpile(code);
}

#[test]
fn test_augassign_lshift() {
    let code = r#"
def double(x: int) -> int:
    x <<= 1
    return x
"#;
    let _ = transpile(code);
}

#[test]
fn test_augassign_rshift() {
    let code = r#"
def halve_bits(x: int) -> int:
    x >>= 1
    return x
"#;
    let _ = transpile(code);
}

// ============================================================================
// Index assignment edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_index_augassign() {
    let code = r#"
def increment_first(items: list[int]) -> list[int]:
    items[0] += 1
    return items
"#;
    let _ = transpile(code);
}

#[test]
fn test_dict_nested_assign() {
    let code = r#"
def set_nested(d: dict[str, dict[str, int]], outer: str, inner: str, val: int) -> None:
    d[outer][inner] = val
"#;
    let _ = transpile(code);
}

// ============================================================================
// Special expression patterns COVERAGE TESTS
// ============================================================================

#[test]
fn test_sorted_with_key() {
    let code = r#"
def sort_by_length(items: list[str]) -> list[str]:
    return sorted(items, key=len)
"#;
    let _ = transpile(code);
}

#[test]
fn test_sorted_with_reverse() {
    let code = r#"
def sort_descending(items: list[int]) -> list[int]:
    return sorted(items, reverse=True)
"#;
    let _ = transpile(code);
}

#[test]
fn test_sorted_with_key_and_reverse() {
    let code = r#"
def sort_by_length_desc(items: list[str]) -> list[str]:
    return sorted(items, key=len, reverse=True)
"#;
    let _ = transpile(code);
}

// ============================================================================
// Binary operation edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_binary_is() {
    let code = r#"
def check_none(x: int) -> bool:
    return x is None
"#;
    let _ = transpile(code);
}

#[test]
fn test_binary_is_not() {
    let code = r#"
def check_not_none(x: int) -> bool:
    return x is not None
"#;
    let _ = transpile(code);
}

#[test]
fn test_binary_in() {
    let code = r#"
def contains(items: list[int], x: int) -> bool:
    return x in items
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_binary_not_in() {
    let code = r#"
def not_contains(items: list[int], x: int) -> bool:
    return x not in items
"#;
    let _ = transpile(code);
}

// ============================================================================
// Method call edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_method_chain() {
    let code = r#"
def process(s: str) -> str:
    return s.strip().lower()
"#;
    let _ = transpile(code);
}

#[test]
fn test_method_on_literal() {
    let code = r#"
def get_words() -> list[str]:
    return "hello world".split()
"#;
    let _ = transpile(code);
}

// ============================================================================
// Class method edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_class_classmethod() {
    let code = r#"
class Factory:
    @classmethod
    def create(cls) -> int:
        return 0
"#;
    let _ = transpile(code);
}

#[test]
fn test_class_property() {
    let code = r#"
class Rectangle:
    def __init__(self, w: int, h: int) -> None:
        self.width = w
        self.height = h

    @property
    def area(self) -> int:
        return self.width * self.height
"#;
    let _ = transpile(code);
}

// ============================================================================
// Exception types COVERAGE TESTS
// ============================================================================

#[test]
fn test_except_index_error() {
    let code = r#"
def safe_get(items: list[int], i: int) -> int:
    try:
        return items[i]
    except IndexError:
        return -1
"#;
    let _ = transpile(code);
}

#[test]
fn test_except_key_error() {
    let code = r#"
def safe_get(d: dict[str, int], key: str) -> int:
    try:
        return d[key]
    except KeyError:
        return 0
"#;
    let _ = transpile(code);
}

#[test]
fn test_except_attribute_error() {
    let code = r#"
def safe_attr(obj: object) -> str:
    try:
        return str(obj.name)
    except AttributeError:
        return "unknown"
"#;
    let _ = transpile(code);
}

// ============================================================================
// Comparison chaining COVERAGE TESTS
// ============================================================================

#[test]
fn test_chained_less_than() {
    let code = r#"
def in_range(x: int, low: int, high: int) -> bool:
    return low < x < high
"#;
    let _ = transpile(code);
}

#[test]
fn test_chained_less_equal() {
    let code = r#"
def in_range_inclusive(x: int, low: int, high: int) -> bool:
    return low <= x <= high
"#;
    let _ = transpile(code);
}

// ============================================================================
// FrozenSet COVERAGE TESTS
// ============================================================================

#[test]
fn test_frozenset_literal() {
    let code = r#"
def get_constants() -> frozenset[int]:
    return frozenset({1, 2, 3})
"#;
    let _ = transpile(code);
}

// ============================================================================
// Multiple assignment targets COVERAGE TESTS
// ============================================================================

#[test]
fn test_tuple_unpack_three() {
    let code = r#"
def process() -> int:
    a, b, c = 1, 2, 3
    return a + b + c
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_tuple_unpack_nested() {
    let code = r#"
def process() -> int:
    (a, b), c = (1, 2), 3
    return a + b + c
"#;
    let _ = transpile(code);
}

#[test]
fn test_star_unpack() {
    let code = r#"
def process(items: list[int]) -> int:
    first, *rest = items
    return first + len(rest)
"#;
    let _ = transpile(code);
}

// ============================================================================
// Global/Nonlocal COVERAGE TESTS
// ============================================================================

#[test]
fn test_global_var() {
    let code = r#"
counter = 0

def increment() -> int:
    global counter
    counter = counter + 1
    return counter
"#;
    let _ = transpile(code);
}

#[test]
fn test_nonlocal_var() {
    let code = r#"
def outer() -> int:
    count = 0
    def inner() -> int:
        nonlocal count
        count = count + 1
        return count
    return inner()
"#;
    let _ = transpile(code);
}

// ============================================================================
// Argparse COVERAGE TESTS
// ============================================================================

#[test]
fn test_argparse_add_argument() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", help="Input file")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_type_str() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str, help="Name")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_type_int() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--count", type=int, help="Count")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_type_float() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--rate", type=float, help="Rate")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_action_store_true() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true", help="Verbose")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_action_store_false() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--quiet", action="store_false", help="Quiet")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_action_count() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="count", help="Verbosity")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_nargs_plus() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="+", help="Files")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_nargs_star() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="*", help="Files")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_nargs_question() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("output", nargs="?", help="Output")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_required() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True, help="Input")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_default() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--count", default=0, help="Count")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_choices() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--level", choices=["low", "medium", "high"], help="Level")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_dest() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("-n", dest="count", help="Count")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_metavar() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", metavar="FILE", help="Input file")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_subparsers() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    parser_init = subparsers.add_parser("init", help="Initialize")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_subparser_add_argument() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    parser_init = subparsers.add_parser("init", help="Initialize")
    parser_init.add_argument("--force", action="store_true", help="Force")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_group() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    group = parser.add_argument_group("options")
    group.add_argument("--input", help="Input")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_mutually_exclusive_group() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--verbose", action="store_true")
    group.add_argument("--quiet", action="store_true")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_set_defaults() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.set_defaults(verbose=False)
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_add_argument_const() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--opt", action="store_const", const=42)
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_subparser_set_defaults() {
    let code = r#"
import argparse

def cmd_init() -> None:
    pass

def main() -> None:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    parser_init = subparsers.add_parser("init")
    parser_init.set_defaults(func=cmd_init)
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_type_path() {
    let code = r#"
import argparse
from pathlib import Path

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--path", type=Path, help="Path")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_argparse_custom_type() {
    let code = r#"
import argparse

def validate_email(s: str) -> str:
    return s

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--email", type=validate_email, help="Email")
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

// ============================================================================
// JSON parsing COVERAGE TESTS
// ============================================================================

#[test]
fn test_json_loads() {
    let code = r#"
import json

def parse(s: str) -> dict[str, int]:
    return json.loads(s)
"#;
    let _ = transpile(code);
}

#[test]
fn test_json_dumps() {
    let code = r#"
import json

def serialize(d: dict[str, int]) -> str:
    return json.dumps(d)
"#;
    let _ = transpile(code);
}

#[test]
fn test_json_value_access() {
    let code = r#"
import json

def get_name(s: str) -> str:
    data = json.loads(s)
    return data["name"]
"#;
    let _ = transpile(code);
}

#[test]
fn test_json_as_str() {
    let code = r#"
import json

def get_value(s: str) -> str:
    data = json.loads(s)
    return data["key"].as_str()
"#;
    let _ = transpile(code);
}

#[test]
fn test_json_as_i64() {
    let code = r#"
import json

def get_count(s: str) -> int:
    data = json.loads(s)
    return data["count"].as_i64()
"#;
    let _ = transpile(code);
}

// ============================================================================
// Exception handling edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_except_bare_raise() {
    let code = r#"
def process(x: int) -> int:
    try:
        return x
    except ValueError:
        raise
"#;
    let _ = transpile(code);
}

#[test]
fn test_except_exit_in_handler() {
    let code = r#"
import sys

def process(x: int) -> int:
    try:
        return x
    except ValueError:
        sys.exit(1)
"#;
    let _ = transpile(code);
}

#[test]
fn test_except_with_continue() {
    let code = r#"
def process(items: list[int]) -> int:
    total = 0
    for item in items:
        try:
            total = total + item
        except ValueError:
            continue
    return total
"#;
    let _ = transpile(code);
}

#[test]
fn test_except_with_break() {
    let code = r#"
def process(items: list[int]) -> int:
    for item in items:
        try:
            return item
        except ValueError:
            break
    return 0
"#;
    let _ = transpile(code);
}

// ============================================================================
// File/IO operations COVERAGE TESTS
// ============================================================================

#[test]
fn test_file_create() {
    let code = r#"
def write_file(path: str, content: str) -> None:
    f = open(path, "w")
    f.write(content)
    f.close()
"#;
    let _ = transpile(code);
}

#[test]
fn test_file_read_lines() {
    let code = r#"
def read_lines(path: str) -> list[str]:
    f = open(path, "r")
    lines = f.readlines()
    f.close()
    return lines
"#;
    let _ = transpile(code);
}

#[test]
fn test_file_with_binary_mode() {
    let code = r#"
def read_binary(path: str) -> bytes:
    with open(path, "rb") as f:
        return f.read()
"#;
    let _ = transpile(code);
}

// ============================================================================
// Complex pattern matching COVERAGE TESTS
// ============================================================================

#[test]
fn test_match_simple() {
    let code = r#"
def classify(x: int) -> str:
    match x:
        case 0:
            return "zero"
        case 1:
            return "one"
        case _:
            return "other"
"#;
    let _ = transpile(code);
}

#[test]
fn test_match_tuple() {
    let code = r#"
def classify(point: tuple[int, int]) -> str:
    match point:
        case (0, 0):
            return "origin"
        case (x, 0):
            return "x-axis"
        case (0, y):
            return "y-axis"
        case (x, y):
            return "other"
"#;
    let _ = transpile(code);
}

// ============================================================================
// Complex assignment patterns COVERAGE TESTS
// ============================================================================

#[test]
fn test_assign_to_slice() {
    let code = r#"
def replace_slice(items: list[int]) -> list[int]:
    items[1:3] = [10, 20]
    return items
"#;
    let _ = transpile(code);
}

#[test]
fn test_assign_delete() {
    let code = r#"
def remove_key(d: dict[str, int], key: str) -> dict[str, int]:
    del d[key]
    return d
"#;
    let _ = transpile(code);
}

// ============================================================================
// Iterator methods COVERAGE TESTS
// ============================================================================

#[test]
fn test_iter_take() {
    let code = r#"
def first_three(items: list[int]) -> list[int]:
    return list(iter(items).take(3))
"#;
    let _ = transpile(code);
}

#[test]
fn test_iter_skip() {
    let code = r#"
def skip_three(items: list[int]) -> list[int]:
    return list(iter(items).skip(3))
"#;
    let _ = transpile(code);
}

#[test]
fn test_iter_chain() {
    let code = r#"
def combine(a: list[int], b: list[int]) -> list[int]:
    return list(iter(a).chain(b))
"#;
    let _ = transpile(code);
}

// ============================================================================
// Box/Dyn Write COVERAGE TESTS
// ============================================================================

#[test]
fn test_dyn_write_stdout_or_file() {
    let code = r#"
import sys

def write_output(use_stdout: bool, path: str, content: str) -> None:
    if use_stdout:
        writer = sys.stdout
    else:
        writer = open(path, "w")
    writer.write(content)
"#;
    let _ = transpile(code);
}

#[test]
fn test_dyn_write_stderr_or_file() {
    let code = r#"
import sys

def write_error(use_stderr: bool, path: str, content: str) -> None:
    if use_stderr:
        writer = sys.stderr
    else:
        writer = open(path, "w")
    writer.write(content)
"#;
    let _ = transpile(code);
}
