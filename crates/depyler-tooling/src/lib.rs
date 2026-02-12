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
