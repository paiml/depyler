// DEPYLER-0514: Missing variable assignments for ternary conditional expressions before try/with blocks
//
// Bug: When a ternary assignment appears before a try/with block, the assignment statement
// is not generated in the Rust output, causing E0425 "cannot find value" errors.
//
// Example Python:
//   hasher = hashlib.md5() if algorithm == "md5" else hashlib.sha256()
//   try:
//       with open(file) as f:
//           hasher.update(f.read())
//
// Expected Rust:
//   let mut hasher = if algorithm == "md5" { ... } else { ... };
//   // try/with block here
//
// Actual Rust (BUG):
//   // hasher assignment MISSING!
//   // try/with block
//   hasher.update(...) // E0425: cannot find value 'hasher'

use depyler_core::DepylerPipeline;

#[test]
fn test_depyler_0514_ternary_assignment_before_try() {
    let python_code = r#"
import hashlib

def calculate_hash(file_path, algorithm):
    if algorithm not in ["md5", "sha256"]:
        raise ValueError(f"Unsupported: {algorithm}")

    hasher = hashlib.md5() if algorithm == "md5" else hashlib.sha256()

    try:
        with open(file_path, "rb") as f:
            while True:
                chunk = f.read(8192)
                if not chunk:
                    break
                hasher.update(chunk)
        return hasher.hexdigest()
    except Exception as e:
        raise RuntimeError(f"Failed: {e}") from e
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code).expect("Transpilation should succeed");
    let rust_code = result;

    // MUST have hasher variable declaration
    assert!(
        rust_code.contains("let") && rust_code.contains("hasher"),
        "Generated Rust code MUST contain 'let hasher' declaration.\nGenerated code:\n{}",
        rust_code
    );

    // MUST have if/else expression for the ternary
    assert!(
        rust_code.contains("if algorithm ==") && rust_code.contains("else"),
        "Generated Rust code MUST contain if/else expression for ternary.\nGenerated code:\n{}",
        rust_code
    );

    // MUST NOT have undefined variable usage
    let lines: Vec<&str> = rust_code.lines().collect();
    let hasher_def_line = lines
        .iter()
        .position(|line: &&str| line.contains("let") && line.contains("hasher"));
    let hasher_use_line = lines
        .iter()
        .position(|line: &&str| line.contains("hasher.") || line.contains("hasher)"));

    match (hasher_def_line, hasher_use_line) {
        (Some(def), Some(use_pos)) => {
            assert!(
                def < use_pos,
                "hasher must be defined (line {}) BEFORE it's used (line {})",
                def + 1,
                use_pos + 1
            );
        }
        (None, Some(use_pos)) => {
            panic!(
                "hasher is used at line {} but NEVER defined! This is E0425 error.",
                use_pos + 1
            );
        }
        _ => {} // Other cases are fine
    }
}

#[test]
fn test_depyler_0514_simple_ternary_assignment() {
    let python_code = r#"
def select_value(flag):
    result = "yes" if flag else "no"
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code).expect("Transpilation should succeed");
    let rust_code = result;

    // Simple ternary should work (regression check)
    assert!(
        rust_code.contains("let") && rust_code.contains("result"),
        "Simple ternary assignment should generate 'let result'.\nGenerated code:\n{}",
        rust_code
    );
}

#[test]
fn test_depyler_0514_ternary_with_function_calls() {
    let python_code = r#"
import hashlib

def get_hasher(algorithm):
    hasher = hashlib.md5() if algorithm == "md5" else hashlib.sha256()
    return hasher
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code).expect("Transpilation should succeed");
    let rust_code = result;

    // Ternary with function calls should generate let binding
    assert!(
        rust_code.contains("let") && rust_code.contains("hasher"),
        "Ternary with function calls MUST generate 'let hasher'.\nGenerated code:\n{}",
        rust_code
    );
}
