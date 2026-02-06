//! Session 11: Integration pattern coverage tests
//!
//! Each test transpiles a complete, realistic Python program to exercise
//! as many code paths as possible through the full transpilation pipeline.

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

// ============================================================================
// Stack implementation
// ============================================================================

#[test]
fn test_s11_integ_stack_class() {
    let code = r#"
class Stack:
    def __init__(self) -> None:
        self.items: list = []

    def push(self, item: int) -> None:
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def peek(self) -> int:
        return self.items[-1]

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("Stack"),
        "Should transpile Stack class. Got: {}",
        result
    );
}

// ============================================================================
// Calculator with multiple operations
// ============================================================================

#[test]
fn test_s11_integ_calculator() {
    let code = r#"
def calculate(a: float, b: float, op: str) -> float:
    if op == "+":
        return a + b
    elif op == "-":
        return a - b
    elif op == "*":
        return a * b
    elif op == "/":
        if b == 0.0:
            return 0.0
        return a / b
    elif op == "//":
        if b == 0.0:
            return 0.0
        return float(int(a) // int(b))
    elif op == "%":
        if b == 0.0:
            return 0.0
        return a % b
    elif op == "**":
        return a ** b
    else:
        return 0.0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn calculate"),
        "Should transpile calculator. Got: {}",
        result
    );
}

// ============================================================================
// Linked list traversal
// ============================================================================

#[test]
fn test_s11_integ_linked_list_ops() {
    let code = r#"
def list_sum(items: list) -> int:
    if len(items) == 0:
        return 0
    total: int = 0
    for item in items:
        total += item
    return total

def list_max(items: list) -> int:
    if len(items) == 0:
        return 0
    best: int = items[0]
    for item in items:
        if item > best:
            best = item
    return best

def list_min(items: list) -> int:
    if len(items) == 0:
        return 0
    best: int = items[0]
    for item in items:
        if item < best:
            best = item
    return best

def list_average(items: list) -> float:
    if len(items) == 0:
        return 0.0
    return float(list_sum(items)) / float(len(items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn list_sum")
            && result.contains("fn list_max")
            && result.contains("fn list_min")
            && result.contains("fn list_average"),
        "Should transpile all list functions. Got: {}",
        result
    );
}

// ============================================================================
// String processing pipeline
// ============================================================================

#[test]
fn test_s11_integ_string_pipeline() {
    let code = r#"
def normalize(text: str) -> str:
    return text.strip().lower()

def word_count(text: str) -> int:
    words: list = text.split()
    return len(words)

def char_frequency(text: str) -> dict:
    freq: dict = {}
    for ch in text:
        if ch in freq:
            freq[ch] = freq[ch] + 1
        else:
            freq[ch] = 1
    return freq

def most_common_char(text: str) -> str:
    freq: dict = char_frequency(text)
    best_char: str = ""
    best_count: int = 0
    for ch, count in freq.items():
        if count > best_count:
            best_count = count
            best_char = ch
    return best_char
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn normalize") && result.contains("fn char_frequency"),
        "Should transpile string pipeline. Got: {}",
        result
    );
}

// ============================================================================
// Sorting algorithms
// ============================================================================

#[test]
fn test_s11_integ_selection_sort() {
    let code = r#"
def selection_sort(arr: list) -> list:
    n: int = len(arr)
    for i in range(n):
        min_idx: int = i
        for j in range(i + 1, n):
            if arr[j] < arr[min_idx]:
                min_idx = j
        if min_idx != i:
            arr[i], arr[min_idx] = arr[min_idx], arr[i]
    return arr
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn selection_sort"),
        "Should transpile selection sort. Got: {}",
        result
    );
}

#[test]
fn test_s11_integ_insertion_sort() {
    let code = r#"
def insertion_sort(arr: list) -> list:
    for i in range(1, len(arr)):
        key: int = arr[i]
        j: int = i - 1
        while j >= 0 and arr[j] > key:
            arr[j + 1] = arr[j]
            j -= 1
        arr[j + 1] = key
    return arr
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn insertion_sort"),
        "Should transpile insertion sort. Got: {}",
        result
    );
}

// ============================================================================
// Matrix operations
// ============================================================================

#[test]
fn test_s11_integ_matrix_ops() {
    let code = r#"
def matrix_add(a: list, b: list) -> list:
    result: list = []
    for i in range(len(a)):
        row: list = []
        for j in range(len(a[0])):
            row.append(a[i][j] + b[i][j])
        result.append(row)
    return result

def matrix_transpose(m: list) -> list:
    if len(m) == 0:
        return []
    rows: int = len(m)
    cols: int = len(m[0])
    result: list = []
    for j in range(cols):
        row: list = []
        for i in range(rows):
            row.append(m[i][j])
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn matrix_add") && result.contains("fn matrix_transpose"),
        "Should transpile matrix ops. Got: {}",
        result
    );
}

// ============================================================================
// Dictionary-heavy program
// ============================================================================

#[test]
fn test_s11_integ_word_counter() {
    let code = r#"
def count_words(text: str) -> dict:
    words: list = text.lower().split()
    counts: dict = {}
    for word in words:
        if word in counts:
            counts[word] = counts[word] + 1
        else:
            counts[word] = 1
    return counts

def top_words(counts: dict, n: int) -> list:
    items: list = []
    for word, count in counts.items():
        items.append((count, word))
    items.sort()
    items.reverse()
    result: list = []
    for i in range(min(n, len(items))):
        result.append(items[i])
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_words") && result.contains("fn top_words"),
        "Should transpile word counter. Got: {}",
        result
    );
}

// ============================================================================
// Error handling patterns
// ============================================================================

#[test]
fn test_s11_integ_robust_parser() {
    let code = r#"
def parse_int_safe(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0

def parse_float_safe(s: str) -> float:
    try:
        return float(s)
    except ValueError:
        return 0.0

def parse_config(text: str) -> dict:
    result: dict = {}
    lines: list = text.split("\n")
    for line in lines:
        line = line.strip()
        if len(line) == 0:
            continue
        parts: list = line.split("=")
        if len(parts) == 2:
            key: str = parts[0].strip()
            val: str = parts[1].strip()
            result[key] = val
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn parse_int_safe") && result.contains("fn parse_config"),
        "Should transpile robust parser. Got: {}",
        result
    );
}

// ============================================================================
// Recursive algorithms
// ============================================================================

#[test]
fn test_s11_integ_recursive_algos() {
    let code = r#"
def fib_recursive(n: int) -> int:
    if n <= 1:
        return n
    return fib_recursive(n - 1) + fib_recursive(n - 2)

def power(base: int, exp: int) -> int:
    if exp == 0:
        return 1
    if exp == 1:
        return base
    if exp % 2 == 0:
        half: int = power(base, exp // 2)
        return half * half
    return base * power(base, exp - 1)

def sum_digits(n: int) -> int:
    if n < 10:
        return n
    return n % 10 + sum_digits(n // 10)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fib_recursive")
            && result.contains("fn power")
            && result.contains("fn sum_digits"),
        "Should transpile recursive algos. Got: {}",
        result
    );
}

// ============================================================================
// State machine pattern
// ============================================================================

#[test]
fn test_s11_integ_state_machine() {
    let code = r#"
def tokenize(text: str) -> list:
    tokens: list = []
    current: str = ""
    in_string: bool = False

    for ch in text:
        if ch == '"':
            if in_string:
                tokens.append(current)
                current = ""
                in_string = False
            else:
                in_string = True
        elif in_string:
            current = current + ch
        elif ch == " " or ch == "\n":
            if len(current) > 0:
                tokens.append(current)
                current = ""
        else:
            current = current + ch

    if len(current) > 0:
        tokens.append(current)

    return tokens
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn tokenize"),
        "Should transpile state machine. Got: {}",
        result
    );
}

// ============================================================================
// Class with multiple methods
// ============================================================================

#[test]
fn test_s11_integ_vector2d() {
    let code = r#"
class Vector2D:
    def __init__(self, x: float, y: float) -> None:
        self.x = x
        self.y = y

    def magnitude(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5

    def dot(self, other) -> float:
        return self.x * other.x + self.y * other.y

    def add(self, other):
        return Vector2D(self.x + other.x, self.y + other.y)

    def scale(self, factor: float):
        return Vector2D(self.x * factor, self.y * factor)
"#;
    let result = transpile(code);
    assert!(
        result.contains("Vector2D"),
        "Should transpile Vector2D. Got: {}",
        result
    );
}

// ============================================================================
// Generator-like pattern
// ============================================================================

#[test]
fn test_s11_integ_range_utilities() {
    let code = r#"
def evens(n: int) -> list:
    return [i for i in range(n) if i % 2 == 0]

def odds(n: int) -> list:
    return [i for i in range(n) if i % 2 != 0]

def multiples(base: int, n: int) -> list:
    return [base * i for i in range(1, n + 1)]

def primes_up_to(n: int) -> list:
    result: list = []
    for num in range(2, n + 1):
        is_prime: bool = True
        for div in range(2, num):
            if num % div == 0:
                is_prime = False
                break
        if is_prime:
            result.append(num)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn evens")
            && result.contains("fn odds")
            && result.contains("fn primes_up_to"),
        "Should transpile range utilities. Got: {}",
        result
    );
}

// ============================================================================
// Complex data processing
// ============================================================================

#[test]
fn test_s11_integ_statistics() {
    let code = r#"
def mean(data: list) -> float:
    if len(data) == 0:
        return 0.0
    total: float = 0.0
    for x in data:
        total += float(x)
    return total / float(len(data))

def variance(data: list) -> float:
    if len(data) == 0:
        return 0.0
    avg: float = mean(data)
    total: float = 0.0
    for x in data:
        diff: float = float(x) - avg
        total += diff * diff
    return total / float(len(data))

def standard_deviation(data: list) -> float:
    return variance(data) ** 0.5

def median(data: list) -> float:
    if len(data) == 0:
        return 0.0
    sorted_data: list = sorted(data)
    n: int = len(sorted_data)
    if n % 2 == 0:
        return float(sorted_data[n // 2 - 1] + sorted_data[n // 2]) / 2.0
    return float(sorted_data[n // 2])
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn mean")
            && result.contains("fn variance")
            && result.contains("fn median"),
        "Should transpile statistics. Got: {}",
        result
    );
}

// ============================================================================
// Multiple if/elif/else with early returns
// ============================================================================

#[test]
fn test_s11_integ_http_status() {
    let code = r#"
def status_message(code: int) -> str:
    if code == 200:
        return "OK"
    elif code == 201:
        return "Created"
    elif code == 204:
        return "No Content"
    elif code == 301:
        return "Moved Permanently"
    elif code == 302:
        return "Found"
    elif code == 400:
        return "Bad Request"
    elif code == 401:
        return "Unauthorized"
    elif code == 403:
        return "Forbidden"
    elif code == 404:
        return "Not Found"
    elif code == 500:
        return "Internal Server Error"
    elif code == 502:
        return "Bad Gateway"
    elif code == 503:
        return "Service Unavailable"
    else:
        return "Unknown"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn status_message"),
        "Should transpile HTTP status. Got: {}",
        result
    );
}

// ============================================================================
// Complex list comprehension with multiple conditions
// ============================================================================

#[test]
fn test_s11_integ_comprehension_complex() {
    let code = r#"
def pythagorean_triples(n: int) -> list:
    result: list = []
    for a in range(1, n):
        for b in range(a, n):
            c_sq: int = a * a + b * b
            c: int = int(c_sq ** 0.5)
            if c * c == c_sq and c <= n:
                result.append((a, b, c))
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pythagorean_triples"),
        "Should transpile complex comprehension. Got: {}",
        result
    );
}

// ============================================================================
// Decorator patterns
// ============================================================================

#[test]
fn test_s11_integ_decorator() {
    let code = r#"
class Registry:
    def __init__(self) -> None:
        self.handlers: dict = {}

    def register(self, name: str) -> None:
        self.handlers[name] = True

    def has(self, name: str) -> bool:
        return name in self.handlers

    def count(self) -> int:
        return len(self.handlers)
"#;
    let result = transpile(code);
    assert!(
        result.contains("Registry"),
        "Should transpile Registry. Got: {}",
        result
    );
}

// ============================================================================
// Complex f-string patterns
// ============================================================================

#[test]
fn test_s11_integ_fstring_report() {
    let code = r#"
def generate_report(name: str, score: int, grade: str) -> str:
    header: str = f"Report for {name}"
    body: str = f"Score: {score}/100 ({grade})"
    if score >= 90:
        status: str = f"{name} passed with distinction"
    elif score >= 60:
        status = f"{name} passed"
    else:
        status = f"{name} did not pass"
    return f"{header}\n{body}\n{status}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn generate_report"),
        "Should transpile f-string report. Got: {}",
        result
    );
}
