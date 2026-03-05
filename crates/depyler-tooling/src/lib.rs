#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::disallowed_methods)]
#![allow(clippy::format_push_string)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::ref_option)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::self_only_used_in_recursion)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_self)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::used_underscore_binding)]

//! # Depyler Tooling - IDE, Debugging, and Infrastructure Support
//!
//! IDE integration, debugging, documentation generation, test generation,
//! module mapping, and infrastructure support for the Depyler transpiler.

pub mod chaos;
pub mod codegen_shim;
pub mod debug;
pub mod doctest_extractor;
pub mod documentation;
pub mod generative_repair;
pub mod hunt_mode;
pub mod ide;
pub mod infrastructure;
pub mod library_mapping;
pub mod module_mapper;
pub mod module_mapper_phf;
pub mod pytest_extractor;
pub mod stdlib_mappings;
pub mod test_generation;
pub mod typeshed_ingest;
