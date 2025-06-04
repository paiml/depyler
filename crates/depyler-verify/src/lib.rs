pub mod contracts;
pub mod properties;
pub mod quickcheck;

use anyhow::Result;
use depyler_core::hir::HirFunction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyVerifier {
    pub enable_quickcheck: bool,
    pub enable_contracts: bool,
    pub test_iterations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub property: String,
    pub status: PropertyStatus,
    pub confidence: f64,
    pub method: VerificationMethod,
    pub counterexamples: Vec<TestCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyStatus {
    Proven,
    HighConfidence,
    Likely,
    Unknown,
    Violated(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationMethod {
    Exhaustive,
    PropertyTesting,
    StaticAnalysis,
    StructuralInduction,
    Heuristic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub inputs: Vec<serde_json::Value>,
    pub expected_output: Option<serde_json::Value>,
    pub actual_output: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl Default for PropertyVerifier {
    fn default() -> Self {
        Self {
            enable_quickcheck: true,
            enable_contracts: true,
            test_iterations: 1000,
        }
    }
}

impl PropertyVerifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.test_iterations = iterations;
        self
    }

    pub fn verify_function(&self, func: &HirFunction) -> Vec<VerificationResult> {
        let mut results = vec![];

        // Property 1: Type preservation
        if let Some(result) = self.verify_type_preservation(func) {
            results.push(result);
        }

        // Property 2: Panic freedom
        if func.properties.panic_free {
            results.push(VerificationResult {
                property: "panic_free".into(),
                status: PropertyStatus::HighConfidence,
                confidence: 0.95,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            });
        }

        // Property 3: Termination
        if func.properties.always_terminates {
            results.push(VerificationResult {
                property: "termination".into(),
                status: PropertyStatus::Proven,
                confidence: 1.0,
                method: VerificationMethod::StructuralInduction,
                counterexamples: vec![],
            });
        }

        // Property 4: Purity
        if func.properties.is_pure {
            results.push(VerificationResult {
                property: "pure".into(),
                status: PropertyStatus::HighConfidence,
                confidence: 0.9,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            });
        }

        results
    }

    fn verify_type_preservation(&self, func: &HirFunction) -> Option<VerificationResult> {
        // Check if all types are properly annotated
        let all_typed = func
            .params
            .iter()
            .all(|(_, ty)| !matches!(ty, depyler_core::hir::Type::Unknown));

        if all_typed && !matches!(func.ret_type, depyler_core::hir::Type::Unknown) {
            Some(VerificationResult {
                property: "type_preservation".into(),
                status: PropertyStatus::Proven,
                confidence: 1.0,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            })
        } else {
            Some(VerificationResult {
                property: "type_preservation".into(),
                status: PropertyStatus::Unknown,
                confidence: 0.0,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            })
        }
    }

    pub fn generate_property_tests(&self, func: &HirFunction) -> Result<String> {
        let test_code = properties::generate_quickcheck_tests(func, self.test_iterations)?;
        Ok(test_code)
    }
}
