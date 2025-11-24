//! DEPYLER-0512: argparse.Namespace Type Annotation Support
//!
//! **ROOT CAUSE**: Type extractor doesn't support module-qualified types (module.Class)
//!
//! **Five Whys**:
//! 1. Why fails? `argparse.Namespace` is `Attribute` AST node
//! 2. Why not handled? No match arm for `ast::Expr::Attribute`
//! 3. Why no attribute support? Most types don't need module prefix
//! 4. Why does argparse need it? It's a concrete stdlib class
//! 5. ROOT: Missing module-qualified type support (module.Class pattern)

#![allow(non_snake_case)]

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile_to_rust(python_code: &str) -> Result<String, String> {
    let ast = parse(python_code, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new()
        .python_to_hir(ast)
        .map_err(|e| e.to_string())?;
    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) = generate_rust_file(&hir, &type_mapper).map_err(|e| e.to_string())?;
    Ok(rust_code)
}

// ============================================================================
// RED PHASE - Failing Tests
// ============================================================================

#[test]
fn test_DEPYLER_0512_argparse_namespace_return_type() {
    // RED: Should fail with "Unsupported type annotation: Attribute"
    let python = r#"
import argparse

def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    return parser.parse_args()
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0512: argparse.Namespace should be supported. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should generate valid Rust (exact type TBD - could be struct or HashMap)
    assert!(
        !rust_code.is_empty(),
        "DEPYLER-0512: Should generate non-empty Rust code"
    );
}

#[test]
fn test_DEPYLER_0512_other_module_qualified_types() {
    // Test other module.Class patterns
    let python = r#"
import typing

def example() -> typing.Any:
    return 42
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0512: typing.Any should be supported. Error:\n{}",
        result.unwrap_err()
    );
}

#[test]
fn test_DEPYLER_0512_pathlib_path() {
    // Common stdlib pattern: pathlib.Path
    let python = r#"
import pathlib

def get_path() -> pathlib.Path:
    return pathlib.Path("/tmp")
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0512: pathlib.Path should be supported. Error:\n{}",
        result.unwrap_err()
    );
}

#[test]
fn test_DEPYLER_0512_nested_module_qualified() {
    // Nested module.submodule.Class
    let python = r#"
import collections.abc

def example() -> collections.abc.Iterable:
    return [1, 2, 3]
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0512: collections.abc.Iterable should be supported. Error:\n{}",
        result.unwrap_err()
    );
}
