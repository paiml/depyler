//! Bug discovery test - systematically transpile examples to find next bug

use depyler_core::ast_bridge;
use rustpython_ast::Suite;
use rustpython_parser::{ast, Parse};

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
    let python =
        std::fs::read_to_string("../../examples/array_test.py").expect("Should read array_test.py");

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

#[test]
fn test_transpile_basic_imports() {
    let python = std::fs::read_to_string("../../examples/test_basic_imports.py")
        .expect("Should read test_basic_imports.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ test_basic_imports.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in test_basic_imports.py: {}", e),
    }
}

#[test]
fn test_transpile_module_mapping() {
    let python = std::fs::read_to_string("../../examples/module_mapping_demo.py")
        .expect("Should read module_mapping_demo.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ module_mapping_demo.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in module_mapping_demo.py: {}", e),
    }
}

#[test]
fn test_transpile_floor_division() {
    let python = std::fs::read_to_string("../../examples/floor_division_test.py")
        .expect("Should read floor_division_test.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ floor_division_test.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in floor_division_test.py: {}", e),
    }
}

#[test]
fn test_transpile_dict_assign_typed() {
    let python = std::fs::read_to_string("../../examples/dict_assign_typed.py")
        .expect("Should read dict_assign_typed.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ dict_assign_typed.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in dict_assign_typed.py: {}", e),
    }
}

#[test]
fn test_transpile_lambda_advanced() {
    let python = std::fs::read_to_string("../../examples/lambda_advanced_test.py")
        .expect("Should read lambda_advanced_test.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ lambda_advanced_test.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in lambda_advanced_test.py: {}", e),
    }
}

#[test]
fn test_transpile_custom_attributes() {
    let python = std::fs::read_to_string("../../examples/custom_attributes_demo.py")
        .expect("Should read custom_attributes_demo.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ custom_attributes_demo.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in custom_attributes_demo.py: {}", e),
    }
}

#[test]
fn test_transpile_type_hints_simple() {
    let python = std::fs::read_to_string("../../examples/test_type_hints_simple.py")
        .expect("Should read test_type_hints_simple.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ test_type_hints_simple.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in test_type_hints_simple.py: {}", e),
    }
}

#[test]
fn test_transpile_tuple_key_dict() {
    let python = std::fs::read_to_string("../../examples/tuple_key_dict.py")
        .expect("Should read tuple_key_dict.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ tuple_key_dict.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in tuple_key_dict.py: {}", e),
    }
}

#[test]
fn test_transpile_type_inference_demo() {
    let python = std::fs::read_to_string("../../examples/type_inference_demo.py")
        .expect("Should read type_inference_demo.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ type_inference_demo.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in type_inference_demo.py: {}", e),
    }
}

#[test]
fn test_transpile_with_statement() {
    let python = std::fs::read_to_string("../../examples/test_with_statement.py")
        .expect("Should read test_with_statement.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ test_with_statement.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in test_with_statement.py: {}", e),
    }
}

#[test]
fn test_transpile_functional_programming_combined() {
    let python = std::fs::read_to_string("../../examples/functional_programming_combined.py")
        .expect("Should read functional_programming_combined.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ functional_programming_combined.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in functional_programming_combined.py: {}", e),
    }
}

#[test]
fn test_transpile_data_analysis_combined() {
    let python = std::fs::read_to_string("../../examples/data_analysis_combined.py")
        .expect("Should read data_analysis_combined.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ data_analysis_combined.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in data_analysis_combined.py: {}", e),
    }
}

#[test]
fn test_transpile_text_processing_combined() {
    let python = std::fs::read_to_string("../../examples/text_processing_combined.py")
        .expect("Should read text_processing_combined.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ text_processing_combined.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in text_processing_combined.py: {}", e),
    }
}

#[test]
fn test_transpile_simulation_combined() {
    let python = std::fs::read_to_string("../../examples/simulation_combined.py")
        .expect("Should read simulation_combined.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ simulation_combined.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in simulation_combined.py: {}", e),
    }
}

#[test]
fn test_transpile_debugging_workflow() {
    let python = std::fs::read_to_string("../../examples/debugging_workflow.py")
        .expect("Should read debugging_workflow.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ debugging_workflow.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in debugging_workflow.py: {}", e),
    }
}

#[test]
fn test_transpile_lifetime_demo() {
    let python = std::fs::read_to_string("../../examples/lifetime_demo.py")
        .expect("Should read lifetime_demo.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ lifetime_demo.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in lifetime_demo.py: {}", e),
    }
}

#[test]
fn test_transpile_lambda_demo() {
    let python = std::fs::read_to_string("../../examples/lambda_demo.py")
        .expect("Should read lambda_demo.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ lambda_demo.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in lambda_demo.py: {}", e),
    }
}

#[test]
fn test_transpile_demo() {
    let python = std::fs::read_to_string("../../examples/demo.py").expect("Should read demo.py");

    let statements = Suite::parse(&python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });

    let result = ast_bridge::AstBridge::new().python_to_hir(ast);

    match result {
        Ok(_) => println!("✅ demo.py transpiles successfully"),
        Err(e) => panic!("❌ Found bug in demo.py: {}", e),
    }
}
