//! Coverage tests for documentation.rs
//!
//! DEPYLER-99MODE-001: Targets documentation.rs (1,392 lines)
//! Covers: doc generation, function signatures, type formatting,
//! API reference, usage guides, migration notes, class docs.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Function documentation
// ============================================================================

#[test]
fn test_doc_simple_function() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_function_with_docstring() {
    let code = r#"
def calculate(x: int) -> int:
    """Calculate the result."""
    return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_function_no_params() {
    let code = r#"
def get_value() -> int:
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_function_no_return() {
    let code = r#"
def do_something(x: int):
    y = x + 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type formatting
// ============================================================================

#[test]
fn test_doc_type_int() {
    let code = r#"
def f(x: int) -> int:
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_type_float() {
    let code = r#"
def f(x: float) -> float:
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_type_bool() {
    let code = r#"
def f(x: bool) -> bool:
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_type_str() {
    let code = r#"
def f(x: str) -> str:
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_type_list() {
    let code = r#"
def f(items: list) -> list:
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_type_dict() {
    let code = r#"
def f(d: dict) -> dict:
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_type_set() {
    let code = r#"
def f(s: set) -> set:
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_type_tuple() {
    let code = r#"
def f() -> tuple:
    return (1, 2, 3)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Class documentation
// ============================================================================

#[test]
fn test_doc_simple_class() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_class_with_methods() {
    let code = r#"
class Calculator:
    def __init__(self):
        self.result = 0

    def add(self, x: int) -> int:
        self.result += x
        return self.result

    def reset(self):
        self.result = 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_class_with_docstring() {
    let code = r#"
class DataProcessor:
    """Process data efficiently."""
    def __init__(self):
        self.data = []

    def process(self) -> list:
        """Process the stored data."""
        return self.data
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple functions
// ============================================================================

#[test]
fn test_doc_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def subtract(a: int, b: int) -> int:
    return a - b

def multiply(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Migration note patterns
// ============================================================================

#[test]
fn test_doc_list_parameter_migration() {
    let code = r#"
def process(items: list) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_optional_return_migration() {
    let code = r#"
def find(items: list, target: int) -> int:
    for item in items:
        if item == target:
            return item
    return -1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex documentation patterns
// ============================================================================

#[test]
fn test_doc_comprehensive_module() {
    let code = r#"
def add(a: int, b: int) -> int:
    """Add two numbers."""
    return a + b

def process_list(items: list) -> list:
    """Process a list of items."""
    return [x * 2 for x in items]

class Counter:
    """A simple counter class."""
    def __init__(self):
        self.count = 0

    def increment(self) -> int:
        """Increment and return count."""
        self.count += 1
        return self.count

    def get_count(self) -> int:
        """Get current count."""
        return self.count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_algorithm_function() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    """Search for target in sorted list."""
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
