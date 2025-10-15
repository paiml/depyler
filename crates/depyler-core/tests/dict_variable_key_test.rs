//! Test for dict access with variable string keys (DEPYLER-0095)
//!
//! This test ensures that dict access with string variables generates correct
//! HashMap.get() calls, not incorrect "as usize" casts.

use depyler_core::DepylerPipeline;

#[test]
fn test_dict_access_with_string_variable() {
    let python_code = r#"
from typing import Dict, List

def lookup_values(data: Dict[str, int], keys: List[str]) -> List[int]:
    """Test dict access with string variable keys."""
    results = []
    for key in keys:
        if key in data:
            results.append(data[key])
        else:
            results.append(0)
    return results
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should NOT contain "as usize" for string keys
    assert!(
        !rust_code.contains("key as usize"),
        "Dict access with string variable should not cast to usize.\nGenerated code:\n{}",
        rust_code
    );

    // Should contain proper HashMap.get() with string key
    assert!(
        rust_code.contains("data.get(key)") || rust_code.contains("data.get(&key)"),
        "Dict access should use .get(key) or .get(&key).\nGenerated code:\n{}",
        rust_code
    );
}

#[test]
fn test_dict_literal_key_access() {
    let python_code = r#"
from typing import Dict

def get_value(data: Dict[str, int]) -> int:
    """Test dict access with string literal."""
    return data["mykey"]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should NOT contain "as usize" for string literal keys
    assert!(
        !rust_code.contains("as usize"),
        "Dict access with string literal should not cast to usize.\nGenerated code:\n{}",
        rust_code
    );

    // Should contain proper HashMap.get() with string literal
    assert!(
        rust_code.contains(".get(\"mykey\")"),
        "Dict access should use .get(\"mykey\").\nGenerated code:\n{}",
        rust_code
    );
}

#[test]
fn test_list_access_with_int_variable() {
    let python_code = r#"
from typing import List

def get_item(items: List[int], index: int) -> int:
    """Test list access with int variable."""
    return items[index]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // SHOULD contain "as usize" for integer index
    assert!(
        rust_code.contains("as usize"),
        "List access with int variable should cast to usize.\nGenerated code:\n{}",
        rust_code
    );
}
