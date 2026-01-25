//! Cache Demo - DEPYLER-CACHE-001
//!
//! Demonstrates the O(1) incremental compilation cache API.
//!
//! # Running
//!
//! ```bash
//! cargo run -p depyler --example cache_demo
//! ```
//!
//! # What This Example Shows
//!
//! 1. Creating cache configuration
//! 2. Opening a SQLite cache with CAS backend
//! 3. Creating hermetic cache keys
//! 4. Storing and retrieving cache entries
//! 5. Viewing cache statistics
//! 6. Running garbage collection
//!
//! # Academic Foundation
//!
//! This cache design is based on:
//! - Build Systems à la Carte (Mokhov et al., ICFP 2018)
//! - WiscKey (Lu et al., FAST 2016) - Key-value separation
//! - Nix (Dolstra et al., ICSE 2004) - Input addressing

use depyler::converge::{
    CacheConfig, CacheEntry, CompilationStatus, SqliteCache, TranspilationCacheKey,
};
use tempfile::TempDir;

fn main() -> anyhow::Result<()> {
    println!("=================================================");
    println!("  DEPYLER-CACHE-001: O(1) Compilation Cache Demo  ");
    println!("=================================================\n");

    // Create a temporary directory for the cache
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();

    println!("1. Creating cache configuration...");
    let config = CacheConfig {
        cache_dir: cache_dir.clone(),
        max_size_bytes: 100 * 1024 * 1024, // 100MB
        max_age_secs: 7 * 24 * 60 * 60,    // 7 days
        min_entries: 10,
    };
    println!("   Cache directory: {}", cache_dir.display());
    println!("   Max size: {} MB", config.max_size_bytes / (1024 * 1024));
    println!(
        "   Max age: {} days\n",
        config.max_age_secs / (24 * 60 * 60)
    );

    // Open the cache
    println!("2. Opening SQLite cache with CAS backend...");
    let cache = SqliteCache::open(config.clone())?;
    println!("   Cache opened successfully!\n");

    // Create a cache key for some Python source
    println!("3. Creating hermetic cache key...");
    let python_source = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;

    // Use the compute method which creates a hermetic key from source + transpiler state
    let cache_key = TranspilationCacheKey::compute(python_source, &config);

    println!("   Source hash: {}...", &cache_key.hex_key()[..16]);
    println!("   Combined key: {}\n", cache_key.hex_key());

    // Store a cache entry
    println!("4. Storing transpilation result in cache...");
    let rust_code = r#"
pub fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
"#;

    let cargo_toml = r#"
[package]
name = "fibonacci"
version = "0.1.0"
edition = "2021"
"#;

    // Create a cache entry (blobs will be computed by store())
    let entry = CacheEntry {
        rust_code_blob: String::new(),  // Computed by store()
        cargo_toml_blob: String::new(), // Computed by store()
        dependencies: vec!["std".to_string()],
        status: CompilationStatus::Success,
        error_messages: vec![],
        created_at: 0,
        last_accessed_at: 0,
        transpilation_time_ms: 15,
    };

    cache.store(&cache_key, rust_code, cargo_toml, entry)?;
    println!("   Entry stored successfully!\n");

    // Retrieve from cache
    println!("5. Looking up cache entry...");
    match cache.lookup(&cache_key)? {
        Some(cached) => {
            println!("   CACHE HIT!");
            println!("   Status: {:?}", cached.status);
            println!("   Transpilation time: {}ms", cached.transpilation_time_ms);
            println!("   Rust code blob: {}...\n", &cached.rust_code_blob[..16]);
        }
        None => {
            println!("   CACHE MISS (unexpected)\n");
        }
    }

    // Check with a different source (should miss)
    println!("6. Testing cache miss with different source...");
    let different_key = TranspilationCacheKey::compute("def hello(): return 'world'", &config);
    match cache.lookup(&different_key)? {
        Some(_) => println!("   Unexpected hit!"),
        None => println!("   CACHE MISS (expected - different source)\n"),
    }

    // View statistics
    println!("7. Cache statistics...");
    let stats = cache.stats()?;
    println!("   Total entries: {}", stats.total_entries);
    println!("   Cache hits: {}", stats.hit_count);
    println!("   Cache misses: {}", stats.miss_count);
    println!("   Hit rate: {:.1}%", stats.hit_rate());
    println!("   Total size: {} bytes\n", stats.total_size_bytes);

    // Run garbage collection
    println!("8. Running garbage collection...");
    let gc_result = cache.gc()?;
    println!("   Entries evicted: {}", gc_result.evicted);
    println!("   Bytes freed: {}\n", gc_result.freed_bytes);

    println!("=================================================");
    println!("  Cache demo completed successfully!");
    println!("=================================================");
    println!("\nTry the CLI commands:");
    println!("  depyler cache stats");
    println!("  depyler cache gc --dry-run");
    println!("  depyler cache clear --force");

    Ok(())
}
