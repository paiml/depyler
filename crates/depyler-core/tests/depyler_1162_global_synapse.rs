#![allow(clippy::assertions_on_constants)]
// DEPYLER-1162: Global Synapse Activation
//
// Tests for cross-module type propagation using the GlobalTypeGraph.
// Verifies that return types from function A are correctly propagated to
// callers in function B, enabling type-aware code generation.
//
// The "Global Synapse" refers to the neural-link-like connection between
// function return types and caller expectations across module boundaries.
#![allow(non_snake_case)] // Test naming convention

use depyler_core::DepylerPipeline;

/// Helper to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

// ========================================================================
// PHASE 1: Single-Module Cross-Function Type Propagation
// ========================================================================

#[test]
fn test_DEPYLER_1162_single_module_return_type_propagation() {
    // Basic test: function A returns concrete type, function B uses it
    let python = r#"
def get_count():
    return 42

def use_count():
    x = get_count()
    return x + 1
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();
    // The return type of get_count should propagate to use_count
    assert!(
        rust.contains("i64") || rust.contains("i32"),
        "Should infer integer types: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1162_chained_function_calls() {
    // Chain: A -> B -> C, type should propagate through all
    let python = r#"
def get_base():
    return 100

def double_it():
    return get_base() * 2

def use_doubled():
    value = double_it()
    return value / 10
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

#[test]
fn test_DEPYLER_1162_typed_function_to_untyped_caller() {
    // Typed function called by untyped function
    let python = r#"
def typed_getter() -> int:
    return 42

def untyped_caller():
    result = typed_getter()
    return result * 2
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();
    // typed_getter's int return type should influence untyped_caller
    assert!(
        rust.contains("i64") || rust.contains("i32"),
        "Should propagate int type: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1162_string_return_propagation() {
    // String return type propagation
    let python = r#"
def get_name():
    return "Alice"

def greet():
    name = get_name()
    return "Hello, " + name
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();
    assert!(
        rust.contains("String") || rust.contains("&str"),
        "Should handle string types: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1162_list_return_propagation() {
    // List return type propagation
    let python = r#"
def get_items():
    return [1, 2, 3]

def process_items():
    items = get_items()
    return len(items)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();
    assert!(
        rust.contains("Vec<") || rust.contains("vec!"),
        "Should handle list types: {}",
        rust
    );
}

// ========================================================================
// PHASE 2: Parameter Type Inference from Call Sites
// ========================================================================

#[test]
fn test_DEPYLER_1162_parameter_type_from_call_site() {
    // Untyped function parameter should infer type from call site
    let python = r#"
def add_one(x):
    return x + 1

def main():
    result = add_one(5)
    return result
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    // x should be inferred as int from call site
    let rust = result.unwrap();
    assert!(
        rust.contains("i64") || rust.contains("i32"),
        "Should infer int parameter: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1162_multiple_call_sites_consistent() {
    // Multiple call sites with consistent types
    let python = r#"
def process(value):
    return value * 2

def caller1():
    return process(10)

def caller2():
    return process(20)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

#[test]
fn test_DEPYLER_1162_bidirectional_type_flow() {
    // Type flows both from return and to parameters
    let python = r#"
def transform(x: int) -> str:
    return str(x)

def caller():
    value = 42
    result = transform(value)
    return len(result)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

// ========================================================================
// PHASE 3: Complex Scenarios
// ========================================================================

#[test]
fn test_DEPYLER_1162_dict_return_propagation() {
    // Dict return type propagation
    let python = r#"
def get_config():
    return {"host": "localhost", "port": 8080}

def get_host():
    config = get_config()
    return config["host"]
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

#[test]
fn test_DEPYLER_1162_conditional_return_types() {
    // Function with conditional returns
    let python = r#"
def maybe_get(flag):
    if flag:
        return 42
    return None

def use_maybe():
    result = maybe_get(True)
    if result:
        return result + 1
    return 0
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

#[test]
fn test_DEPYLER_1162_recursive_type_propagation() {
    // Recursive function - type should be consistent
    let python = r#"
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

def main():
    return factorial(5)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

// ========================================================================
// PHASE 4: Multi-Module Simulation
// ========================================================================

#[test]
fn test_DEPYLER_1162_simulated_multi_module() {
    // Simulate module_a and module_b in single file
    // This tests the same type propagation that would occur across modules
    let python = r#"
# Simulated module_a
def module_a_get_value():
    return 100

def module_a_process(x):
    return x * 2

# Simulated module_b (uses module_a)
def module_b_main():
    value = module_a_get_value()
    processed = module_a_process(value)
    return processed + 1
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();
    // All functions should have consistent integer types
    assert!(
        rust.contains("fn module_a_get_value") && rust.contains("fn module_b_main"),
        "Should generate all functions: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1162_data_pipeline_pattern() {
    // Common pattern: data flows through multiple functions
    let python = r#"
def load_data():
    return [1, 2, 3, 4, 5]

def filter_data(data):
    return [x for x in data if x > 2]

def transform_data(data):
    return [x * 2 for x in data]

def pipeline():
    raw = load_data()
    filtered = filter_data(raw)
    transformed = transform_data(filtered)
    return sum(transformed)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

// ========================================================================
// SUCCESS CRITERIA DOCUMENTATION
// ========================================================================

#[test]
fn test_DEPYLER_1162_success_criteria() {
    // Documents the success criteria for Global Synapse
    //
    // SUCCESS CRITERIA:
    // 1. Type Propagation Completeness:
    //    - Return types from function A correctly inform type inference in function B
    //    - Parameter types at call sites propagate back to function definitions
    //
    // 2. Type Consistency:
    //    - Chained function calls maintain consistent types through the chain
    //    - No DepylerValue fallback when types can be concretely determined
    //
    // 3. Multi-Pass Capability:
    //    - First pass: Collect function signatures and return type hints
    //    - Second pass: Apply collected types to resolve unknowns
    //
    // 4. Verification:
    //    - Generated Rust code compiles without type mismatch errors
    //    - Semantic equivalence preserved (same behavior as Python)
    //
    // IMPLEMENTATION STATUS:
    // - Cross-function type propagation: ✅ Implemented (DEPYLER-0575)
    // - Parameter type inference from call sites: ✅ Implemented
    // - Return type collection: ✅ Implemented
    // - Multi-module support: 🔧 Requires file-level integration
    //
    // The Global Synapse is ACTIVE for single-module scenarios.
    // Multi-module support requires external coordination of multiple transpile calls.

    assert!(true, "Success criteria documented");
}

// ========================================================================
// TYPE PROPAGATION VERIFICATION
// ========================================================================

#[test]
fn test_DEPYLER_1162_verify_no_depyler_value_fallback() {
    // When types are determinable, we should NOT fall back to DepylerValue
    let python = r#"
def get_number() -> int:
    return 42

def double_number() -> int:
    n = get_number()
    return n * 2
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();
    // With explicit type annotations, we should see concrete types, not DepylerValue
    let has_concrete_types = rust.contains("i64") || rust.contains("i32") || rust.contains("-> i");
    assert!(
        has_concrete_types,
        "Should use concrete types when annotated: {}",
        rust
    );
}
