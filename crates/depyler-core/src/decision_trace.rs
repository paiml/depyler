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
    #[cfg(not(feature = "wasm"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0)
    }
    #[cfg(feature = "wasm")]
    {
        0 // WASM doesn't have reliable high-resolution time
    }
}

/// Get current thread ID as u64
fn current_thread_id() -> u64 {
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    hasher.finish()
}

/// Memory-mapped decision writer for zero-blocking trace output
///
/// Uses a circular buffer in a memory-mapped file to capture decisions
/// without blocking the transpilation pipeline.
#[cfg(feature = "decision-tracing")]
pub struct MmapDecisionWriter {
    mmap: memmap2::MmapMut,
    offset: usize,
    capacity: usize,
    decisions: Vec<DepylerDecision>,
}

#[cfg(feature = "decision-tracing")]
impl MmapDecisionWriter {
    /// Default buffer size: 10MB (approximately 78,000 decisions)
    pub const DEFAULT_SIZE: usize = 10 * 1024 * 1024;

    /// Create a new memory-mapped decision writer
    ///
    /// # Arguments
    /// * `path` - Path to the trace file (e.g., "/tmp/depyler_decisions.msgpack")
    /// * `size` - Buffer size in bytes (default: 10MB)
    pub fn new(path: &std::path::Path, size: usize) -> Result<Self, String> {
        use std::fs::OpenOptions;

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory: {}", e))?;
        }

        // Create and pre-allocate file
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| format!("Failed to create file: {}", e))?;

        file.set_len(size as u64)
            .map_err(|e| format!("Failed to set file size: {}", e))?;

        // Memory-map the file
        let mmap = unsafe {
            memmap2::MmapMut::map_mut(&file)
                .map_err(|e| format!("Failed to create memory map: {}", e))?
        };

        Ok(Self {
            mmap,
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

    /// Flush buffered decisions to the memory-mapped file
    pub fn flush(&mut self) -> Result<(), String> {
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

        // Write to memory-mapped region (circular overwrite)
        self.mmap[0..packed.len()].copy_from_slice(&packed);
        self.offset = packed.len();

        // Flush mmap to disk
        self.mmap
            .flush()
            .map_err(|e| format!("Failed to flush mmap: {}", e))?;

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
        let unique_count = displays.iter().collect::<std::collections::HashSet<_>>().len();
        assert_eq!(unique_count, categories.len(), "All categories should have unique display names");
    }

    #[test]
    fn test_decision_category_display() {
        assert_eq!(DecisionCategory::TypeMapping.to_string(), "type_mapping");
        assert_eq!(DecisionCategory::BorrowStrategy.to_string(), "borrow_strategy");
        assert_eq!(DecisionCategory::LifetimeInfer.to_string(), "lifetime_infer");
        assert_eq!(DecisionCategory::MethodDispatch.to_string(), "method_dispatch");
        assert_eq!(DecisionCategory::ImportResolve.to_string(), "import_resolve");
        assert_eq!(DecisionCategory::ErrorHandling.to_string(), "error_handling");
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
}
