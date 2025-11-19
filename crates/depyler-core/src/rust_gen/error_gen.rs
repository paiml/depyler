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

    definitions
}

// Note: Unit tests for this module are covered by integration tests
// that exercise the full transpilation pipeline. The function is simple
// enough (complexity: 2) that dedicated unit tests add minimal value.
// Full test coverage is provided by:
// - integration_tests::test_division_by_zero
// - integration_tests::test_index_error
// - All tests that trigger error type generation
