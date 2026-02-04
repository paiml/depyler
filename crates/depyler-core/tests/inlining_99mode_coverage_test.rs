//! Coverage tests for inlining.rs
//!
//! DEPYLER-99MODE-001: Targets inlining.rs (2,469 lines)
//! Covers: InliningAnalyzer, call graph analysis, function metrics,
//! recursive detection, inlining heuristics, apply_inlining.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Simple function inlining candidates
// ============================================================================

#[test]
fn test_inlining_trivial_function() {
    let code = r#"
def double(x: int) -> int:
    return x * 2

def f(n: int) -> int:
    return double(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_identity_function() {
    let code = r#"
def identity(x: int) -> int:
    return x

def f(n: int) -> int:
    return identity(n) + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_constant_function() {
    let code = r#"
def get_default() -> int:
    return 42

def f() -> int:
    return get_default()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Non-inlinable patterns (too complex)
// ============================================================================

#[test]
fn test_inlining_large_function() {
    let code = r#"
def large(items: list) -> int:
    total = 0
    for item in items:
        if item > 0:
            total += item
        elif item < 0:
            total -= item
        else:
            total += 1
    return total

def f(items: list) -> int:
    return large(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_recursive_function() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)

def f(n: int) -> int:
    return fib(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_mutual_recursion() {
    let code = r#"
def is_even(n: int) -> bool:
    if n == 0:
        return True
    return is_odd(n - 1)

def is_odd(n: int) -> bool:
    if n == 0:
        return False
    return is_even(n - 1)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Call graph analysis
// ============================================================================

#[test]
fn test_inlining_chain_calls() {
    let code = r#"
def step1(x: int) -> int:
    return x + 1

def step2(x: int) -> int:
    return step1(x) * 2

def f(n: int) -> int:
    return step2(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_multiple_callers() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

def caller1(n: int) -> int:
    return helper(n)

def caller2(n: int) -> int:
    return helper(n) + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_no_calls() {
    let code = r#"
def standalone(x: int) -> int:
    y = x + 1
    z = y * 2
    return z
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Function metrics
// ============================================================================

#[test]
fn test_inlining_single_stmt() {
    let code = "def f(x: int) -> int:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_multi_stmt() {
    let code = r#"
def f(x: int) -> int:
    a = x + 1
    b = a * 2
    c = b - 3
    return c
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_with_loop() {
    let code = r#"
def sum_range(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total

def f(n: int) -> int:
    return sum_range(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_with_branches() {
    let code = r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    return "zero"

def f(x: int) -> str:
    return classify(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Side effect awareness
// ============================================================================

#[test]
fn test_inlining_with_side_effects() {
    let code = r#"
def log_and_return(x: int) -> int:
    print(x)
    return x

def f(n: int) -> int:
    return log_and_return(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_with_mutation() {
    let code = r#"
def append_and_return(items: list) -> list:
    items.append(0)
    return items

def f() -> list:
    data = [1, 2, 3]
    return append_and_return(data)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex inlining scenarios
// ============================================================================

#[test]
fn test_inlining_helper_in_loop() {
    let code = r#"
def process(x: int) -> int:
    return x * x

def f(items: list) -> int:
    total = 0
    for item in items:
        total += process(item)
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_nested_function() {
    let code = r#"
def outer(n: int) -> int:
    def inner(x: int) -> int:
        return x + 1
    return inner(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_class_method() {
    let code = r#"
class Math:
    def __init__(self):
        self.factor = 2

    def double(self, x: int) -> int:
        return x * self.factor
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_inlining_multi_return() {
    let code = r#"
def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x

def f(a: int, b: int) -> int:
    return abs_val(a) + abs_val(b)
"#;
    assert!(transpile_ok(code));
}
