//! Session 12: Deep coverage tests for expr_gen.rs and stmt_gen.rs codegen paths
//!
//! Targets uncovered paths:
//! - Complex with-statement patterns
//! - Exception handling with as-clause
//! - Multiple except handlers
//! - For-else and while-else patterns
//! - Complex class patterns (properties, classmethods)
//! - Multiple inheritance
//! - Nested class definitions
//! - Complex default arguments
//! - Keyword arguments
//! - *args/**kwargs in function calls
//! - Nested function calls
//! - Multiple assignment targets
//! - Augmented assignment with complex expressions

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

// ===== With statement patterns =====

#[test]
fn test_s12_with_open_read() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s12_with_open_write() {
    let code = r#"
def write_file(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Got: {}", result);
}

#[test]
fn test_s12_with_no_as() {
    let code = r#"
def process_with():
    with open("test.txt"):
        pass
"#;
    let result = transpile(code);
    assert!(result.contains("process_with"), "Got: {}", result);
}

// ===== Exception handling =====

#[test]
fn test_s12_try_except_as() {
    let code = r#"
def safe_int(s: str) -> int:
    try:
        return int(s)
    except ValueError as e:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_int"), "Got: {}", result);
}

#[test]
fn test_s12_try_except_multiple_types() {
    let code = r#"
def safe_op(x: int) -> int:
    try:
        return 100 // x
    except (ZeroDivisionError, ValueError):
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_op"), "Got: {}", result);
}

#[test]
fn test_s12_try_except_bare() {
    let code = r#"
def catch_all() -> int:
    try:
        return 1
    except:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn catch_all"), "Got: {}", result);
}

#[test]
fn test_s12_try_except_else() {
    let code = r#"
def with_else(s: str) -> int:
    try:
        result = int(s)
    except ValueError:
        return -1
    else:
        return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn with_else"), "Got: {}", result);
}

#[test]
fn test_s12_raise_value_error() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

#[test]
fn test_s12_raise_no_argument() {
    let code = r#"
def reraise() -> int:
    try:
        return 1
    except:
        raise
"#;
    let result = transpile(code);
    assert!(result.contains("fn reraise"), "Got: {}", result);
}

// ===== For-else and while-else =====

#[test]
fn test_s12_for_else() {
    let code = r#"
def find_target(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    else:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_target"), "Got: {}", result);
}

#[test]
fn test_s12_while_else() {
    let code = r#"
def count_down(n: int) -> int:
    while n > 0:
        n -= 1
    else:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_down"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_class_str_repr() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __str__(self) -> str:
        return f"Point({self.x}, {self.y})"

    def __repr__(self) -> str:
        return f"Point(x={self.x}, y={self.y})"
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

#[test]
fn test_s12_class_comparison_methods() {
    let code = r#"
class Comparable:
    def __init__(self, value: int):
        self.value = value

    def __lt__(self, other) -> bool:
        return self.value < other.value

    def __le__(self, other) -> bool:
        return self.value <= other.value

    def __gt__(self, other) -> bool:
        return self.value > other.value

    def __ge__(self, other) -> bool:
        return self.value >= other.value
"#;
    let result = transpile(code);
    assert!(result.contains("Comparable"), "Got: {}", result);
}

#[test]
fn test_s12_class_arithmetic_methods() {
    let code = r#"
class Vector:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y)

    def __sub__(self, other):
        return Vector(self.x - other.x, self.y - other.y)

    def __mul__(self, scalar: float):
        return Vector(self.x * scalar, self.y * scalar)
"#;
    let result = transpile(code);
    assert!(result.contains("Vector"), "Got: {}", result);
}

#[test]
fn test_s12_class_container_methods() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def __len__(self) -> int:
        return len(self.items)

    def __bool__(self) -> bool:
        return len(self.items) > 0

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("Stack"), "Got: {}", result);
}

#[test]
fn test_s12_multiple_inheritance() {
    let code = r#"
class Serializable:
    def serialize(self) -> str:
        return "{}"

class Printable:
    def display(self) -> str:
        return "object"

class Entity(Serializable, Printable):
    def __init__(self, name: str):
        self.name = name
"#;
    let result = transpile(code);
    assert!(result.contains("Entity"), "Got: {}", result);
}

// ===== Function argument patterns =====

#[test]
fn test_s12_keyword_args() {
    let code = r#"
def configure(host: str, port: int, debug: bool) -> dict:
    return {"host": host, "port": port, "debug": debug}

def setup():
    return configure(host="localhost", port=8080, debug=True)
"#;
    let result = transpile(code);
    assert!(result.contains("fn configure"), "Got: {}", result);
}

#[test]
fn test_s12_default_args() {
    let code = r#"
def greet(name: str, greeting: str = "Hello") -> str:
    return greeting + " " + name
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_args_kwargs() {
    let code = r#"
def flex(*args, **kwargs):
    return len(args) + len(kwargs)
"#;
    let result = transpile(code);
    assert!(result.contains("flex"), "Got: {}", result);
}

// ===== Complex assignment patterns =====

#[test]
fn test_s12_multiple_assign_same_value() {
    let code = r#"
def init_vars() -> int:
    a = b = c = 0
    return a + b + c
"#;
    let result = transpile(code);
    assert!(result.contains("fn init_vars"), "Got: {}", result);
}

#[test]
fn test_s12_tuple_unpack_flat() {
    let code = r#"
def unpack_pair(x: int, y: int) -> int:
    a, b = x, y
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn unpack_pair"), "Got: {}", result);
}

// ===== Complex control flow =====

#[test]
fn test_s12_nested_loops_with_break() {
    let code = r#"
def find_pair(matrix: list, target: int) -> tuple:
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                return (i, j)
    return (-1, -1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_pair"), "Got: {}", result);
}

#[test]
fn test_s12_pass_statement() {
    let code = r#"
def placeholder():
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("placeholder"), "Got: {}", result);
}

#[test]
fn test_s12_continue_in_nested_loop() {
    let code = r#"
def skip_zeros(matrix: list) -> list:
    result = []
    for row in matrix:
        for val in row:
            if val == 0:
                continue
            result.append(val)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_zeros"), "Got: {}", result);
}

// ===== String operations =====

#[test]
fn test_s12_string_multiplication() {
    let code = r#"
def repeat_str(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn repeat_str"), "Got: {}", result);
}

#[test]
fn test_s12_string_join() {
    let code = r#"
def join_list(items: list) -> str:
    return ", ".join(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_list"), "Got: {}", result);
}

#[test]
fn test_s12_string_split() {
    let code = r#"
def split_csv(line: str) -> list:
    return line.split(",")
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_csv"), "Got: {}", result);
}

#[test]
fn test_s12_string_startswith() {
    let code = r#"
def is_http(url: str) -> bool:
    return url.startswith("http")
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_http"), "Got: {}", result);
}

#[test]
fn test_s12_string_endswith() {
    let code = r#"
def is_python(filename: str) -> bool:
    return filename.endswith(".py")
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_python"), "Got: {}", result);
}

#[test]
fn test_s12_string_replace() {
    let code = r#"
def clean(text: str) -> str:
    return text.replace("\n", " ")
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"), "Got: {}", result);
}

#[test]
fn test_s12_string_strip() {
    let code = r#"
def trim(s: str) -> str:
    return s.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn trim"), "Got: {}", result);
}

#[test]
fn test_s12_string_upper_lower() {
    let code = r#"
def normalize(s: str) -> str:
    return s.lower().strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

// ===== Print and I/O =====

#[test]
fn test_s12_print_simple() {
    let code = r#"
def hello():
    print("Hello, World!")
"#;
    let result = transpile(code);
    assert!(result.contains("fn hello"), "Got: {}", result);
}

#[test]
fn test_s12_print_multiple_args() {
    let code = r#"
def debug(name: str, value: int):
    print(name, value)
"#;
    let result = transpile(code);
    assert!(result.contains("fn debug"), "Got: {}", result);
}

// ===== Type checking patterns =====

#[test]
fn test_s12_isinstance_check() {
    let code = r#"
def is_int(x) -> bool:
    return isinstance(x, int)
"#;
    let result = transpile(code);
    assert!(result.contains("is_int"), "Got: {}", result);
}

// ===== Complex algorithm patterns =====

#[test]
fn test_s12_binary_search() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn binary_search"), "Got: {}", result);
}

#[test]
fn test_s12_bubble_sort() {
    let code = r#"
def bubble_sort(items: list) -> list:
    n = len(items)
    for i in range(n):
        for j in range(0, n - i - 1):
            if items[j] > items[j + 1]:
                items[j], items[j + 1] = items[j + 1], items[j]
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn bubble_sort"), "Got: {}", result);
}

#[test]
fn test_s12_fibonacci_iterative() {
    let code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a = 0
    b = 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b
"#;
    let result = transpile(code);
    assert!(result.contains("fn fibonacci"), "Got: {}", result);
}

#[test]
fn test_s12_gcd() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn gcd"), "Got: {}", result);
}

#[test]
fn test_s12_matrix_multiply() {
    let code = r#"
def dot_product(a: list, b: list) -> int:
    total = 0
    for i in range(len(a)):
        total += a[i] * b[i]
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn dot_product"), "Got: {}", result);
}

#[test]
fn test_s12_flatten_list() {
    let code = r#"
def flatten(nested: list) -> list:
    result = []
    for item in nested:
        if isinstance(item, list):
            result.extend(flatten(item))
        else:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

// ===== Multiple return types =====

#[test]
fn test_s12_return_boolean_expr() {
    let code = r#"
def is_even(n: int) -> bool:
    return n % 2 == 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_even"), "Got: {}", result);
}

#[test]
fn test_s12_return_conditional() {
    let code = r#"
def sign(x: int) -> int:
    if x > 0:
        return 1
    elif x < 0:
        return -1
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn sign"), "Got: {}", result);
}

// ===== Comprehension variants =====

#[test]
fn test_s12_nested_list_comp() {
    let code = r#"
def flatten_2d(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten_2d"), "Got: {}", result);
}

#[test]
fn test_s12_dict_comp_with_condition() {
    let code = r#"
def filter_dict(d: dict) -> dict:
    return {k: v for k, v in d.items() if v > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_dict"), "Got: {}", result);
}
