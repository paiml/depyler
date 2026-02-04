//! Coverage tests for doctest_extractor.rs
//!
//! DEPYLER-99MODE-001: Targets doctest_extractor.rs (1,101 lines)
//! Covers: doctest extraction from docstrings, multi-line continuation,
//! multiple examples, structured output generation.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Basic doctest patterns
// ============================================================================

#[test]
fn test_doctest_simple_function() {
    let code = r#"
def square(x: int) -> int:
    """Compute square.

    >>> square(4)
    16
    """
    return x * x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doctest_with_return() {
    let code = r#"
def add(a: int, b: int) -> int:
    """Add two numbers.

    >>> add(2, 3)
    5
    """
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doctest_string_function() {
    let code = r#"
def greet(name: str) -> str:
    """Greet someone.

    >>> greet("World")
    'Hello, World!'
    """
    return "Hello, " + name + "!"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple doctests
// ============================================================================

#[test]
fn test_doctest_multiple_examples() {
    let code = r#"
def double(x: int) -> int:
    """Double a number.

    >>> double(0)
    0
    >>> double(5)
    10
    >>> double(-3)
    -6
    """
    return x * 2
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// No doctest patterns
// ============================================================================

#[test]
fn test_doctest_no_docstring() {
    let code = "def f(x: int) -> int:\n    return x + 1\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_doctest_docstring_without_tests() {
    let code = r#"
def f(x: int) -> int:
    """Just a description, no tests."""
    return x + 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Class doctest patterns
// ============================================================================

#[test]
fn test_doctest_class_with_docstring() {
    let code = r#"
class Calculator:
    """A simple calculator.

    >>> c = Calculator()
    """

    def __init__(self):
        self.value = 0

    def add(self, x: int) -> int:
        """Add x to value.

        >>> c = Calculator()
        """
        self.value += x
        return self.value
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Boolean and collection return doctests
// ============================================================================

#[test]
fn test_doctest_bool_return() {
    let code = r#"
def is_even(n: int) -> bool:
    """Check if even.

    >>> is_even(4)
    True
    >>> is_even(3)
    False
    """
    return n % 2 == 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doctest_list_return() {
    let code = r#"
def make_list(n: int) -> list:
    """Make a list.

    >>> make_list(3)
    [0, 1, 2]
    """
    return list(range(n))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple functions with doctests
// ============================================================================

#[test]
fn test_doctest_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    """Add numbers.

    >>> add(1, 2)
    3
    """
    return a + b

def multiply(a: int, b: int) -> int:
    """Multiply numbers.

    >>> multiply(3, 4)
    12
    """
    return a * b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_doctest_empty_function() {
    let code = r#"
def noop():
    """Does nothing.

    >>> noop()
    """
    pass
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_doctest_multiline_docstring() {
    let code = r#"
def process(items: list) -> int:
    """Process a list of items.

    Args:
        items: List of integers

    Returns:
        Sum of all items

    Examples:
        >>> process([1, 2, 3])
        6
    """
    return sum(items)
"#;
    assert!(transpile_ok(code));
}
