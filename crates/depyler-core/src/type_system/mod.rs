//! Type System - Constraint-based type inference with subtyping support
//!
//! This module implements a constraint-based type inference system using:
//! - **Subtyping constraints** (T1 <: T2) instead of pure unification
//! - **Bidirectional type checking** (synthesis ⇒ and checking ⇐)
//! - **SSA form** for flow-sensitive analysis
//! - **Worklist solver** for transitive closure
//!
//! # DEPYLER-0499: TypeEnvironment with Subtyping Constraints
//!
//! Replaces 7 fragmented HashMaps with unified TypeEnvironment.
//!
//! # Architecture
//!
//! - **TypeEnvironment**: Single source of truth (O(1) lookups)
//! - **SubtypeChecker**: Implements T1 <: T2 relation
//! - **WorklistSolver**: Constraint solving with transitive closure
//! - **TypeConstraint**: Supports equality and subtyping constraints
//!
//! # References
//!
//! - Damas-Milner type system (1982) - Hindley-Milner unification
//! - Dunfield & Krishnaswami (2013) - Bidirectional typechecking
//! - Cytron et al. (1991) - SSA form
//! - Pierce (2002) - Types and Programming Languages (Subtyping)

// DEPYLER-0499: New subtyping-aware modules
pub mod constraint;
pub mod subtyping;
pub mod solver;
pub mod type_environment;

// Legacy Hindley-Milner (kept for backward compatibility)
pub mod hindley_milner;

// Re-exports
pub use constraint::{TypeConstraint, ConstraintKind};
pub use subtyping::SubtypeChecker;
pub use solver::{WorklistSolver, Solution};
pub use type_environment::{TypeEnvironment, TypeInfo, VarId};

// Legacy exports
pub use hindley_milner::{Constraint, TypeConstraintSolver, TypeError};
