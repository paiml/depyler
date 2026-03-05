#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::doc_link_with_quotes)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::struct_excessive_bools)]

//! # Depyler HIR - High-level Intermediate Representation
//!
//! Foundation types for the Depyler Python-to-Rust transpiler.
//!
//! This crate provides the core type definitions used throughout the transpilation pipeline:
//! - [`hir`] - Full HIR types (`HirModule`, `HirFunction`, `HirStmt`, `HirExpr`, Type, etc.)
//! - [`simplified_hir`] - Simplified HIR for backend usage
//! - [`error`] - Transpilation error types
//! - [`decision_trace`] - Decision tracing for CITL training (feature-gated)

pub mod decision_trace;
pub mod error;
pub mod hir;
pub mod simplified_hir;
