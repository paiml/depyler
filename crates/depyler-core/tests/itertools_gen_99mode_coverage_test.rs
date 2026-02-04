//! Coverage tests for rust_gen/stdlib_method_gen/itertools.rs
//!
//! DEPYLER-99MODE-001: Targets itertools.rs (1,140 lines)
//! Covers: Python itertools module to Rust iterator adapter mapping,
//! chain, islice, takewhile, accumulate, product, permutations, combinations.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_itertools_import() {
    let code = r#"
import itertools

def f(a: list, b: list) -> list:
    return list(itertools.chain(a, b))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_chain() {
    let code = r#"
import itertools

def f(a: list, b: list) -> list:
    result = []
    for item in itertools.chain(a, b):
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_cycle() {
    let code = r#"
import itertools

def f(items: list, n: int) -> list:
    result = []
    count = 0
    for item in itertools.cycle(items):
        if count >= n:
            break
        result.append(item)
        count += 1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_repeat() {
    let code = r#"
import itertools

def f(val: int, n: int) -> list:
    return list(itertools.repeat(val, n))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_combinations() {
    let code = r#"
import itertools

def f(items: list) -> list:
    return list(itertools.combinations(items, 2))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_permutations() {
    let code = r#"
import itertools

def f(items: list) -> list:
    return list(itertools.permutations(items, 2))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_product() {
    let code = r#"
import itertools

def f(a: list, b: list) -> list:
    return list(itertools.product(a, b))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_islice() {
    let code = r#"
import itertools

def f(items: list, n: int) -> list:
    return list(itertools.islice(items, n))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_takewhile() {
    let code = r#"
import itertools

def f(items: list) -> list:
    return list(itertools.takewhile(lambda x: x > 0, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_dropwhile() {
    let code = r#"
import itertools

def f(items: list) -> list:
    return list(itertools.dropwhile(lambda x: x < 0, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_accumulate() {
    let code = r#"
import itertools

def f(items: list) -> list:
    return list(itertools.accumulate(items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_zip_longest() {
    let code = r#"
import itertools

def f(a: list, b: list) -> list:
    return list(itertools.zip_longest(a, b))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_compress() {
    let code = r#"
import itertools

def f(data: list, selectors: list) -> list:
    return list(itertools.compress(data, selectors))
"#;
    assert!(transpile_ok(code));
}
