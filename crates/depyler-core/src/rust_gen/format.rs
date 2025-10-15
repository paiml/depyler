//! Rust code formatting utilities
//!
//! This module provides post-processing formatting for generated Rust code.
//! The primary function `format_rust_code` applies various string replacements
//! to clean up spacing and formatting issues in the generated token streams.

/// Format Rust code by cleaning up spacing and common formatting issues
///
/// This function applies a series of string replacements to fix common
/// formatting issues that arise from quote! macro expansion:
/// - Statement and block spacing
/// - Method call spacing
/// - Operator spacing
/// - Type annotation spacing
/// - Generic type bracket spacing
///
/// # Arguments
/// * `code` - The Rust code to format as a String
///
/// # Returns
/// Formatted Rust code with improved spacing
///
/// # Example
/// ```
/// let code = "fn main ( ) { println ! ( \"Hello\" ) ; }".to_string();
/// let formatted = format_rust_code(code);
/// // Returns properly spaced Rust code
/// ```
pub fn format_rust_code(code: String) -> String {
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
        // Fix return type spacing
        .replace("->", " -> ")
        .replace(" ->  ", " -> ")
        .replace(" ->   ", " -> ")
        // Fix range spacing
        .replace(" .. ", "..")
        .replace(" ..", "..")
        .replace(".. ", "..")
        // Fix 'in' keyword spacing
        .replace("in(", "in (")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_semicolon_spacing() {
        let input = "let x = 5 ; ".to_string();
        let result = format_rust_code(input);
        assert!(result.contains(";\n"));
    }

    #[test]
    fn test_format_method_call_spacing() {
        let input = "vec . len ( )".to_string();
        let result = format_rust_code(input);
        assert!(result.contains("vec.len()"));
    }

    #[test]
    fn test_format_generic_spacing() {
        let input = "Vec < i32 >".to_string();
        let result = format_rust_code(input);
        assert!(result.contains("Vec<i32>"));
    }

    #[test]
    fn test_format_return_type_spacing() {
        let input = "fn foo()->i32".to_string();
        let result = format_rust_code(input);
        assert!(result.contains(" -> "));
    }
}
