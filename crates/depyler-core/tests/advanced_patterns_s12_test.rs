//! Session 12 Batch 20: Advanced Python patterns for maximum coverage
//!
//! Targets rarely-exercised code paths:
//! - Async/await patterns
//! - Complex decorator combinations
//! - Multiple inheritance
//! - Abstract methods
//! - Property getter/setter patterns
//! - Context manager __enter__/__exit__
//! - Complex string operations (maketrans, template)
//! - Dataclass-like patterns
//! - Enum-like patterns
//! - Complex conditional import patterns
//! - Complex dict/list manipulation algorithms
//! - Complex nested comprehensions

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ===== Async/await patterns =====

#[test]
fn test_s12_async_function() {
    let code = r#"
async def fetch_data(url: str) -> str:
    return url
"#;
    let result = transpile(code);
    assert!(result.contains("fetch_data"), "Got: {}", result);
}

#[test]
fn test_s12_async_with_await() {
    let code = r#"
async def process_url(url: str) -> str:
    data = await fetch(url)
    return data
"#;
    let result = transpile(code);
    assert!(result.contains("process_url"), "Got: {}", result);
}

// ===== Advanced class patterns =====

#[test]
fn test_s12_class_with_properties() {
    let code = r#"
class Rectangle:
    def __init__(self, width: float, height: float):
        self._width = width
        self._height = height

    @property
    def area(self) -> float:
        return self._width * self._height

    @property
    def perimeter(self) -> float:
        return 2 * (self._width + self._height)
"#;
    let result = transpile(code);
    assert!(result.contains("Rectangle"), "Got: {}", result);
}

#[test]
fn test_s12_abstract_base_class() {
    let code = r#"
class Shape:
    def area(self) -> float:
        raise NotImplementedError

    def perimeter(self) -> float:
        raise NotImplementedError

class Square(Shape):
    def __init__(self, side: float):
        self.side = side

    def area(self) -> float:
        return self.side * self.side

    def perimeter(self) -> float:
        return 4 * self.side
"#;
    let result = transpile(code);
    assert!(result.contains("Shape"), "Got: {}", result);
    assert!(result.contains("Square"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_dunder_methods() {
    let code = r#"
class Vector:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __add__(self, other) -> object:
        return Vector(self.x + other.x, self.y + other.y)

    def __sub__(self, other) -> object:
        return Vector(self.x - other.x, self.y - other.y)

    def __mul__(self, scalar: float) -> object:
        return Vector(self.x * scalar, self.y * scalar)

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"

    def magnitude(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("Vector"), "Got: {}", result);
}

// ===== Enum-like patterns =====

#[test]
fn test_s12_enum_class() {
    let code = r#"
class Color:
    RED = 1
    GREEN = 2
    BLUE = 3

    def __init__(self, value: int):
        self.value = value

    def name(self) -> str:
        if self.value == 1:
            return "Red"
        elif self.value == 2:
            return "Green"
        elif self.value == 3:
            return "Blue"
        return "Unknown"
"#;
    let result = transpile(code);
    assert!(result.contains("Color"), "Got: {}", result);
}

// ===== Dataclass-like patterns =====

#[test]
fn test_s12_dataclass_pattern() {
    let code = r#"
class Person:
    def __init__(self, name: str, age: int, email: str):
        self.name = name
        self.age = age
        self.email = email

    def __eq__(self, other) -> bool:
        return self.name == other.name and self.age == other.age

    def __str__(self) -> str:
        return f"{self.name} ({self.age})"

    def to_dict(self) -> dict:
        return {"name": self.name, "age": self.age, "email": self.email}
"#;
    let result = transpile(code);
    assert!(result.contains("Person"), "Got: {}", result);
    assert!(result.contains("to_dict"), "Got: {}", result);
}

// ===== Complex comprehensions =====

#[test]
fn test_s12_nested_list_comprehension() {
    let code = r#"
def multiplication_table(n: int) -> list:
    return [[i * j for j in range(1, n + 1)] for i in range(1, n + 1)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn multiplication_table"), "Got: {}", result);
}

#[test]
fn test_s12_dict_from_comprehension() {
    let code = r#"
def char_positions(s: str) -> dict:
    positions = {}
    for i, c in enumerate(s):
        if c not in positions:
            positions[c] = []
        positions[c].append(i)
    return positions
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_positions"), "Got: {}", result);
}

// ===== Complex string algorithms =====

#[test]
fn test_s12_levenshtein_distance() {
    let code = r#"
def edit_distance(s1: str, s2: str) -> int:
    m = len(s1)
    n = len(s2)
    dp = []
    for i in range(m + 1):
        row = []
        for j in range(n + 1):
            if i == 0:
                row.append(j)
            elif j == 0:
                row.append(i)
            else:
                row.append(0)
        dp.append(row)
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1]
            else:
                dp[i][j] = 1 + min(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1])
    return dp[m][n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn edit_distance"), "Got: {}", result);
}

#[test]
fn test_s12_anagram_check() {
    let code = r#"
def is_anagram(s1: str, s2: str) -> bool:
    if len(s1) != len(s2):
        return False
    count = {}
    for c in s1.lower():
        if c in count:
            count[c] += 1
        else:
            count[c] = 1
    for c in s2.lower():
        if c not in count:
            return False
        count[c] -= 1
        if count[c] < 0:
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_anagram"), "Got: {}", result);
}

// ===== Complex graph algorithms =====

#[test]
fn test_s12_bfs_algorithm() {
    let code = r#"
def bfs(graph: dict, start: str) -> list:
    visited = set()
    queue = [start]
    result = []
    while queue:
        node = queue[0]
        queue = queue[1:]
        if node not in visited:
            visited.add(node)
            result.append(node)
            if node in graph:
                for neighbor in graph[node]:
                    if neighbor not in visited:
                        queue.append(neighbor)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn bfs"), "Got: {}", result);
}

#[test]
fn test_s12_dfs_algorithm() {
    let code = r#"
def dfs(graph: dict, start: str) -> list:
    visited = set()
    result = []

    def visit(node: str):
        if node in visited:
            return
        visited.add(node)
        result.append(node)
        if node in graph:
            for neighbor in graph[node]:
                visit(neighbor)

    visit(start)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn dfs"), "Got: {}", result);
}

// ===== Complex number patterns =====

#[test]
fn test_s12_collatz_sequence() {
    let code = r#"
def collatz(n: int) -> list:
    sequence = [n]
    while n != 1:
        if n % 2 == 0:
            n = n // 2
        else:
            n = 3 * n + 1
        sequence.append(n)
    return sequence
"#;
    let result = transpile(code);
    assert!(result.contains("fn collatz"), "Got: {}", result);
}

#[test]
fn test_s12_roman_numeral_converter() {
    let code = r#"
def to_roman(num: int) -> str:
    values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1]
    symbols = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"]
    result = ""
    for i in range(len(values)):
        while num >= values[i]:
            result += symbols[i]
            num -= values[i]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_roman"), "Got: {}", result);
}

// ===== Complex data structure operations =====

#[test]
fn test_s12_merge_sorted_lists() {
    let code = r#"
def merge_sorted(a: list, b: list) -> list:
    result = []
    i = 0
    j = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i += 1
        else:
            result.append(b[j])
            j += 1
    while i < len(a):
        result.append(a[i])
        i += 1
    while j < len(b):
        result.append(b[j])
        j += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_sorted"), "Got: {}", result);
}

#[test]
fn test_s12_flatten_deep() {
    let code = r#"
def flatten_deep(nested: list) -> list:
    result = []
    stack = list(nested)
    while stack:
        item = stack.pop()
        if isinstance(item, list):
            for sub in item:
                stack.append(sub)
        else:
            result.append(item)
    result.reverse()
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten_deep"), "Got: {}", result);
}

// ===== Complex error handling =====

#[test]
fn test_s12_retry_pattern() {
    let code = r#"
def retry(func, max_attempts: int = 3) -> bool:
    attempts = 0
    while attempts < max_attempts:
        try:
            func()
            return True
        except Exception:
            attempts += 1
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn retry"), "Got: {}", result);
}

// ===== Complex iteration patterns =====

#[test]
fn test_s12_sliding_window() {
    let code = r#"
def sliding_window_max(nums: list, k: int) -> list:
    if not nums or k <= 0:
        return []
    result = []
    for i in range(len(nums) - k + 1):
        window_max = nums[i]
        for j in range(i + 1, i + k):
            if nums[j] > window_max:
                window_max = nums[j]
        result.append(window_max)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn sliding_window_max"), "Got: {}", result);
}

#[test]
fn test_s12_kadanes_algorithm() {
    let code = r#"
def max_subarray_sum(nums: list) -> int:
    if not nums:
        return 0
    max_sum = nums[0]
    current_sum = nums[0]
    for i in range(1, len(nums)):
        if current_sum + nums[i] > nums[i]:
            current_sum = current_sum + nums[i]
        else:
            current_sum = nums[i]
        if current_sum > max_sum:
            max_sum = current_sum
    return max_sum
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_subarray_sum"), "Got: {}", result);
}

// ===== Complex functional patterns =====

#[test]
fn test_s12_pipe_functions() {
    let code = r#"
def double(x: int) -> int:
    return x * 2

def add_one(x: int) -> int:
    return x + 1

def square(x: int) -> int:
    return x * x

def compose(f, g):
    def composed(x: int) -> int:
        return f(g(x))
    return composed
"#;
    let result = transpile(code);
    assert!(result.contains("fn double"), "Got: {}", result);
    assert!(result.contains("fn compose"), "Got: {}", result);
}
