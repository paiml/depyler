//! Simple test for method calls

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_method_call() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def simple_append():
    numbers = [1, 2, 3]
    numbers.append(4)
    return numbers
"#;

    let result = pipeline.transpile(python_code);
    println!("Simple method call result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated code:\n{}", rust_code);
        assert!(rust_code.contains("push"), "Should use push for append");
    } else {
        // Print the error for debugging
        println!("Error: {:?}", result);
    }
}
