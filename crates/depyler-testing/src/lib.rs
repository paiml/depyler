#![allow(clippy::cast_precision_loss)]
#![allow(clippy::disallowed_methods)]
#![allow(clippy::format_push_string)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::unnecessary_debug_formatting)]
#![allow(clippy::unused_self)]
#![allow(clippy::unwrap_used)]

/// Depyler Testing Framework
///
/// Differential testing and validation utilities for the Depyler transpiler.
/// Implements `McKeeman` (1998) "Differential Testing for Software" methodology.
///
/// # Modules
///
/// - `differential`: Differential testing harness comparing Python vs Rust output
pub mod differential;

// Re-export main types for convenience
pub use differential::{
    DifferentialTestResult, DifferentialTester, Mismatch, ProgramOutput, ReprorustedTestSuite,
};
