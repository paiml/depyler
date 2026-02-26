//! Session 12 Batch 21: Real-world Python patterns for maximum coverage
//!
//! These tests represent common real-world Python patterns that
//! exercise the most code paths in the transpiler.

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

// ===== Real-world data processing =====

#[test]
fn test_s12_log_parser() {
    let code = r#"
def parse_log_line(line: str) -> dict:
    parts = line.split(" ", 3)
    if len(parts) < 4:
        return {"raw": line}
    return {
        "timestamp": parts[0],
        "level": parts[1],
        "source": parts[2],
        "message": parts[3],
    }

def filter_errors(lines: list) -> list:
    errors = []
    for line in lines:
        parsed = parse_log_line(line)
        if "level" in parsed and parsed["level"] == "ERROR":
            errors.append(parsed)
    return errors
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_log_line"), "Got: {}", result);
    assert!(result.contains("fn filter_errors"), "Got: {}", result);
}

#[test]
fn test_s12_config_parser() {
    let code = r##"
def parse_config(content: str) -> dict:
    config = {}
    for line in content.strip().split("\n"):
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        if "=" in line:
            key, value = line.split("=", 1)
            config[key.strip()] = value.strip()
    return config
"##;
    let result = transpile(code);
    assert!(result.contains("fn parse_config"), "Got: {}", result);
}

#[test]
fn test_s12_url_parser() {
    let code = r#"
def parse_url(url: str) -> dict:
    result = {"scheme": "", "host": "", "path": "", "query": ""}
    if "://" in url:
        parts = url.split("://", 1)
        result["scheme"] = parts[0]
        url = parts[1]
    if "?" in url:
        url, query = url.split("?", 1)
        result["query"] = query
    if "/" in url:
        host, path = url.split("/", 1)
        result["host"] = host
        result["path"] = "/" + path
    else:
        result["host"] = url
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_url"), "Got: {}", result);
}

// ===== CLI argument parsing =====

#[test]
fn test_s12_arg_parser() {
    let code = r#"
def parse_args(args: list) -> dict:
    result = {"flags": [], "options": {}, "positional": []}
    i = 0
    while i < len(args):
        arg = args[i]
        if arg.startswith("--"):
            key = arg[2:]
            if "=" in key:
                k, v = key.split("=", 1)
                result["options"][k] = v
            elif i + 1 < len(args) and not args[i + 1].startswith("-"):
                result["options"][key] = args[i + 1]
                i += 1
            else:
                result["flags"].append(key)
        elif arg.startswith("-"):
            result["flags"].append(arg[1:])
        else:
            result["positional"].append(arg)
        i += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_args"), "Got: {}", result);
}

// ===== Matrix operations =====

#[test]
fn test_s12_matrix_determinant_2x2() {
    let code = r#"
def det2x2(m: list) -> int:
    return m[0][0] * m[1][1] - m[0][1] * m[1][0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn det2x2"), "Got: {}", result);
}

#[test]
fn test_s12_matrix_identity() {
    let code = r#"
def identity(n: int) -> list:
    result = []
    for i in range(n):
        row = []
        for j in range(n):
            if i == j:
                row.append(1)
            else:
                row.append(0)
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn identity"), "Got: {}", result);
}

// ===== Text processing =====

#[test]
fn test_s12_word_wrap() {
    let code = r#"
def word_wrap(text: str, width: int) -> str:
    words = text.split()
    lines = []
    current_line = ""
    for word in words:
        if not current_line:
            current_line = word
        elif len(current_line) + 1 + len(word) <= width:
            current_line = current_line + " " + word
        else:
            lines.append(current_line)
            current_line = word
    if current_line:
        lines.append(current_line)
    return "\n".join(lines)
"#;
    let result = transpile(code);
    assert!(result.contains("fn word_wrap"), "Got: {}", result);
}

#[test]
fn test_s12_deduplicate() {
    let code = r#"
def deduplicate(items: list) -> list:
    seen = set()
    result = []
    for item in items:
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn deduplicate"), "Got: {}", result);
}

// ===== Statistics =====

#[test]
fn test_s12_median() {
    let code = r#"
def median(values: list) -> float:
    sorted_vals = sorted(values)
    n = len(sorted_vals)
    if n % 2 == 1:
        return sorted_vals[n // 2]
    else:
        mid = n // 2
        return (sorted_vals[mid - 1] + sorted_vals[mid]) / 2.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn median"), "Got: {}", result);
}

#[test]
fn test_s12_moving_average() {
    let code = r#"
def moving_average(values: list, window: int) -> list:
    if len(values) < window:
        return []
    result = []
    for i in range(len(values) - window + 1):
        total = 0.0
        for j in range(window):
            total += values[i + j]
        result.append(total / window)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn moving_average"), "Got: {}", result);
}

// ===== Crypto/encoding patterns =====

#[test]
fn test_s12_base64_like_encode() {
    let code = r#"
def simple_encode(text: str) -> str:
    result = ""
    for c in text:
        code = ord(c)
        result += chr((code + 13) % 128)
    return result

def simple_decode(encoded: str) -> str:
    result = ""
    for c in encoded:
        code = ord(c)
        result += chr((code - 13) % 128)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn simple_encode"), "Got: {}", result);
    assert!(result.contains("fn simple_decode"), "Got: {}", result);
}

// ===== Tree operations =====

#[test]
fn test_s12_tree_depth() {
    let code = r#"
class TreeNode:
    def __init__(self, value: int):
        self.value = value
        self.left = None
        self.right = None

    def depth(self) -> int:
        left_depth = 0
        right_depth = 0
        if self.left is not None:
            left_depth = self.left.depth()
        if self.right is not None:
            right_depth = self.right.depth()
        if left_depth > right_depth:
            return left_depth + 1
        return right_depth + 1
"#;
    let result = transpile(code);
    assert!(result.contains("TreeNode"), "Got: {}", result);
}

// ===== State machine patterns =====

#[test]
fn test_s12_state_machine() {
    let code = r#"
def tokenize(text: str) -> list:
    tokens = []
    current = ""
    state = "normal"
    for c in text:
        if state == "normal":
            if c == '"':
                state = "string"
                current = ""
            elif c == ' ':
                if current:
                    tokens.append(current)
                    current = ""
            else:
                current += c
        elif state == "string":
            if c == '"':
                tokens.append(current)
                current = ""
                state = "normal"
            else:
                current += c
    if current:
        tokens.append(current)
    return tokens
"#;
    let result = transpile(code);
    assert!(result.contains("fn tokenize"), "Got: {}", result);
}

// ===== Complex data validation =====

#[test]
fn test_s12_validate_ip() {
    let code = r#"
def is_valid_ip(ip: str) -> bool:
    parts = ip.split(".")
    if len(parts) != 4:
        return False
    for part in parts:
        if not part.isdigit():
            return False
        num = int(part)
        if num < 0 or num > 255:
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid_ip"), "Got: {}", result);
}

#[test]
fn test_s12_validate_date() {
    let code = r#"
def is_valid_date(date_str: str) -> bool:
    parts = date_str.split("-")
    if len(parts) != 3:
        return False
    year = int(parts[0])
    month = int(parts[1])
    day = int(parts[2])
    if month < 1 or month > 12:
        return False
    if day < 1 or day > 31:
        return False
    if year < 1:
        return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid_date"), "Got: {}", result);
}

// ===== Complex I/O patterns =====

#[test]
fn test_s12_read_csv_to_dicts() {
    let code = r#"
def csv_to_dicts(content: str) -> list:
    lines = content.strip().split("\n")
    if not lines:
        return []
    headers = lines[0].split(",")
    result = []
    for line in lines[1:]:
        values = line.split(",")
        row = {}
        for i in range(len(headers)):
            if i < len(values):
                row[headers[i].strip()] = values[i].strip()
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn csv_to_dicts"), "Got: {}", result);
}

// ===== Complex class with multiple patterns =====

#[test]
fn test_s12_priority_queue() {
    let code = r#"
class PriorityQueue:
    def __init__(self):
        self.items = []

    def push(self, item: int, priority: int):
        self.items.append((priority, item))
        self.items.sort()

    def pop(self) -> int:
        if not self.items:
            raise IndexError("Queue is empty")
        return self.items.pop(0)[1]

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(result.contains("PriorityQueue"), "Got: {}", result);
}
