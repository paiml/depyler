# Specification: Depyler "Hunt Mode" (Automated Calibration)

**Version:** 1.2.0
**Status:** Proposed
**Target:** v3.5.0
**Last Updated:** 2025-12-05

## 1. Executive Summary

"Hunt Mode" is an automated, iterative calibration subsystem for `depyler`. Acknowledging the "80/20 rule" (Pareto Principle) [1] in transpilation—where 80% of code transforms easily but the remaining 20% relies on project-specific idioms—Hunt Mode does not aim for a universal perfect compiler. Instead, it treats the transpiler as an adaptive system that "overfits" to the specific idioms of a target codebase through rapid, automated TDD cycles.

### Toyota Way Alignment

Hunt Mode embodies the Toyota Production System (TPS) philosophy of **continuous improvement through systematic problem-solving** [13]. The core feedback loop mirrors Toyota's PDCA (Plan-Do-Check-Act) cycle [14], while the emphasis on immediate defect correction reflects the **Jidoka** (自働化) principle of "automation with a human touch" [15].

## 2. Philosophy: The "Good Enough" Compiler

Traditional compilers enforce strict correctness globally. `depyler` adopts a **Search-Based Software Engineering (SBSE)** [2] approach, where the goal is to maximize the compilation rate of a *specific* corpus. By generating minimal reproduction cases for local failures and applying heuristic fixes, Hunt Mode reduces the cost of migration without requiring manual compiler engineering for every edge case.

### 2.1 Kaizen (改善) - Continuous Improvement

**Implementation Detail:**
```rust
// crates/depyler-core/src/hunt_mode/kaizen_tracker.rs
pub struct KaizenMetrics {
    pub compilation_rate: f64,           // Current single-shot compile rate
    pub rate_delta: f64,                 // Improvement since last cycle
    pub cycles_since_improvement: u32,   // Plateau detection
    pub cumulative_fixes: u32,           // Total fixes applied
}

impl KaizenMetrics {
    /// Toyota Way: Small, incremental improvements compound
    /// Each TDD cycle should improve rate by at least 0.1%
    pub fn is_improving(&self) -> bool {
        self.rate_delta > 0.001 || self.cycles_since_improvement < 5
    }
}
```

The system measures improvement continuously, following the Toyota principle that "no process can be considered perfect but can always be improved" [16]. Each Hunt Mode cycle targets a single defect pattern, ensuring focused, measurable progress.

### 2.2 Genchi Genbutsu (現地現物) - Go and See

**Implementation Detail:**
```rust
// crates/depyler-core/src/hunt_mode/diagnostics.rs
pub struct FailureObservation {
    pub source_file: PathBuf,
    pub error_code: String,         // E0308, E0432, etc.
    pub error_message: String,
    pub generated_rust: String,     // Actual output - "go and see"
    pub expected_behavior: String,  // What we wanted
    pub root_cause_analysis: RootCause,
}

/// Toyota Way: Never rely on reports alone - observe the actual failure
pub fn observe_compilation_failure(
    python_source: &str,
    rust_output: &str,
) -> FailureObservation {
    // Direct observation of the defect, not abstracted metrics
    let actual_error = rustc_compile(rust_output);
    analyze_root_cause(python_source, rust_output, actual_error)
}
```

## 3. Architecture

Hunt Mode operates as a meta-loop around the core transpiler, utilizing `aprender` [11] for pattern recognition and `entrenar` [12] for adaptive optimization.

### 3.1 The Feedback Loop (PDCA Implementation)

```
┌─────────────────────────────────────────────────────────────────┐
│                    HUNT MODE PDCA CYCLE                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐    │
│   │  PLAN   │───▶│   DO    │───▶│  CHECK  │───▶│   ACT   │    │
│   │ (Hunt)  │    │(Isolate)│    │(Repair) │    │(Verify) │    │
│   └────┬────┘    └────┬────┘    └────┬────┘    └────┬────┘    │
│        │              │              │              │          │
│        ▼              ▼              ▼              ▼          │
│   Classify       Synthesize      Apply         Validate       │
│   Errors         Minimal         Mutator       & Commit       │
│   (aprender)     repro.py        Fix           (Andon)        │
│                                                                │
│   ◀────────────────── KAIZEN LOOP ──────────────────────────▶ │
└─────────────────────────────────────────────────────────────────┘
```

#### Phase 1: PLAN - Hunt (Diagnosis)

**Toyota Principle: Heijunka (平準化) - Level the Workload**

*   Scans the target Python codebase.
*   Uses `aprender::classifier` (RandomForest) to cluster compilation errors into high-level categories.
*   Selects the highest-frequency failure pattern (e.g., "missing `serde` import" or "dynamic tuple indexing").
*   **Levels the workload** by processing errors in frequency order, ensuring maximum impact per cycle [17].

**Implementation Detail:**
```rust
// crates/depyler-core/src/hunt_mode/planner.rs
pub struct HuntPlanner {
    error_clusters: Vec<ErrorCluster>,
    priority_queue: BinaryHeap<PrioritizedPattern>,
}

impl HuntPlanner {
    /// Heijunka: Process highest-impact patterns first
    /// Pareto principle: 20% of patterns cause 80% of failures
    pub fn select_next_target(&mut self) -> Option<FailurePattern> {
        // Sort by frequency * severity * estimated_fix_complexity
        self.priority_queue.pop().map(|p| p.pattern)
    }
}
```

#### Phase 2: DO - Isolate (Reproduction)

**Toyota Principle: Poka-Yoke (ポカヨケ) - Error-Proofing**

*   Synthesizes a minimal, self-contained Python file (`repro.py`) that exhibits the error.
*   Verifies the failure (Red state).
*   **Error-proofs the fix** by creating an automated regression test [18].

**Implementation Detail:**
```rust
// crates/depyler-core/src/hunt_mode/isolator.rs
pub struct MinimalReproducer {
    /// Poka-yoke: Every fix MUST have a failing test first
    pub fn synthesize_repro(&self, pattern: &FailurePattern) -> ReproCase {
        let minimal_python = self.minimize_to_pattern(pattern);

        // Verify this actually fails (TDD Red)
        assert!(
            !self.compiles(&minimal_python),
            "Poka-yoke violation: repro must fail before fix"
        );

        ReproCase {
            source: minimal_python,
            expected_error: pattern.error_code.clone(),
            created_at: Utc::now(),
        }
    }
}
```

#### Phase 3: CHECK - Repair (Heuristic Search)

**Toyota Principle: Jidoka (自働化) - Automation with Human Touch**

*   Searches a library of "Mutators" (code transformations) in `depyler-core`.
*   Applies fixes ranging from rigorous (type system adjustment) to pragmatic (fallback to `serde_json::Value` or `Any`).
*   Leverages `entrenar::citl` (Compiler-in-the-Loop) [12] to prioritize fixes that historically maximize compilation success in similar contexts.
*   Prioritizes **compilability** over performance [3].
*   **Jidoka**: System automatically stops when quality cannot be assured [15].

**Implementation Detail:**
```rust
// crates/depyler-core/src/hunt_mode/repair.rs
pub struct JidokaRepairEngine {
    mutators: Vec<Box<dyn Mutator>>,
    quality_threshold: f64,  // Minimum confidence before applying fix
}

impl JidokaRepairEngine {
    /// Jidoka: Stop the line if fix quality is uncertain
    pub fn attempt_repair(&mut self, repro: &ReproCase) -> RepairResult {
        for mutator in &self.mutators {
            let candidate_fix = mutator.apply(repro);
            let confidence = self.evaluate_fix_confidence(&candidate_fix);

            // Jidoka: Only proceed if quality is assured
            if confidence < self.quality_threshold {
                return RepairResult::NeedsHumanReview {
                    fix: candidate_fix,
                    confidence,
                    reason: "Low confidence - manual review required",
                };
            }

            if self.verify_fix(&candidate_fix) {
                return RepairResult::Success(candidate_fix);
            }
        }
        RepairResult::NoFixFound
    }
}
```

#### Phase 4: ACT - Verify (Validation)

**Toyota Principle: Andon (行灯) - Visual Control / Stop the Line**

*   Compiles the output.
*   Optionally runs property tests using `proptest` (integrated via `aprender` dev-deps).
*   If successful, commits the strategy to the local project configuration (`.depyler/config.toml`) or patches the core.
*   **Andon**: Visual feedback and immediate escalation on failure [19].

**Implementation Detail:**
```rust
// crates/depyler-core/src/hunt_mode/verifier.rs
pub struct AndonVerifier {
    /// Andon board: Real-time visibility into system state
    status: AndonStatus,
}

#[derive(Debug, Clone)]
pub enum AndonStatus {
    Green { compilation_rate: f64, message: String },
    Yellow { warnings: Vec<String>, needs_attention: bool },
    Red { error: String, cycle_halted: bool },
}

impl AndonVerifier {
    /// Andon: Immediate visibility and escalation
    pub fn verify_and_commit(&mut self, fix: &Fix, repro: &ReproCase) -> VerifyResult {
        // Compile the fixed output
        let compile_result = rustc_compile(&fix.rust_output);

        match compile_result {
            Ok(_) => {
                // Run property tests
                if let Err(prop_failure) = run_property_tests(&fix) {
                    self.status = AndonStatus::Yellow {
                        warnings: vec![format!("Property test failed: {}", prop_failure)],
                        needs_attention: true,
                    };
                    return VerifyResult::NeedsReview;
                }

                self.status = AndonStatus::Green {
                    compilation_rate: self.calculate_new_rate(),
                    message: format!("Fix {} applied successfully", fix.ticket_id),
                };

                self.commit_fix(fix);
                VerifyResult::Success
            }
            Err(e) => {
                // STOP THE LINE - fix did not work
                self.status = AndonStatus::Red {
                    error: e.to_string(),
                    cycle_halted: true,
                };
                VerifyResult::FixFailed(e)
            }
        }
    }
}
```

### 3.2 Custom Tuning (Local Overfitting)

**Toyota Principle: Hansei (反省) - Reflection**

Large codebases often contain unique "dialects" of Python. Hunt Mode explicitly supports **Multi-Objective Optimization** [4], balancing general correctness with project-specific pragmatism. It generates a `local_rules.rs` or configuration overlay that effectively "tunes" the compiler to the target project's idiosyncrasies, similar to Profile-Guided Optimization (PGO) [5].

**Implementation Detail:**
```rust
// crates/depyler-core/src/hunt_mode/hansei.rs
pub struct HanseiReflector {
    cycle_history: Vec<CycleOutcome>,
    lessons_learned: Vec<Lesson>,
}

impl HanseiReflector {
    /// Hansei: Reflect on what worked and what didn't
    pub fn reflect_on_cycle(&mut self, outcome: &CycleOutcome) -> Vec<Lesson> {
        let mut lessons = Vec::new();

        // What patterns emerged?
        if outcome.fix_type == FixType::TypeInference {
            lessons.push(Lesson {
                category: "type_system",
                observation: "Type inference failures cluster around Dict[str, Any]",
                action: "Prioritize serde_json::Value fallback for untyped dicts",
            });
        }

        // Five Whys analysis
        let root_causes = self.five_whys_analysis(&outcome.failure);
        for cause in root_causes {
            lessons.push(Lesson {
                category: "root_cause",
                observation: cause.description,
                action: cause.preventive_measure,
            });
        }

        self.lessons_learned.extend(lessons.clone());
        lessons
    }
}
```

### 3.3 Five Whys Integration

**Toyota Principle: Root Cause Analysis**

Every compilation failure triggers a "Five Whys" analysis [20] to identify the true root cause:

```rust
// crates/depyler-core/src/hunt_mode/five_whys.rs
pub struct FiveWhysAnalyzer;

impl FiveWhysAnalyzer {
    /// Example: E0308 type mismatch
    ///
    /// Why 1: `serde_json::Value` expected, `&str` found
    /// Why 2: json.loads() returns Value, function returns HashMap
    /// Why 3: return_type_needs_json_dict() not checking this case
    /// Why 4: json.loads() codegen ignores return type context
    /// Why 5: Missing integration between return type inference and stdlib codegen
    ///
    /// Root Cause: expr_gen.rs line 3816 hardcodes Value return type
    /// Fix: Check return_type_needs_json_dict() before generating
    pub fn analyze(&self, failure: &CompilationFailure) -> RootCauseChain {
        let mut chain = Vec::new();
        let mut current = failure.immediate_cause.clone();

        for depth in 1..=5 {
            let why = self.ask_why(&current, depth);
            chain.push(why.clone());

            if why.is_root_cause {
                break;
            }
            current = why.deeper_cause.unwrap_or(current);
        }

        RootCauseChain { whys: chain }
    }
}
```

## 4. Integration Points

*   **CLI:** `depyler hunt --target ./src --cycles 10`
*   **Oracle:** Uses `depyler-oracle` (powered by `aprender`) to classify error messages and suggest initial fix templates [6].
*   **Training:** Uses `entrenar` to finetune local LLaMA/LoRA models on the project's specific AST transformation patterns, enabling the "Agent" to suggest semantically correct fixes for unique idioms.
*   **Metrics Dashboard:** Real-time Andon board showing compilation rate, active defects, and cycle status.

### 4.1 CLI Integration

```bash
# Basic hunt mode - 10 cycles targeting ./src
depyler hunt --target ./src --cycles 10

# With Andon dashboard (real-time visualization)
depyler hunt --target ./src --cycles 50 --andon

# Export lessons learned (Hansei report)
depyler hunt --target ./src --cycles 10 --hansei-report ./reports/

# Five Whys analysis mode (deep root cause investigation)
depyler hunt --target ./src --five-whys --verbose
```

### 4.2 Configuration Schema

```toml
# .depyler/hunt-config.toml

[hunt]
max_cycles = 100
quality_threshold = 0.85          # Jidoka: minimum confidence
stop_on_plateau = true            # Andon: halt if no progress
plateau_threshold = 5             # Cycles without improvement

[kaizen]
target_rate = 0.80                # Goal: 80% single-shot compile
min_improvement_per_cycle = 0.001 # 0.1% minimum progress
report_interval = 10              # Cycles between reports

[jidoka]
human_review_threshold = 0.70     # Below this → needs manual review
auto_commit_threshold = 0.95      # Above this → auto-apply fix

[hansei]
enable_five_whys = true
lessons_database = ".depyler/lessons.db"
export_format = "markdown"
```

## 5. Scientific Foundation (Annotated Bibliography)

### Automated Program Repair & SBSE

[1] **Pareto, V.** (1896). *Cours d'économie politique*. Lausanne: F. Rouge.
> Foundation of the 80/20 rule. In software, 20% of the features (or idioms) cause 80% of the complexity. Hunt Mode targets the "long tail" of project-specific idioms.

[2] **Harman, M., & Jones, B. F.** (2001). "Search-based software engineering." *Information and Software Technology*, 43(14), 833-839. https://doi.org/10.1016/S0950-5849(01)00189-6
> Establishes the field of applying metaheuristic search techniques (like Hunt Mode's loop) to software engineering problems, treating compilation as an optimization problem.

[3] **Le Goues, C., Nguyen, T., Forrest, S., & Weimer, W.** (2012). "GenProg: A generic method for automatic software repair." *IEEE Transactions on Software Engineering*, 38(1), 54-72. https://doi.org/10.1109/TSE.2011.104
> Demonstrates that genetic algorithms can automatically patch software bugs. Hunt Mode adapts this by patching the *compiler's handling* of code rather than the code itself.

[4] **Deb, K.** (2014). "Multi-objective optimization." In *Search-based software engineering* (pp. 403-449). Springer. https://doi.org/10.1007/978-3-642-25231-0_15
> Supports the trade-off decision-making in Hunt Mode: balancing "Strict Rust Correctness" vs. "Project Compilability."

### Testing & Fault Localization

[5] **Pettis, K., & Hansen, R. C.** (1990). "Profile guided code positioning." *ACM SIGPLAN Notices*, 25(6), 16-27. https://doi.org/10.1145/93548.93550
> While focused on runtime performance, the concept of using execution profiles to tune the compiler mirrors Hunt Mode's use of *compilation failure profiles* to tune codegen.

[6] **Allamanis, M., Barr, E. T., Devanbu, P., & Sutton, C.** (2018). "A survey of machine learning for big code and naturalness." *ACM Computing Surveys*, 51(4), 1-37. https://doi.org/10.1145/3212695
> Validates the use of probabilistic models (`depyler-oracle`) to infer programmer intent and missing types from "natural" coding patterns.

[7] **Chen, T. Y., Kuo, F. C., Liu, H., Poon, P. L., Towey, D., Tse, T. H., & Zhou, Z. Q.** (2018). "Metamorphic testing: A review of challenges and opportunities." *ACM Computing Surveys*, 51(1), 1-27. https://doi.org/10.1145/3143561
> Relevant to the "Isolate" phase: generating variations of code to verify the compiler's behavior remains consistent (or consistently fixed).

### Software Evolution & Quality

[8] **Beck, K.** (2002). *Test Driven Development: By Example*. Addison-Wesley Professional. ISBN: 978-0321146533
> The core loop of Hunt Mode (Red/Green/Refactor) is mechanically automated TDD applied to the compiler itself.

[9] **Arcuri, A., & Yao, X.** (2008). "A novel co-evolutionary approach to automatic software bug fixing." *IEEE Congress on Evolutionary Computation*, 162-168. https://doi.org/10.1109/CEC.2008.4630793
> Suggests co-evolving the test suite (repro cases) and the program (the transpiler), a dynamic mirrored in Hunt Mode's iterative cycle.

[10] **Basili, V. R., & Rombach, H. D.** (1988). "The TAME project: Towards improvement-oriented software environments." *IEEE Transactions on Software Engineering*, 14(6), 758-773. https://doi.org/10.1109/32.6156
> Early work on tailoring software environments to specific project goals. Hunt Mode automates this tailoring, creating a bespoke compilation environment for each legacy codebase.

### Machine Learning Integration

[11] **Gift, N. et al.** (2025). *Aprender: Next-generation machine learning library in pure Rust*. PAIML. https://github.com/paiml/aprender
> Provides the `RandomForestClassifier` and `DriftDetector` primitives used by `depyler-oracle` for error clustering and drift detection.

[12] **Gift, N. et al.** (2025). *Entrenar: Training & Optimization library*. PAIML. https://github.com/paiml/entrenar
> Enables "Compiler-in-the-Loop" (CITL) training, allowing `depyler` to fine-tune its internal heuristic models (LoRA) on the specific codebase being transpiled.

### Toyota Production System & Lean Software Development

[13] **Liker, J. K.** (2004). *The Toyota Way: 14 Management Principles from the World's Greatest Manufacturer*. McGraw-Hill. ISBN: 978-0071392310
> Foundational text on Toyota's management philosophy. Hunt Mode implements principles 1 (long-term philosophy), 5 (build culture of stopping to fix problems), 12 (go and see for yourself), and 14 (become a learning organization through hansei and kaizen).

[14] **Deming, W. E.** (1986). *Out of the Crisis*. MIT Press. ISBN: 978-0262541152
> Introduces the PDCA (Plan-Do-Check-Act) cycle that forms the backbone of Hunt Mode's iterative improvement process. Each hunt cycle is one PDCA rotation.

[15] **Ohno, T.** (1988). *Toyota Production System: Beyond Large-Scale Production*. Productivity Press. ISBN: 978-0915299140
> Original description of Jidoka (autonomation) by its creator. Hunt Mode's automatic quality gates and human-review thresholds directly implement Jidoka principles.

[16] **Imai, M.** (1986). *Kaizen: The Key to Japan's Competitive Success*. McGraw-Hill. ISBN: 978-0075543329
> Defines Kaizen as continuous, incremental improvement. Hunt Mode's per-cycle improvement tracking and plateau detection operationalize this philosophy.

[17] **Rother, M., & Shook, J.** (1999). *Learning to See: Value Stream Mapping*. Lean Enterprise Institute. ISBN: 978-0966784305
> Introduces value stream mapping and the concept of leveling (Heijunka). Hunt Mode's priority queue implements Heijunka by processing highest-value defects first.

[18] **Shingo, S.** (1986). *Zero Quality Control: Source Inspection and the Poka-Yoke System*. Productivity Press. ISBN: 978-0915299072
> Defines Poka-Yoke (mistake-proofing). Hunt Mode's requirement that every fix must have a failing test first is a direct implementation of source inspection.

[19] **Baudin, M.** (2007). *Working with Machines: The Nuts and Bolts of Lean Operations with Jidoka*. Productivity Press. ISBN: 978-1563273292
> Practical guide to implementing Andon systems. Hunt Mode's visual status board and automatic escalation implement Andon in a software context.

[20] **Ohno, T.** (1988). "Ask 'why' five times about every matter." In *Toyota Production System* (pp. 17-20). Productivity Press.
> Original description of the Five Whys technique. Hunt Mode's root cause analysis module implements this directly, tracing each compilation failure to its true source.

### Additional Peer-Reviewed Citations

[21] **Monperrus, M.** (2018). "Automatic software repair: A bibliography." *ACM Computing Surveys*, 51(1), 1-24. https://doi.org/10.1145/3105906
> Comprehensive survey of automatic program repair techniques. Hunt Mode combines search-based repair [2, 3] with learning-based approaches [6, 11].

[22] **Goues, C. L., Pradel, M., & Roychoudhury, A.** (2019). "Automated program repair." *Communications of the ACM*, 62(12), 56-65. https://doi.org/10.1145/3318162
> Recent overview of the APR field. Validates Hunt Mode's approach of treating compilation as a search problem with quality constraints (Jidoka).

---

## Appendix A: Mapping to Toyota Way 14 Principles

| Principle | Hunt Mode Implementation |
|-----------|-------------------------|
| 1. Base decisions on long-term philosophy | Target 80% compile rate, not quick hacks |
| 2. Create continuous process flow | PDCA cycle with no batch processing |
| 3. Use "pull" systems | Process errors as they're discovered |
| 4. Level the workload (Heijunka) | Priority queue by frequency × impact |
| 5. Build culture of stopping to fix | Jidoka quality gates halt on uncertainty |
| 6. Standardized tasks are foundation | Mutator library with consistent patterns |
| 7. Use visual control | Andon dashboard showing system state |
| 8. Use only reliable technology | Battle-tested aprender/entrenar libraries |
| 9. Grow leaders who understand work | Five Whys traces to actual codegen code |
| 10. Develop exceptional people | Hansei lessons improve future cycles |
| 11. Respect extended network | Works with existing Python codebases |
| 12. Go and see (Genchi Genbutsu) | Observe actual rustc errors, not abstractions |
| 13. Make decisions slowly, implement rapidly | Analyze carefully, apply fix atomically |
| 14. Become learning organization | Hansei reflection, lessons database |

## Appendix B: Metrics and Success Criteria

### Key Performance Indicators (KPIs)

| Metric | Target | Measurement |
|--------|--------|-------------|
| Single-shot compile rate | ≥80% | Successful `rustc` / Total examples |
| Cycles to plateau | <100 | Cycles until rate_delta < 0.001 |
| Fix confidence | ≥85% | Average Jidoka score of applied fixes |
| Root cause depth | ≤3 | Average Five Whys depth to root cause |
| Regression rate | 0% | Fixes that break previously passing code |

### Definition of Done (DoD) for Hunt Mode Cycle

- [ ] Failure pattern identified and classified
- [ ] Minimal reproduction case synthesized
- [ ] Fix applied with confidence ≥ quality_threshold
- [ ] `rustc --crate-type lib` passes
- [ ] Property tests pass (if applicable)
- [ ] No regressions in existing examples
- [ ] Commit message references ticket (DEPYLER-XXXX)
- [ ] Hansei lesson recorded
