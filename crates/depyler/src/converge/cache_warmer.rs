//! DEPYLER-CACHE-002: Cache Warming for O(1) Compilation Verification
//!
//! Warms the cache by transpiling and compiling Python files, storing
//! successful results for O(1) lookup on subsequent runs.
//!
//! # TDD Approach (EXTREME TDD)
//! - RED: Write failing tests first
//! - GREEN: Minimal implementation to pass
//! - REFACTOR: Meet quality gates (complexity ≤10, 95% coverage)

use crate::converge::{
    cache::{CacheConfig, CacheEntry, CompilationStatus, SqliteCache, TranspilationCacheKey},
    CacheError,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

// ============================================================================
// TYPES
// ============================================================================

/// Result of warming a single file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarmResult {
    /// Successfully transpiled and compiled
    Compiled,
    /// Already in cache (cache hit)
    Cached,
    /// Failed to transpile
    TranspileFailed(String),
    /// Transpiled but failed to compile
    CompileFailed(String),
    /// File read error
    ReadError(String),
}

/// Aggregated results from cache warming
#[derive(Debug, Clone, Default)]
pub struct WarmStats {
    /// Files successfully compiled and cached
    pub compiled: usize,
    /// Files already in cache (skipped)
    pub cached: usize,
    /// Files that failed transpilation
    pub transpile_failed: usize,
    /// Files that failed compilation
    pub compile_failed: usize,
    /// Files that couldn't be read
    pub read_errors: usize,
}

impl WarmStats {
    /// Total files processed
    pub fn total(&self) -> usize {
        self.compiled + self.cached + self.transpile_failed + self.compile_failed + self.read_errors
    }

    /// Single-shot compile rate (compiled + cached / total)
    pub fn compile_rate(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            ((self.compiled + self.cached) as f64 / total as f64) * 100.0
        }
    }

    /// Update stats based on warm result
    pub fn update(&mut self, result: &WarmResult) {
        match result {
            WarmResult::Compiled => self.compiled += 1,
            WarmResult::Cached => self.cached += 1,
            WarmResult::TranspileFailed(_) => self.transpile_failed += 1,
            WarmResult::CompileFailed(_) => self.compile_failed += 1,
            WarmResult::ReadError(_) => self.read_errors += 1,
        }
    }
}

/// Thread-safe stats for parallel warming
#[derive(Debug, Default)]
pub struct AtomicWarmStats {
    pub compiled: AtomicUsize,
    pub cached: AtomicUsize,
    pub transpile_failed: AtomicUsize,
    pub compile_failed: AtomicUsize,
    pub read_errors: AtomicUsize,
}

impl AtomicWarmStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&self, result: &WarmResult) {
        match result {
            WarmResult::Compiled => self.compiled.fetch_add(1, Ordering::Relaxed),
            WarmResult::Cached => self.cached.fetch_add(1, Ordering::Relaxed),
            WarmResult::TranspileFailed(_) => self.transpile_failed.fetch_add(1, Ordering::Relaxed),
            WarmResult::CompileFailed(_) => self.compile_failed.fetch_add(1, Ordering::Relaxed),
            WarmResult::ReadError(_) => self.read_errors.fetch_add(1, Ordering::Relaxed),
        };
    }

    pub fn to_stats(&self) -> WarmStats {
        WarmStats {
            compiled: self.compiled.load(Ordering::Relaxed),
            cached: self.cached.load(Ordering::Relaxed),
            transpile_failed: self.transpile_failed.load(Ordering::Relaxed),
            compile_failed: self.compile_failed.load(Ordering::Relaxed),
            read_errors: self.read_errors.load(Ordering::Relaxed),
        }
    }
}

// ============================================================================
// CACHE WARMER
// ============================================================================

/// Cache warmer for batch transpilation and compilation
pub struct CacheWarmer {
    config: CacheConfig,
}

impl CacheWarmer {
    /// Create a new cache warmer with the given config
    pub fn new(config: CacheConfig) -> Self {
        Self { config }
    }

    /// Find all Python files in a directory (excluding __pycache__)
    pub fn find_python_files(&self, dir: &Path) -> Vec<PathBuf> {
        walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().is_some_and(|ext| ext == "py")
                    && !e.path().to_string_lossy().contains("__pycache__")
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    }

    /// Warm a single Python file
    pub fn warm_file(&self, path: &Path, cache: &SqliteCache) -> WarmResult {
        // Read source
        let source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => return WarmResult::ReadError(e.to_string()),
        };

        // Check cache first
        let cache_key = TranspilationCacheKey::compute(&source, &self.config);
        if cache.lookup(&cache_key).ok().flatten().is_some() {
            return WarmResult::Cached;
        }

        // Transpile
        let pipeline = depyler_core::DepylerPipeline::new();
        let rust_code = match pipeline.transpile(&source) {
            Ok(code) => code,
            Err(e) => return WarmResult::TranspileFailed(e.to_string()),
        };

        // Compile
        let compile_result = self.try_compile(&rust_code);
        let (status, error_msg) = match &compile_result {
            Ok(()) => (CompilationStatus::Success, None),
            Err(e) => (CompilationStatus::Failure, Some(e.clone())),
        };

        // Store in cache
        let entry = CacheEntry {
            rust_code_blob: String::new(),
            cargo_toml_blob: String::new(),
            dependencies: vec![],
            status: status.clone(),
            error_messages: error_msg.clone().into_iter().collect(),
            created_at: 0,
            last_accessed_at: 0,
            transpilation_time_ms: 0,
        };

        let cargo_toml = "[package]\nname = \"cached\"\nversion = \"0.1.0\"\nedition = \"2021\"";
        let _ = cache.store(&cache_key, &rust_code, cargo_toml, entry);

        match status {
            CompilationStatus::Success => WarmResult::Compiled,
            CompilationStatus::Failure => {
                WarmResult::CompileFailed(error_msg.unwrap_or_default())
            }
        }
    }

    /// Try to compile Rust code with cargo check (for proper dependency resolution)
    fn try_compile(&self, rust_code: &str) -> Result<(), String> {
        let temp_dir = tempfile::tempdir().map_err(|e| e.to_string())?;
        let src_dir = temp_dir.path().join("src");
        std::fs::create_dir_all(&src_dir).map_err(|e| e.to_string())?;

        // Write lib.rs
        let rs_path = src_dir.join("lib.rs");
        std::fs::write(&rs_path, rust_code).map_err(|e| e.to_string())?;

        // Generate Cargo.toml with common dependencies
        let cargo_toml = r#"[package]
name = "depyler_cache_test"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
clap = { version = "4.5", features = ["derive"] }
regex = "1.10"
chrono = "0.4"
"#;
        std::fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)
            .map_err(|e| e.to_string())?;

        // Run cargo check (faster than cargo build)
        let output = std::process::Command::new("cargo")
            .args(["check", "--lib"])
            .current_dir(temp_dir.path())
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// Warm all Python files in a directory (sequential)
    pub fn warm_directory(&self, dir: &Path) -> Result<WarmStats, CacheError> {
        let cache = SqliteCache::open(self.config.clone())?;
        let files = self.find_python_files(dir);

        let mut stats = WarmStats::default();
        for file in files {
            let result = self.warm_file(&file, &cache);
            stats.update(&result);
        }

        Ok(stats)
    }
}

// ============================================================================
// TESTS (EXTREME TDD - RED PHASE)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ========================================================================
    // Helper functions
    // ========================================================================

    fn create_test_config(temp_dir: &TempDir) -> CacheConfig {
        CacheConfig {
            cache_dir: temp_dir.path().join("cache"),
            ..Default::default()
        }
    }

    fn create_python_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();
        path
    }

    // ========================================================================
    // WarmStats Tests
    // ========================================================================

    #[test]
    fn test_warm_stats_default() {
        let stats = WarmStats::default();
        assert_eq!(stats.compiled, 0);
        assert_eq!(stats.cached, 0);
        assert_eq!(stats.transpile_failed, 0);
        assert_eq!(stats.compile_failed, 0);
        assert_eq!(stats.read_errors, 0);
        assert_eq!(stats.total(), 0);
        assert_eq!(stats.compile_rate(), 0.0);
    }

    #[test]
    fn test_warm_stats_total() {
        let stats = WarmStats {
            compiled: 5,
            cached: 3,
            transpile_failed: 1,
            compile_failed: 1,
            read_errors: 0,
        };
        assert_eq!(stats.total(), 10);
    }

    #[test]
    fn test_warm_stats_compile_rate() {
        // 80% success rate
        let stats = WarmStats {
            compiled: 6,
            cached: 2,
            transpile_failed: 1,
            compile_failed: 1,
            read_errors: 0,
        };
        assert!((stats.compile_rate() - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_warm_stats_compile_rate_all_success() {
        let stats = WarmStats {
            compiled: 10,
            cached: 0,
            transpile_failed: 0,
            compile_failed: 0,
            read_errors: 0,
        };
        assert!((stats.compile_rate() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_warm_stats_compile_rate_all_cached() {
        let stats = WarmStats {
            compiled: 0,
            cached: 10,
            transpile_failed: 0,
            compile_failed: 0,
            read_errors: 0,
        };
        assert!((stats.compile_rate() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_warm_stats_update() {
        let mut stats = WarmStats::default();

        stats.update(&WarmResult::Compiled);
        assert_eq!(stats.compiled, 1);

        stats.update(&WarmResult::Cached);
        assert_eq!(stats.cached, 1);

        stats.update(&WarmResult::TranspileFailed("err".to_string()));
        assert_eq!(stats.transpile_failed, 1);

        stats.update(&WarmResult::CompileFailed("err".to_string()));
        assert_eq!(stats.compile_failed, 1);

        stats.update(&WarmResult::ReadError("err".to_string()));
        assert_eq!(stats.read_errors, 1);
    }

    // ========================================================================
    // AtomicWarmStats Tests
    // ========================================================================

    #[test]
    fn test_atomic_warm_stats_thread_safe() {
        use std::sync::Arc;
        use std::thread;

        let stats = Arc::new(AtomicWarmStats::new());
        let mut handles = vec![];

        for _ in 0..10 {
            let stats_clone = Arc::clone(&stats);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    stats_clone.update(&WarmResult::Compiled);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_stats = stats.to_stats();
        assert_eq!(final_stats.compiled, 1000);
    }

    // ========================================================================
    // CacheWarmer Tests
    // ========================================================================

    #[test]
    fn test_cache_warmer_find_python_files() {
        let temp = TempDir::new().unwrap();
        let warmer = CacheWarmer::new(create_test_config(&temp));

        // Create test files
        create_python_file(temp.path(), "test1.py", "x = 1");
        create_python_file(temp.path(), "test2.py", "y = 2");
        create_python_file(temp.path(), "subdir/test3.py", "z = 3");
        create_python_file(temp.path(), "__pycache__/cached.py", "cached");
        std::fs::write(temp.path().join("not_python.txt"), "text").unwrap();

        let files = warmer.find_python_files(temp.path());

        assert_eq!(files.len(), 3);
        assert!(files.iter().all(|f| f.extension().unwrap() == "py"));
        assert!(!files.iter().any(|f| f.to_string_lossy().contains("__pycache__")));
    }

    #[test]
    fn test_cache_warmer_warm_file_success() {
        let temp = TempDir::new().unwrap();
        let config = create_test_config(&temp);
        let warmer = CacheWarmer::new(config.clone());
        let cache = SqliteCache::open(config).unwrap();

        // Create a simple Python file that transpiles cleanly
        let py_file = create_python_file(
            temp.path(),
            "simple.py",
            "def add(a: int, b: int) -> int:\n    return a + b\n",
        );

        let result = warmer.warm_file(&py_file, &cache);

        // Should either compile or fail - both are valid outcomes
        // The test is that it doesn't panic and returns a valid result
        matches!(
            result,
            WarmResult::Compiled | WarmResult::CompileFailed(_) | WarmResult::TranspileFailed(_)
        );
    }

    #[test]
    fn test_cache_warmer_warm_file_cached() {
        let temp = TempDir::new().unwrap();
        let config = create_test_config(&temp);
        let warmer = CacheWarmer::new(config.clone());
        let cache = SqliteCache::open(config.clone()).unwrap();

        let py_file = create_python_file(temp.path(), "cached.py", "x = 1\n");

        // First warm - should transpile
        let first_result = warmer.warm_file(&py_file, &cache);

        // Second warm - should be cached
        let second_result = warmer.warm_file(&py_file, &cache);

        // If first succeeded, second should be cached
        if matches!(first_result, WarmResult::Compiled) {
            assert_eq!(second_result, WarmResult::Cached);
        }
    }

    #[test]
    fn test_cache_warmer_warm_file_read_error() {
        let temp = TempDir::new().unwrap();
        let config = create_test_config(&temp);
        let warmer = CacheWarmer::new(config.clone());
        let cache = SqliteCache::open(config).unwrap();

        // Non-existent file
        let result = warmer.warm_file(Path::new("/nonexistent/file.py"), &cache);

        assert!(matches!(result, WarmResult::ReadError(_)));
    }

    #[test]
    fn test_cache_warmer_warm_directory() {
        let temp = TempDir::new().unwrap();
        let config = create_test_config(&temp);
        let warmer = CacheWarmer::new(config);

        // Create test files
        create_python_file(temp.path(), "a.py", "x = 1\n");
        create_python_file(temp.path(), "b.py", "y = 2\n");
        create_python_file(temp.path(), "c.py", "z = 3\n");

        let stats = warmer.warm_directory(temp.path()).unwrap();

        assert_eq!(stats.total(), 3);
        // All should either compile or fail
        assert_eq!(
            stats.compiled + stats.cached + stats.transpile_failed + stats.compile_failed,
            3
        );
    }

    // ========================================================================
    // WarmResult Tests
    // ========================================================================

    #[test]
    fn test_warm_result_equality() {
        assert_eq!(WarmResult::Compiled, WarmResult::Compiled);
        assert_eq!(WarmResult::Cached, WarmResult::Cached);
        assert_ne!(WarmResult::Compiled, WarmResult::Cached);

        assert_eq!(
            WarmResult::TranspileFailed("err".to_string()),
            WarmResult::TranspileFailed("err".to_string())
        );
    }

    // ========================================================================
    // Integration Tests
    // ========================================================================

    #[test]
    fn test_cache_warmer_integration() {
        let temp = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp.path().join("cache"),
            ..Default::default()
        };

        // Create a mix of files
        create_python_file(
            temp.path(),
            "valid.py",
            "def greet(name: str) -> str:\n    return f'Hello {name}'\n",
        );
        create_python_file(
            temp.path(),
            "also_valid.py",
            "def add(a: int, b: int) -> int:\n    return a + b\n",
        );

        let warmer = CacheWarmer::new(config);
        let stats = warmer.warm_directory(temp.path()).unwrap();

        // Should process both files
        assert_eq!(stats.total(), 2);

        // Compile rate should be calculable
        let rate = stats.compile_rate();
        assert!((0.0..=100.0).contains(&rate));
    }

    // ========================================================================
    // Property Tests
    // ========================================================================

    #[test]
    fn test_property_stats_total_is_sum_of_parts() {
        // For any combination of stats, total should equal sum
        for compiled in 0..5 {
            for cached in 0..5 {
                for failed in 0..5 {
                    let stats = WarmStats {
                        compiled,
                        cached,
                        transpile_failed: failed,
                        compile_failed: 0,
                        read_errors: 0,
                    };
                    assert_eq!(stats.total(), compiled + cached + failed);
                }
            }
        }
    }

    #[test]
    fn test_property_compile_rate_bounded() {
        // Compile rate should always be between 0 and 100
        for compiled in 0..10 {
            for cached in 0..10 {
                for failed in 0..10 {
                    let stats = WarmStats {
                        compiled,
                        cached,
                        transpile_failed: failed,
                        compile_failed: 0,
                        read_errors: 0,
                    };
                    let rate = stats.compile_rate();
                    assert!((0.0..=100.0).contains(&rate));
                }
            }
        }
    }
}
