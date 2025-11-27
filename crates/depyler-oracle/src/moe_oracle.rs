//! MoE (Mixture of Experts) Oracle for Error Classification (DEPYLER-0580)
//!
//! Uses specialized experts for different error categories:
//! - Expert 1: Type System (E0308, E0277, E0606)
//! - Expert 2: Scope/Resolution (E0425, E0412, E0433)
//! - Expert 3: Method/Field (E0599, E0609)
//! - Expert 4: Syntax/Borrowing (E0369, E0282)

use std::collections::HashMap;
use std::path::Path;

use aprender::ensemble::{GatingNetwork, SoftmaxGating};
use aprender::linear_model::LinearRegression;
use aprender::primitives::{Matrix, Vector};
use aprender::traits::Estimator;
use serde::{Deserialize, Serialize};

use crate::classifier::ErrorCategory;
use crate::{OracleError, Result};

/// Error code to expert mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExpertDomain {
    /// Type system errors (E0308, E0277, E0606)
    TypeSystem,
    /// Scope/resolution errors (E0425, E0412, E0433)
    ScopeResolution,
    /// Method/field errors (E0599, E0609)
    MethodField,
    /// Syntax/borrowing errors (E0369, E0282, E0027)
    SyntaxBorrowing,
}

impl ExpertDomain {
    /// Map Rust error code to expert domain
    pub fn from_error_code(code: &str) -> Self {
        match code {
            // Type system errors
            "E0308" | "E0277" | "E0606" | "E0061" => Self::TypeSystem,
            // Scope/resolution errors
            "E0425" | "E0412" | "E0433" | "E0423" => Self::ScopeResolution,
            // Method/field errors
            "E0599" | "E0609" | "E0615" => Self::MethodField,
            // Syntax/borrowing errors
            "E0369" | "E0282" | "E0027" | "E0015" => Self::SyntaxBorrowing,
            // Default to type system for unknown
            _ => Self::TypeSystem,
        }
    }

    /// Get expert index (0-3)
    pub fn index(&self) -> usize {
        match self {
            Self::TypeSystem => 0,
            Self::ScopeResolution => 1,
            Self::MethodField => 2,
            Self::SyntaxBorrowing => 3,
        }
    }
}

/// Expert model for a specific error domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorExpert {
    /// Domain this expert specializes in
    domain: ExpertDomain,
    /// Internal linear model for scoring
    model: LinearRegression,
    /// Fix patterns learned by this expert
    fix_patterns: Vec<FixPattern>,
}

/// A learned fix pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixPattern {
    /// Error code this pattern applies to
    pub error_code: String,
    /// Context keywords that trigger this pattern
    pub context_keywords: Vec<String>,
    /// Suggested fix description
    pub fix_description: String,
    /// Confidence score for this pattern
    pub confidence: f32,
}

impl ErrorExpert {
    /// Create a new expert for the given domain
    pub fn new(domain: ExpertDomain) -> Self {
        Self {
            domain,
            model: LinearRegression::new(),
            fix_patterns: Self::default_patterns(domain),
        }
    }

    /// Default fix patterns for each domain
    fn default_patterns(domain: ExpertDomain) -> Vec<FixPattern> {
        match domain {
            ExpertDomain::TypeSystem => vec![
                FixPattern {
                    error_code: "E0308".to_string(),
                    context_keywords: vec!["expected".to_string(), "found".to_string()],
                    fix_description: "Add type coercion with .into() or as".to_string(),
                    confidence: 0.85,
                },
                FixPattern {
                    error_code: "E0277".to_string(),
                    context_keywords: vec!["trait".to_string(), "implement".to_string()],
                    fix_description: "Implement required trait or add bound".to_string(),
                    confidence: 0.80,
                },
            ],
            ExpertDomain::ScopeResolution => vec![
                FixPattern {
                    error_code: "E0425".to_string(),
                    context_keywords: vec!["cannot find".to_string(), "scope".to_string()],
                    fix_description: "Add use statement or check variable name".to_string(),
                    confidence: 0.90,
                },
                FixPattern {
                    error_code: "E0412".to_string(),
                    context_keywords: vec!["cannot find type".to_string()],
                    fix_description: "Define type or add import".to_string(),
                    confidence: 0.85,
                },
            ],
            ExpertDomain::MethodField => vec![
                FixPattern {
                    error_code: "E0599".to_string(),
                    context_keywords: vec!["method".to_string(), "not found".to_string()],
                    fix_description: "Check method name or implement trait".to_string(),
                    confidence: 0.80,
                },
                FixPattern {
                    error_code: "E0609".to_string(),
                    context_keywords: vec!["no field".to_string()],
                    fix_description: "Check struct field name or add field".to_string(),
                    confidence: 0.85,
                },
            ],
            ExpertDomain::SyntaxBorrowing => vec![
                FixPattern {
                    error_code: "E0369".to_string(),
                    context_keywords: vec!["cannot".to_string(), "binary".to_string()],
                    fix_description: "Add borrow or convert types".to_string(),
                    confidence: 0.75,
                },
                FixPattern {
                    error_code: "E0282".to_string(),
                    context_keywords: vec!["type annotation".to_string()],
                    fix_description: "Add explicit type annotation".to_string(),
                    confidence: 0.90,
                },
            ],
        }
    }

    /// Score how well this expert can handle the given error
    pub fn score(&self, error_code: &str, context: &str) -> f32 {
        let domain_match = if ExpertDomain::from_error_code(error_code) == self.domain {
            0.5
        } else {
            0.0
        };

        let pattern_match = self
            .fix_patterns
            .iter()
            .filter(|p| {
                p.error_code == error_code
                    || p.context_keywords
                        .iter()
                        .any(|kw| context.to_lowercase().contains(&kw.to_lowercase()))
            })
            .map(|p| p.confidence)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
            * 0.5;

        domain_match + pattern_match
    }

    /// Get suggested fix for the error
    pub fn suggest_fix(&self, error_code: &str, context: &str) -> Option<&FixPattern> {
        self.fix_patterns
            .iter()
            .filter(|p| {
                p.error_code == error_code
                    || p.context_keywords
                        .iter()
                        .any(|kw| context.to_lowercase().contains(&kw.to_lowercase()))
            })
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
    }
}

impl Estimator for ErrorExpert {
    fn fit(&mut self, x: &Matrix<f32>, y: &Vector<f32>) -> aprender::Result<()> {
        self.model.fit(x, y)
    }

    fn predict(&self, x: &Matrix<f32>) -> Vector<f32> {
        self.model.predict(x)
    }

    fn score(&self, x: &Matrix<f32>, y: &Vector<f32>) -> f32 {
        self.model.score(x, y)
    }
}

/// MoE Oracle configuration
#[derive(Debug, Clone)]
pub struct MoeOracleConfig {
    /// Number of top experts to use (sparse routing)
    pub top_k: usize,
    /// Temperature for gating softmax
    pub temperature: f32,
    /// Load balancing weight
    pub load_balance_weight: f32,
}

impl Default for MoeOracleConfig {
    fn default() -> Self {
        Self {
            top_k: 2,
            temperature: 1.0,
            load_balance_weight: 0.01,
        }
    }
}

/// MoE Oracle result
#[derive(Debug, Clone)]
pub struct MoeClassificationResult {
    /// Primary expert that handled the error
    pub primary_expert: ExpertDomain,
    /// Expert confidence scores
    pub expert_scores: HashMap<ExpertDomain, f32>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
    /// Overall confidence
    pub confidence: f32,
    /// Error category
    pub category: ErrorCategory,
}

/// MoE-based Oracle for error classification
pub struct MoeOracle {
    /// The 4 specialized experts
    experts: Vec<ErrorExpert>,
    /// Gating network for expert selection
    gating: SoftmaxGating,
    /// Configuration (used for future training customization)
    #[allow(dead_code)]
    config: MoeOracleConfig,
    /// Feature dimension
    feature_dim: usize,
}

impl MoeOracle {
    /// Feature dimension for error encoding
    pub const FEATURE_DIM: usize = 16;

    /// Create a new MoE Oracle with default configuration
    pub fn new() -> Self {
        Self::with_config(MoeOracleConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: MoeOracleConfig) -> Self {
        let experts = vec![
            ErrorExpert::new(ExpertDomain::TypeSystem),
            ErrorExpert::new(ExpertDomain::ScopeResolution),
            ErrorExpert::new(ExpertDomain::MethodField),
            ErrorExpert::new(ExpertDomain::SyntaxBorrowing),
        ];

        let gating = SoftmaxGating::new(Self::FEATURE_DIM, 4).with_temperature(config.temperature);

        Self {
            experts,
            gating,
            config,
            feature_dim: Self::FEATURE_DIM,
        }
    }

    /// Encode error into feature vector
    pub fn encode_error(&self, error_code: &str, context: &str) -> Vec<f32> {
        let mut features = vec![0.0; self.feature_dim];

        // Error code features (0-3): one-hot encoding of expert domain
        let domain = ExpertDomain::from_error_code(error_code);
        features[domain.index()] = 1.0;

        // Error code numeric (4): extract number from E0XXX
        if let Some(num_str) = error_code.strip_prefix('E') {
            if let Ok(num) = num_str.parse::<f32>() {
                features[4] = num / 1000.0; // Normalize
            }
        }

        // Context features (5-15): keyword presence
        let keywords = [
            "type",
            "mismatch",
            "expected",
            "found",
            "trait",
            "method",
            "field",
            "scope",
            "cannot",
            "borrow",
            "move",
        ];
        for (i, kw) in keywords.iter().enumerate() {
            if context.to_lowercase().contains(kw) {
                features[5 + i] = 1.0;
            }
        }

        features
    }

    /// Classify an error and get fix suggestion
    pub fn classify(&self, error_code: &str, context: &str) -> MoeClassificationResult {
        let features = self.encode_error(error_code, context);

        // Get gating weights from neural network
        let gating_weights = self.gating.forward(&features);

        // Primary routing: use error code to determine expert domain
        // This provides deterministic routing even without training
        let primary_domain = ExpertDomain::from_error_code(error_code);
        let primary_expert_idx = primary_domain.index();

        // Collect expert scores with gating weights
        let mut expert_scores = HashMap::new();

        for (idx, weight) in gating_weights.iter().copied().enumerate() {
            let expert = &self.experts[idx];
            // Expert score = domain-specific score + gating weight
            let domain_score = expert.score(error_code, context);
            let combined_score = domain_score + weight * 0.1; // Blend gating influence

            let domain = match idx {
                0 => ExpertDomain::TypeSystem,
                1 => ExpertDomain::ScopeResolution,
                2 => ExpertDomain::MethodField,
                _ => ExpertDomain::SyntaxBorrowing,
            };

            expert_scores.insert(domain, combined_score);
        }

        // Get confidence from the primary domain's expert
        let primary_expert = &self.experts[primary_expert_idx];
        let confidence = primary_expert.score(error_code, context);

        // Get fix suggestion from primary expert
        let suggested_fix = primary_expert
            .suggest_fix(error_code, context)
            .map(|p| p.fix_description.clone());

        // Map to error category
        let category = match primary_domain {
            ExpertDomain::TypeSystem => ErrorCategory::TypeMismatch,
            ExpertDomain::ScopeResolution => ErrorCategory::MissingImport,
            ExpertDomain::MethodField => ErrorCategory::TraitBound,
            ExpertDomain::SyntaxBorrowing => ErrorCategory::BorrowChecker,
        };

        MoeClassificationResult {
            primary_expert: primary_domain,
            expert_scores,
            suggested_fix,
            confidence: confidence.min(1.0),
            category,
        }
    }

    /// Train the MoE oracle on error samples
    pub fn train(&mut self, samples: &[(String, String, ErrorCategory)]) -> Result<()> {
        // Group samples by domain
        let mut domain_samples: HashMap<ExpertDomain, Vec<(Vec<f32>, f32)>> = HashMap::new();

        for (error_code, context, category) in samples {
            let domain = ExpertDomain::from_error_code(error_code);
            let features = self.encode_error(error_code, context);
            let label = match category {
                ErrorCategory::TypeMismatch => 1.0,
                ErrorCategory::BorrowChecker => 2.0,
                ErrorCategory::MissingImport => 3.0,
                ErrorCategory::TraitBound => 4.0,
                _ => 0.0,
            };

            domain_samples.entry(domain).or_default().push((features, label));
        }

        // Train each expert on its domain samples
        // LinearRegression requires at least 2 samples for training
        for (domain, expert_samples) in domain_samples {
            if expert_samples.len() < 2 {
                // Skip training if insufficient samples - expert uses default patterns
                continue;
            }

            let x_data: Vec<f32> = expert_samples.iter().flat_map(|(f, _)| f.clone()).collect();
            let y_data: Vec<f32> = expert_samples.iter().map(|(_, l)| *l).collect();

            let x = Matrix::from_vec(expert_samples.len(), self.feature_dim, x_data)
                .map_err(|e| OracleError::Model(e.to_string()))?;
            let y = Vector::from_slice(&y_data);

            // Train expert, but gracefully handle matrix singularity errors
            // (can occur with highly collinear features like one-hot encoding)
            if let Err(e) = self.experts[domain.index()].fit(&x, &y) {
                eprintln!(
                    "Warning: Expert {:?} training failed (matrix singularity), using default patterns: {}",
                    domain, e
                );
                // Expert will use default fix patterns instead
            }
        }

        Ok(())
    }

    /// Save the MoE oracle to file
    pub fn save(&self, _path: &Path) -> Result<()> {
        // TODO: Implement proper serialization
        Ok(())
    }

    /// Load MoE oracle from file
    pub fn load(_path: &Path) -> Result<Self> {
        // TODO: Implement proper deserialization
        Ok(Self::new())
    }
}

impl Default for MoeOracle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================
    // RED PHASE: Failing tests for TDD
    // ============================================

    #[test]
    fn test_expert_domain_mapping_e0308() {
        let domain = ExpertDomain::from_error_code("E0308");
        assert_eq!(domain, ExpertDomain::TypeSystem);
    }

    #[test]
    fn test_expert_domain_mapping_e0425() {
        let domain = ExpertDomain::from_error_code("E0425");
        assert_eq!(domain, ExpertDomain::ScopeResolution);
    }

    #[test]
    fn test_expert_domain_mapping_e0599() {
        let domain = ExpertDomain::from_error_code("E0599");
        assert_eq!(domain, ExpertDomain::MethodField);
    }

    #[test]
    fn test_expert_domain_mapping_e0369() {
        let domain = ExpertDomain::from_error_code("E0369");
        assert_eq!(domain, ExpertDomain::SyntaxBorrowing);
    }

    #[test]
    fn test_moe_oracle_creation() {
        let oracle = MoeOracle::new();
        assert_eq!(oracle.experts.len(), 4);
        assert_eq!(oracle.feature_dim, 16);
    }

    #[test]
    fn test_error_encoding_dimension() {
        let oracle = MoeOracle::new();
        let features = oracle.encode_error("E0308", "expected i32, found String");
        assert_eq!(features.len(), 16);
    }

    #[test]
    fn test_error_encoding_domain_onehot() {
        let oracle = MoeOracle::new();

        // E0308 should activate TypeSystem (index 0)
        let features = oracle.encode_error("E0308", "type mismatch");
        assert_eq!(features[0], 1.0);
        assert_eq!(features[1], 0.0);

        // E0425 should activate ScopeResolution (index 1)
        let features = oracle.encode_error("E0425", "cannot find value");
        assert_eq!(features[0], 0.0);
        assert_eq!(features[1], 1.0);
    }

    #[test]
    fn test_error_encoding_keywords() {
        let oracle = MoeOracle::new();
        let features = oracle.encode_error("E0308", "type mismatch: expected i32, found String");

        // "type" at index 5, "mismatch" at 6, "expected" at 7, "found" at 8
        assert_eq!(features[5], 1.0); // "type"
        assert_eq!(features[6], 1.0); // "mismatch"
        assert_eq!(features[7], 1.0); // "expected"
        assert_eq!(features[8], 1.0); // "found"
    }

    #[test]
    fn test_classify_type_mismatch() {
        let oracle = MoeOracle::new();
        let result = oracle.classify("E0308", "mismatched types: expected i32, found &str");

        assert_eq!(result.primary_expert, ExpertDomain::TypeSystem);
        assert!(result.confidence > 0.0);
        assert!(result.suggested_fix.is_some());
    }

    #[test]
    fn test_classify_scope_error() {
        let oracle = MoeOracle::new();
        let result = oracle.classify("E0425", "cannot find value `foo` in this scope");

        assert_eq!(result.primary_expert, ExpertDomain::ScopeResolution);
        assert!(result.suggested_fix.is_some());
    }

    #[test]
    fn test_classify_method_not_found() {
        let oracle = MoeOracle::new();
        let result = oracle.classify("E0599", "no method named `bar` found for struct `Foo`");

        assert_eq!(result.primary_expert, ExpertDomain::MethodField);
    }

    #[test]
    fn test_expert_scoring() {
        let expert = ErrorExpert::new(ExpertDomain::TypeSystem);

        // Domain match should give higher score
        let score_e0308 = expert.score("E0308", "type error");
        let score_e0425 = expert.score("E0425", "scope error");

        assert!(score_e0308 > score_e0425);
    }

    #[test]
    fn test_expert_fix_suggestion() {
        let expert = ErrorExpert::new(ExpertDomain::TypeSystem);
        let fix = expert.suggest_fix("E0308", "expected i32");

        assert!(fix.is_some());
        assert!(fix.unwrap().fix_description.contains("coercion"));
    }

    #[test]
    fn test_moe_oracle_train() {
        let mut oracle = MoeOracle::new();

        let samples = vec![
            ("E0308".to_string(), "type mismatch".to_string(), ErrorCategory::TypeMismatch),
            ("E0425".to_string(), "cannot find value".to_string(), ErrorCategory::MissingImport),
            ("E0599".to_string(), "method not found".to_string(), ErrorCategory::TraitBound),
        ];

        let result = oracle.train(&samples);
        assert!(result.is_ok());
    }

    #[test]
    fn test_moe_config_top_k() {
        let config = MoeOracleConfig {
            top_k: 1,
            ..Default::default()
        };
        let oracle = MoeOracle::with_config(config);

        // With top_k=1, primary expert should be correctly routed
        let result = oracle.classify("E0308", "type error");
        // All expert scores are returned for visibility, but primary is domain-matched
        assert_eq!(result.primary_expert, ExpertDomain::TypeSystem);
        assert_eq!(result.expert_scores.len(), 4); // All experts provide scores
    }

    #[test]
    fn test_expert_domain_indices() {
        assert_eq!(ExpertDomain::TypeSystem.index(), 0);
        assert_eq!(ExpertDomain::ScopeResolution.index(), 1);
        assert_eq!(ExpertDomain::MethodField.index(), 2);
        assert_eq!(ExpertDomain::SyntaxBorrowing.index(), 3);
    }

    #[test]
    fn test_default_config() {
        let config = MoeOracleConfig::default();
        assert_eq!(config.top_k, 2);
        assert_eq!(config.temperature, 1.0);
    }

    // Property tests would go here with proptest
    // #[test]
    // fn prop_test_feature_encoding_length() { ... }
}
