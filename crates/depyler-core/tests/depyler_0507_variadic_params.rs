//! DEPYLER-0507: Variadic parameters (*args) missing from function signatures
//!
//! RED → GREEN → REFACTOR Phase: Fix variadic parameter transpilation
//!
//! Root Cause (Five-Whys):
//! 1. Why? Compilation error: "cannot find value 'parts' in this scope"
//! 2. Why? Function signature has zero parameters
//! 3. Why? Variadic parameter not added to params vector
//! 4. Why? convert_nested_function_params() never checks args.vararg field
//! 5. ROOT: Missing code to handle ast::Arguments::vararg

#![allow(non_snake_case)]

use depyler_core::ast_bridge;
use rustpython_ast::Suite;
use rustpython_parser::{ast, Parse};

fn parse_and_generate(python: &str) -> depyler_core::hir::HirModule {
    let statements = Suite::parse(python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });
    let (hir, _type_env) = ast_bridge::AstBridge::new()
        .python_to_hir(ast)
        .expect("Should generate HIR");
    hir
}

#[test]
fn test_simple_variadic_function() {
    // Test 1: Simple variadic function
    let python = r#"
def concat(*args):
    return "".join(args)

result = concat("a", "b", "c")
"#;
    let hir = parse_and_generate(python);

    // Find the concat function
    let concat_func = hir.functions.iter().find(|f| f.name == "concat");
    assert!(concat_func.is_some(), "concat function should exist");

    let concat_func = concat_func.unwrap();
    assert_eq!(concat_func.params.len(), 1, "Should have 1 parameter");
    assert_eq!(concat_func.params[0].name, "args");
    assert!(
        concat_func.params[0].is_vararg,
        "Parameter should be marked as vararg"
    );
}

#[test]
fn test_variadic_with_regular_params() {
    // Test 2: Variadic with regular parameters
    let python = r#"
def format_msg(prefix, *parts):
    return prefix + ": " + " ".join(parts)

msg = format_msg("INFO", "server", "started")
"#;
    let hir = parse_and_generate(python);

    let format_func = hir.functions.iter().find(|f| f.name == "format_msg");
    assert!(format_func.is_some());

    let format_func = format_func.unwrap();
    assert_eq!(format_func.params.len(), 2, "Should have 2 parameters");

    // First param is regular
    assert_eq!(format_func.params[0].name, "prefix");
    assert!(
        !format_func.params[0].is_vararg,
        "prefix should NOT be vararg"
    );

    // Second param is variadic
    assert_eq!(format_func.params[1].name, "parts");
    assert!(format_func.params[1].is_vararg, "parts should be vararg");
}

#[test]
fn test_variadic_path_join() {
    // Test 3: Path joining (from env_info.py reproducer)
    let python = r#"
def join_paths(*parts):
    return os.path.join(*parts)

path = join_paths("home", "user", "docs")
"#;
    let hir = parse_and_generate(python);

    let join_func = hir.functions.iter().find(|f| f.name == "join_paths");
    assert!(join_func.is_some());

    let join_func = join_func.unwrap();
    assert_eq!(join_func.params.len(), 1);
    assert_eq!(join_func.params[0].name, "parts");
    assert!(join_func.params[0].is_vararg);
}

#[test]
fn test_nested_variadic_function() {
    // Test 4: Nested function with variadic params
    let python = r#"
def outer():
    def inner(*args):
        return len(args)
    return inner(1, 2, 3)
"#;
    let hir = parse_and_generate(python);

    let outer_func = hir.functions.iter().find(|f| f.name == "outer");
    assert!(outer_func.is_some());

    // Check outer function's body contains nested function definition
    let outer_func = outer_func.unwrap();

    // Find FunctionDef statement in body
    use depyler_core::hir::HirStmt;
    let nested_func = outer_func.body.iter().find_map(|stmt| {
        if let HirStmt::FunctionDef { name, params, .. } = stmt {
            if name == "inner" {
                Some(params)
            } else {
                None
            }
        } else {
            None
        }
    });

    assert!(nested_func.is_some(), "Should have nested inner function");
    let nested_params = nested_func.unwrap();
    assert_eq!(nested_params.len(), 1);
    assert_eq!(nested_params[0].name, "args");
    assert!(
        nested_params[0].is_vararg,
        "Nested function args should be vararg"
    );
}

#[test]
fn test_variadic_with_no_usage() {
    // Test 5: Variadic parameter that's never used (edge case)
    let python = r#"
def noop(*args):
    pass
"#;
    let hir = parse_and_generate(python);

    let noop_func = hir.functions.iter().find(|f| f.name == "noop");
    assert!(noop_func.is_some());

    let noop_func = noop_func.unwrap();
    assert_eq!(noop_func.params.len(), 1);
    assert!(noop_func.params[0].is_vararg);
}

#[test]
fn test_variadic_with_defaults() {
    // Test 6: Mix of regular params with defaults and variadic
    let python = r#"
def complex_func(a, b=10, *args):
    return a + b + sum(args)
"#;
    let hir = parse_and_generate(python);

    let complex_func = hir.functions.iter().find(|f| f.name == "complex_func");
    assert!(complex_func.is_some());

    let complex_func = complex_func.unwrap();
    assert_eq!(complex_func.params.len(), 3);

    assert_eq!(complex_func.params[0].name, "a");
    assert!(!complex_func.params[0].is_vararg);
    assert!(complex_func.params[0].default.is_none());

    assert_eq!(complex_func.params[1].name, "b");
    assert!(!complex_func.params[1].is_vararg);
    assert!(complex_func.params[1].default.is_some());

    assert_eq!(complex_func.params[2].name, "args");
    assert!(complex_func.params[2].is_vararg);
}
