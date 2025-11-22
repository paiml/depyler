/// Depyler Testing Framework
///
/// Differential testing and validation utilities for the Depyler transpiler.
/// Implements McKeeman (1998) "Differential Testing for Software" methodology.
///
/// # Modules
///
/// - `differential`: Differential testing harness comparing Python vs Rust output

pub mod differential;

// Re-export main types for convenience
pub use differential::{
    DifferentialTester,
    DifferentialTestResult,
    Mismatch,
    ProgramOutput,
    ReprorustedTestSuite,
};
