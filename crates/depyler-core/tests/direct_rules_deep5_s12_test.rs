//! Session 12 Batch 33: Deep direct_rules_convert cold paths
//!
//! Targets remaining cold paths in direct_rules_convert.rs:
//! - Nested function closures capturing variables
//! - Complex try/except with multiple handlers and finally
//! - With-statement patterns (file, lock, context)
//! - Optional/None handling patterns
//! - Augmented assignment in complex contexts
//! - Walrus operator patterns
//! - Complex slice assignments
//! - Multi-target assignments
//! - Chained method calls in assignments
//! - Complex default parameter handling

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

// ===== Nested function closures =====

#[test]
fn test_s12_b33_closure_captures_var() {
    let code = r#"
def make_adder(n: int):
    def adder(x: int) -> int:
        return x + n
    return adder
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_adder"), "Got: {}", result);
}

#[test]
fn test_s12_b33_closure_captures_multiple() {
    let code = r#"
def make_range_checker(lo: int, hi: int):
    def check(x: int) -> bool:
        return lo <= x and x <= hi
    return check
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_range_checker"), "Got: {}", result);
}

#[test]
fn test_s12_b33_closure_modifies_outer() {
    let code = r#"
def counter():
    count = 0
    def increment() -> int:
        nonlocal count
        count += 1
        return count
    return increment
"#;
    let result = transpile(code);
    assert!(result.contains("fn counter"), "Got: {}", result);
}

// ===== Complex try/except =====

#[test]
fn test_s12_b33_try_multiple_except() {
    let code = r#"
def safe_divide(a: int, b: int) -> str:
    try:
        result = a / b
        return str(result)
    except ZeroDivisionError:
        return "division by zero"
    except TypeError:
        return "type error"
    except Exception:
        return "unknown error"
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

#[test]
fn test_s12_b33_try_except_finally() {
    let code = r#"
def read_data(path: str) -> str:
    result = ""
    try:
        result = "success"
    except IOError:
        result = "error"
    finally:
        result += " done"
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_data"), "Got: {}", result);
}

#[test]
fn test_s12_b33_try_except_else() {
    let code = r#"
def parse_number(s: str) -> int:
    try:
        value = int(s)
    except ValueError:
        return -1
    else:
        return value
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_number"), "Got: {}", result);
}

#[test]
fn test_s12_b33_nested_try() {
    let code = r#"
def nested_parse(outer: str, inner: str) -> int:
    try:
        x = int(outer)
        try:
            y = int(inner)
            return x + y
        except ValueError:
            return x
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn nested_parse"), "Got: {}", result);
}

// ===== With-statement patterns =====

#[test]
fn test_s12_b33_with_as_var() {
    let code = r#"
def process_file(path: str) -> str:
    with open(path) as f:
        data = f.read()
    return data
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_file"), "Got: {}", result);
}

#[test]
fn test_s12_b33_with_no_var() {
    let code = r#"
def timed_op() -> int:
    result = 0
    with timer():
        result = 42
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn timed_op"), "Got: {}", result);
}

// ===== Optional/None handling =====

#[test]
fn test_s12_b33_none_comparison_chain() {
    let code = r#"
def coalesce(a, b, c) -> int:
    if a is not None:
        return a
    if b is not None:
        return b
    if c is not None:
        return c
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn coalesce"), "Got: {}", result);
}

#[test]
fn test_s12_b33_none_default_pattern() {
    let code = r#"
def get_or_default(value, default: int) -> int:
    if value is None:
        return default
    return value
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_or_default"), "Got: {}", result);
}

#[test]
fn test_s12_b33_none_in_collection() {
    let code = r#"
def filter_none(items: list) -> list:
    result = []
    for item in items:
        if item is not None:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_none"), "Got: {}", result);
}

// ===== Complex augmented assignments =====

#[test]
fn test_s12_b33_augmented_in_loop() {
    let code = r#"
def running_total(items: list) -> list:
    result = []
    total = 0
    for item in items:
        total += item
        result.append(total)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn running_total"), "Got: {}", result);
}

#[test]
fn test_s12_b33_augmented_conditional() {
    let code = r#"
def classify_sum(items: list) -> int:
    pos = 0
    neg = 0
    for item in items:
        if item > 0:
            pos += item
        else:
            neg += item
    return pos + neg
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b33_augmented_string() {
    let code = r#"
def repeat_str(s: str, n: int) -> str:
    result = ""
    for i in range(n):
        result += s
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn repeat_str"), "Got: {}", result);
}

// ===== Multi-target and tuple assignments =====

#[test]
fn test_s12_b33_triple_swap() {
    let code = r#"
def rotate(a: int, b: int, c: int) -> tuple:
    a, b, c = b, c, a
    return (a, b, c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rotate"), "Got: {}", result);
}

#[test]
fn test_s12_b33_tuple_unpack_in_loop() {
    let code = r#"
def sum_pairs(pairs: list) -> int:
    total = 0
    for a, b in pairs:
        total += a + b
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_pairs"), "Got: {}", result);
}

#[test]
fn test_s12_b33_nested_tuple_return() {
    let code = r#"
def stats(items: list) -> tuple:
    total = sum(items)
    count = len(items)
    return (total, count, total / count)
"#;
    let result = transpile(code);
    assert!(result.contains("fn stats"), "Got: {}", result);
}

// ===== Chained method calls =====

#[test]
fn test_s12_b33_chain_strip_split() {
    let code = r#"
def parse_line(line: str) -> list:
    return line.strip().split(",")
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_line"), "Got: {}", result);
}

#[test]
fn test_s12_b33_chain_lower_replace() {
    let code = r#"
def normalize(text: str) -> str:
    return text.lower().replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

#[test]
fn test_s12_b33_chain_str_format() {
    let code = r#"
def title_case(text: str) -> str:
    words = text.lower().split()
    result = []
    for w in words:
        result.append(w.capitalize())
    return " ".join(result)
"#;
    let result = transpile(code);
    assert!(result.contains("fn title_case"), "Got: {}", result);
}

// ===== Complex conditional patterns =====

#[test]
fn test_s12_b33_elif_chain() {
    let code = r#"
def grade(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"
"#;
    let result = transpile(code);
    assert!(result.contains("fn grade"), "Got: {}", result);
}

#[test]
fn test_s12_b33_nested_if_in_loop() {
    let code = r#"
def categorize(items: list) -> dict:
    result = {"pos": [], "neg": [], "zero": []}
    for item in items:
        if item > 0:
            result["pos"].append(item)
        elif item < 0:
            result["neg"].append(item)
        else:
            result["zero"].append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn categorize"), "Got: {}", result);
}

// ===== Complex default parameters =====

#[test]
fn test_s12_b33_default_param_str() {
    let code = r#"
def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_b33_default_param_int() {
    let code = r#"
def power(base: int, exp: int = 2) -> int:
    result = 1
    for i in range(exp):
        result *= base
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s12_b33_default_param_bool() {
    let code = r#"
def search(items: list, target: int, reverse: bool = False) -> int:
    if reverse:
        for i in range(len(items) - 1, -1, -1):
            if items[i] == target:
                return i
    else:
        for i in range(len(items)):
            if items[i] == target:
                return i
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn search"), "Got: {}", result);
}

// ===== Raise patterns =====

#[test]
fn test_s12_b33_raise_value_error() {
    let code = r#"
def positive_sqrt(n: float) -> float:
    if n < 0.0:
        raise ValueError("cannot take sqrt of negative")
    return n ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_sqrt"), "Got: {}", result);
}

#[test]
fn test_s12_b33_raise_in_else() {
    let code = r#"
def lookup(table: dict, key: str) -> int:
    if key in table:
        return table[key]
    else:
        raise KeyError(f"key not found: {key}")
"#;
    let result = transpile(code);
    assert!(result.contains("fn lookup"), "Got: {}", result);
}

// ===== Global and nonlocal =====

#[test]
fn test_s12_b33_global_var() {
    let code = r#"
count = 0

def increment() -> int:
    global count
    count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn increment"), "Got: {}", result);
}

// ===== Assert patterns =====

#[test]
fn test_s12_b33_assert_with_message() {
    let code = r#"
def safe_index(items: list, idx: int) -> int:
    assert idx >= 0, "index must be non-negative"
    assert idx < len(items), "index out of bounds"
    return items[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_index"), "Got: {}", result);
}

// ===== Complex list/dict operations =====

#[test]
fn test_s12_b33_dict_merge() {
    let code = r#"
def merge(d1: dict, d2: dict) -> dict:
    result = {}
    for k, v in d1.items():
        result[k] = v
    for k, v in d2.items():
        result[k] = v
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge"), "Got: {}", result);
}

#[test]
fn test_s12_b33_dict_filter() {
    let code = r#"
def filter_dict(d: dict, threshold: int) -> dict:
    result = {}
    for k, v in d.items():
        if v >= threshold:
            result[k] = v
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b33_list_flatten() {
    let code = r#"
def flatten(nested: list) -> list:
    result = []
    for sublist in nested:
        for item in sublist:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

#[test]
fn test_s12_b33_list_unique() {
    let code = r#"
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
    assert!(result.contains("fn unique"), "Got: {}", result);
}

// ===== Complex boolean expressions =====

#[test]
fn test_s12_b33_not_in_condition() {
    let code = r#"
def has_all_required(data: dict, required: list) -> bool:
    for key in required:
        if key not in data:
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_all_required"), "Got: {}", result);
}

#[test]
fn test_s12_b33_complex_boolean() {
    let code = r#"
def is_valid_email(email: str) -> bool:
    has_at = "@" in email
    has_dot = "." in email
    not_empty = len(email) > 0
    return has_at and has_dot and not_empty
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid_email"), "Got: {}", result);
}

// ===== String formatting patterns =====

#[test]
fn test_s12_b33_fstring_nested_expr() {
    let code = r#"
def format_item(name: str, price: float, qty: int) -> str:
    return f"{name}: ${price * qty:.2f}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_item"), "Got: {}", result);
}

#[test]
fn test_s12_b33_fstring_conditional() {
    let code = r#"
def status_msg(count: int) -> str:
    return f"Found {count} item{'s' if count != 1 else ''}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn status_msg"), "Got: {}", result);
}

// ===== Comprehension edge cases =====

#[test]
fn test_s12_b33_nested_comprehension() {
    let code = r#"
def flatten_comp(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten_comp"), "Got: {}", result);
}

#[test]
fn test_s12_b33_dict_comprehension_complex() {
    let code = r#"
def invert(d: dict) -> dict:
    return {v: k for k, v in d.items()}
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert"), "Got: {}", result);
}

#[test]
fn test_s12_b33_set_comprehension() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_lengths"), "Got: {}", result);
}
