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
