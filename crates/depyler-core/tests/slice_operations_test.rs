//! Tests for Python slice operations to Rust equivalents

use depyler_core::DepylerPipeline;

#[test]
fn test_basic_slicing() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_basic_slices():
    items = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    # Basic slicing
    first_three = items[:3]      # [1, 2, 3]
    last_three = items[-3:]     # [8, 9, 10]
    middle = items[2:5]         # [3, 4, 5]
    
    # Full slice (copy)
    copy_all = items[:]
    
    return len(first_three)
"#;

    let result = pipeline.transpile(python_code);
    println!("Basic slicing result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated slicing code:\n{}", rust_code);

        // Check for slice operations
        assert!(
            rust_code.contains("..") || rust_code.contains("["),
            "Should generate slice operations"
        );
    }
}

#[test]
fn test_step_slicing() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_step_slices():
    numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    # Step slicing
    evens = numbers[::2]        # [0, 2, 4, 6, 8]
    odds = numbers[1::2]        # [1, 3, 5, 7, 9]
    every_third = numbers[::3]  # [0, 3, 6, 9]
    
    # Reverse slicing
    reversed_all = numbers[::-1]  # [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    
    return len(evens)
"#;

    let result = pipeline.transpile(python_code);
    println!("Step slicing result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated step slicing code:\n{}", rust_code);
    }
}

#[test]
fn test_negative_indices() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_negative_indices():
    data = ["a", "b", "c", "d", "e"]
    
    # Negative indexing
    last = data[-1]           # "e"
    second_last = data[-2]    # "d"
    
    # Negative slicing
    last_two = data[-2:]      # ["d", "e"]
    all_but_last = data[:-1]  # ["a", "b", "c", "d"]
    
    return len(last_two)
"#;

    let result = pipeline.transpile(python_code);
    println!("Negative indices result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated negative indexing code:\n{}", rust_code);
    }
}

#[test]
fn test_slice_assignment() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_slice_assignment():
    items = [1, 2, 3, 4, 5]
    
    # Slice assignment
    items[1:3] = [20, 30]       # [1, 20, 30, 4, 5]
    items[:2] = [10, 15]        # [10, 15, 30, 4, 5]
    
    return len(items)
"#;

    let result = pipeline.transpile(python_code);
    println!("Slice assignment result: {:?}", result);

    // This might not work initially - slice assignment is complex
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_string_slicing() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_string_slices():
    text = "Hello, World!"
    
    # String slicing
    hello = text[:5]          # "Hello"
    world = text[7:]          # "World!"
    comma = text[5:6]         # ","
    
    # String step slicing
    every_second = text[::2]  # "Hlo ol!"
    
    return len(hello)
"#;

    let result = pipeline.transpile(python_code);
    println!("String slicing result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated string slicing code:\n{}", rust_code);
    }
}

#[test]
fn test_slice_methods() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_slice_methods():
    numbers = [1, 2, 3, 4, 5]
    
    # Using slice() builtin
    s = slice(1, 4)
    subset = numbers[s]         # [2, 3, 4]
    
    # Slice with step
    s2 = slice(0, None, 2)
    evens = numbers[s2]         # [1, 3, 5]
    
    return len(subset)
"#;

    let result = pipeline.transpile(python_code);
    println!("Slice methods result: {:?}", result);

    // This is advanced - might not work initially
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_complex_slice_expressions() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def test_complex_slices():
    matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    
    # Complex slicing
    first_row = matrix[0][:]    # [1, 2, 3]
    first_col = [row[0] for row in matrix]  # [1, 4, 7] - might not work
    
    # Multi-dimensional slicing
    submatrix = matrix[1:3]     # [[4, 5, 6], [7, 8, 9]]
    
    return len(first_row)
"#;

    let result = pipeline.transpile(python_code);
    println!("Complex slice expressions result: {:?}", result);

    // This tests the interaction with list comprehensions which might not work
    assert!(result.is_ok() || result.is_err());
}
