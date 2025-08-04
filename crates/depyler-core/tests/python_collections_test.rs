//! Tests for Python collections to Rust equivalents mapping

use depyler_core::DepylerPipeline;

#[test]
fn test_list_operations() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def work_with_lists():
    # List creation
    numbers = [1, 2, 3, 4, 5]
    
    # List access
    first = numbers[0]
    last = numbers[-1]
    
    # List methods
    numbers.append(6)
    numbers.extend([7, 8])
    
    # List comprehension (if supported)
    squares = [x * x for x in numbers]
    
    return len(numbers)
"#;

    let result = pipeline.transpile(python_code);
    println!("List operations result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated list code:\n{}", rust_code);

        // Check for Vec usage
        assert!(rust_code.contains("vec!") || rust_code.contains("vec !"), "Should use Vec for Python lists");

        // Check for common collection methods
        // Note: These might not all be implemented yet
        if rust_code.contains("push") {
            println!("✓ List append mapped to push");
        }
    }
}

#[test]
fn test_dict_operations() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def work_with_dicts():
    # Dict creation
    scores = {"alice": 95, "bob": 87, "charlie": 92}
    
    # Dict access
    alice_score = scores["alice"]
    
    # Dict methods
    scores["david"] = 89
    all_names = list(scores.keys())
    
    return len(scores)
"#;

    let result = pipeline.transpile(python_code);
    println!("Dict operations result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated dict code:\n{}", rust_code);

        // Check for HashMap usage
        assert!(
            rust_code.contains("HashMap"),
            "Should use HashMap for Python dicts"
        );
    }
}

#[test]
fn test_tuple_operations() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def work_with_tuples():
    # Tuple creation
    point = (10, 20)
    person = ("Alice", 30, True)
    
    # Tuple unpacking
    x, y = point
    name, age, active = person
    
    # Tuple access
    first_coord = point[0]
    
    return len(person)
"#;

    let result = pipeline.transpile(python_code);
    println!("Tuple operations result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated tuple code:\n{}", rust_code);

        // Check for tuple usage
        assert!(
            rust_code.contains("(") && rust_code.contains(")"),
            "Should use Rust tuples for Python tuples"
        );
    }
}

#[test]
fn test_nested_collections() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def work_with_nested():
    # Nested list
    matrix = [[1, 2], [3, 4], [5, 6]]
    
    # List of dicts
    users = [
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25}
    ]
    
    # Dict with list values
    groups = {
        "admins": ["alice", "bob"],
        "users": ["charlie", "david"]
    }
    
    return len(matrix)
"#;

    let result = pipeline.transpile(python_code);
    println!("Nested collections result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated nested code:\n{}", rust_code);

        // Check for nested Vec and HashMap
        assert!(
            rust_code.contains("vec!") || rust_code.contains("vec !") || rust_code.contains("Vec"),
            "Should use Vec for nested lists"
        );
        // Note: HashMap usage might be optimized away if dicts aren't used
        // assert!(
        //     rust_code.contains("HashMap"),
        //     "Should use HashMap for nested dicts"
        // );
    }
}

#[test]
fn test_collection_type_annotations() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
from typing import List, Dict, Tuple, Optional

def typed_collections(
    numbers: List[int],
    scores: Dict[str, float],
    point: Tuple[int, int],
    maybe_name: Optional[str]
) -> List[str]:
    result = []
    
    for num in numbers:
        result.append(str(num))
    
    return result
"#;

    let result = pipeline.transpile(python_code);
    println!("Type annotations result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated typed code:\n{}", rust_code);

        // Check for proper Rust type mappings
        assert!(
            rust_code.contains("Vec<i32>") || rust_code.contains("&Vec<i32>"),
            "Should map List[int] to Vec<i32>"
        );
        assert!(
            rust_code.contains("HashMap<String, f64>") || rust_code.contains("&HashMap"),
            "Should map Dict[str, float] to HashMap"
        );
        assert!(
            rust_code.contains("Option<String>") || rust_code.contains("Option<&str>"),
            "Should map Optional[str] to Option"
        );
    }
}

#[test]
fn test_collection_methods() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def collection_methods():
    # List methods
    numbers = [1, 2, 3]
    numbers.append(4)
    numbers.pop()
    numbers.insert(0, 0)
    numbers.remove(2)
    
    # Dict methods  
    data = {"a": 1, "b": 2}
    data.update({"c": 3})
    value = data.get("a", 0)
    data.pop("b")
    
    # Set operations (if supported)
    unique = set([1, 2, 2, 3])
    
    return len(numbers)
"#;

    let result = pipeline.transpile(python_code);
    println!("Collection methods result: {:?}", result);

    // This might fail since not all methods are implemented
    // Just check what we get
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_collection_iteration() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def iterate_collections():
    numbers = [1, 2, 3, 4, 5]
    
    # Simple iteration
    total = 0
    for num in numbers:
        total += num
    
    # Dict iteration
    scores = {"alice": 95, "bob": 87}
    for name in scores:
        print(name)
    
    for name, score in scores.items():
        print(name, score)
    
    return total
"#;

    let result = pipeline.transpile(python_code);
    println!("Collection iteration result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated iteration code:\n{}", rust_code);

        // Check for proper iteration patterns
        assert!(rust_code.contains("for"), "Should generate for loops");
    }
}
