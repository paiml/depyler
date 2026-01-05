//! Standard Library Method Code Generation
//!
//! This module contains extracted stdlib method handlers from expr_gen.rs.
//! Each Python stdlib module gets its own Rust module for testability.
//!
//! ## Architecture
//!
//! Each module exports a `convert_*_method` function that:
//! - Takes method name, arguments, and context
//! - Returns `Result<Option<syn::Expr>>`
//! - Has 100% test coverage via EXTREME TDD
//!
//! ## Modules
//!
//! - `builtin_functions` - Python builtin functions (all, any, zip, etc.)
//! - `functools` - Higher-order functions (reduce, etc.)
//! - `itertools` - Iterator combinatorics (itertools crate)
//! - `json` - JSON serialization (serde_json)
//! - `math` - Mathematical functions (f64 methods)
//! - `os` - Operating system interface (std::env, std::fs)
//! - `pathlib` - Path manipulation (std::path)
//! - `random` - Random number generation (rand crate)
//! - `regex_mod` - Regular expressions (regex crate)
//! - `shutil` - Shell utilities (std::fs)
//! - `string` - String utilities (capwords, etc.)
//! - `time` - Time measurement and manipulation (std::time, chrono)
//! - `warnings` - Warning control (eprintln!)

pub mod builtin_functions;
pub mod functools;
pub mod itertools;
pub mod json;
pub mod math;
pub mod os;
pub mod pathlib;
pub mod random;
pub mod regex_mod;
pub mod shutil;
pub mod string;
pub mod time;
pub mod warnings;

// Re-exports for convenience
pub use functools::convert_functools_method;
pub use itertools::convert_itertools_method;
pub use json::convert_json_method;
pub use math::convert_math_method;
pub use os::convert_os_method;
pub use pathlib::convert_pathlib_method;
pub use random::convert_random_method;
pub use regex_mod::convert_re_method;
pub use shutil::convert_shutil_method;
pub use string::convert_string_method;
pub use time::convert_time_method;
pub use warnings::convert_warnings_method;
