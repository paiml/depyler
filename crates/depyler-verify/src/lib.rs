pub mod contract_verification;
pub mod contracts;
pub mod lifetime_analysis;
pub mod lifetime_shim;
pub mod memory_safety;
pub mod memory_shim;
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
        Self { enable_quickcheck: true, enable_contracts: true, test_iterations: 1000 }
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

        // Property 2: Memory safety
        let mut memory_analyzer = memory_safety::MemorySafetyAnalyzer::new();
        results.push(memory_analyzer.analyze_function(func));

        // Property 2b: Lifetime safety
        let mut lifetime_analyzer = lifetime_analysis::LifetimeAnalyzer::new();
        let lifetime_violations = lifetime_analyzer.analyze_function(func);
        if lifetime_violations.is_empty() {
            results.push(VerificationResult {
                property: "lifetime_safety".into(),
                status: PropertyStatus::Proven,
                confidence: 1.0,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            });
        } else {
            results.push(VerificationResult {
                property: "lifetime_safety".into(),
                status: PropertyStatus::Violated(format!(
                    "{} lifetime violations found",
                    lifetime_violations.len()
                )),
                confidence: 1.0,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            });
        }

        // Property 3: Null safety
        let null_violations = memory_safety::check_null_safety(func);
        if null_violations.is_empty() {
            results.push(VerificationResult {
                property: "null_safety".into(),
                status: PropertyStatus::Proven,
                confidence: 1.0,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            });
        } else {
            results.push(VerificationResult {
                property: "null_safety".into(),
                status: PropertyStatus::Violated(format!(
                    "{} violations found",
                    null_violations.len()
                )),
                confidence: 1.0,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            });
        }

        // Property 4: Panic freedom
        if func.properties.panic_free {
            results.push(VerificationResult {
                property: "panic_free".into(),
                status: PropertyStatus::HighConfidence,
                confidence: 0.95,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            });
        }

        // Property 5: Termination
        if func.properties.always_terminates {
            results.push(VerificationResult {
                property: "termination".into(),
                status: PropertyStatus::Proven,
                confidence: 1.0,
                method: VerificationMethod::StructuralInduction,
                counterexamples: vec![],
            });
        }

        // Property 6: Purity
        if func.properties.is_pure {
            results.push(VerificationResult {
                property: "pure".into(),
                status: PropertyStatus::HighConfidence,
                confidence: 0.9,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            });
        }

        // Property 7: Thread safety (if required)
        if func.annotations.thread_safety == depyler_annotations::ThreadSafety::Required {
            results.push(self.verify_thread_safety(func));
        }

        results
    }

    fn verify_type_preservation(&self, func: &HirFunction) -> Option<VerificationResult> {
        // Check if all types are properly annotated
        let all_typed =
            func.params.iter().all(|param| !matches!(param.ty, depyler_core::hir::Type::Unknown));

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

    fn verify_thread_safety(&self, func: &HirFunction) -> VerificationResult {
        // Check for proper synchronization when thread safety is required
        let has_shared_state = self.detect_shared_state(func);
        let has_proper_sync = func.annotations.interior_mutability
            == depyler_annotations::InteriorMutability::ArcMutex;

        if !has_shared_state || has_proper_sync {
            VerificationResult {
                property: "thread_safety".into(),
                status: PropertyStatus::Proven,
                confidence: 0.95,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            }
        } else {
            VerificationResult {
                property: "thread_safety".into(),
                status: PropertyStatus::Violated(
                    "Shared state without proper synchronization".to_string(),
                ),
                confidence: 1.0,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            }
        }
    }

    fn detect_shared_state(&self, func: &HirFunction) -> bool {
        // Simplified check - in reality would analyze data flow
        func.annotations.ownership_model == depyler_annotations::OwnershipModel::Shared
    }

    pub fn generate_property_tests(&self, func: &HirFunction) -> Result<String> {
        let test_code = properties::generate_quickcheck_tests(func, self.test_iterations)?;
        Ok(test_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_annotations::{
        InteriorMutability, OwnershipModel, ThreadSafety, TranspilationAnnotations,
    };
    use depyler_core::hir::{HirExpr, HirStmt, Type};

    /// Helper function to create a test function
    fn create_test_function(name: &str, is_pure: bool, thread_safe: bool) -> HirFunction {
        use smallvec::smallvec;

        HirFunction {
            name: name.to_string(),
            params: smallvec![depyler_core::hir::HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: depyler_core::hir::FunctionProperties {
                is_pure,
                always_terminates: true,
                panic_free: true,
                max_stack_depth: None,
                can_fail: false,
                error_types: vec![],
                is_async: false,
                is_generator: false,
            },
            annotations: TranspilationAnnotations {
                thread_safety: if thread_safe {
                    ThreadSafety::Required
                } else {
                    ThreadSafety::NotRequired
                },
                ownership_model: OwnershipModel::Owned,
                interior_mutability: InteriorMutability::None,
                ..Default::default()
            },
            docstring: None,
        }
    }

    #[test]
    fn test_property_verifier_creation() {
        let verifier = PropertyVerifier::new();
        assert!(verifier.enable_quickcheck);
        assert!(verifier.enable_contracts);
        assert_eq!(verifier.test_iterations, 1000);
    }

    #[test]
    fn test_with_iterations() {
        let verifier = PropertyVerifier::new().with_iterations(5000);
        assert_eq!(verifier.test_iterations, 5000);
    }

    #[test]
    fn test_verify_pure_function() {
        let verifier = PropertyVerifier::new();
        let func = create_test_function("pure_func", true, false);

        let results = verifier.verify_function(&func);

        // Should have multiple verification results
        assert!(!results.is_empty());

        // Find the purity result
        let purity_result = results.iter().find(|r| r.property == "pure");
        assert!(purity_result.is_some());

        let result = purity_result.unwrap();
        assert!(matches!(result.status, PropertyStatus::HighConfidence));
        assert!(result.confidence >= 0.9);
    }

    #[test]
    fn test_verify_thread_safe_function() {
        let verifier = PropertyVerifier::new();
        let func = create_test_function("thread_safe_func", false, true);

        let results = verifier.verify_function(&func);

        // Should include thread safety verification
        let thread_safety_result = results.iter().find(|r| r.property == "thread_safety");
        assert!(thread_safety_result.is_some());
    }

    #[test]
    fn test_type_preservation_verification() {
        let verifier = PropertyVerifier::new();
        let mut func = create_test_function("typed_func", false, false);

        // Test with fully typed function
        let results = verifier.verify_function(&func);
        let type_result = results.iter().find(|r| r.property == "type_preservation").unwrap();
        assert!(matches!(type_result.status, PropertyStatus::Proven));

        // Test with unknown types
        func.ret_type = Type::Unknown;
        let results = verifier.verify_function(&func);
        let type_result = results.iter().find(|r| r.property == "type_preservation").unwrap();
        assert!(matches!(type_result.status, PropertyStatus::Unknown));
    }

    #[test]
    fn test_memory_safety_verification() {
        let verifier = PropertyVerifier::new();
        let func = create_test_function("memory_safe_func", false, false);

        let results = verifier.verify_function(&func);

        // Should include memory safety checks
        let memory_result = results.iter().find(|r| r.property == "memory_safety");
        assert!(memory_result.is_some());

        let null_result = results.iter().find(|r| r.property == "null_safety");
        assert!(null_result.is_some());
        assert!(matches!(null_result.unwrap().status, PropertyStatus::Proven));
    }

    #[test]
    fn test_property_status_serialization() {
        use serde_json;

        let statuses = vec![
            PropertyStatus::Proven,
            PropertyStatus::HighConfidence,
            PropertyStatus::Likely,
            PropertyStatus::Unknown,
            PropertyStatus::Violated("test violation".to_string()),
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: PropertyStatus = serde_json::from_str(&serialized).unwrap();

            match (&status, &deserialized) {
                (PropertyStatus::Violated(s1), PropertyStatus::Violated(s2)) => assert_eq!(s1, s2),
                _ => assert_eq!(
                    std::mem::discriminant(&status),
                    std::mem::discriminant(&deserialized)
                ),
            }
        }
    }

    #[test]
    fn test_verification_result_creation() {
        let result = VerificationResult {
            property: "test_property".to_string(),
            status: PropertyStatus::Proven,
            confidence: 1.0,
            method: VerificationMethod::Exhaustive,
            counterexamples: vec![],
        };

        assert_eq!(result.property, "test_property");
        assert!(matches!(result.status, PropertyStatus::Proven));
        assert_eq!(result.confidence, 1.0);
        assert!(result.counterexamples.is_empty());
    }

    // ========================================================================
    // Session 11 - Deep Coverage Tests for Untested Paths
    // ========================================================================

    #[test]
    fn test_verify_thread_safety_violated_shared_no_sync() {
        let verifier = PropertyVerifier::new();
        use smallvec::smallvec;
        let func = HirFunction {
            name: "unsafe_shared".to_string(),
            params: smallvec![depyler_core::hir::HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: Default::default(),
            annotations: TranspilationAnnotations {
                thread_safety: ThreadSafety::Required,
                ownership_model: OwnershipModel::Shared,
                interior_mutability: InteriorMutability::None,
                ..Default::default()
            },
            docstring: None,
        };

        let results = verifier.verify_function(&func);
        let thread_result = results.iter().find(|r| r.property == "thread_safety");
        assert!(thread_result.is_some());
        assert!(matches!(thread_result.unwrap().status, PropertyStatus::Violated(_)));
    }

    #[test]
    fn test_verify_thread_safety_proven_with_arc_mutex() {
        let verifier = PropertyVerifier::new();
        use smallvec::smallvec;
        let func = HirFunction {
            name: "safe_shared".to_string(),
            params: smallvec![depyler_core::hir::HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: Default::default(),
            annotations: TranspilationAnnotations {
                thread_safety: ThreadSafety::Required,
                ownership_model: OwnershipModel::Shared,
                interior_mutability: InteriorMutability::ArcMutex,
                ..Default::default()
            },
            docstring: None,
        };

        let results = verifier.verify_function(&func);
        let thread_result = results.iter().find(|r| r.property == "thread_safety");
        assert!(thread_result.is_some());
        assert!(matches!(thread_result.unwrap().status, PropertyStatus::Proven));
    }

    #[test]
    fn test_verify_function_no_pure_no_terminates() {
        let verifier = PropertyVerifier::new();
        use smallvec::smallvec;
        let func = HirFunction {
            name: "impure".to_string(),
            params: smallvec![depyler_core::hir::HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: depyler_core::hir::FunctionProperties {
                is_pure: false,
                always_terminates: false,
                panic_free: false,
                ..Default::default()
            },
            annotations: Default::default(),
            docstring: None,
        };

        let results = verifier.verify_function(&func);
        // Should NOT have purity, termination, or panic_free results
        assert!(!results.iter().any(|r| r.property == "pure"));
        assert!(!results.iter().any(|r| r.property == "termination"));
        assert!(!results.iter().any(|r| r.property == "panic_free"));
    }

    #[test]
    fn test_verify_type_preservation_unknown_param() {
        let verifier = PropertyVerifier::new();
        use smallvec::smallvec;
        let func = HirFunction {
            name: "untyped_param".to_string(),
            params: smallvec![depyler_core::hir::HirParam::new("x".to_string(), Type::Unknown)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let results = verifier.verify_function(&func);
        let type_result = results.iter().find(|r| r.property == "type_preservation").unwrap();
        assert!(matches!(type_result.status, PropertyStatus::Unknown));
        assert_eq!(type_result.confidence, 0.0);
    }

    #[test]
    fn test_detect_shared_state_owned() {
        let verifier = PropertyVerifier::new();
        use smallvec::smallvec;
        let func = HirFunction {
            name: "owned".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![],
            properties: Default::default(),
            annotations: TranspilationAnnotations {
                ownership_model: OwnershipModel::Owned,
                ..Default::default()
            },
            docstring: None,
        };
        assert!(!verifier.detect_shared_state(&func));
    }

    #[test]
    fn test_detect_shared_state_shared() {
        let verifier = PropertyVerifier::new();
        use smallvec::smallvec;
        let func = HirFunction {
            name: "shared".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![],
            properties: Default::default(),
            annotations: TranspilationAnnotations {
                ownership_model: OwnershipModel::Shared,
                ..Default::default()
            },
            docstring: None,
        };
        assert!(verifier.detect_shared_state(&func));
    }

    #[test]
    fn test_generate_property_tests() {
        let verifier = PropertyVerifier::new().with_iterations(100);
        use smallvec::smallvec;
        let func = HirFunction {
            name: "add".to_string(),
            params: smallvec![
                depyler_core::hir::HirParam::new("a".to_string(), Type::Int),
                depyler_core::hir::HirParam::new("b".to_string(), Type::Int),
            ],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("a".to_string())))],
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let test_code = verifier.generate_property_tests(&func);
        assert!(test_code.is_ok());
        let code = test_code.unwrap();
        assert!(!code.is_empty());
        assert!(code.contains("add"));
    }

    #[test]
    fn test_verification_method_serialization() {
        use serde_json;
        let methods = vec![
            VerificationMethod::Exhaustive,
            VerificationMethod::PropertyTesting,
            VerificationMethod::StaticAnalysis,
            VerificationMethod::StructuralInduction,
            VerificationMethod::Heuristic,
        ];

        for method in methods {
            let serialized = serde_json::to_string(&method).unwrap();
            let deserialized: VerificationMethod = serde_json::from_str(&serialized).unwrap();
            assert_eq!(std::mem::discriminant(&method), std::mem::discriminant(&deserialized));
        }
    }

    #[test]
    fn test_test_case_serialization() {
        use serde_json;
        let test_case = TestCase {
            inputs: vec![serde_json::json!(42), serde_json::json!("hello")],
            expected_output: Some(serde_json::json!(42)),
            actual_output: Some(serde_json::json!(43)),
            error: Some("mismatch".to_string()),
        };

        let serialized = serde_json::to_string(&test_case).unwrap();
        let deserialized: TestCase = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.inputs.len(), 2);
        assert!(deserialized.error.is_some());
    }

    #[test]
    fn test_test_case_empty() {
        use serde_json;
        let test_case =
            TestCase { inputs: vec![], expected_output: None, actual_output: None, error: None };

        let serialized = serde_json::to_string(&test_case).unwrap();
        let deserialized: TestCase = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.inputs.is_empty());
        assert!(deserialized.expected_output.is_none());
    }

    #[test]
    fn test_property_verifier_default_eq_new() {
        let v1 = PropertyVerifier::new();
        let v2 = PropertyVerifier::default();
        assert_eq!(v1.enable_quickcheck, v2.enable_quickcheck);
        assert_eq!(v1.enable_contracts, v2.enable_contracts);
        assert_eq!(v1.test_iterations, v2.test_iterations);
    }
}

/// Doctests for public API
///
/// # Example
/// ```
/// use depyler_verify::{PropertyVerifier, PropertyStatus};
/// use depyler_core::hir::{HirFunction, HirParam, Type};
/// use smallvec::smallvec;
///
/// let verifier = PropertyVerifier::new()
///     .with_iterations(100);
///
/// // Create a simple function to verify
/// let func = HirFunction {
///     name: "add".to_string(),
///     params: smallvec![
///         HirParam { name: "a".to_string(), ty: Type::Int, default: None, is_vararg: false },
///         HirParam { name: "b".to_string(), ty: Type::Int, default: None, is_vararg: false }
///     ],
///     ret_type: Type::Int,
///     body: vec![],
///     properties: Default::default(),
///     annotations: Default::default(),
///     docstring: None,
/// };
///
/// let results = verifier.verify_function(&func);
/// assert!(!results.is_empty());
/// ```
pub mod examples {}
