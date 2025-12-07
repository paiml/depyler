//! AST Embeddings for Code2Vec-style code representation (Issue #210).
//!
//! Provides AST-to-vector embeddings for both Python HIR and Rust AST,
//! enabling semantic structural comparison beyond text-based features.
//!
//! ## Architecture
//!
//! ```text
//! Python Source → HIR → Path Contexts → Code2Vec Embedding
//!                                              │
//!                                              ▼
//!                                       Combined Features
//!                                              ↑
//! Rust Source → AST → Path Contexts → Code2Vec Embedding
//! ```

use aprender::primitives::Matrix;
use serde::{Deserialize, Serialize};

/// Configuration for AST embedding extraction
#[derive(Debug, Clone)]
pub struct AstEmbeddingConfig {
    /// Maximum path length in AST traversal (default: 8)
    pub max_path_length: usize,
    /// Maximum number of path contexts per function (default: 200)
    pub max_path_contexts: usize,
    /// Embedding dimension (default: 128)
    pub embedding_dim: usize,
    /// Whether to include terminal nodes (default: true)
    pub include_terminals: bool,
}

impl Default for AstEmbeddingConfig {
    fn default() -> Self {
        Self {
            max_path_length: 8,
            max_path_contexts: 200,
            embedding_dim: 128,
            include_terminals: true,
        }
    }
}

/// A path context in Code2Vec style: (start_terminal, path, end_terminal)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PathContext {
    /// Starting terminal node (e.g., variable name, literal)
    pub start_terminal: String,
    /// Path through AST nodes (e.g., "FunctionDef|arguments|Name")
    pub path: String,
    /// Ending terminal node
    pub end_terminal: String,
}

/// AST embedding for a code snippet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstEmbedding {
    /// The embedding vector
    pub vector: Vec<f32>,
    /// Number of path contexts extracted
    pub path_count: usize,
    /// Source code hash for caching
    pub source_hash: u64,
}

impl AstEmbedding {
    /// Create an empty embedding with the given dimension
    #[must_use]
    pub fn empty(dim: usize) -> Self {
        Self {
            vector: vec![0.0; dim],
            path_count: 0,
            source_hash: 0,
        }
    }

    /// Convert to a row matrix
    #[must_use]
    pub fn to_matrix(&self) -> Matrix<f32> {
        Matrix::from_vec(1, self.vector.len(), self.vector.clone())
            .expect("Embedding dimensions are correct")
    }
}

/// AST Embedder for extracting Code2Vec-style embeddings
pub struct AstEmbedder {
    config: AstEmbeddingConfig,
}

impl AstEmbedder {
    /// Create a new AST embedder with the given configuration
    #[must_use]
    pub fn new(config: AstEmbeddingConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(AstEmbeddingConfig::default())
    }

    /// Extract embedding from Python source code
    #[must_use]
    pub fn embed_python(&self, source: &str) -> AstEmbedding {
        let path_contexts = self.extract_python_paths(source);
        self.paths_to_embedding(&path_contexts, source)
    }

    /// Extract embedding from Rust source code
    #[must_use]
    pub fn embed_rust(&self, source: &str) -> AstEmbedding {
        let path_contexts = self.extract_rust_paths(source);
        self.paths_to_embedding(&path_contexts, source)
    }

    /// Extract path contexts from Python source
    fn extract_python_paths(&self, source: &str) -> Vec<PathContext> {
        // Parse Python to HIR and extract paths
        let mut paths = Vec::new();

        // Simple heuristic extraction for now - will use proper HIR later
        // Look for function definitions and extract terminal pairs
        for line in source.lines() {
            let line = line.trim();
            if line.starts_with("def ") {
                // Extract function name as a terminal
                if let Some(name_end) = line.find('(') {
                    let func_name = line[4..name_end].trim();
                    paths.push(PathContext {
                        start_terminal: "FunctionDef".to_string(),
                        path: "def".to_string(),
                        end_terminal: func_name.to_string(),
                    });
                }
            }

            // Extract variable assignments
            if line.contains('=') && !line.contains("==") {
                if let Some(eq_pos) = line.find('=') {
                    let lhs = line[..eq_pos].trim();
                    let rhs = line[eq_pos + 1..].trim();
                    if !lhs.is_empty() && !rhs.is_empty() {
                        paths.push(PathContext {
                            start_terminal: lhs.to_string(),
                            path: "Assign".to_string(),
                            end_terminal: rhs.chars().take(20).collect(),
                        });
                    }
                }
            }
        }

        // Limit to max contexts
        paths.truncate(self.config.max_path_contexts);
        paths
    }

    /// Extract path contexts from Rust source
    fn extract_rust_paths(&self, source: &str) -> Vec<PathContext> {
        let mut paths = Vec::new();

        // Simple heuristic extraction - will use syn for proper AST
        for line in source.lines() {
            let line = line.trim();

            // Function definitions
            if line.starts_with("fn ") || line.starts_with("pub fn ") {
                let start = if line.starts_with("pub fn ") { 7 } else { 3 };
                if let Some(paren) = line.find('(') {
                    let func_name = line[start..paren].trim();
                    paths.push(PathContext {
                        start_terminal: "FnDef".to_string(),
                        path: "fn".to_string(),
                        end_terminal: func_name.to_string(),
                    });
                }
            }

            // Let bindings
            if line.starts_with("let ") {
                if let Some(eq_pos) = line.find('=') {
                    let binding = line[4..eq_pos].trim().trim_start_matches("mut ");
                    paths.push(PathContext {
                        start_terminal: "LetBinding".to_string(),
                        path: "let".to_string(),
                        end_terminal: binding.split(':').next().unwrap_or("").trim().to_string(),
                    });
                }
            }
        }

        paths.truncate(self.config.max_path_contexts);
        paths
    }

    /// Convert path contexts to embedding vector
    fn paths_to_embedding(&self, paths: &[PathContext], source: &str) -> AstEmbedding {
        let dim = self.config.embedding_dim;
        let mut embedding = vec![0.0f32; dim];

        // Simple bag-of-paths embedding using hash-based features
        for path in paths {
            let path_str = format!("{}|{}|{}", path.start_terminal, path.path, path.end_terminal);
            let hash = self.hash_string(&path_str);

            // Distribute hash across embedding dimensions
            for i in 0..4 {
                let idx = ((hash >> (i * 16)) as usize) % dim;
                embedding[idx] += 1.0;
            }
        }

        // Normalize embedding
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in &mut embedding {
                *x /= norm;
            }
        }

        AstEmbedding {
            vector: embedding,
            path_count: paths.len(),
            source_hash: self.hash_string(source),
        }
    }

    /// Simple string hash for feature indexing
    fn hash_string(&self, s: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    /// Get the embedding dimension
    #[must_use]
    pub fn embedding_dim(&self) -> usize {
        self.config.embedding_dim
    }
}

/// Combined feature extractor using AST embeddings + keyword features
pub struct CombinedEmbeddingExtractor {
    ast_embedder: AstEmbedder,
}

impl CombinedEmbeddingExtractor {
    /// Create a new combined extractor
    #[must_use]
    pub fn new(config: AstEmbeddingConfig) -> Self {
        Self {
            ast_embedder: AstEmbedder::new(config),
        }
    }

    /// Create with defaults
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(AstEmbeddingConfig::default())
    }

    /// Extract combined features from Python source and error message
    #[must_use]
    pub fn extract_features(
        &self,
        python_source: &str,
        rust_source: &str,
        error_message: &str,
    ) -> CombinedFeatures {
        let python_embedding = self.ast_embedder.embed_python(python_source);
        let rust_embedding = self.ast_embedder.embed_rust(rust_source);
        let keyword_features = crate::features::ErrorFeatures::from_error_message(error_message);

        CombinedFeatures {
            python_embedding,
            rust_embedding,
            keyword_features,
        }
    }

    /// Get total feature dimension
    #[must_use]
    pub fn total_dim(&self) -> usize {
        // Python embedding + Rust embedding + keyword features
        self.ast_embedder.embedding_dim() * 2 + crate::features::ErrorFeatures::DIM
    }
}

/// Combined features from AST embeddings and keyword extraction
#[derive(Debug, Clone)]
pub struct CombinedFeatures {
    /// Python AST embedding
    pub python_embedding: AstEmbedding,
    /// Rust AST embedding
    pub rust_embedding: AstEmbedding,
    /// Keyword-based features
    pub keyword_features: crate::features::ErrorFeatures,
}

impl CombinedFeatures {
    /// Convert to a feature vector
    #[must_use]
    pub fn to_vec(&self) -> Vec<f32> {
        let mut features = Vec::new();
        features.extend(&self.python_embedding.vector);
        features.extend(&self.rust_embedding.vector);
        features.extend(self.keyword_features.to_vec());
        features
    }

    /// Convert to a row matrix
    #[must_use]
    pub fn to_matrix(&self) -> Matrix<f32> {
        let vec = self.to_vec();
        let dim = vec.len();
        Matrix::from_vec(1, dim, vec).expect("Feature dimensions are correct")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_embedding_config_default() {
        let config = AstEmbeddingConfig::default();
        assert_eq!(config.max_path_length, 8);
        assert_eq!(config.max_path_contexts, 200);
        assert_eq!(config.embedding_dim, 128);
        assert!(config.include_terminals);
    }

    #[test]
    fn test_embed_python_simple() {
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
def hello(name):
    message = "Hello, " + name
    return message
"#;
        let embedding = embedder.embed_python(source);

        assert_eq!(embedding.vector.len(), 128);
        assert!(embedding.path_count > 0, "Should extract path contexts");
        assert!(embedding.source_hash != 0, "Should have source hash");
    }

    #[test]
    fn test_embed_rust_simple() {
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
fn hello(name: &str) -> String {
    let message = format!("Hello, {}", name);
    message
}
"#;
        let embedding = embedder.embed_rust(source);

        assert_eq!(embedding.vector.len(), 128);
        assert!(embedding.path_count > 0, "Should extract path contexts");
    }

    #[test]
    fn test_embedding_normalization() {
        let embedder = AstEmbedder::with_defaults();
        let source = "def foo(): pass";
        let embedding = embedder.embed_python(source);

        // Check embedding is normalized (L2 norm ≈ 1.0)
        let norm: f32 = embedding.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(
            (norm - 1.0).abs() < 0.01 || norm == 0.0,
            "Embedding should be normalized, got {}",
            norm
        );
    }

    #[test]
    fn test_similar_code_similar_embeddings() {
        let embedder = AstEmbedder::with_defaults();

        let source1 = r#"
def add(a, b):
    result = a + b
    return result
"#;
        let source2 = r#"
def add(x, y):
    sum = x + y
    return sum
"#;
        let source3 = r#"
class Foo:
    def __init__(self):
        self.data = []
"#;

        let emb1 = embedder.embed_python(source1);
        let emb2 = embedder.embed_python(source2);
        let emb3 = embedder.embed_python(source3);

        // Cosine similarity
        let sim_1_2: f32 = emb1
            .vector
            .iter()
            .zip(&emb2.vector)
            .map(|(a, b)| a * b)
            .sum();
        let sim_1_3: f32 = emb1
            .vector
            .iter()
            .zip(&emb3.vector)
            .map(|(a, b)| a * b)
            .sum();

        // Similar functions should have higher similarity than different structures
        assert!(
            sim_1_2 > sim_1_3,
            "Similar functions should have higher similarity: {} vs {}",
            sim_1_2,
            sim_1_3
        );
    }

    #[test]
    fn test_combined_feature_extraction() {
        let extractor = CombinedEmbeddingExtractor::with_defaults();

        let python = "def greet(name): return 'Hello ' + name";
        let rust = "fn greet(name: &str) -> String { format!(\"Hello {}\", name) }";
        let error = "error[E0308]: mismatched types";

        let features = extractor.extract_features(python, rust, error);

        // Check dimensions
        let vec = features.to_vec();
        assert_eq!(
            vec.len(),
            extractor.total_dim(),
            "Feature vector should have correct dimension"
        );

        // Check individual components
        assert_eq!(features.python_embedding.vector.len(), 128);
        assert_eq!(features.rust_embedding.vector.len(), 128);
    }

    #[test]
    fn test_empty_source_handling() {
        let embedder = AstEmbedder::with_defaults();

        let empty_embedding = embedder.embed_python("");
        assert_eq!(empty_embedding.vector.len(), 128);
        assert_eq!(empty_embedding.path_count, 0);
    }

    #[test]
    fn test_to_matrix_conversion() {
        let embedder = AstEmbedder::with_defaults();
        let source = "def foo(): pass";
        let embedding = embedder.embed_python(source);
        let matrix = embedding.to_matrix();

        assert_eq!(matrix.n_rows(), 1);
        assert_eq!(matrix.n_cols(), 128);
    }

    #[test]
    fn test_path_context_extraction_python() {
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
def calculate(x, y):
    result = x + y
    total = result * 2
    return total
"#;
        let embedding = embedder.embed_python(source);

        // Should extract function def + 2 assignments
        assert!(embedding.path_count >= 3, "Should extract multiple paths");
    }

    #[test]
    fn test_deterministic_embeddings() {
        let embedder = AstEmbedder::with_defaults();
        let source = "def foo(x): return x * 2";

        let emb1 = embedder.embed_python(source);
        let emb2 = embedder.embed_python(source);

        assert_eq!(
            emb1.vector, emb2.vector,
            "Same source should produce same embedding"
        );
        assert_eq!(emb1.source_hash, emb2.source_hash);
    }
}
