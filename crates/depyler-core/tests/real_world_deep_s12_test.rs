//! Session 12 Batch 37: Real-world patterns targeting combined cold paths
//!
//! Tests real programs that exercise multiple cold paths simultaneously:
//! - Classes with dict/list/set attributes and complex methods
//! - Functions using multiple builtins (sorted, enumerate, zip)
//! - Error handling with try/except in complex contexts
//! - File I/O patterns
//! - Data processing pipelines

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

// ===== Data processing pipelines =====

#[test]
fn test_s12_b37_word_frequency() {
    let code = r##"
def word_frequency(text: str) -> dict:
    freq = {}
    for word in text.lower().split():
        word = word.strip(".,!?;:")
        if word:
            if word in freq:
                freq[word] += 1
            else:
                freq[word] = 1
    return freq
"##;
    let result = transpile(code);
    assert!(result.contains("fn word_frequency"), "Got: {}", result);
}

#[test]
fn test_s12_b37_csv_parser() {
    let code = r#"
def parse_csv(text: str) -> list:
    rows = []
    for line in text.strip().split("\n"):
        fields = line.split(",")
        rows.append(fields)
    return rows
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_csv"), "Got: {}", result);
}

#[test]
fn test_s12_b37_moving_average() {
    let code = r#"
def moving_average(data: list, window: int) -> list:
    result = []
    for i in range(len(data) - window + 1):
        total = 0.0
        for j in range(window):
            total += data[i + j]
        result.append(total / window)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn moving_average"), "Got: {}", result);
}

#[test]
fn test_s12_b37_histogram() {
    let code = r#"
def histogram(data: list, bins: int) -> dict:
    if not data:
        return {}
    lo = min(data)
    hi = max(data)
    width = (hi - lo) / bins
    counts = {}
    for i in range(bins):
        counts[i] = 0
    for val in data:
        idx = int((val - lo) / width)
        if idx >= bins:
            idx = bins - 1
        counts[idx] += 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn histogram"), "Got: {}", result);
}

// ===== Matrix/vector operations =====

#[test]
fn test_s12_b37_matrix_multiply() {
    let code = r#"
def mat_mul(a: list, b: list) -> list:
    rows_a = len(a)
    cols_a = len(a[0])
    cols_b = len(b[0])
    result = []
    for i in range(rows_a):
        row = []
        for j in range(cols_b):
            total = 0
            for k in range(cols_a):
                total += a[i][k] * b[k][j]
            row.append(total)
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn mat_mul"), "Got: {}", result);
}

#[test]
fn test_s12_b37_dot_product() {
    let code = r#"
def dot(a: list, b: list) -> float:
    total = 0.0
    for i in range(len(a)):
        total += a[i] * b[i]
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn dot"), "Got: {}", result);
}

#[test]
fn test_s12_b37_vector_magnitude() {
    let code = r#"
def magnitude(v: list) -> float:
    total = 0.0
    for x in v:
        total += x * x
    return total ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("fn magnitude"), "Got: {}", result);
}

// ===== String processing =====

#[test]
fn test_s12_b37_tokenizer() {
    let code = r##"
def tokenize(text: str) -> list:
    tokens = []
    current = ""
    for c in text:
        if c.isalnum():
            current += c
        else:
            if current:
                tokens.append(current)
                current = ""
            if not c.isspace():
                tokens.append(c)
    if current:
        tokens.append(current)
    return tokens
"##;
    let result = transpile(code);
    assert!(result.contains("fn tokenize"), "Got: {}", result);
}

#[test]
fn test_s12_b37_run_length_encode() {
    let code = r#"
def rle_encode(s: str) -> str:
    if not s:
        return ""
    result = ""
    count = 1
    for i in range(1, len(s)):
        if s[i] == s[i - 1]:
            count += 1
        else:
            result += s[i - 1]
            if count > 1:
                result += str(count)
            count = 1
    result += s[-1]
    if count > 1:
        result += str(count)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn rle_encode"), "Got: {}", result);
}

#[test]
fn test_s12_b37_longest_common_prefix() {
    let code = r#"
def lcp(strings: list) -> str:
    if not strings:
        return ""
    prefix = strings[0]
    for s in strings:
        while not s.startswith(prefix):
            prefix = prefix[:-1]
            if not prefix:
                return ""
    return prefix
"#;
    let result = transpile(code);
    assert!(result.contains("fn lcp"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_b37_linked_list_class() {
    let code = r##"
class Node:
    def __init__(self, val: int):
        self.val = val
        self.next = None

class LinkedList:
    def __init__(self):
        self.head = None
        self.size = 0

    def push(self, val: int):
        node = Node(val)
        node.next = self.head
        self.head = node
        self.size += 1

    def pop(self) -> int:
        if self.head is None:
            raise IndexError("empty list")
        val = self.head.val
        self.head = self.head.next
        self.size -= 1
        return val

    def peek(self) -> int:
        if self.head is None:
            raise IndexError("empty list")
        return self.head.val

    def length(self) -> int:
        return self.size
"##;
    let result = transpile(code);
    assert!(result.contains("LinkedList"), "Got: {}", result);
}

#[test]
fn test_s12_b37_graph_class() {
    let code = r##"
class Graph:
    def __init__(self):
        self.adjacency = {}

    def add_edge(self, src: str, dst: str):
        if src not in self.adjacency:
            self.adjacency[src] = []
        self.adjacency[src].append(dst)

    def neighbors(self, node: str) -> list:
        if node in self.adjacency:
            return self.adjacency[node]
        return []

    def has_edge(self, src: str, dst: str) -> bool:
        if src not in self.adjacency:
            return False
        return dst in self.adjacency[src]

    def node_count(self) -> int:
        return len(self.adjacency)
"##;
    let result = transpile(code);
    assert!(result.contains("Graph"), "Got: {}", result);
}

#[test]
fn test_s12_b37_priority_queue() {
    let code = r#"
class MinHeap:
    def __init__(self):
        self.data = []

    def push(self, val: int):
        self.data.append(val)
        idx = len(self.data) - 1
        while idx > 0:
            parent = (idx - 1) // 2
            if self.data[parent] > self.data[idx]:
                self.data[parent], self.data[idx] = self.data[idx], self.data[parent]
                idx = parent
            else:
                break

    def peek(self) -> int:
        return self.data[0]

    def size(self) -> int:
        return len(self.data)
"#;
    let result = transpile(code);
    assert!(result.contains("MinHeap"), "Got: {}", result);
}

// ===== Enumerate and zip patterns =====

#[test]
fn test_s12_b37_enumerate_dict_build() {
    let code = r#"
def index_map(items: list) -> dict:
    result = {}
    for i, item in enumerate(items):
        result[item] = i
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn index_map"), "Got: {}", result);
}

#[test]
fn test_s12_b37_zip_sum() {
    let code = r#"
def pairwise_sum(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pairwise_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b37_zip_dict() {
    let code = r#"
def make_dict(keys: list, values: list) -> dict:
    result = {}
    for k, v in zip(keys, values):
        result[k] = v
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_dict"), "Got: {}", result);
}

// ===== Complex algorithm patterns =====

#[test]
fn test_s12_b37_levenshtein() {
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
fn test_s12_b37_knapsack() {
    let code = r#"
def knapsack(weights: list, values: list, capacity: int) -> int:
    n = len(weights)
    dp = []
    for i in range(n + 1):
        row = []
        for j in range(capacity + 1):
            row.append(0)
        dp.append(row)
    for i in range(1, n + 1):
        for w in range(1, capacity + 1):
            if weights[i - 1] <= w:
                dp[i][w] = max(dp[i - 1][w], values[i - 1] + dp[i - 1][w - weights[i - 1]])
            else:
                dp[i][w] = dp[i - 1][w]
    return dp[n][capacity]
"#;
    let result = transpile(code);
    assert!(result.contains("fn knapsack"), "Got: {}", result);
}

#[test]
fn test_s12_b37_longest_increasing() {
    let code = r#"
def lis(arr: list) -> int:
    n = len(arr)
    if n == 0:
        return 0
    dp = []
    for i in range(n):
        dp.append(1)
    for i in range(1, n):
        for j in range(i):
            if arr[j] < arr[i]:
                dp[i] = max(dp[i], dp[j] + 1)
    result = 0
    for x in dp:
        if x > result:
            result = x
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn lis"), "Got: {}", result);
}

// ===== Error handling patterns =====

#[test]
fn test_s12_b37_retry_pattern() {
    let code = r#"
def retry_operation(max_attempts: int) -> bool:
    attempts = 0
    while attempts < max_attempts:
        try:
            result = attempts * 2
            if result > 3:
                return True
        except Exception:
            pass
        attempts += 1
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn retry_operation"), "Got: {}", result);
}

#[test]
fn test_s12_b37_safe_operations() {
    let code = r#"
def safe_divide_all(items: list, divisor: int) -> list:
    results = []
    for item in items:
        try:
            results.append(item / divisor)
        except ZeroDivisionError:
            results.append(0)
    return results
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide_all"), "Got: {}", result);
}

// ===== Decorator patterns =====

#[test]
fn test_s12_b37_staticmethod() {
    let code = r#"
class MathUtils:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b

    @staticmethod
    def multiply(a: int, b: int) -> int:
        return a * b

    @staticmethod
    def factorial(n: int) -> int:
        if n <= 1:
            return 1
        result = 1
        for i in range(2, n + 1):
            result *= i
        return result
"#;
    let result = transpile(code);
    assert!(result.contains("MathUtils"), "Got: {}", result);
}

// ===== Del statement =====

#[test]
fn test_s12_b37_del_dict_key() {
    let code = r#"
def remove_entries(d: dict, keys: list) -> dict:
    for key in keys:
        if key in d:
            del d[key]
    return d
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_entries"), "Got: {}", result);
}

// ===== Pass statement =====

#[test]
fn test_s12_b37_abstract_class() {
    let code = r#"
class Shape:
    def area(self) -> float:
        pass

    def perimeter(self) -> float:
        pass

class Rectangle(Shape):
    def __init__(self, width: float, height: float):
        self.width = width
        self.height = height

    def area(self) -> float:
        return self.width * self.height

    def perimeter(self) -> float:
        return 2.0 * (self.width + self.height)
"#;
    let result = transpile(code);
    assert!(result.contains("Shape"), "Got: {}", result);
    assert!(result.contains("Rectangle"), "Got: {}", result);
}
