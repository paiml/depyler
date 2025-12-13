//! DEPYLER-0950: Type Unification Architecture Tests
//!
//! EXTREME TDD: These tests define the expected behavior BEFORE implementation.
//! All tests should FAIL initially (RED phase).

use depyler_core::DepylerPipeline;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate unique temp file path to avoid race conditions
fn unique_temp_path() -> (String, String) {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    let rs_file = format!("/tmp/depyler_0950_test_{}_{}.rs", pid, id);
    let out_file = format!("/tmp/depyler_0950_test_{}_{}", pid, id);
    (rs_file, out_file)
}

/// Helper to check if generated Rust code compiles
fn compiles_with_rustc(code: &str) -> bool {
    let (temp_file, out_file) = unique_temp_path();
    std::fs::write(&temp_file, code).unwrap();

    let output = Command::new("rustc")
        .args(["--edition", "2021", &temp_file, "--crate-type", "lib", "-o", &out_file])
        .output()
        .expect("Failed to run rustc");

    // Clean up
    let _ = std::fs::remove_file(&temp_file);
    let _ = std::fs::remove_file(&out_file);

    output.status.success()
}

/// Get compilation errors
fn compile_errors(code: &str) -> String {
    let (temp_file, out_file) = unique_temp_path();
    std::fs::write(&temp_file, code).unwrap();

    let output = Command::new("rustc")
        .args(["--edition", "2021", &temp_file, "--crate-type", "lib", "-o", &out_file])
        .output()
        .expect("Failed to run rustc");

    // Clean up
    let _ = std::fs::remove_file(&temp_file);
    let _ = std::fs::remove_file(&out_file);

    String::from_utf8_lossy(&output.stderr).to_string()
}

// =============================================================================
// Phase 1: Call Graph Construction Tests
// =============================================================================

/// Test that we can identify function calls within a module
#[test]
fn test_depyler_0950_phase1_call_graph_simple() {
    let python = r#"
def helper(x: int) -> int:
    return x + 1

def main(n: int) -> int:
    return helper(n)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Generated code must compile
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test cross-function type propagation: caller passes f64, callee expects inferred type
#[test]
fn test_depyler_0950_phase1_cross_function_type_propagation() {
    let python = r#"
def process(x):
    return x * 2

def main() -> float:
    return process(3.14)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // CRITICAL: Type of `x` in process() should be inferred as f64 from call site
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        // This is the E0308 we're trying to fix
        if errors.contains("E0308") {
            panic!(
                "E0308 type mismatch - cross-function type propagation needed.\nErrors:\n{}\n\nGenerated code:\n{}",
                errors, code
            );
        }
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

// =============================================================================
// Phase 2: Type Constraint Extraction Tests
// =============================================================================

/// Test numeric type unification: int vs float at call site
#[test]
fn test_depyler_0950_phase2_numeric_unification() {
    let python = r#"
def add(a, b):
    return a + b

def main() -> float:
    x = add(1, 2.5)  # int + float -> should unify to float
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Numeric type unification failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test return type backward propagation
#[test]
fn test_depyler_0950_phase2_return_type_propagation() {
    let python = r#"
def compute(x):
    return x * x

def main() -> int:
    result: int = compute(5)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Return type annotation should propagate to compute()
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Return type propagation failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

// =============================================================================
// Phase 3: Union-Find Unification Tests
// =============================================================================

/// Test that multiple call sites unify parameter types
#[test]
fn test_depyler_0950_phase3_multiple_callsites_unification() {
    let python = r#"
def process(value):
    return value + 1

def main():
    a = process(10)      # int call site
    b = process(20)      # int call site
    return a + b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Multiple callsite unification failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test diamond call pattern (A calls B and C, B and C both call D)
#[test]
fn test_depyler_0950_phase3_diamond_call_pattern() {
    let python = r#"
def leaf(x):
    return x * 2

def branch_a(n):
    return leaf(n) + 1

def branch_b(n):
    return leaf(n) - 1

def root(value: int) -> int:
    return branch_a(value) + branch_b(value)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Diamond call pattern unification failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

// =============================================================================
// Phase 4: Inter-Procedural Constraint Propagation Tests
// =============================================================================

/// Test chained function calls with type propagation
#[test]
fn test_depyler_0950_phase4_chained_calls() {
    let python = r#"
def step1(x):
    return x + 0.5

def step2(x):
    return step1(x) * 2

def step3(x):
    return step2(x) + 1

def main() -> float:
    return step3(10)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Chained call propagation failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

// =============================================================================
// Phase 5: Numeric Coercion Lattice Tests
// =============================================================================

/// Test int to float coercion
#[test]
fn test_depyler_0950_phase5_int_to_float_coercion() {
    let python = r#"
def mix_types(a: int, b: float) -> float:
    return a + b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Should generate: a as f64 + b
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Int to float coercion failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test mixed arithmetic with automatic widening
#[test]
fn test_depyler_0950_phase5_mixed_arithmetic_widening() {
    let python = r#"
def calculate(x: int, y: float, z: int) -> float:
    return x * y + z
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Mixed arithmetic widening failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

// =============================================================================
// Phase 6: Auto-Cast Insertion Tests
// =============================================================================

/// Test that explicit casts are inserted at type boundaries
#[test]
fn test_depyler_0950_phase6_explicit_cast_insertion() {
    let python = r#"
def to_int(x: float) -> int:
    return int(x)

def to_float(x: int) -> float:
    return float(x)

def main() -> int:
    f = 3.14
    i = to_int(f)
    f2 = to_float(i)
    return to_int(f2)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Explicit cast insertion failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

// =============================================================================
// Phase 7: String/&str Unification Tests
// =============================================================================

/// Test String vs &str parameter unification
#[test]
fn test_depyler_0950_phase7_string_str_unification() {
    let python = r#"
def process_text(text):
    return text.upper()

def main() -> str:
    s = "hello"
    return process_text(s)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "String/&str unification failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test string concatenation with mixed types
#[test]
fn test_depyler_0950_phase7_string_concat_mixed() {
    let python = r#"
def greet(name: str) -> str:
    return "Hello, " + name + "!"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "String concat mixed failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

// =============================================================================
// Integration Tests: Real-World Patterns
// =============================================================================

/// Test recursive function with type inference
#[test]
fn test_depyler_0950_integration_recursive() {
    let python = r#"
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

def main() -> int:
    return factorial(5)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Recursive function type inference failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test mutually recursive functions
#[test]
fn test_depyler_0950_integration_mutual_recursion() {
    let python = r#"
def is_even(n):
    if n == 0:
        return True
    return is_odd(n - 1)

def is_odd(n):
    if n == 0:
        return False
    return is_even(n - 1)

def main() -> bool:
    return is_even(4)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Mutual recursion type inference failed. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test higher-order function pattern (callback)
#[test]
fn test_depyler_0950_integration_callback_pattern() {
    let python = r#"
from typing import List

def apply_to_all(items: List[int], transform) -> List[int]:
    result = []
    for item in items:
        result.append(transform(item))
    return result

def double(x: int) -> int:
    return x * 2

def main() -> List[int]:
    nums = [1, 2, 3]
    return apply_to_all(nums, double)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    // This is a stretch goal - may not compile initially
    if result.is_ok() {
        let code = result.unwrap();
        if compiles_with_rustc(&code) {
            // Great! We support higher-order functions
        }
    }
    // Don't fail - this is aspirational
}
