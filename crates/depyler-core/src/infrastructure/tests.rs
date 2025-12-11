//! EXTREME TDD Tests for Infrastructure Components (DEPYLER-0925)
//!
//! RED PHASE: All tests written BEFORE implementation.
//! These tests define the contract that implementations must satisfy.

use super::*;

// =============================================================================
// FAULT LOCALIZER TESTS (Tarantula Algorithm)
// Spec Reference: Part E, Component 1
// Citation: Jones & Harrold (2005) DOI: 10.1145/1101908.1101949
// =============================================================================

mod fault_localizer_tests {
    use super::*;

    #[test]
    fn test_DEPYLER_0925_tracer_captures_type_inference_decisions() {
        let mut localizer = FaultLocalizer::new();

        let decision = TranspilerDecision {
            id: 1,
            location: SourceLocation { file: "test.py".into(), line: 10, column: 5 },
            decision_type: DecisionType::TypeInference {
                inferred: "i32".into(),
                constraints: vec!["numeric".into(), "int_literal".into()],
            },
            input_context: "x = 42".into(),
            output_generated: "let x: i32 = 42;".into(),
            confidence: 0.95,
            timestamp_ns: 0,
        };

        localizer.record_decision(decision.clone());
        assert_eq!(localizer.decision_count(), 1);

        let recorded = localizer.get_decision(1).unwrap();
        assert_eq!(recorded.id, 1);
    }

    #[test]
    fn test_DEPYLER_0925_tracer_captures_ownership_choices() {
        let mut localizer = FaultLocalizer::new();

        let decision = TranspilerDecision {
            id: 2,
            location: SourceLocation { file: "test.py".into(), line: 20, column: 1 },
            decision_type: DecisionType::OwnershipChoice {
                strategy: "borrow".into(),
                reason: "read-only access".into(),
            },
            input_context: "def foo(x): return x + 1".into(),
            output_generated: "fn foo(x: &i32) -> i32".into(),
            confidence: 0.85,
            timestamp_ns: 100,
        };

        localizer.record_decision(decision);
        assert_eq!(localizer.decision_count(), 1);
    }

    #[test]
    fn test_DEPYLER_0925_tracer_captures_library_mappings() {
        let mut localizer = FaultLocalizer::new();

        let decision = TranspilerDecision {
            id: 3,
            location: SourceLocation { file: "numpy_test.py".into(), line: 5, column: 1 },
            decision_type: DecisionType::LibraryMapping {
                python_api: "numpy.array".into(),
                rust_api: "trueno::Vector".into(),
            },
            input_context: "arr = np.array([1, 2, 3])".into(),
            output_generated: "let arr = Vector::from_slice(&[1, 2, 3]);".into(),
            confidence: 0.90,
            timestamp_ns: 200,
        };

        localizer.record_decision(decision);
        assert_eq!(localizer.decision_count(), 1);
    }

    #[test]
    fn test_DEPYLER_0925_tarantula_suspiciousness_formula() {
        // Reference: Jones & Harrold (2005) Tarantula formula:
        // suspiciousness = (failed/total_failed) / (failed/total_failed + passed/total_passed)

        let mut localizer = FaultLocalizer::new();

        // Decision that appears in 3 failed, 1 passed
        localizer.record_pass(1);
        localizer.record_fail(1);
        localizer.record_fail(1);
        localizer.record_fail(1);

        // Total: 4 failed, 2 passed executions
        localizer.set_totals(4, 2);

        let susp = localizer.suspiciousness(1);

        // failed_ratio = 3/4 = 0.75
        // passed_ratio = 1/2 = 0.5
        // suspiciousness = 0.75 / (0.75 + 0.5) = 0.6
        assert!((susp - 0.6).abs() < 0.01, "Expected ~0.6, got {}", susp);
    }

    #[test]
    fn test_DEPYLER_0925_suspiciousness_ranking_stable() {
        let mut localizer = FaultLocalizer::new();

        // Decision 1: High suspiciousness (appears in many failures)
        for _ in 0..10 { localizer.record_fail(1); }
        localizer.record_pass(1);

        // Decision 2: Low suspiciousness (appears in many passes)
        localizer.record_fail(2);
        for _ in 0..10 { localizer.record_pass(2); }

        localizer.set_totals(11, 11);

        let ranked = localizer.rank_decisions();

        // Decision 1 should be ranked first (higher suspiciousness)
        assert_eq!(ranked[0].0, 1);
        assert_eq!(ranked[1].0, 2);

        // Verify determinism: run 100 times, same result
        for _ in 0..100 {
            let ranked2 = localizer.rank_decisions();
            assert_eq!(ranked2[0].0, 1);
            assert_eq!(ranked2[1].0, 2);
        }
    }

    #[test]
    fn test_DEPYLER_0925_tracer_handles_zero_failures() {
        let mut localizer = FaultLocalizer::new();
        localizer.record_pass(1);
        localizer.set_totals(0, 1);

        // Should not panic, return 0 suspiciousness
        let susp = localizer.suspiciousness(1);
        assert_eq!(susp, 0.0);
    }
}

// =============================================================================
// PATTERN STORE TESTS (HNSW Semantic Search)
// Spec Reference: Part E, Component 2
// Citation: Malkov & Yashunin (2020) DOI: 10.1109/TPAMI.2018.2889473
// =============================================================================

mod pattern_store_tests {
    use super::*;

    #[test]
    fn test_DEPYLER_0925_pattern_store_persistence() {
        let mut store = PatternStore::new();

        let pattern = TranspilationPattern {
            id: "numpy-array-f32".into(),
            python_pattern: "np.array([...])".into(),
            rust_output: "Vector::from_slice(&[...])".into(),
            error_prevented: "E0308".into(),
            confidence: 0.95,
            usage_count: 10,
            success_rate: 0.9,
            embedding: vec![0.1; 384], // 384-dim embedding
        };

        store.add_pattern(pattern.clone());

        // Serialize and deserialize
        let serialized = store.serialize().unwrap();
        let loaded = PatternStore::deserialize(&serialized).unwrap();

        let retrieved = loaded.get_pattern("numpy-array-f32").unwrap();
        assert_eq!(retrieved.id, pattern.id);
        assert_eq!(retrieved.confidence, pattern.confidence);
    }

    #[test]
    fn test_DEPYLER_0925_hnsw_recall_at_10() {
        let mut store = PatternStore::new();

        // Add 100 patterns with random embeddings
        for i in 0..100 {
            let embedding: Vec<f32> = (0..384).map(|j| ((i * 17 + j) % 100) as f32 / 100.0).collect();
            store.add_pattern(TranspilationPattern {
                id: format!("pattern-{}", i),
                python_pattern: format!("pattern_{}", i),
                rust_output: format!("output_{}", i),
                error_prevented: "E0308".into(),
                confidence: 0.8,
                usage_count: 1,
                success_rate: 0.8,
                embedding,
            });
        }

        // Query with pattern-50's embedding
        let query_embedding: Vec<f32> = (0..384).map(|j| ((50 * 17 + j) % 100) as f32 / 100.0).collect();
        let results = store.find_similar(&query_embedding, 10);

        // Recall@10: pattern-50 should be in top 10 results
        let ids: Vec<_> = results.iter().map(|p| p.id.as_str()).collect();
        assert!(ids.contains(&"pattern-50"), "Expected pattern-50 in results: {:?}", ids);
    }

    #[test]
    fn test_DEPYLER_0925_cosine_similarity_edge_cases() {
        let store = PatternStore::new();

        // Zero vector
        let zero = vec![0.0f32; 384];
        let nonzero = vec![1.0f32; 384];

        let sim = store.cosine_similarity(&zero, &nonzero);
        assert!(sim.is_finite(), "Zero vector should not produce NaN/Inf");

        // Identical vectors
        let sim_identical = store.cosine_similarity(&nonzero, &nonzero);
        assert!((sim_identical - 1.0).abs() < 0.001, "Identical vectors should have similarity ~1.0");
    }

    #[test]
    fn test_DEPYLER_0925_pattern_confidence_update() {
        let mut store = PatternStore::new();

        store.add_pattern(TranspilationPattern {
            id: "test-pattern".into(),
            python_pattern: "test".into(),
            rust_output: "test".into(),
            error_prevented: "E0308".into(),
            confidence: 0.5,
            usage_count: 0,
            success_rate: 0.5,
            embedding: vec![0.0; 384],
        });

        // Update with successes
        for _ in 0..10 {
            store.update_confidence("test-pattern", true);
        }

        let pattern = store.get_pattern("test-pattern").unwrap();

        // Confidence should increase toward 1.0
        assert!(pattern.confidence > 0.5, "Confidence should increase with successes");
        assert!(pattern.usage_count == 10, "Usage count should be 10");
    }

    #[test]
    fn test_DEPYLER_0925_pattern_store_lookup_performance() {
        use std::time::Instant;

        let mut store = PatternStore::new();

        // Add 10,000 patterns
        for i in 0..10_000 {
            let embedding: Vec<f32> = (0..384).map(|j| ((i * 13 + j) % 1000) as f32 / 1000.0).collect();
            store.add_pattern(TranspilationPattern {
                id: format!("pattern-{}", i),
                python_pattern: format!("p{}", i),
                rust_output: format!("o{}", i),
                error_prevented: "E0308".into(),
                confidence: 0.8,
                usage_count: 1,
                success_rate: 0.8,
                embedding,
            });
        }

        let query: Vec<f32> = (0..384).map(|j| (j % 100) as f32 / 100.0).collect();

        let start = Instant::now();
        let _results = store.find_similar(&query, 10);
        let elapsed = start.elapsed();

        // Must complete in < 200ms for 10K patterns (brute-force acceptable for MVP)
        // TODO: Implement HNSW for O(log n) and reduce threshold to <10ms
        assert!(elapsed.as_millis() < 200, "Lookup took {}ms, expected <200ms", elapsed.as_millis());
    }
}

// =============================================================================
// CURRICULUM SCHEDULER TESTS (EASY→HARD Ordering)
// Spec Reference: Part E, Component 3
// Citation: Bengio et al. (2009) DOI: 10.1145/1553374.1553380
// =============================================================================

mod curriculum_tests {
    use super::*;

    #[test]
    fn test_DEPYLER_0925_scheduler_respects_difficulty_ordering() {
        let mut scheduler = CurriculumScheduler::new();

        // Add in random order
        scheduler.add_example(FailingExample {
            path: "hard.py".into(),
            errors: vec![
                CompilationError { code: "E0308".into(), message: "type mismatch".into() },
                CompilationError { code: "E0277".into(), message: "trait bound".into() },
                CompilationError { code: "E0425".into(), message: "not found".into() },
            ],
            difficulty: DifficultyLevel::Hard,
            cluster_id: None,
            dependencies: vec![],
        });

        scheduler.add_example(FailingExample {
            path: "easy.py".into(),
            errors: vec![
                CompilationError { code: "E0308".into(), message: "type mismatch".into() },
            ],
            difficulty: DifficultyLevel::Easy,
            cluster_id: Some(1),
            dependencies: vec![],
        });

        scheduler.add_example(FailingExample {
            path: "medium.py".into(),
            errors: vec![
                CompilationError { code: "E0308".into(), message: "type mismatch".into() },
                CompilationError { code: "E0277".into(), message: "trait bound".into() },
            ],
            difficulty: DifficultyLevel::Medium,
            cluster_id: None,
            dependencies: vec![],
        });

        // Easy should come first (highest priority)
        let first = scheduler.pop_next().unwrap();
        assert_eq!(first.path, "easy.py", "Easy should be processed first");

        let second = scheduler.pop_next().unwrap();
        assert_eq!(second.path, "medium.py", "Medium should be second");

        let third = scheduler.pop_next().unwrap();
        assert_eq!(third.path, "hard.py", "Hard should be last");
    }

    #[test]
    fn test_DEPYLER_0925_cluster_bonus_applied() {
        let mut scheduler = CurriculumScheduler::new();

        // Same difficulty, but one has cluster membership
        scheduler.add_example(FailingExample {
            path: "no_cluster.py".into(),
            errors: vec![CompilationError { code: "E0308".into(), message: "".into() }],
            difficulty: DifficultyLevel::Medium,
            cluster_id: None,
            dependencies: vec![],
        });

        scheduler.add_example(FailingExample {
            path: "with_cluster.py".into(),
            errors: vec![CompilationError { code: "E0308".into(), message: "".into() }],
            difficulty: DifficultyLevel::Medium,
            cluster_id: Some(5), // NumPy cluster
            dependencies: vec![],
        });

        // Clustered example should come first (fixes multiple examples)
        let first = scheduler.pop_next().unwrap();
        assert_eq!(first.path, "with_cluster.py", "Clustered example should have priority");
    }

    #[test]
    fn test_DEPYLER_0925_dependency_penalty_applied() {
        let mut scheduler = CurriculumScheduler::new();

        scheduler.add_example(FailingExample {
            path: "no_deps.py".into(),
            errors: vec![CompilationError { code: "E0308".into(), message: "".into() }],
            difficulty: DifficultyLevel::Easy,
            cluster_id: None,
            dependencies: vec![],
        });

        scheduler.add_example(FailingExample {
            path: "with_deps.py".into(),
            errors: vec![CompilationError { code: "E0308".into(), message: "".into() }],
            difficulty: DifficultyLevel::Easy,
            cluster_id: None,
            dependencies: vec!["pattern-a".into(), "pattern-b".into()],
        });

        // No dependencies should come first
        let first = scheduler.pop_next().unwrap();
        assert_eq!(first.path, "no_deps.py", "Example without dependencies should have priority");
    }

    #[test]
    fn test_DEPYLER_0925_graduation_tracking() {
        let mut scheduler = CurriculumScheduler::new();

        scheduler.add_example(FailingExample {
            path: "test1.py".into(),
            errors: vec![],
            difficulty: DifficultyLevel::Easy,
            cluster_id: None,
            dependencies: vec![],
        });

        scheduler.add_example(FailingExample {
            path: "test2.py".into(),
            errors: vec![],
            difficulty: DifficultyLevel::Easy,
            cluster_id: None,
            dependencies: vec![],
        });

        assert_eq!(scheduler.progress(), 0.0);

        let _ = scheduler.pop_next();
        scheduler.graduate("test1.py".into());

        assert!((scheduler.progress() - 0.5).abs() < 0.01, "Progress should be 50%");
    }

    #[test]
    fn test_DEPYLER_0925_scheduler_handles_1000_examples() {
        let mut scheduler = CurriculumScheduler::new();

        for i in 0..1000 {
            scheduler.add_example(FailingExample {
                path: format!("example_{}.py", i),
                errors: vec![CompilationError { code: "E0308".into(), message: "".into() }],
                difficulty: if i % 4 == 0 { DifficultyLevel::Easy }
                    else if i % 4 == 1 { DifficultyLevel::Medium }
                    else if i % 4 == 2 { DifficultyLevel::Hard }
                    else { DifficultyLevel::Expert },
                cluster_id: if i % 3 == 0 { Some((i % 5) as u32) } else { None },
                dependencies: vec![],
            });
        }

        // Should not OOM, should maintain ordering
        let mut easy_count = 0;
        for _ in 0..100 {
            if let Some(example) = scheduler.pop_next() {
                if example.difficulty == DifficultyLevel::Easy {
                    easy_count += 1;
                }
            }
        }

        // Easy examples should dominate early processing
        assert!(easy_count > 50, "Expected >50 easy examples in first 100, got {}", easy_count);
    }
}

// =============================================================================
// KNOWLEDGE DISTILLER TESTS (Pattern Graduation)
// Spec Reference: Part E, Component 4
// Citation: Hinton et al. (2015) arXiv:1503.02531
// =============================================================================

mod distiller_tests {
    use super::*;

    #[test]
    fn test_DEPYLER_0925_graduation_criteria_enforced() {
        let distiller = KnowledgeDistiller::new(GraduationCriteria::default());

        // Pattern that meets all criteria
        let good_pattern = TranspilationPattern {
            id: "mature-pattern".into(),
            python_pattern: "np.mean(arr)".into(),
            rust_output: "arr.mean()".into(),
            error_prevented: "E0308".into(),
            confidence: 0.96, // >= 0.95
            usage_count: 55,  // >= 50
            success_rate: 0.995, // >= 0.99
            embedding: vec![],
        };

        assert!(distiller.ready_for_graduation(&good_pattern));

        // Pattern that fails confidence
        let low_confidence = TranspilationPattern {
            confidence: 0.90, // < 0.95
            ..good_pattern.clone()
        };
        assert!(!distiller.ready_for_graduation(&low_confidence));

        // Pattern that fails usage count
        let low_usage = TranspilationPattern {
            usage_count: 30, // < 50
            ..good_pattern.clone()
        };
        assert!(!distiller.ready_for_graduation(&low_usage));

        // Pattern that fails success rate
        let low_success = TranspilationPattern {
            success_rate: 0.95, // < 0.99
            ..good_pattern.clone()
        };
        assert!(!distiller.ready_for_graduation(&low_success));
    }

    #[test]
    fn test_DEPYLER_0925_generated_rules_compile() {
        let distiller = KnowledgeDistiller::new(GraduationCriteria::default());

        let pattern = TranspilationPattern {
            id: "test-rule".into(),
            python_pattern: "len(x)".into(),
            rust_output: "x.len()".into(),
            error_prevented: "E0599".into(),
            confidence: 0.98,
            usage_count: 100,
            success_rate: 0.99,
            embedding: vec![],
        };

        let rule_code = distiller.generate_rule(&pattern);

        // Should contain function definition
        assert!(rule_code.contains("fn handle_pattern_test_rule"), "Missing function definition");
        assert!(rule_code.contains("confidence: 0.98"), "Missing confidence annotation");
        assert!(rule_code.contains("uses: 100"), "Missing usage count");

        // Should be valid Rust syntax (basic check)
        assert!(rule_code.contains("->"), "Missing return type arrow");
        assert!(rule_code.contains("{"), "Missing opening brace");
        assert!(rule_code.contains("}"), "Missing closing brace");
    }

    #[test]
    fn test_DEPYLER_0925_find_graduation_candidates() {
        let mut store = PatternStore::new();
        let distiller = KnowledgeDistiller::new(GraduationCriteria::default());

        // Add mature pattern
        store.add_pattern(TranspilationPattern {
            id: "mature".into(),
            python_pattern: "mature".into(),
            rust_output: "mature".into(),
            error_prevented: "E0308".into(),
            confidence: 0.98,
            usage_count: 100,
            success_rate: 0.995,
            embedding: vec![0.0; 384],
        });

        // Add immature pattern
        store.add_pattern(TranspilationPattern {
            id: "immature".into(),
            python_pattern: "immature".into(),
            rust_output: "immature".into(),
            error_prevented: "E0308".into(),
            confidence: 0.7,
            usage_count: 5,
            success_rate: 0.8,
            embedding: vec![0.0; 384],
        });

        let candidates = distiller.find_graduation_candidates(&store);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].id, "mature");
    }

    #[test]
    fn test_DEPYLER_0925_distiller_idiomatic_output() {
        let distiller = KnowledgeDistiller::new(GraduationCriteria::default());

        let pattern = TranspilationPattern {
            id: "snake-case-pattern".into(),
            python_pattern: "test".into(),
            rust_output: "test".into(),
            error_prevented: "E0308".into(),
            confidence: 0.99,
            usage_count: 200,
            success_rate: 0.999,
            embedding: vec![],
        };

        let rule = distiller.generate_rule(&pattern);

        // Should use snake_case for function names (idiomatic Rust)
        assert!(rule.contains("handle_pattern_snake_case_pattern"), "Should use snake_case");
        assert!(!rule.contains("handle_pattern_snake-case-pattern"), "Should not have hyphens");
    }
}
