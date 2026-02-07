//! Instance method handlers for ExpressionConverter
//!
//! DEPYLER-COVERAGE-95: Extracted from expr_gen.rs to reduce file size
//! and improve testability. Contains collection and instance method handlers.

mod attribute_convert;
mod comprehensions;
mod constructors;
mod dict_constructors;
mod dict_methods;
mod indexing;
mod instance_dispatch;
mod lambda_generators;
mod list_methods;
mod method_call_routing;
mod regex_methods;
mod set_methods;
mod slicing;
mod string_methods;
mod sys_io_methods;
mod type_helpers;

// Re-export for test compatibility
#[cfg(test)]
pub(crate) use crate::rust_gen::expr_gen::ExpressionConverter;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;
