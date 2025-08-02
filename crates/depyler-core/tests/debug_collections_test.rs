//! Debug specific collection features

use depyler_core::DepylerPipeline;

#[test]
fn test_list_comprehension() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_comprehension():
    # This might be causing the issue
    squares = [x * x for x in range(5)]
    return squares
"#;

    let result = pipeline.transpile(python_code);
    println!("List comprehension result: {:?}", result);
}

#[test]
fn test_extend_method() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_extend():
    numbers = [1, 2, 3]
    numbers.extend([7, 8])
    return numbers
"#;

    let result = pipeline.transpile(python_code);
    println!("Extend method result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated extend code:\n{}", rust_code);
        assert!(rust_code.contains("extend"), "Should use extend");
    }
}

#[test]
fn test_negative_indexing() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_negative():
    numbers = [1, 2, 3, 4, 5]
    # This might be causing the issue
    last = numbers[-1]
    return last
"#;

    let result = pipeline.transpile(python_code);
    println!("Negative indexing result: {:?}", result);
}
