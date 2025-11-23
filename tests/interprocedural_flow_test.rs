//! Integration tests for interprocedural flow analysis
//!
//! These tests validate the complete flow from Python code to mutability inference

use depyler_core::hir::{HirExpr, HirFunction, HirModule, HirParam, HirStmt, Type as PythonType};
use depyler_core::interprocedural::{
    FlowAnalysisResults, IntraproceduralAnalyzer, LocalMutability, MutabilityKind,
    PreciseInterproceduralAnalyzer,
};

/// Helper to create a simple function for testing
fn make_function(name: &str, params: Vec<&str>, body: Vec<HirStmt>) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: params
            .iter()
            .map(|p| HirParam {
                name: p.to_string(),
                type_annotation: Some(PythonType::Custom("State".to_string())),
                default: None,
            })
            .collect(),
        body,
        return_type: Some(PythonType::None),
        decorators: Vec::new(),
        is_async: false,
        docstring: None,
    }
}

#[test]
fn test_intraprocedural_direct_read() {
    // def get_value(state: State) -> int:
    //     return state.value
    let func = make_function(
        "get_value",
        vec!["state"],
        vec![HirStmt::Return {
            value: Some(HirExpr::Attribute {
                value: Box::new(HirExpr::Name("state".to_string())),
                attr: "value".to_string(),
                ctx: (),
            }),
            type_comment: None,
        }],
    );

    let analyzer = IntraproceduralAnalyzer::new(&func);
    let summary = analyzer.analyze();

    assert_eq!(summary.parameters.len(), 1);
    let param = &summary.parameters[0];
    assert_eq!(param.name, "state");
    assert!(!param.has_direct_mutations());
    assert!(param.has_reads());
    assert_eq!(param.minimal_mutability(), LocalMutability::CanBeShared);
}

#[test]
fn test_intraprocedural_direct_mutation() {
    // def set_value(state: State, val: int):
    //     state.value = val
    let func = make_function(
        "set_value",
        vec!["state"],
        vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Attribute {
                value: Box::new(HirExpr::Name("state".to_string())),
                attr: "value".to_string(),
                ctx: (),
            },
            value: HirExpr::Name("val".to_string()),
            type_comment: None,
        }],
    );

    let analyzer = IntraproceduralAnalyzer::new(&func);
    let summary = analyzer.analyze();

    assert_eq!(summary.parameters.len(), 1);
    let param = &summary.parameters[0];
    assert_eq!(param.name, "state");
    assert!(param.has_direct_mutations());
    assert_eq!(param.minimal_mutability(), LocalMutability::NeedsMut);
}

#[test]
fn test_intraprocedural_method_call() {
    // def add_item(state: State, item: Item):
    //     state.items.append(item)
    let func = make_function(
        "add_item",
        vec!["state"],
        vec![HirStmt::Expr {
            value: HirExpr::MethodCall {
                receiver: Box::new(HirExpr::Attribute {
                    value: Box::new(HirExpr::Name("state".to_string())),
                    attr: "items".to_string(),
                    ctx: (),
                }),
                method: "append".to_string(),
                args: vec![HirExpr::Name("item".to_string())],
                keywords: Vec::new(),
            },
        }],
    );

    let analyzer = IntraproceduralAnalyzer::new(&func);
    let summary = analyzer.analyze();

    let param = &summary.parameters[0];
    assert!(param.has_direct_mutations()); // append is a mutating method
}

#[test]
fn test_flow_simple_read_write_chain() {
    // def read_only(state: State) -> int:
    //     return state.value
    //
    // def write_only(state: State):
    //     state.value = 42
    //
    // def mixed(state: State):
    //     x = read_only(state)
    //     write_only(state)

    let read_func = make_function(
        "read_only",
        vec!["state"],
        vec![HirStmt::Return {
            value: Some(HirExpr::Attribute {
                value: Box::new(HirExpr::Name("state".to_string())),
                attr: "value".to_string(),
                ctx: (),
            }),
            type_comment: None,
        }],
    );

    let write_func = make_function(
        "write_only",
        vec!["state"],
        vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Attribute {
                value: Box::new(HirExpr::Name("state".to_string())),
                attr: "value".to_string(),
                ctx: (),
            },
            value: HirExpr::Literal(crate::hir::Literal::Int(42)),
            type_comment: None,
        }],
    );

    let mixed_func = make_function(
        "mixed",
        vec!["state"],
        vec![
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Name("x".to_string()),
                value: HirExpr::Call {
                    func: Box::new(HirExpr::Name("read_only".to_string())),
                    args: vec![HirExpr::Name("state".to_string())],
                    keywords: Vec::new(),
                },
                type_comment: None,
            },
            HirStmt::Expr {
                value: HirExpr::Call {
                    func: Box::new(HirExpr::Name("write_only".to_string())),
                    args: vec![HirExpr::Name("state".to_string())],
                    keywords: Vec::new(),
                },
            },
        ],
    );

    let module = HirModule {
        functions: vec![read_func, write_func, mixed_func],
        classes: Vec::new(),
        imports: Vec::new(),
        docstring: None,
    };

    let analyzer = PreciseInterproceduralAnalyzer::new(&module);
    let results = analyzer.analyze();

    // read_only should have & (SharedBorrow)
    let read_sig = results.get_signature("read_only").unwrap();
    assert_eq!(
        read_sig.parameters[0].mutability,
        MutabilityKind::SharedBorrow
    );

    // write_only should have &mut (MutableBorrow)
    let write_sig = results.get_signature("write_only").unwrap();
    assert_eq!(
        write_sig.parameters[0].mutability,
        MutabilityKind::MutableBorrow
    );

    // mixed should have &mut (because it calls write_only)
    let mixed_sig = results.get_signature("mixed").unwrap();
    assert_eq!(
        mixed_sig.parameters[0].mutability,
        MutabilityKind::MutableBorrow
    );
}

#[test]
fn test_flow_unused_parameter() {
    // def unused(state: State):
    //     pass
    let func = make_function("unused", vec!["state"], vec![]);

    let module = HirModule {
        functions: vec![func],
        classes: Vec::new(),
        imports: Vec::new(),
        docstring: None,
    };

    let analyzer = PreciseInterproceduralAnalyzer::new(&module);
    let results = analyzer.analyze();

    let sig = results.get_signature("unused").unwrap();
    // Unused parameter should be Owned (can take ownership)
    assert_eq!(sig.parameters[0].mutability, MutabilityKind::Owned);
}

#[test]
fn test_flow_analysis_convergence() {
    // Simple recursive function
    // def countdown(state: State, n: int):
    //     if n > 0:
    //         state.count = n
    //         countdown(state, n - 1)

    let func = make_function(
        "countdown",
        vec!["state"],
        vec![HirStmt::If {
            test: HirExpr::Compare {
                left: Box::new(HirExpr::Name("n".to_string())),
                ops: vec![crate::hir::CmpOp::Gt],
                comparators: vec![HirExpr::Literal(crate::hir::Literal::Int(0))],
            },
            body: vec![
                HirStmt::Assign {
                    target: crate::hir::AssignTarget::Attribute {
                        value: Box::new(HirExpr::Name("state".to_string())),
                        attr: "count".to_string(),
                        ctx: (),
                    },
                    value: HirExpr::Name("n".to_string()),
                    type_comment: None,
                },
                HirStmt::Expr {
                    value: HirExpr::Call {
                        func: Box::new(HirExpr::Name("countdown".to_string())),
                        args: vec![
                            HirExpr::Name("state".to_string()),
                            HirExpr::BinOp {
                                left: Box::new(HirExpr::Name("n".to_string())),
                                op: crate::hir::BinOp::Sub,
                                right: Box::new(HirExpr::Literal(crate::hir::Literal::Int(1))),
                            },
                        ],
                        keywords: Vec::new(),
                    },
                },
            ],
            orelse: Vec::new(),
        }],
    );

    let module = HirModule {
        functions: vec![func],
        classes: Vec::new(),
        imports: Vec::new(),
        docstring: None,
    };

    let analyzer = PreciseInterproceduralAnalyzer::new(&module);
    let results = analyzer.analyze();

    // Should converge
    assert!(results.converged());

    // Should infer &mut (because of direct mutation)
    let sig = results.get_signature("countdown").unwrap();
    assert_eq!(sig.parameters[0].mutability, MutabilityKind::MutableBorrow);
}

#[test]
fn test_is_param_mutable_helper() {
    let read_func = make_function(
        "reader",
        vec!["x"],
        vec![HirStmt::Return {
            value: Some(HirExpr::Name("x".to_string())),
            type_comment: None,
        }],
    );

    let module = HirModule {
        functions: vec![read_func],
        classes: Vec::new(),
        imports: Vec::new(),
        docstring: None,
    };

    let analyzer = PreciseInterproceduralAnalyzer::new(&module);
    let results = analyzer.analyze();

    // reader's x should not be mutable
    assert!(!results.is_param_mutable("reader", "x"));
}
