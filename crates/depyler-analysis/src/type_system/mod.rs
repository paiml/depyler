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
//! # DEPYLER-0202: Constraint Collection and HM Solver Integration
//!
//! The ConstraintCollector walks HIR and generates type equations for the
//! Hindley-Milner solver, enabling automatic type inference for unannotated
//! parameters and return types.
//!
//! # Architecture
//!
//! - **TypeEnvironment**: Single source of truth (O(1) lookups)
//! - **SubtypeChecker**: Implements T1 <: T2 relation
//! - **WorklistSolver**: Constraint solving with transitive closure
//! - **TypeConstraint**: Supports equality and subtyping constraints
//! - **ConstraintCollector**: HIR→Constraints bridge (DEPYLER-0202)
//!
//! # References
//!
//! - Damas-Milner type system (1982) - Hindley-Milner unification
//! - Dunfield & Krishnaswami (2013) - Bidirectional typechecking
//! - Cytron et al. (1991) - SSA form
//! - Pierce (2002) - Types and Programming Languages (Subtyping)

// DEPYLER-0499: New subtyping-aware modules
pub mod constraint;
pub mod solver;
pub mod subtyping;
pub mod type_environment;

// DEPYLER-0202: Constraint collection for HM solver integration
pub mod constraint_collector;

// DEPYLER-0950: Inter-procedural type unification
pub mod type_unify;

// Legacy Hindley-Milner (kept for backward compatibility)
pub mod hindley_milner;

// Re-exports
pub use constraint::{ConstraintKind, TypeConstraint};
pub use constraint_collector::ConstraintCollector;
pub use solver::{Solution, WorklistSolver};
pub use subtyping::SubtypeChecker;
pub use type_environment::{TypeEnvironment, TypeInfo, VarId};

// Legacy exports
pub use hindley_milner::{Constraint, TypeConstraintSolver, TypeError};

// DEPYLER-0950 exports
pub use type_unify::{unify_module_types, CallGraph, TypeUnifier};
