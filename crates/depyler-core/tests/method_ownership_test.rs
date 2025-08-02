//! Tests for ownership transfer validation in method calls

use depyler_core::DepylerPipeline;

#[test]
fn test_vector_push_takes_ownership() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def add_to_list(items: List[str], new_item: str):
    items.append(new_item)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for add_to_list:\n{}", rust_code);

    // Vec::push takes ownership of the value
    assert!(rust_code.contains("push"), "Should use push method");
    // The new_item parameter should be moved, not borrowed
    assert!(
        !rust_code.contains("&new_item"),
        "Should not borrow new_item when pushing"
    );
}

#[test]
#[ignore = "Dictionary assignment not yet supported"]
fn test_hashmap_insert_takes_ownership() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def add_to_dict(data: Dict[str, int], key: str, value: int):
    data[key] = value
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for add_to_dict:\n{}", rust_code);

    // HashMap::insert takes ownership of both key and value
    assert!(rust_code.contains("insert"), "Should use insert method");
}

#[test]
fn test_string_method_borrowing() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_string(s: str) -> bool:
    return s.startswith("hello")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for check_string:\n{}", rust_code);

    // String methods like startswith should borrow, not move
    assert!(
        rust_code.contains("starts_with"),
        "Should use starts_with method"
    );
    assert!(rust_code.contains("&"), "Should borrow string parameter");
}

#[test]
fn test_method_chain_ownership() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_string(s: str) -> str:
    return s.strip().upper()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for process_string:\n{}", rust_code);

    // Method chains should handle ownership correctly
    assert!(rust_code.contains("trim"), "Should use trim method");
    assert!(
        rust_code.contains("to_uppercase"),
        "Should use to_uppercase method"
    );
}

#[test]
fn test_consuming_iterator_methods() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def sum_list(numbers: List[int]) -> int:
    return sum(numbers)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for sum_list:\n{}", rust_code);

    // Iterator consuming methods should handle ownership
    assert!(
        rust_code.contains("sum") || rust_code.contains("iter"),
        "Should use iterator methods"
    );
}

#[test]
#[ignore = "Classes not yet supported"]
fn test_self_consuming_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
class Counter:
    def __init__(self):
        self.count = 0
    
    def increment(self):
        self.count += 1
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for Counter:\n{}", rust_code);

    // Methods that mutate self should take &mut self, not self
    assert!(
        rust_code.contains("&mut self") || rust_code.contains("mut self"),
        "Should handle self mutation correctly"
    );
}
