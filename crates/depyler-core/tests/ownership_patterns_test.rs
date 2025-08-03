//! Tests for improved ownership pattern inference

use depyler_annotations::TranspilationAnnotations;
use depyler_core::borrowing_context::{BorrowingContext, BorrowingStrategy};
use depyler_core::hir::{
    BinOp, FunctionProperties, HirExpr, HirFunction, HirStmt, Literal, Type as PythonType,
};
use depyler_core::type_mapper::TypeMapper;
use smallvec::smallvec;

#[test]
fn test_read_only_string_borrowed() {
    let mut ctx = BorrowingContext::new(Some(PythonType::Int));
    let type_mapper = TypeMapper::new();

    // Function that only reads a string parameter
    let func = HirFunction {
        name: "get_length".to_string(),
        params: smallvec![("text".to_string(), PythonType::String)],
        ret_type: PythonType::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("text".to_string())],
        }))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let result = ctx.analyze_function(&func, &type_mapper);
    let strategy = result.param_strategies.get("text").unwrap();

    // Should borrow immutably
    assert_eq!(
        *strategy,
        BorrowingStrategy::BorrowImmutable { lifetime: None }
    );
}

#[test]
fn test_list_append_takes_ownership() {
    let mut ctx = BorrowingContext::new(None);
    let type_mapper = TypeMapper::new();

    // Function that appends to a list
    let func = HirFunction {
        name: "add_item".to_string(),
        params: smallvec![(
            "items".to_string(),
            PythonType::List(Box::new(PythonType::Int))
        )],
        ret_type: PythonType::None,
        body: vec![HirStmt::Expr(HirExpr::Call {
            func: "append".to_string(),
            args: vec![
                HirExpr::Var("items".to_string()),
                HirExpr::Literal(Literal::Int(42)),
            ],
        })],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let result = ctx.analyze_function(&func, &type_mapper);
    let strategy = result.param_strategies.get("items").unwrap();

    // Should take ownership (append modifies the list)
    assert_eq!(*strategy, BorrowingStrategy::TakeOwnership);
}

#[test]
fn test_escaping_parameter_takes_ownership() {
    let mut ctx = BorrowingContext::new(Some(PythonType::String));
    let type_mapper = TypeMapper::new();

    // Function that returns its parameter
    let func = HirFunction {
        name: "identity".to_string(),
        params: smallvec![("value".to_string(), PythonType::String)],
        ret_type: PythonType::String,
        body: vec![HirStmt::Return(Some(HirExpr::Var("value".to_string())))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let result = ctx.analyze_function(&func, &type_mapper);
    let strategy = result.param_strategies.get("value").unwrap();

    // When a string parameter escapes through return, we use Cow for flexibility
    // This allows both borrowed and owned strings to be passed in
    assert_eq!(
        *strategy,
        BorrowingStrategy::UseCow {
            lifetime: "'static".to_string()
        }
    );
}

#[test]
fn test_string_concatenation_uses_cow() {
    let mut ctx = BorrowingContext::new(Some(PythonType::String));
    let type_mapper = TypeMapper::new();

    // Function that returns a modified string
    let func = HirFunction {
        name: "add_suffix".to_string(),
        params: smallvec![("prefix".to_string(), PythonType::String)],
        ret_type: PythonType::String,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("prefix".to_string())),
            right: Box::new(HirExpr::Literal(Literal::String("_suffix".to_string()))),
        }))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let result = ctx.analyze_function(&func, &type_mapper);
    let strategy = result.param_strategies.get("prefix").unwrap();

    // Should use Cow for flexibility when string escapes but isn't mutated in place
    assert_eq!(
        *strategy,
        BorrowingStrategy::UseCow {
            lifetime: "'static".to_string()
        }
    );
}

#[test]
fn test_copy_type_takes_value() {
    let mut ctx = BorrowingContext::new(Some(PythonType::Int));
    let type_mapper = TypeMapper::new();

    // Function with integer parameter
    let func = HirFunction {
        name: "double".to_string(),
        params: smallvec![("n".to_string(), PythonType::Int)],
        ret_type: PythonType::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Var("n".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        }))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let result = ctx.analyze_function(&func, &type_mapper);
    let strategy = result.param_strategies.get("n").unwrap();

    // Should take ownership (Copy types are cheap to copy)
    assert_eq!(*strategy, BorrowingStrategy::TakeOwnership);

    // Should have insight about Copy trait
    assert!(result.insights.iter().any(|insight| {
        matches!(
            insight,
            depyler_core::borrowing_context::BorrowingInsight::SuggestCopyDerive(_)
        )
    }));
}

#[test]
fn test_unnecessary_move_detection() {
    let mut ctx = BorrowingContext::new(None);
    let type_mapper = TypeMapper::new();

    // Function that passes string to a function that could borrow
    let func = HirFunction {
        name: "print_message".to_string(),
        params: smallvec![("msg".to_string(), PythonType::String)],
        ret_type: PythonType::None,
        body: vec![HirStmt::Expr(HirExpr::Call {
            func: "unknown_function".to_string(), // Conservative: assumes ownership
            args: vec![HirExpr::Var("msg".to_string())],
        })],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let result = ctx.analyze_function(&func, &type_mapper);

    // Should detect unnecessary move
    assert!(result.insights.iter().any(|insight| {
        matches!(
            insight,
            depyler_core::borrowing_context::BorrowingInsight::UnnecessaryMove(_)
        )
    }));
}

#[test]
fn test_loop_usage_affects_borrowing() {
    let mut ctx = BorrowingContext::new(None);
    let type_mapper = TypeMapper::new();

    // Function that uses parameter in a loop
    let func = HirFunction {
        name: "count_occurrences".to_string(),
        params: smallvec![
            ("haystack".to_string(), PythonType::String),
            ("needle".to_string(), PythonType::String)
        ],
        ret_type: PythonType::Int,
        body: vec![
            HirStmt::Assign {
                target: depyler_core::hir::AssignTarget::Symbol("count".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
            },
            HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::If {
                    condition: HirExpr::Call {
                        func: "contains".to_string(),
                        args: vec![
                            HirExpr::Var("haystack".to_string()),
                            HirExpr::Var("needle".to_string()),
                        ],
                    },
                    then_body: vec![HirStmt::Assign {
                        target: depyler_core::hir::AssignTarget::Symbol("count".to_string()),
                        value: HirExpr::Binary {
                            op: BinOp::Add,
                            left: Box::new(HirExpr::Var("count".to_string())),
                            right: Box::new(HirExpr::Literal(Literal::Int(1))),
                        },
                    }],
                    else_body: None,
                }],
            },
            HirStmt::Return(Some(HirExpr::Var("count".to_string()))),
        ],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let result = ctx.analyze_function(&func, &type_mapper);

    // Both parameters should be borrowed since they're only read in the loop
    let haystack_strategy = result.param_strategies.get("haystack").unwrap();
    let needle_strategy = result.param_strategies.get("needle").unwrap();

    assert_eq!(
        *haystack_strategy,
        BorrowingStrategy::BorrowImmutable { lifetime: None }
    );
    assert_eq!(
        *needle_strategy,
        BorrowingStrategy::BorrowImmutable { lifetime: None }
    );
}
