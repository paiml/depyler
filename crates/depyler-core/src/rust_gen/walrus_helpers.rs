//! Walrus Operator (Named Expression) Helpers
//!
//! This module contains helpers for handling Python's walrus operator (:=)
//! during code generation. Extracted from expr_gen.rs for better testability.
//!
//! DEPYLER-0792: Walrus operator support for Python 3.8+

use crate::hir::HirExpr;
use std::collections::HashSet;

/// Collect all variable names defined by walrus operators in conditions
/// Recursively walks the expression tree to find NamedExpr targets
pub fn collect_walrus_vars_from_conditions(conditions: &[HirExpr]) -> HashSet<String> {
    let mut vars = HashSet::new();
    for cond in conditions {
        collect_walrus_vars_from_expr(cond, &mut vars);
    }
    vars
}

/// Helper to recursively find NamedExpr (walrus) targets in an expression
pub fn collect_walrus_vars_from_expr(expr: &HirExpr, vars: &mut HashSet<String>) {
    match expr {
        HirExpr::NamedExpr { target, value } => {
            vars.insert(target.clone());
            collect_walrus_vars_from_expr(value, vars);
        }
        HirExpr::Binary { left, right, .. } => {
            collect_walrus_vars_from_expr(left, vars);
            collect_walrus_vars_from_expr(right, vars);
        }
        HirExpr::Unary { operand, .. } => {
            collect_walrus_vars_from_expr(operand, vars);
        }
        HirExpr::Call { args, kwargs, .. } => {
            for arg in args {
                collect_walrus_vars_from_expr(arg, vars);
            }
            for (_, v) in kwargs {
                collect_walrus_vars_from_expr(v, vars);
            }
        }
        HirExpr::MethodCall {
            object,
            args,
            kwargs,
            ..
        } => {
            collect_walrus_vars_from_expr(object, vars);
            for arg in args {
                collect_walrus_vars_from_expr(arg, vars);
            }
            for (_, v) in kwargs {
                collect_walrus_vars_from_expr(v, vars);
            }
        }
        HirExpr::IfExpr { test, body, orelse } => {
            collect_walrus_vars_from_expr(test, vars);
            collect_walrus_vars_from_expr(body, vars);
            collect_walrus_vars_from_expr(orelse, vars);
        }
        HirExpr::Tuple(elts) | HirExpr::List(elts) | HirExpr::Set(elts) => {
            for e in elts {
                collect_walrus_vars_from_expr(e, vars);
            }
        }
        _ => {}
    }
}

/// Check if an expression uses any of the given variable names
pub fn expr_uses_any_var(expr: &HirExpr, var_names: &HashSet<String>) -> bool {
    match expr {
        HirExpr::Var(name) => var_names.contains(name),
        HirExpr::NamedExpr { value, .. } => expr_uses_any_var(value, var_names),
        HirExpr::Binary { left, right, .. } => {
            expr_uses_any_var(left, var_names) || expr_uses_any_var(right, var_names)
        }
        HirExpr::Unary { operand, .. } => expr_uses_any_var(operand, var_names),
        HirExpr::Call { args, kwargs, .. } => {
            args.iter().any(|a| expr_uses_any_var(a, var_names))
                || kwargs.iter().any(|(_, v)| expr_uses_any_var(v, var_names))
        }
        HirExpr::MethodCall {
            object,
            args,
            kwargs,
            ..
        } => {
            expr_uses_any_var(object, var_names)
                || args.iter().any(|a| expr_uses_any_var(a, var_names))
                || kwargs.iter().any(|(_, v)| expr_uses_any_var(v, var_names))
        }
        HirExpr::Tuple(elts) | HirExpr::List(elts) | HirExpr::Set(elts) => {
            elts.iter().any(|e| expr_uses_any_var(e, var_names))
        }
        HirExpr::Index { base, index } => {
            expr_uses_any_var(base, var_names) || expr_uses_any_var(index, var_names)
        }
        HirExpr::Attribute { value, .. } => expr_uses_any_var(value, var_names),
        HirExpr::IfExpr { test, body, orelse } => {
            expr_uses_any_var(test, var_names)
                || expr_uses_any_var(body, var_names)
                || expr_uses_any_var(orelse, var_names)
        }
        _ => false,
    }
}

/// Check if expression contains a walrus operator
pub fn contains_walrus_expr(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::NamedExpr { .. } => true,
        HirExpr::Binary { left, right, .. } => {
            contains_walrus_expr(left) || contains_walrus_expr(right)
        }
        HirExpr::Unary { operand, .. } => contains_walrus_expr(operand),
        HirExpr::Call { args, kwargs, .. } => {
            args.iter().any(contains_walrus_expr)
                || kwargs.iter().any(|(_, v)| contains_walrus_expr(v))
        }
        HirExpr::MethodCall {
            object,
            args,
            kwargs,
            ..
        } => {
            contains_walrus_expr(object)
                || args.iter().any(contains_walrus_expr)
                || kwargs.iter().any(|(_, v)| contains_walrus_expr(v))
        }
        HirExpr::IfExpr { test, body, orelse } => {
            contains_walrus_expr(test) || contains_walrus_expr(body) || contains_walrus_expr(orelse)
        }
        HirExpr::Tuple(elts) | HirExpr::List(elts) | HirExpr::Set(elts) => {
            elts.iter().any(contains_walrus_expr)
        }
        HirExpr::Index { base, index } => contains_walrus_expr(base) || contains_walrus_expr(index),
        HirExpr::Attribute { value, .. } => contains_walrus_expr(value),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, Literal, UnaryOp};

    // ============ collect_walrus_vars_from_conditions tests ============

    #[test]
    fn test_collect_walrus_empty_conditions() {
        let vars = collect_walrus_vars_from_conditions(&[]);
        assert!(vars.is_empty());
    }

    #[test]
    fn test_collect_walrus_no_walrus() {
        let conditions = vec![HirExpr::Var("x".to_string())];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.is_empty());
    }

    #[test]
    fn test_collect_walrus_single_named_expr() {
        let conditions = vec![HirExpr::NamedExpr {
            target: "n".to_string(),
            value: Box::new(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("s".to_string())],
                kwargs: vec![],
            }),
        }];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("n"));
        assert_eq!(vars.len(), 1);
    }

    #[test]
    fn test_collect_walrus_nested_in_binary() {
        let conditions = vec![HirExpr::Binary {
            op: BinOp::Gt,
            left: Box::new(HirExpr::NamedExpr {
                target: "length".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(10))),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(5))),
        }];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("length"));
    }

    #[test]
    fn test_collect_walrus_in_unary() {
        let conditions = vec![HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::NamedExpr {
                target: "result".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Bool(true))),
            }),
        }];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("result"));
    }

    #[test]
    fn test_collect_walrus_in_call_args() {
        let conditions = vec![HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::NamedExpr {
                target: "msg".to_string(),
                value: Box::new(HirExpr::Literal(Literal::String("hello".to_string()))),
            }],
            kwargs: vec![],
        }];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("msg"));
    }

    #[test]
    fn test_collect_walrus_in_call_kwargs() {
        let conditions = vec![HirExpr::Call {
            func: "func".to_string(),
            args: vec![],
            kwargs: vec![(
                "key".to_string(),
                HirExpr::NamedExpr {
                    target: "val".to_string(),
                    value: Box::new(HirExpr::Literal(Literal::Int(42))),
                },
            )],
        }];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("val"));
    }

    #[test]
    fn test_collect_walrus_in_method_call() {
        let conditions = vec![HirExpr::MethodCall {
            object: Box::new(HirExpr::NamedExpr {
                target: "obj".to_string(),
                value: Box::new(HirExpr::Var("x".to_string())),
            }),
            method: "method".to_string(),
            args: vec![],
            kwargs: vec![],
        }];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("obj"));
    }

    #[test]
    fn test_collect_walrus_in_if_expr() {
        let conditions = vec![HirExpr::IfExpr {
            test: Box::new(HirExpr::NamedExpr {
                target: "cond".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Bool(true))),
            }),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        }];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("cond"));
    }

    #[test]
    fn test_collect_walrus_in_tuple() {
        let conditions = vec![HirExpr::Tuple(vec![
            HirExpr::NamedExpr {
                target: "a".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            HirExpr::NamedExpr {
                target: "b".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(2))),
            },
        ])];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("a"));
        assert!(vars.contains("b"));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_collect_walrus_in_list() {
        let conditions = vec![HirExpr::List(vec![HirExpr::NamedExpr {
            target: "item".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(1))),
        }])];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("item"));
    }

    #[test]
    fn test_collect_walrus_in_set() {
        let conditions = vec![HirExpr::Set(vec![HirExpr::NamedExpr {
            target: "elem".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(1))),
        }])];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("elem"));
    }

    #[test]
    fn test_collect_walrus_nested_walrus() {
        // n := (m := len(s))
        let conditions = vec![HirExpr::NamedExpr {
            target: "n".to_string(),
            value: Box::new(HirExpr::NamedExpr {
                target: "m".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(42))),
            }),
        }];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("n"));
        assert!(vars.contains("m"));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_collect_walrus_multiple_conditions() {
        let conditions = vec![
            HirExpr::NamedExpr {
                target: "x".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            HirExpr::NamedExpr {
                target: "y".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(2))),
            },
        ];
        let vars = collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
        assert_eq!(vars.len(), 2);
    }

    // ============ expr_uses_any_var tests ============

    #[test]
    fn test_expr_uses_var_direct_match() {
        let mut var_names = HashSet::new();
        var_names.insert("x".to_string());
        let expr = HirExpr::Var("x".to_string());
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_no_match() {
        let mut var_names = HashSet::new();
        var_names.insert("x".to_string());
        let expr = HirExpr::Var("y".to_string());
        assert!(!expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_empty_set() {
        let var_names = HashSet::new();
        let expr = HirExpr::Var("x".to_string());
        assert!(!expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_named_expr() {
        let mut var_names = HashSet::new();
        var_names.insert("x".to_string());
        let expr = HirExpr::NamedExpr {
            target: "n".to_string(),
            value: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_binary_left() {
        let mut var_names = HashSet::new();
        var_names.insert("x".to_string());
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_binary_right() {
        let mut var_names = HashSet::new();
        var_names.insert("y".to_string());
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Var("y".to_string())),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_unary() {
        let mut var_names = HashSet::new();
        var_names.insert("x".to_string());
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_call_args() {
        let mut var_names = HashSet::new();
        var_names.insert("x".to_string());
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_call_kwargs() {
        let mut var_names = HashSet::new();
        var_names.insert("val".to_string());
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![],
            kwargs: vec![("key".to_string(), HirExpr::Var("val".to_string()))],
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_method_object() {
        let mut var_names = HashSet::new();
        var_names.insert("obj".to_string());
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_tuple() {
        let mut var_names = HashSet::new();
        var_names.insert("x".to_string());
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Var("x".to_string()),
        ]);
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_list() {
        let mut var_names = HashSet::new();
        var_names.insert("item".to_string());
        let expr = HirExpr::List(vec![HirExpr::Var("item".to_string())]);
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_set() {
        let mut var_names = HashSet::new();
        var_names.insert("elem".to_string());
        let expr = HirExpr::Set(vec![HirExpr::Var("elem".to_string())]);
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_index_base() {
        let mut var_names = HashSet::new();
        var_names.insert("arr".to_string());
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_index_index() {
        let mut var_names = HashSet::new();
        var_names.insert("i".to_string());
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("i".to_string())),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_attribute() {
        let mut var_names = HashSet::new();
        var_names.insert("obj".to_string());
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_if_test() {
        let mut var_names = HashSet::new();
        var_names.insert("cond".to_string());
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_if_body() {
        let mut var_names = HashSet::new();
        var_names.insert("x".to_string());
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Var("x".to_string())),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_in_if_orelse() {
        let mut var_names = HashSet::new();
        var_names.insert("y".to_string());
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Var("y".to_string())),
        };
        assert!(expr_uses_any_var(&expr, &var_names));
    }

    #[test]
    fn test_expr_uses_var_literal() {
        let mut var_names = HashSet::new();
        var_names.insert("x".to_string());
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!expr_uses_any_var(&expr, &var_names));
    }

    // ============ contains_walrus_expr tests ============

    #[test]
    fn test_contains_walrus_direct() {
        let expr = HirExpr::NamedExpr {
            target: "n".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(42))),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_binary_left() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::NamedExpr {
                target: "n".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_binary_right() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::NamedExpr {
                target: "n".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_unary() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::NamedExpr {
                target: "n".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(42))),
            }),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_call_arg() {
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![HirExpr::NamedExpr {
                target: "n".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(42))),
            }],
            kwargs: vec![],
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_call_kwarg() {
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![],
            kwargs: vec![(
                "key".to_string(),
                HirExpr::NamedExpr {
                    target: "n".to_string(),
                    value: Box::new(HirExpr::Literal(Literal::Int(42))),
                },
            )],
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_method_object() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::NamedExpr {
                target: "obj".to_string(),
                value: Box::new(HirExpr::Var("x".to_string())),
            }),
            method: "m".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_method_args() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "m".to_string(),
            args: vec![HirExpr::NamedExpr {
                target: "n".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(42))),
            }],
            kwargs: vec![],
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_if_test() {
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::NamedExpr {
                target: "n".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Bool(true))),
            }),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_if_body() {
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::NamedExpr {
                target: "n".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            }),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_if_orelse() {
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::NamedExpr {
                target: "n".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(0))),
            }),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_tuple() {
        let expr = HirExpr::Tuple(vec![HirExpr::NamedExpr {
            target: "n".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(42))),
        }]);
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_list() {
        let expr = HirExpr::List(vec![HirExpr::NamedExpr {
            target: "n".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(42))),
        }]);
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_set() {
        let expr = HirExpr::Set(vec![HirExpr::NamedExpr {
            target: "n".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(42))),
        }]);
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_index_base() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::NamedExpr {
                target: "arr".to_string(),
                value: Box::new(HirExpr::Var("x".to_string())),
            }),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_index_index() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::NamedExpr {
                target: "i".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(0))),
            }),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_in_attribute() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::NamedExpr {
                target: "obj".to_string(),
                value: Box::new(HirExpr::Var("x".to_string())),
            }),
            attr: "field".to_string(),
        };
        assert!(contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_no_walrus_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(!contains_walrus_expr(&expr));
    }

    #[test]
    fn test_contains_walrus_no_walrus_if() {
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(!contains_walrus_expr(&expr));
    }
}
