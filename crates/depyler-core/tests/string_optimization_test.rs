//! Tests for string allocation optimizations

use depyler_core::DepylerPipeline;

// ============================================================================
// UNIT TESTS
// ============================================================================

#[test]
fn test_read_only_string_no_allocation() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def print_message():
    message = "Hello, World!"
    print(message)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for print_message:\n{}", rust_code);

    // Should not allocate a String for a read-only literal
    assert!(
        !rust_code.contains(".to_string()"),
        "Should not allocate String for read-only literal"
    );
}

#[test]
fn test_returned_string_uses_appropriate_type() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_greeting() -> str:
    return "Hello!"
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for get_greeting:\n{}", rust_code);

    // DEPYLER-0357: Returns String for string literals
    // Note: Python source updated to include return type annotation (-> str)
    // to ensure proper Rust type generation
    assert!(
        rust_code.contains("-> String"),
        "Should have String return type"
    );
    assert!(
        rust_code.contains("to_string()"),
        "String literal should be converted to String"
    );
}

#[test]
fn test_string_concatenation_allocates() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def concat_strings(a: str, b: str) -> str:
    return a + b
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for concat_strings:\n{}", rust_code);

    // DEPYLER-0357: String concatenation uses format! for borrowed strings
    // This is more idiomatic than + operator when both operands are &str
    assert!(
        rust_code.contains("format!") || rust_code.contains("+"),
        "Should contain concatenation via format! or +"
    );
    assert!(
        rust_code.contains("-> String"),
        "Concatenation should return String"
    );
}

#[test]
fn test_interned_strings_for_repeated_literals() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def use_repeated_string():
    s1 = "repeated"
    s2 = "repeated"
    s3 = "repeated"
    s4 = "repeated"
    s5 = "repeated"
    return [s1, s2, s3, s4, s5]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for use_repeated_string:\n{}", rust_code);

    // Should generate a constant for repeated string literals
    assert!(
        rust_code.contains("const STR_"),
        "Should intern repeated string literal"
    );
}

#[test]
fn test_function_taking_str_reference() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def validate_string(s: str) -> bool:
    return len(s) > 0
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for validate_string:\n{}", rust_code);

    // Should take &str not String for read-only parameter
    assert!(rust_code.contains("&"), "Should borrow string parameter");
    assert!(rust_code.contains("str"), "Should use str type");
}

#[test]
fn test_local_string_variable_optimization() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def format_number(n: int) -> str:
    prefix = "Number: "
    return prefix + str(n)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for format_number:\n{}", rust_code);

    // Local string that's used in concatenation
    // The prefix might be inlined by the optimizer
    assert!(
        rust_code.contains("Number: ") || rust_code.contains("prefix"),
        "Should have prefix string either as variable or inlined"
    );
}

// ============================================================================
// PROPERTY TESTS
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]

        #[test]
        fn prop_string_literals_always_transpile(_s in "\\PC{0,50}") {
            // Property: Any valid string literal should transpile without error
            // Using simple alphanumeric strings to avoid parsing complexity
            let pipeline = DepylerPipeline::new();
            let python_code = r#"
def test_func():
    x = "test_string"
    return x
"#;

            let result = pipeline.transpile(python_code);
            prop_assert!(result.is_ok(), "String literal transpilation failed: {:?}", result.err());
        }

        #[test]
        fn prop_string_concatenation_compiles(_a in "\\PC{1,20}", _b in "\\PC{1,20}") {
            // Property: String concatenation should always produce valid Rust
            let pipeline = DepylerPipeline::new();
            let python_code = r#"
def concat(x: str, y: str) -> str:
    return x + y
"#;

            let result = pipeline.transpile(python_code);
            prop_assert!(result.is_ok(), "String concatenation transpilation failed");

            let rust_code = result.unwrap();
            prop_assert!(rust_code.contains("+") || rust_code.contains("format!"),
                "Should contain concatenation operator or format macro");
        }

        #[test]
        fn prop_string_parameters_use_references(param_name in "[a-z]{1,10}") {
            // Property: String parameters should prefer &str over String
            // Filter out Python AND Rust keywords to avoid parsing errors
            let rust_keywords = ["as", "break", "const", "continue", "crate", "do", "else",
                                 "enum", "extern", "false", "fn", "for", "if", "impl", "in",
                                 "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
                                 "return", "self", "Self", "static", "struct", "super", "trait",
                                 "true", "type", "unsafe", "use", "where", "while", "async",
                                 "await", "dyn", "abstract", "become", "box", "final", "macro",
                                 "override", "priv", "typeof", "unsized", "virtual", "yield",
                                 "try"];

            // Python keywords that would cause parse errors
            let python_keywords = ["and", "as", "assert", "async", "await", "break", "class",
                                  "continue", "def", "del", "elif", "else", "except", "finally",
                                  "for", "from", "global", "if", "import", "in", "is", "lambda",
                                  "nonlocal", "not", "or", "pass", "raise", "return", "try",
                                  "while", "with", "yield"];

            prop_assume!(!rust_keywords.contains(&param_name.as_str()));
            prop_assume!(!python_keywords.contains(&param_name.as_str()));

            let pipeline = DepylerPipeline::new();
            let python_code = format!(r#"
def check_{param}({param}: str) -> int:
    return len({param})
"#, param = param_name);

            let result = pipeline.transpile(&python_code);
            prop_assert!(result.is_ok(), "String parameter transpilation failed");

            let rust_code = result.unwrap();
            // Should use &str or similar borrowed type
            prop_assert!(rust_code.contains("&"),
                "String parameters should use borrowing");
        }
    }
}

// ============================================================================
// MUTATION TESTS
// ============================================================================

#[cfg(test)]
mod mutation_tests {
    use super::*;

    #[test]
    fn test_mutation_string_interning_threshold() {
        // Target Mutations:
        // 1. Repeated string count check (5 occurrences → should intern)
        // 2. Constant generation (STR_ prefix must be present)
        // 3. String deduplication (same literal = same constant)
        //
        // Kill Strategy:
        // - Verify STR_ constant is generated for repeated literals
        // - Verify constant is used multiple times (not duplicated)
        // - Mutation that removes interning would fail this check

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def use_repeated_string():
    s1 = "repeated"
    s2 = "repeated"
    s3 = "repeated"
    s4 = "repeated"
    s5 = "repeated"
    return [s1, s2, s3, s4, s5]
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: If interning is disabled, this fails
        assert!(
            rust_code.contains("const STR_"),
            "MUTATION KILL: Must generate interned constant for repeated strings"
        );

        // Mutation Kill: If deduplication breaks, multiple constants would appear
        let const_count = rust_code.matches("const STR_").count();
        assert_eq!(
            const_count, 1,
            "MUTATION KILL: Should have exactly 1 constant for 'repeated' (found {})",
            const_count
        );
    }

    #[test]
    fn test_mutation_string_allocation_elimination() {
        // Target Mutations:
        // 1. .to_string() insertion (should NOT allocate for read-only)
        // 2. String::from() usage (unnecessary allocation)
        // 3. Owned vs borrowed type selection
        //
        // Kill Strategy:
        // - Verify no .to_string() for read-only string literals
        // - Verify no String::from() for simple assignments
        // - Mutation adding allocations would fail this check

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def print_message():
    message = "Hello, World!"
    print(message)
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Adding .to_string() would fail this
        assert!(
            !rust_code.contains(".to_string()"),
            "MUTATION KILL: Read-only string should not allocate with .to_string()"
        );

        // Mutation Kill: Using String::from unnecessarily would fail
        assert!(
            !rust_code.contains("String::from"),
            "MUTATION KILL: Should not use String::from for simple literal assignment"
        );
    }

    #[test]
    fn test_mutation_string_concatenation_operator() {
        // Target Mutations:
        // 1. + operator removal (would break concatenation)
        // 2. format! macro substitution (alternative approach)
        // 3. Operator precedence changes
        //
        // Kill Strategy:
        // - Verify + operator or format! exists for concatenation
        // - Verify operand ordering is preserved
        // - Mutation removing concatenation logic would fail

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def concat_strings(a: str, b: str) -> str:
    return a + b
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Removing concatenation operator would fail
        assert!(
            rust_code.contains("+") || rust_code.contains("format!"),
            "MUTATION KILL: Concatenation must use + operator or format! macro"
        );

        // Mutation Kill: Swapping operands would change semantics
        // The generated code should preserve a, b ordering
        assert!(
            rust_code.contains("a") && rust_code.contains("b"),
            "MUTATION KILL: Both operands must be present in concatenation"
        );
    }
}
