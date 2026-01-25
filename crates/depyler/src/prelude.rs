//! Prelude module for depyler transpiled code.
//!
//! This module re-exports all traits and types that transpiled Rust code
//! may need. Generated code should include `use depyler::prelude::*;`
//! to bring all necessary items into scope.
//!
//! # Usage
//!
//! ```rust
//! use depyler::prelude::*;
//!
//! // Now you can use Python methods on Rust types
//! let s = String::from("HELLO");
//! let lower = s.lower();  // Uses PythonStringOps trait
//!
//! let n: i32 = 42;
//! let bits = n.bit_length();  // Uses PythonIntOps trait
//! ```

// Re-export Python operation traits
pub use crate::python_ops::PythonIntOps;
pub use crate::python_ops::PythonListOps;
pub use crate::python_ops::PythonStringOps;

// Re-export PyOps arithmetic traits (DEPYLER-1307)
pub use crate::python_ops::PyAdd;
pub use crate::python_ops::PyDiv;
pub use crate::python_ops::PyFloorDiv;
pub use crate::python_ops::PyMod;
pub use crate::python_ops::PyMul;
pub use crate::python_ops::PyPow;
pub use crate::python_ops::PySub;
