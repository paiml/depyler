//! Utility functions for working with HIR expressions

use crate::hir::HirExpr;

/// Extract the root variable name from a potentially nested expression.
///
/// This function recursively traverses attribute accesses and index operations
/// to find the base variable name.
///
/// This is crucial for interprocedural mutation analysis: when `state.data`
/// is passed to a function that mutates its parameter, we need to know that
/// the root variable `state` is being mutated.
pub fn extract_root_var(expr: &HirExpr) -> Option<String> {
    match expr {
        HirExpr::Var(name) => Some(name.clone()),
        HirExpr::Attribute { value, .. } => extract_root_var(value),
        HirExpr::Index { base, .. } => extract_root_var(base),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_root_var_simple() {
        let expr = HirExpr::Var("state".to_string());
        assert_eq!(extract_root_var(&expr), Some("state".to_string()));
    }

    #[test]
    fn test_extract_root_var_attribute() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("state".to_string())),
            attr: "data".to_string(),
        };
        assert_eq!(extract_root_var(&expr), Some("state".to_string()));
    }

    #[test]
    fn test_extract_root_var_nested_attribute() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("state".to_string())),
                attr: "data".to_string(),
            }),
            attr: "field".to_string(),
        };
        assert_eq!(extract_root_var(&expr), Some("state".to_string()));
    }

    #[test]
    fn test_extract_root_var_index() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("items".to_string())),
            index: Box::new(HirExpr::Literal(crate::hir::Literal::Int(0))),
        };
        assert_eq!(extract_root_var(&expr), Some("items".to_string()));
    }

    #[test]
    fn test_extract_root_var_literal() {
        let expr = HirExpr::Literal(crate::hir::Literal::Int(42));
        assert_eq!(extract_root_var(&expr), None);
    }
}
