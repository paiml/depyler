//! Coverage tests for expr_gen.rs - binary ops, type casting, collection constructors
//!
//! DEPYLER-99MODE-001: Targets expr_gen.rs (62.11% -> 75%+)
//! Covers: binary operator edge cases, type casting, collection constructors,
//! stdlib type calls, iterator utilities, print variations, filter/map,
//! comprehensions, slice, format, open, subprocess patterns.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Power operator edge cases
// ============================================================================

#[test]
fn test_pow_int_int() {
    let code = r#"
def f() -> int:
    return 2 ** 10
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn f"));
}

#[test]
fn test_pow_float_int() {
    let code = r#"
def f() -> float:
    return 2.0 ** 3
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pow_int_float() {
    let code = r#"
def f() -> float:
    return 2 ** 0.5
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pow_var_var() {
    let code = r#"
def f(base: int, exp: int) -> int:
    return base ** exp
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pow_builtin() {
    let code = r#"
def f(x: int, y: int) -> int:
    return pow(x, y)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiplication edge cases (string/list repetition)
// ============================================================================

#[test]
fn test_mul_string_repeat() {
    let code = r#"
def f() -> str:
    return "abc" * 3
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mul_string_repeat_var() {
    let code = r#"
def f(n: int) -> str:
    return "-" * n
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mul_list_repeat() {
    let code = r#"
def f() -> list:
    return [0] * 10
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mul_int_int() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mul_float_float() {
    let code = r#"
def f(a: float, b: float) -> float:
    return a * b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Addition edge cases (string/list concatenation)
// ============================================================================

#[test]
fn test_add_str_str() {
    let code = r#"
def f(a: str, b: str) -> str:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_add_list_list() {
    let code = r#"
def f() -> list:
    return [1, 2] + [3, 4]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_add_int_float() {
    let code = r#"
def f(a: int, b: float) -> float:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_add_str_literal_concat() {
    let code = r#"
def f() -> str:
    return "hello" + " " + "world"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Containment operators (in, not in)
// ============================================================================

#[test]
fn test_in_list() {
    let code = r#"
def f(x: int, items: list) -> bool:
    return x in items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_not_in_list() {
    let code = r#"
def f(x: int, items: list) -> bool:
    return x not in items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_in_dict() {
    let code = r#"
def f(key: str) -> bool:
    d = {"a": 1, "b": 2}
    return key in d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_in_string() {
    let code = r#"
def f(needle: str, haystack: str) -> bool:
    return needle in haystack
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_in_set() {
    let code = r#"
def f(x: int) -> bool:
    s = {1, 2, 3, 4, 5}
    return x in s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_not_in_string() {
    let code = r#"
def f(ch: str, text: str) -> bool:
    return ch not in text
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Floor division and modulo
// ============================================================================

#[test]
fn test_floor_div_int() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a // b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_floor_div_float() {
    let code = r#"
def f(a: float, b: float) -> float:
    return a // b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_modulo() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a % b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comparison operators
// ============================================================================

#[test]
fn test_chained_comparison() {
    let code = r#"
def f(x: int) -> bool:
    return 0 < x < 100
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_triple_comparison() {
    let code = r#"
def f(x: int) -> bool:
    return 0 <= x <= 100
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_is_none() {
    let code = r#"
from typing import Optional
def f(x: Optional[int]) -> bool:
    return x is None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_is_not_none() {
    let code = r#"
from typing import Optional
def f(x: Optional[int]) -> bool:
    return x is not None
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Bitwise operators
// ============================================================================

#[test]
fn test_bitwise_and() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a & b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bitwise_or() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a | b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bitwise_xor() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a ^ b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_left_shift() {
    let code = r#"
def f(a: int, n: int) -> int:
    return a << n
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_right_shift() {
    let code = r#"
def f(a: int, n: int) -> int:
    return a >> n
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Unary operators
// ============================================================================

#[test]
fn test_unary_neg() {
    let code = r#"
def f(x: int) -> int:
    return -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unary_pos() {
    let code = r#"
def f(x: int) -> int:
    return +x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unary_not() {
    let code = r#"
def f(x: bool) -> bool:
    return not x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unary_bitwise_not() {
    let code = r#"
def f(x: int) -> int:
    return ~x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type casting functions
// ============================================================================

#[test]
fn test_int_from_float() {
    let code = r#"
def f(x: float) -> int:
    return int(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_int_from_str() {
    let code = r#"
def f(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_int_from_bool() {
    let code = r#"
def f(b: bool) -> int:
    return int(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_float_from_int() {
    let code = r#"
def f(x: int) -> float:
    return float(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_float_from_str() {
    let code = r#"
def f(s: str) -> float:
    return float(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_from_int() {
    let code = r#"
def f(x: int) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_from_float() {
    let code = r#"
def f(x: float) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_from_bool() {
    let code = r#"
def f(b: bool) -> str:
    return str(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bool_from_int() {
    let code = r#"
def f(x: int) -> bool:
    return bool(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bool_from_str() {
    let code = r#"
def f(s: str) -> bool:
    return bool(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection constructors
// ============================================================================

#[test]
fn test_list_constructor_empty() {
    let code = r#"
def f() -> list:
    return list()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_from_range() {
    let code = r#"
def f() -> list:
    return list(range(10))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_constructor_empty() {
    let code = r#"
def f() -> dict:
    return dict()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_constructor_from_list() {
    let code = r#"
def f() -> int:
    s = set([1, 2, 2, 3, 3, 3])
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_tuple_constructor() {
    let code = r#"
def f() -> tuple:
    return tuple([1, 2, 3])
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_frozenset_constructor() {
    let code = r#"
def f() -> int:
    fs = frozenset([1, 2, 3])
    return len(fs)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Builtin functions
// ============================================================================

#[test]
fn test_len_list() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_len_str() {
    let code = r#"
def f(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_len_dict() {
    let code = r#"
def f(d: dict) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_abs_int() {
    let code = r#"
def f(x: int) -> int:
    return abs(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_abs_float() {
    let code = r#"
def f(x: float) -> float:
    return abs(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_round_float() {
    let code = r#"
def f(x: float) -> int:
    return round(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_round_with_digits() {
    let code = r#"
def f(x: float) -> float:
    return round(x, 2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_min_two_args() {
    let code = r#"
def f(a: int, b: int) -> int:
    return min(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_min_list() {
    let code = r#"
def f(items: list) -> int:
    return min(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_max_two_args() {
    let code = r#"
def f(a: int, b: int) -> int:
    return max(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_max_list() {
    let code = r#"
def f(items: list) -> int:
    return max(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sum_list() {
    let code = r#"
def f(items: list) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sum_with_start() {
    let code = r#"
def f(items: list) -> int:
    return sum(items, 10)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_any_function() {
    let code = r#"
def f(items: list) -> bool:
    return any(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_all_function() {
    let code = r#"
def f(items: list) -> bool:
    return all(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sorted_function() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sorted_reverse() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, reverse=True)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sorted_with_key() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_reversed_function() {
    let code = r#"
def f(items: list) -> list:
    return list(reversed(items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_enumerate_function() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i, v in enumerate(items):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_zip_function() {
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
fn test_isinstance_check() {
    let code = r#"
def f(x: int) -> bool:
    return isinstance(x, int)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hex_function() {
    let code = r#"
def f(x: int) -> str:
    return hex(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bin_function() {
    let code = r#"
def f(x: int) -> str:
    return bin(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_oct_function() {
    let code = r#"
def f(x: int) -> str:
    return oct(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_chr_function() {
    let code = r#"
def f(x: int) -> str:
    return chr(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ord_function() {
    let code = r#"
def f(c: str) -> int:
    return ord(c)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hash_function() {
    let code = r#"
def f(s: str) -> int:
    return hash(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Print function variations
// ============================================================================

#[test]
fn test_print_single() {
    let code = r#"
def f():
    print("hello")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_print_multiple_args() {
    let code = r#"
def f(name: str, age: int):
    print("Name:", name, "Age:", age)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_print_with_sep() {
    let code = r#"
def f():
    print("a", "b", "c", sep=",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_print_with_end() {
    let code = r#"
def f():
    print("hello", end="")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_print_empty() {
    let code = r#"
def f():
    print()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_print_int() {
    let code = r#"
def f(x: int):
    print(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_print_float() {
    let code = r#"
def f(x: float):
    print(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Filter and map builtins
// ============================================================================

#[test]
fn test_filter_with_lambda() {
    let code = r#"
def f(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_map_with_lambda() {
    let code = r#"
def f(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_map_with_str() {
    let code = r#"
def f(items: list) -> list:
    return list(map(str, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_map_with_int() {
    let code = r#"
def f(items: list) -> list:
    return list(map(int, items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comprehensions
// ============================================================================

#[test]
fn test_list_comprehension_simple() {
    let code = r#"
def f(n: int) -> list:
    return [i for i in range(n)]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_comprehension_with_condition() {
    let code = r#"
def f(n: int) -> list:
    return [i for i in range(n) if i % 2 == 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_comprehension_with_transform() {
    let code = r#"
def f(items: list) -> list:
    return [x * x for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_comprehension() {
    let code = r#"
def f(n: int) -> dict:
    return {str(i): i * i for i in range(n)}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_comprehension() {
    let code = r#"
def f(items: list) -> set:
    return {x % 10 for x in items}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_in_sum() {
    let code = r#"
def f(n: int) -> int:
    return sum(i * i for i in range(n))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_in_any() {
    let code = r#"
def f(items: list) -> bool:
    return any(x > 10 for x in items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_in_all() {
    let code = r#"
def f(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Slice operations
// ============================================================================

#[test]
fn test_slice_basic() {
    let code = r#"
def f(items: list) -> list:
    return items[1:3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_from_start() {
    let code = r#"
def f(items: list) -> list:
    return items[:3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_to_end() {
    let code = r#"
def f(items: list) -> list:
    return items[2:]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_with_step() {
    let code = r#"
def f(items: list) -> list:
    return items[::2]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_string() {
    let code = r#"
def f(s: str) -> str:
    return s[1:4]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Index operations
// ============================================================================

#[test]
fn test_index_list() {
    let code = r#"
def f(items: list) -> int:
    return items[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_dict() {
    let code = r#"
def f(d: dict) -> int:
    return d["key"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_string() {
    let code = r#"
def f(s: str) -> str:
    return s[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_negative_index() {
    let code = r#"
def f(items: list) -> int:
    return items[-1]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// F-string formatting
// ============================================================================

#[test]
fn test_fstring_simple() {
    let code = r#"
def f(name: str) -> str:
    return f"Hello, {name}!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_fstring_expression() {
    let code = r#"
def f(x: int) -> str:
    return f"The answer is {x + 1}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_fstring_multiple_values() {
    let code = r#"
def f(name: str, age: int) -> str:
    return f"{name} is {age} years old"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ternary (if-else expression)
// ============================================================================

#[test]
fn test_ternary_int() {
    let code = r#"
def f(x: int) -> int:
    return x if x > 0 else -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ternary_str() {
    let code = r#"
def f(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ternary_nested() {
    let code = r#"
def f(x: int) -> str:
    return "positive" if x > 0 else ("zero" if x == 0 else "negative")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Lambda expressions
// ============================================================================

#[test]
fn test_lambda_simple() {
    let code = r#"
def f() -> int:
    add = lambda x, y: x + y
    return add(3, 4)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_single_arg() {
    let code = r#"
def f() -> int:
    double = lambda x: x * 2
    return double(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_in_sorted() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Named expression (walrus operator)
// ============================================================================

#[test]
fn test_walrus_in_if() {
    let code = r#"
def f(items: list, target: int) -> int:
    for item in items:
        if (doubled := item * 2) > target:
            return doubled
    return -1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Tuple operations
// ============================================================================

#[test]
fn test_tuple_creation() {
    let code = r#"
def f() -> tuple:
    return (1, 2, 3)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_tuple_unpacking() {
    let code = r#"
def f() -> int:
    a, b, c = (1, 2, 3)
    return a + b + c
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_tuple_index() {
    let code = r#"
def f() -> int:
    t = (10, 20, 30)
    return t[1]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Attribute access
// ============================================================================

#[test]
fn test_attribute_access_class() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    def get_x(self) -> int:
        return self.x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_attribute_access_nested() {
    let code = r#"
class Config:
    def __init__(self):
        self.name = "default"
    def get_name(self) -> str:
        return self.name
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Open/file operations
// ============================================================================

#[test]
fn test_open_read() {
    let code = r#"
def f(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_open_write() {
    let code = r#"
def f(path: str, content: str):
    with open(path, "w") as f:
        f.write(content)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_open_append() {
    let code = r#"
def f(path: str, line: str):
    with open(path, "a") as f:
        f.write(line + "\n")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Stdlib type constructors
// ============================================================================

#[test]
fn test_datetime_constructor() {
    let code = r#"
from datetime import datetime
def f():
    dt = datetime(2024, 1, 15)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_date_constructor() {
    let code = r#"
from datetime import date
def f():
    d = date(2024, 1, 15)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_timedelta_constructor() {
    let code = r#"
from datetime import timedelta
def f():
    td = timedelta(days=7)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_path_constructor() {
    let code = r#"
from pathlib import Path
def f() -> str:
    p = Path("/usr/local/bin")
    return str(p)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex expressions combining multiple features
// ============================================================================

#[test]
fn test_complex_list_processing() {
    let code = r#"
def process(data: list) -> list:
    result = []
    for item in data:
        if item > 0:
            result.append(item * 2)
    return sorted(result, reverse=True)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_dict_building() {
    let code = r#"
def build_index(words: list) -> dict:
    index = {}
    for i, word in enumerate(words):
        if word not in index:
            index[word] = []
        index[word].append(i)
    return index
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_string_processing() {
    let code = r#"
def normalize(text: str) -> str:
    words = text.strip().lower().split()
    return " ".join(words)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_math_expression() {
    let code = r#"
def quadratic(a: float, b: float, c: float) -> float:
    discriminant = b ** 2 - 4.0 * a * c
    return (-b + discriminant ** 0.5) / (2.0 * a)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_boolean_logic() {
    let code = r#"
def validate(x: int, y: int, z: int) -> bool:
    return (x > 0 and y > 0 and z > 0) or (x + y + z == 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_nested_comprehension_result() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_enumerate_with_start() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i, v in enumerate(items, 1):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_input_function() {
    let code = r#"
def f() -> str:
    name = input("Enter name: ")
    return name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_input_no_prompt() {
    let code = r#"
def f() -> str:
    return input()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Stdlib module method calls
// ============================================================================

#[test]
fn test_os_path_join() {
    let code = r#"
import os
def f() -> str:
    return os.path.join("/usr", "local", "bin")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_exists() {
    let code = r#"
import os
def f(p: str) -> bool:
    return os.path.exists(p)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_basename() {
    let code = r#"
import os
def f(p: str) -> str:
    return os.path.basename(p)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_dirname() {
    let code = r#"
import os
def f(p: str) -> str:
    return os.path.dirname(p)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_splitext() {
    let code = r#"
import os
def f(p: str) -> tuple:
    return os.path.splitext(p)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_loads() {
    let code = r#"
import json
def f(s: str) -> dict:
    return json.loads(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_dumps() {
    let code = r#"
import json
def f(d: dict) -> str:
    return json.dumps(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_base64_encode() {
    let code = r#"
import base64
def f(data: bytes) -> bytes:
    return base64.b64encode(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_base64_decode() {
    let code = r#"
import base64
def f(data: bytes) -> bytes:
    return base64.b64decode(data)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Boolean short-circuit evaluation
// ============================================================================

#[test]
fn test_and_short_circuit() {
    let code = r#"
def f(items: list) -> bool:
    return len(items) > 0 and items[0] > 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_or_short_circuit() {
    let code = r#"
def f(x: int, default: int) -> int:
    return x or default
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_and_or() {
    let code = r#"
def f(a: bool, b: bool, c: bool) -> bool:
    return (a and b) or (not a and c)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Await expression
// ============================================================================

#[test]
fn test_await_expression() {
    let code = r#"
async def f(url: str) -> str:
    result = await fetch(url)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Statistics module
// ============================================================================

#[test]
fn test_statistics_mean() {
    let code = r#"
import statistics
def f(data: list) -> float:
    return statistics.mean(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_statistics_median() {
    let code = r#"
import statistics
def f(data: list) -> float:
    return statistics.median(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_statistics_stdev() {
    let code = r#"
import statistics
def f(data: list) -> float:
    return statistics.stdev(data)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Copy module
// ============================================================================

#[test]
fn test_copy_copy() {
    let code = r#"
import copy
def f(items: list) -> list:
    return copy.copy(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_copy_deepcopy() {
    let code = r#"
import copy
def f(items: list) -> list:
    return copy.deepcopy(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Heapq module
// ============================================================================

#[test]
fn test_heapq_heappush() {
    let code = r#"
import heapq
def f() -> list:
    heap = []
    heapq.heappush(heap, 3)
    heapq.heappush(heap, 1)
    heapq.heappush(heap, 2)
    return heap
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_heapq_heappop() {
    let code = r#"
import heapq
def f(heap: list) -> int:
    return heapq.heappop(heap)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_heapq_nsmallest() {
    let code = r#"
import heapq
def f(items: list) -> list:
    return heapq.nsmallest(3, items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Bisect module
// ============================================================================

#[test]
fn test_bisect_left() {
    let code = r#"
import bisect
def f(items: list, x: int) -> int:
    return bisect.bisect_left(items, x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bisect_right() {
    let code = r#"
import bisect
def f(items: list, x: int) -> int:
    return bisect.bisect_right(items, x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bisect_insort() {
    let code = r#"
import bisect
def f(items: list, x: int) -> list:
    bisect.insort(items, x)
    return items
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Textwrap module
// ============================================================================

#[test]
fn test_textwrap_wrap() {
    let code = r#"
import textwrap
def f(text: str) -> list:
    return textwrap.wrap(text, 40)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_textwrap_fill() {
    let code = r#"
import textwrap
def f(text: str) -> str:
    return textwrap.fill(text, 40)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_textwrap_dedent() {
    let code = r#"
import textwrap
def f(text: str) -> str:
    return textwrap.dedent(text)
"#;
    assert!(transpile_ok(code));
}
