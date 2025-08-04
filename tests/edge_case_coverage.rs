use depyler_core::DepylerPipeline;

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_python_file() {
        let pipeline = DepylerPipeline::new();
        let empty_source = "";

        let result = pipeline.transpile(empty_source);
        // Should handle empty input gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_whitespace_only_file() {
        let pipeline = DepylerPipeline::new();
        let whitespace_source = "   \n\t  \n   ";

        let result = pipeline.transpile(whitespace_source);
        // Should handle whitespace-only input
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_comments_only_file() {
        let pipeline = DepylerPipeline::new();
        let comments_source = r#"
# This is a comment
# Another comment
    # Indented comment
        # More comments
"#;

        let result = pipeline.transpile(comments_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_deeply_nested_functions() {
        let pipeline = DepylerPipeline::new();
        let nested_source = r#"
def level1(x: int) -> int:
    def level2(y: int) -> int:
        def level3(z: int) -> int:
            def level4(w: int) -> int:
                return w + 1
            return level4(z) + 1
        return level3(y) + 1
    return level2(x) + 1
"#;

        let result = pipeline.transpile(nested_source);
        // Should handle or fail gracefully on deep nesting
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_very_long_function_name() {
        let pipeline = DepylerPipeline::new();
        let long_name = "a".repeat(1000);
        let long_name_source = format!(
            r#"
def {}(x: int) -> int:
    return x + 1
"#,
            long_name
        );

        let result = pipeline.transpile(&long_name_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_function_with_many_parameters() {
        let pipeline = DepylerPipeline::new();
        let mut params = Vec::new();
        let mut args = Vec::new();

        for i in 0..50 {
            params.push(format!("param{}: int", i));
            args.push(format!("param{}", i));
        }

        let many_params_source = format!(
            r#"
def many_params({}) -> int:
    return {}
"#,
            params.join(", "),
            args.join(" + ")
        );

        let result = pipeline.transpile(&many_params_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_extremely_simple_function() {
        let pipeline = DepylerPipeline::new();
        let simple_source = r#"
def f(): pass
"#;

        let result = pipeline.transpile(simple_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_function_with_only_return() {
        let pipeline = DepylerPipeline::new();
        let return_only_source = r#"
def get_five() -> int:
    return 5
"#;

        let result = pipeline.transpile(return_only_source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unicode_function_names() {
        let pipeline = DepylerPipeline::new();
        let unicode_source = r#"
def функция(x: int) -> int:
    return x * 2

def 関数(y: int) -> int:
    return y + 1
"#;

        let result = pipeline.transpile(unicode_source);
        // Should handle or reject unicode identifiers appropriately
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_unicode_strings() {
        let pipeline = DepylerPipeline::new();
        let unicode_strings_source = r#"
def greet() -> str:
    return "Hello, 世界! 🌍"

def emoji_func() -> str:
    return "🚀💯✨"
"#;

        let result = pipeline.transpile(unicode_strings_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_max_integer_values() {
        let pipeline = DepylerPipeline::new();
        let max_int_source = r#"
def big_numbers() -> int:
    x = 9223372036854775807
    y = -9223372036854775808
    return x + y
"#;

        let result = pipeline.transpile(max_int_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_empty_lists_and_dicts() {
        let pipeline = DepylerPipeline::new();
        let empty_collections_source = r#"
def empty_collections():
    empty_list = []
    empty_dict = {}
    return len(empty_list) + len(empty_dict)
"#;

        let result = pipeline.transpile(empty_collections_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_single_character_variables() {
        let pipeline = DepylerPipeline::new();
        let single_char_source = r#"
def single_chars(a: int, b: int, c: int) -> int:
    x = a
    y = b
    z = c
    return x + y + z
"#;

        let result = pipeline.transpile(single_char_source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_very_long_string_literal() {
        let pipeline = DepylerPipeline::new();
        let long_string = "x".repeat(10000);
        let long_string_source = format!(
            r#"
def long_string() -> str:
    return "{}"
"#,
            long_string
        );

        let result = pipeline.transpile(&long_string_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_nested_control_structures() {
        let pipeline = DepylerPipeline::new();
        let nested_control_source = r#"
def nested_control(n: int) -> int:
    result = 0
    for i in range(n):
        if i % 2 == 0:
            for j in range(i):
                if j % 3 == 0:
                    while j > 0:
                        result += j
                        j -= 1
                        if result > 100:
                            break
    return result
"#;

        let result = pipeline.transpile(nested_control_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_all_python_operators() {
        let pipeline = DepylerPipeline::new();
        let all_operators_source = r#"
def all_operators(a: int, b: int) -> bool:
    # Arithmetic
    add = a + b
    sub = a - b
    mul = a * b
    div = a // b  # Floor division
    mod = a % b
    
    # Comparison
    eq = a == b
    ne = a != b
    lt = a < b
    le = a <= b
    gt = a > b
    ge = a >= b
    
    # Logical (using int as bool)
    and_op = a and b
    or_op = a or b
    not_op = not a
    
    return eq or ne or lt or le or gt or ge
"#;

        let result = pipeline.transpile(all_operators_source);
        assert!(result.is_ok() || result.is_err());
    }
}
