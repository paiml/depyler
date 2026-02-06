//! Session 12 Batch 45: Error and edge case paths
//!
//! Specifically targets error paths and edge cases that increase
//! coverage on rarely-exercised branches:
//! - Empty input handling
//! - Single-element collections
//! - Boundary values
//! - Large/complex programs
//! - Mixed-type operations

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

// ===== Empty input handling =====

#[test]
fn test_s12_b45_empty_string_ops() {
    let code = r#"
def process_empty(s: str) -> str:
    if not s:
        return ""
    return s.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_empty"), "Got: {}", result);
}

#[test]
fn test_s12_b45_empty_list_ops() {
    let code = r#"
def safe_first(items: list, default: int) -> int:
    if not items:
        return default
    return items[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_first"), "Got: {}", result);
}

#[test]
fn test_s12_b45_empty_dict_ops() {
    let code = r#"
def safe_lookup(d: dict, key: str, default: str) -> str:
    if not d:
        return default
    return d.get(key, default)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_lookup"), "Got: {}", result);
}

// ===== Single element collections =====

#[test]
fn test_s12_b45_single_element_list() {
    let code = r#"
def wrap(x: int) -> list:
    return [x]
"#;
    let result = transpile(code);
    assert!(result.contains("fn wrap"), "Got: {}", result);
}

#[test]
fn test_s12_b45_single_pair_dict() {
    let code = r#"
def single_entry(k: str, v: int) -> dict:
    return {k: v}
"#;
    let result = transpile(code);
    assert!(result.contains("fn single_entry"), "Got: {}", result);
}

// ===== Complex multi-function programs =====

#[test]
fn test_s12_b45_multi_func_program() {
    let code = r##"
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

def primes_up_to(n: int) -> list:
    result = []
    for i in range(2, n + 1):
        if is_prime(i):
            result.append(i)
    return result

def prime_factors(n: int) -> list:
    factors = []
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors.append(d)
            n = n // d
        d += 1
    if n > 1:
        factors.append(n)
    return factors

def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a

def lcm(a: int, b: int) -> int:
    return a * b // gcd(a, b)
"##;
    let result = transpile(code);
    assert!(result.contains("fn is_prime"), "Got: {}", result);
    assert!(result.contains("fn primes_up_to"), "Got: {}", result);
    assert!(result.contains("fn prime_factors"), "Got: {}", result);
    assert!(result.contains("fn gcd"), "Got: {}", result);
    assert!(result.contains("fn lcm"), "Got: {}", result);
}

// ===== String builder patterns =====

#[test]
fn test_s12_b45_indent_builder() {
    let code = r#"
def indent_lines(text: str, spaces: int) -> str:
    prefix = " " * spaces
    result = ""
    for line in text.split("\n"):
        result += prefix + line + "\n"
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn indent_lines"), "Got: {}", result);
}

#[test]
fn test_s12_b45_url_builder() {
    let code = r##"
def build_url(host: str, path: str, params: dict) -> str:
    url = f"https://{host}/{path}"
    if params:
        parts = []
        for k, v in params.items():
            parts.append(f"{k}={v}")
        url += "?" + "&".join(parts)
    return url
"##;
    let result = transpile(code);
    assert!(result.contains("fn build_url"), "Got: {}", result);
}

// ===== Complex conditional chains =====

#[test]
fn test_s12_b45_http_status_text() {
    let code = r#"
def status_text(code: int) -> str:
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
    else:
        return "Unknown"
"#;
    let result = transpile(code);
    assert!(result.contains("fn status_text"), "Got: {}", result);
}

// ===== Complex class with many methods =====

#[test]
fn test_s12_b45_string_buffer_class() {
    let code = r##"
class StringBuilder:
    def __init__(self):
        self.parts = []
        self.indent = 0

    def add(self, text: str):
        self.parts.append("  " * self.indent + text)

    def add_line(self, text: str):
        self.parts.append("  " * self.indent + text + "\n")

    def increase_indent(self):
        self.indent += 1

    def decrease_indent(self):
        if self.indent > 0:
            self.indent -= 1

    def build(self) -> str:
        return "".join(self.parts)

    def clear(self):
        self.parts = []
        self.indent = 0

    def line_count(self) -> int:
        count = 0
        for part in self.parts:
            if "\n" in part:
                count += 1
        return count
"##;
    let result = transpile(code);
    assert!(result.contains("StringBuilder"), "Got: {}", result);
}

// ===== Complex dict operations =====

#[test]
fn test_s12_b45_nested_dict_builder() {
    let code = r#"
def group_by_type(items: list) -> dict:
    groups = {}
    for item in items:
        t = type(item).__name__
        if t not in groups:
            groups[t] = []
        groups[t].append(item)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_by_type"), "Got: {}", result);
}

#[test]
fn test_s12_b45_dict_diff() {
    let code = r#"
def dict_diff(d1: dict, d2: dict) -> dict:
    result = {}
    for key in d1:
        if key not in d2:
            result[key] = d1[key]
        elif d1[key] != d2[key]:
            result[key] = d1[key]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn dict_diff"), "Got: {}", result);
}

// ===== Numeric algorithms =====

#[test]
fn test_s12_b45_newton_sqrt() {
    let code = r#"
def sqrt_approx(n: float) -> float:
    if n < 0.0:
        raise ValueError("negative input")
    if n == 0.0:
        return 0.0
    guess = n / 2.0
    for i in range(100):
        guess = (guess + n / guess) / 2.0
    return guess
"#;
    let result = transpile(code);
    assert!(result.contains("fn sqrt_approx"), "Got: {}", result);
}

#[test]
fn test_s12_b45_matrix_identity() {
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

// ===== Boolean truth table =====

#[test]
fn test_s12_b45_truth_ops() {
    let code = r#"
def xor(a: bool, b: bool) -> bool:
    return (a or b) and not (a and b)

def nand(a: bool, b: bool) -> bool:
    return not (a and b)

def implies(a: bool, b: bool) -> bool:
    return not a or b
"#;
    let result = transpile(code);
    assert!(result.contains("fn xor"), "Got: {}", result);
    assert!(result.contains("fn nand"), "Got: {}", result);
    assert!(result.contains("fn implies"), "Got: {}", result);
}

// ===== Complex set operations =====

#[test]
fn test_s12_b45_jaccard() {
    let code = r#"
def jaccard(a: set, b: set) -> float:
    if not a and not b:
        return 1.0
    inter = len(a.intersection(b))
    union = len(a.union(b))
    return inter / union
"#;
    let result = transpile(code);
    assert!(result.contains("fn jaccard"), "Got: {}", result);
}
