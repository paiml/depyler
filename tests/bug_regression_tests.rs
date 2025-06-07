use depyler_core::DepylerPipeline;

#[test]
fn test_variable_shadowing_in_loops_bug() {
    // Bug: Variable assignments inside loops create new variables instead of updating existing ones
    let python_code = r#"
def factorial(n: int) -> int:
    result = 1
    for i in range(1, n + 1):
        result = result * i
    return result
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should NOT contain "let mut result =" inside the loop
    assert!(!rust_code.contains("for i in 1"), "Loop should use proper range syntax");
    
    // Should contain proper assignment without let keyword
    let lines: Vec<&str> = rust_code.lines().collect();
    let mut found_assignment = false;
    let mut found_shadowing = false;
    
    for line in lines {
        if line.trim().contains("result = (result * i)") && !line.trim().starts_with("let mut") {
            found_assignment = true;
        }
        if line.trim().starts_with("let mut result = (result * i)") {
            found_shadowing = true;
        }
    }
    
    assert!(found_assignment, "Should have proper assignment: result = result * i");
    assert!(!found_shadowing, "Should not shadow variable with let mut inside loop");
}

#[test]
fn test_binary_search_variable_shadowing_bug() {
    // Bug: Variable assignments in conditionals create new variables
    let python_code = r#"
def binary_search(arr: list, target: int) -> int:
    left = 0
    right = len(arr) - 1
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should NOT contain "let mut left =" or "let mut right =" inside conditionals
    let lines: Vec<&str> = rust_code.lines().collect();
    let mut inside_conditional = false;
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.contains("if") || trimmed.contains("else") {
            inside_conditional = true;
        }
        if inside_conditional && (trimmed.starts_with("let mut left =") || trimmed.starts_with("let mut right =")) {
            panic!("Found variable shadowing inside conditional: {}", trimmed);
        }
        if trimmed == "}" {
            inside_conditional = false;
        }
    }
}

#[test]
fn test_docstring_as_expression_bug() {
    // Bug: Docstrings are treated as expressions that return strings
    let python_code = r#"
def add(a: int, b: int) -> int:
    "Add two numbers"
    return a + b
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should NOT contain docstring as an expression like `"Add two numbers" . to_string ();`
    assert!(!rust_code.contains(`"Add two numbers" . to_string ()`), 
           "Docstring should not be generated as a string expression");
    
    // Should contain proper Rust doc comment
    assert!(rust_code.contains("/// Add two numbers") || rust_code.contains("#[doc"), 
           "Should generate proper Rust documentation");
}

#[test]
fn test_array_length_underflow_bug() {
    // Bug: len(arr) - 1 can underflow if array is empty
    let python_code = r#"
def process_empty_list(arr: list) -> int:
    if len(arr) == 0:
        return -1
    right = len(arr) - 1
    return right
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should handle empty array case properly
    assert!(rust_code.contains("len()") || rust_code.contains(".is_empty()"), 
           "Should use proper length checking");
    
    // Should not cause underflow
    assert!(!rust_code.contains("arr . len () - 1") || rust_code.contains("saturating_sub"), 
           "Should prevent integer underflow in array length calculation");
}

#[test]
fn test_integer_division_bug() {
    // Bug: Python // operator should map to proper integer division
    let python_code = r#"
def divide(a: int, b: int) -> int:
    return a // b
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should use proper integer division, not float division
    assert!(rust_code.contains("/") && !rust_code.contains("/ 2.0"), 
           "Should generate integer division for // operator");
}

#[test]
fn test_list_indexing_bounds_checking_bug() {
    // Bug: Array indexing should have proper bounds checking
    let python_code = r#"
def get_item(arr: list, index: int) -> int:
    return arr[index]
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should use safe indexing methods
    assert!(rust_code.contains(".get(") || rust_code.contains("bounds_check"), 
           "Should use safe array indexing");
}

#[test]
fn test_string_concatenation_efficiency_bug() {
    // Bug: String operations should be efficient
    let python_code = r#"
def concat_strings(a: str, b: str) -> str:
    return a + b
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should use efficient string operations
    assert!(rust_code.contains("format!") || rust_code.contains("String::from") || rust_code.contains("&str"), 
           "Should generate efficient string operations");
}

#[test]
fn test_type_inference_ownership_bug() {
    // Bug: Type inference should generate correct ownership patterns
    let python_code = r#"
def process_data(data: list) -> list:
    result = []
    for item in data:
        result.append(item * 2)
    return result
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should have proper ownership and borrowing
    assert!(rust_code.contains("Vec<") && rust_code.contains("push"), 
           "Should generate proper Vec operations");
    
    // Should not have unnecessary clones or copies
    assert!(!rust_code.contains(".clone().clone()"), 
           "Should not generate redundant clones");
}

#[test]
fn test_docstring_generation_bug() {
    // Bug: Docstrings are converted to expressions instead of comments
    let python_code = r#"
def example_function(x: int) -> int:
    """This is a docstring that should become a comment"""
    return x * 2
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should NOT contain docstring as executable expression
    assert!(!rust_code.contains(r#""This is a docstring that should become a comment" . to_string ()"#), 
           "Docstring should not be an executable expression");
    
    // Should contain proper Rust documentation comment
    assert!(rust_code.contains("/// This is a docstring") || 
           rust_code.contains("#[doc = \"This is a docstring"),
           "Should generate proper Rust documentation");
}

#[test]
fn test_integer_division_operator_bug() {
    // Bug: Python // operator should map to integer division, not float division
    let python_code = r#"
def floor_divide(a: int, b: int) -> int:
    return a // b
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should use integer division, not float division
    // In Rust, integer division with / already truncates toward zero for integers
    // So (a / b) is correct for positive integers, but we should be explicit
    assert!(rust_code.contains("/ b"), "Should contain division operation");
    
    // Should not generate float division
    assert!(!rust_code.contains("/ b as f"), "Should not cast to float for integer division");
}

#[test]
fn test_multiplication_optimization_bug() {
    // Bug: x * 2 should not become bit shift unless safe
    let python_code = r#"
def multiply_by_two(x: int) -> int:
    return x * 2
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should either use multiplication or clearly documented bit shift
    // Bit shift is an optimization but can behave differently for negative numbers
    if rust_code.contains("<<") {
        // If using bit shift, should handle negative numbers correctly
        // or document the optimization
        assert!(rust_code.contains("// Optimized") || rust_code.contains("wrapping_shl"), 
               "Bit shift optimization should be documented or handle edge cases");
    } else {
        assert!(rust_code.contains("* 2") || rust_code.contains("*2"), 
               "Should use multiplication if not optimizing");
    }
}

#[test]
fn test_plain_list_type_mapping_bug() {
    // Bug: Plain 'list' type should map to Vec<T> not literal 'list'
    let python_code = r#"
def process_items(items: list) -> list:
    return items
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should NOT contain literal 'list' type
    assert!(!rust_code.contains("items : list"), 
           "Should not generate literal 'list' type");
    
    // Should contain proper Vec type
    assert!(rust_code.contains("Vec<") || rust_code.contains("&["), 
           "Should generate Vec<T> or slice type for list");
}

#[test]
fn test_integer_underflow_array_length_bug() {
    // Bug: arr.len() - 1 can underflow if array is empty
    let python_code = r#"
def get_last_index(arr: list) -> int:
    return len(arr) - 1
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should use saturating arithmetic or proper bounds checking
    assert!(rust_code.contains("saturating_sub") || 
           rust_code.contains("checked_sub") ||
           rust_code.contains("if") && rust_code.contains("is_empty"), 
           "Should handle potential underflow in array length operations");
}

#[test]
fn test_method_call_spacing_bug() {
    // Bug: Method calls have unnecessary spaces
    let python_code = r#"
def get_length(arr: list) -> int:
    return len(arr)
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should NOT have spaces in method calls
    assert!(!rust_code.contains("arr . len ()"), 
           "Method calls should not have spaces");
    
    // Should have proper method call syntax
    assert!(rust_code.contains("arr.len()") || rust_code.contains("len(arr)"), 
           "Should generate proper method call syntax");
}

#[test]
fn test_bounds_checking_array_indexing_bug() {
    // Bug: Array indexing should have bounds checking
    let python_code = r#"
def get_item(arr: list, i: int) -> int:
    return arr[i]
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Should use safe indexing methods
    assert!(rust_code.contains(".get(") || 
           rust_code.contains("bounds_check") ||
           rust_code.contains("unwrap_or"), 
           "Should use safe array indexing");
}

#[test]
fn test_consistent_type_mapping() {
    // Bug: Type mapping should be consistent across contexts
    let python_code = r#"
def create_list() -> list:
    items = []
    return items
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Return type and variable type should be consistent
    let vec_count = rust_code.matches("Vec<").count();
    let list_count = rust_code.matches(" list").count();
    
    // Should prefer Vec over literal 'list'
    assert!(vec_count >= list_count, 
           "Should consistently use Vec<T> over literal 'list' type");
}

#[test]
fn test_rust_code_formatting_consistency() {
    // Bug: Generated Rust code has inconsistent spacing and formatting issues
    let python_code = r#"
from typing import List

def calculate_sum(numbers: List[int]) -> int:
    """Calculate the sum of a list of integers."""
    total: int = 0
    for n in numbers:
        total += n
    return total
"#;
    
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code).unwrap();
    
    // Check proper generic spacing
    assert!(rust_code.contains("Vec<i32>"), 
            "Expected proper generic spacing 'Vec<i32>', got: {}", rust_code);
    
    // Check proper assignment spacing - should NOT have "=(" 
    assert!(!rust_code.contains("=("), 
            "Found improper assignment spacing '=(', should be ' = (': {}", rust_code);
    
    // Check proper operator spacing
    assert!(rust_code.contains(" = "), 
            "Expected proper assignment operator spacing ' = ': {}", rust_code);
    
    // Check method call spacing - should NOT have spaces in method calls
    assert!(!rust_code.contains(". "), 
            "Found spaces in method calls, should be '.method()': {}", rust_code);
}