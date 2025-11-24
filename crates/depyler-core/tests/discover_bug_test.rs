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

#[test]
fn test_transpile_array_test() {
    let python = std::fs::read_to_string("../../examples/array_test.py")
        .expect("Should read array_test.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ array_test.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in array_test.py: {}", e),
    }
}

#[test]
fn test_transpile_basic_lambda() {
    let python = std::fs::read_to_string("../../examples/basic_lambda.py")
        .expect("Should read basic_lambda.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ basic_lambda.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in basic_lambda.py: {}", e),
    }
}

#[test]
fn test_transpile_deep_nested_dict() {
    let python = std::fs::read_to_string("../../examples/deep_nested_dict.py")
        .expect("Should read deep_nested_dict.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ deep_nested_dict.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in deep_nested_dict.py: {}", e),
    }
}

#[test]
fn test_transpile_dict_assign() {
    let python = std::fs::read_to_string("../../examples/dict_assign.py")
        .expect("Should read dict_assign.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ dict_assign.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in dict_assign.py: {}", e),
    }
}

#[test]
fn test_transpile_simple_class() {
    let python = std::fs::read_to_string("../../examples/simple_class_test.py")
        .expect("Should read simple_class_test.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ simple_class_test.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in simple_class_test.py: {}", e),
    }
}
