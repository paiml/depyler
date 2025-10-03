// DEPYLER-0103: Property tests for type mapper
// Validates Python→Rust type mapping correctness

use depyler_core::hir::Type as PythonType;
use depyler_core::type_mapper::{PrimitiveType, RustType, StringStrategy, TypeMapper};
use quickcheck::{Arbitrary, Gen, TestResult};

// Property 1: Type mapping should be deterministic
#[test]
fn prop_type_mapping_is_deterministic() {
    fn prop(py_type: ArbitraryPythonType) -> bool {
        let mapper = TypeMapper::default();
        let result1 = mapper.map_type(&py_type.0);
        let result2 = mapper.map_type(&py_type.0);
        result1 == result2
    }
    quickcheck::quickcheck(prop as fn(ArbitraryPythonType) -> bool);
}

// Property 2: Primitive types should map to primitive Rust types
#[test]
fn prop_primitives_map_to_primitives() {
    let mapper = TypeMapper::default();

    assert!(matches!(
        mapper.map_type(&PythonType::Int),
        RustType::Primitive(_)
    ));
    assert!(matches!(
        mapper.map_type(&PythonType::Float),
        RustType::Primitive(_)
    ));
    assert!(matches!(
        mapper.map_type(&PythonType::Bool),
        RustType::Primitive(PrimitiveType::Bool)
    ));
}

// Property 3: List[T] should map to Vec<T>
#[test]
fn prop_list_maps_to_vec() {
    fn prop(inner: ArbitraryPythonType) -> bool {
        let mapper = TypeMapper::default();
        let py_list = PythonType::List(Box::new(inner.0.clone()));

        match mapper.map_type(&py_list) {
            RustType::Vec(inner_rust) => {
                // Inner type should be mapped version of inner Python type
                *inner_rust == mapper.map_type(&inner.0)
            }
            _ => false,
        }
    }
    quickcheck::quickcheck(prop as fn(ArbitraryPythonType) -> bool);
}

// Property 4: Dict[K, V] should map to HashMap<K, V>
#[test]
fn prop_dict_maps_to_hashmap() {
    fn prop(key: ArbitraryPythonType, value: ArbitraryPythonType) -> bool {
        let mapper = TypeMapper::default();
        let py_dict = PythonType::Dict(Box::new(key.0.clone()), Box::new(value.0.clone()));

        match mapper.map_type(&py_dict) {
            RustType::HashMap(k, v) => {
                *k == mapper.map_type(&key.0) && *v == mapper.map_type(&value.0)
            }
            _ => false,
        }
    }
    quickcheck::quickcheck(prop as fn(ArbitraryPythonType, ArbitraryPythonType) -> bool);
}

// Property 5: Optional[T] should map to Option<T>
#[test]
fn prop_optional_maps_to_option() {
    fn prop(inner: ArbitraryPythonType) -> bool {
        let mapper = TypeMapper::default();
        let py_optional = PythonType::Optional(Box::new(inner.0.clone()));

        match mapper.map_type(&py_optional) {
            RustType::Option(inner_rust) => *inner_rust == mapper.map_type(&inner.0),
            _ => false,
        }
    }
    quickcheck::quickcheck(prop as fn(ArbitraryPythonType) -> bool);
}

// Property 6: Union[T, None] should map to Option<T>
#[test]
fn test_union_with_none_maps_to_option() {
    let mapper = TypeMapper::default();
    let py_union = PythonType::Union(vec![PythonType::Int, PythonType::None]);

    match mapper.map_type(&py_union) {
        RustType::Option(inner) => {
            assert_eq!(*inner, RustType::Primitive(PrimitiveType::I32));
        }
        _ => panic!("Expected Option type"),
    }
}

// Property 7: Tuple types should preserve order and length
#[test]
fn prop_tuple_preserves_structure() {
    fn prop(types: Vec<ArbitraryPythonType>) -> TestResult {
        if types.is_empty() || types.len() > 10 {
            return TestResult::discard();
        }

        let mapper = TypeMapper::default();
        let py_types: Vec<_> = types.iter().map(|t| t.0.clone()).collect();
        let py_tuple = PythonType::Tuple(py_types.clone());

        match mapper.map_type(&py_tuple) {
            RustType::Tuple(rust_types) => TestResult::from_bool(
                rust_types.len() == py_types.len()
                    && rust_types
                        .iter()
                        .zip(py_types.iter())
                        .all(|(r, p)| r == &mapper.map_type(p)),
            ),
            _ => TestResult::failed(),
        }
    }
    quickcheck::quickcheck(prop as fn(Vec<ArbitraryPythonType>) -> TestResult);
}

// Property 8: Int width preference should be respected
#[test]
fn test_int_width_preference() {
    let mapper_i32 = TypeMapper::new();
    let mapper_i64 = TypeMapper::new().with_i64();

    assert_eq!(
        mapper_i32.map_type(&PythonType::Int),
        RustType::Primitive(PrimitiveType::I32)
    );
    assert_eq!(
        mapper_i64.map_type(&PythonType::Int),
        RustType::Primitive(PrimitiveType::I64)
    );
}

// Property 9: String strategy should be respected
#[test]
fn test_string_strategy() {
    let mapper_owned = TypeMapper::new().with_string_strategy(StringStrategy::AlwaysOwned);

    assert_eq!(mapper_owned.map_type(&PythonType::String), RustType::String);
}

// Property 10: Type parameters should be preserved
#[test]
fn test_type_param_preservation() {
    let mapper = TypeMapper::default();
    let py_typevar = PythonType::TypeVar("T".to_string());

    assert_eq!(
        mapper.map_type(&py_typevar),
        RustType::TypeParam("T".to_string())
    );
}

// Helper: Arbitrary Python types for property testing
#[derive(Debug, Clone)]
struct ArbitraryPythonType(PythonType);

impl Arbitrary for ArbitraryPythonType {
    fn arbitrary(g: &mut Gen) -> Self {
        let depth = g.size().min(3); // Limit recursion depth
        ArbitraryPythonType(gen_python_type(g, depth))
    }
}

fn gen_python_type(g: &mut Gen, depth: usize) -> PythonType {
    if depth == 0 {
        // Base cases only
        g.choose(&[
            PythonType::Int,
            PythonType::Float,
            PythonType::String,
            PythonType::Bool,
            PythonType::None,
        ])
        .unwrap()
        .clone()
    } else {
        // Include recursive cases
        let choice = u8::arbitrary(g) % 8;
        match choice {
            0 => PythonType::Int,
            1 => PythonType::Float,
            2 => PythonType::String,
            3 => PythonType::Bool,
            4 => PythonType::List(Box::new(gen_python_type(g, depth - 1))),
            5 => PythonType::Dict(
                Box::new(gen_python_type(g, depth - 1)),
                Box::new(gen_python_type(g, depth - 1)),
            ),
            6 => PythonType::Optional(Box::new(gen_python_type(g, depth - 1))),
            7 => {
                let len = (usize::arbitrary(g) % 3) + 1;
                let types = (0..len).map(|_| gen_python_type(g, depth - 1)).collect();
                PythonType::Tuple(types)
            }
            _ => PythonType::Int,
        }
    }
}

// Property 11: Nested collections should be handled correctly
#[test]
fn test_nested_collections() {
    let mapper = TypeMapper::default();

    // List[List[int]]
    let nested_list = PythonType::List(Box::new(PythonType::List(Box::new(PythonType::Int))));
    match mapper.map_type(&nested_list) {
        RustType::Vec(inner) => match *inner {
            RustType::Vec(inner2) => {
                assert_eq!(*inner2, RustType::Primitive(PrimitiveType::I32));
            }
            _ => panic!("Expected nested Vec"),
        },
        _ => panic!("Expected Vec"),
    }

    // Dict[str, List[int]]
    let dict_with_list = PythonType::Dict(
        Box::new(PythonType::String),
        Box::new(PythonType::List(Box::new(PythonType::Int))),
    );
    match mapper.map_type(&dict_with_list) {
        RustType::HashMap(k, v) => {
            assert_eq!(*k, RustType::String);
            match *v {
                RustType::Vec(inner) => {
                    assert_eq!(*inner, RustType::Primitive(PrimitiveType::I32));
                }
                _ => panic!("Expected Vec value"),
            }
        }
        _ => panic!("Expected HashMap"),
    }
}

// Property 12: Generic types should map correctly
#[test]
fn test_generic_type_mapping() {
    let mapper = TypeMapper::default();

    // List generic (no params provided)
    let py_custom_list = PythonType::Custom("List".to_string());
    match mapper.map_type(&py_custom_list) {
        RustType::Vec(_) => (),
        _ => panic!("Expected Vec for List"),
    }

    // Dict generic (no params provided)
    let py_custom_dict = PythonType::Custom("Dict".to_string());
    match mapper.map_type(&py_custom_dict) {
        RustType::HashMap(_, _) => (),
        _ => panic!("Expected HashMap for Dict"),
    }
}
