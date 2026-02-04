//! Coverage tests for type_coercion.rs, generator_gen.rs, and var_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets mid-size uncovered modules
//! Covers: type coercion patterns, generator/yield codegen,
//! variable analysis for mutability and scope.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Type coercion: int/float mixed arithmetic
// ============================================================================

#[test]
fn test_coerce_int_plus_float() {
    let code = r#"
def f(a: int, b: float) -> float:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_float_times_int() {
    let code = r#"
def f(x: float, n: int) -> float:
    return x * n
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_int_division() {
    let code = r#"
def f(a: int, b: int) -> float:
    return a / b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_power_to_float() {
    let code = r#"
def f(x: int) -> float:
    return x ** 0.5
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_comparison_mixed() {
    let code = r#"
def f(a: int, b: float) -> bool:
    return a < b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_int_to_float_assignment() {
    let code = r#"
def f() -> float:
    x: float = 5
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_str_to_int() {
    let code = r#"
def f(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_int_to_str() {
    let code = r#"
def f(n: int) -> str:
    return str(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_float_to_int() {
    let code = r#"
def f(x: float) -> int:
    return int(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_bool_to_int() {
    let code = r#"
def f(b: bool) -> int:
    return int(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_complex_arithmetic() {
    let code = r#"
def f(a: int, b: int, c: float) -> float:
    return (a + b) * c - a / c
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coerce_augmented_assign_float() {
    let code = r#"
def f() -> float:
    x: float = 0.0
    x += 1
    x *= 2
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Generator functions
// ============================================================================

#[test]
fn test_generator_simple_yield() {
    let code = r#"
def gen(n: int):
    for i in range(n):
        yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_yield_with_condition() {
    let code = r#"
def evens(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_fibonacci() {
    let code = r#"
def fib():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_yield_string() {
    let code = r#"
def words(text: str):
    for word in text.split():
        yield word
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_yield_computed() {
    let code = r#"
def squares(n: int):
    for i in range(n):
        yield i * i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_with_state() {
    let code = r#"
def counter(start: int, step: int):
    n = start
    while True:
        yield n
        n += step
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_with_early_return() {
    let code = r#"
def limited(items: list, limit: int):
    count = 0
    for item in items:
        if count >= limit:
            return
        yield item
        count += 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_nested_loop() {
    let code = r#"
def pairs(n: int):
    for i in range(n):
        for j in range(i + 1, n):
            yield (i, j)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable analysis: mutability detection
// ============================================================================

#[test]
fn test_var_mutated_by_augmented_assign() {
    let code = r#"
def f() -> int:
    x = 0
    x += 1
    x += 2
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_mutated_in_loop() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_reassigned() {
    let code = r#"
def f(x: int) -> int:
    result = x
    result = result * 2
    result = result + 1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_mutable_list() {
    let code = r#"
def f() -> list:
    items = []
    items.append(1)
    items.append(2)
    items.append(3)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_mutable_dict() {
    let code = r#"
def f() -> dict:
    d = {}
    d["a"] = 1
    d["b"] = 2
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_mutable_set() {
    let code = r#"
def f() -> int:
    s = set()
    s.add(1)
    s.add(2)
    s.add(3)
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_scope_if_else() {
    let code = r#"
def f(flag: bool) -> str:
    if flag:
        msg = "yes"
    else:
        msg = "no"
    return msg
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_scope_try_except() {
    let code = r#"
def f(s: str) -> int:
    try:
        val = int(s)
    except ValueError:
        val = 0
    return val
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_multiple_mutations() {
    let code = r#"
def f(data: list) -> dict:
    result = {}
    count = 0
    for item in data:
        if item not in result:
            result[item] = 0
        result[item] += 1
        count += 1
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression analysis paths
// ============================================================================

#[test]
fn test_expr_string_concat_detection() {
    let code = r#"
def f(a: str, b: str) -> str:
    return a + " " + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_method_chain_detection() {
    let code = r#"
def f(text: str) -> list:
    return text.strip().lower().split(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_nested_function_call() {
    let code = r#"
def f(items: list) -> int:
    return len(sorted(items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_complex_condition() {
    let code = r#"
def f(x: int, y: int, z: int) -> bool:
    return (x > 0 and y > 0) or (z > x + y)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_ternary_in_return() {
    let code = r#"
def f(x: int) -> str:
    return "even" if x % 2 == 0 else "odd"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_list_comprehension_complex() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_dict_comprehension_complex() {
    let code = r#"
def f(items: list) -> dict:
    return {str(i): i * i for i in items if i > 0}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_nested_index() {
    let code = r#"
def f(matrix: list) -> int:
    return matrix[0][0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_fstring_complex() {
    let code = r#"
def f(name: str, score: int) -> str:
    return f"{name}: {score} points ({score * 10}%)"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_lambda_in_sorted() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_lambda_in_filter() {
    let code = r#"
def f(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_lambda_in_map() {
    let code = r#"
def f(items: list) -> list:
    return list(map(lambda x: x.upper(), items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Walrus operator patterns
// ============================================================================

#[test]
fn test_walrus_in_if() {
    let code = r#"
def f(items: list) -> int:
    for item in items:
        if (val := item * 2) > 10:
            return val
    return -1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection constructor edge cases
// ============================================================================

#[test]
fn test_empty_list_typed() {
    let code = r#"
from typing import List
def f() -> List[int]:
    items: List[int] = []
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_empty_dict_typed() {
    let code = r#"
from typing import Dict
def f() -> Dict[str, int]:
    d: Dict[str, int] = {}
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_empty_set_typed() {
    let code = r#"
from typing import Set
def f() -> Set[int]:
    s: Set[int] = set()
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_of_strings() {
    let code = r#"
from typing import List
def f() -> List[str]:
    return ["hello", "world"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_str_str() {
    let code = r#"
from typing import Dict
def f() -> Dict[str, str]:
    return {"key": "value"}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns combining multiple features
// ============================================================================

#[test]
fn test_full_word_counter() {
    let code = r#"
def word_count(text: str) -> dict:
    counts = {}
    for word in text.lower().split():
        if word in counts:
            counts[word] += 1
        else:
            counts[word] = 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_full_matrix_multiply_row() {
    let code = r#"
def dot_product(a: list, b: list) -> int:
    total = 0
    for i in range(len(a)):
        total += a[i] * b[i]
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_full_binary_search() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    low = 0
    high = len(items) - 1
    while low <= high:
        mid = (low + high) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_full_fibonacci_list() {
    let code = r#"
def fibonacci_list(n: int) -> list:
    if n <= 0:
        return []
    if n == 1:
        return [0]
    fibs = [0, 1]
    for i in range(2, n):
        fibs.append(fibs[i - 1] + fibs[i - 2])
    return fibs
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_full_prime_sieve() {
    let code = r#"
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

def primes_up_to(n: int) -> list:
    result = []
    for i in range(2, n + 1):
        if is_prime(i):
            result.append(i)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_full_flatten_nested() {
    let code = r#"
def flatten(matrix: list) -> list:
    result = []
    for row in matrix:
        for item in row:
            result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_full_string_histogram() {
    let code = r#"
def char_histogram(text: str) -> dict:
    hist = {}
    for ch in text:
        if ch in hist:
            hist[ch] += 1
        else:
            hist[ch] = 1
    return hist
"#;
    assert!(transpile_ok(code));
}
