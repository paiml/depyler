//! TDD Tests for Generator Functions (DEPYLER-0115 Phase 1)
//!
//! Phase 1: Simple yield statements
//! Python: yield → Rust: Iterator trait implementation
//!
//! Test Coverage (15 tests):
//! 1. Simple yield single value
//! 2. Yield multiple values
//! 3. Generator with loop
//! 4. Generator with range
//! 5. Generator with conditional yield
//! 6. Generator with parameter
//! 7. Generator with multiple parameters
//! 8. Generator yielding expressions
//! 9. Generator with local variables
//! 10. Generator with computations
//! 11. Generator in for loop
//! 12. Generator converting to list
//! 13. Generator yielding strings
//! 14. Generator with return (StopIteration)
//! 15. Generator with complex logic

use depyler_core::DepylerPipeline;

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_simple_yield_single_value() {
    let python = r#"
def simple_generator():
    yield 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn simple_generator") || rust_code.contains("struct SimpleGenerator"),
        "Should have generator function or struct.\nGot:\n{}",
        rust_code
    );

    // Should have Iterator implementation
    let has_iterator = rust_code.contains("Iterator") || rust_code.contains("iter");
    assert!(has_iterator, "Should have Iterator trait.\nGot:\n{}", rust_code);
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_yield_multiple_values() {
    let python = r#"
def count_to_three():
    yield 1
    yield 2
    yield 3
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("count_to_three") || rust_code.contains("CountToThree"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_with_loop() {
    let python = r#"
def count_up(n: int):
    i = 0
    while i < n:
        yield i
        i = i + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("count_up") || rust_code.contains("CountUp"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_with_range() {
    let python = r#"
def range_generator(n: int):
    for i in range(n):
        yield i
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("range_generator") || rust_code.contains("RangeGenerator"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_with_conditional() {
    let python = r#"
def even_numbers(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("even_numbers") || rust_code.contains("EvenNumbers"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_with_parameter() {
    let python = r#"
def repeat_value(value: int, times: int):
    for i in range(times):
        yield value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("repeat_value") || rust_code.contains("RepeatValue"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_with_multiple_parameters() {
    let python = r#"
def add_sequence(start: int, end: int, step: int):
    current = start
    while current < end:
        yield current
        current = current + step
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("add_sequence") || rust_code.contains("AddSequence"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_yielding_expressions() {
    let python = r#"
def squares(n: int):
    for i in range(n):
        yield i * i
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("squares") || rust_code.contains("Squares"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_with_local_variables() {
    let python = r#"
def fibonacci(n: int):
    a = 0
    b = 1
    for i in range(n):
        yield a
        temp = a
        a = b
        b = temp + b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fibonacci") || rust_code.contains("Fibonacci"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_with_computations() {
    let python = r#"
def powers_of_two(n: int):
    power = 1
    for i in range(n):
        yield power
        power = power * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("powers_of_two") || rust_code.contains("PowersOfTwo"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_in_for_loop() {
    let python = r#"
def simple_range(n: int):
    for i in range(n):
        yield i

def use_generator():
    total = 0
    for value in simple_range(5):
        total = total + value
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("use_generator"),
        "Should have use_generator function.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_to_list() {
    let python = r#"
def numbers(n: int):
    for i in range(n):
        yield i

def get_list():
    return list(numbers(5))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("get_list"),
        "Should have get_list function.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_yielding_strings() {
    let python = r#"
def string_generator():
    yield "first"
    yield "second"
    yield "third"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("string_generator") || rust_code.contains("StringGenerator"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_with_return() {
    let python = r#"
def limited_generator(n: int):
    for i in range(n):
        if i >= 3:
            return
        yield i
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("limited_generator") || rust_code.contains("LimitedGenerator"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Yield not yet implemented
fn test_generator_with_complex_logic() {
    let python = r#"
def complex_generator(start: int, end: int):
    current = start
    while current < end:
        if current % 2 == 0:
            yield current * 2
        else:
            yield current
        current = current + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("complex_generator") || rust_code.contains("ComplexGenerator"),
        "Should have generator.\nGot:\n{}",
        rust_code
    );
}
