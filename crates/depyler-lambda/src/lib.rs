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
