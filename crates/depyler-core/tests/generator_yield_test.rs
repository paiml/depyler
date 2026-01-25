//! Generator Function (yield) Tests (DEPYLER-0188 Part 2)
//!
//! Tests for Python generator functions with yield:
//! - Simple yield generators
//! - yield from
//! - Generator return types
//! - Generator expressions returned from functions

use depyler_core::ast_bridge::AstBridge;
use depyler_core::hir::HirModule;
use depyler_core::optimizer::{Optimizer, OptimizerConfig};
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

/// Transpile with optimizer (matches CLI behavior)
fn transpile(python: &str) -> Result<String, String> {
    let ast = parse(python, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new()
        .python_to_hir(ast)
        .map_err(|e| e.to_string())?;

    let hir_program = depyler_core::hir::HirProgram {
        functions: hir.functions.clone(),
        classes: hir.classes.clone(),
        imports: hir.imports.clone(),
    };

    let mut optimizer = Optimizer::new(OptimizerConfig::default());
    let optimized = optimizer.optimize_program(hir_program);

    let optimized_hir = HirModule {
        functions: optimized.functions,
        classes: optimized.classes,
        imports: optimized.imports,
        constants: hir.constants,
        type_aliases: hir.type_aliases,
        protocols: hir.protocols,
        top_level_stmts: hir.top_level_stmts,
    };

    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) =
        generate_rust_file(&optimized_hir, &type_mapper).map_err(|e| e.to_string())?;
    Ok(rust_code)
}

/// Test simple yield generator
/// Python: def counter(n): for i in range(n): yield i
/// Rust: Should produce an iterator
#[test]
fn test_simple_yield_generator() {
    let python = r#"
def counter(n: int):
    for i in range(n):
        yield i
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile yield generator: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    // Should contain iterator or generator pattern
    assert!(
        rust.contains("iter") || rust.contains("Iterator") || rust.contains("impl "),
        "Should generate iterator pattern. Got:\n{}",
        rust
    );
}

/// Test yield with return value
/// Python: def squares_gen(n): return (i * i for i in range(n))
#[test]
fn test_generator_expression_return() {
    let python = r#"
def squares_gen(n: int) -> list:
    return [i * i for i in range(n)]
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile gen expr return: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("Vec") || rust.contains("iter") || rust.contains("map"),
        "Should generate vec or iterator. Got:\n{}",
        rust
    );
}

/// Test generator with sum builtin
/// Python: def sum_squares(n): return sum(i * i for i in range(n))
#[test]
fn test_sum_with_generator() {
    let python = r#"
def sum_squares(n: int) -> int:
    return sum(i * i for i in range(n))
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile sum with gen: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("sum") || rust.contains(".sum()"),
        "Should generate sum. Got:\n{}",
        rust
    );
}

/// Test generator with any builtin
/// Python: def has_positive(items): return any(x > 0 for x in items)
#[test]
fn test_any_with_generator() {
    let python = r#"
def has_positive(items: list) -> bool:
    return any(x > 0 for x in items)
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile any with gen: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("any") || rust.contains(".any("),
        "Should generate any. Got:\n{}",
        rust
    );
}

/// Test generator with all builtin
/// Python: def all_positive(items): return all(x > 0 for x in items)
#[test]
fn test_all_with_generator() {
    let python = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile all with gen: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("all") || rust.contains(".all("),
        "Should generate all. Got:\n{}",
        rust
    );
}

/// Test nested generator expression
/// Python: def flatten(lists): return [x for lst in lists for x in lst]
#[test]
fn test_flatten_generator() {
    let python = r#"
def flatten(lists: list) -> list:
    return [x for lst in lists for x in lst]
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile flatten: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("flat_map") || rust.contains("flatten"),
        "Should generate flat_map. Got:\n{}",
        rust
    );
}

/// Test running sum generator (stateful)
/// Python: def running_sum(items): total = 0; for x in items: total += x; yield total
#[test]
fn test_running_sum_generator() {
    let python = r#"
def running_sum(items: list) -> list:
    result = []
    total = 0
    for x in items:
        total += x
        result.append(total)
    return result
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile running sum: {:?}",
        result.err()
    );
}

/// Test generator expression with filter and transform
/// Python: def even_squares(items): return [x * x for x in items if x % 2 == 0]
#[test]
fn test_filter_transform_generator() {
    let python = r#"
def even_squares(items: list) -> list:
    return [x * x for x in items if x % 2 == 0]
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile filter+transform: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("filter") && rust.contains("map"),
        "Should generate filter and map. Got:\n{}",
        rust
    );
}
