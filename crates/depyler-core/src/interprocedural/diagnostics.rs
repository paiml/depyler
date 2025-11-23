//! Diagnostics and debugging tools for mutability inference
//!
//! This module provides tools to understand and debug mutability inference decisions:
//! - Detailed diagnostics explaining why each parameter got its mutability
//! - Alternative suggestions for different mutability choices
//! - Call graph visualization export

use super::flow_analyzer::{Confidence, MutabilityKind, MutabilityReason, MutabilitySignature};
use super::intraprocedural::Location;
use std::collections::HashMap;

/// Diagnostic information for a parameter's mutability
#[derive(Debug)]
pub struct MutabilityDiagnostic {
    pub function: String,
    pub parameter: String,
    pub inferred_mutability: MutabilityKind,
    pub confidence: Confidence,
    pub evidence: Vec<Evidence>,
    pub alternative_suggestions: Vec<AlternativeSuggestion>,
}

/// Evidence supporting a mutability decision
#[derive(Debug)]
pub enum Evidence {
    DirectMutation {
        location: Location,
        kind: String,
        field_path: Vec<String>,
    },
    CalleeMutation {
        callee: String,
        location: Location,
    },
    ReadOnly {
        location: Location,
    },
    Unused,
}

/// Alternative mutability suggestion
#[derive(Debug)]
pub struct AlternativeSuggestion {
    pub mutability: MutabilityKind,
    pub rationale: String,
    pub confidence_score: f32,
}

/// Diagnostic generator
pub struct DiagnosticsGenerator<'a> {
    signatures: &'a HashMap<String, MutabilitySignature>,
}

impl<'a> DiagnosticsGenerator<'a> {
    pub fn new(signatures: &'a HashMap<String, MutabilitySignature>) -> Self {
        Self { signatures }
    }

    /// Generate diagnostics for all functions
    pub fn generate_all(&self) -> Vec<MutabilityDiagnostic> {
        let mut diagnostics = Vec::new();

        for (func_name, signature) in self.signatures {
            for param in &signature.parameters {
                diagnostics.push(self.generate_for_param(func_name, param));
            }
        }

        diagnostics
    }

    /// Generate diagnostic for a specific parameter
    pub fn generate_for_param(
        &self,
        func_name: &str,
        param: &super::flow_analyzer::ParameterMutability,
    ) -> MutabilityDiagnostic {
        let evidence = self.build_evidence(&param.rationale);
        let alternatives = self.suggest_alternatives(&param.mutability, &evidence);

        MutabilityDiagnostic {
            function: func_name.to_string(),
            parameter: param.name.clone(),
            inferred_mutability: param.mutability.clone(),
            confidence: param.confidence.clone(),
            evidence,
            alternative_suggestions: alternatives,
        }
    }

    /// Build evidence list from rationale
    fn build_evidence(&self, rationale: &[MutabilityReason]) -> Vec<Evidence> {
        rationale
            .iter()
            .map(|reason| match reason {
                MutabilityReason::DirectMutation {
                    location,
                    field_path,
                } => Evidence::DirectMutation {
                    location: location.clone(),
                    kind: "field write".to_string(),
                    field_path: field_path.clone(),
                },
                MutabilityReason::MutatingMethodCall { location, method } => {
                    Evidence::DirectMutation {
                        location: location.clone(),
                        kind: format!("method call: {}", method),
                        field_path: Vec::new(),
                    }
                }
                MutabilityReason::PassedToMutableParam {
                    callee, location, ..
                } => Evidence::CalleeMutation {
                    callee: callee.clone(),
                    location: location.clone(),
                },
                MutabilityReason::OnlyRead => Evidence::ReadOnly {
                    location: Location::unknown(),
                },
                MutabilityReason::Unused => Evidence::Unused,
                _ => Evidence::ReadOnly {
                    location: Location::unknown(),
                },
            })
            .collect()
    }

    /// Suggest alternative mutability options
    fn suggest_alternatives(
        &self,
        current: &MutabilityKind,
        evidence: &[Evidence],
    ) -> Vec<AlternativeSuggestion> {
        let mut alternatives = Vec::new();

        match current {
            MutabilityKind::MutableBorrow => {
                // If currently &mut, suggest & if only read evidence
                let has_mutations = evidence.iter().any(|e| {
                    matches!(
                        e,
                        Evidence::DirectMutation { .. } | Evidence::CalleeMutation { .. }
                    )
                });

                if !has_mutations {
                    alternatives.push(AlternativeSuggestion {
                        mutability: MutabilityKind::SharedBorrow,
                        rationale: "No direct mutations found, could use immutable borrow"
                            .to_string(),
                        confidence_score: 0.8,
                    });
                }
            }
            MutabilityKind::SharedBorrow => {
                // If currently &, suggest owned if unused
                if evidence.iter().any(|e| matches!(e, Evidence::Unused)) {
                    alternatives.push(AlternativeSuggestion {
                        mutability: MutabilityKind::Owned,
                        rationale: "Parameter is unused, could take ownership".to_string(),
                        confidence_score: 0.5,
                    });
                }
            }
            MutabilityKind::Owned => {
                // If currently owned, suggest & if only read
                let has_reads = evidence
                    .iter()
                    .any(|e| matches!(e, Evidence::ReadOnly { .. }));

                if has_reads {
                    alternatives.push(AlternativeSuggestion {
                        mutability: MutabilityKind::SharedBorrow,
                        rationale: "Parameter is only read, could use immutable borrow".to_string(),
                        confidence_score: 0.7,
                    });
                }
            }
            MutabilityKind::Conditional { .. } => {
                // For conditional, suggest both & and &mut
                alternatives.push(AlternativeSuggestion {
                    mutability: MutabilityKind::MutableBorrow,
                    rationale: "Conservative choice for conditional mutation".to_string(),
                    confidence_score: 0.6,
                });
            }
        }

        alternatives
    }

    /// Export diagnostics as human-readable text
    pub fn export_text(&self) -> String {
        let diagnostics = self.generate_all();
        let mut output = String::new();

        for diag in diagnostics {
            output.push_str(&format!(
                "\n=== {} :: {} ===\n",
                diag.function, diag.parameter
            ));
            output.push_str(&format!("Inferred: {:?}\n", diag.inferred_mutability));
            output.push_str(&format!("Confidence: {:?}\n", diag.confidence));

            output.push_str("\nEvidence:\n");
            for evidence in &diag.evidence {
                match evidence {
                    Evidence::DirectMutation {
                        kind, field_path, ..
                    } => {
                        let path = if field_path.is_empty() {
                            String::new()
                        } else {
                            format!(" ({})", field_path.join("."))
                        };
                        output.push_str(&format!("  - Direct mutation: {}{}\n", kind, path));
                    }
                    Evidence::CalleeMutation { callee, .. } => {
                        output.push_str(&format!("  - Passed to mutating function: {}\n", callee));
                    }
                    Evidence::ReadOnly { .. } => {
                        output.push_str("  - Read-only access\n");
                    }
                    Evidence::Unused => {
                        output.push_str("  - Parameter is unused\n");
                    }
                }
            }

            if !diag.alternative_suggestions.is_empty() {
                output.push_str("\nAlternatives:\n");
                for alt in &diag.alternative_suggestions {
                    output.push_str(&format!(
                        "  - {:?} (confidence: {:.1}%): {}\n",
                        alt.mutability,
                        alt.confidence_score * 100.0,
                        alt.rationale
                    ));
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interprocedural::flow_analyzer::ParameterMutability;

    #[test]
    fn test_diagnostics_generation() {
        let mut signatures = HashMap::new();
        signatures.insert(
            "test_fn".to_string(),
            MutabilitySignature {
                function_name: "test_fn".to_string(),
                parameters: vec![ParameterMutability {
                    name: "x".to_string(),
                    mutability: MutabilityKind::SharedBorrow,
                    confidence: Confidence::High,
                    rationale: vec![MutabilityReason::OnlyRead],
                }],
            },
        );

        let generator = DiagnosticsGenerator::new(&signatures);
        let diagnostics = generator.generate_all();

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].function, "test_fn");
        assert_eq!(diagnostics[0].parameter, "x");
    }

    #[test]
    fn test_export_text() {
        let mut signatures = HashMap::new();
        signatures.insert(
            "test".to_string(),
            MutabilitySignature {
                function_name: "test".to_string(),
                parameters: vec![ParameterMutability {
                    name: "state".to_string(),
                    mutability: MutabilityKind::MutableBorrow,
                    confidence: Confidence::High,
                    rationale: vec![MutabilityReason::DirectMutation {
                        location: Location::unknown(),
                        field_path: vec!["value".to_string()],
                    }],
                }],
            },
        );

        let generator = DiagnosticsGenerator::new(&signatures);
        let text = generator.export_text();

        assert!(text.contains("test :: state"));
        assert!(text.contains("MutableBorrow"));
    }
}
