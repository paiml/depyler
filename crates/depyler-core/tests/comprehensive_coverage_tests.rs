//! Comprehensive coverage tests for depyler-core
//!
//! These tests exercise the full transpilation pipeline to maximize code coverage.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::codegen::hir_to_rust;
use rustpython_ast::Suite;
use rustpython_parser::Parse;

/// Helper to create a ModModule from parsed code
fn make_module(ast: Suite) -> rustpython_ast::Mod {
    rustpython_ast::Mod::Module(rustpython_ast::ModModule {
        body: ast,
        range: Default::default(),
        type_ignores: vec![],
    })
}

fn transpile_succeeds(code: &str) -> bool {
    transpile_code(code).is_some()
}

fn transpile_code(code: &str) -> Option<String> {
    let ast = Suite::parse(code, "<test>").ok()?;
    let bridge = AstBridge::new().with_source(code.to_string());
    let (hir, _type_env) = bridge.python_to_hir(make_module(ast)).ok()?;
    let rust_code = hir_to_rust(&hir).ok()?;
    Some(rust_code)
}

// ============================================================================
// Basic Statement Coverage
// ============================================================================

#[test]
fn test_simple_assignment() {
    assert!(transpile_succeeds("x = 42"));
}

#[test]
fn test_float_assignment() {
    assert!(transpile_succeeds("x = 3.14"));
}

#[test]
fn test_string_assignment() {
    assert!(transpile_succeeds("s = 'hello'"));
}

#[test]
fn test_bool_assignment() {
    assert!(transpile_succeeds("flag = True"));
    assert!(transpile_succeeds("flag = False"));
}

#[test]
fn test_none_assignment() {
    assert!(transpile_succeeds("x = None"));
}

#[test]
fn test_list_assignment() {
    assert!(transpile_succeeds("lst = [1, 2, 3]"));
}

#[test]
fn test_empty_list_assignment() {
    assert!(transpile_succeeds("lst: list[int] = []"));
}

#[test]
fn test_dict_assignment() {
    assert!(transpile_succeeds("d = {'a': 1, 'b': 2}"));
}

#[test]
fn test_empty_dict_assignment() {
    assert!(transpile_succeeds("d: dict[str, int] = {}"));
}

#[test]
fn test_tuple_assignment() {
    assert!(transpile_succeeds("t = (1, 2, 3)"));
}

#[test]
fn test_set_assignment() {
    assert!(transpile_succeeds("s = {1, 2, 3}"));
}

// ============================================================================
// Augmented Assignment Coverage
// ============================================================================

#[test]
fn test_aug_assign_add() {
    let code = r#"
x = 1
x += 1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_sub() {
    let code = r#"
x = 10
x -= 1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_mul() {
    let code = r#"
x = 2
x *= 3
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_div() {
    let code = r#"
x = 10.0
x /= 2
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_floordiv() {
    let code = r#"
x = 10
x //= 3
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_mod() {
    let code = r#"
x = 10
x %= 3
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_pow() {
    let code = r#"
x = 2
x **= 3
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_bitor() {
    let code = r#"
x = 0b1010
x |= 0b0101
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_bitand() {
    let code = r#"
x = 0b1111
x &= 0b0101
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_bitxor() {
    let code = r#"
x = 0b1010
x ^= 0b0110
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_lshift() {
    let code = r#"
x = 1
x <<= 4
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_aug_assign_rshift() {
    let code = r#"
x = 16
x >>= 2
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Control Flow Coverage
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
fn test_if_elif_else() {
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
fn test_nested_if() {
    let code = r#"
def check(x: int, y: int) -> str:
    if x > 0:
        if y > 0:
            return "both positive"
        return "x positive"
    return "x non-positive"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_range() {
    let code = r#"
def sum_range(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_range_start_stop() {
    let code = r#"
def sum_range(start: int, stop: int) -> int:
    total = 0
    for i in range(start, stop):
        total += i
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_range_step() {
    let code = r#"
def sum_evens(n: int) -> int:
    total = 0
    for i in range(0, n, 2):
        total += i
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_list() {
    let code = r#"
def sum_list(items: list[int]) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_enumerate() {
    let code = r#"
def process_indexed(items: list[str]) -> int:
    total = 0
    for i, item in enumerate(items):
        total += i
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_zip() {
    let code = r#"
def combine(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_while_simple() {
    let code = r#"
def countdown(n: int) -> None:
    while n > 0:
        print(n)
        n -= 1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_while_true_break() {
    let code = r#"
def find_first_even(items: list[int]) -> int:
    i = 0
    while True:
        if items[i] % 2 == 0:
            return items[i]
        i += 1
        if i >= len(items):
            break
    return -1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_break_continue() {
    let code = r#"
def process(items: list[int]) -> int:
    total = 0
    for item in items:
        if item < 0:
            continue
        if item > 100:
            break
        total += item
    return total
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Function Coverage
// ============================================================================

#[test]
fn test_function_no_args() {
    let code = r#"
def greet() -> str:
    return "hello"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_one_arg() {
    let code = r#"
def double(x: int) -> int:
    return x * 2
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_multiple_args() {
    let code = r#"
def add(a: int, b: int, c: int) -> int:
    return a + b + c
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_default_args() {
    let code = r#"
def greet(name: str = "world") -> str:
    return "Hello, " + name + "!"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_return_none() {
    let code = r#"
def do_nothing() -> None:
    pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_optional_return() {
    let code = r#"
def find(items: list[int], target: int) -> int | None:
    for item in items:
        if item == target:
            return item
    return None
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Expression Coverage
// ============================================================================

#[test]
fn test_binary_ops() {
    let code = r#"
def math_ops(a: int, b: int) -> int:
    sum = a + b
    diff = a - b
    prod = a * b
    quot = a // b
    rem = a % b
    return sum + diff + prod + quot + rem
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_comparison_ops() {
    let code = r#"
def compare(a: int, b: int) -> bool:
    return a < b and a <= b and a == b or a != b and a >= b and a > b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_unary_ops() {
    let code = r#"
def unary(x: int, flag: bool) -> int:
    neg = -x
    pos = +x
    inv = ~x
    not_flag = not flag
    return neg if not_flag else pos
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_ternary() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_lambda() {
    let code = r#"
def apply(items: list[int]) -> list[int]:
    doubled = list(map(lambda x: x * 2, items))
    return doubled
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_comprehension() {
    let code = r#"
def squares(n: int) -> list[int]:
    return [x * x for x in range(n)]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_comprehension_with_filter() {
    let code = r#"
def even_squares(n: int) -> list[int]:
    return [x * x for x in range(n) if x % 2 == 0]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_comprehension() {
    let code = r#"
def square_dict(n: int) -> dict[int, int]:
    return {x: x * x for x in range(n)}
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Method Call Coverage
// ============================================================================

#[test]
fn test_string_methods() {
    let code = r#"
def process_string(s: str) -> str:
    upper = s.upper()
    lower = s.lower()
    stripped = s.strip()
    return upper + lower + stripped
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_methods() {
    let code = r#"
def process_list(items: list[int]) -> int:
    items.append(42)
    items.extend([1, 2, 3])
    items.pop()
    items.reverse()
    return len(items)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_methods() {
    let code = r#"
def process_dict(d: dict[str, int]) -> list[str]:
    keys = list(d.keys())
    vals = list(d.values())
    items = list(d.items())
    d.clear()
    return keys
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_get() {
    let code = r#"
def safe_get(d: dict[str, int], key: str) -> int:
    return d.get(key, 0)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Class Coverage
// ============================================================================

#[test]
fn test_simple_class() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def distance(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dataclass() {
    let code = r#"
from dataclasses import dataclass

@dataclass
class Person:
    name: str
    age: int
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_class_with_methods() {
    let code = r#"
class Counter:
    def __init__(self) -> None:
        self.count = 0

    def increment(self) -> None:
        self.count += 1

    def get_count(self) -> int:
        return self.count
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Exception Handling Coverage
// ============================================================================

#[test]
fn test_try_except() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_try_except_as() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError as e:
        print("Error occurred")
        return 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_try_finally() {
    let code = r#"
def with_cleanup() -> None:
    try:
        risky_operation()
    finally:
        cleanup()

def risky_operation() -> None:
    pass

def cleanup() -> None:
    pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_raise() {
    let code = r#"
def validate(x: int) -> None:
    if x < 0:
        raise ValueError("x must be non-negative")
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Type Annotation Coverage
// ============================================================================

#[test]
fn test_optional_type() {
    let code = r#"
def maybe_int(x: int | None) -> int:
    if x is None:
        return 0
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_union_type() {
    let code = r#"
def to_str(x: int | float | str) -> str:
    return str(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_generic_list() {
    let code = r#"
def first(items: list[int]) -> int:
    return items[0]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_generic_dict() {
    let code = r#"
def keys(d: dict[str, int]) -> list[str]:
    return list(d.keys())
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_tuple_type() {
    let code = r#"
def pair(a: int, b: str) -> tuple[int, str]:
    return (a, b)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Builtin Function Coverage
// ============================================================================

#[test]
fn test_len() {
    let code = r#"
def count(items: list[int]) -> int:
    return len(items)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_print() {
    let code = r#"
def greet(name: str) -> None:
    print("Hello, " + name + "!")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_range_one_arg() {
    let code = r#"
def count_up(n: int) -> list[int]:
    return list(range(n))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_int_cast() {
    let code = r#"
def to_int(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_float_cast() {
    let code = r#"
def to_float(s: str) -> float:
    return float(s)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_str_cast() {
    let code = r#"
def to_str(x: int) -> str:
    return str(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_bool_cast() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_abs() {
    let code = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_min_max() {
    let code = r#"
def extremes(items: list[int]) -> tuple[int, int]:
    return (min(items), max(items))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_sum() {
    let code = r#"
def total(items: list[int]) -> int:
    return sum(items)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_sorted() {
    let code = r#"
def sort_items(items: list[int]) -> list[int]:
    return sorted(items)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_reversed() {
    let code = r#"
def reverse_items(items: list[int]) -> list[int]:
    return list(reversed(items))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_enumerate_builtin() {
    let code = r#"
def indexed(items: list[str]) -> list[tuple[int, str]]:
    return list(enumerate(items))
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Complex Pattern Coverage
// ============================================================================

#[test]
fn test_nested_data_structures() {
    let code = r#"
def process(data: dict[str, list[int]]) -> int:
    total = 0
    for key, values in data.items():
        for v in values:
            total += v
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_chained_methods() {
    let code = r#"
def process(s: str) -> str:
    return s.strip().lower().replace("a", "b")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_complex_expression() {
    let code = r#"
def calc(a: int, b: int, c: int) -> int:
    return (a + b) * c // 2 - (a - b) % c
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_string_formatting() {
    let code = r#"
def format_message(name: str, count: int) -> str:
    return name + " has " + str(count) + " items"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_multiple_return_paths() {
    let code = r#"
def classify(x: int) -> str:
    if x < 0:
        return "negative"
    elif x == 0:
        return "zero"
    elif x < 10:
        return "small"
    elif x < 100:
        return "medium"
    else:
        return "large"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_early_return() {
    let code = r#"
def find_index(items: list[int], target: int) -> int:
    for i, item in enumerate(items):
        if item == target:
            return i
    return -1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_accumulator_pattern() {
    let code = r#"
def count_positive(items: list[int]) -> int:
    count = 0
    for item in items:
        if item > 0:
            count += 1
    return count
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_builder_pattern() {
    let code = r#"
def build_list(n: int) -> list[int]:
    result: list[int] = []
    for i in range(n):
        if i % 2 == 0:
            result.append(i * 2)
        else:
            result.append(i * 3)
    return result
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Edge Cases
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
fn test_single_line_function() {
    let code = "def identity(x: int) -> int: return x";
    assert!(transpile_succeeds(code));
}

#[test]
fn test_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def sub(a: int, b: int) -> int:
    return a - b

def mul(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_calling_function() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

def main(x: int) -> int:
    return helper(x) + 1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_deeply_nested() {
    let code = r#"
def deep(a: int) -> int:
    if a > 0:
        if a > 10:
            if a > 100:
                return 3
            return 2
        return 1
    return 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_complex_conditions() {
    let code = r#"
def check(a: int, b: int, c: int) -> bool:
    return (a > 0 and b > 0) or (c > 0 and not (a < 0 or b < 0))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_string_operations() {
    let code = r#"
def process_text(text: str) -> list[str]:
    words = text.split()
    filtered = [w for w in words if len(w) > 3]
    return filtered
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_index_operations() {
    let code = r#"
def array_ops(arr: list[int]) -> int:
    first = arr[0]
    last = arr[-1]
    middle = arr[len(arr) // 2]
    return first + last + middle
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_slice_operations() {
    let code = r#"
def slice_ops(arr: list[int]) -> list[int]:
    head = arr[:5]
    tail = arr[-5:]
    middle = arr[2:8]
    return head + tail + middle
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_membership_test() {
    let code = r#"
def contains(items: list[int], target: int) -> bool:
    return target in items
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_not_in() {
    let code = r#"
def not_contains(items: list[int], target: int) -> bool:
    return target not in items
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_is_none() {
    let code = r#"
def is_missing(x: int | None) -> bool:
    return x is None
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_is_not_none() {
    let code = r#"
def is_present(x: int | None) -> bool:
    return x is not None
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Assertion Coverage
// ============================================================================

#[test]
fn test_assert_simple() {
    let code = r#"
def validate(x: int) -> None:
    assert x > 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assert_with_message() {
    let code = r#"
def validate(x: int) -> None:
    assert x > 0, "x must be positive"
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Global/Module Level
// ============================================================================

#[test]
fn test_module_level_constant() {
    let code = r#"
PI = 3.14159

def circle_area(r: float) -> float:
    return PI * r * r
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_multiple_statements() {
    let code = r#"
x = 1
y = 2
z = x + y
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Import Coverage
// ============================================================================

#[test]
fn test_import_typing() {
    let code = r#"
from typing import List, Dict, Optional

def process(items: List[int], mapping: Dict[str, int], opt: Optional[str]) -> None:
    pass
"#;
    assert!(transpile_succeeds(code));
}
