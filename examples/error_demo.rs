//! Demonstration of enhanced error reporting in Depyler
//! Shows how type mismatch errors provide helpful Python→Rust guidance

use depyler_core::error::{ErrorKind, TranspileError};
use depyler_core::error_reporting::EnhancedError;

fn main() {
    println!("=== Depyler Enhanced Error Reporting Demo ===\n");

    // Example 1: String type mismatch
    println!("1. String Type Mismatch (str vs String):");
    println!("-------------------------------------------");
    let error1 = EnhancedError::new(ErrorKind::TypeMismatch {
        expected: "String".to_string(),
        found: "&str".to_string(),
        context: "function return type".to_string(),
    })
    .with_location("example.py", 5, 12)
    .with_source_line("    return text.upper()");

    println!("{}\n", error1);

    // Example 2: Division type mismatch
    println!("2. Division Result Type Mismatch:");
    println!("-------------------------------------------");
    let error2 = EnhancedError::new(ErrorKind::TypeMismatch {
        expected: "f64".to_string(),
        found: "i32".to_string(),
        context: "division operation".to_string(),
    })
    .with_location("example.py", 10, 16)
    .with_source_line("    result = total / count");

    println!("{}\n", error2);

    // Example 3: Option/None type mismatch
    println!("3. Option Type Mismatch (None handling):");
    println!("-------------------------------------------");
    let error3 = EnhancedError::new(ErrorKind::TypeMismatch {
        expected: "Option<i32>".to_string(),
        found: "None".to_string(),
        context: "return value".to_string(),
    })
    .with_location("example.py", 15, 12)
    .with_source_line("    return None");

    println!("{}\n", error3);

    // Example 4: Ownership mismatch
    println!("4. Ownership Mismatch (&str vs String):");
    println!("-------------------------------------------");
    let error4 = EnhancedError::new(ErrorKind::TypeMismatch {
        expected: "&String".to_string(),
        found: "String".to_string(),
        context: "parameter type".to_string(),
    })
    .with_location("example.py", 20, 25)
    .with_source_line("    process_text(name)");

    println!("{}\n", error4);

    println!("=== End of Demo ===");
}
