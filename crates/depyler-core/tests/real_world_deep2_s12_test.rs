//! Session 12 Batch 78: Real-world programs that combine many features
//!
//! Complex realistic programs that exercise multiple codegen
//! paths in a single transpilation for maximum feature interaction coverage.

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

#[test]
fn test_s12_b78_url_parser() {
    let code = r##"
def parse_url(url: str) -> dict:
    result = {"scheme": "", "host": "", "port": 80, "path": "/"}
    if "://" in url:
        scheme, rest = url.split("://", 1)
        result["scheme"] = scheme
    else:
        rest = url
    if "/" in rest:
        host_part, path = rest.split("/", 1)
        result["path"] = "/" + path
    else:
        host_part = rest
    if ":" in host_part:
        host, port_str = host_part.split(":", 1)
        result["host"] = host
        result["port"] = int(port_str)
    else:
        result["host"] = host_part
    return result
"##;
    let result = transpile(code);
    assert!(result.contains("fn parse_url"), "Got: {}", result);
}

#[test]
fn test_s12_b78_csv_parser() {
    let code = r##"
def parse_csv(text: str) -> list:
    rows = []
    for line in text.strip().split("\n"):
        fields = []
        for field in line.split(","):
            fields.append(field.strip())
        rows.append(fields)
    return rows

def csv_to_dicts(text: str) -> list:
    rows = parse_csv(text)
    if not rows:
        return []
    headers = rows[0]
    result = []
    for row in rows[1:]:
        record = {}
        for i, header in enumerate(headers):
            if i < len(row):
                record[header] = row[i]
            else:
                record[header] = ""
        result.append(record)
    return result
"##;
    let result = transpile(code);
    assert!(result.contains("fn parse_csv"), "Got: {}", result);
    assert!(result.contains("fn csv_to_dicts"), "Got: {}", result);
}

#[test]
fn test_s12_b78_statistics() {
    let code = r#"
import math

def mean(values: list) -> float:
    if not values:
        return 0.0
    return sum(values) / len(values)

def variance(values: list) -> float:
    if len(values) < 2:
        return 0.0
    avg = mean(values)
    total = 0.0
    for v in values:
        diff = v - avg
        total += diff * diff
    return total / (len(values) - 1)

def std_dev(values: list) -> float:
    return math.sqrt(variance(values))

def median(values: list) -> float:
    sorted_vals = sorted(values)
    n = len(sorted_vals)
    if n == 0:
        return 0.0
    if n % 2 == 1:
        return sorted_vals[n // 2]
    return (sorted_vals[n // 2 - 1] + sorted_vals[n // 2]) / 2.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn mean"), "Got: {}", result);
    assert!(result.contains("fn variance"), "Got: {}", result);
    assert!(result.contains("fn std_dev"), "Got: {}", result);
    assert!(result.contains("fn median"), "Got: {}", result);
}

#[test]
fn test_s12_b78_json_builder() {
    let code = r##"
def to_json(obj) -> str:
    if obj is None:
        return "null"
    if isinstance(obj, bool):
        return "true" if obj else "false"
    if isinstance(obj, int):
        return str(obj)
    if isinstance(obj, float):
        return str(obj)
    if isinstance(obj, str):
        return '"' + obj + '"'
    if isinstance(obj, list):
        parts = [to_json(item) for item in obj]
        return "[" + ", ".join(parts) + "]"
    if isinstance(obj, dict):
        pairs = []
        for k, v in obj.items():
            pairs.append('"' + str(k) + '": ' + to_json(v))
        return "{" + ", ".join(pairs) + "}"
    return str(obj)
"##;
    let result = transpile(code);
    assert!(result.contains("fn to_json"), "Got: {}", result);
}

#[test]
fn test_s12_b78_tokenizer() {
    let code = r##"
def tokenize(text: str) -> list:
    tokens = []
    i = 0
    while i < len(text):
        if text[i].isspace():
            i += 1
            continue
        if text[i].isdigit():
            j = i
            while j < len(text) and text[j].isdigit():
                j += 1
            tokens.append(("NUMBER", text[i:j]))
            i = j
        elif text[i].isalpha() or text[i] == "_":
            j = i
            while j < len(text) and (text[j].isalnum() or text[j] == "_"):
                j += 1
            tokens.append(("IDENT", text[i:j]))
            i = j
        elif text[i] in "+-*/=<>!":
            if i + 1 < len(text) and text[i + 1] == "=":
                tokens.append(("OP", text[i:i+2]))
                i += 2
            else:
                tokens.append(("OP", text[i]))
                i += 1
        else:
            tokens.append(("PUNCT", text[i]))
            i += 1
    return tokens
"##;
    let result = transpile(code);
    assert!(result.contains("fn tokenize"), "Got: {}", result);
}

#[test]
fn test_s12_b78_linked_list() {
    let code = r#"
class Node:
    def __init__(self, value: int):
        self.value = value
        self.next = None

class LinkedList:
    def __init__(self):
        self.head = None
        self.size = 0

    def push(self, value: int):
        node = Node(value)
        node.next = self.head
        self.head = node
        self.size += 1

    def pop(self) -> int:
        if self.head is None:
            return -1
        value = self.head.value
        self.head = self.head.next
        self.size -= 1
        return value

    def to_list(self) -> list:
        result = []
        current = self.head
        while current is not None:
            result.append(current.value)
            current = current.next
        return result
"#;
    let result = transpile(code);
    assert!(result.contains("LinkedList"), "Got: {}", result);
}

#[test]
fn test_s12_b78_stack_calculator() {
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
            if b != 0.0:
                stack.append(a / b)
            else:
                stack.append(0.0)
        else:
            stack.append(float(token))
    if stack:
        return stack[-1]
    return 0.0
"##;
    let result = transpile(code);
    assert!(result.contains("fn evaluate_rpn"), "Got: {}", result);
}

#[test]
fn test_s12_b78_matrix_ops() {
    let code = r#"
def zeros(rows: int, cols: int) -> list:
    result = []
    for i in range(rows):
        row = [0.0] * cols
        result.append(row)
    return result

def transpose(m: list) -> list:
    if not m:
        return []
    rows = len(m)
    cols = len(m[0])
    result = zeros(cols, rows)
    for i in range(rows):
        for j in range(cols):
            result[j][i] = m[i][j]
    return result

def dot_product(a: list, b: list) -> float:
    total = 0.0
    for i in range(len(a)):
        total += a[i] * b[i]
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn zeros"), "Got: {}", result);
    assert!(result.contains("fn transpose"), "Got: {}", result);
    assert!(result.contains("fn dot_product"), "Got: {}", result);
}

#[test]
fn test_s12_b78_text_search() {
    let code = r#"
def kmp_search(text: str, pattern: str) -> list:
    if not pattern:
        return []
    n = len(text)
    m = len(pattern)
    lps = [0] * m
    length = 0
    i = 1
    while i < m:
        if pattern[i] == pattern[length]:
            length += 1
            lps[i] = length
            i += 1
        elif length > 0:
            length = lps[length - 1]
        else:
            lps[i] = 0
            i += 1
    matches = []
    i = 0
    j = 0
    while i < n:
        if text[i] == pattern[j]:
            i += 1
            j += 1
        if j == m:
            matches.append(i - j)
            j = lps[j - 1]
        elif i < n and text[i] != pattern[j]:
            if j > 0:
                j = lps[j - 1]
            else:
                i += 1
    return matches
"#;
    let result = transpile(code);
    assert!(result.contains("fn kmp_search"), "Got: {}", result);
}

#[test]
fn test_s12_b78_state_machine() {
    let code = r##"
class StateMachine:
    def __init__(self):
        self.state = "idle"
        self.transitions = {}

    def add_transition(self, from_state: str, event: str, to_state: str):
        key = from_state + ":" + event
        self.transitions[key] = to_state

    def handle(self, event: str) -> bool:
        key = self.state + ":" + event
        if key in self.transitions:
            self.state = self.transitions[key]
            return True
        return False

    def is_in(self, state: str) -> bool:
        return self.state == state
"##;
    let result = transpile(code);
    assert!(result.contains("StateMachine"), "Got: {}", result);
}
