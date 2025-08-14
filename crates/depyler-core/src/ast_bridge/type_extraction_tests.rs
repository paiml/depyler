use crate::ast_bridge::type_extraction::TypeExtractor;
use crate::hir::Type;
use rustpython_ast::Expr;
use rustpython_parser::Parse;

#[test]
fn test_extract_simple_types() {
    let cases = vec![
        ("int", Type::Int),
        ("float", Type::Float),
        ("str", Type::String),
        ("bool", Type::Bool),
        ("None", Type::None),
    ];

    for (input, expected) in cases {
        let result = TypeExtractor::extract_simple_type(input).unwrap();
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_extract_generic_simple_types() {
    // Test plain generic types without parameters
    assert_eq!(
        TypeExtractor::extract_simple_type("list").unwrap(),
        Type::List(Box::new(Type::Unknown))
    );
    assert_eq!(
        TypeExtractor::extract_simple_type("dict").unwrap(),
        Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown))
    );
    assert_eq!(
        TypeExtractor::extract_simple_type("set").unwrap(),
        Type::Set(Box::new(Type::Unknown))
    );
}

#[test]
fn test_extract_custom_types() {
    let custom_type = TypeExtractor::extract_simple_type("MyClass").unwrap();
    assert_eq!(custom_type, Type::Custom("MyClass".to_string()));

    let custom_type2 = TypeExtractor::extract_simple_type("DataFrame").unwrap();
    assert_eq!(custom_type2, Type::Custom("DataFrame".to_string()));
}

#[test]
fn test_extract_type_variables() {
    // Single uppercase letters are type variables
    let cases = vec!["T", "U", "V", "K"];

    for type_var in cases {
        let result = TypeExtractor::extract_simple_type(type_var).unwrap();
        assert_eq!(result, Type::TypeVar(type_var.to_string()));
    }

    // Multi-letter names are custom types, not type variables
    let custom = TypeExtractor::extract_simple_type("TT").unwrap();
    assert_eq!(custom, Type::Custom("TT".to_string()));
}

#[test]
fn test_extract_return_type_none() {
    let result = TypeExtractor::extract_return_type(&None).unwrap();
    assert_eq!(result, Type::Unknown);
}

#[test]
fn test_extract_list_type() {
    let expr = Expr::parse("List[int]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(ty, Type::List(Box::new(Type::Int)));

    let expr2 = Expr::parse("List[str]", "<test>").unwrap();
    let ty2 = TypeExtractor::extract_type(&expr2).unwrap();
    assert_eq!(ty2, Type::List(Box::new(Type::String)));
}

#[test]
fn test_extract_dict_type() {
    let expr = Expr::parse("Dict[str, int]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(ty, Type::Dict(Box::new(Type::String), Box::new(Type::Int)));

    let expr2 = Expr::parse("Dict[int, float]", "<test>").unwrap();
    let ty2 = TypeExtractor::extract_type(&expr2).unwrap();
    assert_eq!(ty2, Type::Dict(Box::new(Type::Int), Box::new(Type::Float)));
}

#[test]
fn test_extract_set_type() {
    let expr = Expr::parse("Set[str]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(ty, Type::Set(Box::new(Type::String)));
}

#[test]
fn test_extract_optional_type() {
    let expr = Expr::parse("Optional[int]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(ty, Type::Optional(Box::new(Type::Int)));

    let expr2 = Expr::parse("Optional[str]", "<test>").unwrap();
    let ty2 = TypeExtractor::extract_type(&expr2).unwrap();
    assert_eq!(ty2, Type::Optional(Box::new(Type::String)));
}

#[test]
fn test_extract_union_type() {
    let expr = Expr::parse("Union[int, str]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(ty, Type::Union(vec![Type::Int, Type::String]));

    // Single type in Union
    let expr2 = Expr::parse("Union[int]", "<test>").unwrap();
    let ty2 = TypeExtractor::extract_type(&expr2).unwrap();
    assert_eq!(ty2, Type::Union(vec![Type::Int]));

    // Multiple types
    let expr3 = Expr::parse("Union[int, str, float]", "<test>").unwrap();
    let ty3 = TypeExtractor::extract_type(&expr3).unwrap();
    assert_eq!(ty3, Type::Union(vec![Type::Int, Type::String, Type::Float]));
}

#[test]
fn test_extract_tuple_type() {
    let expr = Expr::parse("tuple[int, str]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(ty, Type::Tuple(vec![Type::Int, Type::String]));

    // Single element tuple
    let expr2 = Expr::parse("tuple[int]", "<test>").unwrap();
    let ty2 = TypeExtractor::extract_type(&expr2).unwrap();
    assert_eq!(ty2, Type::Tuple(vec![Type::Int]));

    // Multiple elements
    let expr3 = Expr::parse("tuple[int, str, bool]", "<test>").unwrap();
    let ty3 = TypeExtractor::extract_type(&expr3).unwrap();
    assert_eq!(ty3, Type::Tuple(vec![Type::Int, Type::String, Type::Bool]));
}

#[test]
fn test_extract_nested_types() {
    // List of Lists
    let expr = Expr::parse("List[List[int]]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(ty, Type::List(Box::new(Type::List(Box::new(Type::Int)))));

    // Dict of Lists
    let expr2 = Expr::parse("Dict[str, List[int]]", "<test>").unwrap();
    let ty2 = TypeExtractor::extract_type(&expr2).unwrap();
    assert_eq!(
        ty2,
        Type::Dict(
            Box::new(Type::String),
            Box::new(Type::List(Box::new(Type::Int)))
        )
    );

    // Optional Dict
    let expr3 = Expr::parse("Optional[Dict[str, int]]", "<test>").unwrap();
    let ty3 = TypeExtractor::extract_type(&expr3).unwrap();
    assert_eq!(
        ty3,
        Type::Optional(Box::new(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )))
    );
}

#[test]
fn test_extract_generic_custom_type() {
    // Custom generic with one parameter
    let expr = Expr::parse("DataFrame[int]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(
        ty,
        Type::Generic {
            base: "DataFrame".to_string(),
            params: vec![Type::Int],
        }
    );

    // Custom generic with multiple parameters
    let expr2 = Expr::parse("Result[int, str]", "<test>").unwrap();
    let ty2 = TypeExtractor::extract_type(&expr2).unwrap();
    assert_eq!(
        ty2,
        Type::Generic {
            base: "Result".to_string(),
            params: vec![Type::Int, Type::String],
        }
    );
}

#[test]
fn test_extract_generic_type_var() {
    let expr = Expr::parse("Generic[T]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(ty, Type::TypeVar("T".to_string()));
}

#[test]
fn test_extract_return_type_with_annotation() {
    let expr = Expr::parse("int", "<test>").unwrap();
    let boxed_expr = Box::new(expr);
    let result = TypeExtractor::extract_return_type(&Some(boxed_expr)).unwrap();
    assert_eq!(result, Type::Int);
}

#[test]
fn test_error_on_unsupported_type() {
    // Test complex expressions that should fail
    let expr = Expr::parse("1 + 2", "<test>").unwrap();
    let result = TypeExtractor::extract_type(&expr);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Unsupported type annotation"));
}

#[test]
fn test_error_on_invalid_dict_params() {
    // Dict with wrong number of parameters
    let expr = Expr::parse("Dict[int]", "<test>").unwrap();
    let result = TypeExtractor::extract_type(&expr);
    assert!(result.is_err());
    // The error message may vary, so just check it failed
}

#[test]
fn test_complex_nested_generics() {
    // List[Optional[Dict[str, Union[int, float]]]]
    let expr = Expr::parse("List[Optional[Dict[str, Union[int, float]]]]", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();

    let expected = Type::List(Box::new(Type::Optional(Box::new(Type::Dict(
        Box::new(Type::String),
        Box::new(Type::Union(vec![Type::Int, Type::Float])),
    )))));

    assert_eq!(ty, expected);
}

#[test]
fn test_extract_type_with_name_expr() {
    let expr = Expr::parse("int", "<test>").unwrap();
    let ty = TypeExtractor::extract_type(&expr).unwrap();
    assert_eq!(ty, Type::Int);
}
