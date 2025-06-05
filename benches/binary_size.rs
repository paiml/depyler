use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

/// Binary size optimization benchmarks
/// Tracks and optimizes for minimal distribution size

#[derive(Debug, Clone)]
struct SectionSizes {
    text: u64,
    data: u64,
    bss: u64,
    total: u64,
}

/// Helper struct to encapsulate build operations
struct BinaryBuilder {
    target: String,
}

impl BinaryBuilder {
    fn new(target: &str) -> Self {
        Self {
            target: target.to_string(),
        }
    }

    fn build(&self, profile: &str) -> Result<u64, String> {
        self.execute_build(profile)?;
        self.measure_size(profile)
    }

    fn build_with_flags(&self, profile: &str, flags: &[&str]) -> Result<u64, String> {
        self.execute_build_with_flags(profile, flags)?;
        self.measure_size(profile)
    }

    fn execute_build(&self, profile: &str) -> Result<(), String> {
        let output = Command::new("cargo")
            .args(&["build", "--profile", profile, "--bin", &self.target])
            .output()
            .map_err(|e| format!("Failed to build {}: {}", self.target, e))?;

        if !output.status.success() {
            return Err(format!(
                "Build failed for profile {}: {}",
                profile,
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(())
    }

    fn execute_build_with_flags(&self, profile: &str, flags: &[&str]) -> Result<(), String> {
        let mut cmd = Command::new("cargo");
        cmd.args(&["build", "--profile", profile, "--bin", &self.target]);
        
        for flag in flags {
            cmd.arg(flag);
        }

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to build {}: {}", self.target, e))?;

        if !output.status.success() {
            return Err(format!(
                "Build failed with flags: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(())
    }

    fn measure_size(&self, profile: &str) -> Result<u64, String> {
        let binary_path = self.get_binary_path(profile);
        let metadata = fs::metadata(&binary_path)
            .map_err(|e| format!("Failed to get size of {}: {}", binary_path, e))?;
        Ok(metadata.len())
    }

    fn get_binary_path(&self, profile: &str) -> String {
        match profile {
            "dev" => format!("target/debug/{}", self.target),
            "min-size" => format!("target/min-size/{}", self.target),
            _ => format!("target/{}/{}", profile, self.target),
        }
    }
}

/// Size measurement utilities
struct SizeMeasurer;

impl SizeMeasurer {
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

        Self::parse_size_output(&output.stdout)
    }

    fn parse_size_output(stdout: &[u8]) -> Result<SectionSizes, String> {
        let size_output = String::from_utf8_lossy(stdout);
        let lines: Vec<&str> = size_output.lines().collect();

        if lines.len() < 2 {
            return Err("Unexpected size output format".to_string());
        }

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

    fn compress_binary(binary_path: &str) -> Result<u64, String> {
        let output = Command::new("gzip")
            .args(&["-c", binary_path])
            .output()
            .map_err(|e| format!("Failed to compress: {}", e))?;

        Ok(output.stdout.len() as u64)
    }
}

/// Benchmark configurations
struct BenchmarkConfig {
    profiles: Vec<(&'static str, &'static str)>,
    feature_sets: Vec<(&'static str, &'static str)>,
    optimization_levels: Vec<(&'static str, &'static str)>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            profiles: vec![
                ("dev", "Development build"),
                ("release", "Release build"),
                ("min-size", "Size-optimized build"),
            ],
            feature_sets: vec![
                ("minimal", "--no-default-features"),
                ("default", ""),
                ("full", "--all-features"),
            ],
            optimization_levels: vec![
                ("0", "No optimization"),
                ("1", "Basic optimization"),
                ("2", "Default optimization"),
                ("3", "Aggressive optimization"),
                ("s", "Size optimization"),
                ("z", "Aggressive size optimization"),
            ],
        }
    }
}

/// Run a benchmark iteration and collect metrics
fn run_benchmark_iteration<F>(
    iterations: u64,
    mut operation: F,
) -> (Duration, Vec<u64>)
where
    F: FnMut() -> Result<u64, String>,
{
    let mut total_duration = Duration::new(0, 0);
    let mut sizes = Vec::new();

    for _ in 0..iterations {
        let start = std::time::Instant::now();
        
        match operation() {
            Ok(size) => sizes.push(size),
            Err(e) => eprintln!("Benchmark iteration failed: {}", e),
        }
        
        total_duration += start.elapsed();
    }

    (total_duration, sizes)
}

fn bench_binary_size_profiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_size");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10);

    let config = BenchmarkConfig::default();
    let builder = BinaryBuilder::new("depyler");

    for (profile, description) in config.profiles {
        group.bench_function(
            BenchmarkId::new("build_and_measure", profile),
            |b| {
                b.iter_custom(|iters| {
                    let (duration, sizes) = run_benchmark_iteration(iters, || {
                        builder.build(profile)
                    });

                    if !sizes.is_empty() {
                        let avg_size = sizes.iter().sum::<u64>() / sizes.len() as u64;
                        eprintln!("Average {} size: {} bytes", description, avg_size);
                        validate_size_limits(profile, avg_size);
                    }

                    duration
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

    let config = BenchmarkConfig::default();
    let builder = BinaryBuilder::new("depyler");

    for (feature_name, feature_flags) in config.feature_sets {
        group.bench_function(
            BenchmarkId::new("feature_build", feature_name),
            |b| {
                b.iter_custom(|iters| {
                    let flags: Vec<&str> = if feature_flags.is_empty() {
                        vec![]
                    } else {
                        feature_flags.split_whitespace().collect()
                    };

                    let (duration, sizes) = run_benchmark_iteration(iters, || {
                        builder.build_with_flags("release", &flags)
                    });

                    if !sizes.is_empty() {
                        let avg_size = sizes.iter().sum::<u64>() / sizes.len() as u64;
                        eprintln!("Average '{}' features size: {} bytes", feature_name, avg_size);
                    }

                    duration
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

    let config = BenchmarkConfig::default();

    for (opt_level, description) in config.optimization_levels {
        group.bench_function(
            BenchmarkId::new("opt_level", opt_level),
            |b| {
                b.iter_custom(|iters| {
                    let (duration, sizes) = run_benchmark_iteration(iters, || {
                        // Clean previous builds
                        let _ = Command::new("cargo").arg("clean").output();

                        // Build with specific optimization level
                        let output = Command::new("cargo")
                            .env("RUSTFLAGS", format!("-C opt-level={}", opt_level))
                            .args(&["build", "--release", "--bin", "depyler"])
                            .output()
                            .expect("Failed to build");

                        if output.status.success() {
                            BinaryBuilder::new("depyler").measure_size("release")
                        } else {
                            Err("Build failed".to_string())
                        }
                    });

                    if !sizes.is_empty() {
                        let avg_size = sizes.iter().sum::<u64>() / sizes.len() as u64;
                        eprintln!("Opt-level {} ({}): avg {} bytes", opt_level, description, avg_size);
                    }

                    duration
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
                    let (duration, _) = run_benchmark_iteration(iters, || {
                        benchmark_strip_compress_variant(strip, compress)
                    });
                    duration
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

    let dependency_configs = vec![
        ("minimal_deps", "--no-default-features --features minimal"),
        ("standard_deps", ""),
        ("all_deps", "--all-features"),
    ];

    let builder = BinaryBuilder::new("depyler");

    for (config_name, flags) in dependency_configs {
        group.bench_function(
            BenchmarkId::new("deps", config_name),
            |b| {
                b.iter_custom(|iters| {
                    let flag_vec: Vec<&str> = if flags.is_empty() {
                        vec![]
                    } else {
                        flags.split_whitespace().collect()
                    };

                    let (duration, sizes) = run_benchmark_iteration(iters, || {
                        builder.build_with_flags("release", &flag_vec)
                    });

                    if !sizes.is_empty() {
                        let avg_size = sizes.iter().sum::<u64>() / sizes.len() as u64;
                        eprintln!("Dependency config '{}': avg {} bytes", config_name, avg_size);
                    }

                    duration
                });
            },
        );
    }

    group.finish();
}

/// Helper functions

fn validate_size_limits(profile: &str, size: u64) {
    match profile {
        "min-size" => {
            assert!(
                size < 5_000_000, // 5MB limit for min-size
                "Min-size build too large: {} bytes",
                size
            );
        }
        "release" => {
            assert!(
                size < 10_000_000, // 10MB limit for release
                "Release build too large: {} bytes",
                size
            );
        }
        _ => {} // No limits for dev builds
    }
}

fn benchmark_strip_compress_variant(strip: bool, compress: bool) -> Result<u64, String> {
    // Build binary
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--release", "--bin", "depyler"]);

    if strip {
        cmd.env("RUSTFLAGS", "-C strip=symbols");
    }

    let output = cmd.output().map_err(|e| format!("Failed to build: {}", e))?;
    
    if !output.status.success() {
        return Err("Build failed".to_string());
    }

    let binary_path = "target/release/depyler";
    let original_size = fs::metadata(binary_path)
        .map_err(|e| format!("Failed to get metadata: {}", e))?
        .len();

    let final_size = if compress {
        SizeMeasurer::compress_binary(binary_path)?
    } else {
        original_size
    };

    eprintln!("Strip: {}, Compress: {} -> {} bytes", strip, compress, final_size);
    Ok(final_size)
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