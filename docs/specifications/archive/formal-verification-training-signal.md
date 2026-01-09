# Formal Verification Training Signal Specification

**Version**: 1.0.0
**Status**: Draft
**Authors**: PAIML Team
**Date**: 2025-12-02
**Related Projects**: depyler, decy, verificar, verus

## Abstract

This specification defines a multi-tool formal verification framework for generating rich oracle training signals beyond traditional compiler feedback. By integrating **Verus** (SMT-based proof), **Miri** (undefined behavior detection), **Kani** (bounded model checking), and **CBMC** (C bounded model checking), we create a hierarchical verification oracle that produces counterexamples, proof failures, and safety violations as high-fidelity training signals for both **depyler** (Python→Rust) and **decy** (C→Rust) transpilers.

## 1. Motivation

### 1.1 Limitations of Current Training Signals

The existing CITL (Compiler-in-the-Loop) training approach relies on three signal sources:

| Signal Source | What It Detects | Limitation |
|--------------|-----------------|------------|
| **rustc errors** | Type mismatches, borrow violations | Binary pass/fail; no semantic insight |
| **I/O comparison** (verificar) | Output divergence | Only catches runtime differences |
| **Property testing** (quickcheck) | Random counterexamples | Probabilistic, may miss edge cases |

> **Annotation [1]:** **Verification Gap**: Jung et al. (RustBelt, 2017) demonstrated that Rust's type system guarantees memory safety but does not verify functional correctness. A transpiled function may compile and produce identical I/O but still contain latent bugs detectable only through formal methods.

### 1.2 Formal Verification Signal Hierarchy

Formal verification tools provide *graduated certainty levels* with increasingly rich training signals:

```
                     ┌──────────────────────────────────────────────────┐
                     │              SIGNAL RICHNESS PYRAMID              │
                     └──────────────────────────────────────────────────┘

                                    ▲
                                   /│\
                                  / │ \
                                 /  │  \    VERUS (SMT Proofs)
                                /   │   \   - Precondition failures
                               /    │    \  - Postcondition violations
                              /     │     \ - Invariant counterexamples
                             /      │      \- Termination proof failures
                            /───────┼───────\
                           /        │        \
                          /    KANI (BMC)     \
                         /   - Assertion       \
                        /     violations        \
                       /    - Panic reachability \
                      /     - Overflow detection  \
                     /───────────────────────────────\
                    /                                 \
                   /         MIRI (UB Detection)       \
                  /   - Use-after-free                  \
                 /    - Data races                       \
                /     - Uninitialized memory              \
               /      - Invalid pointer arithmetic         \
              /─────────────────────────────────────────────\
             /                                               \
            /            RUSTC (Type Checking)                \
           /     - Borrow checker errors                       \
          /      - Type mismatches                              \
         /       - Lifetime violations                           \
        /─────────────────────────────────────────────────────────\
```

### 1.3 Training Signal Value Proposition

| Tool | Signal Type | Training Value | False Positive Rate |
|------|-------------|----------------|---------------------|
| **Verus** | Proof failures + counterexamples | Highest (functional correctness) | ~5% (spec ambiguity) |
| **Kani** | BMC violations + concrete inputs | Very High (exhaustive in bounds) | ~2% (unreachable paths) |
| **Miri** | UB detection | High (runtime safety) | <1% (sound by design) |
| **rustc** | Compile errors | Medium (type safety) | 0% (definitive) |

> **Annotation [2]:** **Counterexample-Guided Learning**: Albarghouthi (2021) demonstrated that SMT counterexamples provide superior training signals for program synthesis compared to simple pass/fail feedback, as they encode *why* a program fails, not just *that* it fails.

## 2. Architecture

### 2.1 Multi-Tool Verification Pipeline

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    FORMAL VERIFICATION TRAINING PIPELINE                     │
└─────────────────────────────────────────────────────────────────────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────────────────────────────┐
│   Source     │    │  Transpiler  │    │         VERIFICATION BATTERY          │
│ (Python/C)   │───▶│(depyler/decy)│───▶│                                      │
└──────────────┘    └──────────────┘    │  ┌────────┐  ┌────────┐  ┌────────┐  │
                                        │  │ rustc  │  │  Miri  │  │  Kani  │  │
                                        │  └───┬────┘  └───┬────┘  └───┬────┘  │
                                        │      │          │          │        │
                                        │      ▼          ▼          ▼        │
                                        │  ┌─────────────────────────────┐    │
                                        │  │     Verus (Spec-Enhanced)   │    │
                                        │  └──────────────┬──────────────┘    │
                                        └─────────────────┼────────────────────┘
                                                          │
                                                          ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SIGNAL AGGREGATION LAYER                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐              │
│  │ Counterexample  │  │ UB Classification│  │ Proof Failure  │              │
│  │   Extractor     │  │    Taxonomy      │  │   Categorizer  │              │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘              │
│           │                    │                    │                        │
│           └────────────────────┼────────────────────┘                        │
│                                ▼                                             │
│                    ┌───────────────────────┐                                 │
│                    │ FormalVerificationSignal │                              │
│                    │ (unified training datum)  │                              │
│                    └───────────────────────┘                                 │
└─────────────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                          TRAINING CORPUS (.fvs format)                       │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ verificar    │  │  entrenar    │  │ depyler-     │  │ decy-oracle  │    │
│  │ integration  │  │  CITL export │  │ oracle .apr  │  │ patterns     │    │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Tool-Specific Integration

#### 2.2.1 Verus Integration (SMT-Based Proofs)

Verus provides the richest signals through its three-mode system (`spec`, `proof`, `exec`):

```rust
// crates/depyler-verify/src/verus_oracle.rs

use verus_vir::ast::{Function, Requires, Ensures, Invariant};

/// Verus verification result with rich training signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerusSignal {
    /// Function being verified
    pub function_name: String,

    /// Verification outcome
    pub verdict: VerusVerdict,

    /// SMT query statistics
    pub smt_stats: SmtStats,

    /// Counterexample if verification failed
    pub counterexample: Option<Counterexample>,

    /// Specific clause that failed (for multi-clause specs)
    pub failed_clause: Option<FailedClause>,

    /// Resource usage (rlimit consumption)
    pub rlimit_used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerusVerdict {
    /// All specifications verified
    Verified,

    /// Precondition violation (caller's fault)
    PreconditionFailed {
        requires_clause: String,
        counterexample_inputs: Vec<(String, String)>,
    },

    /// Postcondition violation (implementation's fault)
    PostconditionFailed {
        ensures_clause: String,
        counterexample_state: String,
    },

    /// Loop invariant violation
    InvariantFailed {
        loop_location: (usize, usize),
        invariant_clause: String,
        iteration: Option<u64>,
    },

    /// Termination proof failed
    TerminationFailed {
        decreases_clause: String,
        witness: Option<String>,
    },

    /// SMT timeout (inconclusive)
    Timeout { rlimit: u64 },

    /// Assertion failed
    AssertionFailed {
        location: (usize, usize),
        assertion: String,
        counterexample: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counterexample {
    /// Variable assignments that trigger failure
    pub assignments: HashMap<String, ConcreteValue>,

    /// Execution trace leading to failure
    pub trace: Vec<TraceStep>,

    /// SMT model (for advanced debugging)
    pub smt_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConcreteValue {
    Int(i128),
    Bool(bool),
    Seq(Vec<ConcreteValue>),
    Struct(HashMap<String, ConcreteValue>),
    Unknown,
}
```

#### 2.2.2 Miri Integration (Undefined Behavior Detection)

Miri detects runtime safety violations that pass the borrow checker:

```rust
// crates/depyler-verify/src/miri_oracle.rs

/// Miri UB detection signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiriSignal {
    /// Whether execution completed without UB
    pub clean: bool,

    /// Detected UB violations
    pub violations: Vec<UndefinedBehavior>,

    /// Memory allocation tracking
    pub allocations: AllocationStats,

    /// Data race detection results
    pub data_races: Vec<DataRace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UndefinedBehavior {
    /// Dereferencing freed memory
    UseAfterFree {
        allocation_id: u64,
        freed_at: StackTrace,
        accessed_at: StackTrace,
    },

    /// Reading uninitialized memory
    UninitializedRead {
        location: StackTrace,
        type_name: String,
    },

    /// Invalid pointer arithmetic
    InvalidPointerArithmetic {
        operation: String,
        pointer: String,
        offset: i128,
    },

    /// Integer overflow in release mode
    IntegerOverflow {
        operation: String,
        operands: (String, String),
        result: String,
    },

    /// Alignment violation
    MisalignedAccess {
        required_alignment: usize,
        actual_alignment: usize,
        type_name: String,
    },

    /// Invalid enum discriminant
    InvalidEnumDiscriminant {
        enum_name: String,
        value: u128,
    },

    /// Transmute to invalid value
    InvalidTransmute {
        from_type: String,
        to_type: String,
        value: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRace {
    /// Location of first access
    pub access1: RaceAccess,
    /// Location of second access
    pub access2: RaceAccess,
    /// Shared memory location
    pub memory_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceAccess {
    pub thread_id: u64,
    pub operation: AccessType,
    pub location: StackTrace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
}
```

#### 2.2.3 Kani Integration (Bounded Model Checking)

Kani provides exhaustive verification within bounds:

```rust
// crates/depyler-verify/src/kani_oracle.rs

/// Kani BMC verification signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaniSignal {
    /// Verification result
    pub result: KaniResult,

    /// Concrete inputs that trigger failure
    pub counterexample: Option<KaniCounterexample>,

    /// Coverage of verification harness
    pub coverage: KaniCoverage,

    /// Property violations found
    pub violations: Vec<KaniViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KaniResult {
    /// All properties verified within bounds
    Success,

    /// Property violation found
    Failure,

    /// Verification incomplete (unwinding limit)
    Incomplete { max_unwind: u32 },

    /// Solver timeout
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaniCounterexample {
    /// Concrete input values triggering failure
    pub inputs: HashMap<String, String>,

    /// Execution trace
    pub trace: Vec<KaniTraceStep>,

    /// CBMC property that failed
    pub property_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KaniViolation {
    /// Assertion failure
    Assertion {
        location: String,
        condition: String,
    },

    /// Arithmetic overflow
    Overflow {
        operation: String,
        location: String,
    },

    /// Out-of-bounds access
    OutOfBounds {
        index: String,
        length: String,
        location: String,
    },

    /// Unwrap on None
    UnwrapNone { location: String },

    /// Division by zero
    DivisionByZero { location: String },

    /// Unreachable code reached
    UnreachableReached { location: String },
}
```

#### 2.2.4 CBMC Integration (C Source Verification for Decy)

For decy, we verify C source before transpilation:

```rust
// crates/decy-verify/src/cbmc_oracle.rs

/// CBMC verification of C source (pre-transpilation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CbmcSignal {
    /// Verification outcome
    pub result: CbmcResult,

    /// Property violations in C source
    pub violations: Vec<CbmcViolation>,

    /// Memory safety issues
    pub memory_issues: Vec<MemorySafetyIssue>,

    /// Counterexample trace
    pub counterexample: Option<CbmcCounterexample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CbmcViolation {
    /// Buffer overflow
    BufferOverflow {
        buffer: String,
        access_index: String,
        buffer_size: String,
    },

    /// Null pointer dereference
    NullDereference {
        pointer: String,
        location: String,
    },

    /// Memory leak
    MemoryLeak {
        allocation_site: String,
        size: String,
    },

    /// Double free
    DoubleFree {
        pointer: String,
        first_free: String,
        second_free: String,
    },

    /// Use of uninitialized variable
    UninitializedUse {
        variable: String,
        location: String,
    },
}
```

## 3. Unified Training Signal Format

### 3.1 FormalVerificationSignal Schema

```rust
// crates/depyler-oracle/src/formal_signal.rs

/// Unified formal verification training signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormalVerificationSignal {
    /// Unique signal identifier
    pub id: Uuid,

    /// Timestamp of verification
    pub timestamp: DateTime<Utc>,

    /// Source code being verified
    pub source: SourceInfo,

    /// Target (transpiled) code
    pub target: TargetInfo,

    /// Verification results from each tool
    pub verification_results: VerificationBattery,

    /// Aggregated training label
    pub label: TrainingLabel,

    /// Decision trace correlation (links to CITL)
    pub decision_trace_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// Source language (Python or C)
    pub language: SourceLanguage,

    /// Original source code
    pub code: String,

    /// Source file path
    pub file_path: Option<PathBuf>,

    /// Function/method name
    pub function_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetInfo {
    /// Transpiled Rust code
    pub code: String,

    /// Verus specifications (if generated)
    pub verus_specs: Option<String>,

    /// Kani harness (if generated)
    pub kani_harness: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationBattery {
    /// rustc compilation result
    pub rustc: Option<RustcSignal>,

    /// Miri UB detection result
    pub miri: Option<MiriSignal>,

    /// Kani BMC result
    pub kani: Option<KaniSignal>,

    /// Verus proof result
    pub verus: Option<VerusSignal>,

    /// CBMC result (for decy only)
    pub cbmc: Option<CbmcSignal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingLabel {
    /// Overall correctness verdict
    pub verdict: OverallVerdict,

    /// Confidence score (0.0-1.0)
    pub confidence: f64,

    /// Error categories detected
    pub error_categories: Vec<ErrorCategory>,

    /// Suggested fix patterns (from counterexamples)
    pub fix_hints: Vec<FixHint>,

    /// Severity ranking (for prioritization)
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverallVerdict {
    /// All tools pass
    FullyVerified,

    /// Compiles but has runtime issues (Miri/Kani fail)
    RuntimeUnsafe,

    /// Compiles but fails specifications (Verus fail)
    SpecificationViolation,

    /// Does not compile
    CompilationFailure,

    /// Inconclusive (timeouts)
    Inconclusive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// Ownership/borrowing issues
    Ownership(OwnershipError),

    /// Lifetime issues
    Lifetime(LifetimeError),

    /// Type system issues
    TypeMismatch(TypeMismatchError),

    /// Memory safety issues
    MemorySafety(MemorySafetyError),

    /// Functional correctness issues
    FunctionalCorrectness(FunctionalError),

    /// Concurrency issues
    Concurrency(ConcurrencyError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixHint {
    /// Pattern name (for oracle lookup)
    pub pattern_name: String,

    /// Confidence that this fix applies
    pub confidence: f64,

    /// Counterexample that informed this hint
    pub evidence: String,

    /// Suggested code transformation
    pub suggested_transform: Option<String>,
}
```

### 3.2 File Format (.fvs - Formal Verification Signal)

```
┌─────────────────────────────────────────────────────────────────┐
│                    .fvs FILE FORMAT                              │
├─────────────────────────────────────────────────────────────────┤
│  Header (16 bytes):                                              │
│    Magic: "FVS\x00" (4 bytes)                                   │
│    Version: u32 (4 bytes)                                       │
│    Signal count: u64 (8 bytes)                                  │
├─────────────────────────────────────────────────────────────────┤
│  Index Section (variable):                                       │
│    [offset: u64, length: u32, signal_id: u128] × count          │
├─────────────────────────────────────────────────────────────────┤
│  Data Section (Zstd compressed):                                 │
│    [FormalVerificationSignal as MessagePack] × count            │
└─────────────────────────────────────────────────────────────────┘
```

## 4. Specification Generation

### 4.1 Automatic Specification Inference

To leverage Verus, we need specifications. We generate these from multiple sources:

```rust
// crates/depyler-verify/src/spec_gen.rs

/// Specification generator for transpiled code
pub struct SpecGenerator {
    /// Python type hints → Verus preconditions
    type_hint_converter: TypeHintConverter,

    /// Docstring contracts → Verus requires/ensures
    docstring_parser: DocstringContractParser,

    /// Property test oracles → Verus postconditions
    property_test_converter: PropertyTestConverter,

    /// I/O examples → Verus test harnesses
    io_example_converter: IoExampleConverter,
}

impl SpecGenerator {
    /// Generate Verus specifications from Python source + docstrings
    pub fn generate_specs(&self, python_source: &str, rust_code: &str)
        -> Result<VerusSpecs>
    {
        let mut specs = VerusSpecs::default();

        // 1. Extract type-based preconditions
        specs.merge(self.type_hint_converter.convert(python_source)?);

        // 2. Extract docstring contracts (@requires, @ensures)
        specs.merge(self.docstring_parser.parse(python_source)?);

        // 3. Convert property tests to postconditions
        specs.merge(self.property_test_converter.convert(python_source)?);

        // 4. Generate test harnesses from I/O examples
        specs.merge(self.io_example_converter.convert(python_source)?);

        Ok(specs)
    }
}

/// Convert Python type hints to Verus preconditions
impl TypeHintConverter {
    fn convert_type_hint(&self, hint: &PyTypeHint) -> Option<VerusRequires> {
        match hint {
            // int → i64 (assume fits in i64)
            PyTypeHint::Int => None, // No additional constraint

            // int with bounds → requires bounds
            PyTypeHint::IntRange { min, max } => Some(VerusRequires {
                clause: format!("{} <= x && x <= {}", min, max),
            }),

            // List[T] → requires len() <= MAX
            PyTypeHint::List { element, max_len } => {
                max_len.map(|len| VerusRequires {
                    clause: format!("x.len() <= {}", len),
                })
            }

            // Optional[T] → no constraint (maps to Option)
            PyTypeHint::Optional { .. } => None,

            // Callable → cannot verify (skip)
            PyTypeHint::Callable { .. } => None,
        }
    }
}

/// Convert docstring contracts to Verus specifications
impl DocstringContractParser {
    fn parse_docstring(&self, docstring: &str) -> Result<VerusSpecs> {
        let mut specs = VerusSpecs::default();

        for line in docstring.lines() {
            if let Some(requires) = self.parse_requires(line) {
                // @requires x > 0 → requires x > 0,
                specs.requires.push(requires);
            }
            if let Some(ensures) = self.parse_ensures(line) {
                // @ensures result >= 0 → ensures result >= 0,
                specs.ensures.push(ensures);
            }
            if let Some(invariant) = self.parse_invariant(line) {
                // @invariant i < len → invariant i < len,
                specs.invariants.push(invariant);
            }
        }

        Ok(specs)
    }
}
```

### 4.2 Specification Templates by Pattern

Common transpilation patterns have known specifications:

```rust
/// Library of specification templates for common patterns
pub const SPEC_TEMPLATES: &[SpecTemplate] = &[
    // List operations
    SpecTemplate {
        pattern: "list_append",
        requires: &[],
        ensures: &["self.len() == old(self).len() + 1"],
        invariants: &[],
    },

    // Binary search
    SpecTemplate {
        pattern: "binary_search",
        requires: &[
            "forall|i: int, j: int| 0 <= i <= j < arr.len() ==> arr[i] <= arr[j]",
        ],
        ensures: &[
            "result.is_some() ==> arr[result.unwrap() as int] == target",
            "result.is_none() ==> forall|i: int| 0 <= i < arr.len() ==> arr[i] != target",
        ],
        invariants: &[
            "0 <= lo <= hi <= arr.len()",
            "forall|i: int| 0 <= i < lo ==> arr[i] < target",
            "forall|i: int| hi <= i < arr.len() ==> arr[i] > target",
        ],
    },

    // Sorting
    SpecTemplate {
        pattern: "sort",
        requires: &[],
        ensures: &[
            "result.len() == old(arr).len()",
            "forall|i: int, j: int| 0 <= i <= j < result.len() ==> result[i] <= result[j]",
            "result.to_multiset() == old(arr).to_multiset()",
        ],
        invariants: &[],
    },

    // Division
    SpecTemplate {
        pattern: "divide",
        requires: &["divisor != 0"],
        ensures: &["result * divisor + remainder == dividend"],
        invariants: &[],
    },
];
```

## 5. Verification Pipeline Execution

### 5.1 Tiered Execution Strategy

To minimize compute costs, we use tiered verification:

```rust
// crates/depyler-verify/src/pipeline.rs

/// Tiered verification pipeline (fail-fast)
pub struct VerificationPipeline {
    rustc_executor: RustcExecutor,
    miri_executor: MiriExecutor,
    kani_executor: KaniExecutor,
    verus_executor: VerusExecutor,
}

impl VerificationPipeline {
    /// Execute verification in tiers, stopping at first failure
    pub async fn verify_tiered(
        &self,
        code: &str,
        specs: Option<&VerusSpecs>,
        config: &VerificationConfig,
    ) -> FormalVerificationSignal {
        let mut signal = FormalVerificationSignal::default();

        // Tier 1: rustc (fastest, always run)
        signal.verification_results.rustc = Some(
            self.rustc_executor.compile(code).await
        );
        if signal.verification_results.rustc.as_ref().unwrap().failed() {
            signal.label.verdict = OverallVerdict::CompilationFailure;
            return signal;
        }

        // Tier 2: Miri (fast UB detection)
        if config.enable_miri {
            signal.verification_results.miri = Some(
                self.miri_executor.execute(code).await
            );
            if signal.verification_results.miri.as_ref().unwrap().has_ub() {
                signal.label.verdict = OverallVerdict::RuntimeUnsafe;
                // Continue to gather more signals
            }
        }

        // Tier 3: Kani (bounded model checking)
        if config.enable_kani {
            signal.verification_results.kani = Some(
                self.kani_executor.verify(code, config.kani_unwind).await
            );
            if signal.verification_results.kani.as_ref().unwrap().failed() {
                signal.label.verdict = OverallVerdict::RuntimeUnsafe;
            }
        }

        // Tier 4: Verus (full specification checking)
        if config.enable_verus && specs.is_some() {
            let specs = specs.unwrap();
            signal.verification_results.verus = Some(
                self.verus_executor.verify(code, specs, config.verus_rlimit).await
            );
            if signal.verification_results.verus.as_ref().unwrap().failed() {
                signal.label.verdict = OverallVerdict::SpecificationViolation;
            }
        }

        // If we got here with no failures, fully verified
        if signal.label.verdict == OverallVerdict::default() {
            signal.label.verdict = OverallVerdict::FullyVerified;
        }

        signal
    }
}
```

### 5.2 Parallel Execution Mode

For batch processing (training corpus generation):

```rust
/// Parallel verification for corpus generation
pub struct ParallelVerificationBatch {
    /// Number of parallel workers
    parallelism: usize,

    /// Shared verification pipeline
    pipeline: Arc<VerificationPipeline>,

    /// Progress tracking
    progress: Arc<AtomicU64>,
}

impl ParallelVerificationBatch {
    pub async fn verify_batch(
        &self,
        samples: Vec<(String, String)>, // (source, target) pairs
        config: &VerificationConfig,
    ) -> Vec<FormalVerificationSignal> {
        let semaphore = Arc::new(Semaphore::new(self.parallelism));

        let tasks: Vec<_> = samples
            .into_iter()
            .map(|(source, target)| {
                let sem = semaphore.clone();
                let pipeline = self.pipeline.clone();
                let config = config.clone();
                let progress = self.progress.clone();

                tokio::spawn(async move {
                    let _permit = sem.acquire().await.unwrap();
                    let result = pipeline.verify_tiered(&target, None, &config).await;
                    progress.fetch_add(1, Ordering::Relaxed);
                    result
                })
            })
            .collect();

        futures::future::join_all(tasks)
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .collect()
    }
}
```

## 6. Integration with Verificar

### 6.1 Extended Oracle Interface

```rust
// In verificar/src/oracle/formal.rs

use depyler_verify::FormalVerificationSignal;

/// Formal verification oracle for verificar
pub struct FormalOracle {
    /// Verification pipeline
    pipeline: VerificationPipeline,

    /// Specification generator
    spec_gen: SpecGenerator,

    /// Configuration
    config: FormalOracleConfig,
}

impl Oracle for FormalOracle {
    type Verdict = FormalVerdict;

    fn verify(
        &self,
        source: &str,
        target: &str,
        _input: &str, // Not used for formal verification
    ) -> Result<VerificationResult<Self::Verdict>> {
        // Generate specifications from source
        let specs = self.spec_gen.generate_specs(source, target)?;

        // Run verification battery
        let signal = self.pipeline.verify_tiered(target, Some(&specs), &self.config)?;

        Ok(VerificationResult {
            verdict: FormalVerdict::from(&signal),
            details: serde_json::to_string(&signal)?,
        })
    }
}

/// Extended verdict type for formal verification
pub enum FormalVerdict {
    /// All formal methods pass
    FullyVerified,

    /// Some tools pass, some fail
    PartiallyVerified {
        passed: Vec<String>,
        failed: Vec<String>,
    },

    /// Critical failure (Miri UB or Kani violation)
    UnsafeCode {
        violations: Vec<String>,
    },

    /// Specification mismatch (Verus failure)
    SpecViolation {
        failed_specs: Vec<String>,
        counterexamples: Vec<String>,
    },

    /// Compilation failure
    CompilationFailed {
        errors: Vec<String>,
    },
}
```

### 6.2 Enhanced Data Pipeline

```rust
// In verificar/src/data/formal_pipeline.rs

/// Data pipeline with formal verification signals
pub struct FormalDataPipeline {
    /// Base pipeline
    base: DataPipeline,

    /// Formal oracle
    formal_oracle: FormalOracle,

    /// Signal storage
    signal_store: FormalSignalStore,
}

impl FormalDataPipeline {
    /// Generate training corpus with formal verification signals
    pub async fn generate_with_formal(
        &self,
        config: &PipelineConfig,
    ) -> Result<FormalCorpusStats> {
        let mut stats = FormalCorpusStats::default();

        // Generate base samples
        let samples = self.base.generate(config)?;

        // Verify each sample with formal methods
        for sample in samples {
            let signal = self.formal_oracle.verify(
                &sample.source_code,
                &sample.target_code.unwrap_or_default(),
                "",
            )?;

            // Update statistics
            match &signal.verdict {
                FormalVerdict::FullyVerified => stats.fully_verified += 1,
                FormalVerdict::PartiallyVerified { .. } => stats.partially_verified += 1,
                FormalVerdict::UnsafeCode { .. } => stats.unsafe_code += 1,
                FormalVerdict::SpecViolation { .. } => stats.spec_violations += 1,
                FormalVerdict::CompilationFailed { .. } => stats.compilation_failed += 1,
            }

            // Store signal
            self.signal_store.append(&signal)?;
        }

        Ok(stats)
    }
}
```

## 7. Training Signal Extraction

### 7.1 Feature Engineering from Formal Signals

```rust
// crates/depyler-oracle/src/formal_features.rs

/// Extract ML features from formal verification signals
pub struct FormalFeatureExtractor {
    /// Feature dimensionality
    pub dimensions: usize,
}

impl FormalFeatureExtractor {
    /// Extract feature vector from formal verification signal
    pub fn extract(&self, signal: &FormalVerificationSignal) -> Vec<f64> {
        let mut features = Vec::with_capacity(self.dimensions);

        // Binary features: tool pass/fail
        features.push(signal.rustc_passed() as u8 as f64);
        features.push(signal.miri_passed() as u8 as f64);
        features.push(signal.kani_passed() as u8 as f64);
        features.push(signal.verus_passed() as u8 as f64);

        // Count features: violation counts
        features.push(signal.miri_violation_count() as f64);
        features.push(signal.kani_violation_count() as f64);
        features.push(signal.verus_failure_count() as f64);

        // Categorical features: error types (one-hot encoded)
        features.extend(self.encode_error_categories(&signal.error_categories()));

        // Numeric features: resource usage
        features.push(signal.verus_rlimit_used() as f64 / 1_000_000.0);
        features.push(signal.kani_unwinding_depth() as f64);

        // Counterexample features
        if let Some(ce) = signal.counterexample() {
            features.push(ce.trace_length() as f64);
            features.push(ce.variable_count() as f64);
        } else {
            features.push(0.0);
            features.push(0.0);
        }

        features
    }

    /// One-hot encode error categories
    fn encode_error_categories(&self, categories: &[ErrorCategory]) -> Vec<f64> {
        let mut encoding = vec![0.0; ERROR_CATEGORY_COUNT];
        for cat in categories {
            encoding[cat.index()] = 1.0;
        }
        encoding
    }
}
```

### 7.2 Counterexample-Guided Fix Suggestion

```rust
// crates/depyler-oracle/src/counterexample_fixer.rs

/// Use counterexamples to guide fix suggestions
pub struct CounterexampleGuidedFixer {
    /// Pattern library
    patterns: PatternLibrary,

    /// Similarity index (HNSW)
    index: HnswIndex,
}

impl CounterexampleGuidedFixer {
    /// Suggest fixes based on counterexample analysis
    pub fn suggest_fix(
        &self,
        signal: &FormalVerificationSignal,
    ) -> Option<FixSuggestion> {
        // Priority order: Verus > Kani > Miri counterexamples
        if let Some(verus) = &signal.verification_results.verus {
            if let Some(ce) = &verus.counterexample {
                return self.analyze_verus_counterexample(ce, verus);
            }
        }

        if let Some(kani) = &signal.verification_results.kani {
            if let Some(ce) = &kani.counterexample {
                return self.analyze_kani_counterexample(ce, kani);
            }
        }

        if let Some(miri) = &signal.verification_results.miri {
            if !miri.violations.is_empty() {
                return self.analyze_miri_violations(&miri.violations);
            }
        }

        None
    }

    fn analyze_verus_counterexample(
        &self,
        ce: &Counterexample,
        signal: &VerusSignal,
    ) -> Option<FixSuggestion> {
        match &signal.verdict {
            VerusVerdict::PreconditionFailed { requires_clause, .. } => {
                // Suggest adding bounds check or validation
                Some(FixSuggestion {
                    pattern_name: "add_precondition_check".into(),
                    confidence: 0.85,
                    evidence: format!("Verus requires: {}", requires_clause),
                    suggested_transform: Some(format!(
                        "Add runtime check: assert!({});",
                        requires_clause
                    )),
                })
            }

            VerusVerdict::PostconditionFailed { ensures_clause, .. } => {
                // Implementation doesn't satisfy spec
                Some(FixSuggestion {
                    pattern_name: "fix_postcondition".into(),
                    confidence: 0.7,
                    evidence: format!(
                        "Counterexample: {:?} violates {}",
                        ce.assignments, ensures_clause
                    ),
                    suggested_transform: None, // Complex fix needed
                })
            }

            VerusVerdict::InvariantFailed { invariant_clause, iteration, .. } => {
                Some(FixSuggestion {
                    pattern_name: "fix_loop_invariant".into(),
                    confidence: 0.75,
                    evidence: format!(
                        "Invariant {} fails at iteration {:?}",
                        invariant_clause, iteration
                    ),
                    suggested_transform: None,
                })
            }

            _ => None,
        }
    }

    fn analyze_miri_violations(
        &self,
        violations: &[UndefinedBehavior],
    ) -> Option<FixSuggestion> {
        // Map UB type to fix pattern
        let first = violations.first()?;

        match first {
            UndefinedBehavior::UseAfterFree { .. } => {
                Some(FixSuggestion {
                    pattern_name: "fix_use_after_free".into(),
                    confidence: 0.9,
                    evidence: "Miri detected use-after-free".into(),
                    suggested_transform: Some("Clone instead of borrow".into()),
                })
            }

            UndefinedBehavior::IntegerOverflow { operation, operands, .. } => {
                Some(FixSuggestion {
                    pattern_name: "fix_overflow".into(),
                    confidence: 0.95,
                    evidence: format!("Overflow in {} with {:?}", operation, operands),
                    suggested_transform: Some(
                        "Use checked_* or wrapping_* operations".into()
                    ),
                })
            }

            UndefinedBehavior::UninitializedRead { type_name, .. } => {
                Some(FixSuggestion {
                    pattern_name: "fix_uninitialized".into(),
                    confidence: 0.9,
                    evidence: format!("Uninitialized {} read", type_name),
                    suggested_transform: Some("Initialize with Default::default()".into()),
                })
            }

            _ => None,
        }
    }
}
```

## 8. C-Specific Integration (Decy)

### 8.1 CBMC Pre-Transpilation Verification

For decy, we verify C source *before* transpilation to catch bugs early:

```rust
// crates/decy-verify/src/cbmc_pre.rs

/// Pre-transpilation C verification with CBMC
pub struct CbmcPreVerifier {
    cbmc_path: PathBuf,
    config: CbmcConfig,
}

impl CbmcPreVerifier {
    /// Verify C source before transpilation
    pub async fn verify_c_source(&self, c_code: &str) -> CbmcSignal {
        // Write to temp file
        let temp_file = self.write_temp_c(c_code)?;

        // Run CBMC with appropriate flags
        let output = Command::new(&self.cbmc_path)
            .args(&[
                "--pointer-check",
                "--bounds-check",
                "--div-by-zero-check",
                "--memory-leak-check",
                "--undefined-shift-check",
                "--unwind", &self.config.unwind.to_string(),
                "--json-ui",
                temp_file.path().to_str().unwrap(),
            ])
            .output()
            .await?;

        // Parse CBMC JSON output
        self.parse_cbmc_output(&output.stdout)
    }

    /// Generate decy-specific signals from CBMC results
    pub fn generate_decy_signal(&self, cbmc: &CbmcSignal) -> DecyTrainingSignal {
        DecyTrainingSignal {
            // Map CBMC violations to decy ownership categories
            ownership_hints: cbmc.violations.iter()
                .filter_map(|v| self.violation_to_ownership_hint(v))
                .collect(),

            // Unsafe blocks needed based on CBMC analysis
            required_unsafe: cbmc.violations.iter()
                .filter_map(|v| self.violation_to_unsafe_reason(v))
                .collect(),
        }
    }

    fn violation_to_ownership_hint(&self, v: &CbmcViolation) -> Option<OwnershipHint> {
        match v {
            CbmcViolation::NullDereference { pointer, .. } => {
                Some(OwnershipHint {
                    pointer_name: pointer.clone(),
                    suggested_type: "Option<Box<T>>".into(),
                    reason: "Null check required".into(),
                })
            }

            CbmcViolation::BufferOverflow { buffer, .. } => {
                Some(OwnershipHint {
                    pointer_name: buffer.clone(),
                    suggested_type: "Vec<T>".into(),
                    reason: "Bounds checking required".into(),
                })
            }

            CbmcViolation::DoubleFree { pointer, .. } => {
                Some(OwnershipHint {
                    pointer_name: pointer.clone(),
                    suggested_type: "Rc<T>".into(),
                    reason: "Reference counting prevents double-free".into(),
                })
            }

            _ => None,
        }
    }
}
```

### 8.2 Cross-Language Signal Correlation

```rust
// crates/decy-oracle/src/cross_language.rs

/// Correlate C (CBMC) and Rust (Miri/Kani/Verus) verification signals
pub struct CrossLanguageCorrelator {
    /// C→Rust source mapping
    source_map: SourceMap,
}

impl CrossLanguageCorrelator {
    /// Correlate pre-transpilation and post-transpilation signals
    pub fn correlate(
        &self,
        c_signal: &CbmcSignal,
        rust_signal: &FormalVerificationSignal,
    ) -> CorrelatedSignal {
        let mut correlations = Vec::new();

        // For each CBMC violation, find corresponding Rust signal
        for c_viol in &c_signal.violations {
            let rust_location = self.source_map.c_to_rust(&c_viol.location());

            // Check if Rust verification caught the same issue
            let rust_caught = self.find_corresponding_rust_issue(
                rust_location,
                c_viol,
                rust_signal,
            );

            correlations.push(ViolationCorrelation {
                c_violation: c_viol.clone(),
                rust_location,
                rust_detection: rust_caught,
                // Did transpilation fix the bug?
                transpilation_fixed: rust_caught.is_none(),
            });
        }

        CorrelatedSignal {
            correlations,
            // Bugs caught in C but not Rust (good transpilation!)
            fixed_by_transpilation: correlations.iter()
                .filter(|c| c.transpilation_fixed)
                .count(),
            // Bugs in both (need manual fix)
            persisted: correlations.iter()
                .filter(|c| !c.transpilation_fixed)
                .count(),
            // New bugs in Rust (bad transpilation!)
            introduced: rust_signal.unique_violations().len(),
        }
    }
}
```

## 9. Metrics and Monitoring

### 9.1 Verification Quality Metrics

```rust
/// Metrics for formal verification signal quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMetrics {
    /// Total samples verified
    pub total_samples: u64,

    /// Pass rates by tool
    pub rustc_pass_rate: f64,
    pub miri_pass_rate: f64,
    pub kani_pass_rate: f64,
    pub verus_pass_rate: f64,

    /// Counterexample statistics
    pub counterexamples_generated: u64,
    pub fix_suggestions_generated: u64,
    pub fix_success_rate: f64,

    /// Resource usage
    pub average_verus_rlimit: f64,
    pub average_kani_unwind: f64,
    pub total_verification_time_ms: u64,

    /// Signal quality indicators
    pub signal_diversity_score: f64,  // Entropy of error categories
    pub counterexample_usefulness: f64,  // How often CEs led to fixes
}
```

### 9.2 Dashboard Integration

```rust
/// Export metrics for Grafana/monitoring
pub struct MetricsExporter {
    prometheus_endpoint: String,
}

impl MetricsExporter {
    pub fn export(&self, metrics: &VerificationMetrics) {
        // Prometheus format
        gauge!("formal_verification_rustc_pass_rate", metrics.rustc_pass_rate);
        gauge!("formal_verification_miri_pass_rate", metrics.miri_pass_rate);
        gauge!("formal_verification_kani_pass_rate", metrics.kani_pass_rate);
        gauge!("formal_verification_verus_pass_rate", metrics.verus_pass_rate);

        counter!("formal_verification_counterexamples", metrics.counterexamples_generated);
        histogram!("formal_verification_time_ms", metrics.total_verification_time_ms);
    }
}
```

## 10. Roadmap

### Phase 1: Foundation (Sprint 1-2)
- **Goal**: Basic Miri integration
- **Deliverables**:
  - `depyler-verify` crate with MiriExecutor
  - Miri signal parsing and UB classification
  - Integration with verificar IoOracle
- **Metric**: 10k samples verified with Miri

### Phase 2: Bounded Model Checking (Sprint 3-4)
- **Goal**: Kani integration for exhaustive checking
- **Deliverables**:
  - KaniExecutor with harness generation
  - Counterexample extraction
  - Integration with depyler-oracle
- **Metric**: 5k samples with Kani counterexamples

### Phase 3: SMT Proofs (Sprint 5-8)
- **Goal**: Verus integration for specification checking
- **Deliverables**:
  - SpecGenerator from Python docstrings
  - VerusExecutor with SMT interaction
  - Counterexample-guided fix suggestions
- **Metric**: 1k samples with Verus proofs

### Phase 4: C Integration (Sprint 9-10)
- **Goal**: CBMC for decy pre-transpilation
- **Deliverables**:
  - CbmcPreVerifier in decy-verify
  - Cross-language signal correlation
  - Ownership hint extraction
- **Metric**: 500 C functions with CBMC + post-transpilation verification

### Phase 5: Unified Pipeline (Sprint 11-12)
- **Goal**: Full formal verification training corpus
- **Deliverables**:
  - FormalDataPipeline in verificar
  - .fvs file format and tooling
  - entrenar export integration
- **Metric**: 50k samples in formal corpus, 90% counterexample usefulness

## 11. References

1. Jung, R., et al. (2017). RustBelt: Securing the foundations of the Rust programming language. *POPL*.
2. Albarghouthi, A. (2021). Introduction to Neural Network Verification. *Foundations and Trends in PL*.
3. Lattuada, A., et al. (2023). Verus: Verifying Rust Programs using Linear Ghost Types. *OOPSLA*.
4. Amazon Web Services. (2024). Kani Rust Verifier. https://model-checking.github.io/kani/
5. Miri Contributors. (2024). Miri: An interpreter for Rust's mid-level intermediate representation. https://github.com/rust-lang/miri
6. Clarke, E., Kroening, D., & Lerda, F. (2004). A tool for checking ANSI-C programs. *TACAS*.
7. Harman, M., & Jones, B. F. (2001). Search-based software engineering. *Information and Software Technology*.
8. Astrauskas, V., et al. (2022). The Prusti Project: Formal Verification for Rust. *Proceedings of the 14th NASA Formal Methods Symposium (NFM)*.
9. Matsushita, Y., Tsukada, T., & Kobayashi, N. (2021). RustHorn: CHC-Based Verification for Rust Programs. *ACM Transactions on Programming Languages and Systems (TOPLAS)*.
10. Matsushita, Y., et al. (2022). RustHornBelt: A Semantic Foundation for Functional Verification of Rust Programs with Unsafe Code. *Proceedings of the 43rd ACM SIGPLAN Conference on Programming Language Design and Implementation (PLDI)*.
11. Ho, S., & Protzenko, J. (2022). Aeneas: Rust Verification by Functional Translation. *Proceedings of the ACM on Programming Languages (ICFP)*.
12. Denis, X., Jourdan, J.H., & Marché, C. (2022). Creusot: A Foundry for the Deductive Verification of Rust Programs. *Proceedings of the International Conference on Formal Engineering Methods (ICFEM)*.
13. Emre, M., et al. (2021). Translating C to Safer Rust. *Proceedings of the ACM on Programming Languages (OOPSLA)*.
14. Wang, Y., et al. (2024). Automated Proof Generation for Rust Code via Self-Evolution. *Proceedings of the International Conference on Learning Representations (ICLR)*.
15. Yang, A. Z. H., et al. (2024). VERT: Verified Equivalent Rust Transpilation with Large Language Models as Few-Shot Learners. *arXiv preprint*.
16. Baranowski, M. S., Smirnov, S., & Su, Z. (2018). Verifying Rust Programs with SMACK. *Proceedings of the 16th International Symposium on Automated Technology for Verification and Analysis (ATVA)*.
17. Toman, J., Pernsteiner, S., & Torlak, E. (2015). Crust: A Bounded Model Checker for Rust. *Proceedings of the 30th IEEE/ACM International Conference on Automated Software Engineering (ASE)*.
