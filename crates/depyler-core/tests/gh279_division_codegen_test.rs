//! GH-279: Python true division `a / b` must never lower to `.join(...)`.
//! Only a statically-Path left operand joins (pathlib `/`).

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

/// The user-function body: text after the last `pub fn <name>` up to its closing brace.
fn fn_body(code: &str, name: &str) -> String {
    let start = code.find(&format!("pub fn {name}(")).expect("fn present");
    let rest = &code[start..];
    rest[..rest.find("\n}").expect("fn end")].to_string()
}

#[test]
fn test_gh279_float_float_division_no_join() {
    let code = transpile("def half(a: float, b: float) -> float:\n    return a / b\n");
    let body = fn_body(&code, "half");
    assert!(!body.contains(".join("), "float/float must not join: {body}");
    assert!(body.contains("py_div") || body.contains(" / "), "must divide: {body}");
}

#[test]
fn test_gh279_int_int_division_no_join() {
    let code = transpile("def ratio(a: int, b: int) -> float:\n    return a / b\n");
    let body = fn_body(&code, "ratio");
    assert!(!body.contains(".join("), "int/int must not join: {body}");
}

#[test]
fn test_gh279_float_cast_division_no_join() {
    let code = transpile("def r(a: int, b: int) -> float:\n    return float(a) / float(b)\n");
    let body = fn_body(&code, "r");
    assert!(!body.contains(".join("), "float(x)/float(y) must not join: {body}");
}

#[test]
fn test_gh279_len_division_no_join() {
    let code =
        transpile("def rate(errors: int, codes: list) -> float:\n    return errors / len(codes)\n");
    let body = fn_body(&code, "rate");
    assert!(!body.contains(".join("), "errors / len(codes) must not join: {body}");
}

#[test]
fn test_gh279_path_div_str_still_joins() {
    let code = transpile(
        "from pathlib import Path\n\ndef child(base: str) -> str:\n    q = Path(base)\n    r = q / \"x\"\n    return str(r)\n",
    );
    let body = fn_body(&code, "child");
    assert!(body.contains(".join("), "Path / str must still join: {body}");
    assert!(!body.contains("py_div"), "Path / str must not py_div: {body}");
}
