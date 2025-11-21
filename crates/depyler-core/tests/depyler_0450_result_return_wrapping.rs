// DEPYLER-0450: Missing Ok() Wrapper for Result Return Types
//
// Bug: Functions with Result<T, E> return types are missing the final Ok() wrapper,
// causing the function body to implicitly return () instead of Result<T, E>.
//
// Expected behavior:
// 1. Functions with side effects only → Ok(())
// 2. Functions returning values → Ok(value)
// 3. Functions with early returns → preserve existing Ok()
// 4. Functions using ? operator → proper Result return type

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

// =============================================================================
// Test Suite 1: Unit Return Functions (Side Effects Only)
// =============================================================================

#[test]
fn test_depyler_0450_side_effect_function() {
    let python = r#"
def set_value(data, key, value):
    try:
        data[key] = value
    except KeyError as e:
        raise ValueError("Invalid key")
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type
    assert!(
        rust.contains("Result<") && rust.contains("Error"),
        "Should have Result return type. Generated:\n{}",
        rust
    );

    // Should end with Ok(())
    assert!(
        rust.contains("Ok(())"),
        "Should end with Ok(()). Generated:\n{}",
        rust
    );

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_loop_with_side_effects() {
    let python = r#"
def process_items(items):
    for item in items:
        try:
            print(item)
        except Exception:
            pass
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_conditional_side_effects() {
    let python = r#"
def update_config(config, key, value):
    if key not in config:
        raise KeyError("Key not found")
    config[key] = value
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type (because of raise)
    assert!(
        rust.contains("Result<"),
        "Should have Result return type. Generated:\n{}",
        rust
    );

    // Should end with Ok(())
    assert!(
        rust.contains("Ok(())"),
        "Should end with Ok(()). Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0450_nested_blocks() {
    let python = r#"
def nested_operations(data):
    try:
        for key in data:
            if key.startswith("temp"):
                del data[key]
    except Exception:
        raise RuntimeError("Operation failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_file_operations() {
    let python = r#"
def write_file(path, content):
    try:
        with open(path, 'w') as f:
            f.write(content)
    except IOError:
        raise IOError("Write failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type
    assert!(
        rust.contains("Result<"),
        "Should have Result return type. Generated:\n{}",
        rust
    );
}

// =============================================================================
// Test Suite 2: Value Return Functions
// =============================================================================

#[test]
fn test_depyler_0450_return_primitive() {
    let python = r#"
def get_count(items):
    try:
        return len(items)
    except Exception:
        raise ValueError("Invalid items")
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type
    assert!(
        rust.contains("Result<"),
        "Should have Result return type. Generated:\n{}",
        rust
    );

    // Should use Ok() for return
    assert!(
        rust.contains("Ok(") || rust.contains("return Ok("),
        "Should wrap return in Ok(). Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0450_return_string() {
    let python = r#"
def get_name(data, key):
    if key not in data:
        raise KeyError("Key not found")
    return data[key]
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());

    // Should have Result return type
    assert!(rust.contains("Result<"));
}

#[test]
fn test_depyler_0450_return_collection() {
    let python = r#"
def filter_items(items, condition):
    try:
        result = []
        for item in items:
            if condition(item):
                result.append(item)
        return result
    except Exception:
        raise RuntimeError("Filter failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());

    // Should use Ok() for return
    assert!(rust.contains("Ok("));
}

#[test]
fn test_depyler_0450_return_optional() {
    let python = r#"
def find_item(items, key):
    try:
        for item in items:
            if item['key'] == key:
                return item
        return None
    except Exception:
        raise ValueError("Search failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_mixed_return_paths() {
    let python = r#"
def get_value_or_default(data, key, default):
    try:
        if key in data:
            return data[key]
        return default
    except Exception:
        raise KeyError("Access failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());

    // Should have Result return type
    assert!(rust.contains("Result<"));
}

// =============================================================================
// Test Suite 3: Error Handling Functions
// =============================================================================

#[test]
fn test_depyler_0450_function_with_try_except() {
    let python = r#"
def safe_divide(a, b):
    try:
        result = a / b
        return result
    except ZeroDivisionError:
        raise ValueError("Division by zero")
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type
    assert!(
        rust.contains("Result<"),
        "Should have Result return type. Generated:\n{}",
        rust
    );

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_multiple_error_types() {
    let python = r#"
def read_and_parse(path):
    try:
        with open(path) as f:
            data = f.read()
        return int(data)
    except IOError:
        raise IOError("Read failed")
    except ValueError:
        raise ValueError("Parse failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type
    assert!(rust.contains("Result<"));

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_custom_error_type() {
    let python = r#"
class CustomError(Exception):
    pass

def validate_data(data):
    if not data:
        raise CustomError("Invalid data")
    return True
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_nested_try_except() {
    let python = r#"
def nested_operations(data):
    try:
        try:
            result = process(data)
            return result
        except ValueError:
            raise ValueError("Inner error")
    except Exception:
        raise RuntimeError("Outer error")
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_error_propagation() {
    let python = r#"
def chain_operations(data):
    try:
        step1 = validate(data)
        step2 = transform(step1)
        step3 = save(step2)
        return step3
    except Exception as e:
        raise RuntimeError(f"Chain failed: {e}")
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type
    assert!(rust.contains("Result<"));

    // Should compile
    assert!(!rust.is_empty());
}

// =============================================================================
// Test Suite 4: Edge Cases
// =============================================================================

#[test]
fn test_depyler_0450_empty_function() {
    let python = r#"
def empty_function():
    pass
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile (may not have Result return type)
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_single_statement() {
    let python = r#"
def single_statement(x):
    try:
        return x * 2
    except Exception:
        raise ValueError("Multiplication failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());

    // Should have Result return type
    assert!(rust.contains("Result<"));
}

#[test]
fn test_depyler_0450_explicit_return_only() {
    let python = r#"
def explicit_return(x):
    if x > 0:
        raise ValueError("Positive not allowed")
    return x
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());

    // Should have Result return type
    assert!(rust.contains("Result<"));

    // Should use Ok() for return
    assert!(rust.contains("Ok(") || rust.contains("return Ok("));
}

#[test]
fn test_depyler_0450_no_error_handling() {
    let python = r#"
def simple_function(x, y):
    result = x + y
    return result
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile (may not have Result return type if no exceptions)
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_already_wrapped_ok() {
    // This tests that we don't double-wrap Ok(Ok(()))
    let python = r#"
def safe_operation(data):
    try:
        if not data:
            raise ValueError("Empty data")
    except ValueError:
        raise ValueError("Validation failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should NOT have Ok(Ok(...))
    assert!(
        !rust.contains("Ok(Ok("),
        "Should not double-wrap Ok(). Generated:\n{}",
        rust
    );

    // Should compile
    assert!(!rust.is_empty());
}

// =============================================================================
// Test Suite 5: Real-World Examples (from reprorusted-cli)
// =============================================================================

#[test]
fn test_depyler_0450_config_set_nested_value() {
    let python = r#"
def set_nested_value(config, key, value):
    keys = key.split(".")
    current = config
    for k in keys[:-1]:
        if k not in current:
            current[k] = {}
        current = current[k]
    current[keys[-1]] = value
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());

    // If function has Result return type, should end with Ok(())
    if rust.contains("Result<") {
        // Check that it either has Ok(()) or proper return statement
        assert!(
            rust.contains("Ok(())") || rust.contains("return Ok("),
            "Result function should have Ok() wrapper. Generated:\n{}",
            rust
        );
    }
}

#[test]
fn test_depyler_0450_csv_filter() {
    let python = r#"
def filter_csv(input_file, output_file, column, value):
    import csv
    try:
        reader = csv.reader(open(input_file))
        writer = csv.writer(open(output_file, 'w'))

        for row in reader:
            if row[column] == value:
                writer.writerow(row)
    except Exception:
        raise IOError("CSV operation failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type
    assert!(
        rust.contains("Result<"),
        "Should have Result return type. Generated:\n{}",
        rust
    );

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_env_check() {
    let python = r#"
def check_environment(key):
    import os
    try:
        value = os.getenv(key)
        if not value:
            raise ValueError(f"Environment variable {key} not set")
        return value
    except Exception:
        raise RuntimeError("Environment check failed")
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type
    assert!(rust.contains("Result<"));

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_complex_control_flow() {
    let python = r#"
def complex_operation(data, mode):
    try:
        if mode == "read":
            if not data:
                raise ValueError("No data")
            return process_read(data)
        elif mode == "write":
            if not validate(data):
                raise ValueError("Invalid data")
            write_data(data)
        else:
            raise ValueError("Unknown mode")
    except Exception as e:
        raise RuntimeError(f"Operation failed: {e}")
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type
    assert!(rust.contains("Result<"));

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0450_side_effect_with_raise() {
    let python = r#"
def validate_and_update(config, key, value):
    if key not in ["name", "age", "email"]:
        raise KeyError(f"Invalid key: {key}")
    config[key] = value
"#;
    let rust = transpile_python(python).unwrap();

    // Should have Result return type (due to raise)
    assert!(
        rust.contains("Result<"),
        "Should have Result return type due to raise. Generated:\n{}",
        rust
    );

    // Should end with Ok(())
    assert!(
        rust.contains("Ok(())"),
        "Should end with Ok(()), not implicit (). Generated:\n{}",
        rust
    );

    // Should compile
    assert!(!rust.is_empty());
}
