//! Type System - Hindley-Milner type inference for Python→Rust transpilation
//!
//! This module implements a constraint-based type inference system using the
//! Hindley-Milner Algorithm W. This provides systematic, provably correct type
//! inference to replace ad-hoc pattern matching.
//!
//! # Architecture
//!
//! The type inference system consists of:
//! - **Type representation**: Extended with unification variables
//! - **Constraint generation**: Collect type equality constraints from HIR
//! - **Unification**: Robinson's algorithm with occurs check
//! - **Substitution**: Apply solved types to the program
//!
//! # References
//!
//! - Damas-Milner type system (1982)
//! - Algorithm W: Principal type-schemes for functional programs
//! - Robinson's unification algorithm (1965)

pub mod hindley_milner;

pub use hindley_milner::{Constraint, TypeConstraintSolver, TypeError};
