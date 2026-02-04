//! Coverage tests for rust_gen/expr_gen.rs
//!
//! DEPYLER-99MODE-001: Targets expr_gen.rs (12,108 lines)
//! Covers: literals, variables, binary/unary ops, index/slice,
//! collections, comprehensions, method calls, builtins, lambda,
//! f-strings, ternary, walrus, generator expressions.

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
// Literal expressions
// ============================================================================

#[test]
fn test_expr_gen_literal_int() {
    let code = "def f() -> int:\n    return 42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_literal_float() {
    let code = "def f() -> float:\n    return 3.14\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_literal_string() {
    let code = "def f() -> str:\n    return \"hello\"\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_literal_bool_true() {
    let code = "def f() -> bool:\n    return True\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_literal_bool_false() {
    let code = "def f() -> bool:\n    return False\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_literal_none() {
    let code = "def f():\n    return None\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable expressions
// ============================================================================

#[test]
fn test_expr_gen_variable_simple() {
    let code = "def f(x: int) -> int:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_variable_reassign() {
    let code = "def f(x: int) -> int:\n    y = x + 1\n    return y\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Binary operations
// ============================================================================

#[test]
fn test_expr_gen_binop_add() {
    let code = "def f(a: int, b: int) -> int:\n    return a + b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_sub() {
    let code = "def f(a: int, b: int) -> int:\n    return a - b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_mul() {
    let code = "def f(a: int, b: int) -> int:\n    return a * b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_div() {
    let code = "def f(a: float, b: float) -> float:\n    return a / b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_floor_div() {
    let code = "def f(a: int, b: int) -> int:\n    return a // b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_modulo() {
    let code = "def f(a: int, b: int) -> int:\n    return a % b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_power() {
    let code = "def f(a: int, b: int) -> int:\n    return a ** b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_bitwise_and() {
    let code = "def f(a: int, b: int) -> int:\n    return a & b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_bitwise_or() {
    let code = "def f(a: int, b: int) -> int:\n    return a | b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_bitwise_xor() {
    let code = "def f(a: int, b: int) -> int:\n    return a ^ b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_lshift() {
    let code = "def f(a: int, b: int) -> int:\n    return a << b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_rshift() {
    let code = "def f(a: int, b: int) -> int:\n    return a >> b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_bool_and() {
    let code = "def f(a: bool, b: bool) -> bool:\n    return a and b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_bool_or() {
    let code = "def f(a: bool, b: bool) -> bool:\n    return a or b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_binop_mixed_int_float() {
    let code = "def f(a: int, b: float) -> float:\n    return a + b\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Unary operations
// ============================================================================

#[test]
fn test_expr_gen_unary_neg() {
    let code = "def f(x: int) -> int:\n    return -x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_unary_not() {
    let code = "def f(x: bool) -> bool:\n    return not x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_unary_pos() {
    let code = "def f(x: int) -> int:\n    return +x\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Comparison operations
// ============================================================================

#[test]
fn test_expr_gen_compare_eq() {
    let code = "def f(a: int, b: int) -> bool:\n    return a == b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_compare_ne() {
    let code = "def f(a: int, b: int) -> bool:\n    return a != b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_compare_lt() {
    let code = "def f(a: int, b: int) -> bool:\n    return a < b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_compare_gt() {
    let code = "def f(a: int, b: int) -> bool:\n    return a > b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_compare_le() {
    let code = "def f(a: int, b: int) -> bool:\n    return a <= b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_compare_ge() {
    let code = "def f(a: int, b: int) -> bool:\n    return a >= b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_compare_in() {
    let code = "def f(x: int, items: list) -> bool:\n    return x in items\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_compare_not_in() {
    let code = "def f(x: int, items: list) -> bool:\n    return x not in items\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_compare_is_none() {
    let code = "def f(x) -> bool:\n    return x is None\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_compare_is_not_none() {
    let code = "def f(x) -> bool:\n    return x is not None\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Index / Subscript
// ============================================================================

#[test]
fn test_expr_gen_index_list() {
    let code = "def f(items: list) -> int:\n    return items[0]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_index_dict() {
    let code = "def f(d: dict) -> str:\n    return d[\"key\"]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_index_negative() {
    let code = "def f(items: list) -> int:\n    return items[-1]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_index_string() {
    let code = "def f(s: str) -> str:\n    return s[0]\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Slicing
// ============================================================================

#[test]
fn test_expr_gen_slice_basic() {
    let code = "def f(items: list) -> list:\n    return items[1:3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_slice_from_start() {
    let code = "def f(items: list) -> list:\n    return items[:3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_slice_to_end() {
    let code = "def f(items: list) -> list:\n    return items[2:]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_slice_string() {
    let code = "def f(s: str) -> str:\n    return s[1:4]\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Collections
// ============================================================================

#[test]
fn test_expr_gen_list_literal() {
    let code = "def f() -> list:\n    return [1, 2, 3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_empty() {
    let code = "def f() -> list:\n    return []\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_dict_literal() {
    let code = "def f() -> dict:\n    return {\"a\": 1, \"b\": 2}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_dict_empty() {
    let code = "def f() -> dict:\n    return {}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_tuple_literal() {
    let code = "def f() -> tuple:\n    return (1, 2, 3)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_set_literal() {
    let code = r#"
def f() -> set:
    return {1, 2, 3}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// List comprehensions
// ============================================================================

#[test]
fn test_expr_gen_list_comp_basic() {
    let code = r#"
def f() -> list:
    return [x * 2 for x in range(10)]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_comp_with_filter() {
    let code = r#"
def f() -> list:
    return [x for x in range(20) if x % 2 == 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_comp_from_list() {
    let code = r#"
def f(items: list) -> list:
    return [x + 1 for x in items]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Dict comprehensions
// ============================================================================

#[test]
fn test_expr_gen_dict_comp() {
    let code = r#"
def f() -> dict:
    return {str(x): x * x for x in range(5)}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Set comprehensions
// ============================================================================

#[test]
fn test_expr_gen_set_comp() {
    let code = r#"
def f(items: list) -> set:
    return {x * 2 for x in items}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Lambda expressions
// ============================================================================

#[test]
fn test_expr_gen_lambda_basic() {
    let code = r#"
def f() -> list:
    items = [3, 1, 4, 1, 5]
    return sorted(items, key=lambda x: x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_lambda_multi_param() {
    let code = r#"
def f() -> list:
    pairs = [(1, 'b'), (2, 'a')]
    return sorted(pairs, key=lambda p: p[1])
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// F-strings
// ============================================================================

#[test]
fn test_expr_gen_fstring_basic() {
    let code = r#"
def f(name: str) -> str:
    return f"Hello, {name}!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_fstring_expr() {
    let code = r#"
def f(x: int) -> str:
    return f"Value is {x + 1}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_fstring_multiple() {
    let code = r#"
def f(a: int, b: int) -> str:
    return f"{a} + {b} = {a + b}"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ternary / conditional expressions
// ============================================================================

#[test]
fn test_expr_gen_ternary() {
    let code = r#"
def f(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_ternary_nested() {
    let code = r#"
def f(x: int) -> str:
    return "positive" if x > 0 else "zero" if x == 0 else "negative"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Method calls - string
// ============================================================================

#[test]
fn test_expr_gen_str_upper() {
    let code = "def f(s: str) -> str:\n    return s.upper()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_lower() {
    let code = "def f(s: str) -> str:\n    return s.lower()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_strip() {
    let code = "def f(s: str) -> str:\n    return s.strip()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_split() {
    let code = "def f(s: str) -> list:\n    return s.split()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_join() {
    let code = r#"
def f(items: list) -> str:
    return ", ".join(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_replace() {
    let code = r#"
def f(s: str) -> str:
    return s.replace("old", "new")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_startswith() {
    let code = r#"
def f(s: str) -> bool:
    return s.startswith("pre")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_endswith() {
    let code = r#"
def f(s: str) -> bool:
    return s.endswith(".py")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_find() {
    let code = r#"
def f(s: str) -> int:
    return s.find("x")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_count() {
    let code = r#"
def f(s: str) -> int:
    return s.count("a")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_isdigit() {
    let code = "def f(s: str) -> bool:\n    return s.isdigit()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_isalpha() {
    let code = "def f(s: str) -> bool:\n    return s.isalpha()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_capitalize() {
    let code = "def f(s: str) -> str:\n    return s.capitalize()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_str_title() {
    let code = "def f(s: str) -> str:\n    return s.title()\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Method calls - list
// ============================================================================

#[test]
fn test_expr_gen_list_append() {
    let code = r#"
def f() -> list:
    items = [1, 2]
    items.append(3)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_extend() {
    let code = r#"
def f() -> list:
    items = [1, 2]
    items.extend([3, 4])
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_pop() {
    let code = r#"
def f(items: list) -> int:
    return items.pop()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_insert() {
    let code = r#"
def f() -> list:
    items = [1, 3]
    items.insert(1, 2)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_reverse() {
    let code = r#"
def f() -> list:
    items = [3, 1, 2]
    items.reverse()
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_sort() {
    let code = r#"
def f() -> list:
    items = [3, 1, 2]
    items.sort()
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_clear() {
    let code = r#"
def f():
    items = [1, 2, 3]
    items.clear()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_index() {
    let code = r#"
def f(items: list) -> int:
    return items.index(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_count() {
    let code = r#"
def f(items: list) -> int:
    return items.count(3)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Method calls - dict
// ============================================================================

#[test]
fn test_expr_gen_dict_get() {
    let code = r#"
def f(d: dict) -> int:
    return d.get("key", 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_dict_keys() {
    let code = r#"
def f(d: dict) -> list:
    result = []
    for k in d.keys():
        result.append(k)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_dict_values() {
    let code = r#"
def f(d: dict) -> list:
    result = []
    for v in d.values():
        result.append(v)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_dict_items() {
    let code = r#"
def f(d: dict) -> list:
    result = []
    for k, v in d.items():
        result.append(k)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_dict_update() {
    let code = r#"
def f() -> dict:
    d = {"a": 1}
    d.update({"b": 2})
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_dict_pop() {
    let code = r#"
def f(d: dict) -> int:
    return d.pop("key", 0)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Set methods
// ============================================================================

#[test]
fn test_expr_gen_set_add() {
    let code = r#"
def f() -> set:
    s = {1, 2}
    s.add(3)
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_set_remove() {
    let code = r#"
def f():
    s = {1, 2, 3}
    s.remove(2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_set_discard() {
    let code = r#"
def f():
    s = {1, 2, 3}
    s.discard(4)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Builtin function calls
// ============================================================================

#[test]
fn test_expr_gen_builtin_range() {
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
fn test_expr_gen_builtin_range_start_stop() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(5, 10):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_builtin_range_step() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(0, 20, 2):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_builtin_int_conversion() {
    let code = "def f(s: str) -> int:\n    return int(s)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_builtin_str_conversion() {
    let code = "def f(n: int) -> str:\n    return str(n)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_builtin_float_conversion() {
    let code = "def f(s: str) -> float:\n    return float(s)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_builtin_bool_conversion() {
    let code = "def f(n: int) -> bool:\n    return bool(n)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_builtin_isinstance() {
    let code = "def f(x: int) -> bool:\n    return isinstance(x, int)\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Attribute access
// ============================================================================

#[test]
fn test_expr_gen_attribute_access() {
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

// ============================================================================
// Complex expression patterns
// ============================================================================

#[test]
fn test_expr_gen_chained_method() {
    let code = r#"
def f(s: str) -> str:
    return s.strip().upper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_nested_calls() {
    let code = "def f(x: int) -> int:\n    return abs(min(x, 0))\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_complex_arithmetic() {
    let code = "def f(a: int, b: int, c: int) -> int:\n    return (a + b) * c - a // b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_string_concat() {
    let code = r#"
def f(a: str, b: str) -> str:
    return a + " " + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_string_multiply() {
    let code = r#"
def f(s: str, n: int) -> str:
    return s * n
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_list_multiply() {
    let code = r#"
def f(n: int) -> list:
    return [0] * n
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_multiple_comparisons() {
    let code = "def f(x: int) -> bool:\n    return 0 < x < 100\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Integration patterns
// ============================================================================

#[test]
fn test_expr_gen_fibonacci() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    a = 0
    b = 1
    for i in range(2, n + 1):
        a, b = b, a + b
    return b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_word_count() {
    let code = r#"
def word_count(text: str) -> dict:
    counts = {}
    for word in text.split():
        w = word.lower()
        counts[w] = counts.get(w, 0) + 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_matrix_add() {
    let code = r#"
def add_vectors(a: list, b: list) -> list:
    result = []
    for i in range(len(a)):
        result.append(a[i] + b[i])
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_filter_and_transform() {
    let code = r#"
def process(items: list) -> list:
    result = []
    for item in items:
        if item > 0:
            result.append(item * 2)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_nested_dict_access() {
    let code = r#"
def f(d: dict) -> int:
    x = d["a"]
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_augmented_assign_add() {
    let code = "def f(x: int) -> int:\n    x += 10\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_augmented_assign_sub() {
    let code = "def f(x: int) -> int:\n    x -= 5\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_augmented_assign_mul() {
    let code = "def f(x: int) -> int:\n    x *= 3\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_print_call() {
    let code = "def f(x: int):\n    print(x)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_print_multi_args() {
    let code = r#"
def f(a: int, b: str):
    print(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_map_filter_combo() {
    let code = r#"
def f(items: list) -> list:
    return list(map(lambda x: x * 2, filter(lambda x: x > 0, items)))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_enumerate_loop() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i, val in enumerate(items):
        total += i * val
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_gen_zip_loop() {
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
fn test_expr_gen_reversed_loop() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}
