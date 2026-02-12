//! # Depyler Analysis - Type Inference, Optimization, and Analysis Passes
//!
//! All analysis, type inference, borrowing analysis, and optimization passes
//! for the Depyler Python-to-Rust transpiler.

pub mod annotation_aware_type_mapper;
pub mod borrowing;
pub mod container_element_inference;
pub mod borrowing_context;
pub mod borrowing_shim;
pub mod const_generic_inference;
pub mod depylint;
pub mod error_reporting;
pub mod escape_analysis;
pub mod generator_state;
pub mod generator_yield_analysis;
pub mod generic_inference;
pub mod inlining;
pub mod lifetime_analysis;
pub mod migration_suggestions;
pub mod optimization;
pub mod optimizer;
pub mod param_type_inference;
pub mod performance_warnings;
pub mod profiling;
pub mod scoring;
pub mod string_optimization;
pub mod type_hints;
pub mod type_inference_telemetry;
pub mod type_mapper;
pub mod type_propagation;
pub mod type_system;
