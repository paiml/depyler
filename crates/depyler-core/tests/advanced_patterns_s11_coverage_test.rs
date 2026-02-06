//! Session 11: Advanced Python pattern tests for deep code path coverage
//!
//! These tests exercise complex, multi-feature Python patterns that trigger
//! deeper code generation paths in stmt_gen, expr_gen, and instance methods.

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
// Dict comprehension variations
// ============================================================================

#[test]
fn test_s11_dict_from_list_indices() {
    let code = r#"
def index_map(items: list) -> dict:
    return {item: idx for idx, item in enumerate(items)}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn index_map"),
        "Should transpile dict comp with enumerate. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_with_condition() {
    let code = r#"
def filtered_dict(items: list) -> dict:
    return {x: x * x for x in items if x > 0}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn filtered_dict"),
        "Should transpile dict comp with filter. Got: {}",
        result
    );
}

// ============================================================================
// Set comprehension
// ============================================================================

#[test]
fn test_s11_set_from_list() {
    let code = r#"
def unique_vals(items: list) -> set:
    return {x for x in items}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn unique_vals"),
        "Should transpile set comp. Got: {}",
        result
    );
}

#[test]
fn test_s11_set_comp_with_transform() {
    let code = r#"
def abs_set(items: list) -> set:
    return {abs(x) for x in items}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn abs_set"),
        "Should transpile set comp with transform. Got: {}",
        result
    );
}

// ============================================================================
// Nested comprehensions
// ============================================================================

#[test]
fn test_s11_nested_list_comp() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn flatten"),
        "Should transpile nested list comp. Got: {}",
        result
    );
}

#[test]
fn test_s11_list_comp_with_method() {
    let code = r#"
def upper_words(words: list) -> list:
    return [w.upper() for w in words]
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_uppercase") || result.contains("fn upper_words"),
        "Should transpile list comp with method. Got: {}",
        result
    );
}

// ============================================================================
// Multiple return paths
// ============================================================================

#[test]
fn test_s11_early_return_guard() {
    let code = r#"
def safe_sqrt(x: float) -> float:
    if x < 0:
        return 0.0
    return x ** 0.5
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_sqrt"),
        "Should transpile early return guard. Got: {}",
        result
    );
}

#[test]
fn test_s11_return_in_loop() {
    let code = r#"
def find_max(items: list) -> int:
    if len(items) == 0:
        return 0
    best: int = items[0]
    for item in items:
        if item > best:
            best = item
    return best
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_max"),
        "Should transpile return after loop. Got: {}",
        result
    );
}

// ============================================================================
// Complex class patterns
// ============================================================================

#[test]
fn test_s11_class_with_multiple_methods() {
    let code = r#"
class Stack:
    def __init__(self) -> None:
        self.items: list = []

    def push(self, item: int) -> None:
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("Stack") || result.contains("struct"),
        "Should transpile Stack class. Got: {}",
        result
    );
}

#[test]
fn test_s11_class_with_property() {
    let code = r#"
class Rectangle:
    def __init__(self, width: float, height: float) -> None:
        self.width = width
        self.height = height

    def area(self) -> float:
        return self.width * self.height

    def perimeter(self) -> float:
        return 2 * (self.width + self.height)
"#;
    let result = transpile(code);
    assert!(
        result.contains("Rectangle") || result.contains("struct"),
        "Should transpile Rectangle class. Got: {}",
        result
    );
}

// ============================================================================
// Generator/iterator patterns
// ============================================================================

#[test]
fn test_s11_generator_sum() {
    let code = r#"
def sum_even(n: int) -> int:
    return sum(x for x in range(n) if x % 2 == 0)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sum_even"),
        "Should transpile generator with sum. Got: {}",
        result
    );
}

#[test]
fn test_s11_any_with_condition() {
    let code = r#"
def has_negative(items: list) -> bool:
    return any(x < 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("any") || result.contains("fn has_negative"),
        "Should transpile any() with generator. Got: {}",
        result
    );
}

#[test]
fn test_s11_all_with_condition() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("all") || result.contains("fn all_positive"),
        "Should transpile all() with generator. Got: {}",
        result
    );
}

// ============================================================================
// Complex string operations
// ============================================================================

#[test]
fn test_s11_string_split_and_process() {
    let code = r#"
def parse_csv_line(line: str) -> list:
    parts = line.split(",")
    return [p.strip() for p in parts]
"#;
    let result = transpile(code);
    assert!(
        result.contains("split") || result.contains("fn parse_csv_line"),
        "Should transpile split+strip. Got: {}",
        result
    );
}

#[test]
fn test_s11_string_replace_multiple() {
    let code = r#"
def clean_text(text: str) -> str:
    result: str = text.replace("\n", " ")
    result = result.replace("\t", " ")
    result = result.replace("  ", " ")
    return result.strip()
"#;
    let result = transpile(code);
    assert!(
        result.contains("replace"),
        "Should transpile multiple replace. Got: {}",
        result
    );
}

#[test]
fn test_s11_string_join_with_transform() {
    let code = r#"
def to_csv(items: list) -> str:
    return ",".join(str(x) for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_csv"),
        "Should transpile join with generator. Got: {}",
        result
    );
}

// ============================================================================
// Recursive functions
// ============================================================================

#[test]
fn test_s11_recursive_factorial() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    let result = transpile(code);
    assert!(
        result.contains("factorial"),
        "Should transpile recursive factorial. Got: {}",
        result
    );
}

#[test]
fn test_s11_recursive_tree_depth() {
    let code = r#"
def max_depth(node: dict) -> int:
    if not node:
        return 0
    left: int = max_depth(node.get("left", {}))
    right: int = max_depth(node.get("right", {}))
    return 1 + max(left, right)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn max_depth"),
        "Should transpile recursive tree depth. Got: {}",
        result
    );
}

// ============================================================================
// Error handling patterns
// ============================================================================

#[test]
fn test_s11_try_with_return() {
    let code = r#"
def safe_parse(data: str) -> int:
    try:
        return int(data)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_parse"),
        "Should transpile try with returns. Got: {}",
        result
    );
}

#[test]
fn test_s11_nested_try() {
    let code = r#"
def robust_process(data: str) -> str:
    try:
        result: str = data.strip()
        try:
            count: int = int(result)
            return str(count)
        except ValueError:
            return result
    except Exception:
        return ""
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn robust_process"),
        "Should transpile nested try. Got: {}",
        result
    );
}

// ============================================================================
// Functional patterns
// ============================================================================

#[test]
fn test_s11_map_filter_reduce() {
    let code = r#"
def process(items: list) -> int:
    evens = [x for x in items if x % 2 == 0]
    doubled = [x * 2 for x in evens]
    return sum(doubled)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn process"),
        "Should transpile map/filter/reduce. Got: {}",
        result
    );
}

#[test]
fn test_s11_sorted_with_key() {
    let code = r#"
def sort_by_length(words: list) -> list:
    return sorted(words, key=lambda w: len(w))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_by_length"),
        "Should transpile sorted with key. Got: {}",
        result
    );
}

// ============================================================================
// Complex data manipulation
// ============================================================================

#[test]
fn test_s11_group_by() {
    let code = r#"
def group_by_first_char(words: list) -> dict:
    groups: dict = {}
    for word in words:
        key: str = word[0]
        if key not in groups:
            groups[key] = []
        groups[key].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn group_by_first_char"),
        "Should transpile group by pattern. Got: {}",
        result
    );
}

#[test]
fn test_s11_matrix_transpose() {
    let code = r#"
def transpose(matrix: list) -> list:
    if not matrix:
        return []
    rows: int = len(matrix)
    cols: int = len(matrix[0])
    result: list = []
    for j in range(cols):
        row: list = []
        for i in range(rows):
            row.append(matrix[i][j])
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn transpose"),
        "Should transpile matrix transpose. Got: {}",
        result
    );
}

#[test]
fn test_s11_sliding_window() {
    let code = r#"
def moving_average(items: list, window: int) -> list:
    result: list = []
    for i in range(len(items) - window + 1):
        total: int = 0
        for j in range(window):
            total = total + items[i + j]
        result.append(total // window)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn moving_average"),
        "Should transpile sliding window. Got: {}",
        result
    );
}

// ============================================================================
// Multiple assignment patterns
// ============================================================================

#[test]
fn test_s11_swap_variables() {
    let code = r#"
def sort_pair(a: int, b: int) -> tuple:
    if a > b:
        a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_pair"),
        "Should transpile variable swap. Got: {}",
        result
    );
}

#[test]
fn test_s11_unpack_from_function() {
    let code = r#"
from typing import Tuple

def get_pair() -> Tuple[int, int]:
    return (1, 2)

def use_pair() -> int:
    a, b = get_pair()
    return a + b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn use_pair"),
        "Should transpile tuple unpacking from function. Got: {}",
        result
    );
}

// ============================================================================
// Conditional expression patterns
// ============================================================================

#[test]
fn test_s11_nested_ternary() {
    let code = r#"
def sign(x: int) -> int:
    return 1 if x > 0 else (-1 if x < 0 else 0)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sign"),
        "Should transpile nested ternary. Got: {}",
        result
    );
}

#[test]
fn test_s11_ternary_with_function() {
    let code = r#"
def safe_divide(a: float, b: float) -> float:
    return a / b if b != 0 else 0.0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_divide"),
        "Should transpile ternary with division. Got: {}",
        result
    );
}

// ============================================================================
// Multiple collection operations
// ============================================================================

#[test]
fn test_s11_list_dedup() {
    let code = r#"
def dedup(items: list) -> list:
    seen: set = set()
    result: list = []
    for item in items:
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn dedup"),
        "Should transpile dedup pattern. Got: {}",
        result
    );
}

#[test]
fn test_s11_merge_dicts() {
    let code = r#"
def merge(a: dict, b: dict) -> dict:
    result: dict = {}
    for key in a:
        result[key] = a[key]
    for key in b:
        result[key] = b[key]
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn merge"),
        "Should transpile dict merge. Got: {}",
        result
    );
}

// ============================================================================
// Numeric edge cases
// ============================================================================

#[test]
fn test_s11_float_comparison() {
    let code = r#"
def approximately_equal(a: float, b: float) -> bool:
    return abs(a - b) < 0.0001
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn approximately_equal"),
        "Should transpile float comparison. Got: {}",
        result
    );
}

#[test]
fn test_s11_integer_bounds() {
    let code = r#"
def clamp_byte(x: int) -> int:
    if x < 0:
        return 0
    if x > 255:
        return 255
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("255"),
        "Should transpile integer bounds. Got: {}",
        result
    );
}

// ============================================================================
// Complex iteration with state
// ============================================================================

#[test]
fn test_s11_run_length_encode() {
    let code = r#"
def rle(text: str) -> list:
    result: list = []
    if len(text) == 0:
        return result
    current: str = text[0]
    count: int = 1
    for i in range(1, len(text)):
        if text[i] == current:
            count = count + 1
        else:
            result.append((current, count))
            current = text[i]
            count = 1
    result.append((current, count))
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn rle"),
        "Should transpile run-length encoding. Got: {}",
        result
    );
}

#[test]
fn test_s11_two_sum() {
    let code = r#"
from typing import Tuple, Optional

def two_sum(nums: list, target: int) -> Optional[Tuple[int, int]]:
    seen: dict = {}
    for i in range(len(nums)):
        complement: int = target - nums[i]
        if complement in seen:
            return (seen[complement], i)
        seen[nums[i]] = i
    return None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn two_sum"),
        "Should transpile two-sum. Got: {}",
        result
    );
}
