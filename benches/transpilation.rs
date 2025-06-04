use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;

use depyler_core::{DepylerPipeline, Config};

/// Comprehensive transpilation performance benchmarks
/// Following NASA/SQLite performance standards

fn generate_python_source(lines: usize) -> String {
    let mut source = String::new();
    source.push_str("from typing import List, Dict, Optional\n\n");
    
    for i in 0..lines {
        match i % 4 {
            0 => {
                source.push_str(&format!(
                    "def function_{}(a: int, b: int) -> int:\n    return a + b + {}\n\n",
                    i, i
                ));
            }
            1 => {
                source.push_str(&format!(
                    "def list_func_{}(items: List[int]) -> int:\n    total = 0\n    for item in items:\n        total += item\n    return total + {}\n\n",
                    i, i
                ));
            }
            2 => {
                source.push_str(&format!(
                    "def dict_func_{}(data: Dict[str, int]) -> Optional[int]:\n    if 'key_{}' in data:\n        return data['key_{}']\n    return None\n\n",
                    i, i, i
                ));
            }
            3 => {
                source.push_str(&format!(
                    "def control_func_{}(n: int) -> str:\n    if n > {}:\n        return 'large'\n    elif n < 0:\n        return 'negative'\n    else:\n        return 'small'\n\n",
                    i, i * 10
                ));
            }
            _ => unreachable!(),
        }
    }
    
    source
}

fn bench_parsing_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");
    group.measurement_time(Duration::from_secs(10));
    
    // Test different source sizes
    for size in [10, 50, 100, 200, 500].iter() {
        let source = generate_python_source(*size);
        let source_bytes = source.len();
        
        group.throughput(Throughput::Bytes(source_bytes as u64));
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("rustpython_parser", size),
            &source,
            |b, source| {
                b.iter(|| {
                    // Benchmark just the parsing phase
                    rustpython_parser::parse_program(black_box(source))
                        .expect("Parse should succeed")
                });
            },
        );
    }
    
    group.finish();
}

fn bench_ast_to_hir_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("ast_to_hir");
    group.measurement_time(Duration::from_secs(10));
    
    for size in [10, 50, 100, 200].iter() {
        let source = generate_python_source(*size);
        let ast = rustpython_parser::parse_program(&source)
            .expect("Parse should succeed");
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("ast_bridge", size),
            &ast,
            |b, ast| {
                b.iter(|| {
                    // Benchmark AST to HIR conversion
                    depyler_core::ast_bridge::convert_program(black_box(ast))
                        .expect("AST conversion should succeed")
                });
            },
        );
    }
    
    group.finish();
}

fn bench_type_inference(c: &mut Criterion) {
    let mut group = c.benchmark_group("type_inference");
    group.measurement_time(Duration::from_secs(10));
    
    for size in [10, 50, 100, 200].iter() {
        let source = generate_python_source(*size);
        let pipeline = DepylerPipeline::new(Config::default());
        
        // Pre-parse to HIR for isolated type inference benchmarking
        let hir = pipeline.parse_to_hir(&source)
            .expect("Should parse to HIR");
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("type_flow_analysis", size),
            &hir,
            |b, hir| {
                b.iter(|| {
                    // Benchmark type inference phase
                    depyler_analyzer::type_flow::analyze_types(black_box(hir))
                        .expect("Type analysis should succeed")
                });
            },
        );
    }
    
    group.finish();
}

fn bench_rust_codegen(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_codegen");
    group.measurement_time(Duration::from_secs(10));
    
    for size in [10, 50, 100, 200].iter() {
        let source = generate_python_source(*size);
        let pipeline = DepylerPipeline::new(Config::default());
        
        // Pre-analyze to typed HIR for isolated codegen benchmarking
        let analyzed_hir = pipeline.analyze_to_typed_hir(&source)
            .expect("Should analyze to typed HIR");
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("rust_emission", size),
            &analyzed_hir,
            |b, hir| {
                b.iter(|| {
                    // Benchmark Rust code generation phase
                    depyler_core::codegen::generate_rust(black_box(hir))
                        .expect("Rust codegen should succeed")
                });
            },
        );
    }
    
    group.finish();
}

fn bench_end_to_end_transpilation(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");
    group.measurement_time(Duration::from_secs(15));
    
    for size in [10, 50, 100, 200, 500].iter() {
        let source = generate_python_source(*size);
        let source_bytes = source.len();
        
        group.throughput(Throughput::Bytes(source_bytes as u64));
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("full_transpilation", size),
            &source,
            |b, source| {
                let pipeline = DepylerPipeline::new(Config::default());
                
                b.iter(|| {
                    // Benchmark complete transpilation pipeline
                    pipeline.transpile(black_box(source))
                        .expect("Full transpilation should succeed")
                });
            },
        );
    }
    
    group.finish();
}

fn bench_verification_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("verification_overhead");
    group.measurement_time(Duration::from_secs(10));
    
    let source = generate_python_source(100);
    
    // Benchmark with verification disabled
    group.bench_function("without_verification", |b| {
        let config = Config { 
            enable_verification: false,
            ..Default::default()
        };
        let pipeline = DepylerPipeline::new(config);
        
        b.iter(|| {
            pipeline.transpile(black_box(&source))
                .expect("Transpilation should succeed")
        });
    });
    
    // Benchmark with verification enabled
    group.bench_function("with_verification", |b| {
        let config = Config { 
            enable_verification: true,
            ..Default::default()
        };
        let pipeline = DepylerPipeline::new(config);
        
        b.iter(|| {
            pipeline.transpile(black_box(&source))
                .expect("Transpilation should succeed")
        });
    });
    
    group.finish();
}

fn bench_real_world_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world");
    group.measurement_time(Duration::from_secs(15));
    
    // Simulate real-world Python patterns
    let scenarios = vec![
        ("data_processing", r#"
from typing import List, Dict
def process_data(items: List[Dict[str, int]]) -> Dict[str, int]:
    result = {}
    for item in items:
        for key, value in item.items():
            if key in result:
                result[key] += value
            else:
                result[key] = value
    return result
"#),
        ("algorithm_implementation", r#"
from typing import List
def quicksort(arr: List[int]) -> List[int]:
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)
"#),
        ("api_processing", r#"
from typing import Dict, Optional, List
def process_api_response(data: Dict[str, any]) -> Optional[List[str]]:
    if 'items' not in data:
        return None
    items = data['items']
    if not isinstance(items, list):
        return None
    return [str(item) for item in items if item is not None]
"#),
    ];
    
    for (scenario_name, source) in scenarios {
        group.throughput(Throughput::Bytes(source.len() as u64));
        
        group.bench_with_input(
            BenchmarkId::new("scenario", scenario_name),
            &source,
            |b, source| {
                let pipeline = DepylerPipeline::new(Config::default());
                
                b.iter(|| {
                    pipeline.transpile(black_box(source))
                        .expect("Real-world transpilation should succeed")
                });
            },
        );
    }
    
    group.finish();
}

fn bench_scalability_stress_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability");
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10); // Fewer samples for large inputs
    
    // Test with increasingly large inputs to find performance cliffs
    for size in [100, 500, 1000, 2000, 5000].iter() {
        let source = generate_python_source(*size);
        let source_bytes = source.len();
        
        group.throughput(Throughput::Bytes(source_bytes as u64));
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("large_file", size),
            &source,
            |b, source| {
                let pipeline = DepylerPipeline::new(Config::default());
                
                b.iter(|| {
                    pipeline.transpile(black_box(source))
                        .expect("Large file transpilation should succeed")
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_parsing_performance,
    bench_ast_to_hir_conversion,
    bench_type_inference,
    bench_rust_codegen,
    bench_end_to_end_transpilation,
    bench_verification_overhead,
    bench_real_world_scenarios,
    bench_scalability_stress_test,
);

criterion_main!(benches);