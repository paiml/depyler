//! Coverage tests for expr_gen.rs and related expression code generation paths
//!
//! DEPYLER-99MODE-S8: Session 8 Batch 6 - targeting expression generation
//! coverage gaps including int-to-float coercion, walrus operators, string ops,
//! lambda expressions, and complex expression patterns.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ── Int-to-Float Coercion (DEPYLER-0582, DEPYLER-0694, DEPYLER-0805) ────

#[test]
fn test_int_literal_with_float_param() {
    let code = transpile(
        r#"
def f(x: float) -> float:
    return x + 1
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_int_literal_subtraction_with_float() {
    let code = transpile(
        r#"
def f(beta: float) -> float:
    return 1 - beta
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_float_multiplication_with_int() {
    let code = transpile(
        r#"
def f(rate: float) -> float:
    return rate * 100
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_int_division_produces_float() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> float:
    return a / b
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_mixed_int_float_expression() {
    let code = transpile(
        r#"
def f(x: float, n: int) -> float:
    return x * n + 1
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Walrus Operator (:=) (DEPYLER-0792) ────────────────────────────────

#[test]
fn test_walrus_in_while_condition() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    total = 0
    while (n := len(items)) > 0:
        total = total + n
    return total
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_walrus_in_if_condition() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    if (n := len(s)) > 10:
        return n
    return 0
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── String Operations ──────────────────────────────────────────────────

#[test]
fn test_string_format_method() {
    let code = transpile(
        r#"
def f(name: str, age: int) -> str:
    return "Hello {} age {}".format(name, age)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_join() {
    let code = transpile(
        r#"
def f(items: list[str]) -> str:
    return ", ".join(items)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_split() {
    let code = transpile(
        r#"
def f(s: str) -> list[str]:
    return s.split(",")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_strip() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.strip()
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_replace() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.replace("old", "new")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_startswith() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.startswith("prefix")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_endswith() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.endswith("suffix")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_find() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return s.find("needle")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_count() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return s.count("a")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_isdigit() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.isdigit()
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_isalpha() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.isalpha()
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Lambda Expressions (DEPYLER-1053) ──────────────────────────────────

#[test]
fn test_lambda_in_filter() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    return list(filter(lambda x: x > 0, items))
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_lambda_in_map() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    return list(map(lambda x: x * 2, items))
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_lambda_in_sorted() {
    let code = transpile(
        r#"
def f(items: list[str]) -> list[str]:
    return sorted(items, key=lambda x: len(x))
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Complex Expression Patterns ────────────────────────────────────────

#[test]
fn test_nested_function_calls() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_chained_comparisons() {
    let code = transpile(
        r#"
def f(x: int) -> bool:
    return 0 < x < 100
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_ternary_with_function_call() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return str(x) if x > 0 else "negative"
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_nested_ternary() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return "positive" if x > 0 else ("zero" if x == 0 else "negative")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_list_comprehension_with_method_call() {
    let code = transpile(
        r#"
def f(words: list[str]) -> list[str]:
    return [w.upper() for w in words]
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_dict_comprehension() {
    let code = transpile(
        r#"
def f(keys: list[str]) -> dict[str, int]:
    return {k: len(k) for k in keys}
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_set_comprehension() {
    let code = transpile(
        r#"
def f(items: list[int]) -> set[int]:
    return {x * 2 for x in items}
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_multiple_assignment() {
    let code = transpile(
        r#"
def f() -> int:
    x, y = 1, 2
    return x + y
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_tuple_unpacking_in_for() {
    let code = transpile(
        r#"
def f(pairs: list[tuple]) -> int:
    total = 0
    for k, v in pairs:
        total = total + v
    return total
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Type Conversions ────────────────────────────────────────────────────

#[test]
fn test_int_to_str() {
    let code = transpile(
        r#"
def f(n: int) -> str:
    return str(n)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_str_to_int() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return int(s)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_float_to_int() {
    let code = transpile(
        r#"
def f(x: float) -> int:
    return int(x)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_int_to_float() {
    let code = transpile(
        r#"
def f(n: int) -> float:
    return float(n)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_bool_to_int() {
    let code = transpile(
        r#"
def f(b: bool) -> int:
    return int(b)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Collection Operations ───────────────────────────────────────────────

#[test]
fn test_list_append() {
    let code = transpile(
        r#"
def f() -> list[int]:
    result: list[int] = []
    result.append(1)
    result.append(2)
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_list_extend() {
    let code = transpile(
        r#"
def f(a: list[int], b: list[int]) -> list[int]:
    a.extend(b)
    return a
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_list_pop() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    return items.pop()
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_list_insert() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    items.insert(0, 99)
    return items
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_list_reverse() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    items.reverse()
    return items
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_list_sort() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    items.sort()
    return items
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_dict_get() {
    let code = transpile(
        r#"
def f(d: dict[str, int], key: str) -> int:
    return d.get(key, 0)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_dict_keys() {
    let code = transpile(
        r#"
def f(d: dict[str, int]) -> list[str]:
    return list(d.keys())
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_dict_values() {
    let code = transpile(
        r#"
def f(d: dict[str, int]) -> list[int]:
    return list(d.values())
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_dict_items_iteration() {
    let code = transpile(
        r#"
def f(d: dict[str, int]) -> int:
    total = 0
    for k, v in d.items():
        total = total + v
    return total
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_set_add() {
    let code = transpile(
        r#"
def f() -> set[int]:
    s: set[int] = set()
    s.add(1)
    s.add(2)
    return s
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_set_remove() {
    let code = transpile(
        r#"
def f(s: set[int]) -> set[int]:
    s.remove(1)
    return s
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Built-in Functions ──────────────────────────────────────────────────

#[test]
fn test_len_of_string() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return len(s)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_len_of_list() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    return len(items)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_abs_of_int() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    return abs(x)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_abs_of_float() {
    let code = transpile(
        r#"
def f(x: float) -> float:
    return abs(x)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_min_max() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return max(min(a, b), 0)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_sum_builtin() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    return sum(items)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_any_all_builtins() {
    let code = transpile(
        r#"
def f(items: list[bool]) -> bool:
    return any(items) and all(items)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_print_function() {
    let code = transpile(
        r#"
def f(x: int) -> None:
    print(x)
    print("hello", x)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Attribute Access ────────────────────────────────────────────────────

#[test]
fn test_class_attribute_access() {
    let code = transpile(
        r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def distance(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#,
    );
    assert!(code.contains("Point") || code.contains("fn"), "code: {code}");
}

#[test]
fn test_class_method_with_self() {
    let code = transpile(
        r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self) -> int:
        self.count = self.count + 1
        return self.count

    def reset(self) -> None:
        self.count = 0
"#,
    );
    assert!(code.contains("Counter") || code.contains("fn"), "code: {code}");
}

// ── Complex Patterns ────────────────────────────────────────────────────

#[test]
fn test_nested_comprehension() {
    let code = transpile(
        r#"
def f(matrix: list[list[int]]) -> list[int]:
    return [x for row in matrix for x in row]
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_multiple_return_values_tuple() {
    let code = transpile(
        r#"
def f(x: int) -> tuple:
    return (x, x * 2, x * 3)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_boolean_short_circuit() {
    let code = transpile(
        r#"
def f(x: int, y: int) -> bool:
    return x > 0 and y > 0 or x == y
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_none_comparison() {
    let code = transpile(
        r#"
from typing import Optional
def f(x: Optional[int]) -> bool:
    return x is None
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_not_none_comparison() {
    let code = transpile(
        r#"
from typing import Optional
def f(x: Optional[int]) -> bool:
    return x is not None
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_in_operator_string() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return "hello" in s
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_not_in_operator() {
    let code = transpile(
        r#"
def f(items: list[int], x: int) -> bool:
    return x not in items
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_multiplication() {
    let code = transpile(
        r#"
def f(s: str, n: int) -> str:
    return s * n
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_negative_indexing() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    return items[-1]
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_slice_basic() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    return items[1:3]
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_slice_with_step() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    return items[::2]
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_assert_with_message() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    assert x > 0, "x must be positive"
    return x
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_multiline_function_with_complex_logic() {
    let code = transpile(
        r#"
def fibonacci(n: int) -> list[int]:
    if n <= 0:
        return []
    result: list[int] = [0, 1]
    for i in range(2, n):
        result.append(result[i - 1] + result[i - 2])
    return result
"#,
    );
    assert!(code.contains("fn fibonacci"), "code: {code}");
}

#[test]
fn test_binary_search() {
    let code = transpile(
        r#"
def binary_search(arr: list[int], target: int) -> int:
    low = 0
    high = len(arr) - 1
    while low <= high:
        mid = (low + high) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1
"#,
    );
    assert!(code.contains("fn binary_search"), "code: {code}");
}

#[test]
fn test_matrix_transpose() {
    let code = transpile(
        r#"
def transpose(matrix: list[list[int]]) -> list[list[int]]:
    rows = len(matrix)
    cols = len(matrix[0])
    result: list[list[int]] = []
    for j in range(cols):
        row: list[int] = []
        for i in range(rows):
            row.append(matrix[i][j])
        result.append(row)
    return result
"#,
    );
    assert!(code.contains("fn transpose"), "code: {code}");
}

#[test]
fn test_global_constant() {
    let code = transpile(
        r#"
MAX_SIZE = 100

def f(x: int) -> int:
    if x > MAX_SIZE:
        return MAX_SIZE
    return x
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_formatting_fstring_complex() {
    let code = transpile(
        r#"
def f(name: str, score: int) -> str:
    return f"Player {name} scored {score} points"
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_multiple_comparisons() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    if x == 1:
        return "one"
    elif x == 2:
        return "two"
    elif x == 3:
        return "three"
    else:
        return "other"
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_exception_handling_with_return() {
    let code = transpile(
        r#"
def safe_div(a: int, b: int) -> float:
    try:
        return a / b
    except ZeroDivisionError:
        return 0.0
"#,
    );
    assert!(code.contains("fn safe_div"), "code: {code}");
}

#[test]
fn test_list_membership_check() {
    let code = transpile(
        r#"
def f(x: int) -> bool:
    valid = [1, 2, 3, 4, 5]
    return x in valid
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_methods_chain() {
    let code = transpile(
        r#"
def normalize(s: str) -> str:
    return s.lower().strip()
"#,
    );
    assert!(code.contains("fn normalize"), "code: {code}");
}

#[test]
fn test_enumerate_with_index() {
    let code = transpile(
        r#"
def f(items: list[str]) -> dict[int, str]:
    result: dict[int, str] = {}
    for i, item in enumerate(items):
        result[i] = item
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_zip_two_lists() {
    let code = transpile(
        r#"
def f(names: list[str], ages: list[int]) -> list[str]:
    result: list[str] = []
    for name, age in zip(names, ages):
        result.append(name)
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_power_operator() {
    let code = transpile(
        r#"
def f(x: float, n: int) -> float:
    return x ** n
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_bitwise_operations() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return (a & b) | (a ^ b)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_augmented_assign_in_loop() {
    let code = transpile(
        r#"
def f(n: int) -> int:
    result = 0
    for i in range(n):
        result += i * i
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_continue_break_in_loop() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    total = 0
    for x in items:
        if x < 0:
            continue
        if x > 100:
            break
        total += x
    return total
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}
