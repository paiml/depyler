use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

#[test]
fn test_basic_dict_assignment() {
    let python_code = r#"
def test_basic():
    d = {}
    d["key"] = "value"
    d[42] = "number"
    return d
"#;

    // Parse Python to AST
    let ast = parse(python_code, Mode::Module, "<test>").expect("Failed to parse Python");

    // Convert to HIR
    let module = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("Failed to convert to HIR");

    // Generate Rust code
    let type_mapper = TypeMapper::default();
    let result = generate_rust_file(&module, &type_mapper).expect("Failed to generate Rust");

    assert!(result.contains("d.insert"));
    assert!(result.contains(r#""key".to_string()"#));
    assert!(result.contains(r#""value""#));
}

#[test]
fn test_nested_dict_assignment() {
    let python_code = r#"
def test_nested():
    d = {}
    d["outer"] = {}
    d["outer"]["inner"] = "value"
    return d
"#;

    // Parse Python to AST
    let ast = parse(python_code, Mode::Module, "<test>").expect("Failed to parse Python");

    // Convert to HIR
    let module = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("Failed to convert to HIR");

    // Generate Rust code
    let type_mapper = TypeMapper::default();
    let result = generate_rust_file(&module, &type_mapper).expect("Failed to generate Rust");

    assert!(result.contains("get_mut"));
    assert!(result.contains("unwrap()"));
}

#[test]
fn test_deep_nested_dict_assignment() {
    let python_code = r#"
def test_deep():
    d = {}
    d["l1"] = {}
    d["l1"]["l2"] = {}
    d["l1"]["l2"]["l3"] = "deep"
    return d
"#;

    // Parse Python to AST
    let ast = parse(python_code, Mode::Module, "<test>").expect("Failed to parse Python");

    // Convert to HIR
    let module = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("Failed to convert to HIR");

    // Generate Rust code
    let type_mapper = TypeMapper::default();
    let result = generate_rust_file(&module, &type_mapper).expect("Failed to generate Rust");

    // Should have two get_mut calls for the deepest assignment
    let get_mut_count = result.matches("get_mut").count();
    assert!(get_mut_count >= 2);
}

#[test]
fn test_tuple_key_dict() {
    let python_code = r#"
def test_tuple_keys():
    d = {}
    d[(0, 0)] = "origin"
    d[(1, 2)] = "point"
    return d
"#;

    // Parse Python to AST
    let ast = parse(python_code, Mode::Module, "<test>").expect("Failed to parse Python");

    // Convert to HIR
    let module = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("Failed to convert to HIR");

    // Generate Rust code
    let type_mapper = TypeMapper::default();
    let result = generate_rust_file(&module, &type_mapper).expect("Failed to generate Rust");

    assert!(result.contains("(0, 0)"));
    assert!(result.contains("(1, 2)"));
    assert!(result.contains(r#""origin""#));
}
