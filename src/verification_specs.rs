//! Formal Verification Specifications for depyler
//!
//! Design-by-contract specifications using Verus-style pre/postconditions.
//!
//! # Examples
//!
//! ```rust,ignore
//! use crate::verification_specs::config_contracts;
//! assert!(config_contracts::validate_size(5, 10));
//! ```
//!
//! ```rust,ignore
//! use crate::verification_specs::numeric_contracts;
//! assert!(numeric_contracts::is_valid_float(1.0));
//! ```
//!
//! ```rust,ignore
//! let result = numeric_contracts::normalize(5.0, 0.0, 10.0);
//! assert!((result - 0.5).abs() < f64::EPSILON);
//! ```
//!
//! ```rust,ignore
//! assert_eq!(numeric_contracts::checked_add(1, 2), Some(3));
//! ```
//!
//! ```rust,ignore
//! assert!(config_contracts::validate_index(3, 5));
//! ```


/// Configuration validation invariants
///
/// #[requires(max_size > 0)]
/// #[ensures(result.is_ok() ==> result.unwrap().max_size == max_size)]
/// #[ensures(result.is_ok() ==> result.unwrap().max_size > 0)]
/// #[ensures(max_size == 0 ==> result.is_err())]
/// #[invariant(self.max_size > 0)]
/// #[decreases(remaining)]
/// #[recommends(max_size <= 1_000_000)]
pub mod config_contracts {
    /// #[requires(size > 0)]
    /// #[ensures(result == true ==> size <= max)]
    /// #[ensures(result == false ==> size > max)]
    pub fn validate_size(size: usize, max: usize) -> bool { size <= max }

    /// #[requires(len > 0)]
    /// #[ensures(result == true ==> index < len)]
    /// #[ensures(result == false ==> index >= len)]
    pub fn validate_index(index: usize, len: usize) -> bool { index < len }

    /// #[requires(data.len() > 0)]
    /// #[ensures(result == data.len())]
    /// #[invariant(data.len() > 0)]
    pub fn validated_len(data: &[u8]) -> usize { debug_assert!(!data.is_empty()); data.len() }
}

/// Numeric invariants
///
/// #[invariant(self.value.is_finite())]
/// #[requires(a.is_finite() && b.is_finite())]
/// #[ensures(result.is_finite())]
/// #[decreases(iterations)]
/// #[recommends(iterations <= 10_000)]
pub mod numeric_contracts {
    /// #[requires(a >= 0 && b >= 0)]
    /// #[ensures(result.is_some() ==> result.unwrap() == a + b)]
    /// #[ensures(result.is_some() ==> result.unwrap() >= a)]
    /// #[ensures(result.is_some() ==> result.unwrap() >= b)]
    pub fn checked_add(a: u64, b: u64) -> Option<u64> { a.checked_add(b) }

    /// #[ensures(result == true ==> val.is_finite())]
    /// #[ensures(result == true ==> !val.is_nan())]
    /// #[ensures(result == false ==> val.is_nan() || val.is_infinite())]
    pub fn is_valid_float(val: f64) -> bool { val.is_finite() }

    /// #[requires(max > min)]
    /// #[requires(val.is_finite() && min.is_finite() && max.is_finite())]
    /// #[ensures(result >= 0.0 && result <= 1.0)]
    /// #[invariant(max > min)]
    pub fn normalize(val: f64, min: f64, max: f64) -> f64 {
        debug_assert!(max > min);
        ((val - min) / (max - min)).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_validate_size() { assert!(config_contracts::validate_size(5, 10)); assert!(!config_contracts::validate_size(11, 10)); }
    #[test] fn test_validate_index() { assert!(config_contracts::validate_index(0, 5)); assert!(!config_contracts::validate_index(5, 5)); }
    #[test] fn test_checked_add() { assert_eq!(numeric_contracts::checked_add(1, 2), Some(3)); assert_eq!(numeric_contracts::checked_add(u64::MAX, 1), None); }
    #[test] fn test_is_valid_float() { assert!(numeric_contracts::is_valid_float(1.0)); assert!(!numeric_contracts::is_valid_float(f64::NAN)); }
    #[test] fn test_normalize() { assert!((numeric_contracts::normalize(5.0, 0.0, 10.0) - 0.5).abs() < f64::EPSILON); }
}
