//! Bug discovery test - systematically transpile examples to find next bug

use depyler_core::ast_bridge;
use rustpython_parser::{Parse, ast};
use rustpython_ast::Suite;

#[test]
fn test_transpile_basic_class_test() {
    let python = std::fs::read_to_string("../../examples/basic_class_test.py")
        .expect("Should read basic_class_test.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    // Try to generate HIR - this should reveal the bug
    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ basic_class_test.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in basic_class_test.py: {}", e),
    }
}

#[test]
fn test_transpile_ast_converters_demo() {
    let python = std::fs::read_to_string("../../examples/ast_converters_demo.py")
        .expect("Should read ast_converters_demo.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    // Try to generate HIR - this should reveal the bug
    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ ast_converters_demo.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in ast_converters_demo.py: {}", e),
    }
}
