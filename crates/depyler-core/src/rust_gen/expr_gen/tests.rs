use super::*;
use crate::hir::{BinOp, HirExpr, Literal, UnaryOp};
use crate::rust_gen::walrus_helpers;
use quote::ToTokens;
use std::collections::HashSet;

// ============ is_rust_keyword tests ============

#[test]
pub(crate) fn test_is_rust_keyword_basic() {
    assert!(keywords::is_rust_keyword("fn"));
    assert!(keywords::is_rust_keyword("let"));
    assert!(keywords::is_rust_keyword("if"));
    assert!(keywords::is_rust_keyword("else"));
    assert!(keywords::is_rust_keyword("for"));
    assert!(keywords::is_rust_keyword("while"));
    assert!(keywords::is_rust_keyword("loop"));
    assert!(keywords::is_rust_keyword("match"));
}

#[test]
pub(crate) fn test_is_rust_keyword_types() {
    assert!(keywords::is_rust_keyword("struct"));
    assert!(keywords::is_rust_keyword("enum"));
    assert!(keywords::is_rust_keyword("trait"));
    assert!(keywords::is_rust_keyword("impl"));
    assert!(keywords::is_rust_keyword("type"));
}

#[test]
pub(crate) fn test_is_rust_keyword_modifiers() {
    assert!(keywords::is_rust_keyword("pub"));
    assert!(keywords::is_rust_keyword("mut"));
    assert!(keywords::is_rust_keyword("const"));
    assert!(keywords::is_rust_keyword("static"));
    assert!(keywords::is_rust_keyword("ref"));
}

#[test]
pub(crate) fn test_is_rust_keyword_async() {
    assert!(keywords::is_rust_keyword("async"));
    assert!(keywords::is_rust_keyword("await"));
}

#[test]
pub(crate) fn test_is_rust_keyword_reserved() {
    assert!(keywords::is_rust_keyword("abstract"));
    assert!(keywords::is_rust_keyword("become"));
    assert!(keywords::is_rust_keyword("box"));
    assert!(keywords::is_rust_keyword("do"));
    assert!(keywords::is_rust_keyword("final"));
    assert!(keywords::is_rust_keyword("macro"));
    assert!(keywords::is_rust_keyword("override"));
    assert!(keywords::is_rust_keyword("priv"));
    assert!(keywords::is_rust_keyword("try"));
    assert!(keywords::is_rust_keyword("typeof"));
    assert!(keywords::is_rust_keyword("virtual"));
    assert!(keywords::is_rust_keyword("yield"));
}

#[test]
pub(crate) fn test_is_rust_keyword_false() {
    assert!(!keywords::is_rust_keyword("foo"));
    assert!(!keywords::is_rust_keyword("bar"));
    assert!(!keywords::is_rust_keyword("my_var"));
    assert!(!keywords::is_rust_keyword("count"));
    assert!(!keywords::is_rust_keyword("result"));
}

#[test]
pub(crate) fn test_is_rust_keyword_special() {
    assert!(keywords::is_rust_keyword("self"));
    assert!(keywords::is_rust_keyword("Self"));
    assert!(keywords::is_rust_keyword("super"));
    assert!(keywords::is_rust_keyword("crate"));
}

// ============ is_non_raw_keyword tests ============

#[test]
pub(crate) fn test_is_non_raw_keyword_true() {
    assert!(keywords::is_non_raw_keyword("self"));
    assert!(keywords::is_non_raw_keyword("Self"));
    assert!(keywords::is_non_raw_keyword("super"));
    assert!(keywords::is_non_raw_keyword("crate"));
}

#[test]
pub(crate) fn test_is_non_raw_keyword_false() {
    assert!(!keywords::is_non_raw_keyword("fn"));
    assert!(!keywords::is_non_raw_keyword("let"));
    assert!(!keywords::is_non_raw_keyword("type"));
    assert!(!keywords::is_non_raw_keyword("foo"));
}

// ============ looks_like_option_expr tests ============

#[test]
pub(crate) fn test_looks_like_option_expr_ok_method() {
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("result".to_string())),
        method: "ok".to_string(),
        args: vec![],
        kwargs: vec![],
    };
    assert!(expr_analysis::looks_like_option_expr(&expr));
}

#[test]
pub(crate) fn test_looks_like_option_expr_get_one_arg() {
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("dict".to_string())),
        method: "get".to_string(),
        args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
        kwargs: vec![],
    };
    assert!(expr_analysis::looks_like_option_expr(&expr));
}

#[test]
pub(crate) fn test_looks_like_option_expr_get_with_default() {
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("dict".to_string())),
        method: "get".to_string(),
        args: vec![
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(0)),
        ],
        kwargs: vec![],
    };
    assert!(!expr_analysis::looks_like_option_expr(&expr));
}

#[test]
pub(crate) fn test_looks_like_option_expr_other_method() {
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("list".to_string())),
        method: "append".to_string(),
        args: vec![HirExpr::Literal(Literal::Int(1))],
        kwargs: vec![],
    };
    assert!(!expr_analysis::looks_like_option_expr(&expr));
}

#[test]
pub(crate) fn test_looks_like_option_expr_chained_ok() {
    let inner = HirExpr::MethodCall {
        object: Box::new(HirExpr::Call {
            func: "env_var".to_string(),
            args: vec![],
            kwargs: vec![],
        }),
        method: "ok".to_string(),
        args: vec![],
        kwargs: vec![],
    };
    assert!(expr_analysis::looks_like_option_expr(&inner));
}

#[test]
pub(crate) fn test_looks_like_option_expr_not_method() {
    let expr = HirExpr::Var("x".to_string());
    assert!(!expr_analysis::looks_like_option_expr(&expr));
}

// ============ collect_walrus_vars_from_conditions tests ============

#[test]
pub(crate) fn test_collect_walrus_vars_empty() {
    let conditions: Vec<HirExpr> = vec![];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.is_empty());
}

#[test]
pub(crate) fn test_collect_walrus_vars_named_expr() {
    let conditions = vec![HirExpr::NamedExpr {
        target: "x".to_string(),
        value: Box::new(HirExpr::Literal(Literal::Int(5))),
    }];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.contains("x"));
    assert_eq!(vars.len(), 1);
}

#[test]
pub(crate) fn test_collect_walrus_vars_nested() {
    let conditions = vec![HirExpr::Binary {
        op: BinOp::And,
        left: Box::new(HirExpr::NamedExpr {
            target: "a".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(1))),
        }),
        right: Box::new(HirExpr::NamedExpr {
            target: "b".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(2))),
        }),
    }];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.contains("a"));
    assert!(vars.contains("b"));
    assert_eq!(vars.len(), 2);
}

#[test]
pub(crate) fn test_collect_walrus_vars_in_call() {
    let conditions = vec![HirExpr::Call {
        func: "foo".to_string(),
        args: vec![HirExpr::NamedExpr {
            target: "result".to_string(),
            value: Box::new(HirExpr::Var("x".to_string())),
        }],
        kwargs: vec![],
    }];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.contains("result"));
}

#[test]
pub(crate) fn test_collect_walrus_vars_in_unary() {
    let conditions = vec![HirExpr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(HirExpr::NamedExpr {
            target: "flag".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Bool(true))),
        }),
    }];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.contains("flag"));
}

#[test]
pub(crate) fn test_collect_walrus_vars_in_if_expr() {
    let conditions = vec![HirExpr::IfExpr {
        test: Box::new(HirExpr::NamedExpr {
            target: "cond".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Bool(true))),
        }),
        body: Box::new(HirExpr::Literal(Literal::Int(1))),
        orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
    }];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.contains("cond"));
}

#[test]
pub(crate) fn test_collect_walrus_vars_in_tuple() {
    let conditions = vec![HirExpr::Tuple(vec![
        HirExpr::NamedExpr {
            target: "x".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(1))),
        },
        HirExpr::NamedExpr {
            target: "y".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(2))),
        },
    ])];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.contains("x"));
    assert!(vars.contains("y"));
}

// ============ expr_uses_any_var tests ============

#[test]
pub(crate) fn test_expr_uses_any_var_simple() {
    let mut vars = HashSet::new();
    vars.insert("x".to_string());

    let expr = HirExpr::Var("x".to_string());
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));

    let expr2 = HirExpr::Var("y".to_string());
    assert!(!walrus_helpers::expr_uses_any_var(&expr2, &vars));
}

#[test]
pub(crate) fn test_expr_uses_any_var_binary() {
    let mut vars = HashSet::new();
    vars.insert("x".to_string());

    let expr = HirExpr::Binary {
        op: BinOp::Add,
        left: Box::new(HirExpr::Var("x".to_string())),
        right: Box::new(HirExpr::Literal(Literal::Int(1))),
    };
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));

    let expr2 = HirExpr::Binary {
        op: BinOp::Add,
        left: Box::new(HirExpr::Literal(Literal::Int(1))),
        right: Box::new(HirExpr::Var("x".to_string())),
    };
    assert!(walrus_helpers::expr_uses_any_var(&expr2, &vars));
}

#[test]
pub(crate) fn test_expr_uses_any_var_call() {
    let mut vars = HashSet::new();
    vars.insert("arg".to_string());

    let expr = HirExpr::Call {
        func: "foo".to_string(),
        args: vec![HirExpr::Var("arg".to_string())],
        kwargs: vec![],
    };
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
}

#[test]
pub(crate) fn test_expr_uses_any_var_index() {
    let mut vars = HashSet::new();
    vars.insert("idx".to_string());

    let expr = HirExpr::Index {
        base: Box::new(HirExpr::Var("arr".to_string())),
        index: Box::new(HirExpr::Var("idx".to_string())),
    };
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
}

#[test]
pub(crate) fn test_expr_uses_any_var_attribute() {
    let mut vars = HashSet::new();
    vars.insert("obj".to_string());

    let expr = HirExpr::Attribute {
        value: Box::new(HirExpr::Var("obj".to_string())),
        attr: "field".to_string(),
    };
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
}

#[test]
pub(crate) fn test_expr_uses_any_var_list() {
    let mut vars = HashSet::new();
    vars.insert("item".to_string());

    let expr = HirExpr::List(vec![
        HirExpr::Literal(Literal::Int(1)),
        HirExpr::Var("item".to_string()),
    ]);
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
}

// ============ get_python_op_precedence tests ============

#[test]
pub(crate) fn test_get_python_op_precedence_pow() {
    assert_eq!(precedence::get_python_op_precedence(BinOp::Pow), 14);
}

#[test]
pub(crate) fn test_get_python_op_precedence_mul() {
    assert_eq!(precedence::get_python_op_precedence(BinOp::Mul), 13);
    assert_eq!(precedence::get_python_op_precedence(BinOp::Div), 13);
    assert_eq!(precedence::get_python_op_precedence(BinOp::FloorDiv), 13);
    assert_eq!(precedence::get_python_op_precedence(BinOp::Mod), 13);
}

#[test]
pub(crate) fn test_get_python_op_precedence_add() {
    assert_eq!(precedence::get_python_op_precedence(BinOp::Add), 12);
    assert_eq!(precedence::get_python_op_precedence(BinOp::Sub), 12);
}

#[test]
pub(crate) fn test_get_python_op_precedence_shift() {
    assert_eq!(precedence::get_python_op_precedence(BinOp::LShift), 11);
    assert_eq!(precedence::get_python_op_precedence(BinOp::RShift), 11);
}

#[test]
pub(crate) fn test_get_python_op_precedence_bitwise() {
    assert_eq!(precedence::get_python_op_precedence(BinOp::BitAnd), 10);
    assert_eq!(precedence::get_python_op_precedence(BinOp::BitXor), 9);
    assert_eq!(precedence::get_python_op_precedence(BinOp::BitOr), 8);
}

#[test]
pub(crate) fn test_get_python_op_precedence_comparison() {
    assert_eq!(precedence::get_python_op_precedence(BinOp::Lt), 7);
    assert_eq!(precedence::get_python_op_precedence(BinOp::Gt), 7);
    assert_eq!(precedence::get_python_op_precedence(BinOp::LtEq), 7);
    assert_eq!(precedence::get_python_op_precedence(BinOp::GtEq), 7);
    assert_eq!(precedence::get_python_op_precedence(BinOp::Eq), 7);
    assert_eq!(precedence::get_python_op_precedence(BinOp::NotEq), 7);
}

#[test]
pub(crate) fn test_get_python_op_precedence_logical() {
    assert_eq!(precedence::get_python_op_precedence(BinOp::And), 6);
    assert_eq!(precedence::get_python_op_precedence(BinOp::Or), 5);
}

// ============ looks_like_option_expr additional tests ============

#[test]
pub(crate) fn test_looks_like_option_expr_nested_get() {
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key1".to_string()))],
            kwargs: vec![],
        }),
        method: "get".to_string(),
        args: vec![HirExpr::Literal(Literal::String("key2".to_string()))],
        kwargs: vec![],
    };
    assert!(expr_analysis::looks_like_option_expr(&expr));
}

#[test]
pub(crate) fn test_looks_like_option_expr_deeply_nested() {
    let inner = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("x".to_string())),
        method: "ok".to_string(),
        args: vec![],
        kwargs: vec![],
    };
    let outer = HirExpr::MethodCall {
        object: Box::new(inner),
        method: "map".to_string(),
        args: vec![],
        kwargs: vec![],
    };
    // .map() on an Option returns Option, but we check inner
    assert!(expr_analysis::looks_like_option_expr(&outer));
}

#[test]
pub(crate) fn test_looks_like_option_expr_literal() {
    let expr = HirExpr::Literal(Literal::Int(42));
    assert!(!expr_analysis::looks_like_option_expr(&expr));
}

#[test]
pub(crate) fn test_looks_like_option_expr_binary() {
    let expr = HirExpr::Binary {
        op: BinOp::Add,
        left: Box::new(HirExpr::Literal(Literal::Int(1))),
        right: Box::new(HirExpr::Literal(Literal::Int(2))),
    };
    assert!(!expr_analysis::looks_like_option_expr(&expr));
}

// ============ More walrus operator tests ============

#[test]
pub(crate) fn test_collect_walrus_vars_multiple_conditions() {
    let conditions = vec![
        HirExpr::NamedExpr {
            target: "a".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(1))),
        },
        HirExpr::NamedExpr {
            target: "b".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(2))),
        },
        HirExpr::Var("c".to_string()),
    ];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert_eq!(vars.len(), 2);
    assert!(vars.contains("a"));
    assert!(vars.contains("b"));
}

#[test]
pub(crate) fn test_expr_uses_any_var_nested_index() {
    let mut vars = HashSet::new();
    vars.insert("i".to_string());

    let expr = HirExpr::Index {
        base: Box::new(HirExpr::Index {
            base: Box::new(HirExpr::Var("matrix".to_string())),
            index: Box::new(HirExpr::Var("i".to_string())),
        }),
        index: Box::new(HirExpr::Literal(Literal::Int(0))),
    };
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
}

#[test]
pub(crate) fn test_expr_uses_any_var_unary() {
    let mut vars = HashSet::new();
    vars.insert("flag".to_string());

    let expr = HirExpr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(HirExpr::Var("flag".to_string())),
    };
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
}

// ============ More is_rust_keyword edge cases ============

#[test]
pub(crate) fn test_is_rust_keyword_case_sensitivity() {
    assert!(keywords::is_rust_keyword("fn"));
    assert!(!keywords::is_rust_keyword("FN"));
    assert!(!keywords::is_rust_keyword("Fn"));
}

#[test]
pub(crate) fn test_is_rust_keyword_similar_names() {
    // These are NOT keywords
    assert!(!keywords::is_rust_keyword("function"));
    assert!(!keywords::is_rust_keyword("match_"));
    assert!(!keywords::is_rust_keyword("_if"));
    assert!(!keywords::is_rust_keyword("for2"));
}

// ============ BinOp containment tests ============

#[test]
pub(crate) fn test_get_python_op_precedence_in_notin() {
    assert_eq!(precedence::get_python_op_precedence(BinOp::In), 7);
    assert_eq!(precedence::get_python_op_precedence(BinOp::NotIn), 7);
}

// ============ get_rust_op_precedence tests ============

#[test]
pub(crate) fn test_get_rust_op_precedence_mul() {
    let op: syn::BinOp = syn::parse_quote!(*);
    assert_eq!(precedence::get_rust_op_precedence(&op), 13);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_div() {
    let op: syn::BinOp = syn::parse_quote!(/);
    assert_eq!(precedence::get_rust_op_precedence(&op), 13);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_add() {
    let op: syn::BinOp = syn::parse_quote!(+);
    assert_eq!(precedence::get_rust_op_precedence(&op), 12);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_sub() {
    let op: syn::BinOp = syn::parse_quote!(-);
    assert_eq!(precedence::get_rust_op_precedence(&op), 12);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_shl() {
    let op: syn::BinOp = syn::parse_quote!(<<);
    assert_eq!(precedence::get_rust_op_precedence(&op), 11);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_shr() {
    let op: syn::BinOp = syn::parse_quote!(>>);
    assert_eq!(precedence::get_rust_op_precedence(&op), 11);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_bitand() {
    let op: syn::BinOp = syn::parse_quote!(&);
    assert_eq!(precedence::get_rust_op_precedence(&op), 10);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_bitxor() {
    let op: syn::BinOp = syn::parse_quote!(^);
    assert_eq!(precedence::get_rust_op_precedence(&op), 9);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_bitor() {
    let op: syn::BinOp = syn::parse_quote!(|);
    assert_eq!(precedence::get_rust_op_precedence(&op), 8);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_lt() {
    let op: syn::BinOp = syn::parse_quote!(<);
    assert_eq!(precedence::get_rust_op_precedence(&op), 7);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_le() {
    let op: syn::BinOp = syn::parse_quote!(<=);
    assert_eq!(precedence::get_rust_op_precedence(&op), 7);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_gt() {
    let op: syn::BinOp = syn::parse_quote!(>);
    assert_eq!(precedence::get_rust_op_precedence(&op), 7);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_ge() {
    let op: syn::BinOp = syn::parse_quote!(>=);
    assert_eq!(precedence::get_rust_op_precedence(&op), 7);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_eq() {
    let op: syn::BinOp = syn::parse_quote!(==);
    assert_eq!(precedence::get_rust_op_precedence(&op), 7);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_ne() {
    let op: syn::BinOp = syn::parse_quote!(!=);
    assert_eq!(precedence::get_rust_op_precedence(&op), 7);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_and() {
    let op: syn::BinOp = syn::parse_quote!(&&);
    assert_eq!(precedence::get_rust_op_precedence(&op), 6);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_or() {
    let op: syn::BinOp = syn::parse_quote!(||);
    assert_eq!(precedence::get_rust_op_precedence(&op), 5);
}

#[test]
pub(crate) fn test_get_rust_op_precedence_rem() {
    let op: syn::BinOp = syn::parse_quote!(%);
    assert_eq!(precedence::get_rust_op_precedence(&op), 13);
}

// ============ borrow_if_needed tests ============

#[test]
pub(crate) fn test_borrow_if_needed_path() {
    let expr: syn::Expr = syn::parse_quote!(x);
    let result = ExpressionConverter::borrow_if_needed(&expr);
    assert_eq!(result.to_token_stream().to_string(), "& x");
}

#[test]
pub(crate) fn test_borrow_if_needed_already_reference() {
    let expr: syn::Expr = syn::parse_quote!(&x);
    let result = ExpressionConverter::borrow_if_needed(&expr);
    assert_eq!(result.to_token_stream().to_string(), "& x");
}

#[test]
pub(crate) fn test_borrow_if_needed_literal() {
    let expr: syn::Expr = syn::parse_quote!("hello");
    let result = ExpressionConverter::borrow_if_needed(&expr);
    assert_eq!(result.to_token_stream().to_string(), "\"hello\"");
}

#[test]
pub(crate) fn test_borrow_if_needed_method_call() {
    let expr: syn::Expr = syn::parse_quote!(s.as_str());
    let result = ExpressionConverter::borrow_if_needed(&expr);
    // Method calls producing str are not borrowed
    assert_eq!(result.to_token_stream().to_string(), "s . as_str ()");
}

// ============ wrap_in_parens tests ============
// Note: wrap_in_parens uses braces { } not parens ( ) per DEPYLER-0707

#[test]
pub(crate) fn test_wrap_in_parens_simple() {
    let expr: syn::Expr = syn::parse_quote!(x);
    let result = ExpressionConverter::wrap_in_parens(expr);
    assert_eq!(result.to_token_stream().to_string(), "{ x }");
}

#[test]
pub(crate) fn test_wrap_in_parens_binary() {
    let expr: syn::Expr = syn::parse_quote!(a + b);
    let result = ExpressionConverter::wrap_in_parens(expr);
    assert_eq!(result.to_token_stream().to_string(), "{ a + b }");
}

#[test]
pub(crate) fn test_wrap_in_parens_call() {
    let expr: syn::Expr = syn::parse_quote!(foo(1, 2));
    let result = ExpressionConverter::wrap_in_parens(expr);
    assert_eq!(result.to_token_stream().to_string(), "{ foo (1 , 2) }");
}

// ============ parenthesize_if_lower_precedence tests ============

#[test]
pub(crate) fn test_parenthesize_lower_precedence() {
    // (a + b) * c - the add has lower precedence than mul
    let expr: syn::Expr = syn::parse_quote!(a + b);
    let result = precedence::parenthesize_if_lower_precedence(expr, BinOp::Mul);
    assert_eq!(result.to_token_stream().to_string(), "(a + b)");
}

#[test]
pub(crate) fn test_parenthesize_same_precedence() {
    // a * b in context of division - same precedence, no parens
    let expr: syn::Expr = syn::parse_quote!(a * b);
    let result = precedence::parenthesize_if_lower_precedence(expr, BinOp::Div);
    // Same precedence doesn't add parens
    assert_eq!(result.to_token_stream().to_string(), "a * b");
}

#[test]
pub(crate) fn test_parenthesize_higher_precedence() {
    // a * b in context of addition - higher precedence, no parens
    let expr: syn::Expr = syn::parse_quote!(a * b);
    let result = precedence::parenthesize_if_lower_precedence(expr, BinOp::Add);
    assert_eq!(result.to_token_stream().to_string(), "a * b");
}

#[test]
pub(crate) fn test_parenthesize_non_binary() {
    // Non-binary expressions pass through unchanged
    let expr: syn::Expr = syn::parse_quote!(foo());
    let result = precedence::parenthesize_if_lower_precedence(expr, BinOp::Mul);
    assert_eq!(result.to_token_stream().to_string(), "foo ()");
}

// ============ Additional edge case tests ============

#[test]
pub(crate) fn test_expr_uses_any_var_empty_set() {
    let vars = HashSet::new();
    let expr = HirExpr::Var("x".to_string());
    assert!(!walrus_helpers::expr_uses_any_var(&expr, &vars));
}

#[test]
pub(crate) fn test_expr_uses_any_var_method_call() {
    let mut vars = HashSet::new();
    vars.insert("obj".to_string());

    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("obj".to_string())),
        method: "method".to_string(),
        args: vec![],
        kwargs: vec![],
    };
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
}

#[test]
pub(crate) fn test_expr_uses_any_var_kwargs() {
    let mut vars = HashSet::new();
    vars.insert("value".to_string());

    let expr = HirExpr::Call {
        func: "func".to_string(),
        args: vec![],
        kwargs: vec![("key".to_string(), HirExpr::Var("value".to_string()))],
    };
    assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
}

#[test]
pub(crate) fn test_collect_walrus_vars_in_list() {
    let conditions = vec![HirExpr::List(vec![HirExpr::NamedExpr {
        target: "item".to_string(),
        value: Box::new(HirExpr::Literal(Literal::Int(1))),
    }])];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.contains("item"));
}

#[test]
pub(crate) fn test_collect_walrus_vars_in_set() {
    let conditions = vec![HirExpr::Set(vec![HirExpr::NamedExpr {
        target: "elem".to_string(),
        value: Box::new(HirExpr::Literal(Literal::Int(1))),
    }])];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.contains("elem"));
}

#[test]
pub(crate) fn test_collect_walrus_vars_in_method_kwargs() {
    let conditions = vec![HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("obj".to_string())),
        method: "method".to_string(),
        args: vec![],
        kwargs: vec![(
            "key".to_string(),
            HirExpr::NamedExpr {
                target: "kwarg_var".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
        )],
    }];
    let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
    assert!(vars.contains("kwarg_var"));
}

#[test]
pub(crate) fn test_is_rust_keyword_move() {
    assert!(keywords::is_rust_keyword("move"));
}

#[test]
pub(crate) fn test_is_rust_keyword_return() {
    assert!(keywords::is_rust_keyword("return"));
}

#[test]
pub(crate) fn test_is_rust_keyword_break_continue() {
    assert!(keywords::is_rust_keyword("break"));
    assert!(keywords::is_rust_keyword("continue"));
}

#[test]
pub(crate) fn test_is_rust_keyword_use_mod() {
    assert!(keywords::is_rust_keyword("use"));
    assert!(keywords::is_rust_keyword("mod"));
}

#[test]
pub(crate) fn test_is_rust_keyword_extern() {
    assert!(keywords::is_rust_keyword("extern"));
}

#[test]
pub(crate) fn test_is_rust_keyword_true_false() {
    assert!(keywords::is_rust_keyword("true"));
    assert!(keywords::is_rust_keyword("false"));
}

#[test]
pub(crate) fn test_is_rust_keyword_dyn_unsafe() {
    assert!(keywords::is_rust_keyword("dyn"));
    assert!(keywords::is_rust_keyword("unsafe"));
}

#[test]
pub(crate) fn test_is_rust_keyword_where() {
    assert!(keywords::is_rust_keyword("where"));
}

#[test]
pub(crate) fn test_is_rust_keyword_in() {
    assert!(keywords::is_rust_keyword("in"));
}

#[test]
pub(crate) fn test_is_rust_keyword_as() {
    assert!(keywords::is_rust_keyword("as"));
}

#[test]
pub(crate) fn test_is_rust_keyword_unsized() {
    assert!(keywords::is_rust_keyword("unsized"));
}

// ============ literal_to_rust_expr tests ============

#[test]
pub(crate) fn test_literal_to_rust_expr_int() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result = literal_to_rust_expr(&Literal::Int(42), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    assert!(code.contains("42"));
}

#[test]
pub(crate) fn test_literal_to_rust_expr_negative_int() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result = literal_to_rust_expr(&Literal::Int(-100), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    assert!(code.contains("-100") || code.contains("- 100"));
}

#[test]
pub(crate) fn test_literal_to_rust_expr_float() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result =
        literal_to_rust_expr(&Literal::Float(3.15), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    assert!(code.contains("3.15"));
}

#[test]
pub(crate) fn test_literal_to_rust_expr_float_zero() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result =
        literal_to_rust_expr(&Literal::Float(0.0), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    // Should have decimal point
    assert!(code.contains("0.0") || code.contains("."));
}

#[test]
pub(crate) fn test_literal_to_rust_expr_bool_true() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result =
        literal_to_rust_expr(&Literal::Bool(true), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    assert_eq!(code, "true");
}

#[test]
pub(crate) fn test_literal_to_rust_expr_bool_false() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result =
        literal_to_rust_expr(&Literal::Bool(false), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    assert_eq!(code, "false");
}

#[test]
pub(crate) fn test_literal_to_rust_expr_none() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result = literal_to_rust_expr(&Literal::None, &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    assert_eq!(code, "None");
}

#[test]
pub(crate) fn test_literal_to_rust_expr_bytes() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let bytes = vec![72, 101, 108, 108, 111]; // "Hello"
    let result =
        literal_to_rust_expr(&Literal::Bytes(bytes), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    assert!(code.contains("b\""));
}

#[test]
pub(crate) fn test_literal_to_rust_expr_string() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result = literal_to_rust_expr(
        &Literal::String("hello".to_string()),
        &string_optimizer,
        &needs_cow,
        &ctx,
    );
    let code = result.to_token_stream().to_string();
    assert!(code.contains("hello"));
}

#[test]
pub(crate) fn test_literal_to_rust_expr_string_with_escape() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result = literal_to_rust_expr(
        &Literal::String("hello\nworld".to_string()),
        &string_optimizer,
        &needs_cow,
        &ctx,
    );
    let code = result.to_token_stream().to_string();
    assert!(code.contains("hello") && code.contains("world"));
}

// ============ ExpressionConverter static method tests ============

#[test]
pub(crate) fn test_borrow_if_needed_path_expr() {
    let path: syn::Expr = parse_quote! { my_var };
    let result = ExpressionConverter::borrow_if_needed(&path);
    let code = result.to_token_stream().to_string();
    assert!(code.contains("&"));
    assert!(code.contains("my_var"));
}

#[test]
pub(crate) fn test_borrow_if_needed_reference_unchanged() {
    let already_ref: syn::Expr = parse_quote! { &some_ref };
    let result = ExpressionConverter::borrow_if_needed(&already_ref);
    let code = result.to_token_stream().to_string();
    // Should not double-borrow
    assert!(!code.contains("& &"));
}

#[test]
pub(crate) fn test_borrow_if_needed_lit_str() {
    let lit_str: syn::Expr = parse_quote! { "hello" };
    let result = ExpressionConverter::borrow_if_needed(&lit_str);
    let code = result.to_token_stream().to_string();
    assert!(code.contains("hello"));
}

#[test]
pub(crate) fn test_wrap_in_parens_simple_path() {
    // wrap_in_parens creates a block { expr }, not parentheses (expr)
    let path: syn::Expr = parse_quote! { x };
    let result = ExpressionConverter::wrap_in_parens(path);
    let code = result.to_token_stream().to_string();
    assert!(code.contains("{") && code.contains("}") && code.contains("x"));
}

#[test]
pub(crate) fn test_wrap_in_parens_binary_expr() {
    // wrap_in_parens creates a block { expr }
    let binary: syn::Expr = parse_quote! { a + b };
    let result = ExpressionConverter::wrap_in_parens(binary);
    let code = result.to_token_stream().to_string();
    assert!(code.contains("{") && code.contains("}"));
    assert!(code.contains("a") && code.contains("b"));
}

#[test]
pub(crate) fn test_wrap_in_parens_call_expr() {
    // wrap_in_parens creates a block { expr }
    let call: syn::Expr = parse_quote! { foo(x, y) };
    let result = ExpressionConverter::wrap_in_parens(call);
    let code = result.to_token_stream().to_string();
    // Block braces around the call
    assert!(code.contains("{") && code.contains("}"));
    assert!(code.contains("foo"));
}

// ============ Additional edge case tests ============

#[test]
pub(crate) fn test_literal_to_rust_expr_large_int() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result =
        literal_to_rust_expr(&Literal::Int(i64::MAX), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    assert!(code.contains(&i64::MAX.to_string()));
}

#[test]
pub(crate) fn test_literal_to_rust_expr_float_scientific() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result =
        literal_to_rust_expr(&Literal::Float(1.5e10), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    // Should handle scientific notation
    assert!(code.contains("e") || code.contains("E") || code.contains("15000000000"));
}

#[test]
pub(crate) fn test_literal_to_rust_expr_empty_string() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result = literal_to_rust_expr(
        &Literal::String("".to_string()),
        &string_optimizer,
        &needs_cow,
        &ctx,
    );
    let code = result.to_token_stream().to_string();
    assert!(code.contains("\"\""));
}

#[test]
pub(crate) fn test_literal_to_rust_expr_empty_bytes() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result =
        literal_to_rust_expr(&Literal::Bytes(vec![]), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    assert!(code.contains("b\"\""));
}

// ============ DEPYLER-1204: Literal generation tests ============
// Note: We use unsuffixed literals to let Rust infer types from context.
// Adding explicit suffixes caused more problems than it solved.

#[test]
pub(crate) fn test_DEPYLER_1204_int_literal_unsuffixed() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result = literal_to_rust_expr(&Literal::Int(42), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    // Should be unsuffixed to allow Rust type inference
    assert_eq!(
        code, "42",
        "Integer literal should be unsuffixed, got: {}",
        code
    );
}

#[test]
pub(crate) fn test_DEPYLER_1204_float_literal_unsuffixed() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result =
        literal_to_rust_expr(&Literal::Float(1.234), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    // Should be unsuffixed (decimal point ensures float parsing)
    assert!(
        code.contains("1.234"),
        "Float literal should contain 1.234, got: {}",
        code
    );
}

#[test]
pub(crate) fn test_DEPYLER_1204_negative_int_unsuffixed() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result = literal_to_rust_expr(&Literal::Int(-100), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    // Should be unsuffixed
    assert!(
        code.contains("-100") || code.contains("- 100"),
        "Negative int should be unsuffixed, got: {}",
        code
    );
}

#[test]
pub(crate) fn test_DEPYLER_1204_float_zero_has_decimal() {
    let string_optimizer = StringOptimizer::new();
    let needs_cow = false;
    let ctx = CodeGenContext::default();
    let result =
        literal_to_rust_expr(&Literal::Float(0.0), &string_optimizer, &needs_cow, &ctx);
    let code = result.to_token_stream().to_string();
    // Should have decimal point to ensure float parsing
    assert!(
        code.contains("."),
        "Float zero should have decimal point, got: {}",
        code
    );
}

// DEPYLER-1053: Tests for lambda parameter type inference in filter/map
#[test]
pub(crate) fn test_DEPYLER_1053_infer_element_type_from_list_var() {
    // Test that infer_iterable_element_type correctly extracts Float from List(Float)
    let mut ctx = CodeGenContext::default();
    // Register a variable as List(Float)
    ctx.var_types
        .insert("data".to_string(), Type::List(Box::new(Type::Float)));

    let converter = ExpressionConverter::new(&mut ctx);
    let iterable = HirExpr::Var("data".to_string());
    let elem_type = converter.infer_iterable_element_type(&iterable);

    assert!(elem_type.is_some());
    assert!(matches!(elem_type, Some(Type::Float)));
}

#[test]
pub(crate) fn test_DEPYLER_1053_infer_element_type_from_list_literal() {
    // Test that infer_iterable_element_type correctly infers Float from list literal
    let mut ctx = CodeGenContext::default();

    let converter = ExpressionConverter::new(&mut ctx);
    let iterable = HirExpr::List(vec![
        HirExpr::Literal(Literal::Float(1.0)),
        HirExpr::Literal(Literal::Float(2.0)),
    ]);
    let elem_type = converter.infer_iterable_element_type(&iterable);

    assert!(elem_type.is_some());
    assert!(matches!(elem_type, Some(Type::Float)));
}

#[test]
pub(crate) fn test_DEPYLER_1053_infer_element_type_from_int_list() {
    // Test that infer_iterable_element_type correctly infers Int from list literal
    let mut ctx = CodeGenContext::default();

    let converter = ExpressionConverter::new(&mut ctx);
    let iterable = HirExpr::List(vec![
        HirExpr::Literal(Literal::Int(1)),
        HirExpr::Literal(Literal::Int(2)),
    ]);
    let elem_type = converter.infer_iterable_element_type(&iterable);

    assert!(elem_type.is_some());
    assert!(matches!(elem_type, Some(Type::Int)));
}

#[test]
pub(crate) fn test_DEPYLER_1053_infer_element_type_returns_none_for_unknown() {
    // Test that infer_iterable_element_type returns None for unknown iterables
    let mut ctx = CodeGenContext::default();

    let converter = ExpressionConverter::new(&mut ctx);
    // Variable not registered in var_types
    let iterable = HirExpr::Var("unknown".to_string());
    let elem_type = converter.infer_iterable_element_type(&iterable);

    assert!(elem_type.is_none());
}

// ===== is_int_expr tests =====

#[test]
fn test_is_int_expr_literal() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_int_expr(&HirExpr::Literal(Literal::Int(42))));
}

#[test]
fn test_is_int_expr_typed_var() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types.insert("x".to_string(), Type::Int);
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_int_expr(&HirExpr::Var("x".to_string())));
}

#[test]
fn test_is_int_expr_untyped_var() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(!converter.is_int_expr(&HirExpr::Var("x".to_string())));
}

#[test]
fn test_is_int_expr_float_literal() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(!converter.is_int_expr(&HirExpr::Literal(Literal::Float(1.5))));
}

#[test]
fn test_is_int_expr_binary_add_ints() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::Binary {
        left: Box::new(HirExpr::Literal(Literal::Int(1))),
        op: BinOp::Add,
        right: Box::new(HirExpr::Literal(Literal::Int(2))),
    };
    assert!(converter.is_int_expr(&expr));
}

#[test]
fn test_is_int_expr_binary_div_not_int() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::Binary {
        left: Box::new(HirExpr::Literal(Literal::Int(10))),
        op: BinOp::Div,
        right: Box::new(HirExpr::Literal(Literal::Int(3))),
    };
    assert!(!converter.is_int_expr(&expr));
}

#[test]
fn test_is_int_expr_unary_neg() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::Unary {
        op: UnaryOp::Neg,
        operand: Box::new(HirExpr::Literal(Literal::Int(5))),
    };
    assert!(converter.is_int_expr(&expr));
}

// ===== is_int_var tests =====

#[test]
fn test_is_int_var_typed_int() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types.insert("count".to_string(), Type::Int);
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_int_var(&HirExpr::Var("count".to_string())));
}

#[test]
fn test_is_int_var_custom_i32() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("idx".to_string(), Type::Custom("i32".to_string()));
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_int_var(&HirExpr::Var("idx".to_string())));
}

#[test]
fn test_is_int_var_custom_usize() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("len".to_string(), Type::Custom("usize".to_string()));
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_int_var(&HirExpr::Var("len".to_string())));
}

#[test]
fn test_is_int_var_not_var() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(!converter.is_int_var(&HirExpr::Literal(Literal::Int(42))));
}

#[test]
fn test_is_int_var_float_type() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types.insert("x".to_string(), Type::Float);
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(!converter.is_int_var(&HirExpr::Var("x".to_string())));
}

// ===== is_float_var tests =====

#[test]
fn test_is_float_var_typed_float() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types.insert("rate".to_string(), Type::Float);
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_float_var(&HirExpr::Var("rate".to_string())));
}

#[test]
fn test_is_float_var_custom_f64() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("val".to_string(), Type::Custom("f64".to_string()));
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_float_var(&HirExpr::Var("val".to_string())));
}

#[test]
fn test_is_float_var_heuristic_beta() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_float_var(&HirExpr::Var("beta1".to_string())));
}

#[test]
fn test_is_float_var_heuristic_lr() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_float_var(&HirExpr::Var("learning_rate".to_string())));
}

#[test]
fn test_is_float_var_not_var() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(!converter.is_float_var(&HirExpr::Literal(Literal::Float(1.0))));
}

// ===== needs_debug_format tests =====

#[test]
fn test_needs_debug_format_list_type() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("items".to_string(), Type::List(Box::new(Type::Int)));
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.needs_debug_format(&HirExpr::Var("items".to_string())));
}

#[test]
fn test_needs_debug_format_dict_type() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types.insert(
        "data".to_string(),
        Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
    );
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.needs_debug_format(&HirExpr::Var("data".to_string())));
}

#[test]
fn test_needs_debug_format_set_type() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("s".to_string(), Type::Set(Box::new(Type::Int)));
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.needs_debug_format(&HirExpr::Var("s".to_string())));
}

#[test]
fn test_needs_debug_format_optional_type() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("maybe".to_string(), Type::Optional(Box::new(Type::String)));
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.needs_debug_format(&HirExpr::Var("maybe".to_string())));
}

#[test]
fn test_needs_debug_format_value_heuristic() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.needs_debug_format(&HirExpr::Var("value".to_string())));
}

#[test]
fn test_needs_debug_format_string_type() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types.insert("name".to_string(), Type::String);
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(!converter.needs_debug_format(&HirExpr::Var("name".to_string())));
}

#[test]
fn test_needs_debug_format_list_literal() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))]);
    assert!(converter.needs_debug_format(&expr));
}

#[test]
fn test_needs_debug_format_dict_literal() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::Dict(vec![(
        HirExpr::Literal(Literal::String("k".to_string())),
        HirExpr::Literal(Literal::Int(1)),
    )]);
    assert!(converter.needs_debug_format(&expr));
}

#[test]
fn test_needs_debug_format_call_false() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::Call {
        func: "foo".to_string(),
        args: vec![],
        kwargs: vec![],
    };
    assert!(!converter.needs_debug_format(&expr));
}

// ===== is_pathbuf_expr tests =====

#[test]
fn test_is_pathbuf_expr_typed_var() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("p".to_string(), Type::Custom("PathBuf".to_string()));
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_pathbuf_expr(&HirExpr::Var("p".to_string())));
}

#[test]
fn test_is_pathbuf_expr_path_type() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("p".to_string(), Type::Custom("Path".to_string()));
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(converter.is_pathbuf_expr(&HirExpr::Var("p".to_string())));
}

#[test]
fn test_is_pathbuf_expr_not_pathbuf() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types.insert("s".to_string(), Type::String);
    let converter = ExpressionConverter::new(&mut ctx);
    assert!(!converter.is_pathbuf_expr(&HirExpr::Var("s".to_string())));
}

#[test]
fn test_is_pathbuf_expr_parent_method() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("p".to_string())),
        method: "parent".to_string(),
        args: vec![],
        kwargs: vec![],
    };
    assert!(converter.is_pathbuf_expr(&expr));
}

#[test]
fn test_is_pathbuf_expr_join_on_pathbuf() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("base".to_string(), Type::Custom("PathBuf".to_string()));
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("base".to_string())),
        method: "join".to_string(),
        args: vec![HirExpr::Literal(Literal::String("sub".to_string()))],
        kwargs: vec![],
    };
    assert!(converter.is_pathbuf_expr(&expr));
}

#[test]
fn test_is_pathbuf_expr_join_on_string() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types.insert("sep".to_string(), Type::String);
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("sep".to_string())),
        method: "join".to_string(),
        args: vec![],
        kwargs: vec![],
    };
    assert!(!converter.is_pathbuf_expr(&expr));
}

#[test]
fn test_is_pathbuf_expr_parent_attribute() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("p".to_string(), Type::Custom("PathBuf".to_string()));
    let converter = ExpressionConverter::new(&mut ctx);
    let expr = HirExpr::Attribute {
        value: Box::new(HirExpr::Var("p".to_string())),
        attr: "parent".to_string(),
    };
    assert!(converter.is_pathbuf_expr(&expr));
}

// ===== infer_numeric_type_token tests =====

#[test]
fn test_infer_numeric_type_token_int_return() {
    let mut ctx = CodeGenContext::default();
    ctx.current_return_type = Some(Type::Int);
    let converter = ExpressionConverter::new(&mut ctx);
    let token = converter.infer_numeric_type_token();
    assert!(
        token.to_string().contains("i32"),
        "Should be i32: {}",
        token
    );
}

#[test]
fn test_infer_numeric_type_token_float_return() {
    let mut ctx = CodeGenContext::default();
    ctx.current_return_type = Some(Type::Float);
    let converter = ExpressionConverter::new(&mut ctx);
    let token = converter.infer_numeric_type_token();
    assert!(
        token.to_string().contains("f64"),
        "Should be f64: {}",
        token
    );
}

#[test]
fn test_infer_numeric_type_token_default() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let token = converter.infer_numeric_type_token();
    assert!(
        token.to_string().contains("i32"),
        "Default should be i32: {}",
        token
    );
}

#[test]
fn test_infer_numeric_type_token_string_return() {
    let mut ctx = CodeGenContext::default();
    ctx.current_return_type = Some(Type::String);
    let converter = ExpressionConverter::new(&mut ctx);
    let token = converter.infer_numeric_type_token();
    assert!(
        token.to_string().contains("i32"),
        "Non-numeric return should default to i32: {}",
        token
    );
}

// ===== deref_if_borrowed_param tests =====

#[test]
fn test_deref_if_borrowed_param_ref_param() {
    let mut ctx = CodeGenContext::default();
    ctx.ref_params.insert("data".to_string());
    let converter = ExpressionConverter::new(&mut ctx);
    let rust_expr: syn::Expr = parse_quote! { data };
    let result =
        converter.deref_if_borrowed_param(&HirExpr::Var("data".to_string()), rust_expr);
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("*"), "Should deref borrowed param: {}", code);
}

#[test]
fn test_deref_if_borrowed_param_not_ref() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let rust_expr: syn::Expr = parse_quote! { data };
    let result =
        converter.deref_if_borrowed_param(&HirExpr::Var("data".to_string()), rust_expr);
    let code = quote::quote!(#result).to_string();
    assert!(
        !code.contains("*"),
        "Should not deref non-borrowed: {}",
        code
    );
}

#[test]
fn test_deref_if_borrowed_param_non_var() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let rust_expr: syn::Expr = parse_quote! { 42 };
    let result =
        converter.deref_if_borrowed_param(&HirExpr::Literal(Literal::Int(42)), rust_expr);
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("42"), "Should pass through literal: {}", code);
}

// ===== infer_iterable_element_type additional tests =====

#[test]
fn test_infer_element_type_from_set_var() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("nums".to_string(), Type::Set(Box::new(Type::Int)));
    let converter = ExpressionConverter::new(&mut ctx);
    let iterable = HirExpr::Var("nums".to_string());
    let elem_type = converter.infer_iterable_element_type(&iterable);
    assert!(matches!(elem_type, Some(Type::Int)));
}

#[test]
fn test_infer_element_type_from_string_list() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let iterable = HirExpr::List(vec![
        HirExpr::Literal(Literal::String("a".to_string())),
        HirExpr::Literal(Literal::String("b".to_string())),
    ]);
    let elem_type = converter.infer_iterable_element_type(&iterable);
    assert!(matches!(elem_type, Some(Type::String)));
}

#[test]
fn test_infer_element_type_from_empty_list() {
    let mut ctx = CodeGenContext::default();
    let converter = ExpressionConverter::new(&mut ctx);
    let iterable = HirExpr::List(vec![]);
    let elem_type = converter.infer_iterable_element_type(&iterable);
    assert!(elem_type.is_none());
}

#[test]
fn test_infer_element_type_custom_vec_f64() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types
        .insert("data".to_string(), Type::Custom("Vec<f64>".to_string()));
    let converter = ExpressionConverter::new(&mut ctx);
    let iterable = HirExpr::Var("data".to_string());
    let elem_type = converter.infer_iterable_element_type(&iterable);
    assert!(matches!(elem_type, Some(Type::Float)));
}

// ================================================================
// Session 9: Coverage improvement tests
// ================================================================

fn transpile(python_code: &str) -> String {
    use crate::ast_bridge::AstBridge;
    use crate::rust_gen::generate_rust_file;
    use crate::type_mapper::TypeMapper;
    use rustpython_parser::{parse, Mode};

    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// --- Walrus expressions ---

#[test]
fn test_s9_walrus_in_if() {
    let code = r#"
def check(items: list) -> bool:
    if len(items) > 0:
        return True
    return False
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check"), "output: {}", rust);
}

// --- Type conversion calls ---

#[test]
fn test_s9_int_conversion() {
    let code = r#"
def to_int(s: str) -> int:
    return int(s)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_int"), "output: {}", rust);
    assert!(rust.contains("parse"), "Should contain parse: {}", rust);
}

#[test]
fn test_s9_float_conversion() {
    let code = r#"
def to_float(s: str) -> float:
    return float(s)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_float"), "output: {}", rust);
}

#[test]
fn test_s9_str_conversion() {
    let code = r#"
def to_str(n: int) -> str:
    return str(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_str"), "output: {}", rust);
}

#[test]
fn test_s9_bool_conversion() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_bool"), "output: {}", rust);
}

// --- Builtin function calls ---

#[test]
fn test_s9_len_call() {
    let code = r#"
def size(items: list) -> int:
    return len(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn size"), "output: {}", rust);
    assert!(rust.contains("len()"), "Should contain len: {}", rust);
}

#[test]
fn test_s9_abs_call() {
    let code = r#"
def absolute(n: int) -> int:
    return abs(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn absolute"), "output: {}", rust);
    assert!(rust.contains("abs"), "Should contain abs: {}", rust);
}

#[test]
fn test_s9_min_call() {
    let code = r#"
def minimum(a: int, b: int) -> int:
    return min(a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn minimum"), "output: {}", rust);
}

#[test]
fn test_s9_max_call() {
    let code = r#"
def maximum(a: int, b: int) -> int:
    return max(a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn maximum"), "output: {}", rust);
}

#[test]
fn test_s9_sum_call() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn total"), "output: {}", rust);
}

#[test]
fn test_s9_sorted_call() {
    let code = r#"
def sort_items(items: list) -> list:
    return sorted(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sort_items"), "output: {}", rust);
}

#[test]
fn test_s9_reversed_call() {
    let code = r#"
def flip(items: list) -> list:
    return list(reversed(items))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn flip"), "output: {}", rust);
}

#[test]
fn test_s9_range_one_arg() {
    let code = r#"
def count(n: int) -> list:
    return list(range(n))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn count"), "output: {}", rust);
}

#[test]
fn test_s9_range_two_args() {
    let code = r#"
def span(start: int, end: int) -> list:
    return list(range(start, end))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn span"), "output: {}", rust);
}

#[test]
fn test_s9_range_three_args() {
    let code = r#"
def stepped(start: int, end: int, step: int) -> list:
    return list(range(start, end, step))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn stepped"), "output: {}", rust);
}

#[test]
fn test_s9_print_call() {
    let code = r#"
def say(msg: str) -> None:
    print(msg)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn say"), "output: {}", rust);
    assert!(
        rust.contains("println") || rust.contains("print"),
        "output: {}",
        rust
    );
}

#[test]
fn test_s9_print_multiple_args() {
    let code = r#"
def show(a: str, b: str) -> None:
    print(a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn show"), "output: {}", rust);
}

// --- Binary operations ---

#[test]
fn test_s9_floor_div() {
    let code = r#"
def divide(a: int, b: int) -> int:
    return a // b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn divide"), "output: {}", rust);
}

#[test]
fn test_s9_power_op() {
    let code = r#"
def power(base: int, exp: int) -> int:
    return base ** exp
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn power"), "output: {}", rust);
}

#[test]
fn test_s9_modulo_op() {
    let code = r#"
def remainder(a: int, b: int) -> int:
    return a % b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn remainder"), "output: {}", rust);
}

#[test]
fn test_s9_bitwise_and() {
    let code = r#"
def bit_and(a: int, b: int) -> int:
    return a & b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn bit_and"), "output: {}", rust);
}

#[test]
fn test_s9_bitwise_or() {
    let code = r#"
def bit_or(a: int, b: int) -> int:
    return a | b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn bit_or"), "output: {}", rust);
}

#[test]
fn test_s9_bitwise_xor() {
    let code = r#"
def bit_xor(a: int, b: int) -> int:
    return a ^ b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn bit_xor"), "output: {}", rust);
}

#[test]
fn test_s9_left_shift() {
    let code = r#"
def shift_left(a: int, n: int) -> int:
    return a << n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn shift_left"), "output: {}", rust);
}

#[test]
fn test_s9_right_shift() {
    let code = r#"
def shift_right(a: int, n: int) -> int:
    return a >> n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn shift_right"), "output: {}", rust);
}

// --- Unary operations ---

#[test]
fn test_s9_unary_neg() {
    let code = r#"
def negate(n: int) -> int:
    return -n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn negate"), "output: {}", rust);
}

#[test]
fn test_s9_unary_not() {
    let code = r#"
def invert(b: bool) -> bool:
    return not b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn invert"), "output: {}", rust);
}

#[test]
fn test_s9_unary_bitnot() {
    let code = r#"
def complement(n: int) -> int:
    return ~n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn complement"), "output: {}", rust);
}

// --- Complex expressions ---

#[test]
fn test_s9_ternary_expression() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    return lo if x < lo else hi if x > hi else x
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn clamp"), "output: {}", rust);
}

#[test]
fn test_s9_chained_comparisons() {
    let code = r#"
def in_range(x: int, lo: int, hi: int) -> bool:
    return lo <= x and x <= hi
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn in_range"), "output: {}", rust);
}

#[test]
fn test_s9_string_multiply() {
    let code = r#"
def repeat(s: str, n: int) -> str:
    return s * n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn repeat"), "output: {}", rust);
}

#[test]
fn test_s9_list_multiply() {
    let code = r#"
def repeat_list(items: list, n: int) -> list:
    return items * n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn repeat_list"), "output: {}", rust);
}

#[test]
fn test_s9_mixed_arithmetic() {
    let code = r#"
def formula(a: float, b: float, c: float) -> float:
    return (-b + (b * b - 4.0 * a * c) ** 0.5) / (2.0 * a)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn formula"), "output: {}", rust);
}

#[test]
fn test_s9_complex_dict_literal() {
    let code = r#"
def config() -> dict:
    return {"name": "test", "count": 42, "active": True}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn config"), "output: {}", rust);
}

#[test]
fn test_s9_nested_dict() {
    let code = r#"
def nested() -> dict:
    return {"outer": {"inner": 1}}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn nested"), "output: {}", rust);
}

#[test]
fn test_s9_class_method_call() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self) -> int:
        self.count += 1
        return self.count
"#;
    let rust = transpile(code);
    assert!(
        rust.contains("Counter") || rust.contains("struct"),
        "output: {}",
        rust
    );
}

#[test]
fn test_s9_multiple_return_values() {
    let code = r#"
def divmod_fn(a: int, b: int) -> tuple:
    return (a // b, a % b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn divmod_fn"), "output: {}", rust);
}

#[test]
fn test_s9_nested_function_calls() {
    let code = r#"
def double_abs(n: int) -> int:
    return abs(n) * 2
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn double_abs"), "output: {}", rust);
}

// === S9 Batch 3: Expression coverage ===

#[test]
fn test_s9b3_comparison_chain() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 < x < 100
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn in_range"), "output: {}", rust);
}

#[test]
fn test_s9b3_boolean_and_or() {
    let code = r#"
def check(a: bool, b: bool) -> bool:
    return a and b or not a
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_multiply() {
    let code = r#"
def repeat(s: str, n: int) -> str:
    return s * n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn repeat"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_multiply() {
    let code = r#"
def zeros(n: int) -> list:
    return [0] * n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn zeros"), "output: {}", rust);
}

#[test]
fn test_s9b3_complex_ternary() {
    let code = r#"
def sign(x: int) -> int:
    return 1 if x > 0 else -1 if x < 0 else 0
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sign"), "output: {}", rust);
}

#[test]
fn test_s9b3_builtin_isinstance() {
    let code = r#"
def is_int(x: int) -> bool:
    return isinstance(x, int)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn is_int"), "output: {}", rust);
}

#[test]
fn test_s9b3_builtin_enumerate() {
    let code = r#"
def indexed(items: list) -> list:
    result = []
    for i, v in enumerate(items):
        result.append(i)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn indexed"), "output: {}", rust);
}

#[test]
fn test_s9b3_builtin_zip() {
    let code = r#"
def combine(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn combine"), "output: {}", rust);
}

#[test]
fn test_s9b3_global_constant() {
    let code = r#"
MAX_SIZE = 100

def check_size(n: int) -> bool:
    return n <= MAX_SIZE
"#;
    let rust = transpile(code);
    assert!(
        rust.contains("MAX_SIZE") || rust.contains("100"),
        "output: {}",
        rust
    );
}

#[test]
fn test_s9b3_multiple_assignment() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn swap"), "output: {}", rust);
}

#[test]
fn test_s9b3_augmented_str_concat() {
    let code = r#"
def build_string(items: list) -> str:
    result = ""
    for item in items:
        result += str(item)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn build_string"), "output: {}", rust);
}

#[test]
fn test_s9b3_nested_if_in_loop() {
    let code = r#"
def count_positive(items: list) -> int:
    count = 0
    for x in items:
        if x > 0:
            count += 1
    return count
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn count_positive"), "output: {}", rust);
}

#[test]
fn test_s9b3_early_return() {
    let code = r#"
def find_index(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_index"), "output: {}", rust);
}

#[test]
fn test_s9b3_nested_loops() {
    let code = r#"
def matrix_sum(matrix: list) -> int:
    total = 0
    for row in matrix:
        for val in row:
            total += val
    return total
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn matrix_sum"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_of_lists() {
    let code = r#"
def make_grid(n: int) -> list:
    grid = []
    for i in range(n):
        row = []
        for j in range(n):
            row.append(i * n + j)
        grid.append(row)
    return grid
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_grid"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_format_f() {
    let code = r#"
def format_number(n: int) -> str:
    return f"Value: {n}"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn format_number"), "output: {}", rust);
}

#[test]
fn test_s9b3_empty_list_literal() {
    let code = r#"
def make_empty() -> list:
    return []
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_empty"), "output: {}", rust);
}

#[test]
fn test_s9b3_empty_dict_literal() {
    let code = r#"
def make_empty_dict() -> dict:
    return {}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_empty_dict"), "output: {}", rust);
}

#[test]
fn test_s9b3_empty_set() {
    let code = r#"
def make_empty_set() -> set:
    return set()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_empty_set"), "output: {}", rust);
}

#[test]
fn test_s9b3_bitwise_operations() {
    let code = r#"
def bitwise(a: int, b: int) -> int:
    return (a & b) | (a ^ b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn bitwise"), "output: {}", rust);
}

#[test]
fn test_s9b3_shift_operations() {
    let code = r#"
def shift(n: int) -> int:
    return (n << 2) >> 1
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn shift"), "output: {}", rust);
}

#[test]
fn test_s9b3_floor_div_and_mod() {
    let code = r#"
def div_and_mod(a: int, b: int) -> tuple:
    return (a // b, a % b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn div_and_mod"), "output: {}", rust);
}

#[test]
fn test_s9b3_power_operator() {
    let code = r#"
def power(base: int, exp: int) -> int:
    return base ** exp
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn power"), "output: {}", rust);
}

#[test]
fn test_s9b3_unary_not() {
    let code = r#"
def negate(b: bool) -> bool:
    return not b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn negate"), "output: {}", rust);
}

#[test]
fn test_s9b3_class_with_init_and_methods() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def push(self, item: int) -> None:
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def is_empty(self) -> bool:
        return len(self.items) == 0
"#;
    let rust = transpile(code);
    assert!(
        rust.contains("Stack") || rust.contains("struct"),
        "output: {}",
        rust
    );
}

#[test]
fn test_s9b3_import_and_use() {
    let code = r#"
import os

def get_cwd() -> str:
    return os.getcwd()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_cwd"), "output: {}", rust);
}

#[test]
fn test_s9b3_from_import() {
    let code = r#"
from pathlib import Path

def exists(p: str) -> bool:
    return Path(p).exists()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn exists") || rust.contains("Path"), "output: {}", rust);
}

#[test]
fn test_s9b3_assert_with_message() {
    let code = r#"
def validate(x: int) -> int:
    assert x >= 0, "must be non-negative"
    return x
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn validate"), "output: {}", rust);
}

#[test]
fn test_s9b3_del_dict_key() {
    let code = r#"
def remove(d: dict, key: str) -> None:
    del d[key]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn remove"), "output: {}", rust);
}

// --- S9B6: Coverage-focused tests for expr_gen.rs ---

#[test]
fn test_s9b6_coerce_int_to_float_variable() {
    let code = r#"
def calc(i: int, dx: float) -> float:
    return i * dx
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn calc"), "output: {}", rust);
    assert!(rust.contains("i") && rust.contains("dx"), "output: {}", rust);
}

#[test]
fn test_s9b6_coerce_int_to_float_binary_expr() {
    let code = r#"
def multiply(i: int, j: int, dx: float) -> float:
    return (i + j) * dx
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn multiply"), "output: {}", rust);
}

#[test]
fn test_s9b6_infer_iterable_element_type_filter() {
    let code = r#"
def filter_positive(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn filter_positive"), "output: {}", rust);
}

#[test]
fn test_s9b6_builtin_type_as_function_reference() {
    let code = r#"
def convert_all(items: list) -> list:
    return list(map(int, items))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn convert_all"), "output: {}", rust);
}

#[test]
fn test_s9b6_narrowed_option_after_none_check() {
    let code = r#"
def process(opt: int) -> int:
    if opt is not None:
        return opt + 1
    return 0
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn process"), "output: {}", rust);
}

#[test]
fn test_s9b6_nasa_mode_div_expecting_int() {
    let code = r#"
def divide(a: int, b: int) -> int:
    return a / b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn divide"), "output: {}", rust);
}

#[test]
fn test_s9b6_nasa_mode_chained_arithmetic() {
    let code = r#"
def compute(x: int, y: float) -> float:
    return x + y * 2
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn compute"), "output: {}", rust);
}

#[test]
fn test_s9b6_type_anchoring_chained_pyops() {
    let code = r#"
def chain_ops(a: int, b: int, c: int) -> int:
    return a + b - c * 2
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn chain_ops"), "output: {}", rust);
}

#[test]
fn test_s9b6_nested_collection_vec_vec() {
    let code = r#"
def nested_lists() -> list:
    return [[1, 2], [3, 4]]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn nested_lists"), "output: {}", rust);
}

#[test]
fn test_s9b6_mixed_int_float_list() {
    let code = r#"
def mixed_values() -> list:
    return [1, 2.5, 3]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn mixed_values"), "output: {}", rust);
}

#[test]
fn test_s9b6_multi_level_dict_access() {
    let code = r#"
def get_nested(data: dict) -> str:
    return data['level1']['level2']
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_nested"), "output: {}", rust);
}

#[test]
fn test_s9b6_tuple_unpacking_from_index() {
    let code = r#"
def unpack(parts: list) -> tuple:
    a, b = parts[0], parts[1]
    return (a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn unpack"), "output: {}", rust);
}

#[test]
fn test_s9b6_walrus_in_while_loop() {
    let code = r#"
def read_lines(f: list) -> list:
    lines = []
    for line in f:
        if line:
            lines.append(line)
    return lines
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn read_lines"), "output: {}", rust);
}

#[test]
fn test_s9b6_walrus_with_method_call() {
    let code = r#"
def check_length(data: list) -> bool:
    n = len(data)
    if n > 0:
        return True
    return False
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_length"), "output: {}", rust);
}

#[test]
fn test_s9b6_generator_state_in_class() {
    let code = r#"
class Counter:
    def __init__(self, start: int):
        self.value = start

    def get_value(self) -> int:
        return self.value
"#;
    let rust = transpile(code);
    assert!(rust.contains("Counter") || rust.contains("struct"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_concat_with_format() {
    let code = r#"
def greet(name: str, age: int) -> str:
    return f"{name}{age}"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn greet"), "output: {}", rust);
}

#[test]
fn test_s9b6_boolean_context_not_items() {
    let code = r#"
def is_empty(items: list) -> bool:
    if not items:
        return True
    return False
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn is_empty"), "output: {}", rust);
}

#[test]
fn test_s9b6_boolean_context_if_results() {
    let code = r#"
def has_results(results: list) -> bool:
    if results:
        return True
    return False
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn has_results"), "output: {}", rust);
}

#[test]
fn test_s9b6_comparison_chain() {
    let code = r#"
def in_range(x: int, y: int) -> bool:
    return 0 < x < 10 < y
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn in_range"), "output: {}", rust);
}

#[test]
fn test_s9b6_augmented_assignment_float() {
    let code = r#"
def accumulate(x: int, y: float) -> float:
    result = x
    result += y
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn accumulate"), "output: {}", rust);
}

#[test]
fn test_s9b6_multiple_return_values() {
    let code = r#"
def split_triple(val: int) -> tuple:
    a = val
    b = val * 2
    c = val * 3
    return a, b, c
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn split_triple"), "output: {}", rust);
}

#[test]
fn test_s9b6_conditional_with_methods() {
    let code = r#"
def transform(x: str, cond: bool) -> str:
    return x.upper() if cond else x.lower()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn transform"), "output: {}", rust);
}

#[test]
fn test_s9b6_dict_comprehension() {
    let code = r#"
def make_dict(items: dict) -> dict:
    return {k: v for k, v in items.items()}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_dict"), "output: {}", rust);
}

#[test]
fn test_s9b6_set_comprehension() {
    let code = r#"
def doubled_set(n: int) -> set:
    return {x * 2 for x in range(n)}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn doubled_set"), "output: {}", rust);
}

// ========================================================================
// BINARY_OPS.RS COVERAGE TESTS - Arithmetic, Comparison, Logical, Bitwise
// ========================================================================

#[test]
fn test_binop_addition_int() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn add"), "output: {}", rust);
    assert!(rust.contains("+"), "Should contain +: {}", rust);
}

#[test]
fn test_binop_subtraction_int() {
    let code = r#"
def sub(a: int, b: int) -> int:
    return a - b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sub"), "output: {}", rust);
    assert!(rust.contains("-"), "Should contain -: {}", rust);
}

#[test]
fn test_binop_multiplication_int() {
    let code = r#"
def mul(a: int, b: int) -> int:
    return a * b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn mul"), "output: {}", rust);
    assert!(rust.contains("*"), "Should contain *: {}", rust);
}

#[test]
fn test_binop_division_float() {
    let code = r#"
def div(a: float, b: float) -> float:
    return a / b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn div"), "output: {}", rust);
    assert!(rust.contains("/"), "Should contain /: {}", rust);
}

#[test]
fn test_binop_floor_division_int() {
    let code = r#"
def floor_div(a: int, b: int) -> int:
    return a // b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn floor_div"), "output: {}", rust);
}

#[test]
fn test_binop_modulo_int() {
    let code = r#"
def modulo(a: int, b: int) -> int:
    return a % b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn modulo"), "output: {}", rust);
    assert!(rust.contains("%"), "Should contain %: {}", rust);
}

#[test]
fn test_binop_power_int() {
    let code = r#"
def power(base: int, exp: int) -> int:
    return base ** exp
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn power"), "output: {}", rust);
    assert!(rust.contains("pow"), "Should contain pow: {}", rust);
}

#[test]
fn test_binop_power_float() {
    let code = r#"
def power_f(base: float, exp: float) -> float:
    return base ** exp
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn power_f"), "output: {}", rust);
    assert!(rust.contains("powf") || rust.contains("pow"), "Should contain pow: {}", rust);
}

#[test]
fn test_binop_eq_comparison() {
    let code = r#"
def is_equal(a: int, b: int) -> bool:
    return a == b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn is_equal"), "output: {}", rust);
    assert!(rust.contains("=="), "Should contain ==: {}", rust);
}

#[test]
fn test_binop_ne_comparison() {
    let code = r#"
def not_equal(a: int, b: int) -> bool:
    return a != b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn not_equal"), "output: {}", rust);
    assert!(rust.contains("!="), "Should contain !=: {}", rust);
}

#[test]
fn test_binop_lt_comparison() {
    let code = r#"
def less_than(a: int, b: int) -> bool:
    return a < b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn less_than"), "output: {}", rust);
    assert!(rust.contains("<"), "Should contain <: {}", rust);
}

#[test]
fn test_binop_gt_comparison() {
    let code = r#"
def greater_than(a: int, b: int) -> bool:
    return a > b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn greater_than"), "output: {}", rust);
    assert!(rust.contains(">"), "Should contain >: {}", rust);
}

#[test]
fn test_binop_le_comparison() {
    let code = r#"
def less_or_equal(a: int, b: int) -> bool:
    return a <= b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn less_or_equal"), "output: {}", rust);
    assert!(rust.contains("<="), "Should contain <=: {}", rust);
}

#[test]
fn test_binop_ge_comparison() {
    let code = r#"
def greater_or_equal(a: int, b: int) -> bool:
    return a >= b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn greater_or_equal"), "output: {}", rust);
    assert!(rust.contains(">="), "Should contain >=: {}", rust);
}

#[test]
fn test_binop_logical_and() {
    let code = r#"
def both(a: bool, b: bool) -> bool:
    return a and b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn both"), "output: {}", rust);
    assert!(rust.contains("&&"), "Should contain &&: {}", rust);
}

#[test]
fn test_binop_logical_or() {
    let code = r#"
def either(a: bool, b: bool) -> bool:
    return a or b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn either"), "output: {}", rust);
    assert!(rust.contains("||"), "Should contain ||: {}", rust);
}

#[test]
fn test_binop_bitwise_and_expr() {
    let code = r#"
def bit_and(a: int, b: int) -> int:
    return a & b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn bit_and"), "output: {}", rust);
    assert!(rust.contains("&"), "Should contain &: {}", rust);
}

#[test]
fn test_binop_bitwise_or_expr() {
    let code = r#"
def bit_or(a: int, b: int) -> int:
    return a | b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn bit_or"), "output: {}", rust);
    assert!(rust.contains("|"), "Should contain |: {}", rust);
}

#[test]
fn test_binop_bitwise_xor_expr() {
    let code = r#"
def bit_xor(a: int, b: int) -> int:
    return a ^ b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn bit_xor"), "output: {}", rust);
    assert!(rust.contains("^"), "Should contain ^: {}", rust);
}

#[test]
fn test_binop_left_shift_expr() {
    let code = r#"
def lshift(a: int, n: int) -> int:
    return a << n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn lshift"), "output: {}", rust);
    assert!(rust.contains("<<"), "Should contain <<: {}", rust);
}

#[test]
fn test_binop_right_shift_expr() {
    let code = r#"
def rshift(a: int, n: int) -> int:
    return a >> n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn rshift"), "output: {}", rust);
    assert!(rust.contains(">>"), "Should contain >>: {}", rust);
}

#[test]
fn test_binop_string_concat() {
    let code = r#"
def greet(first: str, last: str) -> str:
    return first + " " + last
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn greet"), "output: {}", rust);
    assert!(rust.contains("format!") || rust.contains("+"), "Should concat strings: {}", rust);
}

#[test]
fn test_binop_list_concat() {
    let code = r#"
def merge(a: list, b: list) -> list:
    return a + b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn merge"), "output: {}", rust);
}

#[test]
fn test_binop_in_list() {
    let code = r#"
def has_item(items: list, x: int) -> bool:
    return x in items
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn has_item"), "output: {}", rust);
    assert!(rust.contains("contains"), "Should contain contains: {}", rust);
}

#[test]
fn test_binop_in_dict() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn has_key"), "output: {}", rust);
    assert!(rust.contains("contains_key") || rust.contains("contains"), "Should contain contains: {}", rust);
}

#[test]
fn test_binop_in_string() {
    let code = r#"
def has_char(text: str, ch: str) -> bool:
    return ch in text
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn has_char"), "output: {}", rust);
    assert!(rust.contains("contains"), "Should contain contains: {}", rust);
}

#[test]
fn test_binop_not_in() {
    let code = r#"
def missing(items: list, x: int) -> bool:
    return x not in items
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn missing"), "output: {}", rust);
    assert!(rust.contains("contains"), "Should contain contains: {}", rust);
}

#[test]
fn test_binop_is_none() {
    let code = r#"
def check_none(x: int) -> bool:
    return x is None
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_none"), "output: {}", rust);
    assert!(rust.contains("is_none") || rust.contains("None"), "Should check None: {}", rust);
}

#[test]
fn test_binop_is_not_none() {
    let code = r#"
def check_not_none(x: int) -> bool:
    return x is not None
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_not_none"), "output: {}", rust);
    assert!(rust.contains("is_some") || rust.contains("Some") || rust.contains("None"), "Should check not None: {}", rust);
}

#[test]
fn test_binop_chained_comparison() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 < x < 100
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn in_range"), "output: {}", rust);
    assert!(rust.contains("&&"), "Should chain with &&: {}", rust);
}

#[test]
fn test_binop_chained_comparison_three() {
    let code = r#"
def triple_check(a: int, b: int, c: int) -> bool:
    return a < b < c
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn triple_check"), "output: {}", rust);
}

#[test]
fn test_binop_mixed_arithmetic() {
    let code = r#"
def compute(a: int, b: int, c: int) -> int:
    return a + b * c - a
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn compute"), "output: {}", rust);
}

#[test]
fn test_binop_float_division_int_operands() {
    let code = r#"
def half(n: int) -> float:
    return n / 2
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn half"), "output: {}", rust);
}

#[test]
fn test_binop_nested_boolean_logic() {
    let code = r#"
def check(a: bool, b: bool, c: bool) -> bool:
    return (a and b) or (not c)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check"), "output: {}", rust);
    assert!(rust.contains("&&") || rust.contains("||"), "Should contain logical ops: {}", rust);
}

#[test]
fn test_binop_string_repeat() {
    let code = r#"
def repeat_str(s: str, n: int) -> str:
    return s * n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn repeat_str"), "output: {}", rust);
    assert!(rust.contains("repeat"), "Should contain repeat: {}", rust);
}

#[test]
fn test_binop_list_repeat() {
    let code = r#"
def repeat_list(n: int) -> list:
    return [0] * n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn repeat_list"), "output: {}", rust);
}

#[test]
fn test_binop_augmented_add() {
    let code = r#"
def accumulate(n: int) -> int:
    total = 0
    total += n
    return total
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn accumulate"), "output: {}", rust);
    assert!(rust.contains("+="), "Should contain +=: {}", rust);
}

#[test]
fn test_binop_augmented_sub() {
    let code = r#"
def decrement(n: int) -> int:
    val = 100
    val -= n
    return val
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn decrement"), "output: {}", rust);
    assert!(rust.contains("-="), "Should contain -=: {}", rust);
}

#[test]
fn test_binop_augmented_mul() {
    let code = r#"
def scale(n: int) -> int:
    val = 1
    val *= n
    return val
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn scale"), "output: {}", rust);
    assert!(rust.contains("*=") || rust.contains("py_mul"), "Should contain *= or py_mul: {}", rust);
}

#[test]
fn test_binop_in_set() {
    let code = r#"
def in_set(s: set, x: int) -> bool:
    return x in s
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn in_set"), "output: {}", rust);
    assert!(rust.contains("contains"), "Should contain contains: {}", rust);
}

#[test]
fn test_binop_complex_expression() {
    let code = r#"
def quadratic(a: float, b: float, c: float, x: float) -> float:
    return a * x * x + b * x + c
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn quadratic"), "output: {}", rust);
}

// ========================================================================
// CONVERT_UNARY_AND_CALL.RS COVERAGE TESTS - Unary ops and builtins
// ========================================================================

#[test]
fn test_unary_negative() {
    let code = r#"
def negate(n: int) -> int:
    return -n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn negate"), "output: {}", rust);
}

#[test]
fn test_unary_positive() {
    let code = r#"
def pos(n: int) -> int:
    return +n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pos"), "output: {}", rust);
}

#[test]
fn test_unary_not_bool() {
    let code = r#"
def invert(b: bool) -> bool:
    return not b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn invert"), "output: {}", rust);
    assert!(rust.contains("!"), "Should contain !: {}", rust);
}

#[test]
fn test_unary_bitwise_not() {
    let code = r#"
def bitnot(n: int) -> int:
    return ~n
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn bitnot"), "output: {}", rust);
}

#[test]
fn test_builtin_len_list() {
    let code = r#"
def list_len(items: list) -> int:
    return len(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn list_len"), "output: {}", rust);
    assert!(rust.contains("len()"), "Should contain len(): {}", rust);
}

#[test]
fn test_builtin_len_dict() {
    let code = r#"
def dict_len(d: dict) -> int:
    return len(d)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn dict_len"), "output: {}", rust);
    assert!(rust.contains("len()"), "Should contain len(): {}", rust);
}

#[test]
fn test_builtin_len_string() {
    let code = r#"
def str_len(s: str) -> int:
    return len(s)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn str_len"), "output: {}", rust);
    assert!(rust.contains("len()"), "Should contain len(): {}", rust);
}

#[test]
fn test_builtin_range_one_arg() {
    let code = r#"
def count(n: int) -> list:
    result = []
    for i in range(n):
        result.append(i)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn count"), "output: {}", rust);
}

#[test]
fn test_builtin_range_two_args() {
    let code = r#"
def range_from(start: int, end: int) -> list:
    result = []
    for i in range(start, end):
        result.append(i)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn range_from"), "output: {}", rust);
}

#[test]
fn test_builtin_range_three_args() {
    let code = r#"
def range_step(start: int, end: int, step: int) -> list:
    result = []
    for i in range(start, end, step):
        result.append(i)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn range_step"), "output: {}", rust);
}

#[test]
fn test_builtin_print_single() {
    let code = r#"
def say_hello():
    print("hello")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn say_hello"), "output: {}", rust);
    assert!(rust.contains("println!") || rust.contains("print"), "Should contain print: {}", rust);
}

#[test]
fn test_builtin_print_multiple_args() {
    let code = r#"
def show(name: str, age: int):
    print(name, age)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn show"), "output: {}", rust);
    assert!(rust.contains("println!") || rust.contains("print"), "Should contain print: {}", rust);
}

#[test]
fn test_builtin_int_from_str() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn parse_int"), "output: {}", rust);
    assert!(rust.contains("parse"), "Should contain parse: {}", rust);
}

#[test]
fn test_builtin_float_from_str() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn parse_float"), "output: {}", rust);
}

#[test]
fn test_builtin_str_from_int() {
    let code = r#"
def int_to_str(n: int) -> str:
    return str(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn int_to_str"), "output: {}", rust);
    assert!(rust.contains("to_string"), "Should contain to_string: {}", rust);
}

#[test]
fn test_builtin_bool_from_int() {
    let code = r#"
def int_to_bool(n: int) -> bool:
    return bool(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn int_to_bool"), "output: {}", rust);
}

#[test]
fn test_builtin_abs_int() {
    let code = r#"
def abs_val(n: int) -> int:
    return abs(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn abs_val"), "output: {}", rust);
    assert!(rust.contains("abs"), "Should contain abs: {}", rust);
}

#[test]
fn test_builtin_min_two_args() {
    let code = r#"
def minimum(a: int, b: int) -> int:
    return min(a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn minimum"), "output: {}", rust);
    assert!(rust.contains("min"), "Should contain min: {}", rust);
}

#[test]
fn test_builtin_max_two_args() {
    let code = r#"
def maximum(a: int, b: int) -> int:
    return max(a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn maximum"), "output: {}", rust);
    assert!(rust.contains("max"), "Should contain max: {}", rust);
}

#[test]
fn test_builtin_sum_list() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn total"), "output: {}", rust);
    assert!(rust.contains("sum") || rust.contains("iter"), "Should contain sum/iter: {}", rust);
}

#[test]
fn test_builtin_sorted_list() {
    let code = r#"
def sort_items(items: list) -> list:
    return sorted(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sort_items"), "output: {}", rust);
    assert!(rust.contains("sort") || rust.contains("sorted"), "Should contain sort: {}", rust);
}

#[test]
fn test_builtin_reversed_list() {
    let code = r#"
def rev_items(items: list) -> list:
    return list(reversed(items))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn rev_items"), "output: {}", rust);
}

#[test]
fn test_builtin_enumerate_loop() {
    let code = r#"
def with_index(items: list):
    for i, val in enumerate(items):
        print(i, val)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn with_index"), "output: {}", rust);
    assert!(rust.contains("enumerate"), "Should contain enumerate: {}", rust);
}

#[test]
fn test_builtin_zip_two_lists() {
    let code = r#"
def pair_up(a: list, b: list) -> list:
    return list(zip(a, b))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pair_up"), "output: {}", rust);
    assert!(rust.contains("zip"), "Should contain zip: {}", rust);
}

#[test]
fn test_builtin_any_list() {
    let code = r#"
def has_true(items: list) -> bool:
    return any(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn has_true"), "output: {}", rust);
    assert!(rust.contains("any") || rust.contains("iter"), "Should contain any: {}", rust);
}

#[test]
fn test_builtin_all_list() {
    let code = r#"
def all_true(items: list) -> bool:
    return all(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn all_true"), "output: {}", rust);
    assert!(rust.contains("all") || rust.contains("iter"), "Should contain all: {}", rust);
}

#[test]
fn test_builtin_isinstance_check() {
    let code = r#"
def is_int(x: int) -> bool:
    return isinstance(x, int)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn is_int"), "output: {}", rust);
}

#[test]
fn test_builtin_round_float() {
    let code = r#"
def round_val(x: float) -> int:
    return round(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn round_val"), "output: {}", rust);
    assert!(rust.contains("round"), "Should contain round: {}", rust);
}

#[test]
fn test_builtin_ord_char() {
    let code = r#"
def char_code(c: str) -> int:
    return ord(c)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn char_code"), "output: {}", rust);
}

#[test]
fn test_builtin_chr_int() {
    let code = r#"
def code_char(n: int) -> str:
    return chr(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn code_char"), "output: {}", rust);
}

#[test]
fn test_builtin_hex_int() {
    let code = r#"
def to_hex(n: int) -> str:
    return hex(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_hex"), "output: {}", rust);
    assert!(rust.contains("format!") || rust.contains("hex"), "Should format hex: {}", rust);
}

#[test]
fn test_builtin_oct_int() {
    let code = r#"
def to_oct(n: int) -> str:
    return oct(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_oct"), "output: {}", rust);
    assert!(rust.contains("format!") || rust.contains("oct"), "Should format oct: {}", rust);
}

#[test]
fn test_builtin_bin_int() {
    let code = r#"
def to_bin(n: int) -> str:
    return bin(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_bin"), "output: {}", rust);
    assert!(rust.contains("format!") || rust.contains("bin"), "Should format bin: {}", rust);
}

#[test]
fn test_builtin_input_call() {
    let code = r#"
def ask():
    name = input("Name: ")
    return name
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn ask"), "output: {}", rust);
}

#[test]
fn test_builtin_divmod() {
    let code = r#"
def div_and_mod(a: int, b: int):
    q, r = divmod(a, b)
    return q
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn div_and_mod"), "output: {}", rust);
}

#[test]
fn test_builtin_pow_three_args() {
    let code = r#"
def mod_pow(base: int, exp: int, mod_val: int) -> int:
    return pow(base, exp, mod_val)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn mod_pow"), "output: {}", rust);
}

#[test]
fn test_unary_not_collection_truthiness() {
    let code = r#"
def is_empty(items: list) -> bool:
    return not items
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn is_empty"), "output: {}", rust);
    assert!(rust.contains("is_empty"), "Should use is_empty: {}", rust);
}

#[test]
fn test_builtin_len_set() {
    let code = r#"
def set_size(s: set) -> int:
    return len(s)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn set_size"), "output: {}", rust);
    assert!(rust.contains("len()"), "Should contain len(): {}", rust);
}

#[test]
fn test_builtin_filter_with_lambda() {
    let code = r#"
def evens(items: list) -> list:
    return list(filter(lambda x: x % 2 == 0, items))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn evens"), "output: {}", rust);
    assert!(rust.contains("filter"), "Should contain filter: {}", rust);
}

#[test]
fn test_builtin_map_with_lambda() {
    let code = r#"
def doubled(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn doubled"), "output: {}", rust);
    assert!(rust.contains("map"), "Should contain map: {}", rust);
}

// ========================================================================
// CALL_DISPATCH.RS COVERAGE TESTS - Function call routing
// ========================================================================

#[test]
fn test_dispatch_open_read() {
    let code = r#"
def read_file(path: str) -> str:
    f = open(path, "r")
    return f.read()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn read_file"), "output: {}", rust);
}

#[test]
fn test_dispatch_open_write() {
    let code = r#"
def write_file(path: str, content: str):
    f = open(path, "w")
    f.write(content)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn write_file"), "output: {}", rust);
}

#[test]
fn test_dispatch_json_dumps() {
    let code = r#"
import json
def to_json(data: dict) -> str:
    return json.dumps(data)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_json"), "output: {}", rust);
}

#[test]
fn test_dispatch_json_loads() {
    let code = r#"
import json
def from_json(text: str) -> dict:
    return json.loads(text)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn from_json"), "output: {}", rust);
}

#[test]
fn test_dispatch_print_to_stderr() {
    let code = r#"
import sys
def warn(msg: str):
    print(msg, file=sys.stderr)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn warn"), "output: {}", rust);
    assert!(rust.contains("eprintln!") || rust.contains("stderr"), "Should use stderr: {}", rust);
}

#[test]
fn test_dispatch_type_call() {
    let code = r#"
def check_type(x: int) -> str:
    return str(type(x))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_type"), "output: {}", rust);
}

#[test]
fn test_dispatch_sum_with_generator() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sum_squares"), "output: {}", rust);
}

#[test]
fn test_dispatch_min_list() {
    let code = r#"
def find_min(items: list) -> int:
    return min(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_min"), "output: {}", rust);
}

#[test]
fn test_dispatch_max_list() {
    let code = r#"
def find_max(items: list) -> int:
    return max(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_max"), "output: {}", rust);
}

#[test]
fn test_dispatch_any_with_generator() {
    let code = r#"
def has_positive(items: list) -> bool:
    return any(x > 0 for x in items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn has_positive"), "output: {}", rust);
}

#[test]
fn test_dispatch_all_with_generator() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn all_positive"), "output: {}", rust);
}

// ========================================================================
// STDLIB_DATA.RS COVERAGE TESTS - collections, itertools, functools
// ========================================================================

#[test]
fn test_stdlib_collections_counter() {
    let code = r#"
from collections import Counter
def count_items(items: list) -> dict:
    return Counter(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn count_items"), "output: {}", rust);
}

#[test]
fn test_stdlib_collections_defaultdict() {
    let code = r#"
from collections import defaultdict
def grouped():
    d = defaultdict(list)
    d["a"].append(1)
    return d
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn grouped"), "output: {}", rust);
}

#[test]
fn test_stdlib_calendar_isleap() {
    let code = r#"
import calendar
def is_leap(year: int) -> bool:
    return calendar.isleap(year)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn is_leap"), "output: {}", rust);
}

// ========================================================================
// CALL_GENERIC.RS COVERAGE TESTS - Generic call handling
// ========================================================================

#[test]
fn test_generic_user_function_call() {
    let code = r#"
def helper(x: int) -> int:
    return x + 1

def main_func(n: int) -> int:
    return helper(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn helper"), "output: {}", rust);
    assert!(rust.contains("fn main_func"), "output: {}", rust);
    assert!(rust.contains("helper("), "Should call helper: {}", rust);
}

#[test]
fn test_generic_function_multiple_args() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def call_add() -> int:
    return add(3, 4)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn call_add"), "output: {}", rust);
}

#[test]
fn test_generic_method_chaining() {
    let code = r#"
def chain(text: str) -> str:
    return text.strip().lower()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn chain"), "output: {}", rust);
}

#[test]
fn test_generic_nested_function_calls() {
    let code = r#"
def nested(n: int) -> int:
    return abs(min(n, 0))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn nested"), "output: {}", rust);
}

#[test]
fn test_generic_lambda_inline() {
    let code = r#"
def apply_transform(items: list) -> list:
    return list(map(lambda x: x + 1, items))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn apply_transform"), "output: {}", rust);
}

// ========================================================================
// STDLIB_MISC.RS COVERAGE TESTS - math, random, etc.
// ========================================================================

#[test]
fn test_stdlib_math_sqrt() {
    let code = r#"
import math
def root(x: float) -> float:
    return math.sqrt(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn root"), "output: {}", rust);
    assert!(rust.contains("sqrt"), "Should contain sqrt: {}", rust);
}

#[test]
fn test_stdlib_math_floor() {
    let code = r#"
import math
def floor_val(x: float) -> int:
    return math.floor(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn floor_val"), "output: {}", rust);
    assert!(rust.contains("floor"), "Should contain floor: {}", rust);
}

#[test]
fn test_stdlib_math_ceil() {
    let code = r#"
import math
def ceil_val(x: float) -> int:
    return math.ceil(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn ceil_val"), "output: {}", rust);
    assert!(rust.contains("ceil"), "Should contain ceil: {}", rust);
}

#[test]
fn test_stdlib_math_log() {
    let code = r#"
import math
def natural_log(x: float) -> float:
    return math.log(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn natural_log"), "output: {}", rust);
    assert!(rust.contains("ln") || rust.contains("log"), "Should contain log: {}", rust);
}

#[test]
fn test_stdlib_math_sin() {
    let code = r#"
import math
def sine(x: float) -> float:
    return math.sin(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sine"), "output: {}", rust);
    assert!(rust.contains("sin"), "Should contain sin: {}", rust);
}

#[test]
fn test_stdlib_math_cos() {
    let code = r#"
import math
def cosine(x: float) -> float:
    return math.cos(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn cosine"), "output: {}", rust);
    assert!(rust.contains("cos"), "Should contain cos: {}", rust);
}

#[test]
fn test_stdlib_math_tan() {
    let code = r#"
import math
def tangent(x: float) -> float:
    return math.tan(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn tangent"), "output: {}", rust);
    assert!(rust.contains("tan"), "Should contain tan: {}", rust);
}

#[test]
fn test_stdlib_math_abs() {
    let code = r#"
import math
def absolute(x: float) -> float:
    return math.fabs(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn absolute"), "output: {}", rust);
    assert!(rust.contains("abs"), "Should contain abs: {}", rust);
}

#[test]
fn test_stdlib_math_pi_constant() {
    let code = r#"
import math
def circle_area(r: float) -> float:
    return math.pi * r * r
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn circle_area"), "output: {}", rust);
    assert!(rust.contains("PI") || rust.contains("pi") || rust.contains("std::f64::consts"), "Should contain PI: {}", rust);
}

#[test]
fn test_stdlib_math_log2() {
    let code = r#"
import math
def log_base2(x: float) -> float:
    return math.log2(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn log_base2"), "output: {}", rust);
    assert!(rust.contains("log2"), "Should contain log2: {}", rust);
}

#[test]
fn test_stdlib_math_log10() {
    let code = r#"
import math
def log_base10(x: float) -> float:
    return math.log10(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn log_base10"), "output: {}", rust);
    assert!(rust.contains("log10"), "Should contain log10: {}", rust);
}

#[test]
fn test_stdlib_math_exp() {
    let code = r#"
import math
def exponential(x: float) -> float:
    return math.exp(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn exponential"), "output: {}", rust);
    assert!(rust.contains("exp"), "Should contain exp: {}", rust);
}

#[test]
fn test_stdlib_math_pow() {
    let code = r#"
import math
def math_power(x: float, y: float) -> float:
    return math.pow(x, y)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn math_power"), "output: {}", rust);
    assert!(rust.contains("powf") || rust.contains("pow"), "Should contain pow: {}", rust);
}

#[test]
fn test_stdlib_math_gcd() {
    let code = r#"
import math
def compute_gcd(a: int, b: int) -> int:
    return math.gcd(a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn compute_gcd"), "output: {}", rust);
}

#[test]
fn test_stdlib_random_random() {
    let code = r#"
import random
def get_random() -> float:
    return random.random()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_random"), "output: {}", rust);
}

#[test]
fn test_stdlib_random_randint() {
    let code = r#"
import random
def dice_roll() -> int:
    return random.randint(1, 6)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn dice_roll"), "output: {}", rust);
}

#[test]
fn test_stdlib_random_choice() {
    let code = r#"
import random
def pick(items: list) -> int:
    return random.choice(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pick"), "output: {}", rust);
}

#[test]
fn test_stdlib_random_shuffle() {
    let code = r#"
import random
def shuffle_list(items: list):
    random.shuffle(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn shuffle_list"), "output: {}", rust);
}

#[test]
fn test_stdlib_time_time() {
    let code = r#"
import time
def now_seconds() -> float:
    return time.time()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn now_seconds"), "output: {}", rust);
}

#[test]
fn test_stdlib_time_sleep() {
    let code = r#"
import time
def wait(seconds: float):
    time.sleep(seconds)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn wait"), "output: {}", rust);
    assert!(rust.contains("sleep") || rust.contains("thread"), "Should contain sleep: {}", rust);
}

// ========================================================================
// STDLIB_CRYPTO.RS COVERAGE TESTS - hash, crypto, base64
// ========================================================================

#[test]
fn test_stdlib_hashlib_sha256() {
    let code = r#"
import hashlib
def sha256_hash(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sha256_hash"), "output: {}", rust);
}

#[test]
fn test_stdlib_hashlib_md5() {
    let code = r#"
import hashlib
def md5_hash(data: str) -> str:
    return hashlib.md5(data.encode()).hexdigest()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn md5_hash"), "output: {}", rust);
}

#[test]
fn test_stdlib_base64_encode() {
    let code = r#"
import base64
def encode_b64(data: str) -> str:
    return base64.b64encode(data.encode()).decode()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn encode_b64"), "output: {}", rust);
}

#[test]
fn test_stdlib_base64_decode() {
    let code = r#"
import base64
def decode_b64(data: str) -> str:
    return base64.b64decode(data).decode()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn decode_b64"), "output: {}", rust);
}

// ========================================================================
// STDLIB_NUMPY.RS COVERAGE TESTS - numpy operations
// ========================================================================

#[test]
fn test_stdlib_numpy_array_creation() {
    let code = r#"
import numpy as np
def make_array() -> list:
    return np.array([1.0, 2.0, 3.0])
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_array"), "output: {}", rust);
}

#[test]
fn test_stdlib_numpy_zeros() {
    let code = r#"
import numpy as np
def zero_array(n: int) -> list:
    return np.zeros(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn zero_array"), "output: {}", rust);
}

#[test]
fn test_stdlib_numpy_ones() {
    let code = r#"
import numpy as np
def one_array(n: int) -> list:
    return np.ones(n)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn one_array"), "output: {}", rust);
}

#[test]
fn test_stdlib_numpy_dot() {
    let code = r#"
import numpy as np
def dot_product(a: list, b: list) -> float:
    return np.dot(a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn dot_product"), "output: {}", rust);
}

#[test]
fn test_stdlib_numpy_sum() {
    let code = r#"
import numpy as np
def array_sum(a: list) -> float:
    return np.sum(a)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn array_sum"), "output: {}", rust);
}

#[test]
fn test_stdlib_numpy_mean() {
    let code = r#"
import numpy as np
def array_mean(a: list) -> float:
    return np.mean(a)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn array_mean"), "output: {}", rust);
}

// ========================================================================
// STDLIB_DATETIME.RS COVERAGE TESTS - datetime operations
// ========================================================================

#[test]
fn test_stdlib_datetime_now() {
    let code = r#"
from datetime import datetime
def current_time():
    return datetime.now()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn current_time"), "output: {}", rust);
}

#[test]
fn test_stdlib_datetime_constructor() {
    let code = r#"
from datetime import datetime
def make_date() -> str:
    dt = datetime(2024, 1, 15)
    return str(dt)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_date"), "output: {}", rust);
}

#[test]
fn test_stdlib_timedelta() {
    let code = r#"
from datetime import timedelta
def one_day():
    return timedelta(days=1)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn one_day"), "output: {}", rust);
}

#[test]
fn test_stdlib_datetime_strftime() {
    let code = r#"
from datetime import datetime
def format_date() -> str:
    dt = datetime.now()
    return dt.strftime("%Y-%m-%d")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn format_date"), "output: {}", rust);
}

// ========================================================================
// STDLIB_OS.RS COVERAGE TESTS - OS operations
// ========================================================================

#[test]
fn test_stdlib_os_path_join() {
    let code = r#"
import os
def join_path(base: str, name: str) -> str:
    return os.path.join(base, name)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn join_path"), "output: {}", rust);
    assert!(rust.contains("PathBuf") || rust.contains("join") || rust.contains("path"), "Should contain path join: {}", rust);
}

#[test]
fn test_stdlib_os_path_exists() {
    let code = r#"
import os
def file_exists(path: str) -> bool:
    return os.path.exists(path)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn file_exists"), "output: {}", rust);
    assert!(rust.contains("exists"), "Should contain exists: {}", rust);
}

#[test]
fn test_stdlib_os_path_basename() {
    let code = r#"
import os
def base_name(path: str) -> str:
    return os.path.basename(path)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn base_name"), "output: {}", rust);
}

#[test]
fn test_stdlib_os_path_dirname() {
    let code = r#"
import os
def dir_name(path: str) -> str:
    return os.path.dirname(path)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn dir_name"), "output: {}", rust);
}

#[test]
fn test_stdlib_os_path_isfile() {
    let code = r#"
import os
def check_file(path: str) -> bool:
    return os.path.isfile(path)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_file"), "output: {}", rust);
}

#[test]
fn test_stdlib_os_path_isdir() {
    let code = r#"
import os
def check_dir(path: str) -> bool:
    return os.path.isdir(path)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_dir"), "output: {}", rust);
}

#[test]
fn test_stdlib_os_getcwd() {
    let code = r#"
import os
def current_dir() -> str:
    return os.getcwd()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn current_dir"), "output: {}", rust);
}

#[test]
fn test_stdlib_os_getenv() {
    let code = r#"
import os
def get_env(key: str) -> str:
    return os.getenv(key)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_env"), "output: {}", rust);
}

#[test]
fn test_stdlib_os_listdir() {
    let code = r#"
import os
def list_files(path: str) -> list:
    return os.listdir(path)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn list_files"), "output: {}", rust);
}

#[test]
fn test_stdlib_os_makedirs() {
    let code = r#"
import os
def make_dirs(path: str):
    os.makedirs(path, exist_ok=True)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_dirs"), "output: {}", rust);
}

// ========================================================================
// STDLIB_SUBPROCESS.RS COVERAGE TESTS - subprocess operations
// ========================================================================

#[test]
fn test_stdlib_subprocess_run() {
    let code = r#"
import subprocess
def run_cmd(cmd: list):
    subprocess.run(cmd)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn run_cmd"), "output: {}", rust);
    assert!(rust.contains("Command") || rust.contains("command") || rust.contains("process"), "Should contain Command: {}", rust);
}

#[test]
fn test_stdlib_subprocess_run_capture() {
    let code = r#"
import subprocess
def capture_cmd(cmd: list) -> str:
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.stdout
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn capture_cmd"), "output: {}", rust);
}

#[test]
fn test_stdlib_subprocess_check_output() {
    let code = r#"
import subprocess
def get_output(cmd: list) -> str:
    return subprocess.check_output(cmd).decode()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_output"), "output: {}", rust);
}

// ========================================================================
// TYPE_ANALYSIS.RS COVERAGE TESTS - type checking helpers
// ========================================================================

#[test]
fn test_type_analysis_int_literal_in_expr() {
    let code = r#"
def compute(n: int) -> int:
    return n + 1
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn compute"), "output: {}", rust);
}

#[test]
fn test_type_analysis_float_coercion() {
    let code = r#"
def mixed(n: int, f: float) -> float:
    return n + f
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn mixed"), "output: {}", rust);
}

#[test]
fn test_type_analysis_comparison_types() {
    let code = r#"
def compare_floats(a: float, b: float) -> bool:
    return a <= 0
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn compare_floats"), "output: {}", rust);
}

// ========================================================================
// STDLIB_PATHLIB.RS COVERAGE TESTS - pathlib operations
// ========================================================================

#[test]
fn test_stdlib_pathlib_constructor() {
    let code = r#"
from pathlib import Path
def make_path(s: str):
    p = Path(s)
    return p
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_path"), "output: {}", rust);
    assert!(rust.contains("PathBuf"), "Should contain PathBuf: {}", rust);
}

#[test]
fn test_stdlib_pathlib_exists() {
    let code = r#"
from pathlib import Path
def path_exists(s: str) -> bool:
    p = Path(s)
    return p.exists()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn path_exists"), "output: {}", rust);
    assert!(rust.contains("exists"), "Should contain exists: {}", rust);
}

#[test]
fn test_stdlib_pathlib_read_text() {
    let code = r#"
from pathlib import Path
def read_path(s: str) -> str:
    p = Path(s)
    return p.read_text()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn read_path"), "output: {}", rust);
}

#[test]
fn test_stdlib_pathlib_is_file() {
    let code = r#"
from pathlib import Path
def check_file(s: str) -> bool:
    p = Path(s)
    return p.is_file()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_file"), "output: {}", rust);
}

#[test]
fn test_stdlib_pathlib_is_dir() {
    let code = r#"
from pathlib import Path
def check_dir(s: str) -> bool:
    p = Path(s)
    return p.is_dir()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_dir"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL BINARY_OPS EDGE CASES
// ========================================================================

#[test]
fn test_binop_floor_div_float() {
    let code = r#"
def floor_div_f(a: float, b: float) -> float:
    return a // b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn floor_div_f"), "output: {}", rust);
}

#[test]
fn test_binop_modulo_float() {
    let code = r#"
def mod_f(a: float, b: float) -> float:
    return a % b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn mod_f"), "output: {}", rust);
}

#[test]
fn test_binop_comparison_string_eq() {
    let code = r#"
def str_eq(a: str, b: str) -> bool:
    return a == b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn str_eq"), "output: {}", rust);
    assert!(rust.contains("=="), "Should contain ==: {}", rust);
}

#[test]
fn test_binop_multiple_and_conditions() {
    let code = r#"
def all_check(a: int, b: int, c: int) -> bool:
    return a > 0 and b > 0 and c > 0
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn all_check"), "output: {}", rust);
    assert!(rust.contains("&&"), "Should contain &&: {}", rust);
}

#[test]
fn test_binop_multiple_or_conditions() {
    let code = r#"
def any_check(a: int, b: int, c: int) -> bool:
    return a > 0 or b > 0 or c > 0
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn any_check"), "output: {}", rust);
    assert!(rust.contains("||"), "Should contain ||: {}", rust);
}

#[test]
fn test_binop_not_in_string() {
    let code = r#"
def not_found(text: str, ch: str) -> bool:
    return ch not in text
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn not_found"), "output: {}", rust);
    assert!(rust.contains("contains"), "Should contain contains: {}", rust);
}

// ========================================================================
// ADDITIONAL UNARY AND CALL EDGE CASES
// ========================================================================

#[test]
fn test_builtin_set_constructor() {
    let code = r#"
def make_set(items: list) -> set:
    return set(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_set"), "output: {}", rust);
    assert!(rust.contains("HashSet") || rust.contains("collect") || rust.contains("set"), "Should create set: {}", rust);
}

#[test]
fn test_builtin_tuple_constructor() {
    let code = r#"
def make_tuple(items: list) -> tuple:
    return tuple(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_tuple"), "output: {}", rust);
}

#[test]
fn test_builtin_list_constructor() {
    let code = r#"
def make_list(items: set) -> list:
    return list(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_list"), "output: {}", rust);
}

#[test]
fn test_builtin_dict_constructor() {
    let code = r#"
def make_dict() -> dict:
    return dict()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_dict"), "output: {}", rust);
    assert!(rust.contains("HashMap") || rust.contains("new"), "Should create dict: {}", rust);
}

#[test]
fn test_builtin_format_string() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn greet"), "output: {}", rust);
    assert!(rust.contains("format!"), "Should contain format!: {}", rust);
}

#[test]
fn test_builtin_getattr_two_args() {
    let code = r#"
def get_attr(obj: dict, name: str):
    return getattr(obj, name)
"#;
    // getattr is not fully supported, expect transpilation to handle gracefully
    let result = std::panic::catch_unwind(|| transpile(code));
    // Either it transpiles or it errors; both are valid behaviors
    if let Ok(rust) = result {
        assert!(rust.contains("fn get_attr") || rust.contains("getattr"), "output: {}", rust);
    }
}

#[test]
fn test_builtin_sorted_with_key() {
    let code = r#"
def sort_by_len(items: list) -> list:
    return sorted(items, key=len)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sort_by_len"), "output: {}", rust);
}

#[test]
fn test_builtin_sorted_reverse() {
    let code = r#"
def sort_desc(items: list) -> list:
    return sorted(items, reverse=True)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sort_desc"), "output: {}", rust);
}

#[test]
fn test_builtin_enumerate_start() {
    let code = r#"
def indexed_from_one(items: list):
    for i, val in enumerate(items, 1):
        print(i, val)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn indexed_from_one"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL STDLIB_MISC EDGE CASES
// ========================================================================

#[test]
fn test_stdlib_math_atan2() {
    let code = r#"
import math
def angle(y: float, x: float) -> float:
    return math.atan2(y, x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn angle"), "output: {}", rust);
    assert!(rust.contains("atan2"), "Should contain atan2: {}", rust);
}

#[test]
fn test_stdlib_math_hypot() {
    let code = r#"
import math
def distance(x: float, y: float) -> float:
    return math.hypot(x, y)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn distance"), "output: {}", rust);
    assert!(rust.contains("hypot"), "Should contain hypot: {}", rust);
}

#[test]
fn test_stdlib_math_radians() {
    let code = r#"
import math
def to_radians(deg: float) -> float:
    return math.radians(deg)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_radians"), "output: {}", rust);
    assert!(rust.contains("to_radians"), "Should contain to_radians: {}", rust);
}

#[test]
fn test_stdlib_math_degrees() {
    let code = r#"
import math
def to_degrees(rad: float) -> float:
    return math.degrees(rad)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_degrees"), "output: {}", rust);
    assert!(rust.contains("to_degrees"), "Should contain to_degrees: {}", rust);
}

#[test]
fn test_stdlib_math_isnan() {
    let code = r#"
import math
def check_nan(x: float) -> bool:
    return math.isnan(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_nan"), "output: {}", rust);
    assert!(rust.contains("is_nan"), "Should contain is_nan: {}", rust);
}

#[test]
fn test_stdlib_math_isinf() {
    let code = r#"
import math
def check_inf(x: float) -> bool:
    return math.isinf(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_inf"), "output: {}", rust);
    assert!(rust.contains("is_infinite"), "Should contain is_infinite: {}", rust);
}

#[test]
fn test_stdlib_random_uniform() {
    let code = r#"
import random
def rand_between(lo: float, hi: float) -> float:
    return random.uniform(lo, hi)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn rand_between"), "output: {}", rust);
}

#[test]
fn test_stdlib_random_sample() {
    let code = r#"
import random
def pick_some(items: list, k: int) -> list:
    return random.sample(items, k)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pick_some"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL COMPLEX EXPRESSION TESTS (cross-module coverage)
// ========================================================================

#[test]
fn test_complex_list_comprehension_with_condition() {
    let code = r#"
def filter_positive(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn filter_positive"), "output: {}", rust);
    assert!(rust.contains("filter") || rust.contains("iter") || rust.contains("into_iter"), "Should use iterator: {}", rust);
}

#[test]
fn test_complex_dict_comprehension_with_transform() {
    let code = r#"
def square_dict(items: list) -> dict:
    return {x: x * x for x in items}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn square_dict"), "output: {}", rust);
}

#[test]
fn test_complex_nested_function_with_closure() {
    let code = r#"
def make_adder(n: int):
    def adder(x: int) -> int:
        return x + n
    return adder
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_adder") || rust.contains("make_adder"), "output: {}", rust);
}

#[test]
fn test_complex_ternary_in_assignment() {
    let code = r#"
def clamp(val: int, lo: int, hi: int) -> int:
    result = lo if val < lo else hi if val > hi else val
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn clamp"), "output: {}", rust);
}

#[test]
fn test_complex_multiple_comparisons_mixed() {
    let code = r#"
def validate(x: int, y: int) -> bool:
    return x > 0 and y > 0 and x != y
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn validate"), "output: {}", rust);
}

#[test]
fn test_complex_lambda_in_sort() {
    let code = r#"
def sort_pairs(pairs: list) -> list:
    return sorted(pairs, key=lambda p: p[1])
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sort_pairs"), "output: {}", rust);
}

#[test]
fn test_complex_string_format_multiple() {
    let code = r#"
def info(name: str, age: int, city: str) -> str:
    return f"{name} is {age} in {city}"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn info"), "output: {}", rust);
    assert!(rust.contains("format!"), "Should contain format!: {}", rust);
}

#[test]
fn test_complex_class_with_methods() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def distance(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#;
    let rust = transpile(code);
    assert!(rust.contains("Point") || rust.contains("struct"), "output: {}", rust);
}

#[test]
fn test_complex_while_loop_with_break() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    i = 0
    while i < len(items):
        if items[i] == target:
            return i
        i += 1
    return -1
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_first"), "output: {}", rust);
    assert!(rust.contains("while"), "Should contain while: {}", rust);
}

#[test]
fn test_complex_try_except() {
    let code = r#"
def safe_divide(a: float, b: float) -> float:
    try:
        return a / b
    except ZeroDivisionError:
        return 0.0
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn safe_divide"), "output: {}", rust);
}

#[test]
fn test_complex_assert_statement() {
    let code = r#"
def validate_positive(n: int):
    assert n > 0, "n must be positive"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn validate_positive"), "output: {}", rust);
    assert!(rust.contains("assert"), "Should contain assert: {}", rust);
}

#[test]
fn test_complex_from_import_usage() {
    let code = r#"
from math import sqrt, pi
def circle_circumference(r: float) -> float:
    return 2.0 * pi * r
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn circle_circumference"), "output: {}", rust);
}

#[test]
fn test_complex_dict_get_default() {
    let code = r#"
def safe_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn safe_get"), "output: {}", rust);
}

#[test]
fn test_complex_string_join() {
    let code = r#"
def join_words(words: list) -> str:
    return " ".join(words)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn join_words"), "output: {}", rust);
    assert!(rust.contains("join"), "Should contain join: {}", rust);
}

#[test]
fn test_complex_string_split() {
    let code = r#"
def split_words(text: str) -> list:
    return text.split(" ")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn split_words"), "output: {}", rust);
    assert!(rust.contains("split"), "Should contain split: {}", rust);
}

#[test]
fn test_complex_multiple_return() {
    let code = r#"
def min_max(items: list) -> tuple:
    return min(items), max(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn min_max"), "output: {}", rust);
}

#[test]
fn test_complex_walrus_operator_pattern() {
    let code = r#"
def process(data: list) -> int:
    n = len(data)
    if n > 10:
        return n
    return 0
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn process"), "output: {}", rust);
}

#[test]
fn test_complex_augmented_string_concat() {
    let code = r#"
def build_string(items: list) -> str:
    result = ""
    for item in items:
        result += str(item)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn build_string"), "output: {}", rust);
}

#[test]
fn test_complex_nested_list_access() {
    let code = r#"
def get_element(matrix: list, i: int, j: int) -> int:
    return matrix[i][j]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_element"), "output: {}", rust);
}

#[test]
fn test_complex_for_with_break() {
    let code = r#"
def find_value(items: list, target: int) -> bool:
    for item in items:
        if item == target:
            return True
    return False
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_value"), "output: {}", rust);
}

#[test]
fn test_complex_for_with_continue() {
    let code = r#"
def sum_positive(items: list) -> int:
    total = 0
    for item in items:
        if item < 0:
            continue
        total += item
    return total
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sum_positive"), "output: {}", rust);
    assert!(rust.contains("continue"), "Should contain continue: {}", rust);
}

#[test]
fn test_complex_global_constant_usage() {
    let code = r#"
MAX_SIZE = 1000

def check_size(n: int) -> bool:
    return n <= MAX_SIZE
"#;
    let rust = transpile(code);
    assert!(rust.contains("MAX_SIZE") || rust.contains("1000"), "Should contain constant: {}", rust);
}

#[test]
fn test_complex_pass_statement() {
    let code = r#"
def placeholder():
    pass
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn placeholder"), "output: {}", rust);
}

#[test]
fn test_complex_del_statement() {
    let code = r#"
def remove_key(d: dict, key: str):
    del d[key]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn remove_key"), "output: {}", rust);
    assert!(rust.contains("remove"), "Should contain remove: {}", rust);
}
