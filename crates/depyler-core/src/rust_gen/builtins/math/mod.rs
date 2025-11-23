//! Math-related builtin functions
//!
//! This module contains handlers for Python's math-related builtin functions:
//! - abs() - absolute value
//! - round() - rounding to nearest integer or float
//! - pow() - exponentiation
//! - min() - minimum of values or iterable
//! - max() - maximum of values or iterable

pub mod abs;
pub mod minmax;
pub mod pow;
pub mod round;

pub use abs::handle_abs;
pub use minmax::{handle_max, handle_min};
pub use pow::handle_pow;
pub use round::handle_round;
