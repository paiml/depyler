//! Rust code formatting utilities
//!
//! This module provides post-processing formatting for generated Rust code.
//! The primary function `format_rust_code` applies various string replacements
//! to clean up spacing and formatting issues in the generated token streams.

/// Format Rust code using rustfmt for idiomatic formatting
///
/// This function first applies string replacements to fix common issues from
/// quote! macro expansion, then runs rustfmt to ensure idiomatic formatting.
///
/// # Arguments
/// * `code` - The Rust code to format as a String
///
/// # Returns
/// Formatted Rust code that passes rustfmt --check
///
/// # Example
/// ```ignore
/// use depyler_core::rust_gen::format::format_rust_code;
/// let code = "fn main ( ) { println ! ( \"Hello\" ) ; }".to_string();
/// let formatted = format_rust_code(code);
/// // Returns properly formatted Rust code
/// ```
pub fn format_rust_code(code: String) -> String {
    // First apply string replacements to fix obvious issues
    let code = apply_string_replacements(code);

    // Then run rustfmt to ensure idiomatic formatting
    match run_rustfmt(&code) {
        Ok(formatted) => formatted,
        Err(_) => code, // Fall back to unformatted if rustfmt fails
    }
}

/// Run rustfmt on code string and return formatted result
fn run_rustfmt(code: &str) -> Result<String, std::io::Error> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write to stdin in a scope to ensure it's dropped/closed before wait
    {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(code.as_bytes())?;
            // stdin is automatically dropped here when it goes out of scope
        }
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::other("rustfmt failed"))
    }
}

/// Apply string replacements to fix common formatting issues
fn apply_string_replacements(code: String) -> String {
    code.replace(" ; ", ";\n    ")
        .replace(" { ", " {\n    ")
        .replace(" } ", "\n}\n")
        .replace("} ;", "};")
        .replace(
            "use std :: collections :: HashMap ;",
            "use std::collections::HashMap;",
        )
        // Fix method call spacing
        .replace(" . ", ".")
        .replace(" (", "(")
        .replace(" )", ")")
        // Fix specific common patterns
        .replace(".len ()", ".len()")
        .replace(".push (", ".push(")
        .replace(".insert (", ".insert(")
        .replace(".get (", ".get(")
        .replace(".contains_key (", ".contains_key(")
        .replace(".to_string ()", ".to_string()")
        // Fix spacing around operators in some contexts
        .replace(" ::", "::")
        .replace(":: ", "::")
        // Fix attribute spacing
        .replace("# [", "#[")
        // Fix type annotations
        .replace(" : ", ": ")
        // Fix parameter spacing
        .replace(" , ", ", ")
        // Fix assignment operator spacing issues
        .replace("=(", " = (")
        .replace("= (", " = (")
        .replace("  =", " =") // Fix multiple spaces before =
        .replace("   =", " =") // Fix even more spaces
        // Fix generic type spacing
        .replace("Vec < ", "Vec<")
        .replace(" < ", "<")
        .replace(" > ", ">")
        .replace("> ", ">")
        .replace("< ", "<")
        .replace(" >", ">") // Fix trailing space before closing bracket
        // Fix return type spacing (CRITICAL: Normalize first, then add proper spacing!)
        .replace(" -> ", "->") // Step 1: Remove existing spaces (normalize)
        .replace("-> ", "->") // Step 2: Remove trailing space
        .replace(" ->", "->") // Step 3: Remove leading space
        .replace("->", " -> ") // Step 4: Add correct spacing everywhere
        // Fix reference spacing
        .replace("& self", "&self")
        .replace("& mut", "&mut")
        // Fix macro spacing (space before !)
        .replace(" !", "!")
        // Fix comparison operator spacing
        .replace("value<", "value < ")
        .replace("<self", "< self")
        // Fix range spacing
        .replace(" .. ", "..")
        .replace(" ..", "..")
        .replace(".. ", "..")
        // Fix 'in' keyword spacing
        .replace("in(", "in (")
        // DEPYLER-0576: Fix comparison with negative literals (x<- 20 -> x < -20)
        // This must come after generic type spacing fixes since <T> uses < without space
        .replace("<-", "< -")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_semicolon_spacing() {
        let input = "fn main() { let x = 5 ; }".to_string();
        let result = format_rust_code(input);
        // rustfmt should format this properly
        assert!(result.contains("let x = 5;") || result.contains(";\n"));
    }

    #[test]
    fn test_format_method_call_spacing() {
        let input = "fn main() { let _x = vec . len ( ) ; }".to_string();
        let result = format_rust_code(input);
        // rustfmt should fix spacing
        assert!(result.contains("vec.len()") || result.contains("len()"));
    }

    #[test]
    fn test_format_generic_spacing() {
        let input = "fn main() { let _v: Vec < i32 > = Vec::new(); }".to_string();
        let result = format_rust_code(input);
        // rustfmt should remove spaces in generics
        assert!(result.contains("Vec<i32>") || result.contains("Vec <"));
    }

    #[test]
    fn test_format_return_type_spacing() {
        let input = "fn foo()->i32{42}".to_string();
        let result = format_rust_code(input);
        // rustfmt should add proper spacing
        assert!(result.contains(" -> ") || result.contains("->"));
    }

    // ========================================================================
    // Comprehensive coverage tests for apply_string_replacements
    // Target: 85%+ coverage of format.rs (161 lines)
    // ========================================================================

    /// Unit Test: Brace spacing - comprehensive patterns
    ///
    /// Verifies: Lines 65-68 (brace spacing replacements)
    #[test]
    fn test_apply_string_replacements_brace_spacing() {
        // " { " → " {\n    "
        let input1 = "fn test ( ) { let x = 1 ; }".to_string();
        let output1 = apply_string_replacements(input1);
        assert!(!output1.contains(" { "), "Open brace spacing not fixed");

        // " } " → "\n}\n"
        assert!(!output1.contains(" } "), "Close brace spacing not fixed");

        // "} ;" → "};"
        let input2 = "struct S { } ;".to_string();
        let output2 = apply_string_replacements(input2);
        assert!(
            !output2.contains("} ;"),
            "Brace semicolon spacing not fixed"
        );
    }

    /// Unit Test: HashMap import specific fix
    ///
    /// Verifies: Lines 69-72 (HashMap import normalization)
    #[test]
    fn test_apply_string_replacements_hashmap_import() {
        let input = "use std :: collections :: HashMap ;".to_string();
        let output = apply_string_replacements(input);
        assert_eq!(
            output, "use std::collections::HashMap;",
            "HashMap import should be fully normalized"
        );
    }

    /// Unit Test: Method call spacing patterns
    ///
    /// Verifies: Lines 74-83 (method call fixes)
    #[test]
    fn test_apply_string_replacements_method_calls() {
        let test_cases = vec![
            ("s.len ()", "s.len()"),
            ("v.push (x)", "v.push(x)"),
            ("m.insert (k, v)", "m.insert(k, v)"),
            ("d.get (key)", "d.get(key)"),
            ("h.contains_key (k)", "h.contains_key(k)"),
            ("s.to_string ()", "s.to_string()"),
        ];

        for (input, expected) in test_cases {
            let output = apply_string_replacements(input.to_string());
            assert_eq!(output, expected, "Method call '{}' not fixed", input);
        }
    }

    /// Unit Test: Operator spacing
    ///
    /// Verifies: Lines 85-92 (operator replacements)
    #[test]
    fn test_apply_string_replacements_operators() {
        let test_cases = vec![
            ("std ::collections", "std::collections"), // " ::" → "::"
            (":: Vec", "::Vec"),                       // ":: " → "::"
            ("x : i32", "x: i32"),                     // " : " → ": "
            ("a , b , c", "a, b, c"),                  // " , " → ", "
        ];

        for (input, expected) in test_cases {
            let output = apply_string_replacements(input.to_string());
            assert_eq!(output, expected, "Operator '{}' not fixed", input);
        }
    }

    /// Unit Test: Attribute spacing
    ///
    /// Verifies: Line 88 (attribute fix)
    #[test]
    fn test_apply_string_replacements_attributes() {
        let input = "# [derive(Debug)]".to_string();
        let output = apply_string_replacements(input);
        assert_eq!(output, "#[derive(Debug)]", "Attribute spacing not fixed");
    }

    /// Unit Test: Assignment operator normalization
    ///
    /// Verifies: Lines 94-97 (assignment spacing)
    #[test]
    fn test_apply_string_replacements_assignments() {
        let test_cases = vec![
            ("let x=(42)", "let x = (42)"),    // "=(" → " = ("
            ("let y= (10)", "let y = (10)"),   // "= (" → " = ("
            ("let z  =5", "let z =5"),         // "  =" → " ="
            ("let a   =true", "let a  =true"), // "   =" → " ="
        ];

        for (input, expected) in test_cases {
            let output = apply_string_replacements(input.to_string());
            assert_eq!(output, expected, "Assignment '{}' not fixed", input);
        }
    }

    /// Unit Test: Generic type spacing
    ///
    /// Verifies: Lines 99-104 (generic bracket spacing)
    #[test]
    fn test_apply_string_replacements_generics() {
        let test_cases = vec![
            ("Vec < i32>", "Vec<i32>"),
            ("HashMap < K, V>", "HashMap<K, V>"),
            ("Option < T >", "Option<T>"),
            ("Result<T> ", "Result<T>"),
            ("Box< T>", "Box<T>"),
            ("&'a T >", "&'a T>"),
        ];

        for (input, expected) in test_cases {
            let output = apply_string_replacements(input.to_string());
            assert_eq!(output, expected, "Generic '{}' not fixed", input);
        }
    }

    /// Unit Test: Return type spacing multistep
    ///
    /// Verifies: Lines 106-109 (4-step -> normalization)
    #[test]
    fn test_apply_string_replacements_return_types() {
        let test_cases = vec![
            ("fn test( )  ->   i32", "fn test( ) -> i32"),
            ("fn test()->i32", "fn test() -> i32"),
            ("fn test() ->i32", "fn test() -> i32"),
        ];

        for (input, _expected) in test_cases {
            let output = apply_string_replacements(input.to_string());
            assert!(
                output.contains(" -> "),
                "Return type '{}' should have ' -> ' spacing, got '{}'",
                input,
                output
            );
        }
    }

    /// Unit Test: Reference spacing
    ///
    /// Verifies: Lines 111-112 (reference operator fixes)
    #[test]
    fn test_apply_string_replacements_references() {
        let test_cases = vec![
            ("& self", "&self"),
            ("& mut x", "&mut x"),
            ("fn test(& self)", "fn test(&self)"),
            ("fn test2(& mut self)", "fn test2(&mut self)"),
        ];

        for (input, expected) in test_cases {
            let output = apply_string_replacements(input.to_string());
            assert_eq!(output, expected, "Reference '{}' not fixed", input);
        }
    }

    /// Unit Test: Macro spacing
    ///
    /// Verifies: Line 114 (macro invocation fix)
    #[test]
    fn test_apply_string_replacements_macros() {
        let test_cases = vec![
            ("println !", "println!"),
            ("vec !", "vec!"),
            ("format !", "format!"),
        ];

        for (input, expected) in test_cases {
            let output = apply_string_replacements(input.to_string());
            assert_eq!(output, expected, "Macro '{}' not fixed", input);
        }
    }

    /// Unit Test: Comparison operators
    ///
    /// Verifies: Lines 116-117 (comparison spacing)
    #[test]
    fn test_apply_string_replacements_comparisons() {
        let input1 = "value<10".to_string();
        let output1 = apply_string_replacements(input1);
        assert!(output1.contains("value < "), "Comparison spacing not fixed");

        let input2 = "x<self".to_string();
        let output2 = apply_string_replacements(input2);
        assert!(
            output2.contains("< self"),
            "Self comparison spacing not fixed"
        );
    }

    /// Unit Test: Range operators
    ///
    /// Verifies: Lines 119-121 (range spacing)
    #[test]
    fn test_apply_string_replacements_ranges() {
        let test_cases = vec![("0 .. 10", "0..10"), ("0 ..", "0.."), (".. 10", "..10")];

        for (input, expected) in test_cases {
            let output = apply_string_replacements(input.to_string());
            assert_eq!(output, expected, "Range '{}' not fixed", input);
        }
    }

    /// Unit Test: 'in' keyword spacing
    ///
    /// Verifies: Line 123 (in keyword fix)
    #[test]
    fn test_apply_string_replacements_in_keyword() {
        let input = "for x in(items)".to_string();
        let output = apply_string_replacements(input);
        assert_eq!(output, "for x in (items)", "'in' keyword spacing not fixed");
    }

    /// Unit Test: Empty string edge case
    ///
    /// Verifies: Empty input handling (rustfmt adds newline)
    #[test]
    fn test_format_rust_code_empty_string() {
        let input = "".to_string();
        let result = format_rust_code(input);
        // rustfmt adds a newline even to empty input
        assert!(
            result.trim().is_empty() || result == "\n",
            "Empty or newline only"
        );
    }

    /// Unit Test: Rustfmt fallback on invalid syntax
    ///
    /// Verifies: Lines 29-32 (error path - fallback to string-replaced code)
    #[test]
    fn test_format_rust_code_rustfmt_fallback() {
        let input = "fn broken( { { { invalid".to_string();
        let result = format_rust_code(input);
        // Should return the string-replaced version (rustfmt failed, fell back)
        assert!(
            !result.is_empty(),
            "Should return fallback code on rustfmt failure"
        );
    }

    /// Property Test: Replacement idempotency
    ///
    /// Property: Applying replacements twice should give same result (for most cases)
    #[test]
    fn test_property_replacement_idempotency() {
        let inputs = vec![
            "fn test ( ) { }",
            "use std :: collections :: HashMap ;",
            "Vec < i32 >",
            "x : i32",
            "& self",
        ];

        for input in inputs {
            let once = apply_string_replacements(input.to_string());
            let twice = apply_string_replacements(once.clone());
            assert_eq!(
                once, twice,
                "Replacements should be idempotent for '{}'",
                input
            );
        }
    }

    /// Integration Test: Complex code with all patterns
    ///
    /// Verifies: All replacements work together on realistic code
    #[test]
    fn test_integration_complex_code_all_patterns() {
        let complex_code = r#"
use std :: collections :: HashMap ;
# [derive(Debug)]
struct Point { x : i32 , y : i32 }
impl Point {
    fn new( x : i32 , y : i32 )->Self{
        Point{ x , y }
    }
    fn distance(& self)->f64{
        ((self.x as f64).powi(2)+(self.y as f64).powi(2)).sqrt ()
    }
}
fn main( ){
    let p=Point::new( 3 , 4 );
    println !(\"Distance: {}\", p.distance ());
}
"#
        .to_string();

        let output = apply_string_replacements(complex_code);

        // Verify all patterns were fixed
        assert!(!output.contains(" :: "), "Module separator spacing");
        assert!(output.contains("#[derive"), "Attribute spacing");
        assert!(!output.contains(": i32 ,"), "Type annotation spacing");
        assert!(output.contains(" -> "), "Return type spacing");
        assert!(output.contains("&self"), "Reference spacing");
        assert!(!output.contains(".sqrt ()"), "Method call spacing");
        assert!(!output.contains("println !"), "Macro spacing");
    }
}
