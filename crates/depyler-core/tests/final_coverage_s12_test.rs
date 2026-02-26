//! Session 12 Batch 100: Final coverage push patterns
//!
//! Batch 100 - milestone batch combining the most diverse
//! patterns to maximize code path coverage.

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

// ===== Complete mini-programs =====

#[test]
fn test_s12_b100_calculator() {
    let code = r##"
def evaluate(expression: str) -> float:
    tokens = expression.split()
    stack = []
    for token in tokens:
        if token in "+-*/":
            b = stack.pop()
            a = stack.pop()
            if token == "+":
                stack.append(a + b)
            elif token == "-":
                stack.append(a - b)
            elif token == "*":
                stack.append(a * b)
            elif token == "/":
                stack.append(a / b if b != 0.0 else 0.0)
        else:
            stack.append(float(token))
    return stack[0] if stack else 0.0
"##;
    let result = transpile(code);
    assert!(result.contains("fn evaluate"), "Got: {}", result);
}

#[test]
fn test_s12_b100_text_processor() {
    let code = r##"
class TextProcessor:
    def __init__(self, text: str):
        self.text = text
        self.words = text.split()

    def word_count(self) -> int:
        return len(self.words)

    def char_count(self) -> int:
        return len(self.text)

    def unique_words(self) -> int:
        return len(set(w.lower() for w in self.words))

    def most_common(self) -> str:
        freq = {}
        for w in self.words:
            w = w.lower()
            freq[w] = freq.get(w, 0) + 1
        best = ""
        best_count = 0
        for w, c in freq.items():
            if c > best_count:
                best = w
                best_count = c
        return best

    def summary(self) -> dict:
        return {
            "words": self.word_count(),
            "chars": self.char_count(),
            "unique": self.unique_words()
        }
"##;
    let result = transpile(code);
    assert!(result.contains("TextProcessor"), "Got: {}", result);
}

#[test]
fn test_s12_b100_graph_algorithms() {
    let code = r#"
def bfs(graph: dict, start: str) -> list:
    visited = set()
    queue = [start]
    result = []
    while queue:
        node = queue.pop(0)
        if node in visited:
            continue
        visited.add(node)
        result.append(node)
        if node in graph:
            for neighbor in graph[node]:
                if neighbor not in visited:
                    queue.append(neighbor)
    return result

def dfs(graph: dict, start: str) -> list:
    visited = set()
    stack = [start]
    result = []
    while stack:
        node = stack.pop()
        if node in visited:
            continue
        visited.add(node)
        result.append(node)
        if node in graph:
            for neighbor in reversed(graph[node]):
                if neighbor not in visited:
                    stack.append(neighbor)
    return result

def has_path(graph: dict, start: str, end: str) -> bool:
    visited = set()
    queue = [start]
    while queue:
        node = queue.pop(0)
        if node == end:
            return True
        if node in visited:
            continue
        visited.add(node)
        if node in graph:
            for neighbor in graph[node]:
                queue.append(neighbor)
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn bfs"), "Got: {}", result);
    assert!(result.contains("fn dfs"), "Got: {}", result);
    assert!(result.contains("fn has_path"), "Got: {}", result);
}

#[test]
fn test_s12_b100_sorting_suite() {
    let code = r#"
def insertion_sort(items: list) -> list:
    result = list(items)
    for i in range(1, len(result)):
        key = result[i]
        j = i - 1
        while j >= 0 and result[j] > key:
            result[j + 1] = result[j]
            j -= 1
        result[j + 1] = key
    return result

def selection_sort(items: list) -> list:
    result = list(items)
    n = len(result)
    for i in range(n):
        min_idx = i
        for j in range(i + 1, n):
            if result[j] < result[min_idx]:
                min_idx = j
        result[i], result[min_idx] = result[min_idx], result[i]
    return result

def bubble_sort(items: list) -> list:
    result = list(items)
    n = len(result)
    for i in range(n):
        swapped = False
        for j in range(0, n - i - 1):
            if result[j] > result[j + 1]:
                result[j], result[j + 1] = result[j + 1], result[j]
                swapped = True
        if not swapped:
            break
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn insertion_sort"), "Got: {}", result);
    assert!(result.contains("fn selection_sort"), "Got: {}", result);
    assert!(result.contains("fn bubble_sort"), "Got: {}", result);
}

#[test]
fn test_s12_b100_functional_patterns() {
    let code = r#"
def compose(f, g):
    def composed(x):
        return f(g(x))
    return composed

def pipe(value, functions: list):
    result = value
    for func in functions:
        result = func(result)
    return result

def memoize(func):
    cache = {}
    def wrapper(n: int) -> int:
        if n not in cache:
            cache[n] = func(n)
        return cache[n]
    return wrapper
"#;
    let result = transpile(code);
    assert!(result.contains("fn compose"), "Got: {}", result);
    assert!(result.contains("fn pipe"), "Got: {}", result);
}

#[test]
fn test_s12_b100_event_system() {
    let code = r#"
class EventEmitter:
    def __init__(self):
        self.listeners = {}

    def on(self, event: str, callback):
        if event not in self.listeners:
            self.listeners[event] = []
        self.listeners[event].append(callback)

    def emit(self, event: str, data):
        if event in self.listeners:
            for callback in self.listeners[event]:
                callback(data)

    def off(self, event: str):
        if event in self.listeners:
            del self.listeners[event]

    def event_names(self) -> list:
        return list(self.listeners.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("EventEmitter"), "Got: {}", result);
}

#[test]
fn test_s12_b100_data_pipeline() {
    let code = r#"
def read_records(text: str) -> list:
    records = []
    for line in text.strip().split("\n"):
        parts = line.split(",")
        if len(parts) >= 3:
            records.append({
                "name": parts[0].strip(),
                "age": int(parts[1].strip()),
                "score": float(parts[2].strip())
            })
    return records

def filter_records(records: list, min_score: float) -> list:
    return [r for r in records if r["score"] >= min_score]

def average_score(records: list) -> float:
    if not records:
        return 0.0
    return sum(r["score"] for r in records) / len(records)
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_records"), "Got: {}", result);
    assert!(result.contains("fn filter_records"), "Got: {}", result);
    assert!(result.contains("fn average_score"), "Got: {}", result);
}

#[test]
fn test_s12_b100_string_algorithms() {
    let code = r#"
def is_anagram(a: str, b: str) -> bool:
    return sorted(a.lower()) == sorted(b.lower())

def longest_common_prefix(words: list) -> str:
    if not words:
        return ""
    prefix = words[0]
    for word in words[1:]:
        while not word.startswith(prefix):
            prefix = prefix[:-1]
            if not prefix:
                return ""
    return prefix

def run_length_encode(text: str) -> str:
    if not text:
        return ""
    result = ""
    count = 1
    for i in range(1, len(text)):
        if text[i] == text[i - 1]:
            count += 1
        else:
            result += str(count) + text[i - 1]
            count = 1
    result += str(count) + text[-1]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_anagram"), "Got: {}", result);
    assert!(result.contains("fn longest_common_prefix"), "Got: {}", result);
    assert!(result.contains("fn run_length_encode"), "Got: {}", result);
}
