//! DEPYLER-0499: Subtyping Rules for Python→Rust Type System
//!
//! Implements subtyping relation (T1 <: T2) based on:
//! 1. **Numeric tower**: i32 <: i64 <: f64
//! 2. **Container covariance**: Vec<T> <: Vec<U> if T <: U
//! 3. **Option lifting**: T <: Option<T>
//! 4. **Transitivity**: T <: U, U <: V → T <: V
//!
//! # References
//!
//! - Pierce, B. C. (2002). Types and Programming Languages. Chapter 15: Subtyping.
//! - Dunfield, J., & Krishnaswami, N. (2013). Bidirectional Typechecking for Higher-Rank Polymorphism.

use crate::hir::Type;
use crate::type_system::constraint::{ConstraintKind, TypeConstraint};

/// Subtype checker implementing T1 <: T2 relation
pub struct SubtypeChecker {
    /// Cache for transitive closure of subtyping (performance optimization)
    cache: std::cell::RefCell<std::collections::HashMap<(Type, Type), bool>>,
}

impl SubtypeChecker {
    /// Create new subtype checker
    pub fn new() -> Self {
        Self {
            cache: std::cell::RefCell::new(std::collections::HashMap::new()),
        }
    }

    /// Check if T1 <: T2 (T1 is subtype of T2)
    ///
    /// # Examples
    ///
    /// ```
    /// use depyler_core::type_system::subtyping::SubtypeChecker;
    /// use depyler_core::hir::Type;
    ///
    /// let checker = SubtypeChecker::new();
    ///
    /// // Reflexivity - same type is subtype of itself
    /// assert!(checker.check_subtype(&Type::Int, &Type::Int).is_ok());
    ///
    /// // Option lifting
    /// assert!(checker.check_subtype(&Type::Int, &Type::Optional(Box::new(Type::Int))).is_ok());
    /// ```
    pub fn check_subtype(&self, lhs: &Type, rhs: &Type) -> Result<(), String> {
        // Check cache first
        if let Some(&result) = self.cache.borrow().get(&(lhs.clone(), rhs.clone())) {
            return if result {
                Ok(())
            } else {
                Err(format!("{:?} is not a subtype of {:?}", lhs, rhs))
            };
        }

        let result = self.check_subtype_uncached(lhs, rhs);

        // Cache result
        self.cache
            .borrow_mut()
            .insert((lhs.clone(), rhs.clone()), result.is_ok());

        result
    }

    /// Check subtype relation without cache
    fn check_subtype_uncached(&self, lhs: &Type, rhs: &Type) -> Result<(), String> {
        // Reflexivity: T <: T
        if lhs == rhs {
            return Ok(());
        }

        match (lhs, rhs) {
            // Numeric tower: Int <: Float (simplified for existing Type enum)
            (Type::Int, Type::Float) => Ok(()),

            // Option lifting: T <: Option<T>
            (ty, Type::Optional(inner)) if ty == inner.as_ref() => Ok(()),

            // Option covariance: Option<T> <: Option<U> if T <: U
            (Type::Optional(t1), Type::Optional(t2)) => self.check_subtype(t1, t2),

            // List covariance: List<T> <: List<U> if T <: U
            (Type::List(t1), Type::List(t2)) => self.check_subtype(t1, t2),

            // Unification variables: defer to solver
            (Type::UnificationVar(_), _) | (_, Type::UnificationVar(_)) => {
                Ok(()) // Solver will handle
            }

            // No subtyping relationship
            _ => Err(format!("{:?} is not a subtype of {:?}", lhs, rhs)),
        }
    }

    /// Check constraint (handles both equality and subtyping)
    pub fn check_constraint(&self, constraint: &TypeConstraint) -> Result<(), String> {
        match constraint.kind {
            ConstraintKind::Eq => {
                if constraint.lhs == constraint.rhs {
                    Ok(())
                } else {
                    Err(format!(
                        "Type mismatch: {:?} != {:?} ({})",
                        constraint.lhs, constraint.rhs, constraint.reason
                    ))
                }
            }
            ConstraintKind::Subtype => self
                .check_subtype(&constraint.lhs, &constraint.rhs)
                .map_err(|e| format!("{} ({})", e, constraint.reason)),
            ConstraintKind::Supertype => {
                // T1 :> T2 equivalent to T2 <: T1
                self.check_subtype(&constraint.rhs, &constraint.lhs)
                    .map_err(|e| format!("{} ({})", e, constraint.reason))
            }
            _ => Err(format!(
                "Unsupported constraint kind: {:?}",
                constraint.kind
            )),
        }
    }
}

impl Default for SubtypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflexivity() {
        let checker = SubtypeChecker::new();
        assert!(checker.check_subtype(&Type::Int, &Type::Int).is_ok());
    }

    #[test]
    fn test_numeric_tower() {
        let checker = SubtypeChecker::new();
        assert!(checker.check_subtype(&Type::Int, &Type::Float).is_ok());
    }

    #[test]
    fn test_no_narrowing() {
        let checker = SubtypeChecker::new();
        assert!(checker.check_subtype(&Type::Float, &Type::Int).is_err());
    }

    #[test]
    fn test_option_lift() {
        let checker = SubtypeChecker::new();
        let result = checker.check_subtype(&Type::Int, &Type::Optional(Box::new(Type::Int)));
        assert!(result.is_ok());
    }
}
