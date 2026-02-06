//! Session 12 Batch 43: Comprehensive integration patterns
//!
//! Tests that combine many features in single functions to maximize
//! code path coverage through feature interaction.

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

// ===== Full programs with many features combined =====

#[test]
fn test_s12_b43_contact_manager() {
    let code = r##"
class Contact:
    def __init__(self, name: str, email: str, phone: str):
        self.name = name
        self.email = email
        self.phone = phone

class ContactBook:
    def __init__(self):
        self.contacts = {}

    def add(self, name: str, email: str, phone: str):
        self.contacts[name] = Contact(name, email, phone)

    def remove(self, name: str) -> bool:
        if name in self.contacts:
            del self.contacts[name]
            return True
        return False

    def find(self, name: str):
        return self.contacts.get(name, None)

    def search(self, query: str) -> list:
        results = []
        for name, contact in self.contacts.items():
            if query.lower() in name.lower():
                results.append(contact)
        return results

    def size(self) -> int:
        return len(self.contacts)

    def all_names(self) -> list:
        return sorted(self.contacts.keys())
"##;
    let result = transpile(code);
    assert!(result.contains("ContactBook"), "Got: {}", result);
}

#[test]
fn test_s12_b43_text_analyzer() {
    let code = r##"
def analyze_text(text: str) -> dict:
    words = text.lower().split()
    unique = set(words)
    freq = {}
    for word in words:
        if word in freq:
            freq[word] += 1
        else:
            freq[word] = 1

    most_common = ""
    most_count = 0
    for word, count in freq.items():
        if count > most_count:
            most_count = count
            most_common = word

    sentences = text.split(".")
    avg_words = len(words) / max(len(sentences), 1)

    return {
        "total_words": len(words),
        "unique_words": len(unique),
        "most_common": most_common,
        "avg_words_per_sentence": avg_words
    }
"##;
    let result = transpile(code);
    assert!(result.contains("fn analyze_text"), "Got: {}", result);
}

#[test]
fn test_s12_b43_state_machine() {
    let code = r##"
class StateMachine:
    def __init__(self):
        self.state = "idle"
        self.transitions = {}
        self.history = []

    def add_transition(self, from_state: str, event: str, to_state: str):
        key = f"{from_state}:{event}"
        self.transitions[key] = to_state

    def process(self, event: str) -> bool:
        key = f"{self.state}:{event}"
        if key in self.transitions:
            self.history.append(self.state)
            self.state = self.transitions[key]
            return True
        return False

    def get_state(self) -> str:
        return self.state

    def get_history(self) -> list:
        return self.history

    def reset(self):
        self.state = "idle"
        self.history = []
"##;
    let result = transpile(code);
    assert!(result.contains("StateMachine"), "Got: {}", result);
}

#[test]
fn test_s12_b43_sparse_matrix() {
    let code = r#"
class SparseMatrix:
    def __init__(self, rows: int, cols: int):
        self.rows = rows
        self.cols = cols
        self.data = {}

    def set_val(self, r: int, c: int, val: float):
        if val != 0.0:
            self.data[(r, c)] = val
        elif (r, c) in self.data:
            del self.data[(r, c)]

    def get_val(self, r: int, c: int) -> float:
        return self.data.get((r, c), 0.0)

    def nnz(self) -> int:
        return len(self.data)

    def row_sum(self, r: int) -> float:
        total = 0.0
        for key, val in self.data.items():
            if key[0] == r:
                total += val
        return total
"#;
    let result = transpile(code);
    assert!(result.contains("SparseMatrix"), "Got: {}", result);
}

#[test]
fn test_s12_b43_expression_parser() {
    let code = r##"
def evaluate_rpn(tokens: list) -> float:
    stack = []
    for token in tokens:
        if token == "+":
            b = stack.pop()
            a = stack.pop()
            stack.append(a + b)
        elif token == "-":
            b = stack.pop()
            a = stack.pop()
            stack.append(a - b)
        elif token == "*":
            b = stack.pop()
            a = stack.pop()
            stack.append(a * b)
        elif token == "/":
            b = stack.pop()
            a = stack.pop()
            if b == 0.0:
                raise ValueError("division by zero")
            stack.append(a / b)
        else:
            stack.append(float(token))
    return stack[0]
"##;
    let result = transpile(code);
    assert!(result.contains("fn evaluate_rpn"), "Got: {}", result);
}

#[test]
fn test_s12_b43_interval_scheduler() {
    let code = r#"
def max_non_overlapping(intervals: list) -> int:
    if not intervals:
        return 0
    intervals.sort()
    count = 1
    end = intervals[0][1]
    for i in range(1, len(intervals)):
        if intervals[i][0] >= end:
            count += 1
            end = intervals[i][1]
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_non_overlapping"), "Got: {}", result);
}

#[test]
fn test_s12_b43_trie_pattern() {
    let code = r##"
class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_end = False

class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str):
        node = self.root
        for char in word:
            if char not in node.children:
                node.children[char] = TrieNode()
            node = node.children[char]
        node.is_end = True

    def search(self, word: str) -> bool:
        node = self.root
        for char in word:
            if char not in node.children:
                return False
            node = node.children[char]
        return node.is_end

    def starts_with(self, prefix: str) -> bool:
        node = self.root
        for char in prefix:
            if char not in node.children:
                return False
            node = node.children[char]
        return True
"##;
    let result = transpile(code);
    assert!(result.contains("Trie"), "Got: {}", result);
}

#[test]
fn test_s12_b43_json_builder() {
    let code = r##"
def to_json_string(obj) -> str:
    if isinstance(obj, dict):
        parts = []
        for k, v in obj.items():
            parts.append(f'"{k}": {to_json_string(v)}')
        return "{" + ", ".join(parts) + "}"
    elif isinstance(obj, list):
        parts = []
        for item in obj:
            parts.append(to_json_string(item))
        return "[" + ", ".join(parts) + "]"
    elif isinstance(obj, str):
        return f'"{obj}"'
    elif isinstance(obj, bool):
        return "true" if obj else "false"
    elif isinstance(obj, int) or isinstance(obj, float):
        return str(obj)
    return "null"
"##;
    let result = transpile(code);
    assert!(result.contains("fn to_json_string"), "Got: {}", result);
}

#[test]
fn test_s12_b43_simple_regex_match() {
    let code = r##"
def simple_match(pattern: str, text: str) -> bool:
    if not pattern:
        return not text
    if len(pattern) >= 2 and pattern[1] == "*":
        if simple_match(pattern[2:], text):
            return True
        if text and (pattern[0] == "." or pattern[0] == text[0]):
            return simple_match(pattern, text[1:])
        return False
    if text and (pattern[0] == "." or pattern[0] == text[0]):
        return simple_match(pattern[1:], text[1:])
    return False
"##;
    let result = transpile(code);
    assert!(result.contains("fn simple_match"), "Got: {}", result);
}

#[test]
fn test_s12_b43_cache_class() {
    let code = r##"
class Cache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.data = {}
        self.access_count = {}

    def get(self, key: str):
        if key in self.data:
            self.access_count[key] = self.access_count.get(key, 0) + 1
            return self.data[key]
        return None

    def put(self, key: str, value: int):
        if len(self.data) >= self.capacity and key not in self.data:
            self.evict()
        self.data[key] = value
        self.access_count[key] = 1

    def evict(self):
        if not self.data:
            return
        min_key = ""
        min_count = 999999
        for k, count in self.access_count.items():
            if count < min_count:
                min_count = count
                min_key = k
        if min_key in self.data:
            del self.data[min_key]
            del self.access_count[min_key]

    def size(self) -> int:
        return len(self.data)
"##;
    let result = transpile(code);
    assert!(result.contains("Cache"), "Got: {}", result);
}

#[test]
fn test_s12_b43_path_finder() {
    let code = r##"
def find_path(graph: dict, start: str, end: str) -> list:
    if start == end:
        return [start]
    visited = set()
    queue = [[start]]
    while queue:
        path = queue.pop(0)
        node = path[-1]
        if node in visited:
            continue
        visited.add(node)
        if node in graph:
            for neighbor in graph[node]:
                new_path = path + [neighbor]
                if neighbor == end:
                    return new_path
                queue.append(new_path)
    return []
"##;
    let result = transpile(code);
    assert!(result.contains("fn find_path"), "Got: {}", result);
}

#[test]
fn test_s12_b43_scheduler() {
    let code = r#"
class Task:
    def __init__(self, name: str, duration: int, priority: int):
        self.name = name
        self.duration = duration
        self.priority = priority
        self.completed = False

class Scheduler:
    def __init__(self):
        self.tasks = []

    def add_task(self, name: str, duration: int, priority: int):
        self.tasks.append(Task(name, duration, priority))

    def next_task(self):
        best = None
        for task in self.tasks:
            if not task.completed:
                if best is None or task.priority > best.priority:
                    best = task
        return best

    def complete(self, name: str) -> bool:
        for task in self.tasks:
            if task.name == name:
                task.completed = True
                return True
        return False

    def pending_count(self) -> int:
        count = 0
        for task in self.tasks:
            if not task.completed:
                count += 1
        return count
"#;
    let result = transpile(code);
    assert!(result.contains("Scheduler"), "Got: {}", result);
}
