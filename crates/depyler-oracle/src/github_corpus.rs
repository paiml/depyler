//! GitHub history corpus integration via organizational-intelligence-plugin.
//!
//! Recipe for learning from Git history:
//! 1. Use OIP to extract training data: `oip extract-training-data --repo <path>`
//! 2. Import JSON into depyler-oracle training pipeline
//! 3. Map OIP's DefectCategory to depyler's ErrorCategory
//!
//! OIP DefectCategory → depyler ErrorCategory mapping:
//! - OwnershipBorrow, TraitBounds → BorrowChecker, TraitBound
//! - TypeErrors, TypeAnnotationGaps → TypeMismatch
//! - StdlibMapping, ASTTransform, ConfigurationErrors → MissingImport
//! - MemorySafety → LifetimeError
//! - IteratorChain, ComprehensionBugs → Other

use crate::classifier::ErrorCategory;
use crate::moe_oracle::ExpertDomain;
use crate::training::{TrainingDataset, TrainingSample};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// OIP DefectCategory (18 categories from organizational-intelligence-plugin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OipDefectCategory {
    // General defect categories (10)
    MemorySafety,
    ConcurrencyBugs,
    LogicErrors,
    ApiMisuse,
    ResourceLeaks,
    TypeErrors,
    ConfigurationErrors,
    SecurityVulnerabilities,
    PerformanceIssues,
    IntegrationFailures,
    // Transpiler-specific categories (8)
    OperatorPrecedence,
    TypeAnnotationGaps,
    StdlibMapping,
    ASTTransform,
    ComprehensionBugs,
    IteratorChain,
    OwnershipBorrow,
    TraitBounds,
}

impl OipDefectCategory {
    /// Map OIP category to depyler ErrorCategory
    #[must_use]
    pub fn to_error_category(self) -> ErrorCategory {
        match self {
            // Ownership/borrowing issues
            Self::OwnershipBorrow | Self::MemorySafety => ErrorCategory::BorrowChecker,

            // Trait bound issues
            Self::TraitBounds => ErrorCategory::TraitBound,

            // Type errors
            Self::TypeErrors | Self::TypeAnnotationGaps => ErrorCategory::TypeMismatch,

            // Missing imports / stdlib mapping
            Self::StdlibMapping | Self::ConfigurationErrors | Self::ASTTransform => {
                ErrorCategory::MissingImport
            }

            // Lifetime errors (from memory safety patterns)
            Self::ResourceLeaks => ErrorCategory::LifetimeError,

            // Other categories
            Self::ConcurrencyBugs
            | Self::LogicErrors
            | Self::ApiMisuse
            | Self::SecurityVulnerabilities
            | Self::PerformanceIssues
            | Self::IntegrationFailures
            | Self::OperatorPrecedence
            | Self::ComprehensionBugs
            | Self::IteratorChain => ErrorCategory::Other,
        }
    }

    /// Map OIP category to MoE ExpertDomain
    #[must_use]
    pub fn to_expert_domain(self) -> ExpertDomain {
        match self {
            // Type system expert
            Self::TypeErrors | Self::TypeAnnotationGaps | Self::TraitBounds => {
                ExpertDomain::TypeSystem
            }

            // Scope resolution expert
            Self::StdlibMapping
            | Self::ConfigurationErrors
            | Self::IntegrationFailures
            | Self::ASTTransform => ExpertDomain::ScopeResolution,

            // Method/field expert
            Self::ApiMisuse | Self::IteratorChain | Self::ComprehensionBugs => {
                ExpertDomain::MethodField
            }

            // Syntax/borrow expert (default for Rust-specific)
            Self::OwnershipBorrow
            | Self::MemorySafety
            | Self::ResourceLeaks
            | Self::ConcurrencyBugs
            | Self::LogicErrors
            | Self::SecurityVulnerabilities
            | Self::PerformanceIssues
            | Self::OperatorPrecedence => ExpertDomain::SyntaxBorrowing,
        }
    }
}

/// OIP TrainingExample format (from organizational-intelligence-plugin)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OipTrainingExample {
    /// Commit message text
    pub message: String,
    /// Defect category label
    pub label: OipDefectCategory,
    /// Classifier confidence (0.0-1.0)
    pub confidence: f32,
    /// Original commit hash
    pub commit_hash: String,
    /// Author name/email
    pub author: String,
    /// Unix timestamp
    pub timestamp: i64,
    /// Lines added in commit
    pub lines_added: usize,
    /// Lines removed in commit
    pub lines_removed: usize,
    /// Number of files changed
    pub files_changed: usize,
}

/// OIP TrainingDataset format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OipTrainingDataset {
    /// Training examples
    pub train: Vec<OipTrainingExample>,
    /// Validation examples
    pub validation: Vec<OipTrainingExample>,
    /// Test examples
    pub test: Vec<OipTrainingExample>,
}

/// Load OIP training data from JSON file
///
/// # Errors
/// Returns error if file cannot be read or parsed
pub fn load_oip_training_data(path: &Path) -> Result<OipTrainingDataset, std::io::Error> {
    let content = fs::read_to_string(path)?;
    serde_json::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// Convert OIP training data to depyler TrainingDataset
#[must_use]
pub fn convert_oip_to_depyler(oip_data: &OipTrainingDataset) -> TrainingDataset {
    let mut dataset = TrainingDataset::new();

    // Process all examples (train + validation + test)
    let all_examples: Vec<_> = oip_data
        .train
        .iter()
        .chain(oip_data.validation.iter())
        .chain(oip_data.test.iter())
        .collect();

    for example in all_examples {
        let category = example.label.to_error_category();

        // Extract error pattern from commit message
        // OIP stores commit messages, we need to extract the error pattern
        let error_pattern = extract_error_pattern(&example.message);

        // Create fix suggestion from commit message
        let fix = extract_fix_from_commit(&example.message);

        dataset.add(TrainingSample::with_fix(&error_pattern, category, &fix));
    }

    dataset
}

/// Extract error pattern from commit message
fn extract_error_pattern(message: &str) -> String {
    // Look for error code patterns like error[E0308]
    if let Some(start) = message.find("error[E") {
        if let Some(end) = message[start..].find(']') {
            let error_code = &message[start..start + end + 1];
            // Find the error description after the code
            let rest = &message[start + end + 1..];
            if let Some(desc_end) = rest.find('\n') {
                return format!("{}: {}", error_code, rest[..desc_end].trim());
            }
            return error_code.to_string();
        }
    }

    // Extract from conventional commit format: "fix: <description>"
    if let Some(fix_start) = message.to_lowercase().find("fix:") {
        let rest = &message[fix_start + 4..];
        if let Some(end) = rest.find('\n') {
            return rest[..end].trim().to_string();
        }
        return rest.trim().to_string();
    }

    // Fall back to first line
    message.lines().next().unwrap_or(message).to_string()
}

/// Extract fix suggestion from commit message
fn extract_fix_from_commit(message: &str) -> String {
    // Look for "Fix:" or "Solution:" patterns
    let lower = message.to_lowercase();

    for pattern in &["solution:", "fixed by:", "fix:", "resolved:"] {
        if let Some(idx) = lower.find(pattern) {
            let rest = &message[idx + pattern.len()..];
            if let Some(end) = rest.find('\n') {
                return rest[..end].trim().to_string();
            }
            return rest.trim().to_string();
        }
    }

    // Extract from commit title
    message
        .lines()
        .next()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "See commit for fix details".to_string())
}

/// Build corpus from OIP training data file
///
/// # Errors
/// Returns error if file cannot be loaded
pub fn build_github_corpus(oip_json_path: &Path) -> Result<TrainingDataset, std::io::Error> {
    let oip_data = load_oip_training_data(oip_json_path)?;
    Ok(convert_oip_to_depyler(&oip_data))
}

/// Get MoE training samples from OIP data
///
/// Returns (error_code, context, domain) tuples for MoE training
#[must_use]
pub fn get_moe_samples_from_oip(
    oip_data: &OipTrainingDataset,
) -> Vec<(String, String, ExpertDomain)> {
    let mut samples = Vec::new();

    let all_examples: Vec<_> = oip_data
        .train
        .iter()
        .chain(oip_data.validation.iter())
        .chain(oip_data.test.iter())
        .collect();

    for example in all_examples {
        let domain = example.label.to_expert_domain();
        let error_code = infer_error_code_from_category(example.label);
        let context = example.message.clone();

        samples.push((error_code, context, domain));
    }

    samples
}

/// Infer Rust error code from OIP category
fn infer_error_code_from_category(category: OipDefectCategory) -> String {
    match category {
        OipDefectCategory::TypeErrors | OipDefectCategory::TypeAnnotationGaps => {
            "E0308".to_string()
        }
        OipDefectCategory::TraitBounds => "E0277".to_string(),
        OipDefectCategory::OwnershipBorrow | OipDefectCategory::MemorySafety => "E0382".to_string(),
        OipDefectCategory::StdlibMapping | OipDefectCategory::ASTTransform => "E0433".to_string(),
        OipDefectCategory::ApiMisuse | OipDefectCategory::IteratorChain => "E0599".to_string(),
        OipDefectCategory::ConfigurationErrors | OipDefectCategory::IntegrationFailures => {
            "E0425".to_string()
        }
        OipDefectCategory::ResourceLeaks => "E0106".to_string(),
        OipDefectCategory::ComprehensionBugs => "E0609".to_string(),
        _ => "E0000".to_string(), // Generic error
    }
}

/// Statistics about the GitHub corpus
#[derive(Debug, Default)]
pub struct CorpusStats {
    pub total_examples: usize,
    pub by_category: HashMap<String, usize>,
    pub by_expert: HashMap<ExpertDomain, usize>,
    pub avg_confidence: f32,
}

/// Analyze OIP corpus statistics
#[must_use]
pub fn analyze_corpus(oip_data: &OipTrainingDataset) -> CorpusStats {
    let mut stats = CorpusStats::default();

    let all_examples: Vec<_> = oip_data
        .train
        .iter()
        .chain(oip_data.validation.iter())
        .chain(oip_data.test.iter())
        .collect();

    stats.total_examples = all_examples.len();

    let mut total_confidence = 0.0f32;

    for example in &all_examples {
        // Count by OIP category
        let cat_name = format!("{:?}", example.label);
        *stats.by_category.entry(cat_name).or_default() += 1;

        // Count by expert domain
        let domain = example.label.to_expert_domain();
        *stats.by_expert.entry(domain).or_default() += 1;

        total_confidence += example.confidence;
    }

    if !all_examples.is_empty() {
        stats.avg_confidence = total_confidence / all_examples.len() as f32;
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oip_to_error_category_mapping() {
        assert_eq!(
            OipDefectCategory::OwnershipBorrow.to_error_category(),
            ErrorCategory::BorrowChecker
        );
        assert_eq!(
            OipDefectCategory::TypeErrors.to_error_category(),
            ErrorCategory::TypeMismatch
        );
        assert_eq!(
            OipDefectCategory::TraitBounds.to_error_category(),
            ErrorCategory::TraitBound
        );
        assert_eq!(
            OipDefectCategory::StdlibMapping.to_error_category(),
            ErrorCategory::MissingImport
        );
    }

    #[test]
    fn test_oip_to_expert_domain_mapping() {
        assert_eq!(
            OipDefectCategory::TypeErrors.to_expert_domain(),
            ExpertDomain::TypeSystem
        );
        assert_eq!(
            OipDefectCategory::StdlibMapping.to_expert_domain(),
            ExpertDomain::ScopeResolution
        );
        assert_eq!(
            OipDefectCategory::OwnershipBorrow.to_expert_domain(),
            ExpertDomain::SyntaxBorrowing
        );
        assert_eq!(
            OipDefectCategory::ApiMisuse.to_expert_domain(),
            ExpertDomain::MethodField
        );
    }

    #[test]
    fn test_extract_error_pattern() {
        let msg = "fix: error[E0308]: mismatched types\n\ndetails here";
        let pattern = extract_error_pattern(msg);
        assert!(pattern.contains("E0308"));
    }

    #[test]
    fn test_extract_error_pattern_conventional() {
        let msg = "fix: resolve borrow checker issue with lifetime";
        let pattern = extract_error_pattern(msg);
        assert_eq!(pattern, "resolve borrow checker issue with lifetime");
    }

    #[test]
    fn test_extract_fix_from_commit() {
        let msg = "fix: type mismatch\n\nSolution: Use .into() for conversion";
        let fix = extract_fix_from_commit(msg);
        assert_eq!(fix, "Use .into() for conversion");
    }

    #[test]
    fn test_infer_error_code() {
        assert_eq!(
            infer_error_code_from_category(OipDefectCategory::TypeErrors),
            "E0308"
        );
        assert_eq!(
            infer_error_code_from_category(OipDefectCategory::TraitBounds),
            "E0277"
        );
        assert_eq!(
            infer_error_code_from_category(OipDefectCategory::OwnershipBorrow),
            "E0382"
        );
    }

    #[test]
    fn test_convert_empty_dataset() {
        let oip = OipTrainingDataset {
            train: vec![],
            validation: vec![],
            test: vec![],
        };
        let dataset = convert_oip_to_depyler(&oip);
        assert!(dataset.samples().is_empty());
    }

    #[test]
    fn test_analyze_corpus_empty() {
        let oip = OipTrainingDataset {
            train: vec![],
            validation: vec![],
            test: vec![],
        };
        let stats = analyze_corpus(&oip);
        assert_eq!(stats.total_examples, 0);
    }

    #[test]
    fn test_load_real_oip_data_if_exists() {
        // Try to load real OIP training data if available
        let oip_path = std::path::Path::new(
            "/home/noah/src/organizational-intelligence-plugin/training-data.json",
        );

        if oip_path.exists() {
            let oip_data = load_oip_training_data(oip_path).expect("Should load OIP data");
            let stats = analyze_corpus(&oip_data);

            println!("OIP Corpus Statistics:");
            println!("  Total examples: {}", stats.total_examples);
            println!("  Avg confidence: {:.2}", stats.avg_confidence);
            println!("  By category:");
            for (cat, count) in &stats.by_category {
                println!("    {}: {}", cat, count);
            }
            println!("  By expert domain:");
            for (domain, count) in &stats.by_expert {
                println!("    {:?}: {}", domain, count);
            }

            // Convert to depyler format
            let depyler_dataset = convert_oip_to_depyler(&oip_data);
            println!(
                "  Converted to {} depyler samples",
                depyler_dataset.samples().len()
            );

            assert!(stats.total_examples > 0, "Should have training examples");
        } else {
            println!("OIP training data not found at {:?}, skipping", oip_path);
        }
    }

    #[test]
    fn test_convert_with_sample_data() {
        let oip = OipTrainingDataset {
            train: vec![OipTrainingExample {
                message: "fix: error[E0308]: mismatched types\n\nUse .into()".to_string(),
                label: OipDefectCategory::TypeErrors,
                confidence: 0.85,
                commit_hash: "abc123".to_string(),
                author: "test@example.com".to_string(),
                timestamp: 1234567890,
                lines_added: 10,
                lines_removed: 5,
                files_changed: 2,
            }],
            validation: vec![],
            test: vec![],
        };

        let dataset = convert_oip_to_depyler(&oip);
        assert_eq!(dataset.samples().len(), 1);

        let moe_samples = get_moe_samples_from_oip(&oip);
        assert_eq!(moe_samples.len(), 1);
        assert_eq!(moe_samples[0].0, "E0308"); // Error code
        assert_eq!(moe_samples[0].2, ExpertDomain::TypeSystem); // Expert domain
    }
}
