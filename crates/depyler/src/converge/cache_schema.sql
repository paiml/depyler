-- DEPYLER-CACHE-001: SQLite Schema for O(1) Compilation Cache
-- Rev 2: Replaces Parquet with SQLite + CAS (WiscKey pattern)

-- Transpilation cache index (small, fast)
-- Blobs stored separately in CAS filesystem
CREATE TABLE IF NOT EXISTS transpilation_cache (
    -- Primary key: hex-encoded combined hash (64 chars)
    cache_key TEXT PRIMARY KEY,

    -- Component hashes (for debugging/auditing)
    source_hash TEXT NOT NULL,
    transpiler_hash TEXT NOT NULL,
    env_hash TEXT NOT NULL,
    config_hash TEXT NOT NULL,

    -- Blob references (NOT inline - WiscKey pattern)
    rust_code_blob TEXT NOT NULL,      -- SHA256 hash -> CAS path
    cargo_toml_blob TEXT NOT NULL,     -- SHA256 hash -> CAS path

    -- Metadata (small, OK to store inline)
    compilation_status TEXT NOT NULL CHECK (compilation_status IN ('success', 'failure')),
    error_messages TEXT,               -- JSON array if failure
    dependencies TEXT NOT NULL,        -- JSON array of crate names

    -- Timestamps for LRU eviction
    created_at INTEGER NOT NULL,       -- Unix timestamp
    last_accessed_at INTEGER NOT NULL, -- Updated on cache hit

    -- Metrics
    transpilation_time_ms INTEGER
);

-- Index for LRU eviction (garbage collection)
CREATE INDEX IF NOT EXISTS idx_transpilation_lru
ON transpilation_cache(last_accessed_at);

-- Index for transpiler hash (find all entries for a transpiler version)
CREATE INDEX IF NOT EXISTS idx_transpiler_hash
ON transpilation_cache(transpiler_hash);

-- Build cache index
CREATE TABLE IF NOT EXISTS build_cache (
    -- Primary key: hex-encoded build hash
    cache_key TEXT PRIMARY KEY,

    -- Foreign key to transpilation cache
    transpilation_key TEXT NOT NULL REFERENCES transpilation_cache(cache_key),

    -- Build environment
    rustc_version TEXT NOT NULL,
    llvm_version TEXT NOT NULL,
    profile TEXT NOT NULL CHECK (profile IN ('debug', 'release')),
    target_triple TEXT NOT NULL,

    -- Blob reference (NOT inline)
    binary_blob TEXT,                  -- SHA256 hash -> CAS path (NULL if lib)

    -- Metadata
    binary_size INTEGER,
    build_time_ms INTEGER,
    warnings TEXT,                     -- JSON array of clippy warnings

    -- Timestamps
    created_at INTEGER NOT NULL,
    last_accessed_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_build_lru ON build_cache(last_accessed_at);

-- Cache statistics (for monitoring)
CREATE TABLE IF NOT EXISTS cache_stats (
    stat_name TEXT PRIMARY KEY,
    stat_value INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Initialize statistics if not present
INSERT OR IGNORE INTO cache_stats (stat_name, stat_value, updated_at) VALUES
    ('total_hits', 0, 0),
    ('total_misses', 0, 0),
    ('total_evictions', 0, 0),
    ('soundness_violations', 0, 0);  -- Must always be 0!
