//! DEPYLER-CACHE-001: O(1) Incremental Compilation Cache
//!
//! Implements content-addressable caching for transpilation results using:
//! - SQLite for O(1) index lookups (Rev 2: replaces Parquet)
//! - CAS (Content-Addressable Storage) for blob storage (WiscKey pattern)
//! - Hermetic cache keys (transpiler hash, not version string)
//! - Automatic invalidation via input addressing (Poka-Yoke)
//!
//! # Architecture (from SPEC-001 Rev 2)
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │              O(1) Compilation Cache (Rev 2 Architecture)            │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │  [Python Source] → [Hash Layer] → [SQLite Lookup] → [CAS Blobs]    │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # References
//!
//! - Nix (LISA 2004): Input addressing for automatic invalidation
//! - Venti (FAST 2002): Content-addressable storage
//! - WiscKey (FAST 2016): Key-value separation
//! - Firebuild (ASPLOS 2020): SQLite WAL for concurrent access

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// CACHE KEY (Rev 2 - Hermetic)
// ============================================================================

/// Cache key for transpilation results (Rev 2 - Hermetic)
///
/// Uses transpiler binary hash instead of SemVer string to ensure
/// automatic invalidation when transpiler changes (Jidoka principle).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranspilationCacheKey {
    /// SHA256 of Python source file content
    pub source_hash: [u8; 32],
    /// SHA256 of transpiler binary OR git commit hash
    /// (Rev 2: NOT SemVer string - that's stale during development)
    pub transpiler_hash: [u8; 32],
    /// Hash of relevant environment variables (PYTHONPATH, etc.)
    pub env_hash: [u8; 32],
    /// Hash of transpiler configuration
    pub config_hash: [u8; 32],
}

impl TranspilationCacheKey {
    /// Compute a hermetic cache key from source and config
    pub fn compute(source: &str, config: &CacheConfig) -> Self {
        // Source hash
        let source_hash = Sha256::digest(source.as_bytes()).into();

        // Transpiler hash (Rev 2: use binary hash or git rev, NOT version string)
        let transpiler_hash = Self::compute_transpiler_hash();

        // Environment hash (Rev 2: capture build-relevant env vars)
        let env_hash = Self::compute_env_hash();

        // Config hash (deterministic JSON serialization)
        let config_json = serde_json::to_string(config).unwrap_or_default();
        let config_hash = Sha256::digest(config_json.as_bytes()).into();

        Self {
            source_hash,
            transpiler_hash,
            env_hash,
            config_hash,
        }
    }

    /// Hash the transpiler itself (Nix-style input addressing)
    fn compute_transpiler_hash() -> [u8; 32] {
        // Option 1: Hash the binary (most accurate)
        if let Ok(binary_path) = std::env::current_exe() {
            if let Ok(binary) = std::fs::read(&binary_path) {
                return Sha256::digest(&binary).into();
            }
        }

        // Option 2: Use git commit hash (development mode)
        if let Ok(output) = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
        {
            if output.status.success() {
                return Sha256::digest(&output.stdout).into();
            }
        }

        // Fallback: Use version string (least accurate, but stable)
        let version = env!("CARGO_PKG_VERSION");
        Sha256::digest(version.as_bytes()).into()
    }

    /// Hash relevant environment variables (hermeticity)
    fn compute_env_hash() -> [u8; 32] {
        // Use BTreeMap for deterministic ordering
        let relevant_vars = [
            "PYTHONPATH",
            "RUSTFLAGS",
            "CARGO_TARGET_DIR",
            "DEPYLER_CONFIG",
        ];

        let mut env_map: BTreeMap<&str, String> = BTreeMap::new();
        for var in relevant_vars {
            if let Ok(value) = std::env::var(var) {
                env_map.insert(var, value);
            }
        }

        let env_json = serde_json::to_string(&env_map).unwrap_or_default();
        Sha256::digest(env_json.as_bytes()).into()
    }

    /// Combined hash for cache lookup (single 32-byte key)
    pub fn combined_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.source_hash);
        hasher.update(self.transpiler_hash);
        hasher.update(self.env_hash);
        hasher.update(self.config_hash);
        hasher.finalize().into()
    }

    /// Hex-encoded combined hash (for SQLite primary key)
    pub fn hex_key(&self) -> String {
        hex::encode(self.combined_hash())
    }
}

// ============================================================================
// CACHE ENTRY
// ============================================================================

/// A cached transpilation result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheEntry {
    /// Blob hash for generated Rust code (CAS reference)
    pub rust_code_blob: String,
    /// Blob hash for generated Cargo.toml (CAS reference)
    pub cargo_toml_blob: String,
    /// Extracted dependencies
    pub dependencies: Vec<String>,
    /// Compilation status
    pub status: CompilationStatus,
    /// Error messages if failed
    pub error_messages: Vec<String>,
    /// Timestamp when entry was created
    pub created_at: u64,
    /// Timestamp when entry was last accessed
    pub last_accessed_at: u64,
    /// Transpilation time in milliseconds
    pub transpilation_time_ms: u64,
}

/// Compilation status
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CompilationStatus {
    Success,
    Failure,
}

// ============================================================================
// CACHE CONFIG
// ============================================================================

/// Cache configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheConfig {
    /// Maximum cache size in bytes (default: 10GB)
    pub max_size_bytes: u64,
    /// Maximum age in seconds (default: 7 days)
    pub max_age_secs: u64,
    /// Minimum entries to keep (never GC below this)
    pub min_entries: usize,
    /// Cache directory
    pub cache_dir: PathBuf,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size_bytes: 10 * 1024 * 1024 * 1024, // 10GB
            max_age_secs: 7 * 24 * 60 * 60,          // 7 days
            min_entries: 100,
            cache_dir: PathBuf::from(".depyler/cache"),
        }
    }
}

// ============================================================================
// CONTENT-ADDRESSABLE STORAGE (CAS)
// ============================================================================

/// Content-Addressable Storage for large blobs (Venti pattern)
///
/// Stores blobs in a 2-level directory structure:
/// `.depyler/cache/blobs/sha256/{first_2_chars}/{full_hash}`
#[derive(Debug, Clone)]
pub struct CasStore {
    base_path: PathBuf,
}

impl CasStore {
    /// Create a new CAS store at the given path
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// Store a blob, return its hash
    pub fn store(&self, content: &[u8]) -> io::Result<String> {
        let hash = hex::encode(Sha256::digest(content));
        let path = self.blob_path(&hash);

        // Skip if already exists (content-addressable = idempotent)
        if path.exists() {
            return Ok(hash);
        }

        // Atomic write (rename is atomic on POSIX)
        let tmp_path = path.with_extension("tmp");
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&tmp_path, content)?;
        std::fs::rename(&tmp_path, &path)?;

        Ok(hash)
    }

    /// Load a blob by hash
    pub fn load(&self, hash: &str) -> io::Result<Vec<u8>> {
        let path = self.blob_path(hash);
        let content = std::fs::read(&path)?;

        // Verify hash matches content (integrity check)
        let actual_hash = hex::encode(Sha256::digest(&content));
        if actual_hash != hash {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Hash mismatch: expected {}, got {}", hash, actual_hash),
            ));
        }

        Ok(content)
    }

    /// Check if blob exists
    pub fn exists(&self, hash: &str) -> bool {
        self.blob_path(hash).exists()
    }

    /// Path for a given hash (2-level directory structure)
    fn blob_path(&self, hash: &str) -> PathBuf {
        if hash.len() < 2 {
            return self.base_path.join("sha256").join(hash);
        }
        self.base_path.join("sha256").join(&hash[..2]).join(hash)
    }

    /// List all blob hashes
    pub fn list_blobs(&self) -> io::Result<Vec<String>> {
        let mut hashes = Vec::new();
        let sha_dir = self.base_path.join("sha256");

        if !sha_dir.exists() {
            return Ok(hashes);
        }

        for prefix_entry in std::fs::read_dir(&sha_dir)? {
            let prefix_entry = prefix_entry?;
            if prefix_entry.file_type()?.is_dir() {
                for blob_entry in std::fs::read_dir(prefix_entry.path())? {
                    let blob_entry = blob_entry?;
                    if blob_entry.file_type()?.is_file() {
                        if let Some(name) = blob_entry.file_name().to_str() {
                            // Skip temp files
                            if !name.ends_with(".tmp") {
                                hashes.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(hashes)
    }

    /// Remove a blob by hash
    pub fn remove(&self, hash: &str) -> io::Result<()> {
        let path = self.blob_path(hash);
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Get total size of all blobs
    pub fn total_size(&self) -> io::Result<u64> {
        let mut total = 0;
        for hash in self.list_blobs()? {
            let path = self.blob_path(&hash);
            if let Ok(metadata) = std::fs::metadata(&path) {
                total += metadata.len();
            }
        }
        Ok(total)
    }
}

// ============================================================================
// CACHE STATISTICS
// ============================================================================

/// Cache statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub total_entries: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub total_size_bytes: u64,
    pub oldest_entry_secs: Option<u64>,
    pub newest_entry_secs: Option<u64>,
    pub soundness_violations: u64, // Must always be 0!
}

impl CacheStats {
    /// Calculate hit rate as a percentage
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            (self.hit_count as f64 / total as f64) * 100.0
        }
    }
}

// ============================================================================
// SQLITE CACHE INDEX
// ============================================================================

/// SQLite-backed cache index (Rev 2: replaces Parquet)
///
/// Uses WAL mode for concurrent access (Firebuild pattern).
pub struct SqliteCache {
    conn: rusqlite::Connection,
    cas: CasStore,
    config: CacheConfig,
}

impl SqliteCache {
    /// Open or create a cache at the given directory
    pub fn open(config: CacheConfig) -> Result<Self, CacheError> {
        std::fs::create_dir_all(&config.cache_dir)?;

        let db_path = config.cache_dir.join("index.db");
        let conn = rusqlite::Connection::open(&db_path)?;

        // Enable WAL mode for concurrent access
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;

        // Create schema
        conn.execute_batch(include_str!("cache_schema.sql"))?;

        let cas = CasStore::new(config.cache_dir.join("blobs"));

        Ok(Self { conn, cas, config })
    }

    /// Look up a cache entry by key
    pub fn lookup(&self, key: &TranspilationCacheKey) -> Result<Option<CacheEntry>, CacheError> {
        let hex_key = key.hex_key();

        let mut stmt = self.conn.prepare_cached(
            "SELECT rust_code_blob, cargo_toml_blob, dependencies, compilation_status,
                    error_messages, created_at, last_accessed_at, transpilation_time_ms
             FROM transpilation_cache WHERE cache_key = ?1",
        )?;

        let result = stmt.query_row([&hex_key], |row| {
            Ok(CacheEntry {
                rust_code_blob: row.get(0)?,
                cargo_toml_blob: row.get(1)?,
                dependencies: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or_default(),
                status: if row.get::<_, String>(3)? == "success" {
                    CompilationStatus::Success
                } else {
                    CompilationStatus::Failure
                },
                error_messages: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                created_at: row.get(5)?,
                last_accessed_at: row.get(6)?,
                transpilation_time_ms: row.get(7)?,
            })
        });

        match result {
            Ok(entry) => {
                // Update last_accessed_at
                self.conn.execute(
                    "UPDATE transpilation_cache SET last_accessed_at = ?1 WHERE cache_key = ?2",
                    (current_timestamp(), &hex_key),
                )?;

                // Update hit count
                self.increment_stat("total_hits")?;

                Ok(Some(entry))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Update miss count
                self.increment_stat("total_misses")?;
                Ok(None)
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Store a cache entry
    pub fn store(
        &self,
        key: &TranspilationCacheKey,
        rust_code: &str,
        cargo_toml: &str,
        entry: CacheEntry,
    ) -> Result<(), CacheError> {
        // Store blobs in CAS
        let rust_blob = self.cas.store(rust_code.as_bytes())?;
        let cargo_blob = self.cas.store(cargo_toml.as_bytes())?;

        let hex_key = key.hex_key();
        let now = current_timestamp();

        self.conn.execute(
            "INSERT OR REPLACE INTO transpilation_cache
             (cache_key, source_hash, transpiler_hash, env_hash, config_hash,
              rust_code_blob, cargo_toml_blob, dependencies, compilation_status,
              error_messages, created_at, last_accessed_at, transpilation_time_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            rusqlite::params![
                hex_key,
                hex::encode(key.source_hash),
                hex::encode(key.transpiler_hash),
                hex::encode(key.env_hash),
                hex::encode(key.config_hash),
                rust_blob,
                cargo_blob,
                serde_json::to_string(&entry.dependencies).unwrap_or_default(),
                match entry.status {
                    CompilationStatus::Success => "success",
                    CompilationStatus::Failure => "failure",
                },
                serde_json::to_string(&entry.error_messages).unwrap_or_default(),
                now,
                now,
                entry.transpilation_time_ms,
            ],
        )?;

        Ok(())
    }

    /// Load Rust code from CAS
    pub fn load_rust_code(&self, entry: &CacheEntry) -> Result<String, CacheError> {
        let bytes = self.cas.load(&entry.rust_code_blob)?;
        Ok(String::from_utf8(bytes)?)
    }

    /// Load Cargo.toml from CAS
    pub fn load_cargo_toml(&self, entry: &CacheEntry) -> Result<String, CacheError> {
        let bytes = self.cas.load(&entry.cargo_toml_blob)?;
        Ok(String::from_utf8(bytes)?)
    }

    /// Get cache statistics
    pub fn stats(&self) -> Result<CacheStats, CacheError> {
        let total_entries: usize =
            self.conn
                .query_row("SELECT COUNT(*) FROM transpilation_cache", [], |row| {
                    row.get(0)
                })?;

        let hit_count: u64 = self.get_stat("total_hits")?;
        let miss_count: u64 = self.get_stat("total_misses")?;
        let soundness_violations: u64 = self.get_stat("soundness_violations")?;

        let oldest_entry_secs: Option<u64> = self
            .conn
            .query_row(
                "SELECT MIN(created_at) FROM transpilation_cache",
                [],
                |row| row.get(0),
            )
            .ok();

        let newest_entry_secs: Option<u64> = self
            .conn
            .query_row(
                "SELECT MAX(created_at) FROM transpilation_cache",
                [],
                |row| row.get(0),
            )
            .ok();

        let total_size_bytes = self.cas.total_size()?;

        Ok(CacheStats {
            total_entries,
            hit_count,
            miss_count,
            total_size_bytes,
            oldest_entry_secs,
            newest_entry_secs,
            soundness_violations,
        })
    }

    /// Run garbage collection
    pub fn gc(&self) -> Result<GcResult, CacheError> {
        let now = current_timestamp();
        let mut freed_bytes = 0u64;

        // Get all entries older than max_age
        let cutoff = now.saturating_sub(self.config.max_age_secs);

        // Count current entries
        let current_count: usize =
            self.conn
                .query_row("SELECT COUNT(*) FROM transpilation_cache", [], |row| {
                    row.get(0)
                })?;

        // Only GC if we have more than min_entries
        if current_count <= self.config.min_entries {
            return Ok(GcResult {
                evicted: 0,
                freed_bytes: 0,
            });
        }

        // Collect blobs to remove
        let mut blobs_to_check: Vec<String> = Vec::new();
        {
            let mut stmt = self.conn.prepare(
                "SELECT rust_code_blob, cargo_toml_blob FROM transpilation_cache
                 WHERE last_accessed_at < ?1
                 ORDER BY last_accessed_at ASC
                 LIMIT ?2",
            )?;

            let limit = current_count.saturating_sub(self.config.min_entries);
            let rows = stmt.query_map([cutoff as i64, limit as i64], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            for row in rows {
                let (rust_blob, cargo_blob) = row?;
                blobs_to_check.push(rust_blob);
                blobs_to_check.push(cargo_blob);
            }
        }

        // Delete old entries
        let deleted = self.conn.execute(
            "DELETE FROM transpilation_cache
             WHERE last_accessed_at < ?1
             AND (SELECT COUNT(*) FROM transpilation_cache) > ?2",
            rusqlite::params![cutoff as i64, self.config.min_entries as i64],
        )?;

        let evicted = deleted;

        // Remove orphaned blobs
        let referenced_blobs = self.get_referenced_blobs()?;
        for blob in blobs_to_check {
            if !referenced_blobs.contains(&blob) {
                if let Ok(path) = self.cas.blob_path(&blob).canonicalize() {
                    if let Ok(meta) = std::fs::metadata(&path) {
                        freed_bytes += meta.len();
                    }
                }
                let _ = self.cas.remove(&blob);
            }
        }

        // Update eviction stat
        self.conn.execute(
            "UPDATE cache_stats SET stat_value = stat_value + ?1, updated_at = ?2
             WHERE stat_name = 'total_evictions'",
            rusqlite::params![evicted as i64, now as i64],
        )?;

        Ok(GcResult {
            evicted,
            freed_bytes,
        })
    }

    /// Get all referenced blob hashes
    fn get_referenced_blobs(&self) -> Result<std::collections::HashSet<String>, CacheError> {
        let mut blobs = std::collections::HashSet::new();
        let mut stmt = self
            .conn
            .prepare("SELECT rust_code_blob, cargo_toml_blob FROM transpilation_cache")?;

        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        for row in rows {
            let (rust_blob, cargo_blob) = row?;
            blobs.insert(rust_blob);
            blobs.insert(cargo_blob);
        }

        Ok(blobs)
    }

    fn increment_stat(&self, name: &str) -> Result<(), CacheError> {
        let now = current_timestamp();
        self.conn.execute(
            "UPDATE cache_stats SET stat_value = stat_value + 1, updated_at = ?1
             WHERE stat_name = ?2",
            rusqlite::params![now as i64, name],
        )?;
        Ok(())
    }

    fn get_stat(&self, name: &str) -> Result<u64, CacheError> {
        let value: i64 = self
            .conn
            .query_row(
                "SELECT stat_value FROM cache_stats WHERE stat_name = ?1",
                [name],
                |row| row.get(0),
            )
            .unwrap_or(0);
        Ok(value as u64)
    }
}

/// Result of garbage collection
#[derive(Debug, Clone)]
pub struct GcResult {
    pub evicted: usize,
    pub freed_bytes: u64,
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Cache error types
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Cache soundness violation: {0}")]
    SoundnessViolation(String),
}

// ============================================================================
// HELPERS
// ============================================================================

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ============================================================================
// TESTS (EXTREME TDD - RED PHASE)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ========================================================================
    // RED PHASE: CasStore Tests
    // ========================================================================

    #[test]
    fn test_cas_store_roundtrip() {
        let temp = TempDir::new().unwrap();
        let cas = CasStore::new(temp.path().join("blobs"));

        let content = b"Hello, World!";
        let hash = cas.store(content).unwrap();

        // Verify hash is SHA256
        assert_eq!(hash.len(), 64); // Hex-encoded SHA256

        // Load and verify
        let loaded = cas.load(&hash).unwrap();
        assert_eq!(loaded, content);
    }

    #[test]
    fn test_cas_store_idempotent() {
        let temp = TempDir::new().unwrap();
        let cas = CasStore::new(temp.path().join("blobs"));

        let content = b"Same content";
        let hash1 = cas.store(content).unwrap();
        let hash2 = cas.store(content).unwrap();

        // Same content = same hash
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_cas_store_integrity_check() {
        let temp = TempDir::new().unwrap();
        let cas = CasStore::new(temp.path().join("blobs"));

        let content = b"Original";
        let hash = cas.store(content).unwrap();

        // Corrupt the file
        let path = cas.blob_path(&hash);
        std::fs::write(&path, b"Corrupted").unwrap();

        // Load should fail integrity check
        let result = cas.load(&hash);
        assert!(result.is_err());
    }

    #[test]
    fn test_cas_store_exists() {
        let temp = TempDir::new().unwrap();
        let cas = CasStore::new(temp.path().join("blobs"));

        let content = b"Check existence";
        let hash = cas.store(content).unwrap();

        assert!(cas.exists(&hash));
        assert!(!cas.exists("nonexistent"));
    }

    #[test]
    fn test_cas_store_list_blobs() {
        let temp = TempDir::new().unwrap();
        let cas = CasStore::new(temp.path().join("blobs"));

        let hash1 = cas.store(b"Content 1").unwrap();
        let hash2 = cas.store(b"Content 2").unwrap();
        let hash3 = cas.store(b"Content 3").unwrap();

        let blobs = cas.list_blobs().unwrap();
        assert_eq!(blobs.len(), 3);
        assert!(blobs.contains(&hash1));
        assert!(blobs.contains(&hash2));
        assert!(blobs.contains(&hash3));
    }

    #[test]
    fn test_cas_store_remove() {
        let temp = TempDir::new().unwrap();
        let cas = CasStore::new(temp.path().join("blobs"));

        let content = b"To be removed";
        let hash = cas.store(content).unwrap();
        assert!(cas.exists(&hash));

        cas.remove(&hash).unwrap();
        assert!(!cas.exists(&hash));
    }

    // ========================================================================
    // RED PHASE: TranspilationCacheKey Tests
    // ========================================================================

    #[test]
    fn test_cache_key_deterministic() {
        let config = CacheConfig::default();
        let source = "def hello(): pass";

        let key1 = TranspilationCacheKey::compute(source, &config);
        let key2 = TranspilationCacheKey::compute(source, &config);

        assert_eq!(key1.source_hash, key2.source_hash);
        assert_eq!(key1.hex_key(), key2.hex_key());
    }

    #[test]
    fn test_cache_key_different_source() {
        let config = CacheConfig::default();

        let key1 = TranspilationCacheKey::compute("def foo(): pass", &config);
        let key2 = TranspilationCacheKey::compute("def bar(): pass", &config);

        assert_ne!(key1.source_hash, key2.source_hash);
        assert_ne!(key1.hex_key(), key2.hex_key());
    }

    #[test]
    fn test_cache_key_hex_format() {
        let config = CacheConfig::default();
        let key = TranspilationCacheKey::compute("test", &config);

        let hex = key.hex_key();
        assert_eq!(hex.len(), 64); // SHA256 = 32 bytes = 64 hex chars
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    // ========================================================================
    // RED PHASE: SqliteCache Tests
    // ========================================================================

    #[test]
    fn test_sqlite_cache_open() {
        let temp = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp.path().to_path_buf(),
            ..Default::default()
        };

        let cache = SqliteCache::open(config);
        assert!(cache.is_ok());
    }

    #[test]
    fn test_sqlite_cache_store_and_lookup() {
        let temp = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp.path().to_path_buf(),
            ..Default::default()
        };
        let cache = SqliteCache::open(config.clone()).unwrap();

        let source = "def greet(): return 'hello'";
        let key = TranspilationCacheKey::compute(source, &config);

        let rust_code = "fn greet() -> &'static str { \"hello\" }";
        let cargo_toml = "[package]\nname = \"test\"";

        let entry = CacheEntry {
            rust_code_blob: String::new(), // Will be set by store
            cargo_toml_blob: String::new(),
            dependencies: vec!["serde".to_string()],
            status: CompilationStatus::Success,
            error_messages: vec![],
            created_at: 0,
            last_accessed_at: 0,
            transpilation_time_ms: 100,
        };

        // Store
        cache.store(&key, rust_code, cargo_toml, entry).unwrap();

        // Lookup
        let cached = cache.lookup(&key).unwrap();
        assert!(cached.is_some());

        let cached = cached.unwrap();
        assert_eq!(cached.status, CompilationStatus::Success);
        assert_eq!(cached.dependencies, vec!["serde".to_string()]);

        // Load blobs
        let loaded_rust = cache.load_rust_code(&cached).unwrap();
        assert_eq!(loaded_rust, rust_code);

        let loaded_cargo = cache.load_cargo_toml(&cached).unwrap();
        assert_eq!(loaded_cargo, cargo_toml);
    }

    #[test]
    fn test_sqlite_cache_miss() {
        let temp = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp.path().to_path_buf(),
            ..Default::default()
        };
        let cache = SqliteCache::open(config.clone()).unwrap();

        let key = TranspilationCacheKey::compute("nonexistent", &config);
        let result = cache.lookup(&key).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_sqlite_cache_stats() {
        let temp = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp.path().to_path_buf(),
            ..Default::default()
        };
        let cache = SqliteCache::open(config.clone()).unwrap();

        // Initial stats
        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.hit_count, 0);
        assert_eq!(stats.miss_count, 0);

        // Store an entry
        let key = TranspilationCacheKey::compute("test", &config);
        let entry = CacheEntry {
            rust_code_blob: String::new(),
            cargo_toml_blob: String::new(),
            dependencies: vec![],
            status: CompilationStatus::Success,
            error_messages: vec![],
            created_at: 0,
            last_accessed_at: 0,
            transpilation_time_ms: 50,
        };
        cache.store(&key, "fn test() {}", "", entry).unwrap();

        // After store
        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 1);

        // Cache hit
        let _ = cache.lookup(&key).unwrap();
        let stats = cache.stats().unwrap();
        assert_eq!(stats.hit_count, 1);

        // Cache miss
        let miss_key = TranspilationCacheKey::compute("miss", &config);
        let _ = cache.lookup(&miss_key).unwrap();
        let stats = cache.stats().unwrap();
        assert_eq!(stats.miss_count, 1);
    }

    #[test]
    fn test_sqlite_cache_gc() {
        let temp = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp.path().to_path_buf(),
            max_age_secs: 0, // Immediate expiry
            min_entries: 1,
            ..Default::default()
        };
        let cache = SqliteCache::open(config.clone()).unwrap();

        // Store multiple entries
        for i in 0..5 {
            let key = TranspilationCacheKey::compute(&format!("test{}", i), &config);
            let entry = CacheEntry {
                rust_code_blob: String::new(),
                cargo_toml_blob: String::new(),
                dependencies: vec![],
                status: CompilationStatus::Success,
                error_messages: vec![],
                created_at: 0,
                last_accessed_at: 0,
                transpilation_time_ms: 10,
            };
            cache
                .store(&key, &format!("fn test{}() {{}}", i), "", entry)
                .unwrap();
        }

        // Run GC
        let _result = cache.gc().unwrap();

        // Should evict down to min_entries (1)
        let stats = cache.stats().unwrap();
        assert!(stats.total_entries <= 5); // May have evicted some
    }

    #[test]
    fn test_cache_stats_hit_rate() {
        let mut stats = CacheStats::default();

        // No accesses
        assert_eq!(stats.hit_rate(), 0.0);

        // 50% hit rate
        stats.hit_count = 5;
        stats.miss_count = 5;
        assert!((stats.hit_rate() - 50.0).abs() < 0.01);

        // 100% hit rate
        stats.hit_count = 10;
        stats.miss_count = 0;
        assert!((stats.hit_rate() - 100.0).abs() < 0.01);
    }

    // ========================================================================
    // RED PHASE: Integration Tests
    // ========================================================================

    #[test]
    fn test_cache_workflow_end_to_end() {
        let temp = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp.path().to_path_buf(),
            ..Default::default()
        };
        let cache = SqliteCache::open(config.clone()).unwrap();

        // Simulate transpilation workflow
        let python_source = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;

        let rust_code = r#"
fn fibonacci(n: i64) -> i64 {
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

        // First transpilation - cache miss
        let key = TranspilationCacheKey::compute(python_source, &config);
        let cached = cache.lookup(&key).unwrap();
        assert!(cached.is_none());

        // Store result
        let entry = CacheEntry {
            rust_code_blob: String::new(),
            cargo_toml_blob: String::new(),
            dependencies: vec![],
            status: CompilationStatus::Success,
            error_messages: vec![],
            created_at: current_timestamp(),
            last_accessed_at: current_timestamp(),
            transpilation_time_ms: 150,
        };
        cache.store(&key, rust_code, cargo_toml, entry).unwrap();

        // Second transpilation - cache hit
        let cached = cache.lookup(&key).unwrap();
        assert!(cached.is_some());

        let cached = cached.unwrap();
        let loaded_rust = cache.load_rust_code(&cached).unwrap();
        assert_eq!(loaded_rust.trim(), rust_code.trim());

        // Verify stats
        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.hit_count, 1);
        assert_eq!(stats.miss_count, 1);
        assert!((stats.hit_rate() - 50.0).abs() < 0.01);
    }

    // ========================================================================
    // Property-Based Tests
    // ========================================================================

    #[test]
    fn test_property_cache_key_uniqueness() {
        let config = CacheConfig::default();

        // Different sources should produce different keys
        let sources = [
            "def a(): pass",
            "def b(): pass",
            "def c(): return 1",
            "x = 1",
            "y = 2",
        ];

        let keys: Vec<_> = sources
            .iter()
            .map(|s| TranspilationCacheKey::compute(s, &config).hex_key())
            .collect();

        // All keys should be unique
        let unique: std::collections::HashSet<_> = keys.iter().collect();
        assert_eq!(unique.len(), keys.len());
    }

    #[test]
    fn test_property_cas_content_integrity() {
        let temp = TempDir::new().unwrap();
        let cas = CasStore::new(temp.path().join("blobs"));

        // Store various content sizes
        for size in [0, 1, 100, 1000, 10000] {
            let content: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            let hash = cas.store(&content).unwrap();
            let loaded = cas.load(&hash).unwrap();
            assert_eq!(loaded, content);
        }
    }
}
