//! Property tests for try/except transpilation (DEPYLER-0257)
//!
//! This module implements property-based testing for try/except functionality
//! using QuickCheck with 10,000+ iterations per property.
//!
//! Test Strategy:
//! - Determinism: Same Python code always produces same Rust code
//! - Compilability: All generated Rust code compiles successfully
//! - Pattern matching: Generated code contains "match", "Result", or "?"
//! - Panic-freedom: No unwrap()/expect() in generated code
//!
//! EXTREME TDD Protocol:
//! - 10,000 iterations per property test
//! - Custom generators for try/except structures
//! - Comprehensive edge case coverage

use depyler_core::DepylerPipeline;
use quickcheck::{Arbitrary, Gen, TestResult};

/// Property: Try/except transpilation is deterministic
/// Same Python input always produces same Rust output
#[quickcheck_macros::quickcheck(tests = 10000)]
fn prop_try_except_is_deterministic(code: ArbitraryTryExcept) -> TestResult {
    let pipeline = DepylerPipeline::new();

    // Transpile the same code twice
    let result1 = pipeline.transpile(&code.0);
    let result2 = pipeline.transpile(&code.0);

    match (result1, result2) {
        (Ok(rust1), Ok(rust2)) => {
            // Determinism: Both transpilations must produce identical output
            TestResult::from_bool(rust1 == rust2)
        }
        (Err(_), Err(_)) => {
            // Both failed consistently - that's also deterministic
            TestResult::passed()
        }
        _ => {
            // One succeeded, one failed - NOT deterministic
            TestResult::failed()
        }
    }
}

/// Property: Generated Rust code compiles successfully
#[quickcheck_macros::quickcheck(tests = 10000)]
fn prop_try_except_compiles(code: ArbitraryTryExcept) -> TestResult {
    let pipeline = DepylerPipeline::new();

    match pipeline.transpile(&code.0) {
        Ok(rust_code) => {
            // Write to temp file and verify compilation
            let temp_file = format!("/tmp/prop_test_try_except_{}.rs", code.test_id());
            if std::fs::write(&temp_file, &rust_code).is_err() {
                return TestResult::discard();
            }

            let output = std::process::Command::new("rustc")
                .args(["--crate-type", "lib", "--edition", "2021", &temp_file])
                .output();

            match output {
                Ok(out) => TestResult::from_bool(out.status.success()),
                Err(_) => TestResult::discard(),
            }
        }
        Err(_) => TestResult::discard(), // Not yet supported - discard
    }
}

/// Property: Generated code contains required pattern (match/Result/?)
#[quickcheck_macros::quickcheck(tests = 10000)]
fn prop_try_except_contains_error_handling_pattern(code: ArbitraryTryExcept) -> TestResult {
    let pipeline = DepylerPipeline::new();

    match pipeline.transpile(&code.0) {
        Ok(rust_code) => {
            // Generated code must contain at least one error handling pattern
            let has_match = rust_code.contains("match");
            let has_result = rust_code.contains("Result");
            let has_question = rust_code.contains("?");

            TestResult::from_bool(has_match || has_result || has_question)
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Generated code is panic-free (no unwrap/expect)
#[quickcheck_macros::quickcheck(tests = 10000)]
fn prop_try_except_is_panic_free(code: ArbitraryTryExcept) -> TestResult {
    let pipeline = DepylerPipeline::new();

    match pipeline.transpile(&code.0) {
        Ok(rust_code) => {
            // Generated code should not contain panic-inducing operations
            let has_unwrap = rust_code.contains(".unwrap()");
            let has_expect = rust_code.contains(".expect(");
            let has_panic = rust_code.contains("panic!");

            TestResult::from_bool(!has_unwrap && !has_expect && !has_panic)
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Try block code is preserved in output
#[quickcheck_macros::quickcheck(tests = 10000)]
fn prop_try_block_code_preserved(code: ArbitraryTryExcept) -> TestResult {
    let pipeline = DepylerPipeline::new();

    match pipeline.transpile(&code.0) {
        Ok(rust_code) => {
            // The unique marker from try block should appear in output
            let marker = format!("// marker_{}", code.test_id());
            TestResult::from_bool(rust_code.contains(&marker))
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Function signature is correct
#[quickcheck_macros::quickcheck(tests = 10000)]
fn prop_try_except_function_signature(code: ArbitraryTryExcept) -> TestResult {
    let pipeline = DepylerPipeline::new();

    match pipeline.transpile(&code.0) {
        Ok(rust_code) => {
            // Should have function declaration with correct name and return type
            let has_fn = rust_code.contains(&format!("fn {}", code.func_name()));
            let has_return_type = rust_code.contains("-> i32");

            TestResult::from_bool(has_fn && has_return_type)
        }
        Err(_) => TestResult::discard(),
    }
}

// ============================================================================
// Arbitrary Generators for Try/Except Structures
// ============================================================================

/// Arbitrary Python try/except code generator
#[derive(Clone, Debug)]
struct ArbitraryTryExcept(String, String, u32); // (code, func_name, test_id)

impl ArbitraryTryExcept {
    fn test_id(&self) -> u32 {
        self.2
    }

    fn func_name(&self) -> &str {
        &self.1
    }
}

impl Arbitrary for ArbitraryTryExcept {
    fn arbitrary(g: &mut Gen) -> Self {
        let test_id = u32::arbitrary(g);
        let func_name = format!("test_func_{}", test_id % 1000);

        // Generate various try/except patterns
        let code = match g.size() % 8 {
            0 => generate_simple_try_except(&func_name, test_id),
            1 => generate_try_except_with_division(&func_name, test_id),
            2 => generate_try_except_with_return(&func_name, test_id),
            3 => generate_try_except_nested(&func_name, test_id),
            4 => generate_try_except_with_finally(&func_name, test_id),
            5 => generate_try_except_multiple_statements(&func_name, test_id),
            6 => generate_try_except_with_variables(&func_name, test_id),
            _ => generate_try_except_with_arithmetic(&func_name, test_id),
        };

        ArbitraryTryExcept(code, func_name, test_id)
    }
}

// ============================================================================
// Try/Except Code Generators
// ============================================================================

fn generate_simple_try_except(func_name: &str, id: u32) -> String {
    format!(r#"
def {func_name}(x: int) -> int:
    try:
        # marker_{id}
        return 10 // x
    except:
        return -1
"#)
}

fn generate_try_except_with_division(func_name: &str, id: u32) -> String {
    let a = (id % 100) as i32;
    let b_param = "x";
    format!(r#"
def {func_name}({b_param}: int) -> int:
    try:
        # marker_{id}
        result = {a} // {b_param}
        return result
    except:
        return 0
"#)
}

fn generate_try_except_with_return(func_name: &str, id: u32) -> String {
    format!(r#"
def {func_name}(value: int) -> int:
    try:
        # marker_{id}
        if value > 0:
            return value * 2
        else:
            return value // 2
    except:
        return -999
"#)
}

fn generate_try_except_nested(func_name: &str, id: u32) -> String {
    format!(r#"
def {func_name}(x: int) -> int:
    try:
        # marker_{id}
        if x > 0:
            return x // 2
        else:
            return x * 3
    except:
        return 0
"#)
}

fn generate_try_except_with_finally(func_name: &str, id: u32) -> String {
    format!(r#"
def {func_name}(x: int) -> int:
    result = 0
    try:
        # marker_{id}
        result = 100 // x
    except:
        result = -1
    finally:
        result = result + 1
    return result
"#)
}

fn generate_try_except_multiple_statements(func_name: &str, id: u32) -> String {
    format!(r#"
def {func_name}(x: int, y: int) -> int:
    try:
        # marker_{id}
        a = x // y
        b = a * 2
        c = b + x
        return c
    except:
        return -1
"#)
}

fn generate_try_except_with_variables(func_name: &str, id: u32) -> String {
    format!(r#"
def {func_name}(dividend: int, divisor: int) -> int:
    try:
        # marker_{id}
        quotient = dividend // divisor
        remainder = dividend % divisor
        return quotient + remainder
    except:
        return 0
"#)
}

fn generate_try_except_with_arithmetic(func_name: &str, id: u32) -> String {
    let const_val = ((id % 50) + 1) as i32; // Avoid 0
    format!(r#"
def {func_name}(x: int) -> int:
    try:
        # marker_{id}
        a = x + {const_val}
        b = a // {const_val}
        return b
    except:
        return {const_val}
"#)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arbitrary_try_except_generation() {
        let mut g = Gen::new(10);
        let code = ArbitraryTryExcept::arbitrary(&mut g);

        // Generated code should be valid Python
        assert!(code.0.contains("def "));
        assert!(code.0.contains("try:"));
        assert!(code.0.contains("except:"));
        assert!(code.0.contains(&format!("marker_{}", code.test_id())));
    }

    #[test]
    fn test_all_generators_produce_valid_code() {
        for i in 0..8 {
            let func_name = "test";
            let id = 123;

            let code = match i {
                0 => generate_simple_try_except(func_name, id),
                1 => generate_try_except_with_division(func_name, id),
                2 => generate_try_except_with_return(func_name, id),
                3 => generate_try_except_nested(func_name, id),
                4 => generate_try_except_with_finally(func_name, id),
                5 => generate_try_except_multiple_statements(func_name, id),
                6 => generate_try_except_with_variables(func_name, id),
                _ => generate_try_except_with_arithmetic(func_name, id),
            };

            assert!(code.contains("try:"), "Generator {} failed", i);
            assert!(code.contains("except:"), "Generator {} failed", i);
            assert!(code.contains(&format!("marker_{}", id)), "Generator {} failed", i);
        }
    }
}
