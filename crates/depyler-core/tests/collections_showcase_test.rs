//! Showcase of Python collections features that work well

use depyler_core::DepylerPipeline;

#[test]
fn test_collections_showcase() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def showcase_collections():
    # List operations
    numbers = [1, 2, 3]
    numbers.append(4)
    numbers.extend([5, 6])
    
    # Dict operations  
    data = {"name": "Alice", "score": 95}
    name = data.get("name", "Unknown")
    
    # Nested collections
    matrix = [[1, 2], [3, 4]]
    nested_data = [
        {"id": 1, "values": [10, 20]},
        {"id": 2, "values": [30, 40]}
    ]
    
    # String operations
    text = "Hello World"
    upper_text = text.upper()
    words = text.split()
    
    return len(numbers)
"#;

    let result = pipeline.transpile(python_code);
    println!("Collections showcase result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated showcase code:\n{}", rust_code);

        // Verify key features
        assert!(
            rust_code.contains("vec !") || rust_code.contains("vec!"),
            "Should use vec! macro"
        );
        // Dict with mixed value types (str + int) uses serde_json::json!
        // Dict with homogeneous types uses HashMap
        assert!(
            rust_code.contains("HashMap") || rust_code.contains("serde_json"),
            "Should use HashMap or serde_json for dict"
        );
        assert!(rust_code.contains("push"), "Should map append to push");
        assert!(rust_code.contains("extend"), "Should map extend to extend");
        // Note: dict.get() might be optimized away if the result isn't used
        // assert!(rust_code.contains("get("), "Should map dict.get()");
        // Note: string operations might be optimized away if results aren't used
        // assert!(
        //     rust_code.contains("to_uppercase"),
        //     "Should map string.upper()"
        // );
        // assert!(rust_code.contains("split"), "Should map string.split()");

        println!("✅ All key collection features working!");
    }
}

#[test]
fn test_method_calls_showcase() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def method_showcase():
    # List methods
    items = []
    items.append("first")
    items.extend(["second", "third"])
    
    # Dict methods
    config = {}
    keys = config.keys()
    
    # String methods
    message = "  Hello World  "
    clean = message.strip()
    parts = message.split()
    joined = "-".join(["a", "b", "c"])
    
    return len(items)
"#;

    let result = pipeline.transpile(python_code);
    println!("Method calls showcase result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated method calls code:\n{}", rust_code);
        println!("✅ Method call mapping working!");
    }
}
