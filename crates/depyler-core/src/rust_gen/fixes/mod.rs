//! Text-level fix functions for post-codegen Rust output correction.
//!
//! These functions operate on the generated Rust source code as text,
//! fixing patterns that the AST-level codegen cannot handle correctly.
//! Each fix targets a specific rustc error code (E0308, E0599, etc.).

mod collections;
mod depyler_value;
mod enums;
mod misc;
mod numeric;
mod options_results;
mod ownership;
mod strings;
mod truthiness;

pub(super) mod pipeline;
