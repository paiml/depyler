//! Performance benchmarks for Ruchy transpilation

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use depyler_ruchy::{RuchyBackend, RuchyConfig};
use depyler_core::{Hir, HirExpr, HirLiteral, HirBinaryOp, HirParam, TranspilationBackend};

fn create_simple_function() -> Hir {
    Hir {
        root: HirExpr::Function {
            name: "fibonacci".to_string(),
            params: vec![
                HirParam {
                    name: "n".to_string(),
                    typ: Some(depyler_core::HirType::Int),
                    default: None,
                },
            ],
            body: Box::new(HirExpr::If {
                condition: Box::new(HirExpr::Binary {
                    left: Box::new(HirExpr::Identifier("n".to_string())),
                    op: HirBinaryOp::LessEqual,
                    right: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
                }),
                then_branch: Box::new(HirExpr::Identifier("n".to_string())),
                else_branch: Some(Box::new(HirExpr::Binary {
                    left: Box::new(HirExpr::Call {
                        func: Box::new(HirExpr::Identifier("fibonacci".to_string())),
                        args: vec![HirExpr::Binary {
                            left: Box::new(HirExpr::Identifier("n".to_string())),
                            op: HirBinaryOp::Subtract,
                            right: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
                        }],
                    }),
                    op: HirBinaryOp::Add,
                    right: Box::new(HirExpr::Call {
                        func: Box::new(HirExpr::Identifier("fibonacci".to_string())),
                        args: vec![HirExpr::Binary {
                            left: Box::new(HirExpr::Identifier("n".to_string())),
                            op: HirBinaryOp::Subtract,
                            right: Box::new(HirExpr::Literal(HirLiteral::Integer(2))),
                        }],
                    }),
                })),
            }),
            is_async: false,
            return_type: Some(depyler_core::HirType::Int),
        },
        metadata: Default::default(),
    }
}

fn create_large_list(size: usize) -> Hir {
    let elements: Vec<HirExpr> = (0..size)
        .map(|i| HirExpr::Literal(HirLiteral::Integer(i as i64)))
        .collect();
    
    Hir {
        root: HirExpr::List(elements),
        metadata: Default::default(),
    }
}

fn create_nested_expression(depth: usize) -> HirExpr {
    if depth == 0 {
        HirExpr::Literal(HirLiteral::Integer(1))
    } else {
        HirExpr::Binary {
            left: Box::new(create_nested_expression(depth - 1)),
            op: HirBinaryOp::Add,
            right: Box::new(create_nested_expression(depth - 1)),
        }
    }
}

fn create_pipeline_expression() -> Hir {
    // Simulates a list comprehension that would be transformed to pipeline
    Hir {
        root: HirExpr::Call {
            func: Box::new(HirExpr::Identifier("list_comp".to_string())),
            args: vec![
                HirExpr::Binary {
                    left: Box::new(HirExpr::Identifier("x".to_string())),
                    op: HirBinaryOp::Multiply,
                    right: Box::new(HirExpr::Literal(HirLiteral::Integer(2))),
                },
                HirExpr::Identifier("x".to_string()),
                create_large_list(100).root,
                HirExpr::Binary {
                    left: Box::new(HirExpr::Identifier("x".to_string())),
                    op: HirBinaryOp::Greater,
                    right: Box::new(HirExpr::Literal(HirLiteral::Integer(50))),
                },
            ],
        },
        metadata: Default::default(),
    }
}

fn bench_simple_transpilation(c: &mut Criterion) {
    let backend = RuchyBackend::new();
    let hir = create_simple_function();
    
    c.bench_function("simple_function_transpilation", |b| {
        b.iter(|| {
            backend.transpile(black_box(&hir))
        });
    });
}

fn bench_optimization_levels(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_levels");
    let hir = create_simple_function();
    
    for level in 0..=3 {
        let config = RuchyConfig {
            optimization_level: level,
            ..Default::default()
        };
        let backend = RuchyBackend::with_config(config);
        
        group.bench_with_input(
            BenchmarkId::from_parameter(level),
            &hir,
            |b, hir| {
                b.iter(|| backend.transpile(black_box(hir)));
            },
        );
    }
    
    group.finish();
}

fn bench_list_size_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("list_size_scaling");
    let backend = RuchyBackend::new();
    
    for size in [10, 100, 1000, 10000] {
        let hir = create_large_list(size);
        
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &hir,
            |b, hir| {
                b.iter(|| backend.transpile(black_box(hir)));
            },
        );
    }
    
    group.finish();
}

fn bench_expression_depth(c: &mut Criterion) {
    let mut group = c.benchmark_group("expression_depth");
    let backend = RuchyBackend::new();
    
    for depth in [1, 5, 10, 15] {
        let hir = Hir {
            root: create_nested_expression(depth),
            metadata: Default::default(),
        };
        
        group.bench_with_input(
            BenchmarkId::from_parameter(depth),
            &hir,
            |b, hir| {
                b.iter(|| backend.transpile(black_box(hir)));
            },
        );
    }
    
    group.finish();
}

fn bench_pipeline_transformation(c: &mut Criterion) {
    let config_no_pipeline = RuchyConfig {
        use_pipelines: false,
        ..Default::default()
    };
    let config_with_pipeline = RuchyConfig {
        use_pipelines: true,
        ..Default::default()
    };
    
    let backend_no_pipeline = RuchyBackend::with_config(config_no_pipeline);
    let backend_with_pipeline = RuchyBackend::with_config(config_with_pipeline);
    let hir = create_pipeline_expression();
    
    let mut group = c.benchmark_group("pipeline_transformation");
    
    group.bench_function("without_pipeline", |b| {
        b.iter(|| backend_no_pipeline.transpile(black_box(&hir)));
    });
    
    group.bench_function("with_pipeline", |b| {
        b.iter(|| backend_with_pipeline.transpile(black_box(&hir)));
    });
    
    group.finish();
}

fn bench_constant_folding(c: &mut Criterion) {
    let config_no_opt = RuchyConfig {
        optimization_level: 0,
        ..Default::default()
    };
    let config_with_opt = RuchyConfig {
        optimization_level: 2,
        ..Default::default()
    };
    
    let backend_no_opt = RuchyBackend::with_config(config_no_opt);
    let backend_with_opt = RuchyBackend::with_config(config_with_opt);
    
    // Create expression with many constant operations
    let hir = Hir {
        root: HirExpr::Binary {
            left: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Literal(HirLiteral::Integer(2))),
                op: HirBinaryOp::Multiply,
                right: Box::new(HirExpr::Literal(HirLiteral::Integer(3))),
            }),
            op: HirBinaryOp::Add,
            right: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Literal(HirLiteral::Integer(4))),
                op: HirBinaryOp::Multiply,
                right: Box::new(HirExpr::Literal(HirLiteral::Integer(5))),
            }),
        },
        metadata: Default::default(),
    };
    
    let mut group = c.benchmark_group("constant_folding");
    
    group.bench_function("without_folding", |b| {
        b.iter(|| backend_no_opt.transpile(black_box(&hir)));
    });
    
    group.bench_function("with_folding", |b| {
        b.iter(|| backend_with_opt.transpile(black_box(&hir)));
    });
    
    group.finish();
}

fn bench_validation(c: &mut Criterion) {
    #[cfg(feature = "validation")]
    {
        let backend = RuchyBackend::new();
        let valid_code = "fun add(x: i64, y: i64) -> i64 { x + y }";
        
        c.bench_function("output_validation", |b| {
            b.iter(|| backend.validate_output(black_box(valid_code)));
        });
    }
}

fn bench_memory_usage(c: &mut Criterion) {
    let backend = RuchyBackend::new();
    let small_hir = create_simple_function();
    let large_hir = create_large_list(10000);
    
    let mut group = c.benchmark_group("memory_usage");
    
    group.bench_function("small_program", |b| {
        b.iter(|| {
            let _ = black_box(backend.transpile(&small_hir));
        });
    });
    
    group.bench_function("large_program", |b| {
        b.iter(|| {
            let _ = black_box(backend.transpile(&large_hir));
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_simple_transpilation,
    bench_optimization_levels,
    bench_list_size_scaling,
    bench_expression_depth,
    bench_pipeline_transformation,
    bench_constant_folding,
    bench_validation,
    bench_memory_usage,
);

criterion_main!(benches);