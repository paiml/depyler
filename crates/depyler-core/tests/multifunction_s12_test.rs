//! Session 12 Batch 89: Multi-function modules with cross-references
//!
//! Tests modules with multiple functions that call each other,
//! exercising function resolution and cross-reference paths.

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
fn test_s12_b89_helper_chain() {
    let code = r#"
def is_even(n: int) -> bool:
    return n % 2 == 0

def is_odd(n: int) -> bool:
    return not is_even(n)

def partition(items: list) -> tuple:
    evens = [x for x in items if is_even(x)]
    odds = [x for x in items if is_odd(x)]
    return (evens, odds)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_even"), "Got: {}", result);
    assert!(result.contains("fn is_odd"), "Got: {}", result);
    assert!(result.contains("fn partition"), "Got: {}", result);
}

#[test]
fn test_s12_b89_recursive_helpers() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    if b == 0:
        return a
    return gcd(b, a % b)

def lcm(a: int, b: int) -> int:
    return a * b // gcd(a, b)

def lcm_list(items: list) -> int:
    result = items[0]
    for i in range(1, len(items)):
        result = lcm(result, items[i])
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn gcd"), "Got: {}", result);
    assert!(result.contains("fn lcm"), "Got: {}", result);
    assert!(result.contains("fn lcm_list"), "Got: {}", result);
}

#[test]
fn test_s12_b89_validation_chain() {
    let code = r#"
def is_alpha(c: str) -> bool:
    return c.isalpha()

def is_valid_start(c: str) -> bool:
    return is_alpha(c) or c == "_"

def is_valid_ident(name: str) -> bool:
    if not name:
        return False
    if not is_valid_start(name[0]):
        return False
    for c in name[1:]:
        if not (is_alpha(c) or c.isdigit() or c == "_"):
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_alpha"), "Got: {}", result);
    assert!(result.contains("fn is_valid_ident"), "Got: {}", result);
}

#[test]
fn test_s12_b89_sort_helpers() {
    let code = r#"
def swap(items: list, i: int, j: int):
    items[i], items[j] = items[j], items[i]

def partition_pivot(items: list, lo: int, hi: int) -> int:
    pivot = items[hi]
    i = lo
    for j in range(lo, hi):
        if items[j] <= pivot:
            swap(items, i, j)
            i += 1
    swap(items, i, hi)
    return i

def quicksort_range(items: list, lo: int, hi: int):
    if lo < hi:
        p = partition_pivot(items, lo, hi)
        quicksort_range(items, lo, p - 1)
        quicksort_range(items, p + 1, hi)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
    assert!(result.contains("fn quicksort_range"), "Got: {}", result);
}

#[test]
fn test_s12_b89_string_utilities() {
    let code = r#"
def is_blank(s: str) -> bool:
    return len(s.strip()) == 0

def normalize_whitespace(s: str) -> str:
    words = s.split()
    return " ".join(words)

def truncate(s: str, max_len: int) -> str:
    if len(s) <= max_len:
        return s
    return s[:max_len - 3] + "..."

def pad_right(s: str, width: int) -> str:
    if len(s) >= width:
        return s
    return s + " " * (width - len(s))

def repeat(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_blank"), "Got: {}", result);
    assert!(result.contains("fn truncate"), "Got: {}", result);
    assert!(result.contains("fn pad_right"), "Got: {}", result);
}

#[test]
fn test_s12_b89_math_utilities() {
    let code = r#"
def clamp(value: int, lo: int, hi: int) -> int:
    if value < lo:
        return lo
    if value > hi:
        return hi
    return value

def lerp(a: float, b: float, t: float) -> float:
    return a + (b - a) * t

def map_range(value: float, in_lo: float, in_hi: float, out_lo: float, out_hi: float) -> float:
    t = (value - in_lo) / (in_hi - in_lo)
    return lerp(out_lo, out_hi, t)
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"), "Got: {}", result);
    assert!(result.contains("fn lerp"), "Got: {}", result);
    assert!(result.contains("fn map_range"), "Got: {}", result);
}

#[test]
fn test_s12_b89_list_utilities() {
    let code = r#"
def chunk(items: list, size: int) -> list:
    result = []
    for i in range(0, len(items), size):
        result.append(items[i:i + size])
    return result

def interleave(a: list, b: list) -> list:
    result = []
    for i in range(max(len(a), len(b))):
        if i < len(a):
            result.append(a[i])
        if i < len(b):
            result.append(b[i])
    return result

def unique(items: list) -> list:
    seen = set()
    result = []
    for item in items:
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn chunk"), "Got: {}", result);
    assert!(result.contains("fn interleave"), "Got: {}", result);
    assert!(result.contains("fn unique"), "Got: {}", result);
}

#[test]
fn test_s12_b89_dict_utilities() {
    let code = r#"
def merge_with(a: dict, b: dict, default: int) -> dict:
    result = dict(a)
    for key in b:
        if key in result:
            result[key] += b[key]
        else:
            result[key] = b[key]
    return result

def pick(d: dict, keys: list) -> dict:
    return {k: d[k] for k in keys if k in d}

def omit(d: dict, keys: list) -> dict:
    exclude = set(keys)
    return {k: v for k, v in d.items() if k not in exclude}
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_with"), "Got: {}", result);
    assert!(result.contains("fn pick"), "Got: {}", result);
    assert!(result.contains("fn omit"), "Got: {}", result);
}
