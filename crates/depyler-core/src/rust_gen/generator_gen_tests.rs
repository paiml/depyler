//! Comprehensive generator generator tests
//!
//! These tests exercise the generator_gen.rs code paths through the transpilation pipeline.

use crate::DepylerPipeline;

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============================================================================
// BASIC GENERATOR FUNCTIONS
// ============================================================================

#[test]
fn test_simple_yield() {
    assert!(transpile_ok("def gen():\n    yield 1"));
}

#[test]
fn test_multiple_yields() {
    assert!(transpile_ok("def gen():\n    yield 1\n    yield 2\n    yield 3"));
}

#[test]
fn test_yield_in_loop() {
    assert!(transpile_ok("def gen(n):\n    for i in range(n):\n        yield i"));
}

#[test]
fn test_yield_with_state() {
    assert!(transpile_ok("def gen():\n    x = 1\n    yield x\n    x += 1\n    yield x"));
}

#[test]
fn test_yield_from_list() {
    assert!(transpile_ok("def gen():\n    yield from [1, 2, 3]"));
}

#[test]
fn test_yield_from_range() {
    assert!(transpile_ok("def gen():\n    yield from range(10)"));
}

#[test]
fn test_yield_from_generator() {
    assert!(transpile_ok("def gen1():\n    yield 1\n\ndef gen2():\n    yield from gen1()"));
}

// ============================================================================
// GENERATOR WITH CONDITIONS
// ============================================================================

#[test]
fn test_generator_with_if() {
    assert!(transpile_ok("def gen(n):\n    for i in range(n):\n        if i % 2 == 0:\n            yield i"));
}

#[test]
fn test_generator_with_if_else() {
    assert!(transpile_ok("def gen(n):\n    for i in range(n):\n        if i % 2 == 0:\n            yield i\n        else:\n            yield -i"));
}

#[test]
fn test_generator_early_return() {
    assert!(transpile_ok("def gen(n):\n    if n < 0:\n        return\n    for i in range(n):\n        yield i"));
}

// ============================================================================
// GENERATOR WITH MULTIPLE LOOPS
// ============================================================================

#[test]
fn test_generator_nested_loops() {
    assert!(transpile_ok("def gen(rows, cols):\n    for i in range(rows):\n        for j in range(cols):\n            yield (i, j)"));
}

#[test]
fn test_generator_sequential_loops() {
    assert!(transpile_ok("def gen():\n    for i in range(3):\n        yield i\n    for j in range(3, 6):\n        yield j"));
}

// ============================================================================
// GENERATOR WITH STATE VARIABLES
// ============================================================================

#[test]
fn test_generator_accumulator() {
    assert!(transpile_ok("def gen(n):\n    total = 0\n    for i in range(n):\n        total += i\n        yield total"));
}

#[test]
fn test_generator_fibonacci() {
    assert!(transpile_ok("def fib(n):\n    a, b = 0, 1\n    for _ in range(n):\n        yield a\n        a, b = b, a + b"));
}

#[test]
fn test_generator_with_list_state() {
    assert!(transpile_ok("def gen():\n    items = []\n    for i in range(5):\n        items.append(i)\n        yield items.copy()"));
}

// ============================================================================
// GENERATOR WITH PARAMETERS
// ============================================================================

#[test]
fn test_generator_with_single_param() {
    assert!(transpile_ok("def gen(n: int):\n    for i in range(n):\n        yield i"));
}

#[test]
fn test_generator_with_multiple_params() {
    assert!(transpile_ok("def gen(start: int, end: int, step: int):\n    i = start\n    while i < end:\n        yield i\n        i += step"));
}

#[test]
fn test_generator_with_default_param() {
    assert!(transpile_ok("def gen(n: int = 10):\n    for i in range(n):\n        yield i"));
}

// ============================================================================
// GENERATOR WITH TYPE HINTS
// ============================================================================

#[test]
fn test_generator_return_type_int() {
    assert!(transpile_ok("from typing import Iterator\n\ndef gen() -> Iterator[int]:\n    yield 1"));
}

#[test]
fn test_generator_return_type_str() {
    assert!(transpile_ok("from typing import Iterator\n\ndef gen() -> Iterator[str]:\n    yield 'hello'"));
}

#[test]
fn test_generator_return_type_tuple() {
    assert!(transpile_ok("from typing import Iterator, Tuple\n\ndef gen() -> Iterator[Tuple[int, int]]:\n    yield (1, 2)"));
}

// ============================================================================
// GENERATOR EXPRESSIONS
// ============================================================================

#[test]
fn test_generator_expression_simple() {
    assert!(transpile_ok("gen = (x for x in range(10))"));
}

#[test]
fn test_generator_expression_with_condition() {
    assert!(transpile_ok("gen = (x for x in range(10) if x % 2 == 0)"));
}

#[test]
fn test_generator_expression_with_transform() {
    assert!(transpile_ok("gen = (x * 2 for x in range(10))"));
}

#[test]
fn test_generator_expression_nested() {
    assert!(transpile_ok("gen = ((x, y) for x in range(3) for y in range(3))"));
}

// ============================================================================
// CONSUMING GENERATORS
// ============================================================================

#[test]
fn test_generator_in_list() {
    assert!(transpile_ok("def gen():\n    yield 1\n    yield 2\n\nresult = list(gen())"));
}

#[test]
fn test_generator_in_for() {
    assert!(transpile_ok("def gen():\n    yield 1\n    yield 2\n\ndef consume():\n    for x in gen():\n        pass"));
}

#[test]
fn test_generator_in_sum() {
    assert!(transpile_ok("def gen():\n    yield 1\n    yield 2\n\ntotal = sum(gen())"));
}

// ============================================================================
// COMPLEX GENERATOR PATTERNS
// ============================================================================

#[test]
fn test_generator_infinite() {
    assert!(transpile_ok("def count(start: int = 0):\n    n = start\n    while True:\n        yield n\n        n += 1"));
}

#[test]
fn test_generator_with_try() {
    assert!(transpile_ok("def gen(items):\n    for item in items:\n        try:\n            yield int(item)\n        except:\n            pass"));
}

#[test]
fn test_generator_pipeline() {
    assert!(transpile_ok("def gen1():\n    yield 1\n    yield 2\n\ndef gen2():\n    for x in gen1():\n        yield x * 2"));
}

#[test]
fn test_generator_with_break_condition() {
    assert!(transpile_ok("def gen(n):\n    for i in range(1000):\n        if i >= n:\n            break\n        yield i"));
}

// ============================================================================
// GENERATOR WITH WHILE LOOPS
// ============================================================================

#[test]
fn test_generator_while_simple() {
    assert!(transpile_ok("def gen():\n    i = 0\n    while i < 5:\n        yield i\n        i += 1"));
}

#[test]
fn test_generator_while_condition() {
    assert!(transpile_ok("def gen(items):\n    i = 0\n    while i < len(items):\n        yield items[i]\n        i += 1"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_generator_empty() {
    assert!(transpile_ok("def gen():\n    if False:\n        yield 1"));
}

#[test]
fn test_generator_single_yield() {
    assert!(transpile_ok("def gen():\n    yield 42"));
}

#[test]
fn test_generator_yield_none() {
    assert!(transpile_ok("def gen():\n    yield None"));
}

#[test]
fn test_generator_yield_expression() {
    assert!(transpile_ok("def gen(a, b):\n    yield a + b"));
}

#[test]
fn test_generator_yield_function_call() {
    assert!(transpile_ok("def gen():\n    yield len([1, 2, 3])"));
}

#[test]
fn test_generator_yield_method_call() {
    assert!(transpile_ok("def gen():\n    yield 'hello'.upper()"));
}
