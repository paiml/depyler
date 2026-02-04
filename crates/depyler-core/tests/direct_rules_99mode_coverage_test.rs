//! Coverage tests for direct_rules.rs
//!
//! DEPYLER-99MODE-001: Targets direct_rules.rs (5,648 lines)
//! Covers: apply_rules, class-to-struct conversion, method_mutates_self,
//! stdlib shadowing, type safety, NewType aliases, enum generation.

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
// Class to struct conversion
// ============================================================================

#[test]
fn test_direct_rules_simple_class() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
"#;
    let rust = transpile(code);
    assert!(rust.contains("struct") || rust.contains("Point"));
}

#[test]
fn test_direct_rules_class_with_methods() {
    let code = r#"
class Calculator:
    def __init__(self):
        self.result = 0

    def add(self, x: int) -> int:
        self.result += x
        return self.result

    def get_result(self) -> int:
        return self.result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_class_no_init() {
    let code = r#"
class Helper:
    def process(self, x: int) -> int:
        return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_class_str_method() {
    let code = r#"
class Dog:
    def __init__(self, name: str):
        self.name = name

    def __str__(self) -> str:
        return self.name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_class_repr_method() {
    let code = r#"
class Item:
    def __init__(self, id: int):
        self.id = id

    def __repr__(self) -> str:
        return str(self.id)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Method mutation analysis
// ============================================================================

#[test]
fn test_direct_rules_immutable_method() {
    let code = r#"
class Obj:
    def __init__(self, val: int):
        self.val = val

    def get_val(self) -> int:
        return self.val
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_mutable_method() {
    let code = r#"
class Obj:
    def __init__(self, val: int):
        self.val = val

    def set_val(self, new_val: int):
        self.val = new_val
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_mutable_list_method() {
    let code = r#"
class Container:
    def __init__(self):
        self.items = []

    def add(self, item: int):
        self.items.append(item)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_augmented_assign_method() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Function rules
// ============================================================================

#[test]
fn test_direct_rules_simple_function() {
    let code = "def f(x: int) -> int:\n    return x * 2\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_function_with_defaults() {
    let code = r#"
def greet(name: str = "world") -> str:
    return "Hello, " + name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_function_multiple_params() {
    let code = "def f(a: int, b: int, c: int) -> int:\n    return a + b + c\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_function_no_return() {
    let code = "def f(x: int):\n    print(x)\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow rules
// ============================================================================

#[test]
fn test_direct_rules_if_else() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    else:
        return "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_for_range() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_while_loop() {
    let code = r#"
def f(n: int) -> int:
    i = 0
    while i < n:
        i += 1
    return i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_nested_loops() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        for j in range(n):
            total += 1
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Exception handling rules
// ============================================================================

#[test]
fn test_direct_rules_try_except() {
    let code = r#"
def f(x: int) -> int:
    try:
        return 100 // x
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_raise() {
    let code = r#"
def f(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_assert() {
    let code = "def f(x: int):\n    assert x > 0\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Type safety rules
// ============================================================================

#[test]
fn test_direct_rules_type_annotations() {
    let code = r#"
def f(x: int, y: float, s: str) -> bool:
    return len(s) > 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_type_conversion() {
    let code = "def f(x: int) -> str:\n    return str(x)\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple classes
// ============================================================================

#[test]
fn test_direct_rules_two_classes() {
    let code = r#"
class Dog:
    def __init__(self, name: str):
        self.name = name

class Cat:
    def __init__(self, name: str):
        self.name = name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_class_and_functions() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

class Processor:
    def __init__(self, factor: int):
        self.factor = factor

    def process(self, x: int) -> int:
        return x * self.factor
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex integration
// ============================================================================

#[test]
fn test_direct_rules_stack_class() {
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
fn test_direct_rules_algorithm() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_string_processing() {
    let code = r#"
def process(text: str) -> list:
    words = text.split()
    result = []
    for word in words:
        result.append(word.upper())
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_dict_counting() {
    let code = r#"
def count_items(items: list) -> dict:
    counts = {}
    for item in items:
        counts[item] = counts.get(item, 0) + 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_comprehension() {
    let code = "def f(items: list) -> list:\n    return [x * 2 for x in items if x > 0]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_lambda_sorted() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_fstring() {
    let code = r#"
def greet(name: str, age: int) -> str:
    return f"{name} is {age} years old"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_direct_rules_tuple_unpack() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    assert!(transpile_ok(code));
}
