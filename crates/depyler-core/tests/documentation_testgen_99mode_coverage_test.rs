//! Coverage tests for documentation.rs and test_generation.rs
//!
//! DEPYLER-99MODE-001: Targets documentation.rs (1,392 lines) + test_generation.rs (1,290 lines)
//! Covers: docstring extraction, documentation generation,
//! test case generation, property test generation.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Documentation - docstring extraction
// ============================================================================

#[test]
fn test_doc_simple_docstring() {
    let code = r#"
def f() -> int:
    """Return 42."""
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_multiline_docstring() {
    let code = r#"
def process(items: list) -> int:
    """Process a list of items.

    Args:
        items: List of integers

    Returns:
        Sum of all items
    """
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_class_docstring() {
    let code = r#"
class Calculator:
    """A simple calculator class."""

    def __init__(self):
        self.result = 0

    def add(self, x: int) -> int:
        """Add x to result."""
        self.result += x
        return self.result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_no_docstring() {
    let code = "def f(x: int) -> int:\n    return x * 2\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Documentation - various function patterns
// ============================================================================

#[test]
fn test_doc_typed_function() {
    let code = r#"
def add(a: int, b: int) -> int:
    """Add two integers."""
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_default_params() {
    let code = r#"
def greet(name: str = "world") -> str:
    """Return greeting."""
    return "Hello, " + name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doc_complex_function() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    """Find target in sorted list. Returns index or -1."""
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

// ============================================================================
// Test generation - function patterns
// ============================================================================

#[test]
fn test_testgen_simple_function() {
    let code = "def double(x: int) -> int:\n    return x * 2\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_boolean_function() {
    let code = "def is_positive(x: int) -> bool:\n    return x > 0\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_string_function() {
    let code = r#"
def greet(name: str) -> str:
    return "Hello, " + name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_list_function() {
    let code = r#"
def double_list(items: list) -> list:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_multiple_params() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    if x < lo:
        return lo
    if x > hi:
        return hi
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_class_method() {
    let code = r#"
class Counter:
    def __init__(self, start: int):
        self.value = start

    def increment(self) -> int:
        self.value += 1
        return self.value
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Combined patterns
// ============================================================================

#[test]
fn test_doctest_documented_class() {
    let code = r#"
class Stack:
    """A simple stack implementation."""

    def __init__(self):
        """Initialize empty stack."""
        self.items = []

    def push(self, item: int):
        """Push item onto stack."""
        self.items.append(item)

    def pop(self) -> int:
        """Pop and return top item."""
        return self.items.pop()

    def is_empty(self) -> bool:
        """Check if stack is empty."""
        return len(self.items) == 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doctest_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    """Add two numbers."""
    return a + b

def multiply(a: int, b: int) -> int:
    """Multiply two numbers."""
    return a * b

def power(base: int, exp: int) -> int:
    """Raise base to the power of exp."""
    result = 1
    for i in range(exp):
        result *= base
    return result
"#;
    assert!(transpile_ok(code));
}
