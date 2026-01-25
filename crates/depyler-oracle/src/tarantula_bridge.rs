//! Bridge between depyler-core decision traces and Tarantula analysis
//!
//! This module provides conversion between:
//! - `depyler_core::DecisionCategory` → `TranspilerDecision`
//! - `depyler_core::DepylerDecision` → `TranspilerDecisionRecord`
//!
//! # DEPYLER-0631: Strategy #1 Integration

use crate::tarantula::{TranspilerDecision, TranspilerDecisionRecord};
use depyler_core::decision_trace::{DecisionCategory, DepylerDecision};

/// Convert depyler-core DecisionCategory to oracle TranspilerDecision
#[must_use]
pub fn category_to_decision(category: DecisionCategory) -> TranspilerDecision {
    match category {
        DecisionCategory::TypeMapping => TranspilerDecision::TypeInference,
        DecisionCategory::BorrowStrategy => TranspilerDecision::OwnershipInference,
        DecisionCategory::LifetimeInfer => TranspilerDecision::LifetimeInference,
        DecisionCategory::MethodDispatch => TranspilerDecision::MethodTranslation,
        DecisionCategory::ImportResolve => TranspilerDecision::ImportGeneration,
        DecisionCategory::ErrorHandling => TranspilerDecision::ErrorHandling,
        DecisionCategory::Ownership => TranspilerDecision::OwnershipInference,
    }
}

/// Convert depyler-core DepylerDecision to oracle TranspilerDecisionRecord
#[must_use]
pub fn decision_to_record(decision: &DepylerDecision) -> TranspilerDecisionRecord {
    let mut record =
        TranspilerDecisionRecord::new(category_to_decision(decision.category), &decision.name);

    // Add Python line if available
    if decision.py_span.0 > 0 {
        // Estimate line number from character position (rough approximation)
        // In a real implementation, we'd track line numbers directly
        record.python_line = Some(decision.py_span.0 as u32);
    }

    // Add Rust snippet context
    if !decision.chosen_path.is_empty() {
        record = record.with_rust_snippet(&decision.chosen_path);
    }

    // Add source context
    record
        .context
        .insert("source_file".to_string(), decision.source_file.clone());
    record
        .context
        .insert("source_line".to_string(), decision.source_line.to_string());
    record.context.insert(
        "confidence".to_string(),
        format!("{:.2}", decision.confidence),
    );

    if !decision.alternatives.is_empty() {
        record
            .context
            .insert("alternatives".to_string(), decision.alternatives.join(", "));
    }

    record
}

/// Convert a batch of DepylerDecisions to TranspilerDecisionRecords
#[must_use]
pub fn decisions_to_records(decisions: &[DepylerDecision]) -> Vec<TranspilerDecisionRecord> {
    decisions.iter().map(decision_to_record).collect()
}

/// Infer TranspilerDecision from an error code
///
/// This function maps Rust error codes to likely transpiler decisions
/// that could have caused them.
#[must_use]
pub fn infer_decisions_from_error(error_code: &str) -> Vec<TranspilerDecision> {
    match error_code {
        // Type errors
        "E0308" | "E0277" | "E0271" => vec![TranspilerDecision::TypeInference],

        // Borrow checker errors
        "E0382" | "E0505" | "E0507" | "E0502" | "E0499" => {
            vec![TranspilerDecision::OwnershipInference]
        }

        // Lifetime errors
        "E0106" | "E0495" | "E0621" => vec![TranspilerDecision::LifetimeInference],

        // Import/resolution errors
        "E0433" | "E0412" | "E0405" => vec![
            TranspilerDecision::ModuleMapping,
            TranspilerDecision::ImportGeneration,
        ],

        // Method/trait errors
        "E0599" | "E0609" | "E0283" => vec![TranspilerDecision::MethodTranslation],

        // Name resolution
        "E0425" | "E0422" => vec![
            TranspilerDecision::ImportGeneration,
            TranspilerDecision::FunctionSignature,
        ],

        // Function signature issues
        "E0061" | "E0060" | "E0050" => vec![TranspilerDecision::FunctionSignature],

        // Return type issues
        "E0269" => vec![TranspilerDecision::ReturnTypeInference],

        // Default: type inference is a common culprit
        _ => vec![TranspilerDecision::TypeInference],
    }
}

/// Create decision records from error codes when no trace data is available
///
/// This is a fallback for when we don't have full decision traces but
/// need to perform Tarantula analysis.
#[must_use]
pub fn synthetic_decisions_from_errors(
    error_codes: &[String],
    error_messages: &[String],
) -> Vec<TranspilerDecisionRecord> {
    let mut decisions = Vec::new();

    for code in error_codes {
        let inferred = infer_decisions_from_error(code);
        for decision_type in inferred {
            decisions.push(TranspilerDecisionRecord::new(
                decision_type,
                format!("Inferred from error {}", code),
            ));
        }
    }

    // Also check error messages for patterns
    for msg in error_messages {
        let msg_lower = msg.to_lowercase();

        if msg_lower.contains("subprocess") || msg_lower.contains("command") {
            decisions.push(TranspilerDecisionRecord::new(
                TranspilerDecision::ModuleMapping,
                "subprocess module pattern",
            ));
        }

        if msg_lower.contains("datetime") || msg_lower.contains("chrono") {
            decisions.push(TranspilerDecisionRecord::new(
                TranspilerDecision::ModuleMapping,
                "datetime module pattern",
            ));
        }

        if msg_lower.contains("iterator") || msg_lower.contains("into_iter") {
            decisions.push(TranspilerDecisionRecord::new(
                TranspilerDecision::IteratorTransform,
                "iterator pattern",
            ));
        }

        if msg_lower.contains("string") || msg_lower.contains("str") {
            decisions.push(TranspilerDecisionRecord::new(
                TranspilerDecision::StringFormatting,
                "string handling pattern",
            ));
        }
    }

    decisions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_to_decision_mapping() {
        assert_eq!(
            category_to_decision(DecisionCategory::TypeMapping),
            TranspilerDecision::TypeInference
        );
        assert_eq!(
            category_to_decision(DecisionCategory::BorrowStrategy),
            TranspilerDecision::OwnershipInference
        );
        assert_eq!(
            category_to_decision(DecisionCategory::LifetimeInfer),
            TranspilerDecision::LifetimeInference
        );
        assert_eq!(
            category_to_decision(DecisionCategory::MethodDispatch),
            TranspilerDecision::MethodTranslation
        );
        assert_eq!(
            category_to_decision(DecisionCategory::ImportResolve),
            TranspilerDecision::ImportGeneration
        );
        assert_eq!(
            category_to_decision(DecisionCategory::ErrorHandling),
            TranspilerDecision::ErrorHandling
        );
        assert_eq!(
            category_to_decision(DecisionCategory::Ownership),
            TranspilerDecision::OwnershipInference
        );
    }

    #[test]
    fn test_decision_to_record() {
        let decision = DepylerDecision::new(
            DecisionCategory::TypeMapping,
            "int_to_i32",
            "i32",
            &["i64", "usize"],
            0.85,
            "type_mapper.rs",
            100,
        );

        let record = decision_to_record(&decision);

        assert_eq!(record.decision_type, TranspilerDecision::TypeInference);
        assert_eq!(record.description, "int_to_i32");
        assert!(record.context.contains_key("confidence"));
        assert!(record.context.contains_key("alternatives"));
    }

    #[test]
    fn test_infer_decisions_from_error() {
        // Type mismatch
        let decisions = infer_decisions_from_error("E0308");
        assert!(decisions.contains(&TranspilerDecision::TypeInference));

        // Borrow checker
        let decisions = infer_decisions_from_error("E0382");
        assert!(decisions.contains(&TranspilerDecision::OwnershipInference));

        // Import error
        let decisions = infer_decisions_from_error("E0433");
        assert!(decisions.contains(&TranspilerDecision::ModuleMapping));
        assert!(decisions.contains(&TranspilerDecision::ImportGeneration));

        // Method not found
        let decisions = infer_decisions_from_error("E0599");
        assert!(decisions.contains(&TranspilerDecision::MethodTranslation));
    }

    #[test]
    fn test_synthetic_decisions_from_errors() {
        let error_codes = vec!["E0308".to_string(), "E0433".to_string()];
        let error_messages = vec!["subprocess::Command not found".to_string()];

        let decisions = synthetic_decisions_from_errors(&error_codes, &error_messages);

        assert!(!decisions.is_empty());

        // Should have TypeInference from E0308
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::TypeInference));

        // Should have ModuleMapping from E0433
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ModuleMapping));

        // Should have ModuleMapping from "subprocess" in message
        let subprocess_decisions: Vec<_> = decisions
            .iter()
            .filter(|d| d.description.contains("subprocess"))
            .collect();
        assert!(!subprocess_decisions.is_empty());
    }

    #[test]
    fn test_decisions_to_records_batch() {
        let decisions = vec![
            DepylerDecision::new(
                DecisionCategory::TypeMapping,
                "first",
                "i32",
                &[],
                0.9,
                "test.rs",
                1,
            ),
            DepylerDecision::new(
                DecisionCategory::BorrowStrategy,
                "second",
                "&str",
                &["String"],
                0.8,
                "test.rs",
                2,
            ),
        ];

        let records = decisions_to_records(&decisions);

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].decision_type, TranspilerDecision::TypeInference);
        assert_eq!(
            records[1].decision_type,
            TranspilerDecision::OwnershipInference
        );
    }
}
