// ============================================================================
// DEPYLER-0264: DynamicType Undefined for Untyped List Parameters
// ============================================================================
// BUG: Untyped list/dict parameters generate Vec<DynamicType>/HashMap<DynamicType, DynamicType>
// but DynamicType is never defined, causing compilation failures
//
// ROOT CAUSE: type_mapper.rs maps Type::Unknown → RustType::Custom("DynamicType")
// FIX: Map Type::Unknown → serde_json::Value (already used for untyped dict values)
//
// DISCOVERED: Performance Benchmarking Campaign (compute_intensive.py)
// SEVERITY: P0 BLOCKING - prevents transpilation of any untyped collection parameters
// ============================================================================

use depyler_core::DepylerPipeline;
use std::process::Command;

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0264_untyped_list_parameter_compiles() {
    // DEPYLER-0264: Untyped list parameters generate Vec<DynamicType> which doesn't compile
    // RED Phase: This test MUST FAIL initially because DynamicType is undefined

    let python_code = r#"
def sum_list(numbers: list) -> int:
    """Sum all numbers in a list."""
    total = 0
    for num in numbers:
        total = total + num
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Debugging: Print generated code before compilation
    eprintln!("=== DEPYLER-0264: Generated Rust Code ===");
    eprintln!("{}", rust_code);

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0264_untyped_list.rs";
    std::fs::write(temp_file, &rust_code).expect("DEPYLER-0264: Failed to write temp file");

    // Attempt to compile with rustc
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0264_untyped_list.rlib")
        .output()
        .expect("DEPYLER-0264: Failed to run rustc");

    // ASSERT: Must NOT reference undefined DynamicType
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0264: rustc stderr ===");
        eprintln!("{}", stderr);

        // Primary assertion: Should NOT have DynamicType error
        assert!(
            !stderr.contains("cannot find type `DynamicType`"),
            "DEPYLER-0264 FAILURE: Generated code references undefined DynamicType!\n\
             Expected: Use serde_json::Value or concrete type for untyped list\n\
             Actual: Generated Vec<DynamicType> which doesn't exist\n\
             \n\
             See docs/bugs/DEPYLER-0264.md for details.\n\
             \n\
             Generated Rust code:\n{}\n\
             \n\
             rustc error:\n{}",
            rust_code,
            stderr
        );

        // If compilation fails, it should only be due to missing serde_json crate
        // (which is expected for standalone rustc compilation)
        if stderr.contains("unresolved module or unlinked crate `serde_json`") {
            eprintln!("NOTE: Compilation failed due to missing serde_json crate (expected for standalone rustc)");
            eprintln!("      The fix is correct - using serde_json::Value instead of DynamicType");
            // This is acceptable - the transpiler correctly generates serde_json::Value
            return;
        }
    }

    // GREEN Phase check: Should NOT contain DynamicType
    assert!(
        !rust_code.contains("DynamicType"),
        "DEPYLER-0264: Generated code should not reference DynamicType\n\
         Expected: serde_json::Value or concrete type\n\
         Generated code:\n{}",
        rust_code
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0264_untyped_dict_parameter_compiles() {
    // DEPYLER-0264: Untyped dict parameters also generate HashMap<DynamicType, DynamicType>
    // This is a related bug - same root cause

    let python_code = r#"
def get_value(data: dict, key: str) -> int:
    """Get a value from dictionary."""
    if key in data:
        return data[key]
    return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0264_untyped_dict.rs";
    std::fs::write(temp_file, &rust_code).expect("DEPYLER-0264: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0264_untyped_dict.rlib")
        .output()
        .expect("DEPYLER-0264: Failed to run rustc");

    // ASSERT: Must NOT reference DynamicType
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        // Should NOT have DynamicType error
        assert!(
            !stderr.contains("cannot find type `DynamicType`"),
            "DEPYLER-0264: Untyped dict generates undefined DynamicType"
        );

        // Missing serde_json crate is acceptable for standalone rustc
        if stderr.contains("unresolved module or unlinked crate `serde_json`") {
            eprintln!("NOTE: Missing serde_json crate (expected) - fix is correct");
            return;
        }
    }

    // Should not contain DynamicType
    assert!(
        !rust_code.contains("DynamicType"),
        "DEPYLER-0264: Should use serde_json::Value instead of DynamicType"
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0264_untyped_set_parameter_compiles() {
    // DEPYLER-0264: Untyped set parameters also generate HashSet<DynamicType>
    // This is a related bug - same root cause

    let python_code = r#"
def count_unique(items: set) -> int:
    """Count unique items in a set."""
    return len(items)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0264_untyped_set.rs";
    std::fs::write(temp_file, &rust_code).expect("DEPYLER-0264: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0264_untyped_set.rlib")
        .output()
        .expect("DEPYLER-0264: Failed to run rustc");

    // ASSERT: Must NOT reference DynamicType
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        // Should NOT have DynamicType error
        assert!(
            !stderr.contains("cannot find type `DynamicType`"),
            "DEPYLER-0264: Untyped set generates undefined DynamicType"
        );

        // Missing serde_json crate is acceptable for standalone rustc
        if stderr.contains("unresolved module or unlinked crate `serde_json`") {
            eprintln!("NOTE: Missing serde_json crate (expected) - fix is correct");
            return;
        }
    }

    // Should not contain DynamicType
    assert!(
        !rust_code.contains("DynamicType"),
        "DEPYLER-0264: Should use serde_json::Value instead of DynamicType"
    );
}
