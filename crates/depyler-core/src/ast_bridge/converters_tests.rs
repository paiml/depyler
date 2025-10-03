use crate::ast_bridge::converters::{ExprConverter, StmtConverter};
use crate::hir::*;
use rustpython_ast::bigint::BigInt;
use rustpython_ast::{self as ast, Constant, Expr, ExprConstant, ExprName};
use rustpython_parser::Parse;

// Helper function to parse Python expressions
fn parse_expr(code: &str) -> ast::Expr {
    Expr::parse(code, "<test>").unwrap()
}

// Helper function to parse Python statements
fn parse_stmt(code: &str) -> ast::Stmt {
    let module = rustpython_parser::parse(code, rustpython_parser::Mode::Module, "<test>").unwrap();
    match module {
        rustpython_parser::ast::Mod::Module(m) => m.body.into_iter().next().unwrap(),
        _ => panic!("Expected Module"),
    }
}

#[test]
fn test_convert_constant_int() {
    let expr = ExprConstant {
        value: Constant::Int(BigInt::from(42)),
        kind: None,
        range: Default::default(),
    };
    let result = ExprConverter::convert(Expr::Constant(expr)).unwrap();
    assert!(matches!(result, HirExpr::Literal(Literal::Int(42))));
}

#[test]
fn test_convert_constant_float() {
    let expr = ExprConstant {
        value: Constant::Float(3.5),
        kind: None,
        range: Default::default(),
    };
    let result = ExprConverter::convert(Expr::Constant(expr)).unwrap();
    assert!(matches!(result, HirExpr::Literal(Literal::Float(f)) if (f - 3.5).abs() < 0.001));
}

#[test]
fn test_convert_constant_string() {
    let expr = ExprConstant {
        value: Constant::Str("hello".into()),
        kind: None,
        range: Default::default(),
    };
    let result = ExprConverter::convert(Expr::Constant(expr)).unwrap();
    match result {
        HirExpr::Literal(Literal::String(s)) => assert_eq!(s, "hello"),
        _ => panic!("Expected string literal"),
    }
}

#[test]
fn test_convert_constant_bool() {
    let expr = ExprConstant {
        value: Constant::Bool(true),
        kind: None,
        range: Default::default(),
    };
    let result = ExprConverter::convert(Expr::Constant(expr)).unwrap();
    assert!(matches!(result, HirExpr::Literal(Literal::Bool(true))));
}

#[test]
fn test_convert_constant_none() {
    let expr = ExprConstant {
        value: Constant::None,
        kind: None,
        range: Default::default(),
    };
    let result = ExprConverter::convert(Expr::Constant(expr)).unwrap();
    assert!(matches!(result, HirExpr::Literal(Literal::None)));
}

#[test]
fn test_convert_name() {
    let expr = ExprName {
        id: "variable".into(),
        ctx: ast::ExprContext::Load,
        range: Default::default(),
    };
    let result = ExprConverter::convert(Expr::Name(expr)).unwrap();
    match result {
        HirExpr::Var(name) => assert_eq!(name, "variable"),
        _ => panic!("Expected variable"),
    }
}

#[test]
fn test_convert_binop_add() {
    let expr = parse_expr("1 + 2");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Binary { op, left, right } => {
            assert!(matches!(op, BinOp::Add));
            assert!(matches!(*left, HirExpr::Literal(Literal::Int(1))));
            assert!(matches!(*right, HirExpr::Literal(Literal::Int(2))));
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_convert_unaryop_neg() {
    let expr = parse_expr("-42");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Unary { op, operand } => {
            assert!(matches!(op, UnaryOp::Neg));
            assert!(matches!(*operand, HirExpr::Literal(Literal::Int(42))));
        }
        _ => panic!("Expected unary operation"),
    }
}

#[test]
fn test_convert_call_simple() {
    let expr = parse_expr("print('hello')");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Call { func, args } => {
            assert_eq!(func, "print");
            assert_eq!(args.len(), 1);
            match &args[0] {
                HirExpr::Literal(Literal::String(s)) => assert_eq!(s, "hello"),
                _ => panic!("Expected string argument"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_convert_method_call() {
    let expr = parse_expr("obj.method(1, 2)");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::MethodCall {
            object,
            method,
            args,
        } => {
            assert!(matches!(*object, HirExpr::Var(ref name) if name == "obj"));
            assert_eq!(method, "method");
            assert_eq!(args.len(), 2);
        }
        _ => panic!("Expected method call"),
    }
}

#[test]
fn test_convert_list() {
    let expr = parse_expr("[1, 2, 3]");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::List(elems) => {
            assert_eq!(elems.len(), 3);
            assert!(matches!(elems[0], HirExpr::Literal(Literal::Int(1))));
            assert!(matches!(elems[1], HirExpr::Literal(Literal::Int(2))));
            assert!(matches!(elems[2], HirExpr::Literal(Literal::Int(3))));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_convert_dict() {
    let expr = parse_expr("{'a': 1, 'b': 2}");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Dict(items) => {
            assert_eq!(items.len(), 2);
            match &items[0] {
                (HirExpr::Literal(Literal::String(k)), HirExpr::Literal(Literal::Int(v))) => {
                    assert_eq!(k, "a");
                    assert_eq!(*v, 1);
                }
                _ => panic!("Expected string key and int value"),
            }
        }
        _ => panic!("Expected dict"),
    }
}

#[test]
fn test_convert_tuple() {
    let expr = parse_expr("(1, 'hello', True)");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Tuple(elems) => {
            assert_eq!(elems.len(), 3);
            assert!(matches!(elems[0], HirExpr::Literal(Literal::Int(1))));
            assert!(matches!(elems[1], HirExpr::Literal(Literal::String(ref s)) if s == "hello"));
            assert!(matches!(elems[2], HirExpr::Literal(Literal::Bool(true))));
        }
        _ => panic!("Expected tuple"),
    }
}

#[test]
fn test_convert_set() {
    let expr = parse_expr("{1, 2, 3}");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Set(elems) => {
            assert_eq!(elems.len(), 3);
        }
        _ => panic!("Expected set"),
    }
}

#[test]
fn test_convert_subscript_index() {
    let expr = parse_expr("arr[0]");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Index { base, index } => {
            assert!(matches!(*base, HirExpr::Var(ref name) if name == "arr"));
            assert!(matches!(*index, HirExpr::Literal(Literal::Int(0))));
        }
        _ => panic!("Expected index operation"),
    }
}

#[test]
fn test_convert_slice() {
    let expr = parse_expr("arr[1:5]");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Slice {
            base,
            start,
            stop,
            step,
        } => {
            assert!(matches!(*base, HirExpr::Var(ref name) if name == "arr"));
            assert!(
                matches!(start, Some(ref s) if matches!(**s, HirExpr::Literal(Literal::Int(1))))
            );
            assert!(
                matches!(stop, Some(ref s) if matches!(**s, HirExpr::Literal(Literal::Int(5))))
            );
            assert!(step.is_none());
        }
        _ => panic!("Expected slice operation"),
    }
}

#[test]
fn test_convert_compare() {
    let expr = parse_expr("a > b");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Binary { op, left, right } => {
            assert!(matches!(op, BinOp::Gt));
            assert!(matches!(*left, HirExpr::Var(ref name) if name == "a"));
            assert!(matches!(*right, HirExpr::Var(ref name) if name == "b"));
        }
        _ => panic!("Expected comparison"),
    }
}

#[test]
fn test_convert_list_comp() {
    let expr = parse_expr("[x * 2 for x in range(10)]");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::ListComp {
            element,
            target,
            iter: _,
            condition,
        } => {
            assert_eq!(target, "x");
            assert!(condition.is_none());
            // element should be x * 2
            match element.as_ref() {
                HirExpr::Binary { op, .. } => assert!(matches!(op, BinOp::Mul)),
                _ => panic!("Expected binary operation in element"),
            }
        }
        _ => panic!("Expected list comprehension"),
    }
}

#[test]
fn test_convert_list_comp_with_condition() {
    let expr = parse_expr("[x for x in range(10) if x % 2 == 0]");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::ListComp {
            target, condition, ..
        } => {
            assert_eq!(target, "x");
            assert!(condition.is_some());
        }
        _ => panic!("Expected list comprehension"),
    }
}

#[test]
fn test_convert_lambda() {
    let expr = parse_expr("lambda x, y: x + y");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Lambda { params, body } => {
            assert_eq!(params, vec!["x", "y"]);
            assert!(matches!(*body, HirExpr::Binary { op: BinOp::Add, .. }));
        }
        _ => panic!("Expected lambda"),
    }
}

#[test]
fn test_convert_attribute() {
    let expr = parse_expr("obj.attr");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Attribute { value, attr } => {
            assert!(matches!(*value, HirExpr::Var(ref name) if name == "obj"));
            assert_eq!(attr, "attr");
        }
        _ => panic!("Expected attribute"),
    }
}

#[test]
fn test_convert_await() {
    let expr = parse_expr("await async_func()");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::Await { value } => {
            assert!(matches!(*value, HirExpr::Call { .. }));
        }
        _ => panic!("Expected await"),
    }
}

// Statement converter tests
#[test]
fn test_convert_assign() {
    let stmt = parse_stmt("x = 42");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::Assign { target, value } => {
            assert!(matches!(target, AssignTarget::Symbol(ref s) if s == "x"));
            assert!(matches!(value, HirExpr::Literal(Literal::Int(42))));
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_convert_aug_assign() {
    let stmt = parse_stmt("x += 1");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::Assign { target, value } => {
            assert!(matches!(target, AssignTarget::Symbol(ref s) if s == "x"));
            assert!(matches!(value, HirExpr::Binary { op: BinOp::Add, .. }));
        }
        _ => panic!("Expected augmented assignment"),
    }
}

#[test]
fn test_convert_return() {
    let stmt = parse_stmt("return 42");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::Return(Some(expr)) => {
            assert!(matches!(expr, HirExpr::Literal(Literal::Int(42))));
        }
        _ => panic!("Expected return statement"),
    }
}

#[test]
fn test_convert_return_none() {
    let stmt = parse_stmt("return");
    let result = StmtConverter::convert(stmt).unwrap();
    assert!(matches!(result, HirStmt::Return(None)));
}

#[test]
fn test_convert_if() {
    let stmt = parse_stmt("if x > 0:\n    print('positive')");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            assert!(matches!(condition, HirExpr::Binary { op: BinOp::Gt, .. }));
            assert_eq!(then_body.len(), 1);
            assert!(else_body.is_none());
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_convert_if_else() {
    let stmt = parse_stmt("if x > 0:\n    print('positive')\nelse:\n    print('negative')");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::If { else_body, .. } => {
            assert!(else_body.is_some());
            assert_eq!(else_body.unwrap().len(), 1);
        }
        _ => panic!("Expected if-else statement"),
    }
}

#[test]
fn test_convert_while() {
    let stmt = parse_stmt("while x > 0:\n    x -= 1");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::While { condition, body } => {
            assert!(matches!(condition, HirExpr::Binary { op: BinOp::Gt, .. }));
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected while statement"),
    }
}

#[test]
fn test_convert_for() {
    let stmt = parse_stmt("for i in range(10):\n    print(i)");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::For { target, iter, body } => {
            assert_eq!(target, "i");
            assert!(matches!(iter, HirExpr::Call { .. }));
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected for statement"),
    }
}

#[test]
fn test_convert_expr_stmt() {
    let stmt = parse_stmt("print('hello')");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::Expr(expr) => {
            assert!(matches!(expr, HirExpr::Call { .. }));
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_convert_break() {
    let stmt = parse_stmt("break");
    let result = StmtConverter::convert(stmt).unwrap();
    assert!(matches!(result, HirStmt::Break { label: None }));
}

#[test]
fn test_convert_continue() {
    let stmt = parse_stmt("continue");
    let result = StmtConverter::convert(stmt).unwrap();
    assert!(matches!(result, HirStmt::Continue { label: None }));
}

#[test]
fn test_convert_raise() {
    let stmt = parse_stmt("raise ValueError('error')");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::Raise { exception, cause } => {
            assert!(exception.is_some());
            assert!(cause.is_none());
        }
        _ => panic!("Expected raise statement"),
    }
}

#[test]
fn test_convert_with() {
    let stmt = parse_stmt("with open('file') as f:\n    data = f.read()");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::With {
            context,
            target,
            body,
        } => {
            assert!(matches!(context, HirExpr::Call { .. }));
            assert_eq!(target, Some("f".to_string()));
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected with statement"),
    }
}

#[test]
fn test_convert_ann_assign() {
    let stmt = parse_stmt("x: int = 42");
    let result = StmtConverter::convert(stmt).unwrap();
    match result {
        HirStmt::Assign { target, value } => {
            assert!(matches!(target, AssignTarget::Symbol(ref s) if s == "x"));
            assert!(matches!(value, HirExpr::Literal(Literal::Int(42))));
        }
        _ => panic!("Expected annotated assignment"),
    }
}

#[test]
fn test_convert_set_comp() {
    let expr = parse_expr("{x * 2 for x in range(10)}");
    let result = ExprConverter::convert(expr).unwrap();
    match result {
        HirExpr::SetComp {
            element,
            target,
            iter: _,
            condition,
        } => {
            assert_eq!(target, "x");
            assert!(condition.is_none());
            match element.as_ref() {
                HirExpr::Binary { op, .. } => assert!(matches!(op, BinOp::Mul)),
                _ => panic!("Expected binary operation in element"),
            }
        }
        _ => panic!("Expected set comprehension"),
    }
}

#[test]
fn test_error_on_unsupported_expr() {
    // Try to convert a yield expression (not supported)
    let expr = parse_expr("(yield 42)");
    let result = ExprConverter::convert(expr);
    assert!(result.is_err());
}

#[test]
fn test_error_on_chained_comparison() {
    let expr = parse_expr("a < b < c");
    let result = ExprConverter::convert(expr);
    assert!(result.is_err());
}

#[test]
fn test_error_on_multiple_assign_targets() {
    let stmt = parse_stmt("a, b = 1, 2");
    let result = StmtConverter::convert(stmt);
    assert!(result.is_err());
}

// DEPYLER-0101: Tests for 'is None' / 'is not None' operator support
#[test]
fn test_is_none_converts_to_method_call() {
    let expr = parse_expr("x is None");
    let result = ExprConverter::convert(expr).unwrap();

    match result {
        HirExpr::MethodCall { object, method, args } => {
            assert_eq!(method, "is_none");
            assert!(args.is_empty());
            // Object should be the variable 'x'
            assert!(matches!(*object, HirExpr::Var(_)));
        }
        _ => panic!("Expected MethodCall, got {:?}", result),
    }
}

#[test]
fn test_is_not_none_converts_to_is_some() {
    let expr = parse_expr("x is not None");
    let result = ExprConverter::convert(expr).unwrap();

    match result {
        HirExpr::MethodCall { object, method, args } => {
            assert_eq!(method, "is_some");
            assert!(args.is_empty());
            assert!(matches!(*object, HirExpr::Var(_)));
        }
        _ => panic!("Expected MethodCall, got {:?}", result),
    }
}

#[test]
fn test_is_with_non_none_fails() {
    // 'is' operator with non-None values should fail (not supported)
    let expr = parse_expr("x is y");
    let result = ExprConverter::convert(expr);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("is"));
}

#[test]
fn test_complex_expr_is_none() {
    // Test 'is None' with more complex expression
    let expr = parse_expr("func() is None");
    let result = ExprConverter::convert(expr).unwrap();

    match result {
        HirExpr::MethodCall { method, .. } => {
            assert_eq!(method, "is_none");
        }
        _ => panic!("Expected MethodCall"),
    }
}
