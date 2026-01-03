//! Codegen Shim - pure logic separated from I/O
//!
//! Extracts testable logic from codegen.rs

use crate::hir::{AssignTarget, HirExpr, HirStmt, Literal, Type};
use std::collections::HashSet;

/// Check if a type uses HashMap
pub fn uses_hashmap(ty: &Type) -> bool {
    match ty {
        Type::Dict(_, _) => true,
        Type::List(inner) | Type::Optional(inner) => uses_hashmap(inner),
        Type::Tuple(types) => types.iter().any(uses_hashmap),
        Type::Function { params, ret } => params.iter().any(uses_hashmap) || uses_hashmap(ret),
        _ => false,
    }
}

/// Check if statement uses HashMap
pub fn stmt_uses_hashmap(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { value, .. } => expr_uses_hashmap(value),
        HirStmt::Return(Some(expr)) => expr_uses_hashmap(expr),
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_uses_hashmap(condition)
                || body_uses_hashmap(then_body)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body_uses_hashmap(body))
        }
        HirStmt::While { condition, body } => {
            expr_uses_hashmap(condition) || body_uses_hashmap(body)
        }
        HirStmt::For { iter, body, .. } => expr_uses_hashmap(iter) || body_uses_hashmap(body),
        HirStmt::Expr(expr) => expr_uses_hashmap(expr),
        _ => false,
    }
}

/// Check if function body uses HashMap
pub fn body_uses_hashmap(body: &[HirStmt]) -> bool {
    body.iter().any(stmt_uses_hashmap)
}

/// Check if expression uses HashMap
pub fn expr_uses_hashmap(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Dict(_) => true,
        HirExpr::Binary { left, right, .. } => expr_uses_hashmap(left) || expr_uses_hashmap(right),
        HirExpr::Unary { operand, .. } => expr_uses_hashmap(operand),
        HirExpr::Call { args, .. } => args.iter().any(expr_uses_hashmap),
        HirExpr::Index { base, index } => expr_uses_hashmap(base) || expr_uses_hashmap(index),
        HirExpr::List(items) | HirExpr::Tuple(items) => items.iter().any(expr_uses_hashmap),
        _ => false,
    }
}

/// Scope tracker for variable declarations
#[derive(Debug, Default)]
pub struct ScopeTracker {
    declared_vars: Vec<HashSet<String>>,
}

impl ScopeTracker {
    pub fn new() -> Self {
        Self {
            declared_vars: vec![HashSet::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.declared_vars.push(HashSet::new());
    }

    pub fn exit_scope(&mut self) {
        self.declared_vars.pop();
    }

    pub fn declare(&mut self, name: &str) {
        if let Some(scope) = self.declared_vars.last_mut() {
            scope.insert(name.to_string());
        }
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.declared_vars.iter().any(|scope| scope.contains(name))
    }

    pub fn is_declared_in_current_scope(&self, name: &str) -> bool {
        self.declared_vars
            .last()
            .is_some_and(|scope| scope.contains(name))
    }

    pub fn depth(&self) -> usize {
        self.declared_vars.len()
    }
}

/// Extract variable name from assign target
pub fn extract_var_name(target: &AssignTarget) -> Option<String> {
    match target {
        AssignTarget::Symbol(name) => Some(name.clone()),
        AssignTarget::Attribute { value: _, attr: _ } => None,
        AssignTarget::Index { base: _, index: _ } => None,
        AssignTarget::Tuple(targets) => {
            // For tuple unpacking, return first var
            targets.first().and_then(extract_var_name)
        }
    }
}

/// Check if expression is a simple literal
pub fn is_simple_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(_))
}

/// Check if expression is a constant (can be evaluated at compile time)
pub fn is_constant_expr(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Literal(_) => true,
        HirExpr::List(items) | HirExpr::Tuple(items) => items.iter().all(is_constant_expr),
        HirExpr::Unary { operand, .. } => is_constant_expr(operand),
        HirExpr::Binary { left, right, .. } => is_constant_expr(left) && is_constant_expr(right),
        _ => false,
    }
}

/// Estimate expression complexity (for optimization decisions)
pub fn expr_complexity(expr: &HirExpr) -> usize {
    match expr {
        HirExpr::Literal(_) => 1,
        HirExpr::Var(_) => 1,
        HirExpr::List(items) | HirExpr::Tuple(items) => {
            1 + items.iter().map(expr_complexity).sum::<usize>()
        }
        HirExpr::Dict(pairs) => {
            1 + pairs
                .iter()
                .map(|(k, v)| expr_complexity(k) + expr_complexity(v))
                .sum::<usize>()
        }
        HirExpr::Binary { left, right, .. } => 1 + expr_complexity(left) + expr_complexity(right),
        HirExpr::Unary { operand, .. } => 1 + expr_complexity(operand),
        HirExpr::Call { args, .. } => 2 + args.iter().map(expr_complexity).sum::<usize>(),
        HirExpr::Index { base, index } => 1 + expr_complexity(base) + expr_complexity(index),
        HirExpr::Attribute { value, .. } => 1 + expr_complexity(value),
        HirExpr::MethodCall { object, args, .. } => {
            2 + expr_complexity(object) + args.iter().map(expr_complexity).sum::<usize>()
        }
        HirExpr::Lambda { body, .. } => 3 + expr_complexity(body),
        HirExpr::IfExpr { test, body, orelse } => {
            2 + expr_complexity(test) + expr_complexity(body) + expr_complexity(orelse)
        }
        HirExpr::ListComp { element, generators, .. } => {
            3 + expr_complexity(element) + generators.len()
        }
        _ => 1,
    }
}

/// Check if type needs to be boxed (for recursive types)
pub fn needs_boxing(ty: &Type) -> bool {
    match ty {
        Type::Custom(name) => name.starts_with("Box<") || name.contains("Rc<") || name.contains("Arc<"),
        Type::List(inner) => needs_boxing(inner),
        Type::Optional(inner) => needs_boxing(inner),
        Type::Tuple(types) => types.iter().any(needs_boxing),
        _ => false,
    }
}

/// Check if type is a reference type
pub fn is_reference_type(ty: &Type) -> bool {
    match ty {
        Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => true,
        Type::Custom(name) => {
            name.starts_with("Vec<")
                || name.starts_with("HashMap<")
                || name.starts_with("String")
                || name.starts_with("&")
        }
        _ => false,
    }
}

/// Check if type is a primitive (Copy) type
pub fn is_primitive_type(ty: &Type) -> bool {
    matches!(ty, Type::Int | Type::Float | Type::Bool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::BinOp;

    // Helper to create literal expressions
    fn int_expr(n: i64) -> HirExpr {
        HirExpr::Literal(Literal::Int(n))
    }

    fn float_expr(f: f64) -> HirExpr {
        HirExpr::Literal(Literal::Float(f))
    }

    fn bool_expr(b: bool) -> HirExpr {
        HirExpr::Literal(Literal::Bool(b))
    }

    fn string_expr(s: &str) -> HirExpr {
        HirExpr::Literal(Literal::String(s.to_string()))
    }

    fn none_expr() -> HirExpr {
        HirExpr::Literal(Literal::None)
    }

    fn var_expr(name: &str) -> HirExpr {
        HirExpr::Var(name.to_string())
    }

    #[test]
    fn test_uses_hashmap_dict() {
        assert!(uses_hashmap(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
    }

    #[test]
    fn test_uses_hashmap_primitives() {
        assert!(!uses_hashmap(&Type::Int));
        assert!(!uses_hashmap(&Type::Float));
        assert!(!uses_hashmap(&Type::Bool));
        assert!(!uses_hashmap(&Type::String));
    }

    #[test]
    fn test_uses_hashmap_nested() {
        let nested = Type::List(Box::new(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int),
        )));
        assert!(uses_hashmap(&nested));
    }

    #[test]
    fn test_uses_hashmap_optional() {
        let opt_dict = Type::Optional(Box::new(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int),
        )));
        assert!(uses_hashmap(&opt_dict));

        let opt_int = Type::Optional(Box::new(Type::Int));
        assert!(!uses_hashmap(&opt_int));
    }

    #[test]
    fn test_uses_hashmap_tuple() {
        let tuple_with_dict = Type::Tuple(vec![
            Type::Int,
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
        ]);
        assert!(uses_hashmap(&tuple_with_dict));

        let tuple_no_dict = Type::Tuple(vec![Type::Int, Type::String]);
        assert!(!uses_hashmap(&tuple_no_dict));
    }

    #[test]
    fn test_uses_hashmap_function() {
        let func_with_dict_param = Type::Function {
            params: vec![Type::Dict(Box::new(Type::String), Box::new(Type::Int))],
            ret: Box::new(Type::Int),
        };
        assert!(uses_hashmap(&func_with_dict_param));

        let func_with_dict_ret = Type::Function {
            params: vec![Type::Int],
            ret: Box::new(Type::Dict(Box::new(Type::String), Box::new(Type::Int))),
        };
        assert!(uses_hashmap(&func_with_dict_ret));
    }

    #[test]
    fn test_expr_uses_hashmap_dict() {
        let dict_expr = HirExpr::Dict(vec![]);
        assert!(expr_uses_hashmap(&dict_expr));
    }

    #[test]
    fn test_expr_uses_hashmap_primitives() {
        assert!(!expr_uses_hashmap(&int_expr(42)));
        assert!(!expr_uses_hashmap(&float_expr(3.14)));
        assert!(!expr_uses_hashmap(&bool_expr(true)));
        assert!(!expr_uses_hashmap(&string_expr("test")));
    }

    #[test]
    fn test_expr_uses_hashmap_nested() {
        let nested = HirExpr::List(vec![HirExpr::Dict(vec![])]);
        assert!(expr_uses_hashmap(&nested));
    }

    #[test]
    fn test_expr_uses_hashmap_binary() {
        let binary_with_dict = HirExpr::Binary {
            left: Box::new(HirExpr::Dict(vec![])),
            op: BinOp::Add,
            right: Box::new(int_expr(1)),
        };
        assert!(expr_uses_hashmap(&binary_with_dict));

        let binary_no_dict = HirExpr::Binary {
            left: Box::new(int_expr(1)),
            op: BinOp::Add,
            right: Box::new(int_expr(2)),
        };
        assert!(!expr_uses_hashmap(&binary_no_dict));
    }

    #[test]
    fn test_scope_tracker_new() {
        let tracker = ScopeTracker::new();
        assert_eq!(tracker.depth(), 1);
    }

    #[test]
    fn test_scope_tracker_declare() {
        let mut tracker = ScopeTracker::new();
        tracker.declare("x");
        assert!(tracker.is_declared("x"));
        assert!(!tracker.is_declared("y"));
    }

    #[test]
    fn test_scope_tracker_nested_scopes() {
        let mut tracker = ScopeTracker::new();
        tracker.declare("outer");
        tracker.enter_scope();
        tracker.declare("inner");

        assert!(tracker.is_declared("outer"));
        assert!(tracker.is_declared("inner"));
        assert!(tracker.is_declared_in_current_scope("inner"));
        assert!(!tracker.is_declared_in_current_scope("outer"));

        tracker.exit_scope();
        assert!(tracker.is_declared("outer"));
        assert!(!tracker.is_declared("inner"));
    }

    #[test]
    fn test_scope_tracker_depth() {
        let mut tracker = ScopeTracker::new();
        assert_eq!(tracker.depth(), 1);
        tracker.enter_scope();
        assert_eq!(tracker.depth(), 2);
        tracker.enter_scope();
        assert_eq!(tracker.depth(), 3);
        tracker.exit_scope();
        assert_eq!(tracker.depth(), 2);
    }

    #[test]
    fn test_extract_var_name_symbol() {
        let target = AssignTarget::Symbol("x".to_string());
        assert_eq!(extract_var_name(&target), Some("x".to_string()));
    }

    #[test]
    fn test_extract_var_name_attribute() {
        let target = AssignTarget::Attribute {
            value: Box::new(var_expr("obj")),
            attr: "field".to_string(),
        };
        assert_eq!(extract_var_name(&target), None);
    }

    #[test]
    fn test_extract_var_name_index() {
        let target = AssignTarget::Index {
            base: Box::new(var_expr("arr")),
            index: Box::new(int_expr(0)),
        };
        assert_eq!(extract_var_name(&target), None);
    }

    #[test]
    fn test_extract_var_name_tuple() {
        let target = AssignTarget::Tuple(vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ]);
        assert_eq!(extract_var_name(&target), Some("a".to_string()));
    }

    #[test]
    fn test_is_simple_literal() {
        assert!(is_simple_literal(&int_expr(42)));
        assert!(is_simple_literal(&float_expr(3.14)));
        assert!(is_simple_literal(&bool_expr(true)));
        assert!(is_simple_literal(&string_expr("test")));
        assert!(is_simple_literal(&none_expr()));
        assert!(!is_simple_literal(&var_expr("x")));
        assert!(!is_simple_literal(&HirExpr::List(vec![])));
    }

    #[test]
    fn test_is_constant_expr_literals() {
        assert!(is_constant_expr(&int_expr(42)));
        assert!(is_constant_expr(&float_expr(3.14)));
        assert!(is_constant_expr(&bool_expr(true)));
        assert!(is_constant_expr(&string_expr("test")));
        assert!(is_constant_expr(&none_expr()));
    }

    #[test]
    fn test_is_constant_expr_list() {
        let const_list = HirExpr::List(vec![int_expr(1), int_expr(2)]);
        assert!(is_constant_expr(&const_list));

        let non_const_list = HirExpr::List(vec![var_expr("x")]);
        assert!(!is_constant_expr(&non_const_list));
    }

    #[test]
    fn test_is_constant_expr_unary() {
        use crate::hir::UnaryOp;
        let const_unary = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(int_expr(42)),
        };
        assert!(is_constant_expr(&const_unary));
    }

    #[test]
    fn test_is_constant_expr_binary() {
        let const_binary = HirExpr::Binary {
            left: Box::new(int_expr(1)),
            op: BinOp::Add,
            right: Box::new(int_expr(2)),
        };
        assert!(is_constant_expr(&const_binary));

        let non_const_binary = HirExpr::Binary {
            left: Box::new(var_expr("x")),
            op: BinOp::Add,
            right: Box::new(int_expr(2)),
        };
        assert!(!is_constant_expr(&non_const_binary));
    }

    #[test]
    fn test_expr_complexity_literals() {
        assert_eq!(expr_complexity(&int_expr(42)), 1);
        assert_eq!(expr_complexity(&float_expr(3.14)), 1);
        assert_eq!(expr_complexity(&bool_expr(true)), 1);
        assert_eq!(expr_complexity(&string_expr("test")), 1);
        assert_eq!(expr_complexity(&none_expr()), 1);
    }

    #[test]
    fn test_expr_complexity_variable() {
        assert_eq!(expr_complexity(&var_expr("x")), 1);
    }

    #[test]
    fn test_expr_complexity_list() {
        let list = HirExpr::List(vec![int_expr(1), int_expr(2), int_expr(3)]);
        assert_eq!(expr_complexity(&list), 4); // 1 + 3
    }

    #[test]
    fn test_expr_complexity_binary() {
        let binary = HirExpr::Binary {
            left: Box::new(int_expr(1)),
            op: BinOp::Add,
            right: Box::new(int_expr(2)),
        };
        assert_eq!(expr_complexity(&binary), 3); // 1 + 1 + 1
    }

    #[test]
    fn test_needs_boxing() {
        assert!(needs_boxing(&Type::Custom("Box<Node>".to_string())));
        assert!(needs_boxing(&Type::Custom("Rc<Node>".to_string())));
        assert!(needs_boxing(&Type::Custom("Arc<Node>".to_string())));
        assert!(!needs_boxing(&Type::Int));
        assert!(!needs_boxing(&Type::String));
    }

    #[test]
    fn test_needs_boxing_nested() {
        let nested = Type::List(Box::new(Type::Custom("Box<Node>".to_string())));
        assert!(needs_boxing(&nested));
    }

    #[test]
    fn test_is_reference_type() {
        assert!(is_reference_type(&Type::String));
        assert!(is_reference_type(&Type::List(Box::new(Type::Int))));
        assert!(is_reference_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
        assert!(is_reference_type(&Type::Set(Box::new(Type::Int))));
        assert!(is_reference_type(&Type::Custom("Vec<i32>".to_string())));
        assert!(is_reference_type(&Type::Custom("HashMap<String, i32>".to_string())));
        assert!(!is_reference_type(&Type::Int));
        assert!(!is_reference_type(&Type::Float));
    }

    #[test]
    fn test_is_primitive_type() {
        assert!(is_primitive_type(&Type::Int));
        assert!(is_primitive_type(&Type::Float));
        assert!(is_primitive_type(&Type::Bool));
        assert!(!is_primitive_type(&Type::String));
        assert!(!is_primitive_type(&Type::List(Box::new(Type::Int))));
    }

    #[test]
    fn test_stmt_uses_hashmap_assign() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Dict(vec![]),
            type_annotation: None,
        };
        assert!(stmt_uses_hashmap(&stmt));
    }

    #[test]
    fn test_stmt_uses_hashmap_return() {
        let stmt = HirStmt::Return(Some(HirExpr::Dict(vec![])));
        assert!(stmt_uses_hashmap(&stmt));

        let stmt_no_dict = HirStmt::Return(Some(int_expr(42)));
        assert!(!stmt_uses_hashmap(&stmt_no_dict));
    }

    #[test]
    fn test_stmt_uses_hashmap_expr() {
        let stmt = HirStmt::Expr(HirExpr::Dict(vec![]));
        assert!(stmt_uses_hashmap(&stmt));
    }

    #[test]
    fn test_body_uses_hashmap() {
        let body_with_dict = vec![
            HirStmt::Expr(int_expr(1)),
            HirStmt::Expr(HirExpr::Dict(vec![])),
        ];
        assert!(body_uses_hashmap(&body_with_dict));

        let body_no_dict = vec![
            HirStmt::Expr(int_expr(1)),
            HirStmt::Expr(int_expr(2)),
        ];
        assert!(!body_uses_hashmap(&body_no_dict));
    }
}
