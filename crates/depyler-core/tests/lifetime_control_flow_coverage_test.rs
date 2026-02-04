//! Coverage tests for lifetime_analysis.rs and control_flow_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets lifetime_analysis.rs (79.79%) and
//! control_flow_analysis.rs (86.04%) coverage improvements.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Lifetime analysis - try/except parameter usage
// ============================================================================

#[test]
fn test_lifetime_try_except_param_usage() {
    let code = r#"
def process(data: list, fallback: int) -> int:
    try:
        result = data[0]
    except IndexError:
        result = fallback
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_try_finally_param() {
    let code = r#"
def process(data: list) -> int:
    result = 0
    try:
        result = data[0]
    finally:
        print(len(data))
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_try_except_finally_param() {
    let code = r#"
def process(data: list, default: int) -> int:
    try:
        result = data[0]
    except (IndexError, TypeError):
        result = default
    finally:
        print(len(data))
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_list_comprehension_param() {
    let code = r#"
def filter_items(items: list, threshold: int) -> list:
    return [x for x in items if x > threshold]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_dict_comprehension_param() {
    let code = r#"
def build_map(keys: list, prefix: str) -> dict:
    return {k: prefix + k for k in keys}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_set_comprehension_param() {
    let code = r#"
def unique_transformed(items: list, multiplier: int) -> set:
    return {x * multiplier for x in items}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_generator_expression_param() {
    let code = r#"
def sum_filtered(items: list, threshold: int) -> int:
    return sum(x for x in items if x > threshold)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_nested_comprehension_param() {
    let code = r#"
from typing import List
def flatten(lists: List[List[int]]) -> List[int]:
    return [x for lst in lists for x in lst]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_nested_captures_param() {
    let code = r#"
def outer(prefix: str, suffix: str) -> str:
    def inner() -> str:
        return prefix + suffix
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_multi_param_references() {
    let code = r#"
def merge(a: list, b: list) -> list:
    result = []
    i = 0
    j = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i += 1
        else:
            result.append(b[j])
            j += 1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_param_in_loop_condition() {
    let code = r#"
def search(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow analysis - return path analysis
// ============================================================================

#[test]
fn test_control_flow_if_else_returns() {
    let code = r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    else:
        return "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_control_flow_try_all_paths_return() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
    except TypeError:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_control_flow_nested_returns() {
    let code = r#"
def deep_classify(x: int, y: int) -> str:
    if x > 0:
        if y > 0:
            return "Q1"
        else:
            return "Q4"
    else:
        if y > 0:
            return "Q2"
        else:
            return "Q3"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow - variable escaping
// ============================================================================

#[test]
fn test_variable_escaping_if() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        label = "positive"
    else:
        label = "non-positive"
    return label
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_variable_escaping_try() {
    let code = r#"
def f(s: str) -> int:
    try:
        value = int(s)
    except ValueError:
        value = 0
    return value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_variable_escaping_nested_loops() {
    let code = r#"
def f(matrix: list) -> int:
    last = 0
    for row in matrix:
        for val in row:
            last = val
    return last
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_variable_escaping_try_in_loop() {
    let code = r#"
def f(items: list) -> int:
    count = 0
    for item in items:
        try:
            val = int(item)
            count += val
        except:
            pass
    return count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow - variable usage in expressions
// ============================================================================

#[test]
fn test_var_used_in_fstring() {
    let code = r#"
def f(name: str) -> str:
    return f"Hello, {name}!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_used_in_walrus() {
    let code = r#"
def f(items: list) -> int:
    if (n := len(items)) > 0:
        return n
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_used_in_lambda() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_used_in_slice() {
    let code = r#"
def f(items: list) -> list:
    n = len(items)
    return items[:n // 2]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_used_in_generator_expr() {
    let code = r#"
def f(items: list) -> int:
    return sum(x * x for x in items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow - special collections and patterns
// ============================================================================

#[test]
fn test_frozenset_creation() {
    let code = r#"
def f() -> int:
    s = frozenset([1, 2, 3])
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sorted_with_key() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_function() {
    let code = r#"
def count_up(n: int):
    for i in range(n):
        yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_with_condition() {
    let code = r#"
def even_numbers(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow - class analysis
// ============================================================================

#[test]
fn test_class_with_property() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    @property
    def area(self) -> float:
        return 3.14159 * self.radius * self.radius
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_with_staticmethod() {
    let code = r#"
class MathUtils:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow - real-world patterns
// ============================================================================

#[test]
fn test_binary_search() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    low = 0
    high = len(items) - 1
    while low <= high:
        mid = (low + high) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stack_implementation() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_linked_operations() {
    let code = r#"
def process_data(raw: list) -> dict:
    filtered = [x for x in raw if x > 0]
    total = sum(filtered)
    count = len(filtered)
    if count == 0:
        return {"total": 0, "avg": 0, "count": 0}
    avg = total // count
    return {"total": total, "avg": avg, "count": count}
"#;
    assert!(transpile_ok(code));
}
