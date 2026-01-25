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
        assert_eq!(
            format!("{}", ConstraintKind::HasField("x".into())),
            "has field x"
        );
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

    // ============================================================================
    // EXTREME TDD: Additional comprehensive tests
    // ============================================================================

    #[test]
    fn test_constraint_kind_debug() {
        let kinds = vec![
            ConstraintKind::Eq,
            ConstraintKind::Subtype,
            ConstraintKind::Supertype,
            ConstraintKind::Callable,
            ConstraintKind::HasField("field".to_string()),
            ConstraintKind::Arithmetic,
        ];
        for kind in kinds {
            let debug_str = format!("{:?}", kind);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_type_constraint_with_optional() {
        let c = TypeConstraint::subtype(
            Type::Int,
            Type::Optional(Box::new(Type::Int)),
            "int to optional int",
        );
        assert_eq!(c.kind, ConstraintKind::Subtype);
    }

    #[test]
    fn test_type_constraint_with_list() {
        let c = TypeConstraint::eq(
            Type::List(Box::new(Type::Int)),
            Type::List(Box::new(Type::Int)),
            "list equality",
        );
        assert_eq!(c.kind, ConstraintKind::Eq);
    }

    #[test]
    fn test_type_constraint_with_dict() {
        let c = TypeConstraint::eq(
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
            "dict equality",
        );
        assert_eq!(c.kind, ConstraintKind::Eq);
    }

    #[test]
    fn test_constraint_kind_has_field_different() {
        assert_ne!(
            ConstraintKind::HasField("a".into()),
            ConstraintKind::HasField("b".into())
        );
    }

    #[test]
    fn test_type_constraint_debug() {
        let c = TypeConstraint::eq(Type::Int, Type::Int, "test");
        let debug_str = format!("{:?}", c);
        assert!(debug_str.contains("Int"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_type_constraint_partial_eq() {
        let c1 = TypeConstraint::eq(Type::Int, Type::Int, "same");
        let c2 = TypeConstraint::eq(Type::Int, Type::Int, "same");
        let c3 = TypeConstraint::eq(Type::Int, Type::Float, "different");
        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_constraint_kind_clone() {
        let kinds = vec![
            ConstraintKind::Eq,
            ConstraintKind::Subtype,
            ConstraintKind::Supertype,
            ConstraintKind::Callable,
            ConstraintKind::HasField("x".to_string()),
            ConstraintKind::Arithmetic,
        ];
        for kind in kinds {
            let cloned = kind.clone();
            assert_eq!(kind, cloned);
        }
    }

    #[test]
    fn test_type_constraint_reason_conversion() {
        // Test Into<String> conversion for reason
        let c = TypeConstraint::eq(Type::Bool, Type::Bool, String::from("string reason"));
        assert_eq!(c.reason, "string reason");

        let c = TypeConstraint::subtype(Type::Int, Type::Float, "str reason");
        assert_eq!(c.reason, "str reason");
    }

    #[test]
    fn test_constraint_all_display_variants() {
        // Ensure all variants can be displayed
        let constraints = vec![
            TypeConstraint {
                lhs: Type::Int,
                rhs: Type::Int,
                kind: ConstraintKind::Eq,
                reason: "eq test".to_string(),
            },
            TypeConstraint {
                lhs: Type::Int,
                rhs: Type::Float,
                kind: ConstraintKind::Subtype,
                reason: "sub test".to_string(),
            },
            TypeConstraint {
                lhs: Type::Float,
                rhs: Type::Int,
                kind: ConstraintKind::Supertype,
                reason: "super test".to_string(),
            },
            TypeConstraint {
                lhs: Type::Unknown,
                rhs: Type::Int,
                kind: ConstraintKind::Callable,
                reason: "call test".to_string(),
            },
            TypeConstraint {
                lhs: Type::Unknown,
                rhs: Type::String,
                kind: ConstraintKind::HasField("name".to_string()),
                reason: "field test".to_string(),
            },
            TypeConstraint {
                lhs: Type::Int,
                rhs: Type::Int,
                kind: ConstraintKind::Arithmetic,
                reason: "arith test".to_string(),
            },
        ];

        for c in constraints {
            let display = format!("{}", c);
            assert!(!display.is_empty());
            assert!(display.contains(&c.reason));
        }
    }
}
