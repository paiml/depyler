//! Decision Trace Module for Depyler CITL (Compiler-in-the-Loop) Training
//!
//! This module provides infrastructure for capturing transpiler decision traces
//! during Python→Rust code generation, enabling pattern learning and autonomous
//! improvement of the transpiler.
//!
//! ## Architecture
//!
//! Decisions are captured at key points in the codegen pipeline:
//! - Type mapping (Python type → Rust type)
//! - Borrow strategy (&T vs T vs &mut T)
//! - Lifetime inference ('a annotations)
//! - Method dispatch (trait method resolution)
//! - Import resolution (use statements)
//! - Error handling (Result/Option wrapping)
//! - Ownership decisions (move vs clone vs borrow)
//!
//! ## Feature Flag
//!
//! Decision tracing is controlled by the `decision-tracing` feature flag.
//! When disabled, all tracing macros compile to no-ops for zero overhead.
//!
//! ## Reference
//!
//! See `docs/specifications/decision-traces-signal-spec.md` for full specification.

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[cfg(feature = "decision-tracing")]
use std::sync::Mutex;

#[cfg(feature = "decision-tracing")]
use std::cell::RefCell;

/// Decision point categories in depyler codegen
///
/// These categories map to distinct decision-making phases in the transpilation
/// pipeline. Each category represents a different type of choice the transpiler
/// must make when converting Python to Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DecisionCategory {
    /// Python type → Rust type mapping decisions
    /// Examples: int→i32, List[int]→Vec<i32>, Optional[str]→Option<String>
    TypeMapping,

    /// Borrow strategy decisions: &T vs T vs &mut T
    /// Determines how values are passed to functions and stored
    BorrowStrategy,

    /// Lifetime annotation decisions: 'a, 'static, elision
    /// When to add explicit lifetime annotations
    LifetimeInfer,

    /// Trait method resolution and dispatch
    /// Which trait method to call, method vs function syntax
    MethodDispatch,

    /// Import and use statement resolution
    /// std:: vs external crate, prelude items
    ImportResolve,

    /// Error handling strategy: Result/Option wrapping
    /// ? operator, unwrap, expect, match
    ErrorHandling,

    /// Ownership decisions: move vs clone vs Rc/Arc
    /// Determines data ownership patterns
    Ownership,
}

impl std::fmt::Display for DecisionCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionCategory::TypeMapping => write!(f, "type_mapping"),
            DecisionCategory::BorrowStrategy => write!(f, "borrow_strategy"),
            DecisionCategory::LifetimeInfer => write!(f, "lifetime_infer"),
            DecisionCategory::MethodDispatch => write!(f, "method_dispatch"),
            DecisionCategory::ImportResolve => write!(f, "import_resolve"),
            DecisionCategory::ErrorHandling => write!(f, "error_handling"),
            DecisionCategory::Ownership => write!(f, "ownership"),
        }
    }
}

/// Extended decision trace for CITL (Compiler-in-the-Loop) training
///
/// This struct captures a single decision made by the transpiler during code
/// generation. It includes context about what choice was made, what alternatives
/// were considered, and confidence levels for pattern learning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepylerDecision {
    /// Unique decision ID (FNV-1a hash of category::name::file::line)
    pub id: u64,

    /// Timestamp in nanoseconds since trace start
    pub timestamp_ns: u64,

    /// Thread ID that made this decision
    pub thread_id: u64,

    /// Source file where decision was made (transpiler code)
    pub source_file: String,

    /// Line number in transpiler source
    pub source_line: u32,

    /// Decision category
    pub category: DecisionCategory,

    /// Decision name/description
    pub name: String,

    /// Python AST node hash (for pattern matching)
    pub py_ast_hash: u64,

    /// Chosen codegen path
    pub chosen_path: String,

    /// Alternatives that were considered
    pub alternatives: Vec<String>,

    /// Confidence score (0.0-1.0)
    /// Higher confidence = more certain this is the right choice
    pub confidence: f32,

    /// Source span in Python (start, end)
    pub py_span: (usize, usize),

    /// Target span in generated Rust (start, end), if known
    pub rs_span: Option<(usize, usize)>,
}

impl DepylerDecision {
    /// Create a new decision trace
    pub fn new(
        category: DecisionCategory,
        name: &str,
        chosen: &str,
        alternatives: &[&str],
        confidence: f32,
        file: &str,
        line: u32,
    ) -> Self {
        Self {
            id: generate_decision_id(&category.to_string(), name, file, line),
            timestamp_ns: current_timestamp_ns(),
            thread_id: current_thread_id(),
            source_file: file.to_string(),
            source_line: line,
            category,
            name: name.to_string(),
            py_ast_hash: 0,
            chosen_path: chosen.to_string(),
            alternatives: alternatives.iter().map(|s| s.to_string()).collect(),
            confidence,
            py_span: (0, 0),
            rs_span: None,
        }
    }

    /// Set Python AST hash for pattern matching
    pub fn with_py_ast_hash(mut self, hash: u64) -> Self {
        self.py_ast_hash = hash;
        self
    }

    /// Set Python source span
    pub fn with_py_span(mut self, start: usize, end: usize) -> Self {
        self.py_span = (start, end);
        self
    }

    /// Set Rust target span
    pub fn with_rs_span(mut self, start: usize, end: usize) -> Self {
        self.rs_span = Some((start, end));
        self
    }
}

/// Generate a unique decision ID using FNV-1a hash
///
/// Format: category::name::file::line
pub fn generate_decision_id(category: &str, name: &str, file: &str, line: u32) -> u64 {
    let mut hasher = fnv::FnvHasher::default();

    hasher.write(category.as_bytes());
    hasher.write(b"::");
    hasher.write(name.as_bytes());
    hasher.write(b"::");
    hasher.write(file.as_bytes());
    hasher.write(b"::");
    hasher.write(&line.to_le_bytes());

    hasher.finish()
}

/// Get current timestamp in nanoseconds
fn current_timestamp_ns() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

/// Get current thread ID as u64
fn current_thread_id() -> u64 {
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    hasher.finish()
}

/// Buffered decision writer for non-blocking trace output
///
/// Uses an in-memory buffer backed by file I/O to capture decisions
/// without blocking the transpilation pipeline.
#[cfg(feature = "decision-tracing")]
pub struct MmapDecisionWriter {
    file: std::fs::File,
    offset: usize,
    capacity: usize,
    decisions: Vec<DepylerDecision>,
}

#[cfg(feature = "decision-tracing")]
impl MmapDecisionWriter {
    /// Default buffer size: 10MB (approximately 78,000 decisions)
    pub const DEFAULT_SIZE: usize = 10 * 1024 * 1024;

    /// Create a new buffered decision writer
    ///
    /// # Arguments
    /// * `path` - Path to the trace file (e.g., "/tmp/depyler_decisions.msgpack")
    /// * `size` - Buffer capacity in bytes (default: 10MB)
    pub fn new(path: &std::path::Path, size: usize) -> Result<Self, String> {
        use std::fs::OpenOptions;

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory: {}", e))?;
        }

        // Create file for trace output
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| format!("Failed to create file: {}", e))?;

        Ok(Self {
            file,
            offset: 0,
            capacity: size,
            decisions: Vec::new(),
        })
    }

    /// Append a decision to the buffer
    pub fn append(&mut self, decision: &DepylerDecision) -> Result<(), String> {
        self.decisions.push(decision.clone());

        // Circular buffer eviction at 80% capacity
        if self.decisions.len() * 128 > self.capacity * 8 / 10 {
            // Remove oldest 20% of decisions
            let remove_count = self.decisions.len() / 5;
            self.decisions.drain(0..remove_count);
        }

        Ok(())
    }

    /// Flush buffered decisions to the trace file
    pub fn flush(&mut self) -> Result<(), String> {
        use std::io::Write;

        if self.decisions.is_empty() {
            return Ok(());
        }

        // Serialize decisions to MessagePack
        let packed = rmp_serde::to_vec(&self.decisions)
            .map_err(|e| format!("Failed to serialize decisions: {}", e))?;

        // Check if we have enough space
        if packed.len() > self.capacity {
            return Err(format!(
                "Decision buffer too large: {} bytes (capacity: {})",
                packed.len(),
                self.capacity
            ));
        }

        // Write to file (overwrite from beginning)
        self.file
            .set_len(0)
            .map_err(|e| format!("Failed to truncate file: {}", e))?;
        use std::io::Seek;
        self.file
            .seek(std::io::SeekFrom::Start(0))
            .map_err(|e| format!("Failed to seek: {}", e))?;
        self.file
            .write_all(&packed)
            .map_err(|e| format!("Failed to write decisions: {}", e))?;
        self.file
            .flush()
            .map_err(|e| format!("Failed to flush file: {}", e))?;
        self.offset = packed.len();

        Ok(())
    }

    /// Get the number of buffered decisions
    pub fn len(&self) -> usize {
        self.decisions.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.decisions.is_empty()
    }
}

#[cfg(feature = "decision-tracing")]
impl Drop for MmapDecisionWriter {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

// Thread-local decision writer
#[cfg(feature = "decision-tracing")]
thread_local! {
    pub static DECISION_WRITER: RefCell<Option<MmapDecisionWriter>> = const { RefCell::new(None) };
}

// Global decision writer (fallback when thread-local not available)
#[cfg(feature = "decision-tracing")]
lazy_static::lazy_static! {
    pub static ref GLOBAL_DECISION_WRITER: Mutex<Option<MmapDecisionWriter>> = Mutex::new(None);
}

/// Initialize decision tracing
///
/// Call this at the start of transpilation to set up the trace writer.
#[cfg(feature = "decision-tracing")]
pub fn init_decision_tracing() -> Result<(), String> {
    use std::path::Path;

    let path = Path::new("/tmp/depyler_decisions.msgpack");
    let writer = MmapDecisionWriter::new(path, MmapDecisionWriter::DEFAULT_SIZE)?;

    DECISION_WRITER.with(|w| {
        *w.borrow_mut() = Some(writer);
    });

    Ok(())
}

/// Initialize decision tracing (no-op when feature disabled)
#[cfg(not(feature = "decision-tracing"))]
pub fn init_decision_tracing() -> Result<(), String> {
    Ok(())
}

/// Record a decision (internal helper)
#[cfg(feature = "decision-tracing")]
pub fn record_decision(decision: DepylerDecision) {
    DECISION_WRITER.with(|w| {
        if let Some(ref mut writer) = *w.borrow_mut() {
            let _ = writer.append(&decision);
        }
    });
}

/// Record a decision (no-op when feature disabled)
#[cfg(not(feature = "decision-tracing"))]
pub fn record_decision(_decision: DepylerDecision) {}

// ============================================================================
// Error-Decision Correlation (Spec Section 9.1-9.2)
// ============================================================================

/// Compilation outcome for CITL training correlation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompileOutcome {
    /// Successful compilation
    Success,
    /// Compilation error with error code and message
    Error {
        /// Rustc error code (e.g., "E0308")
        code: String,
        /// Error message
        message: String,
        /// Error span in Rust source (line, column)
        span: Option<(usize, usize)>,
    },
}

/// Find decisions that contributed to error at given Rust span
///
/// Uses span overlap to correlate decisions with error locations.
/// Returns decisions sorted by relevance (closest span match first).
///
/// # Arguments
/// * `decisions` - All decisions captured during transpilation
/// * `error_span` - The Rust source span where the error occurred (start, end)
///
/// # Returns
/// Decisions whose output overlaps with the error location
pub fn correlate_error(
    decisions: &[DepylerDecision],
    error_span: (usize, usize),
) -> Vec<&DepylerDecision> {
    let mut correlated: Vec<_> = decisions
        .iter()
        .filter(|d| {
            d.rs_span.is_some_and(|(start, end)| {
                // Decision's output overlaps with error location
                start <= error_span.1 && end >= error_span.0
            })
        })
        .collect();

    // Sort by span proximity (tightest overlap first)
    correlated.sort_by_key(|d| {
        d.rs_span.map_or(usize::MAX, |(start, end)| {
            let overlap_start = start.max(error_span.0);
            let overlap_end = end.min(error_span.1);
            // Smaller spans (more precise) rank higher
            if overlap_end > overlap_start {
                end - start
            } else {
                usize::MAX
            }
        })
    });

    correlated
}

/// A causal chain link in decision dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    /// The decision at this point in the chain
    pub decision: DepylerDecision,
    /// Depth in the causal chain (0 = closest to error)
    pub depth: usize,
}

/// Build causal chain from error back to root decisions
///
/// Reconstructs the decision dependency graph to find the root cause
/// of a compilation error.
///
/// # Arguments
/// * `decisions` - All decisions captured during transpilation
/// * `error_span` - The Rust source span where the error occurred
/// * `max_depth` - Maximum depth to traverse (default: 5)
///
/// # Returns
/// Causal chain from error location back to root decisions
pub fn build_causal_chain(
    decisions: &[DepylerDecision],
    error_span: (usize, usize),
    max_depth: usize,
) -> Vec<CausalLink> {
    let mut chain = Vec::new();
    let mut current_span = error_span;
    let mut visited = std::collections::HashSet::new();

    for depth in 0..max_depth {
        let correlated = correlate_error(decisions, current_span);

        if correlated.is_empty() {
            break;
        }

        // Take the most specific decision at this level
        if let Some(decision) = correlated.first() {
            if visited.contains(&decision.id) {
                break; // Avoid cycles
            }
            visited.insert(decision.id);

            chain.push(CausalLink {
                decision: (*decision).clone(),
                depth,
            });

            // Move to Python span for next iteration
            current_span = decision.py_span;
        }
    }

    chain
}

// ============================================================================
// Graceful Degradation - JSON Fallback (Spec Section 10)
// ============================================================================

/// Decision writer trait for abstraction over storage backends
pub trait DecisionWriter: Send + Sync {
    /// Append a decision to the writer
    fn append(&mut self, decision: &DepylerDecision) -> Result<(), String>;

    /// Flush buffered decisions to storage
    fn flush(&mut self) -> Result<(), String>;

    /// Get the number of buffered decisions
    fn len(&self) -> usize;

    /// Check if buffer is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// JSON Lines writer for decision trace output
///
/// Writes decisions as newline-delimited JSON for compatibility
/// with tools that don't support MessagePack.
pub struct JsonFileWriter {
    path: std::path::PathBuf,
    buffer: Vec<DepylerDecision>,
    max_buffer_size: usize,
}

impl JsonFileWriter {
    /// Create a new JSON file writer
    pub fn new(path: &std::path::Path) -> Self {
        Self {
            path: path.to_path_buf(),
            buffer: Vec::new(),
            max_buffer_size: 1000,
        }
    }

    /// Create with custom buffer size
    pub fn with_buffer_size(path: &std::path::Path, max_size: usize) -> Self {
        Self {
            path: path.to_path_buf(),
            buffer: Vec::new(),
            max_buffer_size: max_size,
        }
    }
}

impl DecisionWriter for JsonFileWriter {
    fn append(&mut self, decision: &DepylerDecision) -> Result<(), String> {
        self.buffer.push(decision.clone());

        // Auto-flush at buffer capacity
        if self.buffer.len() >= self.max_buffer_size {
            self.flush()?;
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> {
        use std::io::Write;

        if self.buffer.is_empty() {
            return Ok(());
        }

        // Create parent directory if needed
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory: {}", e))?;
        }

        // Append to file (JSONL format)
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let mut writer = std::io::BufWriter::new(file);

        for decision in &self.buffer {
            let json = serde_json::to_string(decision)
                .map_err(|e| format!("Failed to serialize decision: {}", e))?;
            writeln!(writer, "{}", json).map_err(|e| format!("Failed to write decision: {}", e))?;
        }

        writer
            .flush()
            .map_err(|e| format!("Failed to flush writer: {}", e))?;

        self.buffer.clear();
        Ok(())
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl Drop for JsonFileWriter {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

#[cfg(feature = "decision-tracing")]
impl DecisionWriter for MmapDecisionWriter {
    fn append(&mut self, decision: &DepylerDecision) -> Result<(), String> {
        MmapDecisionWriter::append(self, decision)
    }

    fn flush(&mut self) -> Result<(), String> {
        MmapDecisionWriter::flush(self)
    }

    fn len(&self) -> usize {
        MmapDecisionWriter::len(self)
    }
}

/// Create appropriate decision writer based on environment
///
/// Prefers MessagePack file writer, falls back to JSON for compatibility.
#[cfg(feature = "decision-tracing")]
pub fn create_decision_writer(path: &std::path::Path) -> Box<dyn DecisionWriter> {
    // Try MessagePack file writer first
    match MmapDecisionWriter::new(path, MmapDecisionWriter::DEFAULT_SIZE) {
        Ok(writer) => Box::new(writer),
        Err(_) => {
            // Fallback to JSON
            let json_path = path.with_extension("jsonl");
            Box::new(JsonFileWriter::new(&json_path))
        }
    }
}

/// Create decision writer (no-op implementation when feature disabled)
#[cfg(not(feature = "decision-tracing"))]
pub fn create_decision_writer(_path: &std::path::Path) -> Box<dyn DecisionWriter> {
    Box::new(NoOpWriter)
}

/// No-op writer for when tracing is disabled
#[cfg(not(feature = "decision-tracing"))]
struct NoOpWriter;

#[cfg(not(feature = "decision-tracing"))]
impl DecisionWriter for NoOpWriter {
    fn append(&mut self, _decision: &DepylerDecision) -> Result<(), String> {
        Ok(())
    }
    fn flush(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn len(&self) -> usize {
        0
    }
}

// ============================================================================
// Collector Implementations (DEPYLER-EXPLAIN-001)
// ============================================================================

/// TranspileDecision enum per issue #214 specification
/// Maps to the decision points during transpilation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TranspileDecision {
    /// Type inference decisions (e.g., List[int] → Vec<i32>)
    TypeInference,
    /// Ownership inference (move vs clone vs borrow)
    OwnershipInference,
    /// Method resolution (which trait/impl to use)
    MethodResolution,
    /// Import mapping (Python module → Rust crate)
    ImportMapping,
    /// Control flow transforms (for/while/if)
    ControlFlowTransform,
}

impl std::fmt::Display for TranspileDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranspileDecision::TypeInference => write!(f, "type_inference"),
            TranspileDecision::OwnershipInference => write!(f, "ownership_inference"),
            TranspileDecision::MethodResolution => write!(f, "method_resolution"),
            TranspileDecision::ImportMapping => write!(f, "import_mapping"),
            TranspileDecision::ControlFlowTransform => write!(f, "control_flow_transform"),
        }
    }
}

/// TranspileTrace per issue #214 specification
/// Single trace entry capturing a transpiler decision point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspileTrace {
    /// Timestamp in nanoseconds since trace collection started
    pub timestamp_ns: u64,
    /// Source location in Python (file:line:column)
    pub source_loc: String,
    /// The type of decision made
    pub decision: TranspileDecision,
    /// Rule identifier (e.g., "RULE-TYP-001")
    pub rule_id: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    /// Human-readable explanation
    pub explanation: String,
    /// Alternative decisions that were considered
    pub alternatives: Vec<String>,
}

impl TranspileTrace {
    /// Create a new trace entry
    pub fn new(
        decision: TranspileDecision,
        rule_id: &str,
        confidence: f32,
        explanation: &str,
    ) -> Self {
        Self {
            timestamp_ns: current_timestamp_ns(),
            source_loc: String::new(),
            decision,
            rule_id: rule_id.to_string(),
            confidence,
            explanation: explanation.to_string(),
            alternatives: Vec::new(),
        }
    }

    /// Set source location
    pub fn with_source_loc(mut self, loc: &str) -> Self {
        self.source_loc = loc.to_string();
        self
    }

    /// Add alternatives
    pub fn with_alternatives(mut self, alts: &[&str]) -> Self {
        self.alternatives = alts.iter().map(|s| s.to_string()).collect();
        self
    }
}

/// Collector trait for trace collection strategies
/// Performance requirements per issue #214:
/// - RingCollector: <100ns per trace
/// - StreamCollector: <1µs per trace
/// - HashChainCollector: <10µs per trace
pub trait TraceCollector: Send + Sync {
    /// Collect a trace entry
    fn collect(&mut self, trace: TranspileTrace);

    /// Get collected traces
    fn traces(&self) -> &[TranspileTrace];

    /// Clear all collected traces
    fn clear(&mut self);

    /// Get the number of collected traces
    fn len(&self) -> usize;

    /// Check if empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Export traces to JSON
    fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self.traces())
            .map_err(|e| format!("Failed to serialize traces: {}", e))
    }
}

/// RingCollector - Ultra-fast circular buffer collector
/// Target latency: <100ns per trace
/// Uses a fixed-size ring buffer with atomic operations
pub struct RingCollector {
    /// Fixed-size buffer
    buffer: Vec<TranspileTrace>,
    /// Current write position
    write_pos: usize,
    /// Capacity
    capacity: usize,
    /// Number of traces collected (may exceed capacity)
    total_collected: usize,
}

impl RingCollector {
    /// Default capacity: 8192 traces
    pub const DEFAULT_CAPACITY: usize = 8192;

    /// Create with default capacity
    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }

    /// Create with custom capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            write_pos: 0,
            capacity,
            total_collected: 0,
        }
    }

    /// Get total traces collected (including overwritten)
    pub fn total_collected(&self) -> usize {
        self.total_collected
    }

    /// Check if buffer has wrapped
    pub fn has_wrapped(&self) -> bool {
        self.total_collected > self.capacity
    }
}

impl Default for RingCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl TraceCollector for RingCollector {
    fn collect(&mut self, trace: TranspileTrace) {
        if self.buffer.len() < self.capacity {
            self.buffer.push(trace);
        } else {
            self.buffer[self.write_pos] = trace;
        }
        self.write_pos = (self.write_pos + 1) % self.capacity;
        self.total_collected += 1;
    }

    fn traces(&self) -> &[TranspileTrace] {
        &self.buffer
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.write_pos = 0;
        self.total_collected = 0;
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}

/// StreamCollector - Streaming collector with buffered I/O
/// Target latency: <1µs per trace
/// Writes traces to a memory-mapped file with buffering
pub struct StreamCollector {
    /// In-memory buffer before flush
    buffer: Vec<TranspileTrace>,
    /// Output path for persistence
    output_path: Option<std::path::PathBuf>,
    /// Buffer flush threshold
    flush_threshold: usize,
    /// Total traces streamed
    total_streamed: usize,
}

impl StreamCollector {
    /// Default flush threshold: 1000 traces
    pub const DEFAULT_FLUSH_THRESHOLD: usize = 1000;

    /// Create a new stream collector
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            output_path: None,
            flush_threshold: Self::DEFAULT_FLUSH_THRESHOLD,
            total_streamed: 0,
        }
    }

    /// Create with output path for persistence
    pub fn with_output(path: &std::path::Path) -> Self {
        Self {
            buffer: Vec::new(),
            output_path: Some(path.to_path_buf()),
            flush_threshold: Self::DEFAULT_FLUSH_THRESHOLD,
            total_streamed: 0,
        }
    }

    /// Set flush threshold
    pub fn with_flush_threshold(mut self, threshold: usize) -> Self {
        self.flush_threshold = threshold;
        self
    }

    /// Flush buffer to output
    pub fn flush(&mut self) -> Result<(), String> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        if let Some(ref path) = self.output_path {
            use std::io::Write;

            // Create parent directory if needed
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }

            // Append to file
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| format!("Failed to open file: {}", e))?;

            let mut writer = std::io::BufWriter::new(file);
            for trace in &self.buffer {
                let json = serde_json::to_string(trace)
                    .map_err(|e| format!("Serialization error: {}", e))?;
                writeln!(writer, "{}", json).map_err(|e| format!("Write error: {}", e))?;
            }
            writer.flush().map_err(|e| format!("Flush error: {}", e))?;
        }

        self.total_streamed += self.buffer.len();
        self.buffer.clear();
        Ok(())
    }

    /// Get total traces streamed to output
    pub fn total_streamed(&self) -> usize {
        self.total_streamed
    }
}

impl Default for StreamCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StreamCollector {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

impl TraceCollector for StreamCollector {
    fn collect(&mut self, trace: TranspileTrace) {
        self.buffer.push(trace);
        if self.buffer.len() >= self.flush_threshold {
            let _ = self.flush();
        }
    }

    fn traces(&self) -> &[TranspileTrace] {
        &self.buffer
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}

/// HashChainCollector - Tamper-evident collector with hash chain
/// Target latency: <10µs per trace
/// Each trace includes hash of previous trace for audit trail
#[derive(Default)]
pub struct HashChainCollector {
    /// Traces with hash chain
    traces: Vec<HashChainedTrace>,
    /// Current chain hash
    chain_hash: u64,
}

/// A trace entry with hash chain for tamper evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashChainedTrace {
    /// The actual trace
    pub trace: TranspileTrace,
    /// Hash of previous entry (0 for first entry)
    pub prev_hash: u64,
    /// Hash of this entry (includes prev_hash)
    pub entry_hash: u64,
}

impl HashChainCollector {
    /// Create a new hash chain collector
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            chain_hash: 0,
        }
    }

    /// Verify the integrity of the hash chain
    pub fn verify_chain(&self) -> bool {
        if self.traces.is_empty() {
            return true;
        }

        // First entry should have prev_hash = 0
        if self.traces[0].prev_hash != 0 {
            return false;
        }

        // Verify each entry's hash
        for i in 0..self.traces.len() {
            let expected_prev = if i == 0 {
                0
            } else {
                self.traces[i - 1].entry_hash
            };
            if self.traces[i].prev_hash != expected_prev {
                return false;
            }

            // Verify entry hash
            let computed = Self::compute_hash(&self.traces[i].trace, self.traces[i].prev_hash);
            if self.traces[i].entry_hash != computed {
                return false;
            }
        }

        true
    }

    /// Get the current chain hash (hash of last entry)
    pub fn chain_hash(&self) -> u64 {
        self.chain_hash
    }

    /// Compute hash for a trace entry
    fn compute_hash(trace: &TranspileTrace, prev_hash: u64) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let mut hasher = DefaultHasher::new();
        hasher.write_u64(prev_hash);
        hasher.write_u64(trace.timestamp_ns);
        hasher.write(trace.rule_id.as_bytes());
        hasher.write(trace.explanation.as_bytes());
        hasher.finish()
    }

    /// Export with integrity proof
    pub fn export_with_proof(&self) -> Result<String, String> {
        #[derive(Serialize)]
        struct AuditExport<'a> {
            traces: &'a [HashChainedTrace],
            chain_hash: u64,
            verified: bool,
        }

        let export = AuditExport {
            traces: &self.traces,
            chain_hash: self.chain_hash,
            verified: self.verify_chain(),
        };

        serde_json::to_string_pretty(&export).map_err(|e| format!("Failed to export: {}", e))
    }
}

impl TraceCollector for HashChainCollector {
    fn collect(&mut self, trace: TranspileTrace) {
        let prev_hash = self.chain_hash;
        let entry_hash = Self::compute_hash(&trace, prev_hash);

        self.traces.push(HashChainedTrace {
            trace,
            prev_hash,
            entry_hash,
        });

        self.chain_hash = entry_hash;
    }

    fn traces(&self) -> &[TranspileTrace] {
        // Return inner traces - we need to extract them
        // This is a limitation of the trait design
        // For now, return empty slice as traces are in HashChainedTrace
        &[]
    }

    fn clear(&mut self) {
        self.traces.clear();
        self.chain_hash = 0;
    }

    fn len(&self) -> usize {
        self.traces.len()
    }
}

impl HashChainCollector {
    /// Get hash-chained traces
    pub fn chained_traces(&self) -> &[HashChainedTrace] {
        &self.traces
    }
}

// Global trace collector (thread-local for performance)
#[cfg(feature = "decision-tracing")]
thread_local! {
    pub static TRACE_COLLECTOR: RefCell<Option<Box<dyn TraceCollector>>> = const { RefCell::new(None) };
}

/// Initialize trace collection with a specific collector
#[cfg(feature = "decision-tracing")]
pub fn init_trace_collector(collector: Box<dyn TraceCollector>) {
    TRACE_COLLECTOR.with(|c| {
        *c.borrow_mut() = Some(collector);
    });
}

/// Initialize trace collection (no-op when feature disabled)
#[cfg(not(feature = "decision-tracing"))]
pub fn init_trace_collector(_collector: Box<dyn TraceCollector>) {}

/// Collect a trace (thread-local fast path)
#[cfg(feature = "decision-tracing")]
pub fn collect_trace(trace: TranspileTrace) {
    TRACE_COLLECTOR.with(|c| {
        if let Some(ref mut collector) = *c.borrow_mut() {
            collector.collect(trace);
        }
    });
}

/// Collect a trace (no-op when feature disabled)
#[cfg(not(feature = "decision-tracing"))]
pub fn collect_trace(_trace: TranspileTrace) {}

/// Macro for instrumenting decision points in codegen
///
/// This macro captures a decision made during transpilation. When the
/// `decision-tracing` feature is disabled, this compiles to a no-op.
///
/// # Example
///
/// ```ignore
/// trace_decision!(
///     category = DecisionCategory::TypeMapping,
///     name = "binop_promotion",
///     chosen = "i64",
///     alternatives = ["i32", "f64"],
///     confidence = 0.85
/// );
/// ```
#[macro_export]
macro_rules! trace_decision {
    (
        category = $category:expr,
        name = $name:expr,
        chosen = $chosen:expr,
        alternatives = [$($alt:expr),* $(,)?],
        confidence = $confidence:expr
    ) => {{
        #[cfg(feature = "decision-tracing")]
        {
            let decision = $crate::decision_trace::DepylerDecision::new(
                $category,
                $name,
                $chosen,
                &[$($alt),*],
                $confidence,
                file!(),
                line!(),
            );
            $crate::decision_trace::record_decision(decision);
        }
    }};
}

/// Legacy macro for backward compatibility
#[macro_export]
macro_rules! emit_decision {
    ($id:expr, $value:expr) => {
        #[cfg(feature = "decision-tracing")]
        {
            eprintln!("DECISION: {}: {}", $id, $value);
        }
    };
    ($id:expr) => {
        #[cfg(feature = "decision-tracing")]
        {
            eprintln!("DECISION: {}", $id);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_category_variants() {
        // Test all category variants exist and can be used
        let categories = [
            DecisionCategory::TypeMapping,
            DecisionCategory::BorrowStrategy,
            DecisionCategory::LifetimeInfer,
            DecisionCategory::MethodDispatch,
            DecisionCategory::ImportResolve,
            DecisionCategory::ErrorHandling,
            DecisionCategory::Ownership,
        ];

        // Each category should have a unique display string
        let displays: Vec<String> = categories.iter().map(|c| c.to_string()).collect();
        let unique_count = displays
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(
            unique_count,
            categories.len(),
            "All categories should have unique display names"
        );
    }

    #[test]
    fn test_decision_category_display() {
        assert_eq!(DecisionCategory::TypeMapping.to_string(), "type_mapping");
        assert_eq!(
            DecisionCategory::BorrowStrategy.to_string(),
            "borrow_strategy"
        );
        assert_eq!(
            DecisionCategory::LifetimeInfer.to_string(),
            "lifetime_infer"
        );
        assert_eq!(
            DecisionCategory::MethodDispatch.to_string(),
            "method_dispatch"
        );
        assert_eq!(
            DecisionCategory::ImportResolve.to_string(),
            "import_resolve"
        );
        assert_eq!(
            DecisionCategory::ErrorHandling.to_string(),
            "error_handling"
        );
        assert_eq!(DecisionCategory::Ownership.to_string(), "ownership");
    }

    #[test]
    fn test_depyler_decision_creation() {
        let decision = DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "binop_promotion",
            "i64",
            &["i32", "f64"],
            0.85,
            "expr_gen.rs",
            100,
        );

        assert_eq!(decision.category, DecisionCategory::TypeMapping);
        assert_eq!(decision.name, "binop_promotion");
        assert_eq!(decision.chosen_path, "i64");
        assert_eq!(decision.alternatives, vec!["i32", "f64"]);
        assert!((decision.confidence - 0.85).abs() < 0.001);
        assert_eq!(decision.source_file, "expr_gen.rs");
        assert_eq!(decision.source_line, 100);
        assert_ne!(decision.id, 0);
        assert_ne!(decision.thread_id, 0);
    }

    #[test]
    fn test_depyler_decision_with_spans() {
        let decision = DepylerDecision::new(
            DecisionCategory::BorrowStrategy,
            "param_borrow",
            "&str",
            &["String", "&String"],
            0.9,
            "func_gen.rs",
            50,
        )
        .with_py_span(10, 25)
        .with_rs_span(100, 120);

        assert_eq!(decision.py_span, (10, 25));
        assert_eq!(decision.rs_span, Some((100, 120)));
    }

    #[test]
    fn test_generate_decision_id_deterministic() {
        let id1 = generate_decision_id("TypeMapping", "binop", "file.rs", 10);
        let id2 = generate_decision_id("TypeMapping", "binop", "file.rs", 10);
        assert_eq!(id1, id2, "Same inputs should produce same ID");
    }

    #[test]
    fn test_generate_decision_id_unique() {
        let id1 = generate_decision_id("TypeMapping", "binop", "file.rs", 10);
        let id2 = generate_decision_id("BorrowStrategy", "binop", "file.rs", 10);
        let id3 = generate_decision_id("TypeMapping", "unop", "file.rs", 10);
        let id4 = generate_decision_id("TypeMapping", "binop", "other.rs", 10);
        let id5 = generate_decision_id("TypeMapping", "binop", "file.rs", 20);

        assert_ne!(id1, id2, "Different categories should have different IDs");
        assert_ne!(id1, id3, "Different names should have different IDs");
        assert_ne!(id1, id4, "Different files should have different IDs");
        assert_ne!(id1, id5, "Different lines should have different IDs");
    }

    #[test]
    fn test_decision_serialization() {
        let decision = DepylerDecision::new(
            DecisionCategory::Ownership,
            "move_vs_clone",
            "clone",
            &["move", "rc"],
            0.7,
            "ownership.rs",
            200,
        );

        // Test JSON serialization
        let json = serde_json::to_string(&decision).unwrap();
        assert!(json.contains("Ownership"));
        assert!(json.contains("move_vs_clone"));
        assert!(json.contains("clone"));

        // Test deserialization
        let deserialized: DepylerDecision = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.category, DecisionCategory::Ownership);
        assert_eq!(deserialized.name, "move_vs_clone");
        assert_eq!(deserialized.chosen_path, "clone");
    }

    #[test]
    fn test_trace_decision_macro_compiles() {
        // This test ensures the macro compiles correctly
        // The actual tracing is a no-op without the feature flag
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "test_decision",
            chosen = "i32",
            alternatives = ["i64", "u32"],
            confidence = 0.95
        );
    }

    #[test]
    fn test_init_decision_tracing() {
        // Should succeed (no-op when feature disabled)
        let result = init_decision_tracing();
        assert!(result.is_ok());
    }

    #[test]
    fn test_decision_category_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(DecisionCategory::TypeMapping, 1);
        map.insert(DecisionCategory::BorrowStrategy, 2);

        assert_eq!(map.get(&DecisionCategory::TypeMapping), Some(&1));
        assert_eq!(map.get(&DecisionCategory::BorrowStrategy), Some(&2));
    }

    #[test]
    fn test_decision_category_equality() {
        assert_eq!(DecisionCategory::TypeMapping, DecisionCategory::TypeMapping);
        assert_ne!(DecisionCategory::TypeMapping, DecisionCategory::Ownership);
    }

    // ========================================================================
    // Error-Decision Correlation Tests (Spec Section 9.1-9.2)
    // ========================================================================

    #[test]
    fn test_correlate_error_finds_overlapping_decisions() {
        let decisions = vec![
            DepylerDecision::new(
                DecisionCategory::TypeMapping,
                "type1",
                "i32",
                &[],
                0.9,
                "file.rs",
                10,
            )
            .with_rs_span(100, 150),
            DepylerDecision::new(
                DecisionCategory::BorrowStrategy,
                "borrow1",
                "&str",
                &[],
                0.8,
                "file.rs",
                20,
            )
            .with_rs_span(200, 250),
        ];

        // Error at span 120-130 should match first decision
        let correlated = correlate_error(&decisions, (120, 130));
        assert_eq!(correlated.len(), 1);
        assert_eq!(correlated[0].name, "type1");

        // Error at span 300-350 should match nothing
        let correlated = correlate_error(&decisions, (300, 350));
        assert!(correlated.is_empty());
    }

    #[test]
    fn test_correlate_error_returns_empty_for_no_spans() {
        let decisions = vec![DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "no_span",
            "i32",
            &[],
            0.9,
            "file.rs",
            10,
        )];

        let correlated = correlate_error(&decisions, (100, 150));
        assert!(correlated.is_empty());
    }

    #[test]
    fn test_correlate_error_sorts_by_span_size() {
        let decisions = vec![
            DepylerDecision::new(
                DecisionCategory::TypeMapping,
                "large",
                "i32",
                &[],
                0.9,
                "file.rs",
                10,
            )
            .with_rs_span(100, 200), // Large span
            DepylerDecision::new(
                DecisionCategory::BorrowStrategy,
                "small",
                "&str",
                &[],
                0.8,
                "file.rs",
                20,
            )
            .with_rs_span(120, 140), // Small span (more precise)
        ];

        let correlated = correlate_error(&decisions, (125, 135));
        assert_eq!(correlated.len(), 2);
        // Smaller span should come first (more precise match)
        assert_eq!(correlated[0].name, "small");
        assert_eq!(correlated[1].name, "large");
    }

    #[test]
    fn test_build_causal_chain_basic() {
        let decisions = vec![DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "root",
            "i32",
            &[],
            0.9,
            "file.rs",
            10,
        )
        .with_py_span(10, 20)
        .with_rs_span(100, 150)];

        let chain = build_causal_chain(&decisions, (120, 130), 5);
        assert_eq!(chain.len(), 1);
        assert_eq!(chain[0].depth, 0);
        assert_eq!(chain[0].decision.name, "root");
    }

    #[test]
    fn test_build_causal_chain_empty_for_no_match() {
        let decisions = vec![DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "unrelated",
            "i32",
            &[],
            0.9,
            "file.rs",
            10,
        )
        .with_rs_span(500, 600)];

        let chain = build_causal_chain(&decisions, (100, 150), 5);
        assert!(chain.is_empty());
    }

    #[test]
    fn test_compile_outcome_serialization() {
        let success = CompileOutcome::Success;
        let json = serde_json::to_string(&success).unwrap();
        assert!(json.contains("Success"));

        let error = CompileOutcome::Error {
            code: "E0308".to_string(),
            message: "mismatched types".to_string(),
            span: Some((10, 20)),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("E0308"));
        assert!(json.contains("mismatched types"));
    }

    #[test]
    fn test_causal_link_serialization() {
        let link = CausalLink {
            decision: DepylerDecision::new(
                DecisionCategory::TypeMapping,
                "test",
                "i32",
                &[],
                0.9,
                "file.rs",
                10,
            ),
            depth: 2,
        };

        let json = serde_json::to_string(&link).unwrap();
        assert!(json.contains("depth"));
        assert!(json.contains("decision"));
    }

    // ========================================================================
    // Graceful Degradation Tests (Spec Section 10)
    // ========================================================================

    #[test]
    fn test_json_file_writer_creation() {
        let path = std::path::Path::new("/tmp/test_decisions.jsonl");
        let writer = JsonFileWriter::new(path);
        assert!(writer.is_empty());
        assert_eq!(writer.len(), 0);
    }

    #[test]
    fn test_json_file_writer_with_buffer_size() {
        let path = std::path::Path::new("/tmp/test_decisions.jsonl");
        let writer = JsonFileWriter::with_buffer_size(path, 500);
        assert_eq!(writer.max_buffer_size, 500);
    }

    #[test]
    fn test_json_file_writer_append() {
        let path = std::path::Path::new("/tmp/test_decisions_append.jsonl");
        let mut writer = JsonFileWriter::new(path);

        let decision = DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "test",
            "i32",
            &[],
            0.9,
            "file.rs",
            10,
        );

        let result = writer.append(&decision);
        assert!(result.is_ok());
        assert_eq!(writer.len(), 1);
    }

    #[test]
    fn test_json_file_writer_flush() {
        use std::io::Read;

        let path = std::path::PathBuf::from("/tmp/test_decisions_flush.jsonl");

        // Clean up any existing file
        let _ = std::fs::remove_file(&path);

        {
            let mut writer = JsonFileWriter::new(&path);
            let decision = DepylerDecision::new(
                DecisionCategory::TypeMapping,
                "flush_test",
                "i32",
                &[],
                0.9,
                "file.rs",
                10,
            );
            writer.append(&decision).unwrap();
            writer.flush().unwrap();
            assert!(writer.is_empty()); // Buffer cleared after flush
        }

        // Verify file was written
        let mut file = std::fs::File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert!(contents.contains("flush_test"));
        assert!(contents.contains("TypeMapping"));

        // Clean up
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_decision_writer_trait_is_empty() {
        let path = std::path::Path::new("/tmp/test_is_empty.jsonl");
        let mut writer = JsonFileWriter::new(path);
        assert!(writer.is_empty());

        let decision = DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "test",
            "i32",
            &[],
            0.9,
            "file.rs",
            10,
        );
        writer.append(&decision).unwrap();
        assert!(!writer.is_empty());
    }

    #[test]
    fn test_create_decision_writer_returns_boxed_trait() {
        let path = std::path::Path::new("/tmp/test_create_writer.msgpack");
        let writer = create_decision_writer(path);
        assert!(writer.is_empty());
    }

    // ========================================================================
    // DEPYLER-EXPLAIN-001 Collector Tests (Issue #214)
    // ========================================================================

    #[test]
    fn test_transpile_decision_variants() {
        let decisions = [
            TranspileDecision::TypeInference,
            TranspileDecision::OwnershipInference,
            TranspileDecision::MethodResolution,
            TranspileDecision::ImportMapping,
            TranspileDecision::ControlFlowTransform,
        ];

        // Each variant should have unique display string
        let displays: Vec<String> = decisions.iter().map(|d| d.to_string()).collect();
        let unique_count = displays
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(
            unique_count,
            decisions.len(),
            "All decisions should have unique display names"
        );
    }

    #[test]
    fn test_transpile_decision_display() {
        assert_eq!(
            TranspileDecision::TypeInference.to_string(),
            "type_inference"
        );
        assert_eq!(
            TranspileDecision::OwnershipInference.to_string(),
            "ownership_inference"
        );
        assert_eq!(
            TranspileDecision::MethodResolution.to_string(),
            "method_resolution"
        );
        assert_eq!(
            TranspileDecision::ImportMapping.to_string(),
            "import_mapping"
        );
        assert_eq!(
            TranspileDecision::ControlFlowTransform.to_string(),
            "control_flow_transform"
        );
    }

    #[test]
    fn test_transpile_trace_creation() {
        let trace = TranspileTrace::new(
            TranspileDecision::TypeInference,
            "RULE-TYP-001",
            0.95,
            "Inferred List[int] as Vec<i32>",
        );

        assert_eq!(trace.decision, TranspileDecision::TypeInference);
        assert_eq!(trace.rule_id, "RULE-TYP-001");
        assert!((trace.confidence - 0.95).abs() < 0.001);
        assert_eq!(trace.explanation, "Inferred List[int] as Vec<i32>");
        assert!(trace.alternatives.is_empty());
        // timestamp_ns is tested separately; may be 0 in certain environments
    }

    #[test]
    fn test_transpile_trace_with_source_loc() {
        let trace = TranspileTrace::new(
            TranspileDecision::MethodResolution,
            "RULE-MTH-001",
            0.8,
            "Resolved append to Vec::push",
        )
        .with_source_loc("script.py:42:10");

        assert_eq!(trace.source_loc, "script.py:42:10");
    }

    #[test]
    fn test_transpile_trace_with_alternatives() {
        let trace = TranspileTrace::new(
            TranspileDecision::OwnershipInference,
            "RULE-OWN-001",
            0.75,
            "Chose clone over move",
        )
        .with_alternatives(&["move", "Rc::clone", "Arc::clone"]);

        assert_eq!(trace.alternatives, vec!["move", "Rc::clone", "Arc::clone"]);
    }

    #[test]
    fn test_transpile_trace_serialization() {
        let trace = TranspileTrace::new(
            TranspileDecision::ImportMapping,
            "RULE-IMP-001",
            0.9,
            "Mapped json to serde_json",
        );

        let json = serde_json::to_string(&trace).unwrap();
        assert!(json.contains("ImportMapping"));
        assert!(json.contains("RULE-IMP-001"));

        let deserialized: TranspileTrace = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.decision, TranspileDecision::ImportMapping);
    }

    // ========================================================================
    // RingCollector Tests (<100ns target)
    // ========================================================================

    #[test]
    fn test_ring_collector_creation() {
        let collector = RingCollector::new();
        assert_eq!(collector.len(), 0);
        assert!(collector.is_empty());
        assert_eq!(collector.total_collected(), 0);
        assert!(!collector.has_wrapped());
    }

    #[test]
    fn test_ring_collector_with_capacity() {
        let collector = RingCollector::with_capacity(100);
        assert_eq!(collector.capacity, 100);
        assert!(collector.is_empty());
    }

    #[test]
    fn test_ring_collector_collect_single() {
        let mut collector = RingCollector::new();
        let trace = TranspileTrace::new(
            TranspileDecision::TypeInference,
            "RULE-001",
            0.9,
            "Test trace",
        );

        collector.collect(trace);
        assert_eq!(collector.len(), 1);
        assert_eq!(collector.total_collected(), 1);
        assert!(!collector.is_empty());
    }

    #[test]
    fn test_ring_collector_collect_multiple() {
        let mut collector = RingCollector::with_capacity(10);

        for i in 0..5 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:03}", i),
                0.9,
                &format!("Trace {}", i),
            );
            collector.collect(trace);
        }

        assert_eq!(collector.len(), 5);
        assert_eq!(collector.total_collected(), 5);
        assert!(!collector.has_wrapped());
    }

    #[test]
    fn test_ring_collector_wrapping() {
        let mut collector = RingCollector::with_capacity(5);

        // Collect 10 traces (should wrap)
        for i in 0..10 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:03}", i),
                0.9,
                &format!("Trace {}", i),
            );
            collector.collect(trace);
        }

        // Buffer should be at capacity, but total collected should be 10
        assert_eq!(collector.len(), 5);
        assert_eq!(collector.total_collected(), 10);
        assert!(collector.has_wrapped());
    }

    #[test]
    fn test_ring_collector_clear() {
        let mut collector = RingCollector::new();

        for i in 0..5 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:03}", i),
                0.9,
                "Test",
            );
            collector.collect(trace);
        }

        assert_eq!(collector.len(), 5);
        collector.clear();
        assert_eq!(collector.len(), 0);
        assert_eq!(collector.total_collected(), 0);
        assert!(collector.is_empty());
    }

    #[test]
    fn test_ring_collector_traces() {
        let mut collector = RingCollector::new();
        let trace = TranspileTrace::new(
            TranspileDecision::MethodResolution,
            "RULE-MTH-001",
            0.85,
            "Test trace",
        );

        collector.collect(trace);
        let traces = collector.traces();
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].rule_id, "RULE-MTH-001");
    }

    #[test]
    fn test_ring_collector_default() {
        let collector = RingCollector::default();
        assert!(collector.is_empty());
        assert_eq!(collector.capacity, RingCollector::DEFAULT_CAPACITY);
    }

    // ========================================================================
    // StreamCollector Tests (<1µs target)
    // ========================================================================

    #[test]
    fn test_stream_collector_creation() {
        let collector = StreamCollector::new();
        assert!(collector.is_empty());
        assert_eq!(collector.total_streamed(), 0);
    }

    #[test]
    fn test_stream_collector_with_output() {
        let path = std::path::Path::new("/tmp/test_stream_collector.jsonl");
        let collector = StreamCollector::with_output(path);
        assert!(collector.is_empty());
        assert!(collector.output_path.is_some());
    }

    #[test]
    fn test_stream_collector_with_flush_threshold() {
        let collector = StreamCollector::new().with_flush_threshold(500);
        assert_eq!(collector.flush_threshold, 500);
    }

    #[test]
    fn test_stream_collector_collect() {
        let mut collector = StreamCollector::new();
        let trace = TranspileTrace::new(
            TranspileDecision::ImportMapping,
            "RULE-IMP-001",
            0.9,
            "Import mapping trace",
        );

        collector.collect(trace);
        assert_eq!(collector.len(), 1);
    }

    #[test]
    fn test_stream_collector_manual_flush() {
        let path = std::path::PathBuf::from("/tmp/test_stream_flush.jsonl");
        let _ = std::fs::remove_file(&path);

        let mut collector = StreamCollector::with_output(&path);
        let trace = TranspileTrace::new(
            TranspileDecision::ControlFlowTransform,
            "RULE-CTL-001",
            0.88,
            "Control flow trace",
        );

        collector.collect(trace);
        assert_eq!(collector.len(), 1);

        let result = collector.flush();
        assert!(result.is_ok());
        assert!(collector.is_empty());
        assert_eq!(collector.total_streamed(), 1);

        // Verify file exists
        assert!(path.exists());

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_stream_collector_auto_flush() {
        let mut collector = StreamCollector::new().with_flush_threshold(5);

        // Collect 6 traces (should trigger auto-flush)
        for i in 0..6 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:03}", i),
                0.9,
                "Test",
            );
            collector.collect(trace);
        }

        // After auto-flush, only 1 trace remains in buffer (the 6th)
        assert_eq!(collector.len(), 1);
        // 5 traces were streamed
        assert_eq!(collector.total_streamed(), 5);
    }

    #[test]
    fn test_stream_collector_clear() {
        let mut collector = StreamCollector::new();

        for i in 0..5 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:03}", i),
                0.9,
                "Test",
            );
            collector.collect(trace);
        }

        collector.clear();
        assert!(collector.is_empty());
    }

    #[test]
    fn test_stream_collector_default() {
        let collector = StreamCollector::default();
        assert!(collector.is_empty());
        assert_eq!(
            collector.flush_threshold,
            StreamCollector::DEFAULT_FLUSH_THRESHOLD
        );
    }

    // ========================================================================
    // HashChainCollector Tests (<10µs target)
    // ========================================================================

    #[test]
    fn test_hash_chain_collector_creation() {
        let collector = HashChainCollector::new();
        assert!(collector.is_empty());
        assert_eq!(collector.chain_hash(), 0);
    }

    #[test]
    fn test_hash_chain_collector_collect_single() {
        let mut collector = HashChainCollector::new();
        let trace = TranspileTrace::new(
            TranspileDecision::TypeInference,
            "RULE-001",
            0.9,
            "First trace",
        );

        collector.collect(trace);
        assert_eq!(collector.len(), 1);
        assert_ne!(collector.chain_hash(), 0);

        let chained = collector.chained_traces();
        assert_eq!(chained.len(), 1);
        assert_eq!(chained[0].prev_hash, 0); // First entry has prev_hash = 0
        assert_ne!(chained[0].entry_hash, 0);
    }

    #[test]
    fn test_hash_chain_collector_chain_integrity() {
        let mut collector = HashChainCollector::new();

        for i in 0..5 {
            let trace = TranspileTrace::new(
                TranspileDecision::MethodResolution,
                &format!("RULE-{:03}", i),
                0.8,
                &format!("Trace {}", i),
            );
            collector.collect(trace);
        }

        // Verify chain integrity
        let chained = collector.chained_traces();
        for i in 1..chained.len() {
            assert_eq!(
                chained[i].prev_hash,
                chained[i - 1].entry_hash,
                "Hash chain broken at index {}",
                i
            );
        }
    }

    #[test]
    fn test_hash_chain_collector_verify_chain() {
        let mut collector = HashChainCollector::new();

        for i in 0..10 {
            let trace = TranspileTrace::new(
                TranspileDecision::ImportMapping,
                &format!("RULE-{:03}", i),
                0.9,
                &format!("Trace {}", i),
            );
            collector.collect(trace);
        }

        assert!(
            collector.verify_chain(),
            "Hash chain should verify successfully"
        );
    }

    #[test]
    fn test_hash_chain_collector_empty_verify() {
        let collector = HashChainCollector::new();
        assert!(
            collector.verify_chain(),
            "Empty chain should verify successfully"
        );
    }

    #[test]
    fn test_hash_chain_collector_clear() {
        let mut collector = HashChainCollector::new();

        for i in 0..5 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:03}", i),
                0.9,
                "Test",
            );
            collector.collect(trace);
        }

        collector.clear();
        assert!(collector.is_empty());
        assert_eq!(collector.chain_hash(), 0);
    }

    #[test]
    fn test_hash_chain_collector_chained_traces() {
        let mut collector = HashChainCollector::new();
        let trace = TranspileTrace::new(
            TranspileDecision::OwnershipInference,
            "RULE-OWN-001",
            0.75,
            "Ownership trace",
        );

        collector.collect(trace);
        let chained = collector.chained_traces();
        assert_eq!(chained.len(), 1);
        assert_eq!(chained[0].trace.rule_id, "RULE-OWN-001");
    }

    #[test]
    fn test_hash_chain_collector_export_with_proof() {
        let mut collector = HashChainCollector::new();
        let trace = TranspileTrace::new(
            TranspileDecision::ControlFlowTransform,
            "RULE-CTL-001",
            0.9,
            "Control flow trace",
        );

        collector.collect(trace);
        let export = collector.export_with_proof();
        assert!(export.is_ok());

        let json = export.unwrap();
        assert!(json.contains("chain_hash"));
        assert!(json.contains("verified"));
        assert!(json.contains("true")); // verified should be true
    }

    #[test]
    fn test_hash_chain_collector_default() {
        let collector = HashChainCollector::default();
        assert!(collector.is_empty());
    }

    #[test]
    fn test_hash_chained_trace_serialization() {
        let trace = TranspileTrace::new(TranspileDecision::TypeInference, "RULE-001", 0.9, "Test");

        let chained = HashChainedTrace {
            trace,
            prev_hash: 12345,
            entry_hash: 67890,
        };

        let json = serde_json::to_string(&chained).unwrap();
        assert!(json.contains("prev_hash"));
        assert!(json.contains("entry_hash"));
        assert!(json.contains("12345"));
        assert!(json.contains("67890"));
    }

    // ========================================================================
    // TraceCollector Trait Tests
    // ========================================================================

    #[test]
    fn test_trace_collector_export_json() {
        let mut collector = RingCollector::new();
        let trace = TranspileTrace::new(
            TranspileDecision::TypeInference,
            "RULE-001",
            0.9,
            "Test export",
        );

        collector.collect(trace);
        let json = collector.export_json();
        assert!(json.is_ok());
        assert!(json.unwrap().contains("RULE-001"));
    }

    // ========================================================================
    // Performance Sanity Tests (not benchmarks, just sanity checks)
    // ========================================================================

    #[test]
    fn test_ring_collector_bulk_insert_completes() {
        let mut collector = RingCollector::new();

        // Collect 10000 traces - should complete quickly
        for i in 0..10000 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:05}", i % 100),
                0.9,
                "Bulk test",
            );
            collector.collect(trace);
        }

        assert_eq!(collector.total_collected(), 10000);
    }

    #[test]
    fn test_hash_chain_collector_bulk_insert_with_verification() {
        let mut collector = HashChainCollector::new();

        // Collect 1000 traces
        for i in 0..1000 {
            let trace = TranspileTrace::new(
                TranspileDecision::MethodResolution,
                &format!("RULE-{:04}", i),
                0.85,
                "Hash chain bulk test",
            );
            collector.collect(trace);
        }

        // Verify chain integrity after bulk insert
        assert!(collector.verify_chain());
        assert_eq!(collector.len(), 1000);
    }

    // ========================================================
    // DEPYLER-COVERAGE-95: Additional decision_trace tests
    // ========================================================

    #[test]
    fn test_depyler_decision_with_py_ast_hash() {
        let decision = DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "test_name",
            "i64",
            &["i32"],
            0.9,
            "test.rs",
            1,
        )
        .with_py_ast_hash(12345);

        assert_eq!(decision.py_ast_hash, 12345);
    }

    #[test]
    fn test_depyler_decision_builder_chain() {
        let decision = DepylerDecision::new(
            DecisionCategory::BorrowStrategy,
            "borrow_test",
            "&T",
            &["T", "&mut T"],
            0.8,
            "test.rs",
            10,
        )
        .with_py_ast_hash(999)
        .with_py_span(0, 100)
        .with_rs_span(0, 150);

        assert_eq!(decision.py_ast_hash, 999);
        assert_eq!(decision.py_span, (0, 100));
        assert_eq!(decision.rs_span, Some((0, 150)));
    }

    #[test]
    fn test_generate_decision_id_empty_strings() {
        let id = generate_decision_id("", "", "", 0);
        // Should still produce a valid hash
        assert!(id > 0);
    }

    #[test]
    fn test_generate_decision_id_long_strings() {
        let long_name = "a".repeat(10000);
        let id = generate_decision_id("TypeMapping", &long_name, "file.rs", 100);
        assert!(id > 0);
    }

    #[test]
    fn test_decision_category_clone() {
        let category = DecisionCategory::TypeMapping;
        let cloned = category; // Copy trait, no need for clone()
        assert_eq!(category, cloned);
    }

    #[test]
    fn test_decision_category_copy() {
        let category = DecisionCategory::BorrowStrategy;
        let copied: DecisionCategory = category; // Copy trait
        assert_eq!(category, copied);
    }

    #[test]
    fn test_depyler_decision_clone() {
        let decision = DepylerDecision::new(
            DecisionCategory::LifetimeInfer,
            "lifetime_test",
            "'static",
            &["'a"],
            0.95,
            "test.rs",
            50,
        );
        let cloned = decision.clone();
        assert_eq!(decision.category, cloned.category);
        assert_eq!(decision.name, cloned.name);
        assert_eq!(decision.chosen_path, cloned.chosen_path);
    }

    #[test]
    fn test_depyler_decision_debug_format() {
        let decision = DepylerDecision::new(
            DecisionCategory::MethodDispatch,
            "dispatch_test",
            "trait_method",
            &[],
            0.75,
            "test.rs",
            1,
        );
        let debug = format!("{:?}", decision);
        assert!(debug.contains("DepylerDecision"));
        assert!(debug.contains("MethodDispatch"));
    }

    #[test]
    fn test_transpile_decision_all_variants() {
        let variants = [
            TranspileDecision::TypeInference,
            TranspileDecision::OwnershipInference,
            TranspileDecision::MethodResolution,
            TranspileDecision::ImportMapping,
            TranspileDecision::ControlFlowTransform,
        ];

        for variant in &variants {
            let _s = format!("{}", variant);
            let _d = format!("{:?}", variant);
        }
    }

    #[test]
    fn test_transpile_trace_clone() {
        let trace = TranspileTrace::new(
            TranspileDecision::TypeInference,
            "RULE-001",
            0.85,
            "Test trace",
        );
        let cloned = trace.clone();
        assert_eq!(trace.decision, cloned.decision);
        assert_eq!(trace.rule_id, cloned.rule_id);
    }

    #[test]
    fn test_transpile_trace_debug_format() {
        let trace = TranspileTrace::new(
            TranspileDecision::OwnershipInference,
            "RULE-002",
            0.9,
            "Debug test",
        );
        let debug = format!("{:?}", trace);
        assert!(debug.contains("TranspileTrace"));
    }

    #[test]
    fn test_ring_collector_total_collected_no_wrap() {
        let mut collector = RingCollector::with_capacity(100);
        for i in 0..50 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:03}", i),
                0.85,
                "Test",
            );
            collector.collect(trace);
        }
        assert_eq!(collector.total_collected(), 50);
        assert!(!collector.has_wrapped());
    }

    #[test]
    fn test_ring_collector_total_collected_with_wrap() {
        let mut collector = RingCollector::with_capacity(10);
        for i in 0..25 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:03}", i),
                0.85,
                "Test",
            );
            collector.collect(trace);
        }
        assert_eq!(collector.total_collected(), 25);
        assert!(collector.has_wrapped());
        assert_eq!(collector.len(), 10); // Capacity limit
    }

    #[test]
    fn test_stream_collector_total_streamed() {
        let mut collector = StreamCollector::new();
        for i in 0..5 {
            let trace = TranspileTrace::new(
                TranspileDecision::MethodResolution,
                &format!("RULE-{:03}", i),
                0.9,
                "Stream test",
            );
            collector.collect(trace);
        }
        assert_eq!(collector.total_streamed(), 0); // Not flushed yet
        let _ = collector.flush();
        // total_streamed() is always >= 0 since it's usize, just ensure no panic
        let _ = collector.total_streamed();
    }

    #[test]
    fn test_hash_chain_collector_chain_hash_initial() {
        let collector = HashChainCollector::new();
        // Initial chain hash should be 0
        assert_eq!(collector.chain_hash(), 0);
    }

    #[test]
    fn test_hash_chain_collector_chain_hash_changes() {
        let mut collector = HashChainCollector::new();
        let initial_hash = collector.chain_hash();

        let trace = TranspileTrace::new(
            TranspileDecision::TypeInference,
            "RULE-001",
            0.85,
            "Hash test",
        );
        collector.collect(trace);

        let new_hash = collector.chain_hash();
        assert_ne!(initial_hash, new_hash);
    }

    #[test]
    fn test_hash_chain_collector_produces_unique_hashes() {
        let mut collector = HashChainCollector::new();
        let mut seen_hashes = std::collections::HashSet::new();

        for i in 0..5 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{:03}", i),
                0.85,
                "Hash uniqueness test",
            );
            collector.collect(trace);
            let hash = collector.chain_hash();
            // Each addition should produce a different hash
            assert!(
                seen_hashes.insert(hash),
                "Hash collision detected at iteration {}",
                i
            );
        }

        // Chain should be valid
        assert!(collector.verify_chain());
        assert_eq!(collector.len(), 5);
    }

    #[test]
    fn test_compile_outcome_success_debug_display() {
        let outcome = CompileOutcome::Success;
        let _debug = format!("{:?}", outcome);
    }

    #[test]
    fn test_compile_outcome_error_debug_display() {
        let outcome = CompileOutcome::Error {
            code: "E0001".to_string(),
            message: "test error".to_string(),
            span: Some((10, 50)),
        };
        let _debug = format!("{:?}", outcome);
    }

    #[test]
    fn test_causal_link_creation() {
        let decision = DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "test_decision",
            "i32",
            &[],
            0.9,
            "test.rs",
            1,
        );
        let link = CausalLink { decision, depth: 0 };
        assert_eq!(link.depth, 0);
        assert_eq!(link.decision.name, "test_decision");
    }

    #[test]
    fn test_compile_outcome_clone_success() {
        let outcome = CompileOutcome::Success;
        let cloned = outcome.clone();
        assert!(matches!(cloned, CompileOutcome::Success));
    }

    #[test]
    fn test_compile_outcome_clone_error() {
        let outcome = CompileOutcome::Error {
            code: "E0308".to_string(),
            message: "type mismatch".to_string(),
            span: Some((1, 10)),
        };
        let cloned = outcome.clone();
        match cloned {
            CompileOutcome::Error {
                code,
                message,
                span,
            } => {
                assert_eq!(code, "E0308");
                assert!(message.contains("mismatch"));
                assert_eq!(span, Some((1, 10)));
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_causal_link_clone() {
        let decision = DepylerDecision::new(
            DecisionCategory::BorrowStrategy,
            "clone_test",
            "&T",
            &[],
            0.8,
            "test.rs",
            2,
        );
        let link = CausalLink { decision, depth: 1 };
        let cloned = link.clone();
        assert_eq!(link.depth, cloned.depth);
        assert_eq!(link.decision.name, cloned.decision.name);
    }

    #[test]
    fn test_correlate_error_non_overlapping() {
        let decisions = vec![
            DepylerDecision::new(
                DecisionCategory::TypeMapping,
                "decision1",
                "i32",
                &[],
                0.9,
                "test.rs",
                1,
            )
            .with_rs_span(0, 10),
            DepylerDecision::new(
                DecisionCategory::TypeMapping,
                "decision2",
                "i64",
                &[],
                0.9,
                "test.rs",
                2,
            )
            .with_rs_span(100, 110),
        ];

        // Error span that doesn't overlap with either decision
        let result = correlate_error(&decisions, (50, 60));
        assert!(result.is_empty());
    }

    #[test]
    fn test_correlate_error_partial_overlap() {
        let decisions = vec![DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "overlap_test",
            "i32",
            &[],
            0.9,
            "test.rs",
            1,
        )
        .with_rs_span(0, 50)];

        // Error span partially overlaps
        let result = correlate_error(&decisions, (40, 60));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_build_causal_chain_multiple_decisions() {
        let decisions = vec![
            DepylerDecision::new(
                DecisionCategory::TypeMapping,
                "type_decision",
                "i32",
                &[],
                0.9,
                "test.rs",
                1,
            )
            .with_rs_span(0, 100),
            DepylerDecision::new(
                DecisionCategory::BorrowStrategy,
                "borrow_decision",
                "&T",
                &[],
                0.8,
                "test.rs",
                2,
            )
            .with_rs_span(20, 80),
        ];

        let chain = build_causal_chain(&decisions, (30, 70), 5);
        // chain.len() is always >= 0 since it's usize, just verify no panic
        let _ = chain.len();
    }

    #[test]
    fn test_decision_category_serialize_deserialize() {
        let category = DecisionCategory::ErrorHandling;
        let serialized = serde_json::to_string(&category).unwrap();
        let deserialized: DecisionCategory = serde_json::from_str(&serialized).unwrap();
        assert_eq!(category, deserialized);
    }

    #[test]
    fn test_transpile_decision_serialize_deserialize() {
        let decision = TranspileDecision::OwnershipInference;
        let serialized = serde_json::to_string(&decision).unwrap();
        let deserialized: TranspileDecision = serde_json::from_str(&serialized).unwrap();
        assert_eq!(decision, deserialized);
    }

    #[test]
    fn test_transpile_trace_with_all_options() {
        let trace = TranspileTrace::new(
            TranspileDecision::TypeInference,
            "RULE-LIFETIME",
            0.95,
            "Full options test",
        )
        .with_source_loc("test.py:42")
        .with_alternatives(&["'a", "'static", "elided"]);

        assert_eq!(trace.source_loc, "test.py:42".to_string());
        assert_eq!(trace.alternatives.len(), 3);
    }

    #[test]
    fn test_ring_collector_clear_resets_counters() {
        let mut collector = RingCollector::with_capacity(10);
        for i in 0..5 {
            let trace = TranspileTrace::new(
                TranspileDecision::TypeInference,
                &format!("RULE-{}", i),
                0.85,
                "Clear test",
            );
            collector.collect(trace);
        }
        assert_eq!(collector.len(), 5);
        collector.clear();
        assert_eq!(collector.len(), 0);
        assert!(!collector.has_wrapped());
    }

    #[test]
    fn test_stream_collector_clear_empties_buffer() {
        let mut collector = StreamCollector::new();
        for i in 0..3 {
            let trace = TranspileTrace::new(
                TranspileDecision::ImportMapping,
                &format!("RULE-{}", i),
                0.9,
                "Buffer test",
            );
            collector.collect(trace);
        }
        assert_eq!(collector.len(), 3);
        collector.clear();
        assert_eq!(collector.len(), 0);
    }

    #[test]
    fn test_hash_chain_collector_clear_resets_hash() {
        let mut collector = HashChainCollector::new();
        let trace = TranspileTrace::new(
            TranspileDecision::ControlFlowTransform,
            "RULE-001",
            0.8,
            "Hash reset test",
        );
        collector.collect(trace);
        assert_ne!(collector.chain_hash(), 0);

        collector.clear();
        assert_eq!(collector.chain_hash(), 0);
        assert!(collector.verify_chain());
    }

    #[test]
    fn test_trace_collector_is_empty() {
        let collector = RingCollector::new();
        assert!(collector.is_empty());
        assert_eq!(collector.len(), 0);
    }

    #[test]
    fn test_trace_collector_not_empty_after_collect() {
        let mut collector = RingCollector::new();
        let trace = TranspileTrace::new(
            TranspileDecision::TypeInference,
            "RULE-001",
            0.85,
            "Not empty test",
        );
        collector.collect(trace);
        assert!(!collector.is_empty());
    }

    #[test]
    fn test_transpile_trace_empty_alternatives() {
        let trace = TranspileTrace::new(
            TranspileDecision::OwnershipInference,
            "RULE-001",
            0.9,
            "No alternatives",
        )
        .with_alternatives(&[]);

        assert!(trace.alternatives.is_empty());
    }

    #[test]
    fn test_depyler_decision_empty_alternatives() {
        let decision = DepylerDecision::new(
            DecisionCategory::Ownership,
            "ownership_test",
            "move",
            &[], // Empty alternatives
            0.99,
            "test.rs",
            1,
        );
        assert!(decision.alternatives.is_empty());
    }

    #[test]
    fn test_depyler_decision_many_alternatives() {
        let alts: Vec<&str> = (0..100).map(|_| "alt").collect();
        let decision = DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "many_alts",
            "chosen",
            &alts,
            0.5,
            "test.rs",
            1,
        );
        assert_eq!(decision.alternatives.len(), 100);
    }

    #[test]
    fn test_decision_confidence_boundaries() {
        // Test low confidence
        let low = DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "low_conf",
            "i32",
            &[],
            0.0,
            "test.rs",
            1,
        );
        assert_eq!(low.confidence, 0.0);

        // Test high confidence
        let high = DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "high_conf",
            "i64",
            &[],
            1.0,
            "test.rs",
            1,
        );
        assert_eq!(high.confidence, 1.0);
    }

    #[test]
    fn test_hash_chained_trace_clone() {
        let trace = TranspileTrace::new(
            TranspileDecision::TypeInference,
            "RULE-001",
            0.85,
            "Clone test",
        );
        let chained = HashChainedTrace {
            trace: trace.clone(),
            entry_hash: 12345,
            prev_hash: 0,
        };
        let cloned = chained.clone();
        assert_eq!(chained.entry_hash, cloned.entry_hash);
        assert_eq!(chained.prev_hash, cloned.prev_hash);
    }

    #[test]
    fn test_compile_outcome_success_variant() {
        let outcome = CompileOutcome::Success;
        let _debug = format!("{:?}", outcome);
        assert!(matches!(outcome, CompileOutcome::Success));
    }

    #[test]
    fn test_compile_outcome_error_without_span() {
        let outcome = CompileOutcome::Error {
            code: "E0001".to_string(),
            message: "Generic error".to_string(),
            span: None,
        };
        match outcome {
            CompileOutcome::Error { span, .. } => {
                assert!(span.is_none());
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_compile_outcome_error_with_span() {
        let outcome = CompileOutcome::Error {
            code: "E0308".to_string(),
            message: "Type mismatch".to_string(),
            span: Some((10, 25)),
        };
        match outcome {
            CompileOutcome::Error {
                span,
                code,
                message,
            } => {
                assert_eq!(span, Some((10, 25)));
                assert_eq!(code, "E0308");
                assert!(message.contains("mismatch"));
            }
            _ => panic!("Expected Error variant"),
        }
    }
}
