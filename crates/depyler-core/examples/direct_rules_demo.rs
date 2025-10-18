//! Example demonstrating direct rules conversion from HIR to Rust

use depyler_core::direct_rules::apply_rules;
use depyler_core::hir::*;
use depyler_core::type_mapper::TypeMapper;
use smallvec::smallvec;

fn main() {
    // Create a sample Python-like module in HIR
    let module = create_sample_module();

    // Create type mapper
    let type_mapper = TypeMapper::new();

    // Apply direct rules to convert to Rust AST
    match apply_rules(&module, &type_mapper) {
        Ok(rust_file) => {
            // Convert to string using prettyplease or similar
            println!("Generated Rust code:");
            println!("{:#?}", rust_file);
        }
        Err(e) => {
            eprintln!("Error converting module: {}", e);
        }
    }
}

fn create_sample_module() -> HirModule {
    HirModule {
        imports: vec![Import {
            module: "std".to_string(),
            items: vec![ImportItem::Named("sqrt".to_string())],
        }],
        functions: vec![
            create_add_function(),
            create_factorial_function(),
            create_fibonacci_function(),
        ],
        classes: vec![create_calculator_class()],
        type_aliases: vec![TypeAlias {
            name: "Number".to_string(),
            target_type: Type::Union(vec![Type::Int, Type::Float]),
            is_newtype: false,
        }],
        protocols: vec![],
    }
}

fn create_add_function() -> HirFunction {
    HirFunction {
        name: "add".to_string(),
        params: smallvec![
            HirParam::new("a".to_string(), Type::Int),
            HirParam::new("b".to_string(), Type::Int)
        ],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Var("b".to_string())),
        }))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: Some("Add two numbers together".to_string()),
    }
}

fn create_factorial_function() -> HirFunction {
    HirFunction {
        name: "factorial".to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::LtEq,
                left: Box::new(HirExpr::Var("n".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(HirExpr::Var("n".to_string())),
                right: Box::new(HirExpr::Call {
                    func: "factorial".to_string(),
                    args: vec![HirExpr::Binary {
                        op: BinOp::Sub,
                        left: Box::new(HirExpr::Var("n".to_string())),
                        right: Box::new(HirExpr::Literal(Literal::Int(1))),
                    }],
                }),
            }))]),
        }],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: Some("Calculate factorial recursively".to_string()),
    }
}

fn create_fibonacci_function() -> HirFunction {
    HirFunction {
        name: "fibonacci".to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![
            // a = 0
            HirStmt::Assign {
                target: AssignTarget::Symbol("a".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            // b = 1
            HirStmt::Assign {
                target: AssignTarget::Symbol("b".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            // for _ in range(n):
            HirStmt::For {
                target: AssignTarget::Symbol("_".to_string()),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Var("n".to_string())],
                },
                body: vec![
                    // temp = b
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("temp".to_string()),
                        value: HirExpr::Var("b".to_string()),
                        type_annotation: None,
                    },
                    // b = a + b
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("b".to_string()),
                        value: HirExpr::Binary {
                            op: BinOp::Add,
                            left: Box::new(HirExpr::Var("a".to_string())),
                            right: Box::new(HirExpr::Var("b".to_string())),
                        },
                        type_annotation: None,
                    },
                    // a = temp
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("a".to_string()),
                        value: HirExpr::Var("temp".to_string()),
                        type_annotation: None,
                    },
                ],
            },
            // return a
            HirStmt::Return(Some(HirExpr::Var("a".to_string()))),
        ],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: Some("Calculate nth Fibonacci number".to_string()),
    }
}

fn create_calculator_class() -> HirClass {
    HirClass {
        name: "Calculator".to_string(),
        base_classes: vec![],
        fields: vec![
            // Field: result
            HirField {
                name: "result".to_string(),
                field_type: Type::Float,
                default_value: Some(HirExpr::Literal(Literal::Float(0.0))),
                is_class_var: false,
            },
        ],
        methods: vec![
            // Method: __init__
            HirMethod {
                name: "__init__".to_string(),
                params: smallvec![HirParam::new("self".to_string(), Type::Unknown)],
                ret_type: Type::None,
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Attribute {
                        value: Box::new(HirExpr::Var("self".to_string())),
                        attr: "result".to_string(),
                    },
                    value: HirExpr::Literal(Literal::Float(0.0)),
                    type_annotation: None,
                }],
                is_static: false,
                is_classmethod: false,
                is_property: false,
                is_async: false,
                docstring: None,
            },
            // Method: add
            HirMethod {
                name: "add".to_string(),
                params: smallvec![
                    HirParam::new("self".to_string(), Type::Unknown),
                    HirParam::new("value".to_string(), Type::Float)
                ],
                ret_type: Type::None,
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Attribute {
                        value: Box::new(HirExpr::Var("self".to_string())),
                        attr: "result".to_string(),
                    },
                    value: HirExpr::Binary {
                        op: BinOp::Add,
                        left: Box::new(HirExpr::Attribute {
                            value: Box::new(HirExpr::Var("self".to_string())),
                            attr: "result".to_string(),
                        }),
                        right: Box::new(HirExpr::Var("value".to_string())),
                    },
                    type_annotation: None,
                }],
                is_static: false,
                is_classmethod: false,
                is_property: false,
                is_async: false,
                docstring: Some("Add value to result".to_string()),
            },
            // Method: get_result (property)
            HirMethod {
                name: "get_result".to_string(),
                params: smallvec![HirParam::new("self".to_string(), Type::Unknown)],
                ret_type: Type::Float,
                body: vec![HirStmt::Return(Some(HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("self".to_string())),
                    attr: "result".to_string(),
                }))],
                is_static: false,
                is_classmethod: false,
                is_property: true,
                is_async: false,
                docstring: None,
            },
        ],
        is_dataclass: false,
        docstring: Some("A simple calculator class".to_string()),
    }
}
