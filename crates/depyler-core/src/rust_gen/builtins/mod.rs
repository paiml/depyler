//! Builtin function handlers
//!
//! This module contains handlers for Python's builtin functions organized by category.

pub mod math;

pub use math::{handle_abs, handle_max, handle_min, handle_pow, handle_round};
