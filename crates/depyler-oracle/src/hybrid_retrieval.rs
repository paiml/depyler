//! Hybrid Retrieval with BM25 + TF-IDF Reciprocal Rank Fusion
//!
//! Combines lexical search (BM25) with TF-IDF similarity for improved
//! fix pattern retrieval. Uses Reciprocal Rank Fusion (RRF) to merge
//! rankings from both methods.
//!
//! # Algorithm
//!
//! ```text
//! RRF_score(d) = Σ 1/(k + rank_i(d))
//! ```
//!
//! where k=60 (Cormack et al. 2009) and rank_i(d) is the rank of
//! document d in ranking system i.
//!
//! # References
//!
//! - Lewis et al. (2020): Retrieval-Augmented Generation
//! - Cormack et al. (2009): Reciprocal Rank Fusion
//! - Robertson et al. (1994): BM25

use std::collections::HashMap;

use crate::OracleError;

/// BM25 parameter: term frequency saturation (typical: 1.2-2.0)
const BM25_K1: f64 = 1.5;

/// BM25 parameter: document length normalization (typical: 0.75)
const BM25_B: f64 = 0.75;

/// RRF fusion constant (Cormack et al. 2009)
const RRF_K: f64 = 60.0;

/// BM25 scorer for lexical text search
///
/// Implements Okapi BM25 ranking function for comparing query
/// against a corpus of documents.
#[derive(Debug, Clone)]
pub struct Bm25Scorer {
    /// Document frequencies: term -> count of docs containing term
    doc_frequencies: HashMap<String, usize>,
    /// Total number of documents in corpus
    num_docs: usize,
    /// Average document length in tokens
    avg_doc_len: f64,
    /// IDF cache: term -> IDF value
    idf_cache: HashMap<String, f64>,
    /// Corpus documents (tokenized)
    documents: Vec<Vec<String>>,
}

impl Bm25Scorer {
    /// Create a new BM25 scorer
    #[must_use]
    pub fn new() -> Self {
        Self {
            doc_frequencies: HashMap::new(),
            num_docs: 0,
            avg_doc_len: 0.0,
            idf_cache: HashMap::new(),
            documents: Vec::new(),
        }
    }

    /// Fit the scorer on a corpus of documents
    ///
    /// # Arguments
    ///
    /// * `documents` - Corpus of text documents
    ///
    /// # Errors
    ///
    /// Returns error if corpus is empty.
    pub fn fit<S: AsRef<str>>(&mut self, documents: &[S]) -> Result<(), OracleError> {
        if documents.is_empty() {
            return Err(OracleError::Feature(
                "Cannot fit BM25 on empty corpus".to_string(),
            ));
        }

        // Tokenize and compute statistics
        let mut total_len = 0usize;
        self.documents.clear();
        self.doc_frequencies.clear();

        for doc in documents {
            let tokens = tokenize(doc.as_ref());
            total_len += tokens.len();

            // Count unique terms per document for DF
            let unique_terms: std::collections::HashSet<_> = tokens.iter().cloned().collect();
            for term in unique_terms {
                *self.doc_frequencies.entry(term).or_insert(0) += 1;
            }

            self.documents.push(tokens);
        }

        self.num_docs = documents.len();
        self.avg_doc_len = total_len as f64 / self.num_docs as f64;

        // Precompute IDF values
        self.idf_cache.clear();
        for (term, df) in &self.doc_frequencies {
            let idf = compute_idf(*df, self.num_docs);
            self.idf_cache.insert(term.clone(), idf);
        }

        Ok(())
    }

    /// Score a query against all documents
    ///
    /// # Arguments
    ///
    /// * `query` - Query text
    ///
    /// # Returns
    ///
    /// Vector of (document_index, score) pairs sorted by score descending.
    #[must_use]
    pub fn score(&self, query: &str) -> Vec<(usize, f64)> {
        let query_tokens = tokenize(query);
        let mut scores: Vec<(usize, f64)> = self
            .documents
            .iter()
            .enumerate()
            .map(|(idx, doc_tokens)| {
                let score = self.score_document(&query_tokens, doc_tokens);
                (idx, score)
            })
            .collect();

        // Sort by score descending
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores
    }

    /// Score a single document against query tokens
    fn score_document(&self, query_tokens: &[String], doc_tokens: &[String]) -> f64 {
        let doc_len = doc_tokens.len() as f64;

        // Count term frequencies in document
        let mut tf_counts: HashMap<&str, usize> = HashMap::new();
        for token in doc_tokens {
            *tf_counts.entry(token.as_str()).or_insert(0) += 1;
        }

        let mut score = 0.0;
        for term in query_tokens {
            let tf = *tf_counts.get(term.as_str()).unwrap_or(&0) as f64;
            let idf = self.idf_cache.get(term).copied().unwrap_or(0.0);

            // BM25 formula
            let numerator = tf * (BM25_K1 + 1.0);
            let denominator = tf + BM25_K1 * (1.0 - BM25_B + BM25_B * doc_len / self.avg_doc_len);
            score += idf * numerator / denominator;
        }

        score
    }

    /// Get number of documents in corpus
    #[must_use]
    pub fn num_docs(&self) -> usize {
        self.num_docs
    }

    /// Get average document length
    #[must_use]
    pub fn avg_doc_len(&self) -> f64 {
        self.avg_doc_len
    }
}

impl Default for Bm25Scorer {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute IDF (Inverse Document Frequency)
fn compute_idf(doc_freq: usize, num_docs: usize) -> f64 {
    let n = num_docs as f64;
    let df = doc_freq as f64;
    // IDF with smoothing: log((N - df + 0.5) / (df + 0.5) + 1)
    ((n - df + 0.5) / (df + 0.5) + 1.0).ln()
}

/// Simple whitespace tokenizer with lowercasing
fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split_whitespace()
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Reciprocal Rank Fusion result
#[derive(Debug, Clone)]
pub struct RrfResult {
    /// Document index
    pub doc_idx: usize,
    /// Fused RRF score
    pub score: f64,
    /// BM25 rank (1-indexed, 0 if not in top-k)
    pub bm25_rank: usize,
    /// TF-IDF rank (1-indexed, 0 if not in top-k)
    pub tfidf_rank: usize,
}

/// Compute Reciprocal Rank Fusion between two rankings
///
/// # Arguments
///
/// * `bm25_ranking` - BM25 ranking: vec of (doc_idx, score)
/// * `tfidf_ranking` - TF-IDF ranking: vec of (doc_idx, score)
/// * `top_k` - Maximum number of results to return
///
/// # Returns
///
/// RRF-fused ranking sorted by combined score descending.
#[must_use]
pub fn reciprocal_rank_fusion(
    bm25_ranking: &[(usize, f64)],
    tfidf_ranking: &[(usize, f64)],
    top_k: usize,
) -> Vec<RrfResult> {
    // Build rank maps (1-indexed ranks)
    let bm25_ranks: HashMap<usize, usize> = bm25_ranking
        .iter()
        .enumerate()
        .map(|(rank, (idx, _))| (*idx, rank + 1))
        .collect();

    let tfidf_ranks: HashMap<usize, usize> = tfidf_ranking
        .iter()
        .enumerate()
        .map(|(rank, (idx, _))| (*idx, rank + 1))
        .collect();

    // Collect all unique document indices
    let mut all_docs: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for (idx, _) in bm25_ranking {
        all_docs.insert(*idx);
    }
    for (idx, _) in tfidf_ranking {
        all_docs.insert(*idx);
    }

    // Compute RRF scores
    let mut results: Vec<RrfResult> = all_docs
        .into_iter()
        .map(|doc_idx| {
            let bm25_rank = bm25_ranks.get(&doc_idx).copied().unwrap_or(0);
            let tfidf_rank = tfidf_ranks.get(&doc_idx).copied().unwrap_or(0);

            // RRF: sum of 1/(k + rank) for each ranking system
            let mut score = 0.0;
            if bm25_rank > 0 {
                score += 1.0 / (RRF_K + bm25_rank as f64);
            }
            if tfidf_rank > 0 {
                score += 1.0 / (RRF_K + tfidf_rank as f64);
            }

            RrfResult {
                doc_idx,
                score,
                bm25_rank,
                tfidf_rank,
            }
        })
        .collect();

    // Sort by RRF score descending
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    // Return top-k
    results.truncate(top_k);
    results
}

/// Hybrid retriever combining BM25 and TF-IDF
///
/// Provides unified interface for hybrid retrieval with RRF fusion.
pub struct HybridRetriever {
    /// BM25 scorer
    bm25: Bm25Scorer,
    /// TF-IDF extractor (uses existing depyler TfidfFeatureExtractor)
    tfidf: crate::tfidf::TfidfFeatureExtractor,
    /// Original documents for reference
    documents: Vec<String>,
    /// Whether retriever is fitted
    is_fitted: bool,
}

impl HybridRetriever {
    /// Create a new hybrid retriever
    #[must_use]
    pub fn new() -> Self {
        Self {
            bm25: Bm25Scorer::new(),
            tfidf: crate::tfidf::TfidfFeatureExtractor::new(),
            documents: Vec::new(),
            is_fitted: false,
        }
    }

    /// Fit the retriever on a corpus
    ///
    /// # Errors
    ///
    /// Returns error if fitting fails.
    pub fn fit<S: AsRef<str> + Clone>(&mut self, documents: &[S]) -> Result<(), OracleError> {
        self.bm25.fit(documents)?;
        self.tfidf.fit(documents)?;
        self.documents = documents.iter().map(|d| d.as_ref().to_string()).collect();
        self.is_fitted = true;
        Ok(())
    }

    /// Query the hybrid retriever
    ///
    /// # Arguments
    ///
    /// * `query` - Query text
    /// * `top_k` - Maximum number of results
    ///
    /// # Returns
    ///
    /// Vector of (document, RRF result) pairs sorted by score.
    ///
    /// # Errors
    ///
    /// Returns error if retriever not fitted.
    pub fn query(&self, query: &str, top_k: usize) -> Result<Vec<(String, RrfResult)>, OracleError> {
        if !self.is_fitted {
            return Err(OracleError::Feature(
                "HybridRetriever not fitted. Call fit() first".to_string(),
            ));
        }

        // Get BM25 ranking
        let bm25_ranking = self.bm25.score(query);

        // Get TF-IDF ranking
        let tfidf_ranking = self.tfidf_rank(query)?;

        // Fuse with RRF
        let rrf_results = reciprocal_rank_fusion(&bm25_ranking, &tfidf_ranking, top_k);

        // Map back to documents
        let results: Vec<(String, RrfResult)> = rrf_results
            .into_iter()
            .filter_map(|r| {
                self.documents
                    .get(r.doc_idx)
                    .map(|doc| (doc.clone(), r))
            })
            .collect();

        Ok(results)
    }

    /// Get TF-IDF ranking for query
    fn tfidf_rank(&self, query: &str) -> Result<Vec<(usize, f64)>, OracleError> {
        // Transform query to TF-IDF vector
        let query_vec = self.tfidf.transform(&[query])?;

        // Transform all documents
        let doc_vecs = self.tfidf.transform(&self.documents)?;

        // Compute cosine similarities
        let mut rankings: Vec<(usize, f64)> = (0..self.documents.len())
            .map(|idx| {
                let sim = cosine_similarity(&query_vec, 0, &doc_vecs, idx);
                (idx, sim)
            })
            .collect();

        // Sort by similarity descending
        rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok(rankings)
    }

    /// Check if retriever is fitted
    #[must_use]
    pub fn is_fitted(&self) -> bool {
        self.is_fitted
    }

    /// Get number of documents
    #[must_use]
    pub fn num_docs(&self) -> usize {
        self.documents.len()
    }
}

impl Default for HybridRetriever {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute cosine similarity between two vectors in matrices
fn cosine_similarity(
    a_matrix: &aprender::primitives::Matrix<f64>,
    a_row: usize,
    b_matrix: &aprender::primitives::Matrix<f64>,
    b_row: usize,
) -> f64 {
    let cols = a_matrix.n_cols();
    if cols != b_matrix.n_cols() {
        return 0.0;
    }

    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for col in 0..cols {
        let a_val = a_matrix.get(a_row, col);
        let b_val = b_matrix.get(b_row, col);
        dot += a_val * b_val;
        norm_a += a_val * a_val;
        norm_b += b_val * b_val;
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom == 0.0 {
        0.0
    } else {
        dot / denom
    }
}

// ============================================================================
// EXTREME TDD Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // BM25 Scorer Tests
    // ========================================================================

    #[test]
    fn test_bm25_scorer_new() {
        let scorer = Bm25Scorer::new();
        assert_eq!(scorer.num_docs(), 0);
        assert_eq!(scorer.avg_doc_len(), 0.0);
    }

    #[test]
    fn test_bm25_fit_empty_corpus() {
        let mut scorer = Bm25Scorer::new();
        let empty: Vec<&str> = vec![];
        let result = scorer.fit(&empty);
        assert!(result.is_err());
    }

    #[test]
    fn test_bm25_fit_success() {
        let mut scorer = Bm25Scorer::new();
        let docs = vec![
            "expected i32 found str",
            "cannot borrow as mutable",
            "lifetime does not live long enough",
        ];

        let result = scorer.fit(&docs);
        assert!(result.is_ok());
        assert_eq!(scorer.num_docs(), 3);
        assert!(scorer.avg_doc_len() > 0.0);
    }

    #[test]
    fn test_bm25_score_exact_match_highest() {
        let mut scorer = Bm25Scorer::new();
        let docs = vec![
            "expected i32 found str",
            "cannot borrow as mutable",
            "type mismatch error",
        ];
        scorer.fit(&docs).unwrap();

        let scores = scorer.score("expected i32 found str");

        // Exact match should have highest score
        assert!(!scores.is_empty());
        assert_eq!(scores[0].0, 0); // First doc should be ranked first
        assert!(scores[0].1 > scores[1].1); // And have higher score than others
    }

    #[test]
    fn test_bm25_score_partial_match() {
        let mut scorer = Bm25Scorer::new();
        let docs = vec![
            "type mismatch expected i32",
            "cannot borrow mutably",
            "expected value found reference",
        ];
        scorer.fit(&docs).unwrap();

        let scores = scorer.score("expected");

        // Documents with "expected" should rank higher
        let top_indices: Vec<usize> = scores.iter().take(2).map(|(idx, _)| *idx).collect();
        assert!(top_indices.contains(&0)); // "expected i32"
        assert!(top_indices.contains(&2)); // "expected value"
    }

    #[test]
    fn test_bm25_idf_common_terms_lower() {
        let mut scorer = Bm25Scorer::new();
        let docs = vec![
            "error error error",
            "error message here",
            "error type found",
            "unique distinct different",
        ];
        scorer.fit(&docs).unwrap();

        // Common term "error" should have lower IDF than rare term
        let error_idf = scorer.idf_cache.get("error").copied().unwrap_or(0.0);
        let unique_idf = scorer.idf_cache.get("unique").copied().unwrap_or(f64::MAX);

        assert!(unique_idf > error_idf, "Rare terms should have higher IDF");
    }

    // ========================================================================
    // Tokenization Tests
    // ========================================================================

    #[test]
    fn test_tokenize_basic() {
        let tokens = tokenize("Hello World");
        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_with_punctuation() {
        let tokens = tokenize("error[E0308]: expected `i32`, found `str`");
        assert!(tokens.contains(&"expected".to_string()));
        assert!(tokens.contains(&"i32".to_string()));
        assert!(tokens.contains(&"str".to_string()));
    }

    #[test]
    fn test_tokenize_empty() {
        let tokens = tokenize("");
        assert!(tokens.is_empty());
    }

    // ========================================================================
    // IDF Tests
    // ========================================================================

    #[test]
    fn test_compute_idf_rare_term() {
        // Term in 1 of 100 docs
        let idf = compute_idf(1, 100);
        assert!(idf > 4.0, "Rare term should have high IDF");
    }

    #[test]
    fn test_compute_idf_common_term() {
        // Term in 90 of 100 docs
        let idf = compute_idf(90, 100);
        assert!(idf < 1.0, "Common term should have low IDF");
    }

    #[test]
    fn test_compute_idf_all_docs() {
        // Term in all docs
        let idf = compute_idf(100, 100);
        assert!(idf > 0.0, "IDF should still be positive with smoothing");
    }

    // ========================================================================
    // RRF Tests
    // ========================================================================

    #[test]
    fn test_rrf_empty_rankings() {
        let bm25: Vec<(usize, f64)> = vec![];
        let tfidf: Vec<(usize, f64)> = vec![];

        let result = reciprocal_rank_fusion(&bm25, &tfidf, 10);
        assert!(result.is_empty());
    }

    #[test]
    fn test_rrf_single_ranking() {
        let bm25 = vec![(0, 1.0), (1, 0.5), (2, 0.3)];
        let tfidf: Vec<(usize, f64)> = vec![];

        let result = reciprocal_rank_fusion(&bm25, &tfidf, 10);

        assert_eq!(result.len(), 3);
        // Doc 0 is rank 1 in BM25, should have highest score
        assert_eq!(result[0].doc_idx, 0);
        assert!(result[0].bm25_rank > 0);
        assert_eq!(result[0].tfidf_rank, 0);
    }

    #[test]
    fn test_rrf_fusion_boosts_agreement() {
        // Both rankings agree on doc 0 being best
        let bm25 = vec![(0, 1.0), (1, 0.5), (2, 0.3)];
        let tfidf = vec![(0, 0.9), (2, 0.4), (1, 0.2)];

        let result = reciprocal_rank_fusion(&bm25, &tfidf, 10);

        // Doc 0 should be top (rank 1 in both)
        assert_eq!(result[0].doc_idx, 0);
        // Score should be higher than if only in one ranking
        let expected_score = 1.0 / (RRF_K + 1.0) + 1.0 / (RRF_K + 1.0);
        assert!((result[0].score - expected_score).abs() < 0.001);
    }

    #[test]
    fn test_rrf_top_k_limiting() {
        let bm25: Vec<(usize, f64)> = (0..100).map(|i| (i, 1.0 / (i as f64 + 1.0))).collect();
        let tfidf: Vec<(usize, f64)> = (0..100).map(|i| (i, 1.0 / (i as f64 + 1.0))).collect();

        let result = reciprocal_rank_fusion(&bm25, &tfidf, 5);

        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_rrf_disjoint_rankings() {
        // BM25 and TF-IDF return completely different docs
        let bm25 = vec![(0, 1.0), (1, 0.5)];
        let tfidf = vec![(2, 0.9), (3, 0.4)];

        let result = reciprocal_rank_fusion(&bm25, &tfidf, 10);

        assert_eq!(result.len(), 4);
        // All docs should have equal scores (each appears in exactly one ranking at same rank)
        let top_score = result[0].score;
        let second_score = result[1].score;
        assert!((top_score - second_score).abs() < 0.001);
    }

    // ========================================================================
    // Hybrid Retriever Tests
    // ========================================================================

    #[test]
    fn test_hybrid_retriever_new() {
        let retriever = HybridRetriever::new();
        assert!(!retriever.is_fitted());
        assert_eq!(retriever.num_docs(), 0);
    }

    #[test]
    fn test_hybrid_retriever_query_without_fit() {
        let retriever = HybridRetriever::new();
        let result = retriever.query("test query", 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_hybrid_retriever_fit_and_query() {
        let mut retriever = HybridRetriever::new();
        let docs = vec![
            "expected i32 found str type mismatch",
            "cannot borrow as mutable borrow checker error",
            "lifetime does not live long enough",
            "missing lifetime specifier",
        ];

        retriever.fit(&docs).unwrap();
        assert!(retriever.is_fitted());
        assert_eq!(retriever.num_docs(), 4);

        let results = retriever.query("type mismatch expected", 3).unwrap();

        assert!(!results.is_empty());
        assert!(results.len() <= 3);

        // First result should contain "type mismatch" or "expected"
        let (top_doc, _) = &results[0];
        assert!(
            top_doc.contains("type") || top_doc.contains("expected"),
            "Top result should match query terms"
        );
    }

    #[test]
    fn test_hybrid_retriever_returns_documents() {
        let mut retriever = HybridRetriever::new();
        let docs = vec!["document one", "document two", "document three"];

        retriever.fit(&docs).unwrap();
        let results = retriever.query("one", 5).unwrap();

        // Should return actual document strings, not indices
        for (doc, _) in &results {
            assert!(docs.contains(&doc.as_str()));
        }
    }

    // ========================================================================
    // Cosine Similarity Tests
    // ========================================================================

    #[test]
    fn test_cosine_similarity_identical() {
        let matrix =
            aprender::primitives::Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 1.0, 2.0, 3.0])
                .unwrap();

        let sim = cosine_similarity(&matrix, 0, &matrix, 1);
        assert!((sim - 1.0).abs() < 0.001, "Identical vectors should have similarity 1.0");
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let matrix =
            aprender::primitives::Matrix::from_vec(2, 2, vec![1.0, 0.0, 0.0, 1.0]).unwrap();

        let sim = cosine_similarity(&matrix, 0, &matrix, 1);
        assert!((sim - 0.0).abs() < 0.001, "Orthogonal vectors should have similarity 0.0");
    }

    #[test]
    fn test_cosine_similarity_zero_vector() {
        let matrix =
            aprender::primitives::Matrix::from_vec(2, 2, vec![1.0, 2.0, 0.0, 0.0]).unwrap();

        let sim = cosine_similarity(&matrix, 0, &matrix, 1);
        assert_eq!(sim, 0.0, "Zero vector should return 0 similarity");
    }

    // ========================================================================
    // Property Tests
    // ========================================================================

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_bm25_scores_non_negative(
            doc1 in "[a-z ]{5,50}",
            doc2 in "[a-z ]{5,50}",
            query in "[a-z ]{1,20}"
        ) {
            let mut scorer = Bm25Scorer::new();
            scorer.fit(&[doc1.as_str(), doc2.as_str()]).unwrap();

            let scores = scorer.score(&query);
            for (_, score) in scores {
                prop_assert!(score >= 0.0, "BM25 scores should be non-negative");
            }
        }

        #[test]
        fn prop_rrf_scores_bounded(
            n_docs in 1usize..50
        ) {
            let bm25: Vec<(usize, f64)> = (0..n_docs)
                .map(|i| (i, 1.0 / (i as f64 + 1.0)))
                .collect();
            let tfidf: Vec<(usize, f64)> = (0..n_docs)
                .map(|i| (i, 1.0 / (i as f64 + 1.0)))
                .collect();

            let results = reciprocal_rank_fusion(&bm25, &tfidf, n_docs);

            for r in results {
                // Maximum RRF score is 2 * 1/(k+1) when doc is rank 1 in both
                let max_score = 2.0 / (RRF_K + 1.0);
                prop_assert!(r.score <= max_score + 0.001);
                prop_assert!(r.score >= 0.0);
            }
        }

        #[test]
        fn prop_tokenize_deterministic(text in "[a-zA-Z ]{0,100}") {
            let tokens1 = tokenize(&text);
            let tokens2 = tokenize(&text);
            prop_assert_eq!(tokens1, tokens2);
        }

        #[test]
        fn prop_idf_monotonic(
            df1 in 1usize..50,
            df2 in 1usize..50
        ) {
            let n_docs = 100;
            let idf1 = compute_idf(df1, n_docs);
            let idf2 = compute_idf(df2, n_docs);

            // IDF should be monotonically decreasing with DF
            if df1 < df2 {
                prop_assert!(idf1 >= idf2, "IDF should decrease as DF increases");
            }
        }
    }

    // ========================================================================
    // Integration Tests
    // ========================================================================

    #[test]
    fn test_hybrid_retrieval_full_pipeline() {
        let mut retriever = HybridRetriever::new();

        // Realistic error message corpus
        let corpus = vec![
            "error[E0308]: expected `i32`, found `&str`",
            "error[E0308]: mismatched types expected i32 found String",
            "error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable",
            "error[E0597]: `x` does not live long enough",
            "error[E0106]: missing lifetime specifier",
            "error[E0277]: the trait bound `Foo: Clone` is not satisfied",
            "error[E0425]: cannot find value `foo` in this scope",
        ];

        retriever.fit(&corpus).unwrap();

        // Query should find type mismatch errors
        let results = retriever.query("type mismatch expected found", 3).unwrap();
        assert!(!results.is_empty());

        let (top_doc, top_result) = &results[0];
        assert!(
            top_doc.contains("expected") || top_doc.contains("found"),
            "Top result should match type mismatch query"
        );
        assert!(top_result.score > 0.0);
    }
}
