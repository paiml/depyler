use depyler_core::{hir::Type, DepylerPipeline};

#[test]
fn test_simple_generic_function() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def identity(x: T) -> T:
    return x
"#;

    let result = pipeline.transpile(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should generate a generic function
    assert!(rust_code.contains("pub fn identity<T: Clone>(x: T)"));
    assert!(rust_code.contains("-> T"));
    assert!(rust_code.contains("return x"));
}

#[test]
fn test_generic_list_function() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import List

def first_element(items: List[T]) -> T:
    return items[0]
"#;

    let result = pipeline.transpile(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should generate a generic function with Vec<T>
    assert!(rust_code.contains("T: Clone") || rust_code.contains("T:Clone"));
    assert!(rust_code.contains("Vec<T>"));
    assert!(rust_code.contains("-> T") || rust_code.contains("Result<T"));
}

#[test]
fn test_multiple_type_parameters() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Tuple

def pair(a: T, b: U) -> Tuple[T, U]:
    return (a, b)
"#;

    let result = pipeline.transpile(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should generate function with two type parameters
    // Check that both T and U are declared with Clone bounds
    assert!(rust_code.contains("T: Clone") || rust_code.contains("T:Clone"));
    assert!(rust_code.contains("U: Clone") || rust_code.contains("U:Clone"));
    // Check parameter types
    assert!(rust_code.contains("a:") && rust_code.contains("T"));
    assert!(rust_code.contains("b:") && rust_code.contains("U"));
    // Return type might be Tuple<T, U> instead of (T, U)
    assert!(rust_code.contains("Tuple<T, U>") || rust_code.contains("(T, U)"));
}

#[test]
fn test_generic_with_constraints() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def compare(a: T, b: T) -> bool:
    return a < b
"#;

    let result = pipeline.transpile(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should infer PartialOrd constraint from < operator
    assert!(
        rust_code.contains("T: Clone + PartialOrd") || rust_code.contains("T: PartialOrd + Clone")
    );
}

#[test]
fn test_union_type() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Union

def process_value(x: Union[int, str]) -> str:
    if isinstance(x, int):
        return str(x)
    else:
        return x
"#;

    let hir = pipeline.parse_to_hir(python_code).unwrap();
    assert_eq!(hir.functions.len(), 1);

    let func = &hir.functions[0];
    match &func.params[0].1 {
        Type::Union(types) => {
            assert_eq!(types.len(), 2);
            assert!(types.contains(&Type::Int));
            assert!(types.contains(&Type::String));
        }
        _ => panic!("Expected Union type"),
    }
}

#[test]
fn test_generic_dict() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Dict

def get_value(mapping: Dict[K, V], key: K) -> V:
    return mapping[key]
"#;

    let result = pipeline.transpile(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should generate function with K and V type parameters
    assert!(rust_code.contains("K: Clone") || rust_code.contains("K:"));
    assert!(rust_code.contains("V: Clone") || rust_code.contains("V:"));
    assert!(rust_code.contains("HashMap<K, V>"));
}

#[test]
fn test_type_var_in_optional() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Optional

def maybe_value(x: Optional[T]) -> T:
    return x
"#;

    let result = pipeline.transpile(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should handle Optional with type parameter
    assert!(rust_code.contains("T: Clone") || rust_code.contains("T:Clone"));
    assert!(rust_code.contains("x: Option<T>"));
}

// Test commented out - class instantiation not yet supported
// #[test]
// fn test_generic_class_instantiation() {
//     let pipeline = DepylerPipeline::new();
//     let python_code = r#"
// from typing import Generic
//
// def create_container() -> Container[int]:
//     return Container[int]()
// "#;
//
//     let hir = pipeline.parse_to_hir(python_code).unwrap();
//     assert_eq!(hir.functions.len(), 1);
//
//     let func = &hir.functions[0];
//     match &func.ret_type {
//         Type::Generic { base, params } => {
//             assert_eq!(base, "Container");
//             assert_eq!(params.len(), 1);
//             assert_eq!(params[0], Type::Int);
//         }
//         _ => panic!("Expected Generic type"),
//     }
// }
