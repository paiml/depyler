//! Module Lookup Performance Benchmarks
//!
//! DEPYLER-O1MAP-001: Compare HashMap vs PHF lookup performance
//!
//! Run with:
//! ```bash
//! cargo bench --bench module_lookup
//! cargo bench --bench module_lookup --features phf-lookup
//! ```

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;

use depyler_core::module_mapper::ModuleMapper;

/// Test modules for benchmarking
const TEST_MODULES: &[&str] = &[
    "json",
    "os",
    "sys",
    "math",
    "re",
    "random",
    "datetime",
    "collections",
    "pathlib",
    "itertools",
    "numpy",
    "sklearn.linear_model",
    "subprocess",
    "asyncio",
    "hashlib",
];

/// Test items for benchmarking
const TEST_ITEMS: &[(&str, &str)] = &[
    ("json", "loads"),
    ("json", "dumps"),
    ("math", "sqrt"),
    ("math", "sin"),
    ("os", "getcwd"),
    ("sys", "argv"),
    ("re", "compile"),
    ("random", "randint"),
];

fn bench_hashmap_module_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("module_lookup/hashmap");
    group.measurement_time(Duration::from_secs(5));

    let mapper = ModuleMapper::new();

    // Single lookup
    group.bench_function("single_lookup", |b| {
        b.iter(|| {
            black_box(mapper.get_mapping(black_box("json")))
        })
    });

    // Sequential lookups (all modules)
    group.throughput(Throughput::Elements(TEST_MODULES.len() as u64));
    group.bench_function("sequential_all_modules", |b| {
        b.iter(|| {
            for module in TEST_MODULES {
                black_box(mapper.get_mapping(black_box(module)));
            }
        })
    });

    // Miss lookup (unknown module)
    group.bench_function("miss_lookup", |b| {
        b.iter(|| {
            black_box(mapper.get_mapping(black_box("nonexistent_module")))
        })
    });

    group.finish();
}

#[cfg(feature = "phf-lookup")]
fn bench_phf_module_lookup(c: &mut Criterion) {
    use depyler_core::module_mapper_phf;

    let mut group = c.benchmark_group("module_lookup/phf");
    group.measurement_time(Duration::from_secs(5));

    // Single lookup
    group.bench_function("single_lookup", |b| {
        b.iter(|| {
            black_box(module_mapper_phf::get_module_mapping(black_box("json")))
        })
    });

    // Sequential lookups (all modules)
    group.throughput(Throughput::Elements(TEST_MODULES.len() as u64));
    group.bench_function("sequential_all_modules", |b| {
        b.iter(|| {
            for module in TEST_MODULES {
                black_box(module_mapper_phf::get_module_mapping(black_box(module)));
            }
        })
    });

    // Miss lookup (unknown module)
    group.bench_function("miss_lookup", |b| {
        b.iter(|| {
            black_box(module_mapper_phf::get_module_mapping(black_box("nonexistent_module")))
        })
    });

    group.finish();
}

#[cfg(feature = "phf-lookup")]
fn bench_phf_item_lookup(c: &mut Criterion) {
    use depyler_core::module_mapper_phf;

    let mut group = c.benchmark_group("item_lookup/phf");
    group.measurement_time(Duration::from_secs(5));

    // Single item lookup
    group.bench_function("single_lookup", |b| {
        b.iter(|| {
            black_box(module_mapper_phf::get_item_mapping(black_box("json"), black_box("loads")))
        })
    });

    // Sequential item lookups
    group.throughput(Throughput::Elements(TEST_ITEMS.len() as u64));
    group.bench_function("sequential_all_items", |b| {
        b.iter(|| {
            for (module, item) in TEST_ITEMS {
                black_box(module_mapper_phf::get_item_mapping(black_box(module), black_box(item)));
            }
        })
    });

    group.finish();
}

fn bench_mapper_initialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("mapper_init");
    group.measurement_time(Duration::from_secs(5));

    // HashMap initialization (runtime)
    group.bench_function("hashmap_new", |b| {
        b.iter(|| {
            black_box(ModuleMapper::new())
        })
    });

    group.finish();
}

fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup_scaling");
    group.measurement_time(Duration::from_secs(3));

    let mapper = ModuleMapper::new();

    // Test scaling with number of lookups
    for size in [1, 10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::new("hashmap", size), size, |b, &size| {
            b.iter(|| {
                for _ in 0..size {
                    black_box(mapper.get_mapping(black_box("json")));
                }
            })
        });
    }

    #[cfg(feature = "phf-lookup")]
    {
        use depyler_core::module_mapper_phf;
        for size in [1, 10, 100, 1000].iter() {
            group.throughput(Throughput::Elements(*size as u64));
            group.bench_with_input(BenchmarkId::new("phf", size), size, |b, &size| {
                b.iter(|| {
                    for _ in 0..size {
                        black_box(module_mapper_phf::get_module_mapping(black_box("json")));
                    }
                })
            });
        }
    }

    group.finish();
}

#[cfg(feature = "phf-lookup")]
criterion_group!(
    benches,
    bench_hashmap_module_lookup,
    bench_phf_module_lookup,
    bench_phf_item_lookup,
    bench_mapper_initialization,
    bench_scaling,
);

#[cfg(not(feature = "phf-lookup"))]
criterion_group!(
    benches,
    bench_hashmap_module_lookup,
    bench_mapper_initialization,
    bench_scaling,
);

criterion_main!(benches);
