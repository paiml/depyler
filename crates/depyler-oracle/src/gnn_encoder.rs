//! GNN Error Encoder Integration for Depyler (Strategy #5 - DEPYLER-0635)
//!
//! Provides structural pattern matching using Graph Neural Networks
//! per Yasunaga & Liang (2020) for improved error pattern detection.
//!
//! ## Key Features
//!
//! - **Program-Feedback Graphs**: Connects source AST with compiler diagnostics
//! - **GNN Message Passing**: Learns context-aware error representations
//! - **Structural Similarity**: Finds similar errors even with different text
//!
//! ## Architecture
//!
//! ```text
//! Python Source ─┐
//!                ├─► ProgramFeedbackGraph ─► GNN Encoder ─► ErrorEmbedding
//! Rust Errors ───┘
//!                                                              │
//!                                                              ▼
//!                                            HNSW Index ─► Similar Patterns
//! ```
//!
//! ## References
//!
//! - DEPYLER-0635: Strategy #5 implementation
//! - Yasunaga & Liang (2020): "Graph-based Self-supervised Program Repair"

use std::collections::HashMap;

// Use public exports from aprender::citl
use aprender::citl::{
    CodeReplacement, CompilerDiagnostic, CompilerSuggestion, DiagnosticSeverity,
    SourceSpan, SuggestionApplicability, TypeInfo,
    GNNErrorEncoder, ProgramFeedbackGraph,
    Difficulty, ErrorCategory as AprenderErrorCategory, ErrorCode as AprenderErrorCode,
};

use crate::ast_embeddings::{AstEmbedder, AstEmbeddingConfig};
use crate::classifier::ErrorCategory;
use crate::error_patterns::ErrorPattern;
use crate::tarantula::TranspilerDecision;

/// Configuration for GNN-based error encoding
#[derive(Debug, Clone)]
pub struct GnnEncoderConfig {
    /// Hidden dimension for GNN layers (default: 64)
    pub hidden_dim: usize,
    /// Output embedding dimension (default: 256)
    pub output_dim: usize,
    /// Similarity threshold for pattern matching (default: 0.7)
    pub similarity_threshold: f32,
    /// Maximum number of similar patterns to return (default: 5)
    pub max_similar: usize,
    /// Whether to use HNSW index for fast search (default: true)
    pub use_hnsw: bool,
    /// Whether to include AST embeddings (Issue #210)
    pub use_ast_embeddings: bool,
    /// AST embedding dimension (default: 128)
    pub ast_embedding_dim: usize,
}

impl Default for GnnEncoderConfig {
    fn default() -> Self {
        Self {
            hidden_dim: 64,
            output_dim: 256,
            similarity_threshold: 0.7,
            max_similar: 5,
            use_hnsw: true,
            use_ast_embeddings: true,  // Issue #210: Enable by default
            ast_embedding_dim: 128,
        }
    }
}

/// A structural error pattern with GNN embedding
#[derive(Debug, Clone)]
pub struct StructuralPattern {
    /// Pattern ID
    pub id: String,
    /// Error code (e.g., "E0308")
    pub error_code: String,
    /// GNN-based embedding
    pub embedding: Vec<f32>,
    /// Original error pattern (text-based)
    pub error_pattern: Option<ErrorPattern>,
    /// Number of times this pattern has been matched
    pub match_count: u32,
    /// Success rate when applied
    pub success_rate: f64,
}

/// Result from structural similarity search
#[derive(Debug, Clone)]
pub struct SimilarPattern {
    /// Pattern ID
    pub pattern_id: String,
    /// Cosine similarity score (0.0 - 1.0)
    pub similarity: f32,
    /// The matched pattern
    pub pattern: StructuralPattern,
}

/// Statistics for the GNN encoder
#[derive(Debug, Clone, Default)]
pub struct GnnEncoderStats {
    /// Total patterns indexed
    pub patterns_indexed: usize,
    /// Total queries performed
    pub queries_performed: usize,
    /// Successful matches (above threshold)
    pub successful_matches: usize,
    /// Average similarity score
    pub avg_similarity: f64,
}

/// GNN-based error encoder for structural pattern matching
pub struct DepylerGnnEncoder {
    config: GnnEncoderConfig,
    /// Underlying aprender GNN encoder
    encoder: GNNErrorEncoder,
    /// AST embedder for Code2Vec-style embeddings (Issue #210)
    ast_embedder: Option<AstEmbedder>,
    /// Indexed patterns with embeddings
    patterns: HashMap<String, StructuralPattern>,
    /// Statistics
    stats: GnnEncoderStats,
}

impl DepylerGnnEncoder {
    /// Create a new GNN encoder with the given configuration
    #[must_use]
    pub fn new(config: GnnEncoderConfig) -> Self {
        let encoder = GNNErrorEncoder::new(config.hidden_dim, config.output_dim);

        // Issue #210: Initialize AST embedder if enabled
        let ast_embedder = if config.use_ast_embeddings {
            Some(AstEmbedder::new(AstEmbeddingConfig {
                embedding_dim: config.ast_embedding_dim,
                ..AstEmbeddingConfig::default()
            }))
        } else {
            None
        };

        Self {
            config,
            encoder,
            ast_embedder,
            patterns: HashMap::new(),
            stats: GnnEncoderStats::default(),
        }
    }

    /// Create with default configuration
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(GnnEncoderConfig::default())
    }

    /// Index an error pattern from the pattern library
    pub fn index_pattern(&mut self, pattern: &ErrorPattern, source_context: &str) {
        let diagnostic = self.pattern_to_diagnostic(pattern);
        let embedding = self.encoder.encode(&diagnostic, source_context);

        let structural = StructuralPattern {
            id: pattern.id.clone(),
            error_code: pattern.error_code.clone(),
            embedding: embedding.vector,
            error_pattern: Some(pattern.clone()),
            match_count: 0,
            success_rate: pattern.confidence,
        };

        self.patterns.insert(pattern.id.clone(), structural);
        self.stats.patterns_indexed += 1;
    }

    /// Encode a new error and find similar patterns
    pub fn find_similar(
        &mut self,
        error_code: &str,
        error_message: &str,
        source_context: &str,
    ) -> Vec<SimilarPattern> {
        self.stats.queries_performed += 1;

        // Build diagnostic from error
        let diagnostic = self.build_diagnostic(error_code, error_message);
        let query_embedding = self.encoder.encode(&diagnostic, source_context);

        // Find similar patterns
        let mut results = Vec::new();

        for (id, pattern) in &self.patterns {
            let similarity = self.cosine_similarity(&query_embedding.vector, &pattern.embedding);

            if similarity >= self.config.similarity_threshold {
                results.push(SimilarPattern {
                    pattern_id: id.clone(),
                    similarity,
                    pattern: pattern.clone(),
                });
            }
        }

        // Sort by similarity (highest first)
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        results.truncate(self.config.max_similar);

        // Update statistics
        if !results.is_empty() {
            self.stats.successful_matches += 1;
            let total_sim: f64 = results.iter().map(|r| f64::from(r.similarity)).sum();
            let count = self.stats.successful_matches as f64;
            self.stats.avg_similarity =
                (self.stats.avg_similarity * (count - 1.0) + total_sim / results.len() as f64) / count;
        }

        results
    }

    /// Record that a pattern was successfully applied
    pub fn record_match_success(&mut self, pattern_id: &str) {
        if let Some(pattern) = self.patterns.get_mut(pattern_id) {
            pattern.match_count += 1;
            // Update success rate with exponential moving average
            pattern.success_rate = pattern.success_rate * 0.9 + 0.1;
        }
    }

    /// Record that a pattern match failed
    pub fn record_match_failure(&mut self, pattern_id: &str) {
        if let Some(pattern) = self.patterns.get_mut(pattern_id) {
            pattern.match_count += 1;
            // Update success rate with exponential moving average
            pattern.success_rate *= 0.9;
        }
    }

    /// Get the embedding for an error
    #[must_use]
    pub fn encode_error(
        &self,
        error_code: &str,
        error_message: &str,
        source_context: &str,
    ) -> Vec<f32> {
        let diagnostic = self.build_diagnostic(error_code, error_message);
        let embedding = self.encoder.encode(&diagnostic, source_context);
        embedding.vector
    }

    /// Get combined embedding (GNN + AST) for an error (Issue #210)
    ///
    /// Returns a concatenated feature vector:
    /// - First `output_dim` elements: GNN embedding from program-feedback graph
    /// - Next `ast_embedding_dim` elements: Code2Vec AST embedding (if enabled)
    ///
    /// This hybrid approach combines structural similarity (GNN) with
    /// syntactic patterns (Code2Vec) for more robust error matching.
    #[must_use]
    pub fn encode_combined(
        &self,
        error_code: &str,
        error_message: &str,
        python_source: &str,
        rust_source: &str,
    ) -> Vec<f32> {
        // GNN embedding from error + Rust source
        let gnn_embedding = self.encode_error(error_code, error_message, rust_source);

        // If AST embeddings are enabled, concatenate them
        if let Some(ref ast_embedder) = self.ast_embedder {
            let python_ast = ast_embedder.embed_python(python_source);
            let rust_ast = ast_embedder.embed_rust(rust_source);

            // Concatenate: GNN + Python AST + Rust AST
            let mut combined = gnn_embedding;
            combined.extend(&python_ast.vector);
            combined.extend(&rust_ast.vector);
            combined
        } else {
            gnn_embedding
        }
    }

    /// Get the total dimension of combined embeddings
    #[must_use]
    pub fn combined_dim(&self) -> usize {
        if self.config.use_ast_embeddings {
            // GNN + Python AST + Rust AST
            self.config.output_dim + self.config.ast_embedding_dim * 2
        } else {
            self.config.output_dim
        }
    }

    /// Build a program feedback graph for visualization/debugging
    #[must_use]
    pub fn build_graph(
        &self,
        error_code: &str,
        error_message: &str,
        source_context: &str,
    ) -> ProgramFeedbackGraph {
        let diagnostic = self.build_diagnostic(error_code, error_message);
        self.encoder.build_graph(&diagnostic, source_context)
    }

    /// Get encoder statistics
    #[must_use]
    pub fn stats(&self) -> &GnnEncoderStats {
        &self.stats
    }

    /// Get configuration
    #[must_use]
    pub fn config(&self) -> &GnnEncoderConfig {
        &self.config
    }

    /// Get number of indexed patterns
    #[must_use]
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    /// Get all patterns
    pub fn patterns(&self) -> impl Iterator<Item = &StructuralPattern> {
        self.patterns.values()
    }

    /// Convert an error pattern to a compiler diagnostic
    fn pattern_to_diagnostic(&self, pattern: &ErrorPattern) -> CompilerDiagnostic {
        let error_code = self.depyler_to_aprender_code(&pattern.error_code);
        let span = SourceSpan::single_line("source.rs", 1, 1, 80);

        let mut diagnostic = CompilerDiagnostic::new(
            error_code,
            DiagnosticSeverity::Error,
            &pattern.error_pattern,
            span.clone(),
        );

        // Add type info if it looks like a type mismatch
        if pattern.error_pattern.contains("expected") && pattern.error_pattern.contains("found") {
            diagnostic = diagnostic
                .with_expected(TypeInfo::new("ExpectedType"))
                .with_found(TypeInfo::new("FoundType"));
        }

        // Add suggestion if there's a fix diff
        if !pattern.fix_diff.is_empty() {
            let suggestion = CompilerSuggestion::new(
                "Apply fix",
                SuggestionApplicability::MachineApplicable,
                CodeReplacement::new(span, &pattern.fix_diff),
            );
            diagnostic = diagnostic.with_suggestion(suggestion);
        }

        diagnostic
    }

    /// Build a diagnostic from error code and message
    fn build_diagnostic(&self, error_code: &str, error_message: &str) -> CompilerDiagnostic {
        let aprender_code = self.depyler_to_aprender_code(error_code);
        let span = SourceSpan::single_line("source.rs", 1, 1, 80);

        CompilerDiagnostic::new(
            aprender_code,
            DiagnosticSeverity::Error,
            error_message,
            span,
        )
    }

    /// Convert depyler error code to aprender error code
    fn depyler_to_aprender_code(&self, code: &str) -> AprenderErrorCode {
        let category = match code {
            "E0308" => AprenderErrorCategory::TypeMismatch,
            "E0382" | "E0502" | "E0503" => AprenderErrorCategory::Ownership,
            "E0106" | "E0495" => AprenderErrorCategory::Lifetime,
            "E0277" => AprenderErrorCategory::TraitBound,
            "E0433" | "E0412" => AprenderErrorCategory::Import,
            _ => AprenderErrorCategory::Unknown,
        };

        let difficulty = match code {
            "E0308" | "E0433" | "E0412" => Difficulty::Easy,
            "E0382" | "E0277" => Difficulty::Medium,
            "E0502" | "E0503" | "E0106" => Difficulty::Hard,
            "E0495" => Difficulty::Expert,
            _ => Difficulty::Medium,
        };

        AprenderErrorCode::new(code, category, difficulty)
    }

    /// Compute cosine similarity between two vectors
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a < 1e-10 || norm_b < 1e-10 {
            return 0.0;
        }

        dot / (norm_a * norm_b)
    }
}

/// Map depyler ErrorCategory to aprender ErrorCategory
#[must_use]
pub fn map_error_category(category: ErrorCategory) -> AprenderErrorCategory {
    match category {
        ErrorCategory::TypeMismatch => AprenderErrorCategory::TypeMismatch,
        ErrorCategory::BorrowChecker => AprenderErrorCategory::Ownership,
        ErrorCategory::LifetimeError => AprenderErrorCategory::Lifetime,
        ErrorCategory::TraitBound => AprenderErrorCategory::TraitBound,
        ErrorCategory::MissingImport => AprenderErrorCategory::Import,
        _ => AprenderErrorCategory::Unknown,
    }
}

/// Infer transpiler decision from structural similarity match
#[must_use]
pub fn infer_decision_from_match(pattern: &StructuralPattern) -> Option<TranspilerDecision> {
    // Try to get from the underlying error pattern
    if let Some(ref error_pattern) = pattern.error_pattern {
        return error_pattern.decision_type;
    }

    // Infer from error code
    match pattern.error_code.as_str() {
        "E0308" | "E0277" => Some(TranspilerDecision::TypeInference),
        "E0382" | "E0502" | "E0503" => Some(TranspilerDecision::OwnershipInference),
        "E0106" | "E0495" => Some(TranspilerDecision::LifetimeInference),
        "E0433" | "E0412" => Some(TranspilerDecision::ImportGeneration),
        "E0599" | "E0609" => Some(TranspilerDecision::MethodTranslation),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnn_encoder_config_default() {
        let config = GnnEncoderConfig::default();
        assert_eq!(config.hidden_dim, 64);
        assert_eq!(config.output_dim, 256);
        assert!((config.similarity_threshold - 0.7).abs() < f32::EPSILON);
        assert_eq!(config.max_similar, 5);
        assert!(config.use_hnsw);
    }

    #[test]
    fn test_gnn_encoder_creation() {
        let encoder = DepylerGnnEncoder::with_defaults();
        assert_eq!(encoder.pattern_count(), 0);
        assert_eq!(encoder.stats().patterns_indexed, 0);
    }

    #[test]
    fn test_encode_error() {
        let encoder = DepylerGnnEncoder::with_defaults();
        let embedding = encoder.encode_error(
            "E0308",
            "mismatched types: expected i32, found String",
            "let x: i32 = \"hello\";",
        );

        assert_eq!(embedding.len(), 256);

        // Embedding should be normalized (unit length)
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(
            (norm - 1.0).abs() < 0.1 || norm < 0.1,
            "Embedding should be normalized or near-zero, got {}",
            norm
        );
    }

    #[test]
    fn test_index_pattern() {
        let mut encoder = DepylerGnnEncoder::with_defaults();

        let pattern = ErrorPattern::new("E0308", "mismatched types", "+let x: i32 = 42;");

        encoder.index_pattern(&pattern, "let x: i32 = \"hello\";");

        assert_eq!(encoder.pattern_count(), 1);
        assert_eq!(encoder.stats().patterns_indexed, 1);
    }

    #[test]
    fn test_find_similar_empty() {
        let mut encoder = DepylerGnnEncoder::with_defaults();

        let results = encoder.find_similar(
            "E0308",
            "type mismatch",
            "let x = 5;",
        );

        assert!(results.is_empty());
        assert_eq!(encoder.stats().queries_performed, 1);
        assert_eq!(encoder.stats().successful_matches, 0);
    }

    #[test]
    fn test_find_similar_with_patterns() {
        let mut encoder = DepylerGnnEncoder::new(GnnEncoderConfig {
            similarity_threshold: 0.0, // Accept all matches for testing
            ..Default::default()
        });

        // Index a pattern
        let pattern = ErrorPattern::new("E0308", "mismatched types", "+fix");
        encoder.index_pattern(&pattern, "let x: i32 = \"hello\";");

        // Query with similar error
        let results = encoder.find_similar(
            "E0308",
            "mismatched types",
            "let y: i32 = \"world\";",
        );

        // Should find at least some similarity
        assert!(!results.is_empty());
        assert!(results[0].similarity > 0.0);
    }

    #[test]
    fn test_record_match_success() {
        let mut encoder = DepylerGnnEncoder::with_defaults();

        let pattern = ErrorPattern::new("E0308", "type error", "+fix");
        encoder.index_pattern(&pattern, "source");

        let pattern_id = encoder.patterns.keys().next().unwrap().clone();
        let initial_rate = encoder.patterns.get(&pattern_id).unwrap().success_rate;

        encoder.record_match_success(&pattern_id);

        let new_rate = encoder.patterns.get(&pattern_id).unwrap().success_rate;
        // Success should increase rate (or keep it high)
        assert!(new_rate >= initial_rate * 0.8);
    }

    #[test]
    fn test_record_match_failure() {
        let mut encoder = DepylerGnnEncoder::with_defaults();

        let pattern = ErrorPattern::new("E0308", "type error", "+fix");
        encoder.index_pattern(&pattern, "source");

        let pattern_id = encoder.patterns.keys().next().unwrap().clone();
        let initial_rate = encoder.patterns.get(&pattern_id).unwrap().success_rate;

        encoder.record_match_failure(&pattern_id);

        let new_rate = encoder.patterns.get(&pattern_id).unwrap().success_rate;
        // Failure should decrease rate
        assert!(new_rate <= initial_rate);
    }

    #[test]
    fn test_build_graph() {
        let encoder = DepylerGnnEncoder::with_defaults();
        let graph = encoder.build_graph(
            "E0308",
            "mismatched types",
            "let x: i32 = \"hello\";",
        );

        // Graph should have at least one node (diagnostic)
        assert!(graph.num_nodes() > 0);
    }

    #[test]
    fn test_cosine_similarity() {
        let encoder = DepylerGnnEncoder::with_defaults();

        // Same vectors should have similarity 1.0
        let v1 = vec![1.0, 0.0, 0.0];
        assert!((encoder.cosine_similarity(&v1, &v1) - 1.0).abs() < 0.001);

        // Orthogonal vectors should have similarity 0.0
        let v2 = vec![0.0, 1.0, 0.0];
        assert!(encoder.cosine_similarity(&v1, &v2).abs() < 0.001);

        // Empty vectors
        let empty: Vec<f32> = vec![];
        assert!(encoder.cosine_similarity(&empty, &empty).abs() < 0.001);
    }

    #[test]
    fn test_depyler_to_aprender_code() {
        let encoder = DepylerGnnEncoder::with_defaults();

        let code = encoder.depyler_to_aprender_code("E0308");
        assert_eq!(code.category, AprenderErrorCategory::TypeMismatch);
        assert_eq!(code.difficulty, Difficulty::Easy);

        let code = encoder.depyler_to_aprender_code("E0382");
        assert_eq!(code.category, AprenderErrorCategory::Ownership);

        let code = encoder.depyler_to_aprender_code("E9999");
        assert_eq!(code.category, AprenderErrorCategory::Unknown);
    }

    #[test]
    fn test_map_error_category() {
        assert_eq!(
            map_error_category(ErrorCategory::TypeMismatch),
            AprenderErrorCategory::TypeMismatch
        );
        assert_eq!(
            map_error_category(ErrorCategory::BorrowChecker),
            AprenderErrorCategory::Ownership
        );
        assert_eq!(
            map_error_category(ErrorCategory::LifetimeError),
            AprenderErrorCategory::Lifetime
        );
    }

    #[test]
    fn test_infer_decision_from_match() {
        let pattern = StructuralPattern {
            id: "test".to_string(),
            error_code: "E0308".to_string(),
            embedding: vec![0.0; 256],
            error_pattern: None,
            match_count: 0,
            success_rate: 1.0,
        };

        assert_eq!(
            infer_decision_from_match(&pattern),
            Some(TranspilerDecision::TypeInference)
        );
    }

    #[test]
    fn test_similar_errors_have_similar_embeddings() {
        let encoder = DepylerGnnEncoder::with_defaults();

        // Two similar type mismatch errors
        let e1 = encoder.encode_error(
            "E0308",
            "mismatched types: expected i32, found String",
            "let x: i32 = \"hello\";",
        );
        let e2 = encoder.encode_error(
            "E0308",
            "mismatched types: expected i64, found &str",
            "let y: i64 = \"world\";",
        );

        let sim = encoder.cosine_similarity(&e1, &e2);
        assert!(
            sim > 0.0,
            "Similar errors should have positive similarity, got {}",
            sim
        );
    }

    #[test]
    fn test_different_errors_produce_valid_embeddings() {
        let encoder = DepylerGnnEncoder::with_defaults();

        // Type mismatch vs ownership error
        let e1 = encoder.encode_error(
            "E0308",
            "mismatched types",
            "let x: i32 = \"hello\";",
        );
        let e2 = encoder.encode_error(
            "E0382",
            "borrow of moved value",
            "let x = vec![1]; let y = x; x.push(1);",
        );

        // Both embeddings should have the correct dimension
        assert_eq!(e1.len(), 256);
        assert_eq!(e2.len(), 256);

        // Both embeddings should be valid (non-NaN, non-infinite)
        assert!(e1.iter().all(|x| !x.is_nan() && x.is_finite()));
        assert!(e2.iter().all(|x| !x.is_nan() && x.is_finite()));
    }

    // Issue #210: Combined embedding tests

    #[test]
    fn test_combined_embedding_dimension() {
        let encoder = DepylerGnnEncoder::with_defaults();

        // Default: GNN (256) + Python AST (128) + Rust AST (128) = 512
        assert_eq!(encoder.combined_dim(), 512);

        // Without AST embeddings
        let config = GnnEncoderConfig {
            use_ast_embeddings: false,
            ..Default::default()
        };
        let encoder_no_ast = DepylerGnnEncoder::new(config);
        assert_eq!(encoder_no_ast.combined_dim(), 256);
    }

    #[test]
    fn test_encode_combined_with_ast() {
        let encoder = DepylerGnnEncoder::with_defaults();

        let python_source = r#"
def add(a, b):
    return a + b
"#;
        let rust_source = r#"
fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let combined = encoder.encode_combined(
            "E0308",
            "mismatched types",
            python_source,
            rust_source,
        );

        // Should have correct total dimension
        assert_eq!(combined.len(), encoder.combined_dim());
        assert_eq!(combined.len(), 512);

        // All values should be valid
        assert!(combined.iter().all(|x| !x.is_nan() && x.is_finite()));
    }

    #[test]
    fn test_encode_combined_without_ast() {
        let config = GnnEncoderConfig {
            use_ast_embeddings: false,
            ..Default::default()
        };
        let encoder = DepylerGnnEncoder::new(config);

        let combined = encoder.encode_combined(
            "E0308",
            "mismatched types",
            "def foo(): pass",
            "fn foo() {}",
        );

        // Should only have GNN dimension
        assert_eq!(combined.len(), 256);
    }

    #[test]
    fn test_combined_embedding_deterministic() {
        let encoder = DepylerGnnEncoder::with_defaults();

        let python = "def greet(name): return 'Hello ' + name";
        let rust = "fn greet(name: &str) -> String { format!(\"Hello {}\", name) }";

        let e1 = encoder.encode_combined("E0308", "type mismatch", python, rust);
        let e2 = encoder.encode_combined("E0308", "type mismatch", python, rust);

        // Same inputs should produce same outputs
        assert_eq!(e1, e2);
    }

    #[test]
    fn test_ast_embedder_initialized() {
        let encoder = DepylerGnnEncoder::with_defaults();
        assert!(encoder.ast_embedder.is_some());

        let config = GnnEncoderConfig {
            use_ast_embeddings: false,
            ..Default::default()
        };
        let encoder = DepylerGnnEncoder::new(config);
        assert!(encoder.ast_embedder.is_none());
    }
}
