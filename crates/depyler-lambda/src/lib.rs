#![allow(clippy::cast_precision_loss)]
#![allow(clippy::disallowed_methods)]
#![allow(clippy::doc_link_with_quotes)]
#![allow(clippy::format_push_string)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_self)]

//! # Depyler Lambda - AWS Lambda Transpilation Support
//!
//! Lambda-specific code generation, type inference, optimization, and testing
//! for the Depyler Python-to-Rust transpiler.

pub mod lambda_codegen;
pub mod lambda_errors;
pub mod lambda_inference;
pub mod lambda_optimizer;
pub mod lambda_testing;
pub mod lambda_types;
