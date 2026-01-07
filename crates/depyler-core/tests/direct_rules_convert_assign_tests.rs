//! EXTREME TDD: Tests for direct_rules_convert assignment functions
//! Coverage: convert_symbol_assignment, convert_index_assignment, convert_attribute_assignment

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    DepylerPipeline::new().transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

fn transpile_contains(code: &str, needle: &str) -> bool {
    transpile(code).map(|s| s.contains(needle)).unwrap_or(false)
}

// ============ convert_symbol_assignment tests ============

#[test]
fn test_symbol_assign_int() {
    assert!(transpile_ok("def f() -> int:\n    x = 42\n    return x"));
}

#[test]
fn test_symbol_assign_float() {
    assert!(transpile_ok("def f() -> float:\n    x = 3.14\n    return x"));
}

#[test]
fn test_symbol_assign_string() {
    assert!(transpile_ok("def f() -> str:\n    x = \"hello\"\n    return x"));
}

#[test]
fn test_symbol_assign_bool() {
    assert!(transpile_ok("def f() -> bool:\n    x = True\n    return x"));
}

#[test]
fn test_symbol_assign_list() {
    assert!(transpile_ok("def f() -> list:\n    x = [1, 2, 3]\n    return x"));
}

#[test]
fn test_symbol_assign_dict() {
    assert!(transpile_ok("def f() -> dict:\n    x = {\"a\": 1}\n    return x"));
}

#[test]
fn test_symbol_assign_expression() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    x = a + b\n    return x"));
}

#[test]
fn test_symbol_assign_call() {
    assert!(transpile_ok("def f(s: str) -> str:\n    x = s.upper()\n    return x"));
}

#[test]
fn test_symbol_reassign() {
    let code = r#"
def f(x: int) -> int:
    y = x
    y = y + 1
    y = y * 2
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_symbol_assign_from_param() {
    assert!(transpile_ok("def f(x: int) -> int:\n    y = x\n    return y"));
}

#[test]
fn test_symbol_assign_with_type_annotation() {
    let code = r#"
def f() -> int:
    x: int = 42
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_symbol_augmented_assign_add() {
    assert!(transpile_ok("def f(x: int) -> int:\n    x += 1\n    return x"));
}

#[test]
fn test_symbol_augmented_assign_sub() {
    assert!(transpile_ok("def f(x: int) -> int:\n    x -= 1\n    return x"));
}

#[test]
fn test_symbol_augmented_assign_mul() {
    assert!(transpile_ok("def f(x: int) -> int:\n    x *= 2\n    return x"));
}

#[test]
fn test_symbol_augmented_assign_div() {
    assert!(transpile_ok("def f(x: float) -> float:\n    x /= 2.0\n    return x"));
}

// ============ convert_index_assignment tests ============

#[test]
fn test_index_assign_list() {
    let code = r#"
def f(items: list) -> list:
    items[0] = 100
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_assign_dict() {
    let code = r#"
def f(data: dict) -> dict:
    data["key"] = "value"
    return data
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_assign_nested_list() {
    let code = r#"
def f(matrix: list) -> list:
    matrix[0][0] = 1
    return matrix
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_assign_variable_index() {
    let code = r#"
def f(items: list, i: int) -> list:
    items[i] = 0
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_assign_expression_index() {
    let code = r#"
def f(items: list, i: int) -> list:
    items[i + 1] = 0
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_assign_in_loop() {
    let code = r#"
def f(items: list) -> list:
    for i in range(len(items)):
        items[i] = items[i] * 2
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_index_assign_dict_int_key() {
    let code = r#"
def f(data: dict) -> dict:
    data[42] = "answer"
    return data
"#;
    assert!(transpile_ok(code));
}

// ============ convert_attribute_assignment tests ============

#[test]
fn test_attr_assign_simple() {
    let code = r#"
class Point:
    x: int
    y: int

    def set_x(self, val: int) -> None:
        self.x = val
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_attr_assign_multiple() {
    let code = r#"
class Point:
    x: int
    y: int

    def set_coords(self, x: int, y: int) -> None:
        self.x = x
        self.y = y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_attr_assign_from_expression() {
    let code = r#"
class Point:
    x: int

    def double_x(self) -> None:
        self.x = self.x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_attr_assign_in_if() {
    let code = r#"
class Counter:
    count: int

    def maybe_increment(self, should: bool) -> None:
        if should:
            self.count = self.count + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_attr_assign_in_loop() {
    let code = r#"
class Accumulator:
    total: int

    def add_all(self, items: list) -> None:
        for item in items:
            self.total = self.total + item
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_attr_assign_init() {
    let code = r#"
class Person:
    name: str
    age: int

    def __init__(self, name: str, age: int) -> None:
        self.name = name
        self.age = age
"#;
    assert!(transpile_ok(code));
}

// ============ Mixed assignment tests ============

#[test]
fn test_mixed_symbol_and_index() {
    let code = r#"
def f(items: list) -> int:
    x = items[0]
    items[0] = x + 1
    return items[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_chained_assignments() {
    let code = r#"
def f(x: int) -> int:
    a = x
    b = a
    c = b
    return c
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_swap_values() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    temp = a
    a = b
    b = temp
    return (a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_tuple_assignment() {
    let code = r#"
def f(point: tuple) -> int:
    x, y = point
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_multiple_tuple_unpack() {
    let code = r#"
def f(data: tuple) -> int:
    a, b, c = data
    return a + b + c
"#;
    assert!(transpile_ok(code));
}

// ============ Edge cases ============

#[test]
fn test_assign_none() {
    let code = r#"
def f() -> None:
    x = None
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_assign_empty_list() {
    assert!(transpile_ok("def f() -> list:\n    x = []\n    return x"));
}

#[test]
fn test_assign_empty_dict() {
    assert!(transpile_ok("def f() -> dict:\n    x = {}\n    return x"));
}

#[test]
fn test_assign_empty_string() {
    assert!(transpile_ok("def f() -> str:\n    x = \"\"\n    return x"));
}

#[test]
fn test_assign_negative_int() {
    assert!(transpile_ok("def f() -> int:\n    x = -42\n    return x"));
}

#[test]
fn test_assign_scientific_notation() {
    assert!(transpile_ok("def f() -> float:\n    x = 1e10\n    return x"));
}

#[test]
fn test_assign_boolean_expression() {
    let code = r#"
def f(a: int, b: int) -> bool:
    x = a > b
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_assign_ternary() {
    let code = r#"
def f(x: int) -> int:
    y = x if x > 0 else -x
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_assign_list_comprehension() {
    let code = r#"
def f(items: list) -> list:
    doubled = [x * 2 for x in items]
    return doubled
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_assign_dict_comprehension() {
    let code = r#"
def f(items: list) -> dict:
    result = {i: i * 2 for i in items}
    return result
"#;
    assert!(transpile_ok(code));
}
