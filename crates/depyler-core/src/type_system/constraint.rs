//! DEPYLER-0499: Type Constraints with Subtyping Support
//!
//! Replaces pure equality unification (T1 = T2) with subtyping constraints (T1 <: T2).
//!
//! # Constraint Kinds
//!
//! - **Eq**: T1 == T2 (identity, must be exactly same type)
//! - **Subtype**: T1 <: T2 (T1 can be used where T2 expected)
//! - **Supertype**: T1 :> T2 (reverse subtyping)
//!
//! # Example
//!
//! ```rust
//! use depyler_core::type_system::constraint::{TypeConstraint, ConstraintKind};
//! use depyler_core::hir::Type;
//!
//! // Int argument passed to Optional<Int> parameter (subtyping)
//! let constraint = TypeConstraint {
//!     lhs: Type::Int,
//!     rhs: Type::Optional(Box::new(Type::Int)),
//!     kind: ConstraintKind::Subtype,
//!     reason: "Function call: process_optional(n)".to_string(),
//! };
//! ```

use crate::hir::Type;

/// Type constraint representing a relationship between two types
#[derive(Debug, Clone, PartialEq)]
pub struct TypeConstraint {
    /// Left-hand side type
    pub lhs: Type,
    /// Right-hand side type
    pub rhs: Type,
    /// Kind of constraint (equality vs subtyping)
    pub kind: ConstraintKind,
    /// Human-readable reason for error messages
    pub reason: String,
}

/// Kind of type constraint
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintKind {
    /// T1 == T2 (exact equality, no subtyping)
    Eq,

    /// T1 <: T2 (T1 is subtype of T2)
    ///
    /// Permits:
    /// - i32 <: i64 (widening)
    /// - T <: Option<T> (lifting)
    /// - Vec<T> <: Vec<U> if T <: U (covariance)
    Subtype,

    /// T1 :> T2 (T1 is supertype of T2, equivalent to T2 <: T1)
    Supertype,

    /// T1 callable with args → T2 (function type)
    Callable,

    /// T1 has field with type T2
    HasField(String),

    /// T1 and T2 support arithmetic operations
    Arithmetic,
}

impl TypeConstraint {
    /// Create equality constraint (T1 == T2)
    pub fn eq(lhs: Type, rhs: Type, reason: impl Into<String>) -> Self {
        Self {
            lhs,
            rhs,
            kind: ConstraintKind::Eq,
            reason: reason.into(),
        }
    }

    /// Create subtype constraint (T1 <: T2)
    pub fn subtype(lhs: Type, rhs: Type, reason: impl Into<String>) -> Self {
        Self {
            lhs,
            rhs,
            kind: ConstraintKind::Subtype,
            reason: reason.into(),
        }
    }

    /// Create supertype constraint (T1 :> T2)
    pub fn supertype(lhs: Type, rhs: Type, reason: impl Into<String>) -> Self {
        Self {
            lhs,
            rhs,
            kind: ConstraintKind::Supertype,
            reason: reason.into(),
        }
    }
}

impl std::fmt::Display for ConstraintKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintKind::Eq => write!(f, "=="),
            ConstraintKind::Subtype => write!(f, "<:"),
            ConstraintKind::Supertype => write!(f, ":>"),
            ConstraintKind::Callable => write!(f, "callable"),
            ConstraintKind::HasField(field) => write!(f, "has field {}", field),
            ConstraintKind::Arithmetic => write!(f, "arithmetic"),
        }
    }
}

impl std::fmt::Display for TypeConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {:?} ({})",
            self.lhs, self.kind, self.rhs, self.reason
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_eq() {
        let c = TypeConstraint::eq(Type::Int, Type::Int, "test");
        assert_eq!(c.kind, ConstraintKind::Eq);
        assert_eq!(c.lhs, Type::Int);
        assert_eq!(c.rhs, Type::Int);
        assert_eq!(c.reason, "test");
    }

    #[test]
    fn test_constraint_subtype() {
        let c = TypeConstraint::subtype(Type::Int, Type::Float, "widening");
        assert_eq!(c.kind, ConstraintKind::Subtype);
        assert_eq!(c.lhs, Type::Int);
        assert_eq!(c.rhs, Type::Float);
    }

    #[test]
    fn test_constraint_supertype() {
        let c = TypeConstraint::supertype(Type::Float, Type::Int, "narrowing");
        assert_eq!(c.kind, ConstraintKind::Supertype);
    }

    #[test]
    fn test_constraint_kind_display() {
        assert_eq!(format!("{}", ConstraintKind::Eq), "==");
        assert_eq!(format!("{}", ConstraintKind::Subtype), "<:");
        assert_eq!(format!("{}", ConstraintKind::Supertype), ":>");
        assert_eq!(format!("{}", ConstraintKind::Callable), "callable");
        assert_eq!(format!("{}", ConstraintKind::HasField("x".into())), "has field x");
        assert_eq!(format!("{}", ConstraintKind::Arithmetic), "arithmetic");
    }

    #[test]
    fn test_type_constraint_display() {
        let c = TypeConstraint::eq(Type::Int, Type::Float, "mismatch");
        let s = format!("{}", c);
        assert!(s.contains("=="));
        assert!(s.contains("mismatch"));
    }

    #[test]
    fn test_constraint_clone() {
        let c1 = TypeConstraint::subtype(Type::String, Type::String, "clone test");
        let c2 = c1.clone();
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_constraint_kind_eq() {
        assert_eq!(ConstraintKind::Eq, ConstraintKind::Eq);
        assert_ne!(ConstraintKind::Eq, ConstraintKind::Subtype);
        assert_eq!(
            ConstraintKind::HasField("a".into()),
            ConstraintKind::HasField("a".into())
        );
    }
}
