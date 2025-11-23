//! Bidirectional flow analysis for interprocedural mutability inference
//!
//! This module implements precise mutability inference across function boundaries
//! using a bidirectional flow analysis approach:
//! - Bottom-up: Compute minimal mutability requirements starting from leaves
//! - Top-down: Refine based on caller requirements
//! - Fixed-point: Handle recursive functions with iterative refinement

use super::call_graph::CallGraph;
use super::intraprocedural::{IntraproceduralSummary, LocalMutability, Location};
use std::collections::HashMap;

/// Kind of mutability for a parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutabilityKind {
    /// Take ownership (T)
    Owned,
    /// Immutable borrow (&T)
    SharedBorrow,
    /// Mutable borrow (&mut T)
    MutableBorrow,
    /// Conditional mutability (for future enhancement)
    Conditional {
        mutable_paths: Vec<String>,
        immutable_paths: Vec<String>,
    },
}

/// Confidence level in the mutability inference
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Confidence {
    High,   // Direct evidence from code
    Medium, // Inferred from call patterns
    Low,    // Conservative default
}

/// Reason for a mutability decision
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutabilityReason {
    DirectMutation {
        location: Location,
        field_path: Vec<String>,
    },
    MutatingMethodCall {
        location: Location,
        method: String,
    },
    PassedToMutableParam {
        callee: String,
        param: String,
        location: Location,
    },
    ReturnedMutably {
        location: Location,
    },
    Aliased {
        alias_name: String,
        mutated: bool,
    },
    ConservativeDefault,
    OnlyRead,
    Unused,
}

/// Mutability information for a single parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterMutability {
    pub name: String,
    pub mutability: MutabilityKind,
    pub confidence: Confidence,
    pub rationale: Vec<MutabilityReason>,
}

/// Complete mutability signature for a function
#[derive(Debug, Clone)]
pub struct MutabilitySignature {
    pub function_name: String,
    pub parameters: Vec<ParameterMutability>,
}

impl MutabilitySignature {
    pub fn get_param(&self, name: &str) -> Option<&ParameterMutability> {
        self.parameters.iter().find(|p| p.name == name)
    }

    pub fn get_param_by_index(&self, idx: usize) -> Option<&ParameterMutability> {
        self.parameters.get(idx)
    }
}

/// Results of the complete flow analysis
#[derive(Debug)]
pub struct FlowAnalysisResults {
    signatures: HashMap<String, MutabilitySignature>,
    converged: bool,
    iterations: usize,
}

impl FlowAnalysisResults {
    pub fn new(
        signatures: HashMap<String, MutabilitySignature>,
        converged: bool,
        iterations: usize,
    ) -> Self {
        Self {
            signatures,
            converged,
            iterations,
        }
    }

    pub fn get_signature(&self, func_name: &str) -> Option<&MutabilitySignature> {
        self.signatures.get(func_name)
    }

    pub fn is_param_mutable(&self, func_name: &str, param_name: &str) -> bool {
        self.get_signature(func_name)
            .and_then(|sig| sig.get_param(param_name))
            .map(|p| matches!(p.mutability, MutabilityKind::MutableBorrow))
            .unwrap_or(false)
    }

    pub fn converged(&self) -> bool {
        self.converged
    }

    pub fn iterations(&self) -> usize {
        self.iterations
    }
}

/// Bidirectional flow analyzer
pub struct FlowAnalyzer<'a> {
    call_graph: &'a CallGraph,
    intraprocedural: HashMap<String, IntraproceduralSummary>,
    signatures: HashMap<String, MutabilitySignature>,
}

impl<'a> FlowAnalyzer<'a> {
    pub fn new(
        call_graph: &'a CallGraph,
        intraprocedural: HashMap<String, IntraproceduralSummary>,
    ) -> Self {
        Self {
            call_graph,
            intraprocedural,
            signatures: HashMap::new(),
        }
    }

    /// Run complete bidirectional analysis
    pub fn analyze(mut self) -> FlowAnalysisResults {
        // Bottom-up: Compute minimal mutability
        self.bottom_up_pass();

        // Top-down: Refine based on caller needs
        self.top_down_pass();

        // Fixed-point iteration for SCCs (recursive functions)
        let (converged, iterations) = self.fixpoint_iteration();

        FlowAnalysisResults::new(self.signatures, converged, iterations)
    }

    /// Bottom-up pass: start from leaves, compute minimal mutability
    fn bottom_up_pass(&mut self) {
        // Process functions in reverse topological order (leaves first)
        for func_name in self.call_graph.reverse_topological_order() {
            let signature = self.infer_signature_bottom_up(&func_name);
            self.signatures.insert(func_name, signature);
        }
    }

    /// Infer signature for a function using bottom-up analysis
    fn infer_signature_bottom_up(&self, func_name: &str) -> MutabilitySignature {
        let summary = match self.intraprocedural.get(func_name) {
            Some(s) => s,
            None => {
                // Function not found, return empty signature
                return MutabilitySignature {
                    function_name: func_name.to_string(),
                    parameters: Vec::new(),
                };
            }
        };

        let mut params = Vec::new();

        for param_usage in &summary.parameters {
            let (mutability, confidence, rationale) = match param_usage.minimal_mutability() {
                LocalMutability::NeedsMut => {
                    // Direct mutation → definitely needs &mut
                    let rationale = param_usage
                        .direct_mutations
                        .iter()
                        .map(|m| MutabilityReason::DirectMutation {
                            location: m.location.clone(),
                            field_path: m.field_path.clone(),
                        })
                        .collect();
                    (MutabilityKind::MutableBorrow, Confidence::High, rationale)
                }
                LocalMutability::CanBeShared => {
                    // No mutations, only reads → can use &
                    (
                        MutabilityKind::SharedBorrow,
                        Confidence::High,
                        vec![MutabilityReason::OnlyRead],
                    )
                }
                LocalMutability::PassedToCallees => {
                    // Has call sites → check callee requirements
                    self.infer_from_call_sites(param_usage)
                }
                LocalMutability::Unused => {
                    // Not used → can take ownership (simplest)
                    (
                        MutabilityKind::Owned,
                        Confidence::Medium,
                        vec![MutabilityReason::Unused],
                    )
                }
            };

            params.push(ParameterMutability {
                name: param_usage.name.clone(),
                mutability,
                confidence,
                rationale,
            });
        }

        MutabilitySignature {
            function_name: func_name.to_string(),
            parameters: params,
        }
    }

    /// Infer mutability from call sites
    fn infer_from_call_sites(
        &self,
        param_usage: &super::intraprocedural::ParameterUsageAnalysis,
    ) -> (MutabilityKind, Confidence, Vec<MutabilityReason>) {
        let mut needs_mut = false;
        let mut rationale = Vec::new();

        for call_site in &param_usage.call_sites {
            // Check if the callee needs &mut for this parameter
            if let Some(callee_sig) = self.signatures.get(&call_site.callee) {
                if let Some(callee_param) =
                    callee_sig.get_param_by_index(call_site.callee_param_position)
                {
                    match &callee_param.mutability {
                        MutabilityKind::MutableBorrow => {
                            needs_mut = true;
                            rationale.push(MutabilityReason::PassedToMutableParam {
                                callee: call_site.callee.clone(),
                                param: callee_param.name.clone(),
                                location: call_site.location.clone(),
                            });
                        }
                        MutabilityKind::SharedBorrow => {
                            // Read-only callee, doesn't affect our mutability
                        }
                        MutabilityKind::Owned => {
                            // Callee takes ownership - special case
                            // For now, treat as not needing &mut
                        }
                        MutabilityKind::Conditional { .. } => {
                            // Conservative: assume it might need &mut
                            needs_mut = true;
                            rationale.push(MutabilityReason::ConservativeDefault);
                        }
                    }
                }
            } else {
                // Callee not analyzed yet - be conservative
                needs_mut = true;
                rationale.push(MutabilityReason::ConservativeDefault);
            }
        }

        if needs_mut {
            (MutabilityKind::MutableBorrow, Confidence::Medium, rationale)
        } else if param_usage.has_reads() {
            (
                MutabilityKind::SharedBorrow,
                Confidence::High,
                vec![MutabilityReason::OnlyRead],
            )
        } else {
            (
                MutabilityKind::Owned,
                Confidence::Low,
                vec![MutabilityReason::Unused],
            )
        }
    }

    /// Top-down pass: refine based on caller requirements
    fn top_down_pass(&mut self) {
        // Process in topological order (roots first, leaves last)
        for func_name in self.call_graph.topological_order() {
            self.refine_signature_top_down(&func_name);
        }
    }

    /// Refine signature based on how callers use the function
    fn refine_signature_top_down(&mut self, func_name: &str) {
        // For now, this is a placeholder
        // In the future, we can refine based on:
        // - How the return value is used
        // - Whether callers always pass the same mutability
        // - Optimization opportunities
    }

    /// Fixed-point iteration for strongly connected components (recursive functions)
    fn fixpoint_iteration(&mut self) -> (bool, usize) {
        let max_iterations = 10;
        let mut iteration = 0;
        let mut changed = true;

        let sccs = self.call_graph.strongly_connected_components();

        while changed && iteration < max_iterations {
            changed = false;
            iteration += 1;

            // Process each SCC
            for scc in &sccs {
                if scc.len() <= 1 && !self.is_self_recursive(&scc[0]) {
                    // Not recursive, skip
                    continue;
                }

                // Re-analyze each function in the SCC
                for func_name in scc {
                    let old_sig = self.signatures.get(func_name.as_str()).cloned();
                    let new_sig = self.infer_signature_bottom_up(func_name);

                    let sigs_different =
                        match old_sig {
                            Some(ref old) => {
                                // Compare parameters
                                old.parameters.len() != new_sig.parameters.len()
                                    || old.parameters.iter().zip(&new_sig.parameters).any(
                                        |(a, b)| a.name != b.name || a.mutability != b.mutability,
                                    )
                            }
                            None => true,
                        };

                    if sigs_different {
                        self.signatures.insert(func_name.clone(), new_sig);
                        changed = true;
                    }
                }
            }
        }

        let converged = !changed || iteration == 0;

        if !converged {
            // Didn't converge, apply conservative defaults
            eprintln!(
                "Warning: Fixed-point iteration did not converge after {} iterations",
                max_iterations
            );
        }

        (converged, iteration)
    }

    /// Check if a function is self-recursive
    fn is_self_recursive(&self, func_name: &str) -> bool {
        self.call_graph.calls_itself(func_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutability_kind_equality() {
        assert_eq!(MutabilityKind::SharedBorrow, MutabilityKind::SharedBorrow);
        assert_ne!(MutabilityKind::SharedBorrow, MutabilityKind::MutableBorrow);
    }

    #[test]
    fn test_parameter_mutability() {
        let param = ParameterMutability {
            name: "state".to_string(),
            mutability: MutabilityKind::MutableBorrow,
            confidence: Confidence::High,
            rationale: vec![MutabilityReason::DirectMutation {
                location: Location::unknown(),
                field_path: vec!["value".to_string()],
            }],
        };

        assert_eq!(param.name, "state");
        assert_eq!(param.mutability, MutabilityKind::MutableBorrow);
    }

    #[test]
    fn test_mutability_signature() {
        let sig = MutabilitySignature {
            function_name: "test_fn".to_string(),
            parameters: vec![ParameterMutability {
                name: "x".to_string(),
                mutability: MutabilityKind::SharedBorrow,
                confidence: Confidence::High,
                rationale: vec![MutabilityReason::OnlyRead],
            }],
        };

        assert!(sig.get_param("x").is_some());
        assert!(sig.get_param("y").is_none());
        assert_eq!(
            sig.get_param("x").unwrap().mutability,
            MutabilityKind::SharedBorrow
        );
    }

    #[test]
    fn test_flow_analysis_results() {
        let mut signatures = HashMap::new();
        signatures.insert(
            "test".to_string(),
            MutabilitySignature {
                function_name: "test".to_string(),
                parameters: vec![],
            },
        );

        let results = FlowAnalysisResults::new(signatures, true, 1);
        assert!(results.converged());
        assert_eq!(results.iterations(), 1);
        assert!(results.get_signature("test").is_some());
        assert!(results.get_signature("nonexistent").is_none());
    }
}
