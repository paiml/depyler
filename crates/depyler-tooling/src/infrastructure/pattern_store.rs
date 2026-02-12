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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pattern(id: &str, confidence: f32, embedding: Vec<f32>) -> TranspilationPattern {
        TranspilationPattern {
            id: id.to_string(),
            python_pattern: format!("def {}(): pass", id),
            rust_output: format!("fn {}() {{}}", id),
            error_prevented: "E0001".to_string(),
            confidence,
            usage_count: 0,
            success_rate: 1.0,
            embedding,
        }
    }

    // ========================================================================
    // TranspilationPattern tests
    // ========================================================================

    #[test]
    fn test_transpilation_pattern_new() {
        let pattern = TranspilationPattern {
            id: "pattern_1".to_string(),
            python_pattern: "x = 1".to_string(),
            rust_output: "let x = 1;".to_string(),
            error_prevented: "E0001".to_string(),
            confidence: 0.9,
            usage_count: 5,
            success_rate: 0.8,
            embedding: vec![0.1, 0.2, 0.3],
        };

        assert_eq!(pattern.id, "pattern_1");
        assert_eq!(pattern.confidence, 0.9);
        assert_eq!(pattern.usage_count, 5);
    }

    #[test]
    fn test_transpilation_pattern_clone() {
        let pattern = make_pattern("test", 0.85, vec![1.0, 2.0, 3.0]);
        let cloned = pattern.clone();
        assert_eq!(pattern.id, cloned.id);
        assert_eq!(pattern.confidence, cloned.confidence);
    }

    #[test]
    fn test_transpilation_pattern_debug() {
        let pattern = make_pattern("debug_test", 0.5, vec![]);
        let debug_str = format!("{:?}", pattern);
        assert!(debug_str.contains("debug_test"));
        assert!(debug_str.contains("TranspilationPattern"));
    }

    #[test]
    fn test_transpilation_pattern_serialize_deserialize() {
        let pattern = make_pattern("serialize_test", 0.75, vec![0.5, 0.5]);
        let json = serde_json::to_string(&pattern).unwrap();
        let deserialized: TranspilationPattern = serde_json::from_str(&json).unwrap();
        assert_eq!(pattern.id, deserialized.id);
        assert_eq!(pattern.confidence, deserialized.confidence);
    }

    // ========================================================================
    // PatternStore tests
    // ========================================================================

    #[test]
    fn test_pattern_store_new() {
        let store = PatternStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_pattern_store_default() {
        let store = PatternStore::default();
        assert!(store.is_empty());
    }

    #[test]
    fn test_pattern_store_add_pattern() {
        let mut store = PatternStore::new();
        let pattern = make_pattern("add_test", 0.9, vec![]);

        store.add_pattern(pattern);

        assert!(!store.is_empty());
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_pattern_store_get_pattern() {
        let mut store = PatternStore::new();
        store.add_pattern(make_pattern("get_test", 0.8, vec![]));

        let pattern = store.get_pattern("get_test");
        assert!(pattern.is_some());
        assert_eq!(pattern.unwrap().id, "get_test");

        let missing = store.get_pattern("nonexistent");
        assert!(missing.is_none());
    }

    #[test]
    fn test_pattern_store_get_pattern_mut() {
        let mut store = PatternStore::new();
        store.add_pattern(make_pattern("mut_test", 0.5, vec![]));

        if let Some(pattern) = store.get_pattern_mut("mut_test") {
            pattern.confidence = 0.99;
        }

        let pattern = store.get_pattern("mut_test").unwrap();
        assert_eq!(pattern.confidence, 0.99);
    }

    #[test]
    fn test_pattern_store_multiple_patterns() {
        let mut store = PatternStore::new();
        store.add_pattern(make_pattern("p1", 0.1, vec![]));
        store.add_pattern(make_pattern("p2", 0.2, vec![]));
        store.add_pattern(make_pattern("p3", 0.3, vec![]));

        assert_eq!(store.len(), 3);
        assert!(store.get_pattern("p1").is_some());
        assert!(store.get_pattern("p2").is_some());
        assert!(store.get_pattern("p3").is_some());
    }

    #[test]
    fn test_pattern_store_overwrite_same_id() {
        let mut store = PatternStore::new();
        store.add_pattern(make_pattern("same_id", 0.5, vec![]));
        store.add_pattern(make_pattern("same_id", 0.9, vec![]));

        assert_eq!(store.len(), 1);
        assert_eq!(store.get_pattern("same_id").unwrap().confidence, 0.9);
    }

    #[test]
    fn test_pattern_store_patterns_iterator() {
        let mut store = PatternStore::new();
        store.add_pattern(make_pattern("p1", 0.1, vec![]));
        store.add_pattern(make_pattern("p2", 0.2, vec![]));

        let count = store.patterns().count();
        assert_eq!(count, 2);
    }

    // ========================================================================
    // Cosine similarity tests
    // ========================================================================

    #[test]
    fn test_cosine_similarity_identical() {
        let store = PatternStore::new();
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];

        let sim = store.cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let store = PatternStore::new();
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];

        let sim = store.cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let store = PatternStore::new();
        let a = vec![1.0, 0.0];
        let b = vec![-1.0, 0.0];

        let sim = store.cosine_similarity(&a, &b);
        assert!((sim - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_similar() {
        let store = PatternStore::new();
        let a = vec![1.0, 1.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];

        let sim = store.cosine_similarity(&a, &b);
        // cos(45°) ≈ 0.707
        assert!(sim > 0.7 && sim < 0.72);
    }

    #[test]
    fn test_cosine_similarity_zero_vector() {
        let store = PatternStore::new();
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![0.0, 0.0, 0.0];

        let sim = store.cosine_similarity(&a, &b);
        assert_eq!(sim, 0.0);
    }

    #[test]
    fn test_cosine_similarity_both_zero() {
        let store = PatternStore::new();
        let a = vec![0.0, 0.0];
        let b = vec![0.0, 0.0];

        let sim = store.cosine_similarity(&a, &b);
        assert_eq!(sim, 0.0);
    }

    // ========================================================================
    // Find similar tests
    // ========================================================================

    #[test]
    fn test_find_similar_empty_store() {
        let store = PatternStore::new();
        let query = vec![1.0, 0.0, 0.0];

        let results = store.find_similar(&query, 5);
        assert!(results.is_empty());
    }

    #[test]
    fn test_find_similar_returns_most_similar() {
        let mut store = PatternStore::new();
        store.add_pattern(make_pattern("far", 0.5, vec![0.0, 1.0, 0.0]));
        store.add_pattern(make_pattern("near", 0.5, vec![1.0, 0.1, 0.0]));

        let query = vec![1.0, 0.0, 0.0];
        let results = store.find_similar(&query, 1);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "near");
    }

    #[test]
    fn test_find_similar_respects_k() {
        let mut store = PatternStore::new();
        for i in 0..10 {
            store.add_pattern(make_pattern(
                &format!("p{}", i),
                0.5,
                vec![i as f32, 0.0, 0.0],
            ));
        }

        let query = vec![5.0, 0.0, 0.0];
        let results = store.find_similar(&query, 3);

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_find_similar_k_larger_than_store() {
        let mut store = PatternStore::new();
        store.add_pattern(make_pattern("only_one", 0.5, vec![1.0, 0.0]));

        let query = vec![1.0, 0.0];
        let results = store.find_similar(&query, 10);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_find_similar_skips_empty_embeddings() {
        let mut store = PatternStore::new();
        store.add_pattern(make_pattern("with_embedding", 0.5, vec![1.0, 0.0]));
        store.add_pattern(make_pattern("without_embedding", 0.5, vec![]));

        let query = vec![1.0, 0.0];
        let results = store.find_similar(&query, 10);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "with_embedding");
    }

    // ========================================================================
    // Update confidence tests
    // ========================================================================

    #[test]
    fn test_update_confidence_success() {
        let mut store = PatternStore::new();
        let mut pattern = make_pattern("conf_test", 0.5, vec![]);
        pattern.success_rate = 0.5;
        store.add_pattern(pattern);

        store.update_confidence("conf_test", true);

        let pattern = store.get_pattern("conf_test").unwrap();
        assert!(pattern.confidence > 0.5);
        assert_eq!(pattern.usage_count, 1);
    }

    #[test]
    fn test_update_confidence_failure() {
        let mut store = PatternStore::new();
        let mut pattern = make_pattern("conf_fail", 0.5, vec![]);
        pattern.success_rate = 0.5;
        store.add_pattern(pattern);

        store.update_confidence("conf_fail", false);

        let pattern = store.get_pattern("conf_fail").unwrap();
        assert!(pattern.confidence < 0.5);
    }

    #[test]
    fn test_update_confidence_nonexistent() {
        let mut store = PatternStore::new();
        // Should not panic
        store.update_confidence("nonexistent", true);
    }

    #[test]
    fn test_update_confidence_repeated_success() {
        let mut store = PatternStore::new();
        let mut pattern = make_pattern("repeat", 0.5, vec![]);
        pattern.success_rate = 0.5;
        store.add_pattern(pattern);

        for _ in 0..10 {
            store.update_confidence("repeat", true);
        }

        let pattern = store.get_pattern("repeat").unwrap();
        assert!(pattern.confidence > 0.8);
        assert_eq!(pattern.usage_count, 10);
    }

    #[test]
    fn test_update_confidence_success_rate() {
        let mut store = PatternStore::new();
        let mut pattern = make_pattern("rate_test", 0.5, vec![]);
        pattern.success_rate = 1.0;
        store.add_pattern(pattern);

        // 2 successes, 1 failure
        store.update_confidence("rate_test", true);
        store.update_confidence("rate_test", true);
        store.update_confidence("rate_test", false);

        let pattern = store.get_pattern("rate_test").unwrap();
        assert_eq!(pattern.usage_count, 3);
        // Success rate should be around 0.67
        assert!(pattern.success_rate > 0.6 && pattern.success_rate < 0.7);
    }

    // ========================================================================
    // Serialization tests
    // ========================================================================

    #[test]
    fn test_serialize_empty_store() {
        let store = PatternStore::new();
        let json = store.serialize().unwrap();
        assert_eq!(json, "[]");
    }

    #[test]
    fn test_serialize_with_patterns() {
        let mut store = PatternStore::new();
        store.add_pattern(make_pattern("ser1", 0.5, vec![1.0]));
        store.add_pattern(make_pattern("ser2", 0.6, vec![2.0]));

        let json = store.serialize().unwrap();
        assert!(json.contains("ser1") || json.contains("ser2"));
    }

    #[test]
    fn test_deserialize_empty() {
        let store = PatternStore::deserialize("[]").unwrap();
        assert!(store.is_empty());
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let mut original = PatternStore::new();
        original.add_pattern(make_pattern("rt1", 0.75, vec![1.0, 2.0]));
        original.add_pattern(make_pattern("rt2", 0.85, vec![3.0, 4.0]));

        let json = original.serialize().unwrap();
        let restored = PatternStore::deserialize(&json).unwrap();

        assert_eq!(restored.len(), 2);
        assert!(restored.get_pattern("rt1").is_some());
        assert!(restored.get_pattern("rt2").is_some());
    }

    #[test]
    fn test_deserialize_invalid_json() {
        let result = PatternStore::deserialize("not valid json");
        assert!(result.is_err());
    }

    // ========================================================================
    // Edge case tests
    // ========================================================================

    #[test]
    fn test_empty_embedding() {
        let pattern = make_pattern("empty_emb", 0.5, vec![]);
        assert!(pattern.embedding.is_empty());
    }

    #[test]
    fn test_large_embedding() {
        let embedding: Vec<f32> = (0..384).map(|i| i as f32 / 384.0).collect();
        let pattern = make_pattern("large_emb", 0.5, embedding.clone());
        assert_eq!(pattern.embedding.len(), 384);
    }
}
