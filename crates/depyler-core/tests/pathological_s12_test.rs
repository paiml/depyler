//! Session 12 Batch 99: Pathological and stress patterns
//!
//! Tests with unusual or pathological patterns that exercise
//! rarely-hit codegen branches.

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
fn test_s12_b99_many_params() {
    let code = r#"
def many(a: int, b: int, c: int, d: int, e: int, f: int) -> int:
    return a + b + c + d + e + f
"#;
    let result = transpile(code);
    assert!(result.contains("fn many"), "Got: {}", result);
}

#[test]
fn test_s12_b99_many_returns() {
    let code = r#"
def classify(x: int) -> str:
    if x < -100:
        return "very negative"
    if x < -10:
        return "negative"
    if x < 0:
        return "small negative"
    if x == 0:
        return "zero"
    if x < 10:
        return "small positive"
    if x < 100:
        return "positive"
    return "very positive"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

#[test]
fn test_s12_b99_deeply_nested_if() {
    let code = r#"
def deep_check(a: int, b: int, c: int) -> str:
    if a > 0:
        if b > 0:
            if c > 0:
                return "all positive"
            else:
                return "c not positive"
        else:
            if c > 0:
                return "b not positive"
            else:
                return "b and c not positive"
    else:
        return "a not positive"
"#;
    let result = transpile(code);
    assert!(result.contains("fn deep_check"), "Got: {}", result);
}

#[test]
fn test_s12_b99_long_chain() {
    let code = r#"
def transform(text: str) -> str:
    result = text.strip()
    result = result.lower()
    result = result.replace("  ", " ")
    result = result.replace("\t", " ")
    result = result.replace("\n", " ")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn transform"), "Got: {}", result);
}

#[test]
fn test_s12_b99_many_list_ops() {
    let code = r#"
def build_list() -> list:
    items = []
    items.append(1)
    items.append(2)
    items.append(3)
    items.insert(0, 0)
    items.extend([4, 5])
    items.pop()
    items.remove(2)
    items.reverse()
    items.sort()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_list"), "Got: {}", result);
}

#[test]
fn test_s12_b99_many_dict_ops() {
    let code = r##"
def build_dict() -> dict:
    d = {}
    d["a"] = 1
    d["b"] = 2
    d["c"] = 3
    d.update({"d": 4})
    val = d.pop("b", 0)
    d.setdefault("e", 5)
    keys = list(d.keys())
    values = list(d.values())
    return d
"##;
    let result = transpile(code);
    assert!(result.contains("fn build_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b99_many_set_ops() {
    let code = r#"
def build_set() -> set:
    s = set()
    s.add(1)
    s.add(2)
    s.add(3)
    s.discard(2)
    s.add(4)
    other = {3, 4, 5}
    combined = s.union(other)
    common = s.intersection(other)
    diff = s.difference(other)
    return combined
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_set"), "Got: {}", result);
}

#[test]
fn test_s12_b99_complex_comprehension() {
    let code = r#"
def matrix_ops(n: int) -> list:
    identity = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    doubled = [[cell * 2 for cell in row] for row in identity]
    flat = [cell for row in doubled for cell in row]
    nonzero = [x for x in flat if x != 0]
    return nonzero
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix_ops"), "Got: {}", result);
}

#[test]
fn test_s12_b99_exception_types() {
    let code = r##"
def safe_convert(value: str) -> str:
    try:
        return str(int(value))
    except ValueError:
        pass
    try:
        return str(float(value))
    except ValueError:
        pass
    try:
        if value.lower() in ("true", "false"):
            return value.lower()
    except AttributeError:
        pass
    return value
"##;
    let result = transpile(code);
    assert!(result.contains("fn safe_convert"), "Got: {}", result);
}

#[test]
fn test_s12_b99_multiple_loops() {
    let code = r#"
def find_common(a: list, b: list, c: list) -> list:
    result = []
    for x in a:
        found_in_b = False
        for y in b:
            if x == y:
                found_in_b = True
                break
        if not found_in_b:
            continue
        for z in c:
            if x == z:
                result.append(x)
                break
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_common"), "Got: {}", result);
}

#[test]
fn test_s12_b99_fibonacci_iterative() {
    let code = r#"
def fibonacci(n: int) -> list:
    if n <= 0:
        return []
    if n == 1:
        return [0]
    result = [0, 1]
    for i in range(2, n):
        result.append(result[i - 1] + result[i - 2])
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn fibonacci"), "Got: {}", result);
}

#[test]
fn test_s12_b99_prime_sieve() {
    let code = r#"
def sieve(limit: int) -> list:
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = False
    is_prime[1] = False
    i = 2
    while i * i <= limit:
        if is_prime[i]:
            j = i * i
            while j <= limit:
                is_prime[j] = False
                j += i
        i += 1
    return [i for i in range(limit + 1) if is_prime[i]]
"#;
    let result = transpile(code);
    assert!(result.contains("fn sieve"), "Got: {}", result);
}
