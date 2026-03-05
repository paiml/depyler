#![allow(clippy::assigning_clones)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::disallowed_methods)]
#![allow(clippy::doc_link_with_quotes)]
#![allow(clippy::filter_map_next)]
#![allow(clippy::format_push_string)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_continue)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::ref_option)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::self_only_used_in_recursion)]
#![allow(clippy::similar_names)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_self)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::used_underscore_binding)]

//! # Depyler Analysis - Type Inference, Optimization, and Analysis Passes
//!
//! All analysis, type inference, borrowing analysis, and optimization passes
//! for the Depyler Python-to-Rust transpiler.

pub mod annotation_aware_type_mapper;
pub mod borrowing;
pub mod borrowing_context;
pub mod borrowing_shim;
pub mod const_generic_inference;
pub mod container_element_inference;
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
