/// DEPYLER-0260: Generator Compilation Tests
///
/// These tests verify that transpiled generator code actually compiles
/// with `rustc --crate-type lib --deny warnings`.
///
/// BUG: Generated generators use undefined `DynamicType` which causes
/// compilation failures.

use depyler_core::hir::*;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use std::process::Command;
use depyler_annotations::TranspilationAnnotations;
use smallvec::smallvec;

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0260_simple_generator_compiles() {
    // RED Phase Test: This test MUST FAIL initially
    // Create a simple generator function
    let module = HirModule {
        functions: vec![HirFunction {
            name: "count_to_n".to_string(),
            params: smallvec![HirParam::new("n".to_string(), Type::Int)],
            ret_type: Type::Generator(Box::new(Type::Int)),
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("i".to_string()),
                    value: HirExpr::Literal(Literal::Int(0)),
                    type_annotation: None,
                },
                HirStmt::While {
                    condition: HirExpr::Binary {
                        op: BinOp::Lt,
                        left: Box::new(HirExpr::Var("i".to_string())),
                        right: Box::new(HirExpr::Var("n".to_string())),
                    },
                    body: vec![
                        HirStmt::Yield(HirExpr::Var("i".to_string())),
                        HirStmt::Assign {
                            target: AssignTarget::Symbol("i".to_string()),
                            value: HirExpr::Binary {
                                op: BinOp::Add,
                                left: Box::new(HirExpr::Var("i".to_string())),
                                right: Box::new(HirExpr::Literal(Literal::Int(1))),
                            },
                            type_annotation: None,
                        },
                    ],
                },
            ],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: Some("Count from 0 to n".to_string()),
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
        constants: vec![],
    };

    // Generate Rust code
    let type_mapper = TypeMapper::default();
    let (rust_code, _dependencies) = generate_rust_file(&module, &type_mapper)
        .expect("DEPYLER-0260: Code generation should not fail");

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0260_generator.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0260: Failed to write temp file");

    // Attempt to compile with rustc
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--deny")
        .arg("warnings")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0260_generator.rlib")
        .output()
        .expect("DEPYLER-0260: Failed to run rustc");

    // ASSERT: Generated code must compile
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        eprintln!("=== DEPYLER-0260 COMPILATION FAILURE ===");
        eprintln!("Generated Rust code:");
        eprintln!("{}", rust_code);
        eprintln!("\n=== rustc stderr ===");
        eprintln!("{}", stderr);
        eprintln!("\n=== rustc stdout ===");
        eprintln!("{}", stdout);
        panic!(
            "DEPYLER-0260: Generated generator code failed to compile!\n\
             Expected: rustc success\n\
             Got: rustc exit code {}\n\
             Error: {}",
            output.status.code().unwrap_or(-1),
            stderr
        );
    }

    // Cleanup
    let _ = std::fs::remove_file(temp_file);
    let _ = std::fs::remove_file("/tmp/test_depyler_0260_generator.rlib");
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0260_generator_no_dynamictype() {
    // RED Phase Test: Verify generated code doesn't use DynamicType
    let module = HirModule {
        functions: vec![HirFunction {
            name: "simple_gen".to_string(),
            params: smallvec![],
            ret_type: Type::Generator(Box::new(Type::Int)),
            body: vec![
                HirStmt::Yield(HirExpr::Literal(Literal::Int(42))),
            ],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
        constants: vec![],
    };

    let type_mapper = TypeMapper::default();
    let (rust_code, _dependencies) = generate_rust_file(&module, &type_mapper)
        .expect("Code generation should not fail");

    // ASSERT: Generated code must NOT contain "DynamicType"
    assert!(
        !rust_code.contains("DynamicType"),
        "DEPYLER-0260: Generated code must not use undefined DynamicType!\n\
         Generated code:\n{}",
        rust_code
    );

    // ASSERT: Generated code must use concrete type (i32)
    assert!(
        rust_code.contains("Iterator<Item = i32>"),
        "DEPYLER-0260: Generator must use concrete Item type (i32), not DynamicType!\n\
         Generated code:\n{}",
        rust_code
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0260_fibonacci_generator_compiles() {
    // RED Phase Test: Fibonacci generator must compile
    let module = HirModule {
        functions: vec![HirFunction {
            name: "fibonacci".to_string(),
            params: smallvec![HirParam::new("n".to_string(), Type::Int)],
            ret_type: Type::Generator(Box::new(Type::Int)),
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Tuple(vec![
                        AssignTarget::Symbol("a".to_string()),
                        AssignTarget::Symbol("b".to_string()),
                    ]),
                    value: HirExpr::Tuple(vec![
                        HirExpr::Literal(Literal::Int(0)),
                        HirExpr::Literal(Literal::Int(1)),
                    ]),
                    type_annotation: None,
                },
                HirStmt::Assign {
                    target: AssignTarget::Symbol("count".to_string()),
                    value: HirExpr::Literal(Literal::Int(0)),
                    type_annotation: None,
                },
                HirStmt::While {
                    condition: HirExpr::Binary {
                        op: BinOp::Lt,
                        left: Box::new(HirExpr::Var("count".to_string())),
                        right: Box::new(HirExpr::Var("n".to_string())),
                    },
                    body: vec![
                        HirStmt::Yield(HirExpr::Var("a".to_string())),
                        HirStmt::Assign {
                            target: AssignTarget::Tuple(vec![
                                AssignTarget::Symbol("a".to_string()),
                                AssignTarget::Symbol("b".to_string()),
                            ]),
                            value: HirExpr::Tuple(vec![
                                HirExpr::Var("b".to_string()),
                                HirExpr::Binary {
                                    op: BinOp::Add,
                                    left: Box::new(HirExpr::Var("a".to_string())),
                                    right: Box::new(HirExpr::Var("b".to_string())),
                                },
                            ]),
                            type_annotation: None,
                        },
                        HirStmt::Assign {
                            target: AssignTarget::Symbol("count".to_string()),
                            value: HirExpr::Binary {
                                op: BinOp::Add,
                                left: Box::new(HirExpr::Var("count".to_string())),
                                right: Box::new(HirExpr::Literal(Literal::Int(1))),
                            },
                            type_annotation: None,
                        },
                    ],
                },
            ],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: Some("Generate Fibonacci sequence".to_string()),
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
        constants: vec![],
    };

    let type_mapper = TypeMapper::default();
    let (rust_code, _dependencies) = generate_rust_file(&module, &type_mapper)
        .expect("Code generation should not fail");

    let temp_file = "/tmp/test_depyler_0260_fibonacci.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("Failed to write temp file");

    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--deny")
        .arg("warnings")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0260_fibonacci.rlib")
        .output()
        .expect("Failed to run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("=== Generated Code ===\n{}", rust_code);
        eprintln!("\n=== Compilation Error ===\n{}", stderr);
        panic!("DEPYLER-0260: Fibonacci generator failed to compile!");
    }

    // Cleanup
    let _ = std::fs::remove_file(temp_file);
    let _ = std::fs::remove_file("/tmp/test_depyler_0260_fibonacci.rlib");
}
