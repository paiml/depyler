//! 99-mode coverage tests for stmt_gen.rs
//!
//! DEPYLER-99MODE-001: Targets stmt_gen.rs coverage (77.49% → 90%)
//! Covers: assignment variants, for loop patterns, try/except complex,
//! raise edge cases, with statements, truthiness, nested functions.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Assignment target variants
// ============================================================================

#[test]
fn test_tuple_unpacking_assign() {
    let code = r#"
def f() -> int:
    a, b, c = 1, 2, 3
    return a + b + c
"#;
    assert!(transpile_ok(code));
}

// Nested tuple unpacking not supported yet
// #[test]
// fn test_nested_tuple_unpack() { ... }

#[test]
fn test_index_assign() {
    let code = r#"
def f() -> int:
    data = [1, 2, 3]
    data[0] = 10
    return data[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_attr_assign() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    def set_x(self, val: int):
        self.x = val
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_augmented_assign() {
    let code = r#"
def f() -> int:
    d = {"a": 1, "b": 2}
    d["a"] += 10
    return d["a"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_empty_list_typed() {
    let code = r#"
from typing import List
def f() -> List[int]:
    items: List[int] = []
    items.append(1)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_empty_dict_typed() {
    let code = r#"
from typing import Dict
def f() -> Dict[str, int]:
    d: Dict[str, int] = {}
    d["key"] = 42
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_typed_assign() {
    let code = r#"
from typing import Set
def f() -> int:
    s: Set[int] = {1, 2, 3}
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optional_dict_assign() {
    let code = r#"
from typing import Optional, Dict
def f() -> int:
    config: Optional[Dict[str, int]] = {"debug": 1}
    if config is not None:
        return config["debug"]
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// For loop patterns
// ============================================================================

#[test]
fn test_for_enumerate_loop() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i, val in enumerate(items):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_zip_loop() {
    let code = r#"
def f(a: list, b: list) -> int:
    total = 0
    for x, y in zip(a, b):
        total += 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_dict_items_loop() {
    let code = r#"
def f() -> int:
    d = {"a": 1, "b": 2}
    total = 0
    for k, v in d.items():
        total += v
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_string_chars_loop() {
    let code = r#"
def f(text: str) -> int:
    count = 0
    for ch in text:
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_range_step_loop() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(0, 10, 2):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_negative_step_loop() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10, 0, -1):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_reversed_loop() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in reversed(items):
        total += 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_sorted_loop() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in sorted(items):
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_char_methods() {
    let code = r#"
def f(text: str) -> str:
    result = ""
    for ch in text:
        result += ch.upper()
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_stdin_loop() {
    let code = r#"
import sys
def f():
    for line in sys.stdin:
        print(line.strip())
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Try/except complex patterns
// ============================================================================

#[test]
fn test_try_multi_handlers() {
    let code = r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_bare_except() {
    let code = r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except:
        return -99
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_else_clause() {
    let code = r#"
def f(x: int) -> int:
    try:
        result = 100 // x
    except ZeroDivisionError:
        result = 0
    else:
        result = result * 2
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_finally_clause() {
    let code = r#"
def f(s: str) -> int:
    result = 0
    try:
        result = int(s)
    except ValueError:
        result = -1
    finally:
        print("done")
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_all_parts() {
    let code = r#"
def f(x: int) -> int:
    result = 0
    try:
        result = 100 // x
    except ZeroDivisionError:
        result = -1
    else:
        result += 10
    finally:
        print("cleanup")
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_named_exc() {
    let code = r#"
def f(s: str) -> str:
    try:
        x = int(s)
        return str(x)
    except ValueError as e:
        return str(e)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_floor_div_zero() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_var_hoisting() {
    let code = r#"
def f() -> str:
    try:
        result = "success"
    except Exception:
        result = "error"
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Raise statement edge cases
// ============================================================================

#[test]
fn test_raise_value_err() {
    let code = r#"
def f(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_raise_type_err() {
    let code = r#"
def f(x: int) -> int:
    if x == 0:
        raise TypeError("expected non-zero")
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_raise_runtime_err() {
    let code = r#"
def f() -> int:
    raise RuntimeError("not implemented")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_raise_not_impl() {
    let code = r#"
def f():
    raise NotImplementedError("subclass must implement")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bare_re_raise() {
    let code = r#"
def f(x: int) -> int:
    try:
        return 100 // x
    except ZeroDivisionError:
        raise
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// With statement variants
// ============================================================================

#[test]
fn test_with_file_read() {
    let code = r#"
def f(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_with_file_write() {
    let code = r#"
def f(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_with_no_as() {
    let code = r#"
def f():
    with open("test.txt"):
        print("inside context")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Truthiness and negation
// ============================================================================

#[test]
fn test_truth_empty_list() {
    let code = r#"
def f(items: list) -> bool:
    if items:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_truth_empty_str() {
    let code = r#"
def f(s: str) -> bool:
    if s:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_not_truth_list() {
    let code = r#"
def f(items: list) -> bool:
    if not items:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_not_strip_result() {
    let code = r#"
def f(s: str) -> bool:
    if not s.strip():
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_while_truth() {
    let code = r#"
def f(items: list) -> int:
    count = 0
    while items:
        items.pop()
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_self_attr_truth() {
    let code = r#"
class Config:
    def __init__(self):
        self.options = []
    def has_options(self) -> bool:
        if not self.options:
            return False
        return True
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested function definitions
// ============================================================================

#[test]
fn test_nested_fn_simple() {
    let code = r#"
def outer() -> int:
    def inner(x: int) -> int:
        return x + 1
    return inner(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_captures() {
    let code = r#"
def make_adder(n: int):
    def adder(x: int) -> int:
        return x + n
    return adder
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_recursive() {
    let code = r#"
def outer() -> int:
    def factorial(n: int) -> int:
        if n <= 1:
            return 1
        return n * factorial(n - 1)
    return factorial(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sibling_nested_fns() {
    let code = r#"
def outer() -> str:
    def proc_a() -> str:
        result = "a"
        return result
    def proc_b() -> str:
        result = "b"
        return result
    return proc_a() + proc_b()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex statement combinations
// ============================================================================

#[test]
fn test_elif_chain() {
    let code = r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_while_break() {
    let code = r#"
def f(items: list, target: int) -> int:
    i = 0
    while i < len(items):
        if items[i] == target:
            break
        i += 1
    return i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_break_continue() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(20):
        if i % 3 == 0:
            continue
        if i > 15:
            break
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_word_count_pattern() {
    let code = r#"
def word_count(text: str) -> dict:
    counts = {}
    for word in text.split():
        if word in counts:
            counts[word] += 1
        else:
            counts[word] = 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_guard_clauses() {
    let code = r#"
def process(data: list) -> int:
    if not data:
        return 0
    if len(data) == 1:
        return data[0]
    return sum(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_chained_methods() {
    let code = r#"
def f(text: str) -> str:
    return text.strip().lower().replace(" ", "_")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_comprehension_assign() {
    let code = r#"
def f(n: int) -> list:
    squares = [i * i for i in range(n)]
    return squares
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_comprehension_assign() {
    let code = r#"
def f(n: int) -> dict:
    squares = {i: i * i for i in range(n)}
    return squares
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ternary_assign() {
    let code = r#"
def f(x: int) -> str:
    result = "positive" if x > 0 else "non-positive"
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_double_negation() {
    let code = r#"
def f(x: int) -> bool:
    if not not x:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_none() {
    let code = r#"
def f(x: int):
    if x < 0:
        return
    print(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_augmented_assign_ops() {
    let code = r#"
def f(x: int) -> int:
    result = x
    result += 10
    result -= 3
    result *= 2
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_return_expr() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return (a + b) * c - (a - b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn f"));
}
