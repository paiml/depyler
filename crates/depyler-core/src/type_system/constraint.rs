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
//! // i32 argument passed to i64 parameter (subtyping)
//! let constraint = TypeConstraint {
//!     lhs: Type::Int32,
//!     rhs: Type::Int64,
//!     kind: ConstraintKind::Subtype,
//!     reason: "Function call: fibonacci(n - 1)".to_string(),
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
