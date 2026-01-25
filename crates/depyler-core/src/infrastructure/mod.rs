//! Infrastructure-First Implementation (DEPYLER-0925)
//!
//! Core infrastructure components for systematic convergence to 80% compilation rate.
//! Implements Part E of the single-shot compilation strategy specification.
//!
//! ## Components
//!
//! 1. **FaultLocalizer** (Tarantula algorithm) - Identify suspicious codegen decisions
//! 2. **PatternStore** (HNSW search) - Store and retrieve successful transpilation patterns
//! 3. **CurriculumScheduler** - Process errors EASY→HARD for optimal convergence
//! 4. **KnowledgeDistiller** - Graduate high-confidence patterns to hardcoded rules
//!
//! ## Toyota Way Principles
//!
//! - Jidoka: Build quality in through systematic fault localization
//! - Kaizen: Continuous improvement via pattern accumulation
//! - Genchi Genbutsu: Direct observation through tracer infrastructure
//! - Poka-Yoke: Error-proofing through curriculum learning
//!
//! ## References
//!
//! - Jones & Harrold (2005): Tarantula fault localization
//! - Malkov & Yashunin (2020): HNSW approximate nearest neighbor search
//! - Bengio et al. (2009): Curriculum learning
//! - Hinton et al. (2015): Knowledge distillation

pub mod curriculum;
pub mod distiller;
pub mod fault_localizer;
pub mod pattern_store;

pub use curriculum::{CompilationError, CurriculumScheduler, DifficultyLevel, FailingExample};
pub use distiller::{GraduationCriteria, KnowledgeDistiller};
pub use fault_localizer::{DecisionType, FaultLocalizer, SourceLocation, TranspilerDecision};
pub use pattern_store::{PatternStore, TranspilationPattern};

#[cfg(test)]
mod tests;
