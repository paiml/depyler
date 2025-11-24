//! DEPYLER-0483: Incorrect &mut Parameter Type Inference
//!
//! Tests that parameters used immutably are NOT inferred as &mut

use depyler_core::DepylerPipeline;

#[test]
fn test_value_parameter_not_mut() {
    // Python: value is only READ, not mutated
    let python = r#"
def set_value(data, value):
    data[0] = value  # value is READ, not mutated
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // value should be &str (immutable), not &mut str
    assert!(
        !rust.contains("value: &mut str") && !rust.contains("value: &mut String"),
        "BUG: value incorrectly inferred as &mut (it's only READ)\nGenerated:\n{}",
        rust
    );

    // Should be immutable borrow
    assert!(
        rust.contains("value: &str")
            || rust.contains("value: &String")
            || rust.contains("value: String"),
        "Expected value to be &str or String (immutable)\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_mutated_parameter_is_mut() {
    // Python: value IS mutated (reassigned)
    let python = r#"
def mutate_value(data, value):
    value = value.upper()  # value IS mutated
    data[0] = value
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // value should be mut because it's reassigned
    assert!(
        rust.contains("mut value"),
        "Expected value to be mut (it's reassigned)\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_dict_key_value_assignment() {
    // The actual bug case from example_config
    let python = r#"
def set_nested_value(config, key, value):
    keys = key.split(".")
    current = config
    for k in keys[:-1]:
        if k not in current:
            current[k] = {}
        current = current[k]
    current[keys[-1]] = value  # value is READ, not MUTATED
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // value should NOT be &mut
    assert!(
        !rust.contains("value: &mut"),
        "BUG: value incorrectly inferred as &mut in dict assignment\nGenerated:\n{}",
        rust
    );
}
