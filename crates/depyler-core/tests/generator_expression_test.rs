//! TDD Tests for Generator Expressions (DEPYLER-TBD v3.13.0)
//!
//! Generator expressions: lazy iterators with comprehension syntax
//! Python: `(x * 2 for x in range(5))` → Rust: `(0..5).map(|x| x * 2)`
//!
//! Test Coverage (20 tests):
//! Phase 1: Basic generator expressions (10 tests)
//! Phase 2: Nested generator expressions (5 tests)
//! Phase 3: Edge cases (5 tests)

use depyler_core::DepylerPipeline;

// ============================================================================
// Phase 1: Basic Generator Expressions (10 tests)
// ============================================================================

#[test]
fn test_simple_generator_expression() {
    let python = r#"
def use_gen() -> list:
    gen = (x for x in range(5))
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have simple iterator chain
    assert!(
        rust_code.contains(".into_iter()") || rust_code.contains("0..5"),
        "Should have iterator.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_with_transform() {
    let python = r#"
def use_gen() -> list:
    gen = (x * 2 for x in range(5))
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have map transformation
    assert!(
        rust_code.contains(".map("),
        "Should have .map() transformation.\\nGot:\\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("* 2") || rust_code.contains("*2"),
        "Should have multiplication.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_with_filter() {
    let python = r#"
def use_gen() -> list:
    gen = (x for x in range(10) if x % 2 == 0)
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have filter
    assert!(
        rust_code.contains(".filter("),
        "Should have .filter().\\nGot:\\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("% 2") && rust_code.contains("== 0"),
        "Should have modulo check.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_map_and_filter() {
    let python = r#"
def use_gen() -> list:
    gen = (x * 2 for x in range(10) if x > 5)
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have both filter and map
    assert!(
        rust_code.contains(".filter(") && rust_code.contains(".map("),
        "Should have both .filter() and .map().\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_in_sum() {
    let python = r#"
def calculate() -> int:
    return sum(x**2 for x in range(5))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have sum without intermediate collection
    assert!(
        rust_code.contains(".sum()"),
        "Should have .sum().\\nGot:\\n{}",
        rust_code
    );

    // Should NOT collect to Vec first
    assert!(
        !rust_code.contains("Vec::new()") || rust_code.matches("Vec").count() <= 1,
        "Should not create intermediate Vec.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_in_max() {
    let python = r#"
def find_max(nums: list) -> int:
    return max(x * 2 for x in nums)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have max without intermediate collection
    assert!(
        rust_code.contains(".max()"),
        "Should have .max().\\nGot:\\n{}",
        rust_code
    );

    assert!(
        rust_code.contains(".map("),
        "Should have .map() for transformation.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_with_list_source() {
    let python = r#"
def use_gen(nums: list) -> list:
    gen = (x + 1 for x in nums)
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should iterate over input list
    assert!(
        rust_code.contains(".iter()") || rust_code.contains(".into_iter()"),
        "Should have iterator over list.\\nGot:\\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("+ 1") || rust_code.contains("+1"),
        "Should have +1 operation.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_string_transform() {
    let python = r#"
def use_gen(words: list) -> list:
    gen = (w.upper() for w in words)
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have string method call
    assert!(
        rust_code.contains(".to_uppercase()") || rust_code.contains("upper"),
        "Should have uppercase operation.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_tuple_result() {
    let python = r#"
def use_gen() -> list:
    gen = ((x, x*2) for x in range(3))
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should generate tuple results
    assert!(
        rust_code.contains("(x") && rust_code.contains("x *"),
        "Should create tuples.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_immediate_consume() {
    let python = r#"
def calculate() -> int:
    # Generator expression consumed without assignment
    return sum(x for x in range(100))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should NOT create intermediate variable
    assert!(
        rust_code.contains(".sum()"),
        "Should have .sum().\\nGot:\\n{}",
        rust_code
    );

    // Should be single expression
    let sum_count = rust_code.matches(".sum()").count();
    assert_eq!(sum_count, 1, "Should have exactly one .sum() call");
}

// ============================================================================
// Phase 2: Nested Generator Expressions (5 tests)
// ============================================================================

#[test]
#[ignore] // FUTURE: Generator expressions - nested not yet implemented
fn test_nested_generator_expression() {
    let python = r#"
def use_gen() -> list:
    gen = (x + y for x in range(3) for y in range(3))
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have nested iteration (flat_map or nested loops)
    assert!(
        rust_code.contains(".flat_map(") || rust_code.contains("for"),
        "Should have nested iteration.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Generator expressions - nested not yet implemented
fn test_nested_generator_with_condition() {
    let python = r#"
def use_gen() -> list:
    gen = ((x, y) for x in range(3) for y in range(x))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should handle dependent iteration (y depends on x)
    assert!(
        rust_code.contains("impl Iterator"),
        "Should have Iterator implementation.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Generator expressions - nested not yet implemented
fn test_nested_generator_with_filter() {
    let python = r#"
def use_gen() -> list:
    gen = (x * y for x in range(3) for y in range(3) if x != y)
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have filter in nested context
    assert!(
        rust_code.contains(".filter(") || rust_code.contains("if"),
        "Should have filter condition.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Generator expressions - nested not yet implemented
fn test_generator_of_generator_expressions() {
    let python = r#"
def use_gen() -> list:
    # Generator of generators (complex)
    outer = ((x, list(y for y in range(x))) for x in range(3))
    return list(outer)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should handle nested generator expressions
    assert!(
        rust_code.contains(".map("),
        "Should have nested transformations.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Generator expressions - nested not yet implemented
fn test_cartesian_product_generator() {
    let python = r#"
def use_gen(a: list, b: list) -> list:
    gen = ((x, y) for x in a for y in b)
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should create cartesian product
    assert!(
        rust_code.contains(".flat_map(") || rust_code.contains("impl Iterator"),
        "Should have cartesian product logic.\\nGot:\\n{}",
        rust_code
    );
}

// ============================================================================
// Phase 3: Edge Cases (5 tests)
// ============================================================================

#[test]
fn test_generator_expression_with_complex_condition() {
    let python = r#"
def use_gen(nums: list) -> list:
    gen = (x for x in nums if x > 0 and x < 100)
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should handle compound boolean conditions
    assert!(
        rust_code.contains("&&") || rust_code.contains("and"),
        "Should have AND condition.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_with_function_call() {
    let python = r#"
def double(x: int) -> int:
    return x * 2

def use_gen() -> list:
    gen = (double(x) for x in range(5))
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should call function in map
    assert!(
        rust_code.contains("double("),
        "Should call double function.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_variable_capture() {
    let python = r#"
def use_gen(multiplier: int) -> list:
    gen = (x * multiplier for x in range(5))
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should capture multiplier variable
    assert!(
        rust_code.contains("multiplier"),
        "Should reference multiplier.\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_enumerate_pattern() {
    let python = r#"
def use_gen(items: list) -> list:
    gen = ((i, item) for i, item in enumerate(items))
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should use .enumerate()
    assert!(
        rust_code.contains(".enumerate()"),
        "Should have .enumerate().\\nGot:\\n{}",
        rust_code
    );
}

#[test]
fn test_generator_expression_zip_pattern() {
    let python = r#"
def use_gen(a: list, b: list) -> list:
    gen = ((x, y) for x, y in zip(a, b))
    return list(gen)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should use .zip()
    assert!(
        rust_code.contains(".zip("),
        "Should have .zip().\\nGot:\\n{}",
        rust_code
    );
}
