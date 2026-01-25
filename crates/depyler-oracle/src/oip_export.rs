// Allow future parquet feature flag (not yet in Cargo.toml)
#![allow(unexpected_cfgs)]

//! OIP CITL Export Module (DEPYLER-0636)
//!
//! Enables Depyler → OIP data flow for closed-loop training.
//! Complements the existing OIP → Depyler import in `github_corpus.rs`.
//!
//! # Features
//!
//! - `DepylerExport` format matching OIP's `citl::DepylerExport`
//! - `ErrorCodeClass` for GNN feature extraction
//! - Parquet batch export via alimentar
//!
//! # Example
//!
//! ```rust,ignore
//! use depyler_oracle::oip_export::{DepylerExport, ErrorCodeClass, export_corpus};
//!
//! let export = DepylerExport::new("E0308", "mismatched types", "example.py")
//!     .with_confidence(0.95)
//!     .with_oip_category("TypeErrors");
//!
//! let class = ErrorCodeClass::from_error_code("E0308"); // Type
//! ```

use crate::classifier::ErrorCategory;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

/// Error code class for GNN feature extraction (matches OIP citl.rs)
///
/// Provides categorical features for graph neural network input.
/// Maps rustc error codes to 5 high-level classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ErrorCodeClass {
    /// Type errors: E0308, E0412
    Type = 0,
    /// Borrow errors: E0502, E0503, E0505, E0382, E0507
    Borrow = 1,
    /// Name resolution: E0425, E0433
    Name = 2,
    /// Trait errors: E0277
    Trait = 3,
    /// All other errors
    #[default]
    Other = 4,
}

impl ErrorCodeClass {
    /// Get numeric value for feature encoding
    #[must_use]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Convert to one-hot encoded feature vector
    #[must_use]
    pub fn to_one_hot(&self) -> [f32; 5] {
        let mut features = [0.0; 5];
        features[*self as usize] = 1.0;
        features
    }

    /// Classify error code into class
    #[must_use]
    pub fn from_error_code(code: &str) -> Self {
        match code {
            // Type errors
            "E0308" | "E0606" | "E0061" => Self::Type,
            // Borrow errors
            "E0502" | "E0503" | "E0505" | "E0382" | "E0507" => Self::Borrow,
            // Name resolution
            "E0425" | "E0433" | "E0412" => Self::Name,
            // Trait errors
            "E0277" => Self::Trait,
            // Other
            _ => Self::Other,
        }
    }

    /// Get all error codes for this class
    #[must_use]
    pub fn error_codes(&self) -> &'static [&'static str] {
        match self {
            Self::Type => &["E0308", "E0606", "E0061"],
            Self::Borrow => &["E0502", "E0503", "E0505", "E0382", "E0507"],
            Self::Name => &["E0425", "E0433", "E0412"],
            Self::Trait => &["E0277"],
            Self::Other => &[],
        }
    }

    /// Human-readable name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Type => "Type",
            Self::Borrow => "Borrow",
            Self::Name => "Name",
            Self::Trait => "Trait",
            Self::Other => "Other",
        }
    }
}

/// Span information for diagnostic location
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpanInfo {
    pub line_start: u32,
    pub column_start: u32,
}

impl SpanInfo {
    #[must_use]
    pub fn new(line: u32, column: u32) -> Self {
        Self {
            line_start: line,
            column_start: column,
        }
    }
}

/// Suggestion information from compiler
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestionInfo {
    pub replacement: String,
    pub applicability: String,
}

impl SuggestionInfo {
    #[must_use]
    pub fn new(replacement: impl Into<String>, applicability: impl Into<String>) -> Self {
        Self {
            replacement: replacement.into(),
            applicability: applicability.into(),
        }
    }

    #[must_use]
    pub fn machine_applicable(replacement: impl Into<String>) -> Self {
        Self::new(replacement, "MachineApplicable")
    }
}

/// Depyler CITL export record (matches OIP citl::DepylerExport)
///
/// This is the format that OIP expects when importing Depyler error data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepylerExport {
    /// Source Python file that was transpiled
    pub source_file: String,
    /// Rustc error code (e.g., "E0308")
    pub error_code: Option<String>,
    /// Clippy lint name (e.g., "clippy::unwrap_used")
    pub clippy_lint: Option<String>,
    /// Diagnostic level: "error", "warning", "note"
    pub level: String,
    /// Error message text
    pub message: String,
    /// Pre-mapped OIP category (optional, OIP can derive from error_code)
    pub oip_category: Option<String>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Location in generated Rust code
    pub span: Option<SpanInfo>,
    /// Compiler suggestion if available
    pub suggestion: Option<SuggestionInfo>,
    /// Unix timestamp
    pub timestamp: i64,
    /// Depyler version
    pub depyler_version: String,
}

impl DepylerExport {
    /// Create a new export record
    #[must_use]
    pub fn new(
        error_code: impl Into<String>,
        message: impl Into<String>,
        source_file: impl Into<String>,
    ) -> Self {
        Self {
            source_file: source_file.into(),
            error_code: Some(error_code.into()),
            clippy_lint: None,
            level: "error".to_string(),
            message: message.into(),
            oip_category: None,
            confidence: 0.7, // Default confidence
            span: None,
            suggestion: None,
            timestamp: chrono::Utc::now().timestamp(),
            depyler_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Create from clippy lint
    #[must_use]
    pub fn from_clippy(
        lint: impl Into<String>,
        message: impl Into<String>,
        source_file: impl Into<String>,
    ) -> Self {
        Self {
            source_file: source_file.into(),
            error_code: None,
            clippy_lint: Some(lint.into()),
            level: "warning".to_string(),
            message: message.into(),
            oip_category: None,
            confidence: 0.7,
            span: None,
            suggestion: None,
            timestamp: chrono::Utc::now().timestamp(),
            depyler_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Set confidence score
    #[must_use]
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Set OIP category
    #[must_use]
    pub fn with_oip_category(mut self, category: impl Into<String>) -> Self {
        self.oip_category = Some(category.into());
        self
    }

    /// Set span location
    #[must_use]
    pub fn with_span(mut self, line: u32, column: u32) -> Self {
        self.span = Some(SpanInfo::new(line, column));
        self
    }

    /// Set suggestion
    #[must_use]
    pub fn with_suggestion(mut self, suggestion: SuggestionInfo) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// Set diagnostic level
    #[must_use]
    pub fn with_level(mut self, level: impl Into<String>) -> Self {
        self.level = level.into();
        self
    }

    /// Get error code class for GNN features
    #[must_use]
    pub fn error_code_class(&self) -> ErrorCodeClass {
        self.error_code
            .as_ref()
            .map(|c| ErrorCodeClass::from_error_code(c))
            .unwrap_or_default()
    }

    /// Map to OIP category based on error code
    #[must_use]
    pub fn derive_oip_category(&self) -> Option<&'static str> {
        self.error_code.as_ref().and_then(|code| {
            Some(match code.as_str() {
                "E0308" => "TypeErrors",
                "E0412" => "TypeAnnotationGaps",
                "E0502" | "E0503" | "E0505" => "OwnershipBorrow",
                "E0382" | "E0507" => "MemorySafety",
                "E0277" => "TraitBounds",
                "E0425" | "E0433" => "StdlibMapping",
                "E0599" | "E0615" => "ASTTransform",
                "E0614" => "OperatorPrecedence",
                "E0658" => "ConfigurationErrors",
                _ => return None,
            })
        })
    }

    /// Convert from internal ErrorCategory
    pub fn from_error_category(
        category: ErrorCategory,
        message: impl Into<String>,
        source_file: impl Into<String>,
    ) -> Self {
        let oip_category = match category {
            ErrorCategory::TypeMismatch => "TypeErrors",
            ErrorCategory::BorrowChecker => "OwnershipBorrow",
            ErrorCategory::MissingImport => "StdlibMapping",
            ErrorCategory::SyntaxError => "ASTTransform",
            ErrorCategory::LifetimeError => "MemorySafety",
            ErrorCategory::TraitBound => "TraitBounds",
            ErrorCategory::Other => "LogicErrors",
        };

        Self {
            source_file: source_file.into(),
            error_code: None,
            clippy_lint: None,
            level: "error".to_string(),
            message: message.into(),
            oip_category: Some(oip_category.to_string()),
            confidence: 0.7,
            span: None,
            suggestion: None,
            timestamp: chrono::Utc::now().timestamp(),
            depyler_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Export corpus to JSONL file (OIP-compatible format)
///
/// # Errors
///
/// Returns error if file cannot be created or written
pub fn export_to_jsonl<P: AsRef<Path>>(exports: &[DepylerExport], path: P) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);

    for export in exports {
        let json = serde_json::to_string(export)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        writeln!(writer, "{}", json)?;
    }

    writer.flush()?;
    Ok(())
}

/// Export statistics
#[derive(Debug, Clone, Default)]
pub struct ExportStats {
    pub total_records: usize,
    pub by_error_code: HashMap<String, usize>,
    pub by_category: HashMap<String, usize>,
    pub by_class: HashMap<ErrorCodeClass, usize>,
    pub avg_confidence: f32,
}

impl ExportStats {
    /// Calculate statistics from export records
    #[must_use]
    pub fn from_exports(exports: &[DepylerExport]) -> Self {
        let mut by_error_code = HashMap::new();
        let mut by_category = HashMap::new();
        let mut by_class = HashMap::new();
        let mut confidence_sum = 0.0f32;

        for export in exports {
            if let Some(ref code) = export.error_code {
                *by_error_code.entry(code.clone()).or_insert(0) += 1;
            }
            if let Some(ref cat) = export.oip_category {
                *by_category.entry(cat.clone()).or_insert(0) += 1;
            }
            let class = export.error_code_class();
            *by_class.entry(class).or_insert(0) += 1;

            confidence_sum += export.confidence;
        }

        let avg_confidence = if exports.is_empty() {
            0.0
        } else {
            confidence_sum / exports.len() as f32
        };

        Self {
            total_records: exports.len(),
            by_error_code,
            by_category,
            by_class,
            avg_confidence,
        }
    }

    /// Print statistics summary
    pub fn print_summary(&self) {
        println!("Export Statistics:");
        println!("  Total records: {}", self.total_records);
        println!("  Avg confidence: {:.2}", self.avg_confidence);
        println!("\n  By Error Code:");
        for (code, count) in &self.by_error_code {
            println!("    {}: {}", code, count);
        }
        println!("\n  By Class:");
        for (class, count) in &self.by_class {
            println!("    {:?}: {}", class, count);
        }
    }
}

/// Configuration for Parquet export
#[derive(Debug, Clone)]
pub struct ParquetExportConfig {
    /// Batch size for writing
    pub batch_size: usize,
    /// Enable compression
    pub compression: bool,
    /// Row group size
    pub row_group_size: usize,
}

impl Default for ParquetExportConfig {
    fn default() -> Self {
        Self {
            batch_size: 1024,
            compression: true,
            row_group_size: 10000,
        }
    }
}

/// Export corpus to Parquet file (for large corpora)
///
/// Uses Arrow/Parquet for efficient columnar storage.
///
/// # Errors
///
/// Returns error if file cannot be created or data conversion fails
#[cfg(feature = "parquet")]
pub fn export_to_parquet<P: AsRef<Path>>(
    exports: &[DepylerExport],
    path: P,
    config: &ParquetExportConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    use arrow::array::{Float32Array, Int64Array, StringArray, UInt32Array};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use parquet::basic::Compression;
    use parquet::file::properties::WriterProperties;
    use std::sync::Arc;

    // Define schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("source_file", DataType::Utf8, false),
        Field::new("error_code", DataType::Utf8, true),
        Field::new("clippy_lint", DataType::Utf8, true),
        Field::new("level", DataType::Utf8, false),
        Field::new("message", DataType::Utf8, false),
        Field::new("oip_category", DataType::Utf8, true),
        Field::new("confidence", DataType::Float32, false),
        Field::new("span_line", DataType::UInt32, true),
        Field::new("span_column", DataType::UInt32, true),
        Field::new("timestamp", DataType::Int64, false),
        Field::new("depyler_version", DataType::Utf8, false),
    ]));

    // Build arrays
    let source_files: Vec<&str> = exports.iter().map(|e| e.source_file.as_str()).collect();
    let error_codes: Vec<Option<&str>> = exports.iter().map(|e| e.error_code.as_deref()).collect();
    let clippy_lints: Vec<Option<&str>> =
        exports.iter().map(|e| e.clippy_lint.as_deref()).collect();
    let levels: Vec<&str> = exports.iter().map(|e| e.level.as_str()).collect();
    let messages: Vec<&str> = exports.iter().map(|e| e.message.as_str()).collect();
    let oip_categories: Vec<Option<&str>> =
        exports.iter().map(|e| e.oip_category.as_deref()).collect();
    let confidences: Vec<f32> = exports.iter().map(|e| e.confidence).collect();
    let span_lines: Vec<Option<u32>> = exports
        .iter()
        .map(|e| e.span.as_ref().map(|s| s.line_start))
        .collect();
    let span_columns: Vec<Option<u32>> = exports
        .iter()
        .map(|e| e.span.as_ref().map(|s| s.column_start))
        .collect();
    let timestamps: Vec<i64> = exports.iter().map(|e| e.timestamp).collect();
    let versions: Vec<&str> = exports.iter().map(|e| e.depyler_version.as_str()).collect();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(source_files)),
            Arc::new(StringArray::from(error_codes)),
            Arc::new(StringArray::from(clippy_lints)),
            Arc::new(StringArray::from(levels)),
            Arc::new(StringArray::from(messages)),
            Arc::new(StringArray::from(oip_categories)),
            Arc::new(Float32Array::from(confidences)),
            Arc::new(UInt32Array::from(span_lines)),
            Arc::new(UInt32Array::from(span_columns)),
            Arc::new(Int64Array::from(timestamps)),
            Arc::new(StringArray::from(versions)),
        ],
    )?;

    // Write to Parquet
    let file = std::fs::File::create(path)?;
    let props = if config.compression {
        WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build()
    } else {
        WriterProperties::builder().build()
    };

    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;
    writer.write(&batch)?;
    writer.close()?;

    Ok(())
}

/// Batch exporter for streaming large corpora
pub struct BatchExporter {
    exports: Vec<DepylerExport>,
    batch_size: usize,
}

impl BatchExporter {
    /// Create new batch exporter
    #[must_use]
    pub fn new(batch_size: usize) -> Self {
        Self {
            exports: Vec::with_capacity(batch_size),
            batch_size,
        }
    }

    /// Add export record
    pub fn add(&mut self, export: DepylerExport) {
        self.exports.push(export);
    }

    /// Check if batch is full
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.exports.len() >= self.batch_size
    }

    /// Get current batch size
    #[must_use]
    pub fn len(&self) -> usize {
        self.exports.len()
    }

    /// Check if empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.exports.is_empty()
    }

    /// Drain batch and return exports
    pub fn drain(&mut self) -> Vec<DepylerExport> {
        std::mem::take(&mut self.exports)
    }

    /// Export current batch to JSONL and drain
    ///
    /// # Errors
    ///
    /// Returns error if file operations fail
    pub fn flush_to_jsonl<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<usize> {
        let count = self.exports.len();
        if count > 0 {
            export_to_jsonl(&self.exports, path)?;
            self.exports.clear();
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ErrorCodeClass tests ====================

    #[test]
    fn test_error_code_class_from_type_errors() {
        assert_eq!(
            ErrorCodeClass::from_error_code("E0308"),
            ErrorCodeClass::Type
        );
        assert_eq!(
            ErrorCodeClass::from_error_code("E0412"),
            ErrorCodeClass::Name
        );
        assert_eq!(
            ErrorCodeClass::from_error_code("E0606"),
            ErrorCodeClass::Type
        );
    }

    #[test]
    fn test_error_code_class_from_borrow_errors() {
        assert_eq!(
            ErrorCodeClass::from_error_code("E0502"),
            ErrorCodeClass::Borrow
        );
        assert_eq!(
            ErrorCodeClass::from_error_code("E0382"),
            ErrorCodeClass::Borrow
        );
        assert_eq!(
            ErrorCodeClass::from_error_code("E0507"),
            ErrorCodeClass::Borrow
        );
    }

    #[test]
    fn test_error_code_class_from_name_errors() {
        assert_eq!(
            ErrorCodeClass::from_error_code("E0425"),
            ErrorCodeClass::Name
        );
        assert_eq!(
            ErrorCodeClass::from_error_code("E0433"),
            ErrorCodeClass::Name
        );
    }

    #[test]
    fn test_error_code_class_from_trait_errors() {
        assert_eq!(
            ErrorCodeClass::from_error_code("E0277"),
            ErrorCodeClass::Trait
        );
    }

    #[test]
    fn test_error_code_class_from_unknown() {
        assert_eq!(
            ErrorCodeClass::from_error_code("E9999"),
            ErrorCodeClass::Other
        );
        assert_eq!(
            ErrorCodeClass::from_error_code("UNKNOWN"),
            ErrorCodeClass::Other
        );
    }

    #[test]
    fn test_error_code_class_as_u8() {
        assert_eq!(ErrorCodeClass::Type.as_u8(), 0);
        assert_eq!(ErrorCodeClass::Borrow.as_u8(), 1);
        assert_eq!(ErrorCodeClass::Name.as_u8(), 2);
        assert_eq!(ErrorCodeClass::Trait.as_u8(), 3);
        assert_eq!(ErrorCodeClass::Other.as_u8(), 4);
    }

    #[test]
    fn test_error_code_class_to_one_hot() {
        assert_eq!(ErrorCodeClass::Type.to_one_hot(), [1.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            ErrorCodeClass::Borrow.to_one_hot(),
            [0.0, 1.0, 0.0, 0.0, 0.0]
        );
        assert_eq!(ErrorCodeClass::Name.to_one_hot(), [0.0, 0.0, 1.0, 0.0, 0.0]);
        assert_eq!(
            ErrorCodeClass::Trait.to_one_hot(),
            [0.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(
            ErrorCodeClass::Other.to_one_hot(),
            [0.0, 0.0, 0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn test_error_code_class_name() {
        assert_eq!(ErrorCodeClass::Type.name(), "Type");
        assert_eq!(ErrorCodeClass::Borrow.name(), "Borrow");
    }

    // ==================== DepylerExport tests ====================

    #[test]
    fn test_depyler_export_new() {
        let export = DepylerExport::new("E0308", "mismatched types", "example.py");
        assert_eq!(export.error_code, Some("E0308".to_string()));
        assert_eq!(export.message, "mismatched types");
        assert_eq!(export.source_file, "example.py");
        assert_eq!(export.level, "error");
        assert!((export.confidence - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_depyler_export_from_clippy() {
        let export = DepylerExport::from_clippy("clippy::unwrap_used", "unwrap called", "test.py");
        assert_eq!(export.clippy_lint, Some("clippy::unwrap_used".to_string()));
        assert_eq!(export.error_code, None);
        assert_eq!(export.level, "warning");
    }

    #[test]
    fn test_depyler_export_with_confidence() {
        let export = DepylerExport::new("E0308", "msg", "file.py").with_confidence(0.95);
        assert!((export.confidence - 0.95).abs() < 0.01);
    }

    #[test]
    fn test_depyler_export_confidence_clamped() {
        let export = DepylerExport::new("E0308", "msg", "file.py").with_confidence(1.5);
        assert!((export.confidence - 1.0).abs() < 0.01);

        let export2 = DepylerExport::new("E0308", "msg", "file.py").with_confidence(-0.5);
        assert!((export2.confidence - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_depyler_export_with_oip_category() {
        let export = DepylerExport::new("E0308", "msg", "file.py").with_oip_category("TypeErrors");
        assert_eq!(export.oip_category, Some("TypeErrors".to_string()));
    }

    #[test]
    fn test_depyler_export_with_span() {
        let export = DepylerExport::new("E0308", "msg", "file.py").with_span(42, 10);
        assert_eq!(export.span, Some(SpanInfo::new(42, 10)));
    }

    #[test]
    fn test_depyler_export_with_suggestion() {
        let suggestion = SuggestionInfo::machine_applicable(".parse::<i32>()");
        let export = DepylerExport::new("E0308", "msg", "file.py").with_suggestion(suggestion);
        assert!(export.suggestion.is_some());
        assert_eq!(
            export.suggestion.unwrap().applicability,
            "MachineApplicable"
        );
    }

    #[test]
    fn test_depyler_export_error_code_class() {
        let export = DepylerExport::new("E0308", "msg", "file.py");
        assert_eq!(export.error_code_class(), ErrorCodeClass::Type);

        let export2 = DepylerExport::from_clippy("clippy::unwrap", "msg", "file.py");
        assert_eq!(export2.error_code_class(), ErrorCodeClass::Other);
    }

    #[test]
    fn test_depyler_export_derive_oip_category() {
        let export = DepylerExport::new("E0308", "msg", "file.py");
        assert_eq!(export.derive_oip_category(), Some("TypeErrors"));

        let export2 = DepylerExport::new("E0382", "msg", "file.py");
        assert_eq!(export2.derive_oip_category(), Some("MemorySafety"));

        let export3 = DepylerExport::new("E9999", "msg", "file.py");
        assert_eq!(export3.derive_oip_category(), None);
    }

    #[test]
    fn test_depyler_export_from_error_category() {
        let export = DepylerExport::from_error_category(
            ErrorCategory::TypeMismatch,
            "type error",
            "test.py",
        );
        assert_eq!(export.oip_category, Some("TypeErrors".to_string()));
    }

    #[test]
    fn test_depyler_export_serialization() {
        let export = DepylerExport::new("E0308", "mismatched types", "example.py")
            .with_confidence(0.95)
            .with_oip_category("TypeErrors");

        let json = serde_json::to_string(&export).unwrap();
        assert!(json.contains("E0308"));
        assert!(json.contains("TypeErrors"));

        let parsed: DepylerExport = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.error_code, export.error_code);
    }

    // ==================== SpanInfo tests ====================

    #[test]
    fn test_span_info_new() {
        let span = SpanInfo::new(42, 10);
        assert_eq!(span.line_start, 42);
        assert_eq!(span.column_start, 10);
    }

    // ==================== SuggestionInfo tests ====================

    #[test]
    fn test_suggestion_info_new() {
        let suggestion = SuggestionInfo::new(".parse()", "MaybeIncorrect");
        assert_eq!(suggestion.replacement, ".parse()");
        assert_eq!(suggestion.applicability, "MaybeIncorrect");
    }

    #[test]
    fn test_suggestion_info_machine_applicable() {
        let suggestion = SuggestionInfo::machine_applicable(".into()");
        assert_eq!(suggestion.applicability, "MachineApplicable");
    }

    // ==================== export_to_jsonl tests ====================

    #[test]
    fn test_export_to_jsonl() {
        let exports = vec![
            DepylerExport::new("E0308", "type mismatch", "file1.py"),
            DepylerExport::new("E0382", "use of moved value", "file2.py"),
        ];

        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("exports.jsonl");

        export_to_jsonl(&exports, &path).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2);

        // Verify JSONL format
        let _: DepylerExport = serde_json::from_str(lines[0]).unwrap();
        let _: DepylerExport = serde_json::from_str(lines[1]).unwrap();
    }

    #[test]
    fn test_export_to_jsonl_empty() {
        let exports: Vec<DepylerExport> = vec![];

        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("empty.jsonl");

        export_to_jsonl(&exports, &path).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.is_empty());
    }

    // ==================== ExportStats tests ====================

    #[test]
    fn test_export_stats_from_exports() {
        let exports = vec![
            DepylerExport::new("E0308", "msg1", "file1.py")
                .with_confidence(0.9)
                .with_oip_category("TypeErrors"),
            DepylerExport::new("E0308", "msg2", "file2.py")
                .with_confidence(0.8)
                .with_oip_category("TypeErrors"),
            DepylerExport::new("E0382", "msg3", "file3.py")
                .with_confidence(0.7)
                .with_oip_category("MemorySafety"),
        ];

        let stats = ExportStats::from_exports(&exports);

        assert_eq!(stats.total_records, 3);
        assert!((stats.avg_confidence - 0.8).abs() < 0.01);
        assert_eq!(stats.by_error_code.get("E0308"), Some(&2));
        assert_eq!(stats.by_error_code.get("E0382"), Some(&1));
        assert_eq!(stats.by_category.get("TypeErrors"), Some(&2));
        assert_eq!(stats.by_class.get(&ErrorCodeClass::Type), Some(&2));
        assert_eq!(stats.by_class.get(&ErrorCodeClass::Borrow), Some(&1));
    }

    #[test]
    fn test_export_stats_empty() {
        let exports: Vec<DepylerExport> = vec![];
        let stats = ExportStats::from_exports(&exports);

        assert_eq!(stats.total_records, 0);
        assert!((stats.avg_confidence - 0.0).abs() < 0.01);
    }

    // ==================== BatchExporter tests ====================

    #[test]
    fn test_batch_exporter_new() {
        let exporter = BatchExporter::new(100);
        assert!(exporter.is_empty());
        assert_eq!(exporter.len(), 0);
        assert!(!exporter.is_full());
    }

    #[test]
    fn test_batch_exporter_add() {
        let mut exporter = BatchExporter::new(2);

        exporter.add(DepylerExport::new("E0308", "msg", "file.py"));
        assert_eq!(exporter.len(), 1);
        assert!(!exporter.is_full());

        exporter.add(DepylerExport::new("E0382", "msg", "file.py"));
        assert_eq!(exporter.len(), 2);
        assert!(exporter.is_full());
    }

    #[test]
    fn test_batch_exporter_drain() {
        let mut exporter = BatchExporter::new(10);
        exporter.add(DepylerExport::new("E0308", "msg", "file.py"));
        exporter.add(DepylerExport::new("E0382", "msg", "file.py"));

        let drained = exporter.drain();
        assert_eq!(drained.len(), 2);
        assert!(exporter.is_empty());
    }

    #[test]
    fn test_batch_exporter_flush_to_jsonl() {
        let mut exporter = BatchExporter::new(10);
        exporter.add(DepylerExport::new("E0308", "msg", "file.py"));

        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("batch.jsonl");

        let count = exporter.flush_to_jsonl(&path).unwrap();
        assert_eq!(count, 1);
        assert!(exporter.is_empty());

        // Verify file was written
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("E0308"));
    }

    #[test]
    fn test_batch_exporter_flush_empty() {
        let mut exporter = BatchExporter::new(10);

        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("empty_batch.jsonl");

        let count = exporter.flush_to_jsonl(&path).unwrap();
        assert_eq!(count, 0);
    }
}
