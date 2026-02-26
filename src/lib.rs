//! Depyler workspace root library
//!
//! This crate provides formal verification specifications and design-by-contract
//! invariants for the Depyler Python-to-Rust transpiler.
//!
//! For the main transpiler library, see the `depyler-core` crate.
//! For the CLI tool, see the `depyler` crate.

// Verus-style specification comments use function call syntax in doc comments,
// which triggers the doc_markdown lint. These are formal specs, not regular docs.
#[allow(clippy::doc_markdown)]
pub mod verification_specs;
