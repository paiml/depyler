// EXTREME TDD: Tests for default parameter functionality (DEPYLER-0104)
// These tests define the expected behavior BEFORE implementation

use depyler_annotations::TranspilationAnnotations;
use depyler_core::hir::{FunctionProperties, HirExpr, HirFunction, HirParam, Literal, Type};
use smallvec::smallvec;

/// Test 1: Function with one default parameter (None)
#[test]
fn test_function_with_none_default() {
    let func = HirFunction {
        name: "greet".to_string(),
        params: smallvec![
            HirParam {
                name: "name".to_string(),
                ty: Type::String,
                default: None,
                    is_vararg: false,
            },
            HirParam {
                name: "greeting".to_string(),
                ty: Type::Optional(Box::new(Type::String)),
                default: Some(HirExpr::Literal(Literal::None)),
            },
        ],
        ret_type: Type::String,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    assert_eq!(func.params.len(), 2);
    assert_eq!(func.params[0].name, "name");
    assert_eq!(func.params[0].default, None);
    assert_eq!(func.params[1].name, "greeting");
    assert!(func.params[1].default.is_some());
}

/// Test 2: Function with integer default value
#[test]
fn test_function_with_int_default() {
    let func = HirFunction {
        name: "increment".to_string(),
        params: smallvec![
            HirParam {
                name: "x".to_string(),
                ty: Type::Int,
                default: None,
                    is_vararg: false,
            },
            HirParam {
                name: "step".to_string(),
                ty: Type::Int,
                default: Some(HirExpr::Literal(Literal::Int(1))),
            },
        ],
        ret_type: Type::Int,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    assert_eq!(func.params.len(), 2);
    assert_eq!(func.params[1].name, "step");
    match &func.params[1].default {
        Some(HirExpr::Literal(Literal::Int(1))) => {}
        _ => panic!("Expected default value of 1"),
    }
}

/// Test 3: Function with string default value
#[test]
fn test_function_with_string_default() {
    let func = HirFunction {
        name: "log".to_string(),
        params: smallvec![
            HirParam {
                name: "message".to_string(),
                ty: Type::String,
                default: None,
                    is_vararg: false,
            },
            HirParam {
                name: "level".to_string(),
                ty: Type::String,
                default: Some(HirExpr::Literal(Literal::String("INFO".to_string()))),
            },
        ],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    match &func.params[1].default {
        Some(HirExpr::Literal(Literal::String(s))) if s == "INFO" => {}
        _ => panic!("Expected default value of 'INFO'"),
    }
}

/// Test 4: Function with boolean default value
#[test]
fn test_function_with_bool_default() {
    let func = HirFunction {
        name: "process".to_string(),
        params: smallvec![
            HirParam {
                name: "data".to_string(),
                ty: Type::String,
                default: None,
                    is_vararg: false,
            },
            HirParam {
                name: "verbose".to_string(),
                ty: Type::Bool,
                default: Some(HirExpr::Literal(Literal::Bool(false))),
            },
        ],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    match &func.params[1].default {
        Some(HirExpr::Literal(Literal::Bool(false))) => {}
        _ => panic!("Expected default value of false"),
    }
}

/// Test 5: Function with list default (empty list)
#[test]
fn test_function_with_empty_list_default() {
    let func = HirFunction {
        name: "extend".to_string(),
        params: smallvec![HirParam {
            name: "items".to_string(),
            ty: Type::List(Box::new(Type::Int)),
            default: Some(HirExpr::List(vec![])),
        },],
        ret_type: Type::List(Box::new(Type::Int)),
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    match &func.params[0].default {
        Some(HirExpr::List(items)) if items.is_empty() => {}
        _ => panic!("Expected default value of empty list"),
    }
}

/// Test 6: Function with dict/HashMap default (None → empty dict pattern)
#[test]
fn test_function_with_dict_none_default() {
    let func = HirFunction {
        name: "fibonacci_memo".to_string(),
        params: smallvec![
            HirParam {
                name: "n".to_string(),
                ty: Type::Int,
                default: None,
                    is_vararg: false,
            },
            HirParam {
                name: "memo".to_string(),
                ty: Type::Optional(Box::new(Type::Dict(
                    Box::new(Type::Int),
                    Box::new(Type::Int)
                ))),
                default: Some(HirExpr::Literal(Literal::None)),
            },
        ],
        ret_type: Type::Int,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    // This is the fibonacci_memo.py pattern: def fib(n, memo: Dict[int, int] = None)
    assert_eq!(func.params[1].name, "memo");
    assert!(matches!(func.params[1].ty, Type::Optional(_)));
    assert!(matches!(
        func.params[1].default,
        Some(HirExpr::Literal(Literal::None))
    ));
}

/// Test 7: Multiple defaults in sequence
#[test]
fn test_function_with_multiple_defaults() {
    let func = HirFunction {
        name: "configure".to_string(),
        params: smallvec![
            HirParam {
                name: "host".to_string(),
                ty: Type::String,
                default: Some(HirExpr::Literal(Literal::String("localhost".to_string()))),
            },
            HirParam {
                name: "port".to_string(),
                ty: Type::Int,
                default: Some(HirExpr::Literal(Literal::Int(8080))),
            },
            HirParam {
                name: "debug".to_string(),
                ty: Type::Bool,
                default: Some(HirExpr::Literal(Literal::Bool(false))),
            },
        ],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    assert!(func.params.iter().all(|p| p.default.is_some()));
}

/// Test 8: No default parameters (backward compatibility)
#[test]
fn test_function_with_no_defaults() {
    let func = HirFunction {
        name: "add".to_string(),
        params: smallvec![
            HirParam {
                name: "a".to_string(),
                ty: Type::Int,
                default: None,
                    is_vararg: false,
            },
            HirParam {
                name: "b".to_string(),
                ty: Type::Int,
                default: None,
                    is_vararg: false,
            },
        ],
        ret_type: Type::Int,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    assert!(func.params.iter().all(|p| p.default.is_none()));
}

/// Test 9: Mixed parameters (some with defaults, some without)
#[test]
fn test_function_with_mixed_defaults() {
    let func = HirFunction {
        name: "fetch".to_string(),
        params: smallvec![
            HirParam {
                name: "url".to_string(),
                ty: Type::String,
                default: None,
                    is_vararg: false,
            },
            HirParam {
                name: "timeout".to_string(),
                ty: Type::Int,
                default: Some(HirExpr::Literal(Literal::Int(30))),
            },
            HirParam {
                name: "retries".to_string(),
                ty: Type::Int,
                default: Some(HirExpr::Literal(Literal::Int(3))),
            },
        ],
        ret_type: Type::String,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    assert_eq!(func.params[0].default, None);
    assert!(func.params[1].default.is_some());
    assert!(func.params[2].default.is_some());
}

/// Test 10: Float default value
#[test]
fn test_function_with_float_default() {
    let func = HirFunction {
        name: "scale".to_string(),
        params: smallvec![
            HirParam {
                name: "value".to_string(),
                ty: Type::Float,
                default: None,
                    is_vararg: false,
            },
            HirParam {
                name: "factor".to_string(),
                ty: Type::Float,
                default: Some(HirExpr::Literal(Literal::Float(1.0))),
            },
        ],
        ret_type: Type::Float,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    match &func.params[1].default {
        Some(HirExpr::Literal(Literal::Float(f))) if (*f - 1.0).abs() < f64::EPSILON => {}
        _ => panic!("Expected default value of 1.0"),
    }
}

/// Test 11: HirParam helper constructor
#[test]
fn test_hir_param_constructor() {
    let param = HirParam::new("x".to_string(), Type::Int);
    assert_eq!(param.name, "x");
    assert_eq!(param.ty, Type::Int);
    assert_eq!(param.default, None);

    let param_with_default = HirParam::with_default(
        "y".to_string(),
        Type::Int,
        HirExpr::Literal(Literal::Int(10)),
    );
    assert_eq!(param_with_default.name, "y");
    assert!(param_with_default.default.is_some());
}

/// Test 12: Serialization/deserialization of HirParam
#[test]
fn test_hir_param_serde() {
    let param = HirParam {
        name: "count".to_string(),
        ty: Type::Int,
        default: Some(HirExpr::Literal(Literal::Int(0))),
    };

    let json = serde_json::to_string(&param).unwrap();
    let deserialized: HirParam = serde_json::from_str(&json).unwrap();

    assert_eq!(param, deserialized);
}
