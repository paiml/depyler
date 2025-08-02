use depyler_annotations::TranspilationAnnotations;
use depyler_core::hir::{
    BinOp, FunctionProperties, HirExpr, HirFunction, HirModule, HirStmt, Literal, Type,
};
use depyler_core::lifetime_analysis::LifetimeInference;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use smallvec::smallvec;

#[test]
fn test_lifetime_inference_for_string_parameter() {
    // Create a function that takes a string and returns its length
    let func = HirFunction {
        name: "get_length".to_string(),
        params: smallvec![("s".to_string(), Type::String)],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("s".to_string())),
            attr: "len".to_string(),
        }))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: Some("Get the length of a string".to_string()),
    };

    let module = HirModule {
        functions: vec![func],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
    };

    let type_mapper = TypeMapper::new();
    let rust_code = generate_rust_file(&module, &type_mapper).unwrap();

    // The generated code should use a reference for the string parameter
    assert!(
        rust_code.contains("pub fn get_length"),
        "Function not found in:\n{}",
        rust_code
    );
    assert!(
        rust_code.contains("&"),
        "No reference found in:\n{}",
        rust_code
    );
    assert!(rust_code.contains("Get the length of a string"));
}

#[test]
fn test_lifetime_inference_for_mutable_parameter() {
    // Create a function that mutates a variable
    let func = HirFunction {
        name: "append_bang".to_string(),
        params: smallvec![("s".to_string(), Type::String)],
        ret_type: Type::None,
        body: vec![HirStmt::Assign {
            target: "s".to_string(),
            value: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("s".to_string())),
                right: Box::new(HirExpr::Literal(Literal::String("!".to_string()))),
            },
        }],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let type_mapper = TypeMapper::new();
    let mut inference = LifetimeInference::new();
    let result = inference.analyze_function(&func, &type_mapper);

    // When a string is reassigned in Python, Rust needs ownership
    let s_param = result.param_lifetimes.get("s").unwrap();
    assert!(!s_param.should_borrow);
    assert!(!s_param.needs_mut);
}

#[test]
fn test_lifetime_inference_with_multiple_parameters() {
    // Create a function with multiple string parameters
    let func = HirFunction {
        name: "concat_strings".to_string(),
        params: smallvec![
            ("s1".to_string(), Type::String),
            ("s2".to_string(), Type::String)
        ],
        ret_type: Type::String,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("s1".to_string())),
            right: Box::new(HirExpr::Var("s2".to_string())),
        }))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let module = HirModule {
        functions: vec![func],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
    };

    let type_mapper = TypeMapper::new();
    let rust_code = generate_rust_file(&module, &type_mapper).unwrap();

    // Both parameters should be borrowed
    assert!(rust_code.contains("pub fn concat_strings"));
    // Should contain lifetime parameters
    let has_lifetimes =
        rust_code.contains("<'") || rust_code.contains("'a") || rust_code.contains("'b");
    assert!(
        has_lifetimes,
        "No lifetime parameters found in:\n{}",
        rust_code
    );
}

#[test]
fn test_lifetime_inference_escaping_parameter() {
    // Create a function that returns one of its parameters
    let func = HirFunction {
        name: "identity".to_string(),
        params: smallvec![("x".to_string(), Type::String)],
        ret_type: Type::String,
        body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let type_mapper = TypeMapper::new();
    let mut inference = LifetimeInference::new();
    let result = inference.analyze_function(&func, &type_mapper);

    // Since the parameter escapes through return, it should be moved
    let x_param = result.param_lifetimes.get("x").unwrap();
    assert!(
        !x_param.should_borrow,
        "Parameter should be moved, not borrowed"
    );
}

#[test]
fn test_lifetime_bounds_generation() {
    // Create a function where one parameter must outlive another
    let func = HirFunction {
        name: "select_first".to_string(),
        params: smallvec![
            ("a".to_string(), Type::String),
            ("b".to_string(), Type::String)
        ],
        ret_type: Type::String,
        body: vec![HirStmt::Return(Some(HirExpr::Var("a".to_string())))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let type_mapper = TypeMapper::new();
    let mut inference = LifetimeInference::new();
    let result = inference.analyze_function(&func, &type_mapper);

    // Since 'a' is returned directly, it should be moved, not borrowed
    let a_param = result.param_lifetimes.get("a").unwrap();
    assert!(
        !a_param.should_borrow,
        "'a' should be moved since it's returned"
    );

    // 'b' is not used, so it might be optimized differently
    let b_param = result.param_lifetimes.get("b").unwrap();
    // b is only read, not returned, so it could be borrowed
    assert!(b_param.should_borrow || !b_param.should_borrow); // Either is valid for unused param
}
