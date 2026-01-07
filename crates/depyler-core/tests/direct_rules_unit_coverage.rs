//! Unit tests for direct_rules.rs to increase coverage
//! DEPYLER-COVERAGE-95: Direct tests for functions used by rust_gen

use depyler_core::direct_rules::{
    convert_class_to_struct, is_stdlib_shadowing_name, method_mutates_self, rust_type_to_syn_type,
    safe_class_name,
};
use depyler_core::hir::{
    AssignTarget, HirClass, HirExpr, HirField, HirMethod, HirParam, HirStmt, Literal, Type,
};
use depyler_core::type_mapper::{PrimitiveType, RustType, TypeMapper};
use smallvec::smallvec;

// ============ is_stdlib_shadowing_name tests ============

#[test]
fn test_stdlib_shadowing_primitive_types() {
    assert!(is_stdlib_shadowing_name("bool"));
    assert!(is_stdlib_shadowing_name("char"));
    assert!(is_stdlib_shadowing_name("str"));
    assert!(is_stdlib_shadowing_name("i32"));
    assert!(is_stdlib_shadowing_name("i64"));
    assert!(is_stdlib_shadowing_name("u32"));
    assert!(is_stdlib_shadowing_name("u64"));
    assert!(is_stdlib_shadowing_name("f32"));
    assert!(is_stdlib_shadowing_name("f64"));
    assert!(is_stdlib_shadowing_name("isize"));
    assert!(is_stdlib_shadowing_name("usize"));
}

#[test]
fn test_stdlib_shadowing_prelude_types() {
    assert!(is_stdlib_shadowing_name("Box"));
    assert!(is_stdlib_shadowing_name("Vec"));
    assert!(is_stdlib_shadowing_name("String"));
    assert!(is_stdlib_shadowing_name("Option"));
    assert!(is_stdlib_shadowing_name("Result"));
    assert!(is_stdlib_shadowing_name("Some"));
    assert!(is_stdlib_shadowing_name("None"));
    assert!(is_stdlib_shadowing_name("Ok"));
    assert!(is_stdlib_shadowing_name("Err"));
}

#[test]
fn test_stdlib_shadowing_collections() {
    assert!(is_stdlib_shadowing_name("HashMap"));
    assert!(is_stdlib_shadowing_name("HashSet"));
    assert!(is_stdlib_shadowing_name("BTreeMap"));
    assert!(is_stdlib_shadowing_name("BTreeSet"));
    assert!(is_stdlib_shadowing_name("VecDeque"));
    assert!(is_stdlib_shadowing_name("LinkedList"));
}

#[test]
fn test_stdlib_shadowing_smart_pointers() {
    assert!(is_stdlib_shadowing_name("Rc"));
    assert!(is_stdlib_shadowing_name("Arc"));
    assert!(is_stdlib_shadowing_name("RefCell"));
    assert!(is_stdlib_shadowing_name("Cell"));
    assert!(is_stdlib_shadowing_name("Mutex"));
    assert!(is_stdlib_shadowing_name("RwLock"));
}

#[test]
fn test_stdlib_shadowing_traits() {
    assert!(is_stdlib_shadowing_name("Iterator"));
    assert!(is_stdlib_shadowing_name("Clone"));
    assert!(is_stdlib_shadowing_name("Copy"));
    assert!(is_stdlib_shadowing_name("Debug"));
    assert!(is_stdlib_shadowing_name("Default"));
    assert!(is_stdlib_shadowing_name("Display"));
    assert!(is_stdlib_shadowing_name("Eq"));
    assert!(is_stdlib_shadowing_name("Hash"));
}

#[test]
fn test_stdlib_shadowing_io_types() {
    assert!(is_stdlib_shadowing_name("Read"));
    assert!(is_stdlib_shadowing_name("Write"));
    assert!(is_stdlib_shadowing_name("Seek"));
    assert!(is_stdlib_shadowing_name("BufRead"));
}

#[test]
fn test_stdlib_shadowing_path_types() {
    assert!(is_stdlib_shadowing_name("Path"));
    assert!(is_stdlib_shadowing_name("PathBuf"));
    assert!(is_stdlib_shadowing_name("OsStr"));
    assert!(is_stdlib_shadowing_name("OsString"));
}

#[test]
fn test_stdlib_shadowing_time_types() {
    assert!(is_stdlib_shadowing_name("Duration"));
    assert!(is_stdlib_shadowing_name("Instant"));
    assert!(is_stdlib_shadowing_name("SystemTime"));
}

#[test]
fn test_stdlib_shadowing_other() {
    assert!(is_stdlib_shadowing_name("Error"));
    assert!(is_stdlib_shadowing_name("Range"));
    assert!(is_stdlib_shadowing_name("Cow"));
}

#[test]
fn test_stdlib_shadowing_non_shadowing() {
    assert!(!is_stdlib_shadowing_name("MyClass"));
    assert!(!is_stdlib_shadowing_name("Point"));
    assert!(!is_stdlib_shadowing_name("User"));
    assert!(!is_stdlib_shadowing_name("Config"));
    assert!(!is_stdlib_shadowing_name("Service"));
    assert!(!is_stdlib_shadowing_name("Handler"));
}

// ============ safe_class_name tests ============

#[test]
fn test_safe_class_name_shadowing() {
    assert_eq!(safe_class_name("Vec"), "PyVec");
    assert_eq!(safe_class_name("String"), "PyString");
    assert_eq!(safe_class_name("Option"), "PyOption");
    assert_eq!(safe_class_name("Result"), "PyResult");
    assert_eq!(safe_class_name("HashMap"), "PyHashMap");
    assert_eq!(safe_class_name("Box"), "PyBox");
}

#[test]
fn test_safe_class_name_non_shadowing() {
    assert_eq!(safe_class_name("MyClass"), "MyClass");
    assert_eq!(safe_class_name("Point"), "Point");
    assert_eq!(safe_class_name("User"), "User");
    assert_eq!(safe_class_name("Config"), "Config");
}

// ============ rust_type_to_syn_type tests ============

#[test]
fn test_rust_type_to_syn_primitive_types() {
    let ty = rust_type_to_syn_type(&RustType::Primitive(PrimitiveType::I32)).unwrap();
    assert_eq!(quote::quote!(#ty).to_string(), "i32");

    let ty = rust_type_to_syn_type(&RustType::Primitive(PrimitiveType::I64)).unwrap();
    assert_eq!(quote::quote!(#ty).to_string(), "i64");

    let ty = rust_type_to_syn_type(&RustType::Primitive(PrimitiveType::F64)).unwrap();
    assert_eq!(quote::quote!(#ty).to_string(), "f64");

    let ty = rust_type_to_syn_type(&RustType::Primitive(PrimitiveType::Bool)).unwrap();
    assert_eq!(quote::quote!(#ty).to_string(), "bool");

    let ty = rust_type_to_syn_type(&RustType::String).unwrap();
    assert_eq!(quote::quote!(#ty).to_string(), "String");
}

#[test]
fn test_rust_type_to_syn_unit() {
    let ty = rust_type_to_syn_type(&RustType::Unit).unwrap();
    assert_eq!(quote::quote!(#ty).to_string(), "()");
}

#[test]
fn test_rust_type_to_syn_vec() {
    let ty = rust_type_to_syn_type(&RustType::Vec(Box::new(RustType::Primitive(
        PrimitiveType::I32,
    ))))
    .unwrap();
    assert!(quote::quote!(#ty).to_string().contains("Vec"));
}

#[test]
fn test_rust_type_to_syn_hashmap() {
    let ty = rust_type_to_syn_type(&RustType::HashMap(
        Box::new(RustType::String),
        Box::new(RustType::Primitive(PrimitiveType::I32)),
    ))
    .unwrap();
    let s = quote::quote!(#ty).to_string();
    assert!(s.contains("HashMap"));
}

#[test]
fn test_rust_type_to_syn_option() {
    let ty = rust_type_to_syn_type(&RustType::Option(Box::new(RustType::Primitive(
        PrimitiveType::I32,
    ))))
    .unwrap();
    let s = quote::quote!(#ty).to_string();
    assert!(s.contains("Option"));
}

#[test]
fn test_rust_type_to_syn_tuple() {
    let ty = rust_type_to_syn_type(&RustType::Tuple(vec![
        RustType::Primitive(PrimitiveType::I32),
        RustType::String,
    ]))
    .unwrap();
    let s = quote::quote!(#ty).to_string();
    assert!(s.contains("i32"));
    assert!(s.contains("String"));
}

#[test]
fn test_rust_type_to_syn_result() {
    let ty = rust_type_to_syn_type(&RustType::Result(
        Box::new(RustType::Primitive(PrimitiveType::I32)),
        Box::new(RustType::String),
    ))
    .unwrap();
    let s = quote::quote!(#ty).to_string();
    assert!(s.contains("Result"));
}

// ============ method_mutates_self tests ============

fn make_self_var() -> HirExpr {
    HirExpr::Var("self".to_string())
}

fn make_attr_assign(attr_name: &str) -> HirStmt {
    HirStmt::Assign {
        target: AssignTarget::Attribute {
            value: Box::new(make_self_var()),
            attr: attr_name.to_string(),
        },
        value: HirExpr::Literal(Literal::Int(42)),
        type_annotation: None,
    }
}

fn make_var_assign(var_name: &str) -> HirStmt {
    HirStmt::Assign {
        target: AssignTarget::Symbol(var_name.to_string()),
        value: HirExpr::Literal(Literal::Int(42)),
        type_annotation: None,
    }
}

fn make_method(body: Vec<HirStmt>) -> HirMethod {
    HirMethod {
        name: "test_method".to_string(),
        params: smallvec![HirParam {
            name: "self".to_string(),
            ty: Type::Unknown,
            default: None,
            is_vararg: false,
        }],
        ret_type: Type::None,
        body,
        is_static: false,
        is_classmethod: false,
        is_property: false,
        is_async: false,
        docstring: None,
    }
}

#[test]
fn test_method_mutates_self_simple_assignment() {
    let method = make_method(vec![make_attr_assign("value")]);
    assert!(method_mutates_self(&method));
}

#[test]
fn test_method_mutates_self_no_mutation() {
    let method = make_method(vec![make_var_assign("x")]);
    assert!(!method_mutates_self(&method));
}

#[test]
fn test_method_mutates_self_empty_body() {
    let method = make_method(vec![]);
    assert!(!method_mutates_self(&method));
}

#[test]
fn test_method_mutates_self_in_if_then() {
    let method = make_method(vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![make_attr_assign("field")],
        else_body: None,
    }]);
    assert!(method_mutates_self(&method));
}

#[test]
fn test_method_mutates_self_in_if_else() {
    let method = make_method(vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![make_var_assign("x")],
        else_body: Some(vec![make_attr_assign("field")]),
    }]);
    assert!(method_mutates_self(&method));
}

#[test]
fn test_method_mutates_self_in_while() {
    let method = make_method(vec![HirStmt::While {
        condition: HirExpr::Literal(Literal::Bool(true)),
        body: vec![make_attr_assign("counter")],
    }]);
    assert!(method_mutates_self(&method));
}

#[test]
fn test_method_mutates_self_in_for() {
    let method = make_method(vec![HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: HirExpr::Literal(Literal::Int(0)),
        body: vec![make_attr_assign("total")],
    }]);
    assert!(method_mutates_self(&method));
}

#[test]
fn test_method_mutates_self_return_only() {
    let method = make_method(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
        42,
    ))))]);
    assert!(!method_mutates_self(&method));
}

#[test]
fn test_method_mutates_self_pass_only() {
    let method = make_method(vec![HirStmt::Pass]);
    assert!(!method_mutates_self(&method));
}

// ============ convert_class_to_struct tests ============

fn make_simple_class(name: &str) -> HirClass {
    HirClass {
        name: name.to_string(),
        base_classes: vec![],
        fields: vec![],
        methods: vec![],
        is_dataclass: false,
        docstring: None,
        type_params: vec![],
    }
}

fn make_class_with_fields(name: &str, fields: Vec<(&str, Type)>) -> HirClass {
    HirClass {
        name: name.to_string(),
        base_classes: vec![],
        fields: fields
            .into_iter()
            .map(|(n, t)| HirField {
                name: n.to_string(),
                field_type: t,
                default_value: None,
                is_class_var: false,
            })
            .collect(),
        methods: vec![],
        is_dataclass: false,
        docstring: None,
        type_params: vec![],
    }
}

#[test]
fn test_convert_class_to_struct_simple() {
    let class = make_simple_class("Point");
    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    assert!(!items.is_empty());
}

#[test]
fn test_convert_class_to_struct_with_fields() {
    let class = make_class_with_fields("Point", vec![("x", Type::Int), ("y", Type::Int)]);
    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    assert!(!items.is_empty());
}

#[test]
fn test_convert_class_to_struct_shadowing_name() {
    let class = make_simple_class("Vec");
    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    let code = items
        .iter()
        .map(|i| quote::quote!(#i).to_string())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(code.contains("PyVec"));
}

#[test]
fn test_convert_class_to_struct_with_method() {
    let mut class = make_simple_class("Counter");
    class.methods.push(HirMethod {
        name: "increment".to_string(),
        params: smallvec![HirParam {
            name: "self".to_string(),
            ty: Type::Unknown,
            default: None,
            is_vararg: false,
        }],
        ret_type: Type::None,
        body: vec![make_attr_assign("count")],
        is_static: false,
        is_classmethod: false,
        is_property: false,
        is_async: false,
        docstring: None,
    });

    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    assert!(items.len() >= 2);
}

#[test]
fn test_convert_class_to_struct_dataclass() {
    let mut class =
        make_class_with_fields("Config", vec![("name", Type::String), ("value", Type::Int)]);
    class.is_dataclass = true;

    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    assert!(!items.is_empty());
}

// ============ Additional edge case tests ============

#[test]
fn test_safe_class_name_all_integer_types() {
    for ty in &[
        "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128",
    ] {
        assert!(is_stdlib_shadowing_name(ty), "{} should be shadowing", ty);
        assert!(
            safe_class_name(ty).starts_with("Py"),
            "{} should get Py prefix",
            ty
        );
    }
}

#[test]
fn test_rust_type_to_syn_hashset() {
    let ty = rust_type_to_syn_type(&RustType::HashSet(Box::new(RustType::String))).unwrap();
    let s = quote::quote!(#ty).to_string();
    assert!(s.contains("HashSet"));
}

#[test]
fn test_rust_type_to_syn_custom() {
    let ty = rust_type_to_syn_type(&RustType::Custom("MyType".to_string())).unwrap();
    let s = quote::quote!(#ty).to_string();
    assert!(s.contains("MyType"));
}

// ============ More coverage for edge cases ============

#[test]
fn test_convert_class_with_float_field() {
    let class = make_class_with_fields("Measurement", vec![("value", Type::Float)]);
    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    assert!(!items.is_empty());
}

#[test]
fn test_convert_class_with_bool_field() {
    let class = make_class_with_fields("Flag", vec![("enabled", Type::Bool)]);
    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    assert!(!items.is_empty());
}

#[test]
fn test_convert_class_with_list_field() {
    let class = make_class_with_fields(
        "Container",
        vec![("items", Type::List(Box::new(Type::Int)))],
    );
    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    assert!(!items.is_empty());
}

#[test]
fn test_convert_class_with_dict_field() {
    let class = make_class_with_fields(
        "Cache",
        vec![(
            "data",
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
        )],
    );
    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    assert!(!items.is_empty());
}

#[test]
fn test_convert_class_with_optional_field() {
    let class = make_class_with_fields(
        "MaybeValue",
        vec![("value", Type::Optional(Box::new(Type::Int)))],
    );
    let type_mapper = TypeMapper::new();
    let vararg_funcs = std::collections::HashSet::new();
    let items = convert_class_to_struct(&class, &type_mapper, &vararg_funcs).unwrap();
    assert!(!items.is_empty());
}

#[test]
fn test_method_mutates_self_nested_if() {
    let method = make_method(vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![make_attr_assign("nested")],
            else_body: None,
        }],
        else_body: None,
    }]);
    assert!(method_mutates_self(&method));
}

#[test]
fn test_method_mutates_self_nested_loops() {
    let method = make_method(vec![HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: HirExpr::Literal(Literal::Int(0)),
        body: vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![make_attr_assign("deeply_nested")],
        }],
    }]);
    assert!(method_mutates_self(&method));
}
