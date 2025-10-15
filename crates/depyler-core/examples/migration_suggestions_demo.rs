//! Example demonstrating migration suggestions from Python to Rust idioms

use depyler_core::hir::*;
use depyler_core::migration_suggestions::{MigrationAnalyzer, MigrationConfig};
use smallvec::smallvec;

fn main() {
    // Create analyzer with custom config
    let config = MigrationConfig {
        suggest_iterators: true,
        suggest_error_handling: true,
        suggest_ownership: true,
        suggest_performance: true,
        verbosity: 2, // Show detailed suggestions
    };

    let mut analyzer = MigrationAnalyzer::new(config);

    // Create a sample program with various Python patterns
    let program = create_sample_program();

    // Analyze and get suggestions
    let suggestions = analyzer.analyze_program(&program);

    // Format and display suggestions
    let output = analyzer.format_suggestions(&suggestions);
    println!("{}", output);
}

fn create_sample_program() -> HirProgram {
    HirProgram {
        imports: vec![],
        functions: vec![
            create_accumulator_function(),
            create_type_check_function(),
            create_while_true_function(),
            create_mutable_param_function(),
        ],
        classes: vec![],
    }
}

// Function demonstrating accumulator pattern
fn create_accumulator_function() -> HirFunction {
    HirFunction {
        name: "filter_even_numbers".to_string(),
        params: smallvec![HirParam::new(
            "numbers".to_string(),
            Type::List(Box::new(Type::Int))
        )],
        ret_type: Type::List(Box::new(Type::Int)),
        body: vec![
            // result = []
            HirStmt::Assign {
                target: AssignTarget::Symbol("result".to_string()),
                value: HirExpr::List(vec![]),
                type_annotation: None,
            },
            // for num in numbers:
            HirStmt::For {
                target: "num".to_string(),
                iter: HirExpr::Var("numbers".to_string()),
                body: vec![
                    // if num % 2 == 0:
                    HirStmt::If {
                        condition: HirExpr::Binary {
                            op: BinOp::Eq,
                            left: Box::new(HirExpr::Binary {
                                op: BinOp::Mod,
                                left: Box::new(HirExpr::Var("num".to_string())),
                                right: Box::new(HirExpr::Literal(Literal::Int(2))),
                            }),
                            right: Box::new(HirExpr::Literal(Literal::Int(0))),
                        },
                        then_body: vec![
                            // result.append(num)
                            HirStmt::Expr(HirExpr::MethodCall {
                                object: Box::new(HirExpr::Var("result".to_string())),
                                method: "append".to_string(),
                                args: vec![HirExpr::Var("num".to_string())],
                            }),
                        ],
                        else_body: None,
                    },
                ],
            },
            // return result
            HirStmt::Return(Some(HirExpr::Var("result".to_string()))),
        ],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: Some("Filter even numbers from a list".to_string()),
    }
}

// Function demonstrating type checking pattern
fn create_type_check_function() -> HirFunction {
    HirFunction {
        name: "process_value".to_string(),
        params: smallvec![HirParam::new("value".to_string(), Type::Unknown)],
        ret_type: Type::Unknown,
        body: vec![
            // if isinstance(value, str):
            HirStmt::If {
                condition: HirExpr::Call {
                    func: "isinstance".to_string(),
                    args: vec![
                        HirExpr::Var("value".to_string()),
                        HirExpr::Var("str".to_string()),
                    ],
                },
                then_body: vec![
                    // return value.upper()
                    HirStmt::Return(Some(HirExpr::MethodCall {
                        object: Box::new(HirExpr::Var("value".to_string())),
                        method: "upper".to_string(),
                        args: vec![],
                    })),
                ],
                else_body: Some(vec![
                    // elif isinstance(value, int):
                    HirStmt::If {
                        condition: HirExpr::Call {
                            func: "isinstance".to_string(),
                            args: vec![
                                HirExpr::Var("value".to_string()),
                                HirExpr::Var("int".to_string()),
                            ],
                        },
                        then_body: vec![
                            // return value * 2
                            HirStmt::Return(Some(HirExpr::Binary {
                                op: BinOp::Mul,
                                left: Box::new(HirExpr::Var("value".to_string())),
                                right: Box::new(HirExpr::Literal(Literal::Int(2))),
                            })),
                        ],
                        else_body: None,
                    },
                ]),
            },
            // return None
            HirStmt::Return(Some(HirExpr::Literal(Literal::None))),
        ],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: Some("Process value based on its type".to_string()),
    }
}

// Function demonstrating while True pattern
fn create_while_true_function() -> HirFunction {
    HirFunction {
        name: "server_loop".to_string(),
        params: smallvec![],
        ret_type: Type::Unknown,
        body: vec![
            // while True:
            HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![
                    // if should_stop():
                    HirStmt::If {
                        condition: HirExpr::Call {
                            func: "should_stop".to_string(),
                            args: vec![],
                        },
                        then_body: vec![
                            // break
                            HirStmt::Break { label: None },
                        ],
                        else_body: None,
                    },
                    // process_request()
                    HirStmt::Expr(HirExpr::Call {
                        func: "process_request".to_string(),
                        args: vec![],
                    }),
                ],
            },
        ],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: Some("Main server loop".to_string()),
    }
}

// Function demonstrating mutable parameter pattern
fn create_mutable_param_function() -> HirFunction {
    HirFunction {
        name: "add_to_list".to_string(),
        params: smallvec![
            HirParam::new("items".to_string(), Type::List(Box::new(Type::Unknown))),
            HirParam::new("new_item".to_string(), Type::Unknown)
        ],
        ret_type: Type::Unknown,
        body: vec![
            // items.append(new_item)
            HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("items".to_string())),
                method: "append".to_string(),
                args: vec![HirExpr::Var("new_item".to_string())],
            }),
            // return items
            HirStmt::Return(Some(HirExpr::Var("items".to_string()))),
        ],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: Some("Add item to list (modifies parameter)".to_string()),
    }
}
