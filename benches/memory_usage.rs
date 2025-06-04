use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use depyler_core::{DepylerPipeline, Config};

/// Memory usage tracking allocator
pub struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static PEAK_ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            let size = layout.size();
            let new_allocated = ALLOCATED.fetch_add(size, Ordering::Relaxed) + size;
            
            // Update peak if necessary
            let mut peak = PEAK_ALLOCATED.load(Ordering::Relaxed);
            while new_allocated > peak {
                match PEAK_ALLOCATED.compare_exchange_weak(peak, new_allocated, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(x) => peak = x,
                }
            }
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

pub fn reset_memory_tracking() {
    ALLOCATED.store(0, Ordering::Relaxed);
    PEAK_ALLOCATED.store(0, Ordering::Relaxed);
}

pub fn get_current_allocation() -> usize {
    ALLOCATED.load(Ordering::Relaxed)
}

pub fn get_peak_allocation() -> usize {
    PEAK_ALLOCATED.load(Ordering::Relaxed)
}

fn generate_memory_test_source(complexity: usize) -> String {
    let mut source = String::new();
    source.push_str("from typing import List, Dict, Optional, Tuple\n\n");
    
    // Generate increasingly complex nested data structures
    for i in 0..complexity {
        source.push_str(&format!(
            r#"
def complex_function_{}(
    data: Dict[str, List[Tuple[int, str, Optional[Dict[str, int]]]]],
    filters: List[str],
    config: Dict[str, Dict[str, List[int]]]
) -> Optional[Dict[str, List[int]]]:
    result = {{}}
    
    for key in filters:
        if key in data:
            items = data[key]
            processed = []
            
            for idx, name, metadata in items:
                if idx > {} and name.startswith("prefix_{}"):
                    if metadata is not None:
                        for meta_key, meta_value in metadata.items():
                            if meta_key in config and len(config[meta_key]) > {}:
                                processed.append(meta_value + idx)
                    else:
                        processed.append(idx * 2)
            
            if processed:
                result[key] = processed
    
    return result if result else None

"#,
            i, i * 10, i, i % 5
        ));
    }
    
    source
}

fn bench_ast_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("ast_memory");
    group.measurement_time(Duration::from_secs(10));
    
    for complexity in [10, 50, 100, 200].iter() {
        let source = generate_memory_test_source(*complexity);
        
        group.throughput(Throughput::Elements(*complexity as u64));
        group.throughput(Throughput::Bytes(source.len() as u64));
        
        group.bench_with_input(
            BenchmarkId::new("ast_size", complexity),
            &source,
            |b, source| {
                b.iter_custom(|iters| {
                    let mut total_duration = Duration::new(0, 0);
                    let mut peak_memory = 0;
                    
                    for _ in 0..iters {
                        reset_memory_tracking();
                        
                        let start = std::time::Instant::now();
                        let ast = rustpython_parser::parse_program(black_box(source))
                            .expect("Parse should succeed");
                        let duration = start.elapsed();
                        
                        // Keep AST alive for memory measurement
                        black_box(&ast);
                        
                        peak_memory = peak_memory.max(get_peak_allocation());
                        total_duration += duration;
                        
                        // Force drop
                        drop(ast);
                    }
                    
                    // Report peak memory usage
                    eprintln!("AST peak memory for {} functions: {} bytes", 
                             complexity, peak_memory);
                    
                    total_duration
                });
            },
        );
    }
    
    group.finish();
}

fn bench_hir_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("hir_memory");
    group.measurement_time(Duration::from_secs(10));
    
    for complexity in [10, 50, 100, 200].iter() {
        let source = generate_memory_test_source(*complexity);
        let pipeline = DepylerPipeline::new(Config::default());
        
        group.throughput(Throughput::Elements(*complexity as u64));
        
        group.bench_with_input(
            BenchmarkId::new("hir_size", complexity),
            &source,
            |b, source| {
                b.iter_custom(|iters| {
                    let mut total_duration = Duration::new(0, 0);
                    let mut peak_memory = 0;
                    
                    for _ in 0..iters {
                        reset_memory_tracking();
                        
                        let start = std::time::Instant::now();
                        let hir = pipeline.parse_to_hir(black_box(source))
                            .expect("Should parse to HIR");
                        let duration = start.elapsed();
                        
                        // Keep HIR alive for memory measurement
                        black_box(&hir);
                        
                        peak_memory = peak_memory.max(get_peak_allocation());
                        total_duration += duration;
                        
                        // Force drop
                        drop(hir);
                    }
                    
                    // Report peak memory usage
                    eprintln!("HIR peak memory for {} functions: {} bytes", 
                             complexity, peak_memory);
                    
                    total_duration
                });
            },
        );
    }
    
    group.finish();
}

fn bench_transpilation_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("transpilation_memory");
    group.measurement_time(Duration::from_secs(15));
    
    for complexity in [10, 50, 100, 200].iter() {
        let source = generate_memory_test_source(*complexity);
        
        group.throughput(Throughput::Elements(*complexity as u64));
        group.throughput(Throughput::Bytes(source.len() as u64));
        
        group.bench_with_input(
            BenchmarkId::new("full_pipeline_memory", complexity),
            &source,
            |b, source| {
                b.iter_custom(|iters| {
                    let mut total_duration = Duration::new(0, 0);
                    let mut peak_memory = 0;
                    let mut memory_per_byte = Vec::new();
                    
                    for _ in 0..iters {
                        reset_memory_tracking();
                        
                        let start = std::time::Instant::now();
                        let pipeline = DepylerPipeline::new(Config::default());
                        let result = pipeline.transpile(black_box(source))
                            .expect("Transpilation should succeed");
                        let duration = start.elapsed();
                        
                        // Keep result alive for memory measurement
                        black_box(&result);
                        
                        let current_peak = get_peak_allocation();
                        peak_memory = peak_memory.max(current_peak);
                        
                        // Calculate memory efficiency (bytes allocated per source byte)
                        let efficiency = current_peak as f64 / source.len() as f64;
                        memory_per_byte.push(efficiency);
                        
                        total_duration += duration;
                        
                        // Force drop
                        drop(result);
                    }
                    
                    // Report memory statistics
                    let avg_efficiency: f64 = memory_per_byte.iter().sum::<f64>() / memory_per_byte.len() as f64;
                    eprintln!("Transpilation peak memory for {} functions: {} bytes", 
                             complexity, peak_memory);
                    eprintln!("Average memory efficiency: {:.2} bytes allocated per source byte", 
                             avg_efficiency);
                    
                    // Assert memory efficiency stays reasonable (< 50x source size)
                    assert!(avg_efficiency < 50.0, 
                           "Memory usage too high: {:.2}x source size", avg_efficiency);
                    
                    total_duration
                });
            },
        );
    }
    
    group.finish();
}

fn bench_memory_leaks_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_leaks");
    group.measurement_time(Duration::from_secs(10));
    
    let source = generate_memory_test_source(50);
    
    group.bench_function("repeated_transpilation", |b| {
        b.iter_custom(|iters| {
            reset_memory_tracking();
            let initial_memory = get_current_allocation();
            
            let start = std::time::Instant::now();
            
            for _ in 0..iters {
                let pipeline = DepylerPipeline::new(Config::default());
                let result = pipeline.transpile(black_box(&source))
                    .expect("Transpilation should succeed");
                
                // Immediately drop to test cleanup
                drop(result);
                drop(pipeline);
            }
            
            let duration = start.elapsed();
            let final_memory = get_current_allocation();
            
            // Check for memory leaks (allowing some reasonable growth)
            let memory_growth = final_memory.saturating_sub(initial_memory);
            let max_acceptable_growth = iters * 1024; // 1KB per iteration max
            
            eprintln!("Memory growth after {} iterations: {} bytes", 
                     iters, memory_growth);
            
            assert!(memory_growth < max_acceptable_growth,
                   "Potential memory leak detected: {} bytes growth over {} iterations",
                   memory_growth, iters);
            
            duration
        });
    });
    
    group.finish();
}

fn bench_memory_fragmentation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_fragmentation");
    group.measurement_time(Duration::from_secs(10));
    
    // Test with many small vs few large transpilations
    let small_sources: Vec<String> = (0..100)
        .map(|i| format!("def func_{}(x: int) -> int:\n    return x + {}\n", i, i))
        .collect();
    
    let large_source = generate_memory_test_source(100);
    
    group.bench_function("many_small_transpilations", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = Duration::new(0, 0);
            
            for _ in 0..iters {
                reset_memory_tracking();
                
                let start = std::time::Instant::now();
                let pipeline = DepylerPipeline::new(Config::default());
                
                for source in &small_sources {
                    let _result = pipeline.transpile(black_box(source))
                        .expect("Small transpilation should succeed");
                }
                
                total_duration += start.elapsed();
            }
            
            total_duration
        });
    });
    
    group.bench_function("single_large_transpilation", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = Duration::new(0, 0);
            
            for _ in 0..iters {
                reset_memory_tracking();
                
                let start = std::time::Instant::now();
                let pipeline = DepylerPipeline::new(Config::default());
                let _result = pipeline.transpile(black_box(&large_source))
                    .expect("Large transpilation should succeed");
                
                total_duration += start.elapsed();
            }
            
            total_duration
        });
    });
    
    group.finish();
}

criterion_group!(
    memory_benches,
    bench_ast_memory_usage,
    bench_hir_memory_usage,
    bench_transpilation_memory_efficiency,
    bench_memory_leaks_detection,
    bench_memory_fragmentation,
);

criterion_main!(memory_benches);