//! Borrowing Shim - pure logic separated from I/O
//!
//! Extracts testable logic from borrowing.rs

use crate::hir::{ConstGeneric, Type};
use std::collections::HashSet;

/// Analysis result for a single parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowingPattern {
    /// Parameter should be taken by value (moved)
    Owned,
    /// Parameter can be borrowed immutably
    Borrowed,
    /// Parameter needs mutable borrow
    MutableBorrow,
}

impl BorrowingPattern {
    /// Get the Rust syntax prefix for this pattern
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Owned => "",
            Self::Borrowed => "&",
            Self::MutableBorrow => "&mut ",
        }
    }

    /// Check if this pattern requires a reference
    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Borrowed | Self::MutableBorrow)
    }

    /// Check if this pattern is mutable
    pub fn is_mutable(&self) -> bool {
        matches!(self, Self::MutableBorrow)
    }
}

/// Check if a type is copyable (doesn't need borrowing)
pub fn is_copyable(ty: &Type) -> bool {
    match ty {
        Type::Int | Type::Float | Type::Bool => true,
        Type::Optional(inner) => is_copyable(inner),
        Type::Tuple(types) if types.len() <= 12 => types.iter().all(is_copyable),
        Type::Custom(name) => is_copy_type_name(name),
        _ => false,
    }
}

/// Check if a type name represents a Copy type
pub fn is_copy_type_name(name: &str) -> bool {
    matches!(
        name,
        "i8" | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
            | "bool"
            | "char"
            | "()"
    )
}

/// Check if a type should be passed by reference for efficiency
pub fn should_pass_by_ref(ty: &Type) -> bool {
    match ty {
        Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => true,
        Type::Custom(name) => {
            name.starts_with("Vec<")
                || name.starts_with("HashMap<")
                || name.starts_with("HashSet<")
                || name.starts_with("String")
                || name.starts_with("BTreeMap<")
                || name.starts_with("BTreeSet<")
        }
        Type::Tuple(types) if types.len() > 3 => true,
        _ => false,
    }
}

/// Determine borrowing pattern based on usage
pub fn determine_pattern(
    is_mutated: bool,
    is_escaping: bool,
    is_loop_used: bool,
    ty: &Type,
) -> BorrowingPattern {
    if is_escaping {
        // Must be owned if it escapes
        BorrowingPattern::Owned
    } else if is_mutated {
        // Needs mutable borrow if mutated
        BorrowingPattern::MutableBorrow
    } else if is_copyable(ty) {
        // Small Copy types should be passed by value
        BorrowingPattern::Owned
    } else if is_loop_used && should_pass_by_ref(ty) {
        // Loop usage of large types should be borrowed
        BorrowingPattern::Borrowed
    } else if should_pass_by_ref(ty) {
        BorrowingPattern::Borrowed
    } else {
        BorrowingPattern::Owned
    }
}

/// Convert type to Rust string representation
pub fn type_to_rust_string(ty: &Type) -> String {
    match ty {
        Type::Int => "i64".to_string(),
        Type::Float => "f64".to_string(),
        Type::Bool => "bool".to_string(),
        Type::String => "String".to_string(),
        Type::None => "()".to_string(),
        Type::Unknown => "T".to_string(),
        Type::List(inner) => format!("Vec<{}>", type_to_rust_string(inner)),
        Type::Dict(key, value) => {
            format!(
                "HashMap<{}, {}>",
                type_to_rust_string(key),
                type_to_rust_string(value)
            )
        }
        Type::Set(inner) => format!("HashSet<{}>", type_to_rust_string(inner)),
        Type::Optional(inner) => format!("Option<{}>", type_to_rust_string(inner)),
        Type::Tuple(types) => {
            let inner: Vec<_> = types.iter().map(type_to_rust_string).collect();
            format!("({})", inner.join(", "))
        }
        Type::Custom(name) => name.clone(),
        Type::Function { params, ret } => {
            let param_strs: Vec<_> = params.iter().map(type_to_rust_string).collect();
            format!(
                "fn({}) -> {}",
                param_strs.join(", "),
                type_to_rust_string(ret)
            )
        }
        Type::TypeVar(name) => name.clone(),
        Type::UnificationVar(id) => format!("?T{}", id),
        Type::Generic { base, params } => {
            let param_strs: Vec<_> = params.iter().map(type_to_rust_string).collect();
            format!("{}<{}>", base, param_strs.join(", "))
        }
        Type::Union(types) => {
            // Rust doesn't have union types, approximate with enum
            let inner: Vec<_> = types.iter().map(type_to_rust_string).collect();
            format!("Either<{}>", inner.join(", "))
        }
        Type::Array { element_type, size } => {
            let size_str = match size {
                ConstGeneric::Literal(n) => n.to_string(),
                ConstGeneric::Parameter(name) => name.clone(),
                ConstGeneric::Expression(expr) => expr.clone(),
            };
            format!("[{}; {}]", type_to_rust_string(element_type), size_str)
        }
        Type::Final(inner) => type_to_rust_string(inner),
    }
}

/// Generate parameter signature with borrowing pattern
pub fn generate_param_signature(name: &str, ty: &Type, pattern: &BorrowingPattern) -> String {
    let type_str = type_to_rust_string(ty);
    match pattern {
        BorrowingPattern::Owned => format!("{}: {}", name, type_str),
        BorrowingPattern::Borrowed => format!("{}: &{}", name, type_str),
        BorrowingPattern::MutableBorrow => format!("{}: &mut {}", name, type_str),
    }
}

/// Parameter usage tracking
#[derive(Debug, Default)]
pub struct ParamUsage {
    pub is_mutated: bool,
    pub is_escaping: bool,
    pub is_loop_used: bool,
    pub is_read: bool,
}

impl ParamUsage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_mutated(&mut self) {
        self.is_mutated = true;
    }

    pub fn mark_escaping(&mut self) {
        self.is_escaping = true;
    }

    pub fn mark_loop_used(&mut self) {
        self.is_loop_used = true;
    }

    pub fn mark_read(&mut self) {
        self.is_read = true;
    }

    pub fn get_pattern(&self, ty: &Type) -> BorrowingPattern {
        determine_pattern(self.is_mutated, self.is_escaping, self.is_loop_used, ty)
    }
}

/// Collect all variable names from a set of identifiers
pub fn collect_param_names(params: &[String]) -> HashSet<String> {
    params.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrowing_pattern_prefix() {
        assert_eq!(BorrowingPattern::Owned.prefix(), "");
        assert_eq!(BorrowingPattern::Borrowed.prefix(), "&");
        assert_eq!(BorrowingPattern::MutableBorrow.prefix(), "&mut ");
    }

    #[test]
    fn test_borrowing_pattern_is_reference() {
        assert!(!BorrowingPattern::Owned.is_reference());
        assert!(BorrowingPattern::Borrowed.is_reference());
        assert!(BorrowingPattern::MutableBorrow.is_reference());
    }

    #[test]
    fn test_borrowing_pattern_is_mutable() {
        assert!(!BorrowingPattern::Owned.is_mutable());
        assert!(!BorrowingPattern::Borrowed.is_mutable());
        assert!(BorrowingPattern::MutableBorrow.is_mutable());
    }

    #[test]
    fn test_is_copyable_primitives() {
        assert!(is_copyable(&Type::Int));
        assert!(is_copyable(&Type::Float));
        assert!(is_copyable(&Type::Bool));
    }

    #[test]
    fn test_is_copyable_non_primitives() {
        assert!(!is_copyable(&Type::String));
        assert!(!is_copyable(&Type::List(Box::new(Type::Int))));
        assert!(!is_copyable(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
    }

    #[test]
    fn test_is_copyable_optional() {
        assert!(is_copyable(&Type::Optional(Box::new(Type::Int))));
        assert!(!is_copyable(&Type::Optional(Box::new(Type::String))));
    }

    #[test]
    fn test_is_copyable_tuple() {
        let small_copy_tuple = Type::Tuple(vec![Type::Int, Type::Bool]);
        assert!(is_copyable(&small_copy_tuple));

        let small_non_copy_tuple = Type::Tuple(vec![Type::Int, Type::String]);
        assert!(!is_copyable(&small_non_copy_tuple));
    }

    #[test]
    fn test_is_copy_type_name() {
        assert!(is_copy_type_name("i32"));
        assert!(is_copy_type_name("u64"));
        assert!(is_copy_type_name("f64"));
        assert!(is_copy_type_name("bool"));
        assert!(is_copy_type_name("char"));
        assert!(is_copy_type_name("()"));
        assert!(!is_copy_type_name("String"));
        assert!(!is_copy_type_name("Vec"));
    }

    #[test]
    fn test_should_pass_by_ref() {
        assert!(should_pass_by_ref(&Type::String));
        assert!(should_pass_by_ref(&Type::List(Box::new(Type::Int))));
        assert!(should_pass_by_ref(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
        assert!(should_pass_by_ref(&Type::Set(Box::new(Type::Int))));
        assert!(!should_pass_by_ref(&Type::Int));
        assert!(!should_pass_by_ref(&Type::Bool));
    }

    #[test]
    fn test_should_pass_by_ref_custom() {
        assert!(should_pass_by_ref(&Type::Custom("Vec<i32>".to_string())));
        assert!(should_pass_by_ref(&Type::Custom(
            "HashMap<String, i32>".to_string()
        )));
        assert!(should_pass_by_ref(&Type::Custom("String".to_string())));
        assert!(!should_pass_by_ref(&Type::Custom("i32".to_string())));
    }

    #[test]
    fn test_should_pass_by_ref_large_tuple() {
        let large_tuple = Type::Tuple(vec![Type::Int; 5]);
        assert!(should_pass_by_ref(&large_tuple));

        let small_tuple = Type::Tuple(vec![Type::Int; 2]);
        assert!(!should_pass_by_ref(&small_tuple));
    }

    #[test]
    fn test_determine_pattern_escaping() {
        let pattern = determine_pattern(false, true, false, &Type::String);
        assert_eq!(pattern, BorrowingPattern::Owned);
    }

    #[test]
    fn test_determine_pattern_mutated() {
        let pattern = determine_pattern(true, false, false, &Type::String);
        assert_eq!(pattern, BorrowingPattern::MutableBorrow);
    }

    #[test]
    fn test_determine_pattern_copyable() {
        let pattern = determine_pattern(false, false, false, &Type::Int);
        assert_eq!(pattern, BorrowingPattern::Owned);
    }

    #[test]
    fn test_determine_pattern_borrowed() {
        let pattern = determine_pattern(false, false, false, &Type::String);
        assert_eq!(pattern, BorrowingPattern::Borrowed);
    }

    #[test]
    fn test_determine_pattern_loop_used() {
        let pattern = determine_pattern(false, false, true, &Type::String);
        assert_eq!(pattern, BorrowingPattern::Borrowed);
    }

    #[test]
    fn test_type_to_rust_string_primitives() {
        assert_eq!(type_to_rust_string(&Type::Int), "i64");
        assert_eq!(type_to_rust_string(&Type::Float), "f64");
        assert_eq!(type_to_rust_string(&Type::Bool), "bool");
        assert_eq!(type_to_rust_string(&Type::String), "String");
        assert_eq!(type_to_rust_string(&Type::None), "()");
        assert_eq!(type_to_rust_string(&Type::Unknown), "T");
    }

    #[test]
    fn test_type_to_rust_string_containers() {
        assert_eq!(
            type_to_rust_string(&Type::List(Box::new(Type::Int))),
            "Vec<i64>"
        );
        assert_eq!(
            type_to_rust_string(&Type::Dict(Box::new(Type::String), Box::new(Type::Int))),
            "HashMap<String, i64>"
        );
        assert_eq!(
            type_to_rust_string(&Type::Set(Box::new(Type::Int))),
            "HashSet<i64>"
        );
    }

    #[test]
    fn test_type_to_rust_string_optional() {
        assert_eq!(
            type_to_rust_string(&Type::Optional(Box::new(Type::Int))),
            "Option<i64>"
        );
    }

    #[test]
    fn test_type_to_rust_string_tuple() {
        let tuple = Type::Tuple(vec![Type::Int, Type::String, Type::Bool]);
        assert_eq!(type_to_rust_string(&tuple), "(i64, String, bool)");
    }

    #[test]
    fn test_type_to_rust_string_function() {
        let func = Type::Function {
            params: vec![Type::Int, Type::String],
            ret: Box::new(Type::Bool),
        };
        assert_eq!(type_to_rust_string(&func), "fn(i64, String) -> bool");
    }

    #[test]
    fn test_type_to_rust_string_typevar() {
        assert_eq!(type_to_rust_string(&Type::TypeVar("T".to_string())), "T");
        assert_eq!(type_to_rust_string(&Type::TypeVar("U".to_string())), "U");
    }

    #[test]
    fn test_type_to_rust_string_unification_var() {
        assert_eq!(type_to_rust_string(&Type::UnificationVar(0)), "?T0");
        assert_eq!(type_to_rust_string(&Type::UnificationVar(42)), "?T42");
    }

    #[test]
    fn test_type_to_rust_string_generic() {
        let generic = Type::Generic {
            base: "Result".to_string(),
            params: vec![Type::Int, Type::String],
        };
        assert_eq!(type_to_rust_string(&generic), "Result<i64, String>");
    }

    #[test]
    fn test_type_to_rust_string_union() {
        let union = Type::Union(vec![Type::Int, Type::String]);
        assert_eq!(type_to_rust_string(&union), "Either<i64, String>");
    }

    #[test]
    fn test_type_to_rust_string_array() {
        let array = Type::Array {
            element_type: Box::new(Type::Int),
            size: ConstGeneric::Literal(10),
        };
        assert_eq!(type_to_rust_string(&array), "[i64; 10]");

        let array_param = Type::Array {
            element_type: Box::new(Type::Float),
            size: ConstGeneric::Parameter("N".to_string()),
        };
        assert_eq!(type_to_rust_string(&array_param), "[f64; N]");

        let array_expr = Type::Array {
            element_type: Box::new(Type::Bool),
            size: ConstGeneric::Expression("N + 1".to_string()),
        };
        assert_eq!(type_to_rust_string(&array_expr), "[bool; N + 1]");
    }

    #[test]
    fn test_type_to_rust_string_final() {
        let final_type = Type::Final(Box::new(Type::Int));
        assert_eq!(type_to_rust_string(&final_type), "i64");
    }

    #[test]
    fn test_generate_param_signature() {
        assert_eq!(
            generate_param_signature("x", &Type::Int, &BorrowingPattern::Owned),
            "x: i64"
        );
        assert_eq!(
            generate_param_signature("s", &Type::String, &BorrowingPattern::Borrowed),
            "s: &String"
        );
        assert_eq!(
            generate_param_signature(
                "v",
                &Type::List(Box::new(Type::Int)),
                &BorrowingPattern::MutableBorrow
            ),
            "v: &mut Vec<i64>"
        );
    }

    #[test]
    fn test_param_usage_new() {
        let usage = ParamUsage::new();
        assert!(!usage.is_mutated);
        assert!(!usage.is_escaping);
        assert!(!usage.is_loop_used);
        assert!(!usage.is_read);
    }

    #[test]
    fn test_param_usage_mark_mutated() {
        let mut usage = ParamUsage::new();
        usage.mark_mutated();
        assert!(usage.is_mutated);
    }

    #[test]
    fn test_param_usage_mark_escaping() {
        let mut usage = ParamUsage::new();
        usage.mark_escaping();
        assert!(usage.is_escaping);
    }

    #[test]
    fn test_param_usage_mark_loop_used() {
        let mut usage = ParamUsage::new();
        usage.mark_loop_used();
        assert!(usage.is_loop_used);
    }

    #[test]
    fn test_param_usage_mark_read() {
        let mut usage = ParamUsage::new();
        usage.mark_read();
        assert!(usage.is_read);
    }

    #[test]
    fn test_param_usage_get_pattern() {
        let mut usage = ParamUsage::new();
        assert_eq!(usage.get_pattern(&Type::String), BorrowingPattern::Borrowed);

        usage.mark_mutated();
        assert_eq!(
            usage.get_pattern(&Type::String),
            BorrowingPattern::MutableBorrow
        );

        let mut escaping_usage = ParamUsage::new();
        escaping_usage.mark_escaping();
        assert_eq!(
            escaping_usage.get_pattern(&Type::String),
            BorrowingPattern::Owned
        );
    }

    #[test]
    fn test_collect_param_names() {
        let params = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let names = collect_param_names(&params);
        assert!(names.contains("a"));
        assert!(names.contains("b"));
        assert!(names.contains("c"));
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn test_collect_param_names_empty() {
        let params: Vec<String> = vec![];
        let names = collect_param_names(&params);
        assert!(names.is_empty());
    }
}
