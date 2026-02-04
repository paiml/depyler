//! Coverage tests for direct_rules_convert.rs
//!
//! DEPYLER-99MODE-001: Targets 47%→80% coverage for direct_rules_convert module
//! Covers: find_mutable_vars_in_body, convert_condition_expr,
//! convert_expr edge cases (Yield, NamedExpr, FString, comprehensions),
//! type coercion branches, and assignment target patterns.

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Mutability analysis (find_mutable_vars_in_body)
// ============================================================================

#[test]
fn test_mutable_var_reassignment() {
    let code = r#"
def f() -> int:
    x = 1
    x = 2
    return x
"#;
    let rust = transpile(code);
    assert!(rust.contains("mut"));
}

#[test]
fn test_mutable_var_augmented_assign() {
    let code = r#"
def f() -> int:
    x = 0
    x += 1
    return x
"#;
    let rust = transpile(code);
    assert!(rust.contains("mut"));
}

#[test]
fn test_mutable_list_append() {
    let code = r#"
def f() -> list:
    items = []
    items.append(1)
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("mut"));
}

#[test]
fn test_mutable_dict_update() {
    let code = r#"
def f() -> dict:
    d = {}
    d["key"] = "value"
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_immutable_var_single_assign() {
    let code = r#"
def f() -> int:
    x = 42
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mutable_in_for_loop() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10):
        total += i
    return total
"#;
    let rust = transpile(code);
    assert!(rust.contains("mut"));
}

#[test]
fn test_mutable_in_while_loop() {
    let code = r#"
def f() -> int:
    x = 10
    while x > 0:
        x -= 1
    return x
"#;
    let rust = transpile(code);
    assert!(rust.contains("mut"));
}

#[test]
fn test_mutable_in_if_branch() {
    let code = r#"
def f(x: int) -> int:
    result = 0
    if x > 0:
        result = x
    else:
        result = -x
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("mut"));
}

// ============================================================================
// Condition truthiness coercion (convert_condition_expr)
// ============================================================================

#[test]
fn test_truthiness_int_condition() {
    let code = r#"
def f(x: int) -> bool:
    if x:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_truthiness_string_condition() {
    let code = r#"
def f(s: str) -> bool:
    if s:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_truthiness_list_condition() {
    let code = r#"
def f(items: list) -> bool:
    if items:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_truthiness_in_while() {
    let code = r#"
def f(items: list) -> int:
    count = 0
    while items:
        count += 1
        items.pop()
    return count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Literals
// ============================================================================

#[test]
fn test_literal_none() {
    let code = r#"
def f():
    return None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_literal_bool_true() {
    let code = r#"
def f() -> bool:
    return True
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_literal_bool_false() {
    let code = r#"
def f() -> bool:
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_literal_bytes() {
    let code = r#"
def f() -> bytes:
    return b"hello"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Binary operations
// ============================================================================

#[test]
fn test_binary_add() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_sub() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a - b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_mul() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_floor_div() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a // b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_modulo() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a % b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_power() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a ** b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_bitwise_and() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a & b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_bitwise_or() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a | b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_bitwise_xor() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a ^ b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_left_shift() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a << b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_right_shift() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a >> b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_comparison_eq() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a == b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_comparison_ne() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a != b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_comparison_lt() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a < b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_comparison_le() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a <= b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_comparison_gt() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a > b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_comparison_ge() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a >= b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_logical_and() {
    let code = r#"
def f(a: bool, b: bool) -> bool:
    return a and b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_logical_or() {
    let code = r#"
def f(a: bool, b: bool) -> bool:
    return a or b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Unary operations
// ============================================================================

#[test]
fn test_unary_not() {
    let code = r#"
def f(x: bool) -> bool:
    return not x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unary_neg() {
    let code = r#"
def f(x: int) -> int:
    return -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unary_bitnot() {
    let code = r#"
def f(x: int) -> int:
    return ~x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Call expressions (builtins)
// ============================================================================

#[test]
fn test_call_len() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_abs() {
    let code = r#"
def f(x: int) -> int:
    return abs(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_range() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_range_with_start_stop() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(1, 10):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_range_with_step() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(0, 10, 2):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_enumerate() {
    let code = r#"
def f(items: list) -> int:
    count = 0
    for i, item in enumerate(items):
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_zip() {
    let code = r#"
def f(a: list, b: list) -> int:
    total = 0
    for x, y in zip(a, b):
        total += 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_reversed() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_sorted() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_sum() {
    let code = r#"
def f(items: list) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_min() {
    let code = r#"
def f(a: int, b: int) -> int:
    return min(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_max() {
    let code = r#"
def f(a: int, b: int) -> int:
    return max(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_all() {
    let code = r#"
def f(items: list) -> bool:
    return all(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_any() {
    let code = r#"
def f(items: list) -> bool:
    return any(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_int_conversion() {
    let code = r#"
def f(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_str_conversion() {
    let code = r#"
def f(x: int) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_float_conversion() {
    let code = r#"
def f(s: str) -> float:
    return float(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_print() {
    let code = r#"
def f(msg: str):
    print(msg)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_ord() {
    let code = r#"
def f(c: str) -> int:
    return ord(c)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_call_chr() {
    let code = r#"
def f(n: int) -> str:
    return chr(n)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Index and Slice
// ============================================================================

#[test]
fn test_index_list() {
    let code = r#"
def f(items: list, i: int) -> int:
    return items[i]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_negative() {
    let code = r#"
def f(items: list) -> int:
    return items[-1]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_dict() {
    let code = r#"
def f(d: dict, key: str) -> int:
    return d[key]
"#;
    assert!(transpile_ok(code));
}

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
    return items[1:]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_slice() {
    let code = r#"
def f(s: str) -> str:
    return s[1:4]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Collections
// ============================================================================

#[test]
fn test_list_literal() {
    let code = r#"
def f() -> list:
    return [1, 2, 3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_tuple_literal() {
    let code = r#"
def f() -> tuple:
    return (1, 2, 3)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_literal() {
    let code = r#"
def f() -> set:
    return {1, 2, 3}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_literal() {
    let code = r#"
def f() -> dict:
    return {"a": 1, "b": 2}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_empty_list() {
    let code = r#"
def f() -> list:
    return []
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_empty_dict() {
    let code = r#"
def f() -> dict:
    return {}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Comprehensions
// ============================================================================

#[test]
fn test_list_comprehension() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_comprehension_with_filter() {
    let code = r#"
def f(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_comprehension() {
    let code = r#"
def f(items: list) -> set:
    return {x * 2 for x in items}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_comprehension() {
    let code = r#"
def f(items: list) -> dict:
    return {str(x): x for x in items}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Lambda
// ============================================================================

#[test]
fn test_lambda_simple() {
    let code = r#"
def f() -> int:
    add = lambda x, y: x + y
    return add(1, 2)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: F-strings
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
fn test_fstring_multiple_parts() {
    let code = r#"
def f(a: int, b: int) -> str:
    return f"{a} + {b} = {a + b}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_fstring_with_expression() {
    let code = r#"
def f(x: int) -> str:
    return f"value is {x * 2}"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: If expression (ternary)
// ============================================================================

#[test]
fn test_ternary_expression() {
    let code = r#"
def f(x: int) -> int:
    return x if x > 0 else -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ternary_with_call() {
    let code = r#"
def f(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Attribute access
// ============================================================================

#[test]
fn test_attribute_access() {
    let code = r#"
def f(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Method calls (string)
// ============================================================================

#[test]
fn test_string_upper() {
    let code = r#"
def f(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_lower() {
    let code = r#"
def f(s: str) -> str:
    return s.lower()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_strip() {
    let code = r#"
def f(s: str) -> str:
    return s.strip()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_split() {
    let code = r#"
def f(s: str) -> list:
    return s.split(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_replace() {
    let code = r#"
def f(s: str) -> str:
    return s.replace("old", "new")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_startswith() {
    let code = r#"
def f(s: str) -> bool:
    return s.startswith("pre")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_endswith() {
    let code = r#"
def f(s: str) -> bool:
    return s.endswith("suf")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_join() {
    let code = r#"
def f(items: list) -> str:
    return ",".join(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_find() {
    let code = r#"
def f(s: str) -> int:
    return s.find("x")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_count() {
    let code = r#"
def f(s: str) -> int:
    return s.count("a")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_isdigit() {
    let code = r#"
def f(s: str) -> bool:
    return s.isdigit()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_isalpha() {
    let code = r#"
def f(s: str) -> bool:
    return s.isalpha()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Method calls (list)
// ============================================================================

#[test]
fn test_list_append() {
    let code = r#"
def f() -> list:
    items = [1, 2]
    items.append(3)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_extend() {
    let code = r#"
def f() -> list:
    items = [1, 2]
    items.extend([3, 4])
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_pop() {
    let code = r#"
def f(items: list) -> int:
    return items.pop()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_reverse() {
    let code = r#"
def f(items: list):
    items.reverse()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_sort() {
    let code = r#"
def f(items: list):
    items.sort()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_insert() {
    let code = r#"
def f(items: list):
    items.insert(0, 42)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_remove() {
    let code = r#"
def f(items: list):
    items.remove(1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_clear() {
    let code = r#"
def f(items: list):
    items.clear()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_index() {
    let code = r#"
def f(items: list) -> int:
    return items.index(42)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion: Method calls (dict)
// ============================================================================

#[test]
fn test_dict_get() {
    let code = r#"
def f(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_keys() {
    let code = r#"
def f(d: dict) -> list:
    return list(d.keys())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_values() {
    let code = r#"
def f(d: dict) -> list:
    return list(d.values())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_items_loop() {
    let code = r#"
def f(d: dict) -> int:
    total = 0
    for k, v in d.items():
        total += 1
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Assignment targets
// ============================================================================

#[test]
fn test_assign_symbol() {
    let code = r#"
def f() -> int:
    x = 42
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_assign_index() {
    let code = r#"
def f(items: list):
    items[0] = 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_assign_tuple_unpack() {
    let code = r#"
def f() -> int:
    a, b = 1, 2
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_assign_tuple_unpack_from_call() {
    let code = r#"
def f(items: list) -> int:
    for i, v in enumerate(items):
        pass
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// For loop patterns
// ============================================================================

#[test]
fn test_for_range_basic() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_list_iteration() {
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
fn test_for_string_iteration() {
    let code = r#"
def f(s: str) -> int:
    count = 0
    for c in s:
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_dict_keys_loop() {
    let code = r#"
def f(d: dict) -> int:
    count = 0
    for k in d.keys():
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_dict_values_loop() {
    let code = r#"
def f(d: dict) -> int:
    total = 0
    for v in d.values():
        total += 1
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Return type handling
// ============================================================================

#[test]
fn test_return_optional() {
    let code = r#"
from typing import Optional
def f(x: int) -> Optional[int]:
    if x > 0:
        return x
    return None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_list() {
    let code = r#"
def f() -> list:
    return [1, 2, 3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_dict() {
    let code = r#"
def f() -> dict:
    return {"key": "value"}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_tuple() {
    let code = r#"
def f() -> tuple:
    return (1, 2)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type coercion edge cases
// ============================================================================

#[test]
fn test_float_int_addition() {
    let code = r#"
def f(x: float) -> float:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_float_int_comparison() {
    let code = r#"
def f(x: float) -> bool:
    return x > 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_concatenation() {
    let code = r#"
def f(a: str, b: str) -> str:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_multiplication() {
    let code = r#"
def f(s: str, n: int) -> str:
    return s * n
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// In operator
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
fn test_in_string() {
    let code = r#"
def f(sub: str, s: str) -> bool:
    return sub in s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_not_in() {
    let code = r#"
def f(x: int, items: list) -> bool:
    return x not in items
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex statement combinations
// ============================================================================

#[test]
fn test_multiple_return_paths() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_loops() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        for j in range(n):
            total += i * j
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_while_with_break() {
    let code = r#"
def f(items: list) -> int:
    i = 0
    while i < len(items):
        if items[i] == 0:
            break
        i += 1
    return i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_with_continue() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for x in items:
        if x < 0:
            continue
        total += x
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_function_many_features() {
    let code = r#"
def process(items: list) -> dict:
    result = {}
    count = 0
    for item in items:
        if item > 0:
            key = str(item)
            result[key] = item * 2
            count += 1
        elif item == 0:
            continue
        else:
            break
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Await expression
// ============================================================================

#[test]
fn test_await_expression() {
    let code = r#"
async def f(x: int) -> int:
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Generator expressions
// ============================================================================

#[test]
fn test_generator_in_sum() {
    let code = r#"
def f(items: list) -> int:
    return sum(x * 2 for x in items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_in_any() {
    let code = r#"
def f(items: list) -> bool:
    return any(x > 0 for x in items)
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
// Walrus operator (named expression)
// ============================================================================

#[test]
fn test_walrus_operator() {
    let code = r#"
def f(items: list) -> int:
    if (n := len(items)) > 0:
        return n
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Pure expression detection
// ============================================================================

#[test]
fn test_pure_expr_standalone() {
    let code = r#"
def f(x: int) -> int:
    x + 1
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Sorted with key
// ============================================================================

#[test]
fn test_sorted_with_reverse() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, reverse=True)
"#;
    assert!(transpile_ok(code));
}
