//! Pattern Store with Semantic Search (DEPYLER-0925)
//!
//! Stores successful transpilation patterns and enables O(log n) retrieval
//! using approximate nearest neighbor search via semantic embeddings.
//!
//! ## Design
//!
//! Uses HNSW (Hierarchical Navigable Small World) algorithm for efficient
//! similarity search. Reference: Malkov & Yashunin (2020).
//!
//! Patterns include:
//! - Python source pattern (normalized AST)
//! - Generated Rust output
//! - Error code prevented
//! - Confidence score (updated via online learning)
//! - 384-dimensional semantic embedding

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A successful transpilation pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspilationPattern {
    pub id: String,
    pub python_pattern: String,
    pub rust_output: String,
    pub error_prevented: String,
    pub confidence: f32,
    pub usage_count: u32,
    pub success_rate: f32,
    pub embedding: Vec<f32>,
}

/// Pattern store with semantic search capability
#[derive(Default)]
pub struct PatternStore {
    patterns: HashMap<String, TranspilationPattern>,
}

impl PatternStore {
    /// Create a new pattern store
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
        }
    }

    /// Add a pattern to the store
    pub fn add_pattern(&mut self, pattern: TranspilationPattern) {
        self.patterns.insert(pattern.id.clone(), pattern);
    }

    /// Get a pattern by ID
    pub fn get_pattern(&self, id: &str) -> Option<&TranspilationPattern> {
        self.patterns.get(id)
    }

    /// Get mutable reference to a pattern
    pub fn get_pattern_mut(&mut self, id: &str) -> Option<&mut TranspilationPattern> {
        self.patterns.get_mut(id)
    }

    /// Serialize the store to JSON
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        let patterns: Vec<_> = self.patterns.values().collect();
        serde_json::to_string(&patterns)
    }

    /// Deserialize the store from JSON
    pub fn deserialize(json: &str) -> Result<Self, serde_json::Error> {
        let patterns: Vec<TranspilationPattern> = serde_json::from_str(json)?;
        let mut store = Self::new();
        for pattern in patterns {
            store.add_pattern(pattern);
        }
        Ok(store)
    }

    /// Calculate cosine similarity between two embeddings
    pub fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a < f32::EPSILON || norm_b < f32::EPSILON {
            return 0.0;
        }

        dot / (norm_a * norm_b)
    }

    /// Find k most similar patterns to query embedding
    ///
    /// Uses brute-force search for now. TODO: Implement HNSW for O(log n).
    pub fn find_similar(&self, query: &[f32], k: usize) -> Vec<&TranspilationPattern> {
        let mut similarities: Vec<_> = self
            .patterns
            .values()
            .filter(|p| !p.embedding.is_empty())
            .map(|p| (p, self.cosine_similarity(query, &p.embedding)))
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        similarities.into_iter().take(k).map(|(p, _)| p).collect()
    }

    /// Update pattern confidence based on compilation result
    ///
    /// Uses exponential moving average: confidence = (1-α)*confidence + α*outcome
    pub fn update_confidence(&mut self, pattern_id: &str, success: bool) {
        if let Some(pattern) = self.patterns.get_mut(pattern_id) {
            pattern.usage_count += 1;
            let alpha = 0.1; // Learning rate
            let outcome = if success { 1.0 } else { 0.0 };
            pattern.confidence = (1.0 - alpha) * pattern.confidence + alpha * outcome;

            // Update success rate
            let total = pattern.usage_count as f32;
            let successes = pattern.success_rate * (total - 1.0) + outcome;
            pattern.success_rate = successes / total;
        }
    }

    /// Get all patterns (for iteration)
    pub fn patterns(&self) -> impl Iterator<Item = &TranspilationPattern> {
        self.patterns.values()
    }

    /// Number of patterns in store
    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    /// Check if store is empty
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }
}
