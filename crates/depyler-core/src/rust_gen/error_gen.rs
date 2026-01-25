//! Error type generation
//!
//! This module generates Rust struct definitions for Python error types
//! like `ZeroDivisionError` and `IndexError`.

use crate::rust_gen::CodeGenContext;
use quote::quote;

/// Generate error type definitions if needed
///
/// Generates struct definitions for Python error types like ZeroDivisionError and IndexError.
/// These types implement the standard Error trait and provide appropriate Display formatting.
///
/// # Arguments
/// * `ctx` - Code generation context containing flags for which error types are needed
///
/// # Returns
/// A vector of `TokenStream`s containing the error type definitions
///
/// # Example
/// ```
/// // If ctx.needs_zerodivisionerror is true, generates:
/// // #[derive(Debug, Clone)]
/// // pub struct ZeroDivisionError { message: String }
/// // impl std::fmt::Display for ZeroDivisionError { ... }
/// // impl std::error::Error for ZeroDivisionError {}
/// ```
pub fn generate_error_type_definitions(ctx: &CodeGenContext) -> Vec<proc_macro2::TokenStream> {
    let mut definitions = Vec::new();

    if ctx.needs_zerodivisionerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct ZeroDivisionError {
                message: String,
            }

            impl std::fmt::Display for ZeroDivisionError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "division by zero: {}", self.message)
                }
            }

            impl std::error::Error for ZeroDivisionError {}

            impl ZeroDivisionError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    if ctx.needs_indexerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct IndexError {
                message: String,
            }

            impl std::fmt::Display for IndexError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "index out of range: {}", self.message)
                }
            }

            impl std::error::Error for IndexError {}

            impl IndexError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    if ctx.needs_valueerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct ValueError {
                message: String,
            }

            impl std::fmt::Display for ValueError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "value error: {}", self.message)
                }
            }

            impl std::error::Error for ValueError {}

            impl ValueError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    if ctx.needs_argumenttypeerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct ArgumentTypeError {
                message: String,
            }

            impl std::fmt::Display for ArgumentTypeError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "argument type error: {}", self.message)
                }
            }

            impl std::error::Error for ArgumentTypeError {}

            impl ArgumentTypeError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    // DEPYLER-0551: RuntimeError for generic runtime failures
    if ctx.needs_runtimeerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct RuntimeError {
                message: String,
            }

            impl std::fmt::Display for RuntimeError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "runtime error: {}", self.message)
                }
            }

            impl std::error::Error for RuntimeError {}

            impl RuntimeError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    // DEPYLER-0551: FileNotFoundError for file system errors
    if ctx.needs_filenotfounderror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct FileNotFoundError {
                message: String,
            }

            impl std::fmt::Display for FileNotFoundError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "file not found: {}", self.message)
                }
            }

            impl std::error::Error for FileNotFoundError {}

            impl FileNotFoundError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    // GH-204: SyntaxError for parsing/syntax failures
    if ctx.needs_syntaxerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct SyntaxError {
                message: String,
            }

            impl std::fmt::Display for SyntaxError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "syntax error: {}", self.message)
                }
            }

            impl std::error::Error for SyntaxError {}

            impl SyntaxError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    // GH-204: TypeError for type mismatch errors
    if ctx.needs_typeerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct TypeError {
                message: String,
            }

            impl std::fmt::Display for TypeError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "type error: {}", self.message)
                }
            }

            impl std::error::Error for TypeError {}

            impl TypeError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    // GH-204: KeyError for dictionary key lookup failures
    if ctx.needs_keyerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct KeyError {
                message: String,
            }

            impl std::fmt::Display for KeyError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "key error: {}", self.message)
                }
            }

            impl std::error::Error for KeyError {}

            impl KeyError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    // GH-204: IOError for I/O operation failures
    if ctx.needs_ioerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct IOError {
                message: String,
            }

            impl std::fmt::Display for IOError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "io error: {}", self.message)
                }
            }

            impl std::error::Error for IOError {}

            impl IOError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    // GH-204: AttributeError for missing attribute access
    if ctx.needs_attributeerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct AttributeError {
                message: String,
            }

            impl std::fmt::Display for AttributeError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "attribute error: {}", self.message)
                }
            }

            impl std::error::Error for AttributeError {}

            impl AttributeError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    // GH-204: StopIteration for iterator exhaustion (used in generators)
    if ctx.needs_stopiteration {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct StopIteration {
                message: String,
            }

            impl std::fmt::Display for StopIteration {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "stop iteration: {}", self.message)
                }
            }

            impl std::error::Error for StopIteration {}

            impl StopIteration {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    definitions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust_gen::CodeGenContext;

    fn context_with_flags(flags: &[&str]) -> CodeGenContext<'static> {
        let mut ctx = CodeGenContext::default();
        for flag in flags {
            match *flag {
                "zerodivisionerror" => ctx.needs_zerodivisionerror = true,
                "indexerror" => ctx.needs_indexerror = true,
                "valueerror" => ctx.needs_valueerror = true,
                "argumenttypeerror" => ctx.needs_argumenttypeerror = true,
                "runtimeerror" => ctx.needs_runtimeerror = true,
                "filenotfounderror" => ctx.needs_filenotfounderror = true,
                "syntaxerror" => ctx.needs_syntaxerror = true,
                "typeerror" => ctx.needs_typeerror = true,
                "keyerror" => ctx.needs_keyerror = true,
                "ioerror" => ctx.needs_ioerror = true,
                "attributeerror" => ctx.needs_attributeerror = true,
                "stopiteration" => ctx.needs_stopiteration = true,
                _ => {}
            }
        }
        ctx
    }

    #[test]
    fn test_empty_context_generates_nothing() {
        let ctx = CodeGenContext::default();
        let defs = generate_error_type_definitions(&ctx);
        assert!(defs.is_empty());
    }

    #[test]
    fn test_zerodivisionerror_generation() {
        let ctx = context_with_flags(&["zerodivisionerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("ZeroDivisionError"));
        assert!(code.contains("division by zero"));
        assert!(code.contains("impl std :: fmt :: Display"));
        assert!(code.contains("impl std :: error :: Error"));
    }

    #[test]
    fn test_indexerror_generation() {
        let ctx = context_with_flags(&["indexerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("IndexError"));
        assert!(code.contains("index out of range"));
    }

    #[test]
    fn test_valueerror_generation() {
        let ctx = context_with_flags(&["valueerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("ValueError"));
        assert!(code.contains("value error"));
    }

    #[test]
    fn test_argumenttypeerror_generation() {
        let ctx = context_with_flags(&["argumenttypeerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("ArgumentTypeError"));
        assert!(code.contains("argument type error"));
    }

    #[test]
    fn test_runtimeerror_generation() {
        let ctx = context_with_flags(&["runtimeerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("RuntimeError"));
        assert!(code.contains("runtime error"));
    }

    #[test]
    fn test_filenotfounderror_generation() {
        let ctx = context_with_flags(&["filenotfounderror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("FileNotFoundError"));
        assert!(code.contains("file not found"));
    }

    #[test]
    fn test_syntaxerror_generation() {
        let ctx = context_with_flags(&["syntaxerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("SyntaxError"));
        assert!(code.contains("syntax error"));
    }

    #[test]
    fn test_typeerror_generation() {
        let ctx = context_with_flags(&["typeerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("TypeError"));
        assert!(code.contains("type error"));
    }

    #[test]
    fn test_keyerror_generation() {
        let ctx = context_with_flags(&["keyerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("KeyError"));
        assert!(code.contains("key error"));
    }

    #[test]
    fn test_ioerror_generation() {
        let ctx = context_with_flags(&["ioerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("IOError"));
        assert!(code.contains("io error"));
    }

    #[test]
    fn test_attributeerror_generation() {
        let ctx = context_with_flags(&["attributeerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("AttributeError"));
        assert!(code.contains("attribute error"));
    }

    #[test]
    fn test_stopiteration_generation() {
        let ctx = context_with_flags(&["stopiteration"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 1);
        let code = defs[0].to_string();
        assert!(code.contains("StopIteration"));
        assert!(code.contains("stop iteration"));
    }

    #[test]
    fn test_multiple_errors_generation() {
        let ctx =
            context_with_flags(&["zerodivisionerror", "indexerror", "valueerror", "keyerror"]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 4);
    }

    #[test]
    fn test_all_errors_generation() {
        let ctx = context_with_flags(&[
            "zerodivisionerror",
            "indexerror",
            "valueerror",
            "argumenttypeerror",
            "runtimeerror",
            "filenotfounderror",
            "syntaxerror",
            "typeerror",
            "keyerror",
            "ioerror",
            "attributeerror",
            "stopiteration",
        ]);
        let defs = generate_error_type_definitions(&ctx);
        assert_eq!(defs.len(), 12);
    }

    #[test]
    fn test_error_struct_has_new_method() {
        let ctx = context_with_flags(&["valueerror"]);
        let defs = generate_error_type_definitions(&ctx);
        let code = defs[0].to_string();
        assert!(code.contains("fn new"));
        assert!(code.contains("impl Into < String >"));
    }

    #[test]
    fn test_error_derives() {
        let ctx = context_with_flags(&["keyerror"]);
        let defs = generate_error_type_definitions(&ctx);
        let code = defs[0].to_string();
        assert!(code.contains("derive"));
        assert!(code.contains("Debug"));
        assert!(code.contains("Clone"));
    }

    #[test]
    fn test_error_has_message_field() {
        let ctx = context_with_flags(&["runtimeerror"]);
        let defs = generate_error_type_definitions(&ctx);
        let code = defs[0].to_string();
        assert!(code.contains("message"));
        assert!(code.contains("String"));
    }

    #[test]
    fn test_order_independence() {
        // Test that flags order doesn't affect which errors are generated
        let ctx1 = context_with_flags(&["zerodivisionerror", "keyerror"]);
        let ctx2 = context_with_flags(&["keyerror", "zerodivisionerror"]);
        let defs1 = generate_error_type_definitions(&ctx1);
        let defs2 = generate_error_type_definitions(&ctx2);
        assert_eq!(defs1.len(), defs2.len());
    }
}
