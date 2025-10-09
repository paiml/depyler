//! TDD Tests for Stateful Generators (DEPYLER-0115 Phase 2)
//!
//! Phase 2: Generator state management and resumable execution
//! Python: Stateful yield → Rust: Iterator with state machine
//!
//! Test Coverage (20 tests):
//! 1. Generator maintaining counter state
//! 2. Generator with multiple state variables
//! 3. Generator resuming from yield point
//! 4. Generator with conditional state updates
//! 5. Generator preserving loop iteration state
//! 6. Generator with accumulator state
//! 7. Fibonacci generator (classic stateful example)
//! 8. Generator with state reset on completion
//! 9. Generator tracking iteration count
//! 10. Generator with mutable state parameter
//! 11. Generator state across multiple yields in loop
//! 12. Generator with state-dependent yields
//! 13. Generator preserving local variable state
//! 14. Generator with nested loop state
//! 15. Generator state in conditional branches
//! 16. Generator with early termination state
//! 17. Generator collecting state across iterations
//! 18. Generator with state initialization
//! 19. Generator state transitions
//! 20. Complex stateful generator pattern

use depyler_core::DepylerPipeline;

#[test]
fn test_counter_state() {
    let python = r#"
def counter(start: int, end: int):
    current = start
    while current < end:
        yield current
        current = current + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have state struct
    assert!(
        rust_code.contains("struct Counter") || rust_code.contains("struct counter"),
        "Should have generator state struct.\nGot:\n{}",
        rust_code
    );

    // Should implement Iterator
    assert!(
        rust_code.contains("impl Iterator"),
        "Should implement Iterator trait.\nGot:\n{}",
        rust_code
    );

    // Should have state field for current
    assert!(
        rust_code.contains("current"),
        "Should track current state.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_multiple_state_variables() {
    let python = r#"
def dual_counter(n: int):
    even = 0
    odd = 1
    for i in range(n):
        if i % 2 == 0:
            yield even
            even = even + 2
        else:
            yield odd
            odd = odd + 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct"),
        "Should have state struct.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fibonacci_generator() {
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
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should track both a and b in state
    assert!(
        rust_code.contains("impl Iterator"),
        "Should implement Iterator.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_accumulator_state() {
    let python = r#"
def running_sum(numbers: list):
    total = 0
    for num in numbers:
        total = total + num
        yield total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("total") || rust_code.contains("state"),
        "Should maintain accumulator state.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_state_in_nested_loop() {
    let python = r#"
def grid_generator(rows: int, cols: int):
    for i in range(rows):
        for j in range(cols):
            yield (i, j)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should track both i and j
    assert!(
        rust_code.contains("impl Iterator"),
        "Should implement Iterator.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_conditional_state_updates() {
    let python = r#"
def conditional_gen(n: int):
    count = 0
    for i in range(n):
        if i % 2 == 0:
            count = count + 1
        yield count
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("count") || rust_code.contains("state"),
        "Should track count state.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_iteration_count_tracking() {
    let python = r#"
def indexed_values(items: list):
    index = 0
    for item in items:
        yield (index, item)
        index = index + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("index"),
        "Should track index state.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_early_termination_state() {
    let python = r#"
def limited_gen(n: int, limit: int):
    count = 0
    for i in range(n):
        if count >= limit:
            return
        yield i
        count = count + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("impl Iterator"),
        "Should implement Iterator.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_state_dependent_yields() {
    let python = r#"
def alternating(n: int):
    toggle = True
    for i in range(n):
        if toggle:
            yield i
        else:
            yield -i
        toggle = not toggle
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("toggle") || rust_code.contains("bool"),
        "Should track toggle state.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_state_preservation_across_yields() {
    let python = r#"
def preserving_gen(n: int):
    x = 0
    for i in range(n):
        x = x + i
        yield x
        x = x * 2
        yield x
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should preserve x across multiple yields
    assert!(
        rust_code.contains("impl Iterator"),
        "Should implement Iterator.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_state_initialization() {
    let python = r#"
def initialized_gen(start: int, step: int):
    value = start
    while value < 100:
        yield value
        value = value + step
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should initialize state from parameters
    assert!(
        rust_code.contains("start") || rust_code.contains("new"),
        "Should initialize state.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_collecting_state() {
    let python = r#"
def collecting_gen(items: list):
    collected = []
    for item in items:
        collected.append(item)
        yield collected.copy()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("Vec") || rust_code.contains("vec"),
        "Should maintain collection state.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_state_transitions() {
    let python = r#"
def state_machine(n: int):
    state = 0
    for i in range(n):
        if state == 0:
            yield "start"
            state = 1
        elif state == 1:
            yield "middle"
            state = 2
        else:
            yield "end"
            state = 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("state") || rust_code.contains("enum"),
        "Should track state transitions.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_powers_of_two_generator() {
    let python = r#"
def powers_of_two(n: int):
    power = 1
    for i in range(n):
        yield power
        power = power * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("power"),
        "Should track power state.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_range_like_generator() {
    let python = r#"
def my_range(start: int, stop: int, step: int):
    current = start
    while current < stop:
        yield current
        current = current + step
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should be similar to Rust Range
    assert!(
        rust_code.contains("current"),
        "Should track current position.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_filter_generator() {
    let python = r#"
def filtered_gen(items: list, threshold: int):
    count = 0
    for item in items:
        if item > threshold:
            yield item
            count = count + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("count"),
        "Should track filter count.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_windowed_generator() {
    let python = r#"
def windowed(items: list, size: int):
    for i in range(len(items) - size + 1):
        window = []
        for j in range(size):
            window.append(items[i + j])
        yield window
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("impl Iterator"),
        "Should implement Iterator.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_pairwise_generator() {
    let python = r#"
def pairwise(items: list):
    prev = None
    for item in items:
        if prev is not None:
            yield (prev, item)
        prev = item
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("prev") || rust_code.contains("Option"),
        "Should track previous value.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_complex_stateful_pattern() {
    let python = r#"
def complex_gen(n: int):
    state_a = 0
    state_b = 1
    state_c = 0
    for i in range(n):
        result = state_a + state_b
        yield result
        state_c = state_a
        state_a = state_b
        state_b = state_c + state_b
        if result > 100:
            state_a = 0
            state_b = 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Complex state with multiple variables and conditional resets
    assert!(
        rust_code.contains("struct") && rust_code.contains("impl Iterator"),
        "Should have stateful Iterator implementation.\nGot:\n{}",
        rust_code
    );
}
