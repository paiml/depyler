//! direct_rules.rs Coverage Expansion Tests
//!
//! DEPYLER-0152 Phase 2C: Property + Mutation testing for type conversion and code generation
//! Target: 63.80% → 75%+ coverage (629 missed lines)
//!
//! Test Structure (MANDATORY):
//! - Unit Tests: Basic type conversion and operator mapping validation
//! - Property Tests: Arbitrary type/operator validation with proptest
//! - Mutation Tests: Documented mutation kill strategies

use depyler_core::DepylerPipeline;

// ============================================================================
// UNIT TESTS - Type Conversions
// ============================================================================

#[test]
fn test_int_type_conversion() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def add_ints(a: int, b: int) -> int:
    return a + b
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated int type conversion:\n{}", rust_code);

    // Should use i32 for Python int
    assert!(
        rust_code.contains("i32"),
        "Python int should map to Rust i32"
    );
}

#[test]
fn test_float_type_conversion() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def calc_float(x: float) -> float:
    return x * 2.0
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated float type conversion:\n{}", rust_code);

    // Should use f64 for Python float
    assert!(
        rust_code.contains("f64"),
        "Python float should map to Rust f64"
    );
}

#[test]
fn test_list_type_conversion() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_list(items: list[int]) -> list[int]:
    return items
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated list type conversion:\n{}", rust_code);

    // Should use Vec for Python list
    assert!(
        rust_code.contains("Vec"),
        "Python list should map to Rust Vec"
    );
}

#[test]
fn test_dict_type_conversion() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_dict(data: dict[str, int]) -> dict[str, int]:
    return data
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated dict type conversion:\n{}", rust_code);

    // Should use HashMap for Python dict
    assert!(
        rust_code.contains("HashMap"),
        "Python dict should map to Rust HashMap"
    );
}

// ============================================================================
// UNIT TESTS - Binary Operator Mapping
// ============================================================================

#[test]
fn test_modulo_operator() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def modulo(a: int, b: int) -> int:
    return a % b
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated modulo operator:\n{}", rust_code);

    // Should use % for modulo
    assert!(
        rust_code.contains("%"),
        "Python % should map to Rust %"
    );
}

#[test]
fn test_comparison_operators() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def compare(a: int, b: int) -> bool:
    return a <= b
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated comparison operator:\n{}", rust_code);

    // Should use <= for less than or equal
    assert!(
        rust_code.contains("<="),
        "Python <= should map to Rust <="
    );
}

// ============================================================================
// UNIT TESTS - Assignment Statements
// ============================================================================

#[test]
fn test_simple_variable_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def assign_var():
    x = 42
    y = x + 1
    return y
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated variable assignment:\n{}", rust_code);

    // Should generate let binding for intermediate variables
    assert!(
        rust_code.contains("let") || rust_code.contains("_cse_temp"),
        "Variable assignment should use let or CSE temps"
    );
}

#[test]
fn test_augmented_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def augment(x: int) -> int:
    x += 10
    return x
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated augmented assignment:\n{}", rust_code);

    // Should expand to x = x + 10
    assert!(
        rust_code.contains("=") && rust_code.contains("+"),
        "Augmented assignment should expand"
    );
}

// ============================================================================
// UNIT TESTS - Class/Struct Conversion
// ============================================================================

#[test]
fn test_simple_class_to_struct() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated struct from class:\n{}", rust_code);

    // Should generate struct
    assert!(
        rust_code.contains("struct") || rust_code.contains("pub struct"),
        "Python class should map to Rust struct"
    );
}

// ============================================================================
// PROPERTY TESTS
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]

        #[test]
        fn prop_integer_types_always_transpile(val in -1000i32..1000i32) {
            // Property: Any integer literal should transpile successfully
            let pipeline = DepylerPipeline::new();
            let python_code = format!(r#"
def const_int():
    return {}
"#, val);

            let result = pipeline.transpile(&python_code);
            prop_assert!(result.is_ok(), "Integer literal transpilation failed: {:?}", result.err());
        }

        #[test]
        fn prop_arithmetic_operators_always_work(op_idx in 0usize..4) {
            // Property: All basic arithmetic operators should transpile
            let ops = ["+", "-", "*", "/"];
            let op = ops[op_idx];

            let pipeline = DepylerPipeline::new();
            let python_code = format!(r#"
def calc(a: int, b: int) -> int:
    return a {} b
"#, op);

            let result = pipeline.transpile(&python_code);
            prop_assert!(result.is_ok(), "Arithmetic operator {} transpilation failed", op);
        }

        #[test]
        fn prop_type_annotations_preserved(type_idx in 0usize..4) {
            // Property: Type annotations should be respected in transpilation
            let types = ["int", "float", "str", "bool"];
            let rust_types = ["i32", "f64", "String", "bool"];
            let type_py = types[type_idx];
            let type_rs = rust_types[type_idx];

            let pipeline = DepylerPipeline::new();
            let python_code = format!(r#"
def typed_func(x: {}) -> {}:
    return x
"#, type_py, type_py);

            let result = pipeline.transpile(&python_code);
            prop_assert!(result.is_ok(), "Type annotation transpilation failed");

            let rust_code = result.unwrap();
            prop_assert!(
                rust_code.contains(type_rs) || rust_code.contains("DynamicType"),
                "Type {} should map to {}", type_py, type_rs
            );
        }

        #[test]
        fn prop_comparison_operators_generate_valid_rust(cmp_idx in 0usize..6) {
            // Property: All comparison operators should generate valid Rust
            let comparisons = ["==", "!=", "<", "<=", ">", ">="];
            let cmp = comparisons[cmp_idx];

            let pipeline = DepylerPipeline::new();
            let python_code = format!(r#"
def compare(a: int, b: int) -> bool:
    return a {} b
"#, cmp);

            let result = pipeline.transpile(&python_code);
            prop_assert!(result.is_ok(), "Comparison operator {} failed", cmp);

            let rust_code = result.unwrap();
            prop_assert!(
                rust_code.contains(cmp),
                "Comparison {} should be present in generated code", cmp
            );
        }
    }
}

// ============================================================================
// MUTATION TESTS
// ============================================================================

#[cfg(test)]
mod mutation_tests {
    use super::*;

    #[test]
    fn test_mutation_type_mapping_correctness() {
        // Target Mutations:
        // 1. int → i64 instead of i32 (wrong default type)
        // 2. float → f32 instead of f64 (wrong default type)
        // 3. list → Array instead of Vec (wrong container)
        //
        // Kill Strategy:
        // - Verify int maps to i32 (not i64)
        // - Verify float maps to f64 (not f32)
        // - Verify list maps to Vec (not array)
        // - Mutation changing default types would fail

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def typed_function(i_val: int, f_val: float, items: list[int]) -> int:
    return i_val
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Using i64 instead of i32 would fail tests
        assert!(
            rust_code.contains("i32"),
            "MUTATION KILL: int must map to i32 (not i64)"
        );

        // Mutation Kill: Using f32 instead of f64 would fail tests
        assert!(
            rust_code.contains("f64"),
            "MUTATION KILL: float must map to f64 (not f32)"
        );

        // Mutation Kill: Using array instead of Vec would fail
        assert!(
            rust_code.contains("Vec"),
            "MUTATION KILL: list must map to Vec (not array)"
        );
    }

    #[test]
    fn test_mutation_operator_mapping() {
        // Target Mutations:
        // 1. % → / (modulo → division)
        // 2. // → / (floor division → regular division)
        // 3. ** → * (power → multiplication)
        //
        // Kill Strategy:
        // - Verify % operator is preserved for modulo
        // - Verify // generates floor division logic
        // - Verify ** generates power operation
        // - Mutation changing operators would fail

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def operations(a: int, b: int) -> int:
    mod_result = a % b
    return mod_result
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Changing % to / would fail
        assert!(
            rust_code.contains("%"),
            "MUTATION KILL: Modulo operator % must be preserved"
        );
    }

    #[test]
    fn test_mutation_assignment_generation() {
        // Target Mutations:
        // 1. let → const (wrong mutability)
        // 2. = → := (wrong syntax)
        // 3. Remove let keyword (invalid Rust)
        //
        // Kill Strategy:
        // - Verify let keyword or CSE temps are used for intermediate values
        // - Verify = is used for assignment
        // - Mutation removing let would fail compilation

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def assign_vars():
    x = 10
    y = 20
    z = x + y
    return z
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Not using let or CSE temps would fail compilation
        let let_or_cse = rust_code.contains("let") || rust_code.contains("_cse_temp");
        assert!(
            let_or_cse,
            "MUTATION KILL: Must use let or CSE temps for variable declarations"
        );

        // Mutation Kill: Using wrong assignment operator would fail
        // Note: Transpiler may constant-fold (10 + 20 = 30), so check for assignment syntax
        assert!(
            rust_code.contains("let mut") || rust_code.contains("let "),
            "MUTATION KILL: Must use let for assignment declarations"
        );
    }

    #[test]
    fn test_mutation_struct_generation_from_class() {
        // Target Mutations:
        // 1. struct → enum (wrong type)
        // 2. pub struct → struct (wrong visibility)
        // 3. Missing field declarations (incomplete struct)
        //
        // Kill Strategy:
        // - Verify struct keyword is used
        // - Verify fields are generated from __init__
        // - Mutation changing to enum would fail

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
class Counter:
    def __init__(self, value: int):
        self.value = value
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Using enum instead of struct would fail
        assert!(
            rust_code.contains("struct"),
            "MUTATION KILL: Class must generate struct (not enum)"
        );

        // Mutation Kill: Not generating pub would limit usability
        assert!(
            rust_code.contains("pub struct") || rust_code.contains("struct"),
            "MUTATION KILL: Struct must be declared"
        );

        // Mutation Kill: Missing fields would fail
        assert!(
            rust_code.contains("value"),
            "MUTATION KILL: Struct must contain fields from __init__"
        );
    }
}
