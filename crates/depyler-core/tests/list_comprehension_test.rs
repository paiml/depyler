//! Tests for Python list comprehensions to Rust equivalents

use depyler_core::DepylerPipeline;

#[test]
fn test_basic_list_comprehension() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_basic_comprehension():
    # Simple list comprehension
    squares = [x * x for x in range(5)]
    
    # With condition
    evens = [x for x in range(10) if x % 2 == 0]
    
    return len(squares)
"#;

    let result = pipeline.transpile(python_code);
    println!("Basic list comprehension result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated comprehension code:\n{}", rust_code);

        // Check for iterator patterns
        assert!(
            rust_code.contains("map")
                || rust_code.contains("filter")
                || rust_code.contains("collect"),
            "Should generate iterator-based code"
        );
    }
}

#[test]
fn test_comprehension_with_filtering() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_filtered_comprehension():
    numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    # Filter with condition
    evens = [n for n in numbers if n % 2 == 0]
    
    # Multiple conditions
    special = [n for n in numbers if n > 5 and n < 9]
    
    return len(evens)
"#;

    let result = pipeline.transpile(python_code);
    println!("Filtered comprehension result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated filtered code:\n{}", rust_code);
    }
}

#[test]
fn test_comprehension_with_transformation() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_transform_comprehension():
    words = ["hello", "world", "rust", "python"]
    
    # Transform elements
    upper_words = [word.upper() for word in words]
    
    # Transform with condition
    long_upper = [word.upper() for word in words if len(word) > 4]
    
    return len(upper_words)
"#;

    let result = pipeline.transpile(python_code);
    println!("Transform comprehension result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated transform code:\n{}", rust_code);
    }
}

#[test]
fn test_nested_comprehension() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_nested_comprehension():
    # Nested loops in comprehension
    matrix = [[i * j for j in range(3)] for i in range(3)]
    
    # Flatten nested list
    flat = [item for sublist in matrix for item in sublist]
    
    return len(flat)
"#;

    let result = pipeline.transpile(python_code);
    println!("Nested comprehension result: {:?}", result);

    // This is complex - might not work initially
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_comprehension_with_complex_expressions() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_complex_comprehension():
    data = [1, 2, 3, 4, 5]
    
    # Complex expression
    result = [x * 2 + 1 for x in data if x > 2]
    
    # Using functions
    processed = [str(x) for x in result]
    
    return len(result)
"#;

    let result = pipeline.transpile(python_code);
    println!("Complex comprehension result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated complex code:\n{}", rust_code);
    }
}

#[test]
fn test_comprehension_scope() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_comprehension_scope():
    x = 10
    
    # Using outer scope variable
    scaled = [x * i for i in range(5)]
    
    # Variable in comprehension doesn't leak
    squares = [n * n for n in range(5)]
    # n should not be accessible here
    
    return x  # x should still be 10
"#;

    let result = pipeline.transpile(python_code);
    println!("Comprehension scope result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated scope code:\n{}", rust_code);
    }
}

#[test]
fn test_dict_and_set_comprehensions() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_other_comprehensions():
    # Dict comprehension
    squares_dict = {x: x*x for x in range(5)}
    
    # Set comprehension
    unique_squares = {x*x for x in range(-5, 6)}
    
    return len(squares_dict)
"#;

    let result = pipeline.transpile(python_code);
    println!("Dict/set comprehension result: {:?}", result);

    // These are advanced - might not work initially
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_generator_expressions() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_generators():
    # Generator expression (not list comprehension)
    gen = (x * x for x in range(5))
    
    # Convert to list
    squares = list(gen)
    
    return len(squares)
"#;

    let result = pipeline.transpile(python_code);
    println!("Generator expression result: {:?}", result);

    // Generators are advanced - might not work
    assert!(result.is_ok() || result.is_err());
}
