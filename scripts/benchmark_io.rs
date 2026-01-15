//! Deterministic I/O Benchmark: Disk vs RAM Disk
//! Measures: write, read, compile latency

use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;
use std::time::Instant;
use std::process::Command;

const TEST_SIZE_MB: usize = 100;
const ITERATIONS: usize = 5;

fn benchmark_write(path: &Path) -> Vec<u128> {
    let data = vec![0xABu8; TEST_SIZE_MB * 1024 * 1024];
    let mut times = Vec::with_capacity(ITERATIONS);
    
    for i in 0..ITERATIONS {
        let file_path = path.join(format!("bench_write_{}.bin", i));
        let start = Instant::now();
        let mut f = File::create(&file_path).unwrap();
        f.write_all(&data).unwrap();
        f.sync_all().unwrap(); // Force flush to storage
        times.push(start.elapsed().as_millis());
        fs::remove_file(&file_path).ok();
    }
    times
}

fn benchmark_read(path: &Path) -> Vec<u128> {
    let data = vec![0xABu8; TEST_SIZE_MB * 1024 * 1024];
    let file_path = path.join("bench_read.bin");
    
    // Create test file
    let mut f = File::create(&file_path).unwrap();
    f.write_all(&data).unwrap();
    f.sync_all().unwrap();
    drop(f);
    
    let mut times = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        // Clear OS cache (best effort)
        let mut buf = vec![0u8; TEST_SIZE_MB * 1024 * 1024];
        let start = Instant::now();
        let mut f = File::open(&file_path).unwrap();
        f.read_exact(&mut buf).unwrap();
        times.push(start.elapsed().as_millis());
    }
    fs::remove_file(&file_path).ok();
    times
}

fn benchmark_cargo_check(target_dir: &Path) -> Vec<u128> {
    let mut times = Vec::with_capacity(ITERATIONS);
    
    // Create minimal Rust project
    let proj_dir = target_dir.join("bench_proj");
    fs::create_dir_all(&proj_dir).ok();
    
    fs::write(proj_dir.join("Cargo.toml"), r#"
[package]
name = "bench"
version = "0.1.0"
edition = "2021"

[dependencies]
"#).unwrap();
    
    fs::create_dir_all(proj_dir.join("src")).ok();
    fs::write(proj_dir.join("src/main.rs"), r#"
fn main() {
    let x: i64 = 42;
    let y: i64 = 100;
    println!("{}", x + y);
}
"#).unwrap();
    
    // Warm up
    Command::new("cargo")
        .args(["check"])
        .current_dir(&proj_dir)
        .env("CARGO_TARGET_DIR", target_dir)
        .output()
        .ok();
    
    // Clean and benchmark
    for _ in 0..ITERATIONS {
        // Clean target
        fs::remove_dir_all(target_dir.join("debug")).ok();
        
        let start = Instant::now();
        let output = Command::new("cargo")
            .args(["check"])
            .current_dir(&proj_dir)
            .env("CARGO_TARGET_DIR", target_dir)
            .output()
            .unwrap();
        times.push(start.elapsed().as_millis());
        
        if !output.status.success() {
            eprintln!("cargo check failed");
        }
    }
    
    fs::remove_dir_all(&proj_dir).ok();
    times
}

fn stats(times: &[u128]) -> (u128, u128, u128) {
    let mut sorted = times.to_vec();
    sorted.sort();
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let avg = sorted.iter().sum::<u128>() / sorted.len() as u128;
    (min, avg, max)
}

fn main() {
    let disk_path = Path::new("/Users/noahgift/src/depyler/.compile_test_tmp");
    let ram_path = Path::new("/Volumes/DepylerRAM");
    
    fs::create_dir_all(disk_path).ok();
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          DETERMINISTIC I/O BENCHMARK: DISK vs RAM            ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║ Test Size: {} MB | Iterations: {}                            ║", TEST_SIZE_MB, ITERATIONS);
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    // Write benchmark
    println!("📝 WRITE BENCHMARK ({} MB sync writes):", TEST_SIZE_MB);
    let disk_write = benchmark_write(disk_path);
    let ram_write = benchmark_write(ram_path);
    let (d_min, d_avg, d_max) = stats(&disk_write);
    let (r_min, r_avg, r_max) = stats(&ram_write);
    println!("   DISK:     min={:>5}ms  avg={:>5}ms  max={:>5}ms", d_min, d_avg, d_max);
    println!("   RAM:      min={:>5}ms  avg={:>5}ms  max={:>5}ms", r_min, r_avg, r_max);
    println!("   SPEEDUP:  {:.1}x faster", d_avg as f64 / r_avg.max(1) as f64);
    println!();
    
    // Read benchmark
    println!("📖 READ BENCHMARK ({} MB reads):", TEST_SIZE_MB);
    let disk_read = benchmark_read(disk_path);
    let ram_read = benchmark_read(ram_path);
    let (d_min, d_avg, d_max) = stats(&disk_read);
    let (r_min, r_avg, r_max) = stats(&ram_read);
    println!("   DISK:     min={:>5}ms  avg={:>5}ms  max={:>5}ms", d_min, d_avg, d_max);
    println!("   RAM:      min={:>5}ms  avg={:>5}ms  max={:>5}ms", r_min, r_avg, r_max);
    println!("   SPEEDUP:  {:.1}x faster", d_avg as f64 / r_avg.max(1) as f64);
    println!();
    
    // Cargo check benchmark
    println!("🔨 CARGO CHECK BENCHMARK (clean builds):");
    let disk_cargo = benchmark_cargo_check(&disk_path.join("target"));
    let ram_cargo = benchmark_cargo_check(&ram_path.join("target"));
    let (d_min, d_avg, d_max) = stats(&disk_cargo);
    let (r_min, r_avg, r_max) = stats(&ram_cargo);
    println!("   DISK:     min={:>5}ms  avg={:>5}ms  max={:>5}ms", d_min, d_avg, d_max);
    println!("   RAM:      min={:>5}ms  avg={:>5}ms  max={:>5}ms", r_min, r_avg, r_max);
    println!("   SPEEDUP:  {:.1}x faster", d_avg as f64 / r_avg.max(1) as f64);
    println!();
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                        CONCLUSION                            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    let total_disk = stats(&disk_write).1 + stats(&disk_read).1 + stats(&disk_cargo).1;
    let total_ram = stats(&ram_write).1 + stats(&ram_read).1 + stats(&ram_cargo).1;
    println!("   Total DISK time:  {} ms", total_disk);
    println!("   Total RAM time:   {} ms", total_ram);
    println!("   Overall speedup:  {:.1}x", total_disk as f64 / total_ram.max(1) as f64);
}
