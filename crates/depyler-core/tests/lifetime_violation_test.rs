//! Tests for detecting lifetime violations

use depyler_core::DepylerPipeline;

#[test]
fn test_dangling_reference_detection() {
    let pipeline = DepylerPipeline::new();

    // This should fail: returning a reference to a local variable
    let python_code = r#"
def get_local_ref() -> str:
    local = "temporary"
    return local
"#;

    let result = pipeline.transpile(python_code);
    // In current implementation, this might succeed but generate incorrect code
    // We should detect this pattern and handle it appropriately
    println!("Result: {:?}", result);
}

#[test]
fn test_reference_outlives_source() {
    let pipeline = DepylerPipeline::new();

    // This pattern should be detected: storing a reference that outlives its source
    let python_code = r#"
def store_reference(container: List[str], temp: str):
    container.append(temp)
"#;

    let result = pipeline.transpile(python_code);
    println!("Result for store_reference: {:?}", result);
}

#[test]
fn test_iterator_invalidation() {
    let pipeline = DepylerPipeline::new();

    // This should be detected: modifying a collection while iterating
    let python_code = r#"
def modify_while_iterating(items: List[int]):
    for item in items:
        if item > 5:
            items.remove(item)
"#;

    let result = pipeline.transpile(python_code);
    println!("Result for modify_while_iterating: {:?}", result);
}

#[test]
fn test_multiple_mutable_borrows() {
    let pipeline = DepylerPipeline::new();

    // This pattern should be detected: multiple mutable references
    let python_code = r#"
def double_mut_borrow(data: List[int]):
    ref1 = data
    ref2 = data
    ref1.append(1)
    ref2.append(2)
"#;

    let result = pipeline.transpile(python_code);
    println!("Result for double_mut_borrow: {:?}", result);
}

#[test]
fn test_use_after_move() {
    let pipeline = DepylerPipeline::new();

    // This should be detected: using a value after it's been moved
    let python_code = r#"
def use_after_move(s: str) -> str:
    result = consume_string(s)
    print(s)  # s has been moved
    return result
    
def consume_string(s: str) -> str:
    return s.upper()
"#;

    let result = pipeline.transpile(python_code);
    println!("Result for use_after_move: {:?}", result);
}

#[test]
fn test_escaping_closure_reference() {
    let pipeline = DepylerPipeline::new();

    // This should be detected: closure capturing reference that doesn't live long enough
    let python_code = r#"
def create_closure():
    local_data = "temporary"
    def inner():
        return local_data
    return inner
"#;

    let result = pipeline.transpile(python_code);
    println!("Result for escaping_closure: {:?}", result);
}

#[test]
fn test_field_lifetime_mismatch() {
    let pipeline = DepylerPipeline::new();

    // This should be detected: struct field with incompatible lifetime
    let python_code = r#"
class Container:
    def __init__(self, data: str):
        self.data = data
        
def create_container(temp: str) -> Container:
    return Container(temp)
"#;

    let result = pipeline.transpile(python_code);
    println!("Result for field_lifetime_mismatch: {:?}", result);
}
