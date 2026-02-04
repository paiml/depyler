//! Coverage tests for expr_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets expr_analysis.rs (3,124 lines)
//! Covers: usize detection, iterator identification, float inference,
//! numpy detection, option detection, purity analysis, kwarg extraction.

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
// usize detection: len(), range(), index operations
// ============================================================================

#[test]
fn test_expr_len_returns_usize() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_len_in_arithmetic() {
    let code = r#"
def f(a: list, b: list) -> int:
    return len(a) + len(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_len_comparison() {
    let code = r#"
def f(items: list) -> bool:
    return len(items) > 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_range_usage() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_range_with_len() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i in range(len(items)):
        total += items[i]
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_nested_len_chain() {
    let code = r#"
def f(items: list) -> int:
    return len(items) * 2 + 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Iterator detection: generators, enumerate, zip, map, filter
// ============================================================================

#[test]
fn test_expr_enumerate_iterator() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for idx, val in enumerate(items):
        total += idx
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_zip_iterator() {
    let code = r#"
def f(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_sorted_builtin() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_reversed_builtin() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Float inference: literals, variables, function returns, propagation
// ============================================================================

#[test]
fn test_expr_float_literal_inference() {
    let code = r#"
def f() -> float:
    return 3.14
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_float_variable_inference() {
    let code = r#"
def f(x: float) -> float:
    y = x + 1.0
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_float_propagation_mul() {
    let code = r#"
def f(x: float) -> float:
    return x * 3 + 4
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_float_propagation_div() {
    let code = r#"
def f(x: float) -> float:
    return x / 2.0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_float_function_return() {
    let code = r#"
def get_pi() -> float:
    return 3.14159

def f() -> float:
    return get_pi() + 1.0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_float_mixed_arithmetic() {
    let code = r#"
def f(a: int, b: float) -> float:
    return a * b + b / a
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Pure expression detection
// ============================================================================

#[test]
fn test_expr_pure_literal() {
    let code = r#"
def f() -> int:
    x = 5
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_pure_binary() {
    let code = r#"
def f(x: int, y: int) -> int:
    return x + y * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_pure_unary() {
    let code = r#"
def f(x: int) -> int:
    return -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_pure_tuple() {
    let code = r#"
def f(x: int, y: int) -> tuple:
    return (x, y)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_pure_list_literal() {
    let code = r#"
def f() -> list:
    return [1, 2, 3]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Option-returning expression detection
// ============================================================================

#[test]
fn test_expr_dict_get_option() {
    let code = r#"
def f(d: dict, key: str) -> int:
    val = d.get(key)
    if val is not None:
        return val
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_dict_get_default() {
    let code = r#"
def f(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_list_pop_option() {
    let code = r#"
def f(items: list) -> int:
    if len(items) > 0:
        return items.pop()
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Chained operations / complex expressions
// ============================================================================

#[test]
fn test_expr_chained_arithmetic() {
    let code = r#"
def f(x: int, y: int, z: int) -> int:
    return ((x + y) * z) + (x - y)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_nested_function_calls() {
    let code = r#"
def f(items: list) -> int:
    return len(sorted(items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_method_chain() {
    let code = r#"
def f(text: str) -> list:
    return text.strip().lower().split(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_complex_boolean() {
    let code = r#"
def f(x: int, y: int, z: int) -> bool:
    return (x > 0 and y > 0) or (z > x + y)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_ternary_expression() {
    let code = r#"
def f(x: int) -> str:
    return "pos" if x > 0 else "neg"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_nested_ternary() {
    let code = r#"
def f(x: int) -> str:
    return "pos" if x > 0 else ("zero" if x == 0 else "neg")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// String expression patterns
// ============================================================================

#[test]
fn test_expr_string_literal_extract() {
    let code = r#"
def f() -> str:
    return "hello world"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_fstring_complex() {
    let code = r#"
def f(name: str, age: int) -> str:
    return f"{name} is {age} years old"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_string_concat() {
    let code = r#"
def f(a: str, b: str) -> str:
    return a + " " + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_string_multiply() {
    let code = r#"
def f(s: str, n: int) -> str:
    return s * n
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Kwarg extraction patterns
// ============================================================================

#[test]
fn test_expr_kwarg_string_extraction() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_kwarg_in_print() {
    let code = r#"
def f():
    print("hello", end="")
    print("world", end="\n")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_kwarg_sep_in_print() {
    let code = r#"
def f():
    print("a", "b", "c", sep=", ")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type conversion detection
// ============================================================================

#[test]
fn test_expr_int_conversion() {
    let code = r#"
def f(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_float_conversion() {
    let code = r#"
def f(s: str) -> float:
    return float(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_str_conversion() {
    let code = r#"
def f(n: int) -> str:
    return str(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_bool_conversion() {
    let code = r#"
def f(n: int) -> bool:
    return bool(n)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection construction patterns
// ============================================================================

#[test]
fn test_expr_list_comprehension() {
    let code = r#"
def f(n: int) -> list:
    return [i * i for i in range(n)]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_list_comp_with_condition() {
    let code = r#"
def f(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_dict_comprehension() {
    let code = r#"
def f(items: list) -> dict:
    return {str(i): i for i in items}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_set_comprehension() {
    let code = r#"
def f(items: list) -> set:
    return {x * 2 for x in items}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Slice and index patterns
// ============================================================================

#[test]
fn test_expr_simple_index() {
    let code = r#"
def f(items: list) -> int:
    return items[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_negative_index() {
    let code = r#"
def f(items: list) -> int:
    return items[-1]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_slice_basic() {
    let code = r#"
def f(items: list) -> list:
    return items[1:3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_slice_from_start() {
    let code = r#"
def f(items: list) -> list:
    return items[:3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_slice_to_end() {
    let code = r#"
def f(items: list) -> list:
    return items[2:]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Containment checking (in / not in)
// ============================================================================

#[test]
fn test_expr_in_list_check() {
    let code = r#"
def f(items: list, target: int) -> bool:
    return target in items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_not_in_list_check() {
    let code = r#"
def f(items: list, target: int) -> bool:
    return target not in items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_in_dict_check() {
    let code = r#"
def f(d: dict, key: str) -> bool:
    return key in d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_in_string_check() {
    let code = r#"
def f(text: str, sub: str) -> bool:
    return sub in text
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Bitwise operations
// ============================================================================

#[test]
fn test_expr_bitwise_and() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a & b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_bitwise_or() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a | b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_bitwise_xor() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a ^ b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_left_shift() {
    let code = r#"
def f(a: int, n: int) -> int:
    return a << n
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_right_shift() {
    let code = r#"
def f(a: int, n: int) -> int:
    return a >> n
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Builtin function patterns
// ============================================================================

#[test]
fn test_expr_abs_builtin() {
    let code = r#"
def f(x: int) -> int:
    return abs(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_min_max_builtins() {
    let code = r#"
def f(a: int, b: int) -> int:
    return min(a, max(b, 0))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_sum_builtin() {
    let code = r#"
def f(items: list) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_any_all_builtins() {
    let code = r#"
def f(items: list) -> bool:
    return any(items) and all(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_round_builtin() {
    let code = r#"
def f(x: float) -> int:
    return round(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_isinstance_check() {
    let code = r#"
def f(x: int) -> bool:
    return isinstance(x, int)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Lambda and functional patterns
// ============================================================================

#[test]
fn test_expr_lambda_simple() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_filter_with_lambda() {
    let code = r#"
def f(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_map_with_lambda() {
    let code = r#"
def f(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// File and I/O expression detection
// ============================================================================

#[test]
fn test_expr_open_file() {
    let code = r#"
def f(path: str) -> str:
    with open(path) as file:
        return file.read()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_print_variants() {
    let code = r#"
def f(x: int):
    print(x)
    print("value:", x)
    print(f"The value is {x}")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex integration patterns
// ============================================================================

#[test]
fn test_expr_matrix_operation() {
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
fn test_expr_running_average() {
    let code = r#"
def running_avg(items: list) -> list:
    result = []
    total = 0
    for i in range(len(items)):
        total += items[i]
        result.append(total / (i + 1))
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_string_processing() {
    let code = r#"
def process(text: str) -> dict:
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
fn test_expr_nested_comprehension() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [item for row in matrix for item in row]
"#;
    assert!(transpile_ok(code));
}
