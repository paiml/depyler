use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

/// Binary size optimization benchmarks
/// Tracks and optimizes for minimal distribution size

fn measure_binary_size(profile: &str, target: &str) -> Result<u64, String> {
    // Build with specific profile
    let output = Command::new("cargo")
        .args(&["build", "--profile", profile, "--bin", target])
        .output()
        .map_err(|e| format!("Failed to build {}: {}", target, e))?;
    
    if !output.status.success() {
        return Err(format!(
            "Build failed for profile {}: {}",
            profile,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    // Get binary path
    let binary_path = if profile == "dev" {
        format!("target/debug/{}", target)
    } else if profile == "min-size" {
        format!("target/min-size/{}", target)
    } else {
        format!("target/{}/{}", profile, target)
    };
    
    // Measure file size
    let metadata = fs::metadata(&binary_path)
        .map_err(|e| format!("Failed to get size of {}: {}", binary_path, e))?;
    
    Ok(metadata.len())
}

fn get_section_sizes(binary_path: &str) -> Result<SectionSizes, String> {
    let output = Command::new("size")
        .arg(binary_path)
        .output()
        .map_err(|e| format!("Failed to run size command: {}", e))?;
    
    if !output.status.success() {
        return Err(format!(
            "size command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    let size_output = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = size_output.lines().collect();
    
    if lines.len() < 2 {
        return Err("Unexpected size output format".to_string());
    }
    
    // Parse size output (text, data, bss, dec, hex, filename format)
    let parts: Vec<&str> = lines[1].split_whitespace().collect();
    if parts.len() < 6 {
        return Err("Unexpected size output format".to_string());
    }
    
    Ok(SectionSizes {
        text: parts[0].parse().map_err(|_| "Failed to parse text size")?,
        data: parts[1].parse().map_err(|_| "Failed to parse data size")?,
        bss: parts[2].parse().map_err(|_| "Failed to parse bss size")?,
        total: parts[3].parse().map_err(|_| "Failed to parse total size")?,
    })
}

#[derive(Debug, Clone)]
struct SectionSizes {
    text: u64,
    data: u64,
    bss: u64,
    total: u64,
}

fn bench_binary_size_profiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_size");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10);
    
    let profiles = vec![
        ("dev", "Development build"),
        ("release", "Release build"),
        ("min-size", "Size-optimized build"),
    ];
    
    for (profile, description) in profiles {
        group.bench_function(
            BenchmarkId::new("build_and_measure", profile),
            |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = Duration::new(0, 0);
                    let mut sizes = Vec::new();
                    
                    for _ in 0..iters {
                        let start = std::time::Instant::now();
                        
                        match measure_binary_size(profile, "depyler") {
                            Ok(size) => {
                                sizes.push(size);
                                eprintln!("{}: {} bytes", description, size);
                            }
                            Err(e) => {
                                eprintln!("Failed to measure {}: {}", profile, e);
                            }
                        }
                        
                        total_duration += start.elapsed();
                    }
                    
                    if !sizes.is_empty() {
                        let avg_size = sizes.iter().sum::<u64>() / sizes.len() as u64;
                        eprintln!("Average {} size: {} bytes", description, avg_size);
                        
                        // Size assertions for quality gates
                        match profile {
                            "min-size" => {
                                assert!(avg_size < 5_000_000, // 5MB limit for min-size
                                       "Min-size build too large: {} bytes", avg_size);
                            }
                            "release" => {
                                assert!(avg_size < 10_000_000, // 10MB limit for release
                                       "Release build too large: {} bytes", avg_size);
                            }
                            _ => {} // No limits for dev builds
                        }
                    }
                    
                    total_duration
                });
            },
        );
    }
    
    group.finish();
}

fn bench_feature_size_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("feature_size_impact");
    group.measurement_time(Duration::from_secs(120));
    group.sample_size(5);
    
    let feature_sets = vec![
        ("minimal", "--no-default-features"),
        ("default", ""),
        ("full", "--all-features"),
    ];
    
    for (feature_name, feature_flags) in feature_sets {
        group.bench_function(
            BenchmarkId::new("feature_build", feature_name),
            |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = Duration::new(0, 0);
                    let mut sizes = Vec::new();
                    
                    for _ in 0..iters {
                        let start = std::time::Instant::now();
                        
                        // Build with specific feature set
                        let mut cmd = Command::new("cargo");
                        cmd.args(&["build", "--release", "--bin", "depyler"]);
                        
                        if !feature_flags.is_empty() {
                            cmd.arg(feature_flags);
                        }
                        
                        let output = cmd.output().expect("Failed to build");
                        
                        if output.status.success() {
                            if let Ok(size) = measure_binary_size("release", "depyler") {
                                sizes.push(size);
                                eprintln!("Feature set '{}': {} bytes", feature_name, size);
                            }
                        } else {
                            eprintln!("Build failed for feature set '{}'", feature_name);
                        }
                        
                        total_duration += start.elapsed();
                    }
                    
                    if !sizes.is_empty() {
                        let avg_size = sizes.iter().sum::<u64>() / sizes.len() as u64;
                        eprintln!("Average '{}' features size: {} bytes", feature_name, avg_size);
                    }
                    
                    total_duration
                });
            },
        );
    }
    
    group.finish();
}

fn bench_compilation_speed_vs_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_speed_vs_size");
    group.measurement_time(Duration::from_secs(180));
    group.sample_size(5);
    
    let optimization_levels = vec![
        ("0", "No optimization"),
        ("1", "Basic optimization"),
        ("2", "Default optimization"),
        ("3", "Aggressive optimization"),
        ("s", "Size optimization"),
        ("z", "Aggressive size optimization"),
    ];
    
    for (opt_level, description) in optimization_levels {
        group.bench_function(
            BenchmarkId::new("opt_level", opt_level),
            |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = Duration::new(0, 0);
                    let mut compile_times = Vec::new();
                    let mut binary_sizes = Vec::new();
                    
                    for _ in 0..iters {
                        // Clean previous builds
                        let _ = Command::new("cargo").arg("clean").output();
                        
                        let start = std::time::Instant::now();
                        
                        // Build with specific optimization level
                        let output = Command::new("cargo")
                            .env("RUSTFLAGS", format!("-C opt-level={}", opt_level))
                            .args(&["build", "--release", "--bin", "depyler"])
                            .output()
                            .expect("Failed to build");
                        
                        let compile_duration = start.elapsed();
                        
                        if output.status.success() {
                            compile_times.push(compile_duration);
                            
                            if let Ok(size) = measure_binary_size("release", "depyler") {
                                binary_sizes.push(size);
                                eprintln!("Opt-level {}: {} ms compile, {} bytes", 
                                         opt_level, 
                                         compile_duration.as_millis(), 
                                         size);
                            }
                        }
                        
                        total_duration += compile_duration;
                    }
                    
                    if !compile_times.is_empty() && !binary_sizes.is_empty() {
                        let avg_compile_time = compile_times.iter().sum::<Duration>() / compile_times.len() as u32;
                        let avg_size = binary_sizes.iter().sum::<u64>() / binary_sizes.len() as u64;
                        
                        eprintln!("Opt-level {} ({}): avg {} ms compile, avg {} bytes", 
                                 opt_level, description, 
                                 avg_compile_time.as_millis(), avg_size);
                    }
                    
                    total_duration
                });
            },
        );
    }
    
    group.finish();
}

fn bench_strip_and_compression_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("strip_compression");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(5);
    
    let variants = vec![
        ("unstripped", false, false),
        ("stripped", true, false),
        ("compressed", false, true),
        ("stripped_compressed", true, true),
    ];
    
    for (variant_name, strip, compress) in variants {
        group.bench_function(
            BenchmarkId::new("variant", variant_name),
            |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = Duration::new(0, 0);
                    
                    for _ in 0..iters {
                        let start = std::time::Instant::now();
                        
                        // Build binary
                        let mut cmd = Command::new("cargo");
                        cmd.args(&["build", "--release", "--bin", "depyler"]);
                        
                        if strip {
                            cmd.env("RUSTFLAGS", "-C strip=symbols");
                        }
                        
                        let _ = cmd.output().expect("Failed to build");
                        
                        let binary_path = "target/release/depyler";
                        let original_size = fs::metadata(binary_path)
                            .expect("Failed to get binary metadata")
                            .len();
                        
                        let final_size = if compress {
                            // Compress with gzip
                            let output = Command::new("gzip")
                                .args(&["-c", binary_path])
                                .output()
                                .expect("Failed to compress");
                            
                            output.stdout.len() as u64
                        } else {
                            original_size
                        };
                        
                        eprintln!("Variant '{}': {} bytes", variant_name, final_size);
                        
                        total_duration += start.elapsed();
                    }
                    
                    total_duration
                });
            },
        );
    }
    
    group.finish();
}

fn bench_dependency_size_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("dependency_impact");
    group.measurement_time(Duration::from_secs(90));
    group.sample_size(3);
    
    // Test builds with and without heavy dependencies
    let dependency_configs = vec![
        ("minimal_deps", "--no-default-features --features minimal"),
        ("standard_deps", ""),
        ("all_deps", "--all-features"),
    ];
    
    for (config_name, flags) in dependency_configs {
        group.bench_function(
            BenchmarkId::new("deps", config_name),
            |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = Duration::new(0, 0);
                    
                    for _ in 0..iters {
                        let start = std::time::Instant::now();
                        
                        let mut cmd = Command::new("cargo");
                        cmd.args(&["build", "--release", "--bin", "depyler"]);
                        
                        if !flags.is_empty() {
                            for flag in flags.split_whitespace() {
                                cmd.arg(flag);
                            }
                        }
                        
                        let output = cmd.output().expect("Failed to build");
                        
                        if output.status.success() {
                            if let Ok(size) = measure_binary_size("release", "depyler") {
                                eprintln!("Dependency config '{}': {} bytes", config_name, size);
                            }
                        }
                        
                        total_duration += start.elapsed();
                    }
                    
                    total_duration
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    size_benches,
    bench_binary_size_profiles,
    bench_feature_size_impact,
    bench_compilation_speed_vs_size,
    bench_strip_and_compression_impact,
    bench_dependency_size_impact,
);

criterion_main!(size_benches);