//! DEPYLER-0962: Pareto-Complete Type Inference Tests
//!
//! EXTREME TDD: These tests MUST FAIL initially.
//! They define the target behavior for Phase 1-3 of the Pareto spec.
//!
//! Phase 1: Type fallback + import tracking (target: 40% convergence)
//! Phase 2: Bidirectional propagation + call-site (target: 65% convergence)
//! Phase 3: Constraint solving (target: 80% convergence)

use depyler_core::hir::Type;
use depyler_core::DepylerPipeline;

/// Helper to transpile Python to Rust
fn transpile(python: &str) -> Result<String, String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python).map_err(|e| e.to_string())
}

/// Helper to check if transpiled code parses as valid Rust
fn compiles(python: &str) -> bool {
    match transpile(python) {
        Ok(rust_code) => syn::parse_file(&rust_code).is_ok(),
        Err(_) => false,
    }
}

// ============================================================================
// PHASE 1: Type Fallback Tests
// ============================================================================

#[test]
fn test_phase1_unknown_type_does_not_become_serde_value() {
    // FAILING TEST: Currently, unknown types fall back to serde_json::Value
    // Target: Unknown types should use proper Rust types or explicit Any
    let python = r#"
def process(data):
    """Process data without type hints."""
    return data.get("key")
"#;
    let rust = transpile(python).unwrap();

    // Should NOT contain serde_json::Value for simple dict access
    assert!(
        !rust.contains("serde_json::Value"),
        "Unknown types should not default to serde_json::Value.\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_phase1_dict_access_infers_hashmap() {
    // FAILING TEST: Dict access should infer HashMap type
    let python = r#"
def get_value(d):
    """Get value from dict."""
    return d.get("key", "default")
"#;
    let rust = transpile(python).unwrap();

    // Should infer HashMap<String, String> from usage
    assert!(
        rust.contains("HashMap") || rust.contains("std::collections::HashMap"),
        "Dict access should infer HashMap type.\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_phase1_list_append_infers_vec() {
    // FAILING TEST: List operations should infer Vec type
    let python = r#"
def collect(items):
    """Collect items into a list."""
    result = []
    for item in items:
        result.append(item)
    return result
"#;
    let rust = transpile(python).unwrap();

    // Should infer Vec<T> from append usage
    assert!(
        rust.contains("Vec<") || rust.contains("vec!"),
        "List operations should infer Vec type.\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_phase1_boolean_context_coercion() {
    // FAILING TEST: Values in boolean context should be properly coerced
    let python = r#"
def check_buffer(buffer: list) -> bool:
    """Check if buffer is non-empty."""
    if buffer:
        return True
    return False
"#;
    let rust = transpile(python).unwrap();

    // Should NOT use raw collection in if condition
    // Should use .is_empty() or len() > 0
    assert!(
        rust.contains("is_empty()")
            || rust.contains(".len()")
            || rust.contains("!buffer.is_empty()"),
        "Boolean context should use proper emptiness check.\nGenerated:\n{}",
        rust
    );
}

// ============================================================================
// PHASE 2: Bidirectional Type Propagation Tests
// ============================================================================

#[test]
fn test_phase2_return_type_propagates_backward() {
    // FAILING TEST: Return type should propagate backward to infer variable types
    let python = r#"
def compute() -> int:
    """Compute value."""
    return 42

def get_count() -> int:
    """Get count."""
    x = compute()
    return x
"#;
    let rust = transpile(python).unwrap();

    // x should be typed, code should compile
    assert!(
        compiles(python),
        "Return type should propagate backward.\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_phase2_parameter_type_propagates_to_body() {
    // FAILING TEST: Parameter types should propagate through function body
    let python = r#"
def double(x: int) -> int:
    """Double a number."""
    result = x * 2
    return result
"#;
    let rust = transpile(python).unwrap();

    // result should be typed as i32/i64
    assert!(
        compiles(python),
        "Parameter types should propagate to body.\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_phase2_chained_method_type_flow() {
    // FAILING TEST: Types should flow through method chains
    let python = r#"
def process(text: str) -> str:
    """Process text."""
    result = text.strip().lower().replace("a", "b")
    return result
"#;
    let rust = transpile(python).unwrap();

    // Each method in chain should preserve String type
    assert!(
        compiles(python),
        "Method chain should preserve types.\nGenerated:\n{}",
        rust
    );
}

// ============================================================================
// PHASE 3: Type Unification Tests
// ============================================================================

#[test]
fn test_phase3_optional_type_unification() {
    // FAILING TEST: Optional types should unify correctly
    let python = r#"
from typing import Optional

def find(items: list, key: str) -> Optional[str]:
    """Find item by key."""
    for item in items:
        if item == key:
            return item
    return None
"#;
    let rust = transpile(python).unwrap();

    // Should generate Option<String> return type
    assert!(
        rust.contains("Option<String>") || rust.contains("Option<&str>"),
        "Optional should unify to Option<T>.\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_phase3_dict_comprehension_type_inference() {
    // FAILING TEST: Dict comprehension should infer key/value types
    let python = r#"
def invert(d: dict) -> dict:
    """Invert dictionary."""
    return {v: k for k, v in d.items()}
"#;
    let rust = transpile(python).unwrap();

    // Should infer HashMap with swapped types
    assert!(
        rust.contains("HashMap"),
        "Dict comprehension should infer types.\nGenerated:\n{}",
        rust
    );
}

// ============================================================================
// Integration Tests: Real-World Patterns
// ============================================================================

#[test]
fn test_integration_simple_arithmetic() {
    // Test basic arithmetic compiles
    let python = r#"
def add(a: int, b: int) -> int:
    """Add two numbers."""
    return a + b
"#;

    assert!(compiles(python), "Simple arithmetic should compile");
}

#[test]
fn test_integration_string_operations() {
    // Test string operations compile
    let python = r#"
def greet(name: str) -> str:
    """Greet a person."""
    return "Hello, " + name + "!"
"#;

    assert!(compiles(python), "String operations should compile");
}

#[test]
fn test_integration_list_operations() {
    // Test list operations compile
    let python = r#"
def sum_list(nums: list[int]) -> int:
    """Sum a list of numbers."""
    total = 0
    for n in nums:
        total = total + n
    return total
"#;

    assert!(compiles(python), "List operations should compile");
}

// ============================================================================
// Type System Unit Tests
// ============================================================================

#[test]
fn test_type_enum_has_expected_variants() {
    // Test Type enum has expected variants
    let unknown = Type::Unknown;
    let int_type = Type::Int;
    let float_type = Type::Float;
    let string_type = Type::String;

    assert!(matches!(unknown, Type::Unknown));
    assert!(matches!(int_type, Type::Int));
    assert!(float_type.is_numeric());
    assert!(!string_type.is_numeric());
}

#[test]
fn test_type_is_container() {
    // Test container type detection
    let list_type = Type::List(Box::new(Type::Int));
    let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
    let int_type = Type::Int;

    assert!(list_type.is_container());
    assert!(dict_type.is_container());
    assert!(!int_type.is_container());
}

#[test]
fn test_type_unification_basic() {
    // Test basic type unification concepts
    let int_type = Type::Int;
    let float_type = Type::Float;

    // Numeric types should have common trait
    assert!(int_type.is_numeric());
    assert!(float_type.is_numeric());
}
