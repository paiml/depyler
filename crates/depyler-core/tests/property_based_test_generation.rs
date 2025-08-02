//! Tests for generating property-based tests using quickcheck

use depyler_core::DepylerPipeline;

#[test]
fn test_generate_quickcheck_for_pure_function() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def add(a: int, b: int) -> int:
    """Adds two integers"""
    return a + b
"#;

    let result = pipeline.transpile(python_code);
    println!("Pure function result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated code:\n{}", rust_code);

        // Should generate quickcheck test for pure functions
        assert!(
            rust_code.contains("#[cfg(test)]") || rust_code.contains("mod tests"),
            "Should generate test module"
        );
        assert!(
            rust_code.contains("quickcheck") || rust_code.contains("proptest"),
            "Should use property-based testing"
        );
    }
}

#[test]
fn test_generate_properties_for_sorting() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def bubble_sort(arr: list[int]) -> list[int]:
    """Sorts an array using bubble sort"""
    n = len(arr)
    for i in range(n):
        for j in range(0, n-i-1):
            if arr[j] > arr[j+1]:
                arr[j], arr[j+1] = arr[j+1], arr[j]
    return arr
"#;

    let result = pipeline.transpile(python_code);
    println!("Sorting function result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated test code:\n{}", rust_code);

        // Properties to test for sorting:
        // 1. Length preservation
        // 2. Elements are sorted
        // 3. Same elements (multiset equality)
    }
}

#[test]
fn test_generate_invariant_tests() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def abs_value(x: int) -> int:
    """Returns absolute value"""
    if x < 0:
        return -x
    return x
"#;

    let result = pipeline.transpile(python_code);
    println!("Abs function result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated invariant tests:\n{}", rust_code);

        // Invariants to test:
        // 1. Result is always non-negative
        // 2. abs(x) == abs(-x)
        // 3. abs(x) >= x
    }
}

#[test]
fn test_generate_roundtrip_properties() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def encode(s: str) -> str:
    """Simple ROT13 encoding"""
    result = ""
    for c in s:
        if 'a' <= c <= 'z':
            result += chr((ord(c) - ord('a') + 13) % 26 + ord('a'))
        elif 'A' <= c <= 'Z':
            result += chr((ord(c) - ord('A') + 13) % 26 + ord('A'))
        else:
            result += c
    return result

def decode(s: str) -> str:
    """ROT13 decoding (same as encoding)"""
    return encode(s)
"#;

    let result = pipeline.transpile(python_code);
    println!("Encoding functions result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated roundtrip test:\n{}", rust_code);

        // Roundtrip property: decode(encode(s)) == s
    }
}

#[test]
fn test_generate_bounds_checking() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def safe_divide(a: int, b: int) -> int:
    """Integer division with safety check"""
    if b == 0:
        return 0
    return a // b
"#;

    let result = pipeline.transpile(python_code);
    println!("Safe divide result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated bounds test:\n{}", rust_code);

        // Properties:
        // 1. Never panics (no divide by zero)
        // 2. Returns 0 when b == 0
    }
}

#[test]
fn test_generate_idempotence_test() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def normalize_path(path: str) -> str:
    """Normalizes a file path"""
    # Remove duplicate slashes
    while '//' in path:
        path = path.replace('//', '/')
    # Remove trailing slash unless root
    if len(path) > 1 and path.endswith('/'):
        path = path[:-1]
    return path
"#;

    let result = pipeline.transpile(python_code);
    println!("Normalize path result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated idempotence test:\n{}", rust_code);

        // Idempotence: normalize(normalize(x)) == normalize(x)
    }
}

#[test]
fn test_custom_generators() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def is_valid_email(email: str) -> bool:
    """Checks if email is valid"""
    if '@' not in email:
        return False
    parts = email.split('@')
    if len(parts) != 2:
        return False
    local, domain = parts
    return len(local) > 0 and len(domain) > 0 and '.' in domain
"#;

    let result = pipeline.transpile(python_code);
    println!("Email validation result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated custom generator test:\n{}", rust_code);

        // Should generate custom email generators for testing
    }
}

#[test]
fn test_shrinking_support() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def find_max(arr: list[int]) -> int:
    """Finds maximum element in array"""
    if len(arr) == 0:
        return 0
    max_val = arr[0]
    for x in arr:
        if x > max_val:
            max_val = x
    return max_val
"#;

    let result = pipeline.transpile(python_code);
    println!("Find max result: {:?}", result);

    if let Ok(rust_code) = result {
        println!("Generated shrinking test:\n{}", rust_code);

        // Should support shrinking to find minimal failing cases
    }
}
