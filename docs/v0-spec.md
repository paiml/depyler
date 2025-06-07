## Depyler V1: Pragmatic Python-to-Rust Transpiler with Progressive Verification

### Revised Architecture: Practical Verification-First Design

Building on the feedback, Depyler V1 adopts a **progressive verification**
strategy: ship working transpilation immediately, layer verification
incrementally, prove properties opportunistically.

```rust
// Revised pipeline with optional verification stages
pub struct DepylerPipeline {
    parser: PyParser,           // rustpython-parser initially
    analyzer: CoreAnalyzer,     // PMAT metrics + type analysis
    transpiler: DirectTranspiler,
    verifier: Option<PropertyVerifier>, // Optional for V1
    mcp_client: LazyMcpClient,         // Lazy-loaded for edge cases
}

// Simplified provable stage for V1
pub trait AnalyzableStage {
    type Input;
    type Output;
    type Metrics;  // Replace Proof with Metrics for V1
    
    fn execute(&self, input: Self::Input) -> Result<(Self::Output, Self::Metrics)>;
    fn validate(&self, output: &Self::Output) -> ValidationResult;
}
```

### Revised V1 File Structure (Leaner, More Focused)

```
depyler/
├── Cargo.toml
├── Makefile                      # PMAT-validated build
├── README.md
├── ROADMAP.md                   # Verification milestones
├── benches/
│   └── transpilation.rs         # Real-world benchmarks
├── crates/
│   ├── depyler-core/           # Core transpilation engine
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── ast_bridge.rs   # Python AST → HIR
│   │   │   ├── hir.rs          # High-level IR
│   │   │   ├── direct_rules.rs # Pattern → Rust mappings
│   │   │   ├── type_mapper.rs  # Python → Rust type mapping
│   │   │   └── codegen.rs      # Rust code generation
│   ├── depyler-analyzer/        # Quality metrics
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── complexity.rs   # McCabe & cognitive
│   │   │   ├── type_flow.rs    # Type inference/checking
│   │   │   └── metrics.rs      # Aggregated metrics
│   ├── depyler-verify/          # Lightweight verification
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── properties.rs   # Property definitions
│   │   │   ├── quickcheck.rs   # Property-based testing
│   │   │   └── contracts.rs    # Pre/post conditions
│   ├── depyler-mcp/            # MCP fallback
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── protocol.rs     # MCP wire protocol
│   │   │   └── validator.rs    # Response validation
│   └── depyler/                # CLI binary
│       ├── src/
│       │   └── main.rs
├── examples/
│   ├── showcase/               # V1 capability demos
│   └── validation/             # Test suite
├── python-stdlib-stubs/        # Type stubs for analysis
└── tests/
    ├── semantics/              # Python behavior tests
    └── transpilation/          # Rust output tests
```

### Realistic Python V1 Subset

```python
# V1.0: Core imperative subset with mandatory type hints

from typing import List, Dict, Optional

# Functions with primitive types
def calculate_sum(numbers: List[int]) -> int:
    total: int = 0
    for n in numbers:
        total += n
    return total

# Simple generic containers
def process_config(config: Dict[str, str]) -> Optional[str]:
    if "debug" in config:
        return config["debug"]
    return None

# Basic pattern matching (→ Rust match)
def classify_number(n: int) -> str:
    if n == 0:
        return "zero"
    elif n > 0:
        return "positive"
    else:
        return "negative"

# V1 limitations:
# - No classes (deferred to V1.1)
# - No async/await (V1.2)
# - No generators (V2)
# - No dynamic features (eval, getattr, etc.)
# - No multiple inheritance
# - No metaclasses
```

### High-Level Intermediate Representation (HIR)

```rust
// Simplified HIR focusing on correctness over optimization
#[derive(Debug, Clone, PartialEq)]
pub enum HirExpr {
    Literal(Literal),
    Var(Symbol),
    Binary { op: BinOp, left: Box<HirExpr>, right: Box<HirExpr> },
    Call { func: Symbol, args: Vec<HirExpr> },
    Index { base: Box<HirExpr>, index: Box<HirExpr> },
    // Ownership hints from analysis
    Borrow { expr: Box<HirExpr>, mutable: bool },
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: Symbol,
    pub params: Vec<(Symbol, Type)>,
    pub ret_type: Type,
    pub body: Vec<HirStmt>,
    // Inferred properties
    pub properties: FunctionProperties,
}

#[derive(Debug, Default)]
pub struct FunctionProperties {
    pub is_pure: bool,
    pub max_stack_depth: Option<usize>,
    pub always_terminates: bool,
    pub panic_free: bool,  // No index out of bounds, etc.
}
```

### Type Mapping Strategy

```rust
// Conservative type mapping with explicit widening
pub struct TypeMapper {
    width_preference: IntWidth,  // i32 vs i64
    string_type: StringStrategy, // String vs &str vs Cow
}

pub enum StringStrategy {
    AlwaysOwned,      // String everywhere (safe, simple)
    InferBorrowing,   // &str where possible (V1.1)
    CowByDefault,     // Cow<'static, str> (V1.2)
}

impl TypeMapper {
    pub fn map_type(&self, py_type: &PythonType) -> RustType {
        match py_type {
            PythonType::Int => RustType::Primitive(self.width_preference),
            PythonType::Str => match self.string_type {
                StringStrategy::AlwaysOwned => RustType::String,
                _ => RustType::String, // V1: Always owned
            },
            PythonType::List(inner) => {
                RustType::Vec(Box::new(self.map_type(inner)))
            },
            PythonType::Dict(k, v) => {
                RustType::HashMap(
                    Box::new(self.map_type(k)),
                    Box::new(self.map_type(v)),
                )
            },
            PythonType::Optional(inner) => {
                RustType::Option(Box::new(self.map_type(inner)))
            },
            _ => RustType::Unsupported(py_type.to_string()),
        }
    }
}
```

### Pragmatic Verification Approach

```rust
// V1: Property-based testing instead of SMT solving
pub struct PropertyVerifier {
    quickcheck: QuickCheck,
    contract_checker: ContractChecker,
}

#[derive(Debug, Serialize)]
pub struct VerificationResult {
    pub property: String,
    pub status: PropertyStatus,
    pub confidence: f64,  // 0.0 - 1.0
    pub method: VerificationMethod,
    pub counterexamples: Vec<TestCase>,
}

#[derive(Debug, Serialize)]
pub enum PropertyStatus {
    Proven,           // Exhaustive for small domains
    HighConfidence,   // 10k+ tests passed
    Likely,          // 1k+ tests passed
    Unknown,         // Not enough data
    Violated(String), // Counterexample found
}

impl PropertyVerifier {
    // V1: Test-based verification
    pub fn verify_function(&self, func: &HirFunction) -> Vec<VerificationResult> {
        let mut results = vec![];
        
        // Property 1: Type preservation
        if let Some(result) = self.verify_type_preservation(func) {
            results.push(result);
        }
        
        // Property 2: Panic freedom (bounds checking)
        if func.properties.panic_free {
            results.push(VerificationResult {
                property: "panic_free".into(),
                status: PropertyStatus::HighConfidence,
                confidence: 0.95,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            });
        }
        
        // Property 3: Termination (simple cases)
        if Self::has_simple_termination(func) {
            results.push(VerificationResult {
                property: "termination".into(),
                status: PropertyStatus::Proven,
                confidence: 1.0,
                method: VerificationMethod::StructuralInduction,
                counterexamples: vec![],
            });
        }
        
        results
    }
}
```

### MCP Integration (Realistic V1)

```rust
#[derive(Debug, Serialize)]
pub struct McpTranspilationRequest {
    pub version: &'static str,
    pub python_ast: serde_json::Value,
    pub error_context: ErrorContext,
    pub quality_hints: QualityHints,
}

#[derive(Debug, Serialize)]
pub struct QualityHints {
    pub target_complexity: u32,      // From PMAT
    pub preferred_types: Vec<String>, // "use Vec not slice"
    pub style_level: StyleLevel,     // Basic, Idiomatic, Optimized
}

#[derive(Debug, Deserialize)]
pub struct McpTranspilationResponse {
    pub rust_code: String,
    pub explanation: String,         // Why this approach
    pub test_cases: Vec<TestCase>,  // For validation
    pub confidence: f64,
    pub alternative_approaches: Vec<AlternativeApproach>,
}

// V1: Validate via compilation and testing, not formal proofs
impl McpValidator {
    pub async fn validate_response(&self, resp: &McpTranspilationResponse) -> ValidationResult {
        // 1. Parse Rust code
        let parsed = syn::parse_str::<syn::File>(&resp.rust_code)?;
        
        // 2. Type check (using rust-analyzer as library)
        let type_errors = self.type_check(&parsed).await?;
        
        // 3. Run provided test cases
        let test_results = self.run_tests(&resp.test_cases).await?;
        
        // 4. Complexity analysis
        let complexity = self.analyze_complexity(&parsed)?;
        
        ValidationResult {
            syntactically_valid: true,
            type_checks: type_errors.is_empty(),
            tests_pass: test_results.all_pass(),
            complexity_acceptable: complexity.cyclomatic < 10,
            explanation_quality: self.score_explanation(&resp.explanation),
        }
    }
}
```

### Realistic Performance Profile

| Metric                   | V1 Target      | Measurement Method        |
| ------------------------ | -------------- | ------------------------- |
| **Parsing**              | 20MB/s         | Using rustpython-parser   |
| **HIR Generation**       | 40MB/s         | Post-parsing transform    |
| **Direct Transpilation** | 90% coverage   | On V1 safe subset         |
| **Type Inference**       | O(n)           | Linear in function size   |
| **Property Checking**    | <50ms/function | QuickCheck with 1k tests  |
| **MCP Round-trip**       | <2s            | Including validation      |
| **Binary Size**          | 4.5MB          | No embedded SMT solver    |
| **Memory Usage**         | 10x source     | Peak during transpilation |

### Analysis Output Example

```bash
$ depyler analyze examples/dataflow.py

Depyler Analysis Report v1.0.0
═══════════════════════════════

Source: examples/dataflow.py (2.3 KB)
Parse: 18.7 MB/s (rustpython-parser v0.3)

Complexity Metrics:
  Functions: 5
  Avg Cyclomatic: 3.2 (✓ Good)
  Max Cognitive: 8 (✓ Acceptable)
  Type Coverage: 100% (fully annotated)

Transpilation Feasibility:
  Direct: 4/5 functions (80%)
  MCP Required: 1 function (list comprehension)
  
Properties Verified:
  ✓ Type preservation (high confidence)
  ✓ Panic-free operations (static analysis)
  ✓ Termination (4/5 proven, 1 unknown)

Generated Artifacts:
  → dataflow.rs (3.1 KB)
  → dataflow.metrics.json
  → dataflow.tests.rs (property tests)
```

### Code Generation Example

**Input:**

```python
def binary_search(arr: List[int], target: int) -> int:
    """Find target in sorted array, return -1 if not found."""
    left, right = 0, len(arr) - 1
    
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1
```

**Output:**

```rust
/// Find target in sorted array, return -1 if not found.
/// 
/// Depyler: Transpiled from binary_search (dataflow.py:1-13)
/// Properties: panic_free (bounds checked), terminates (proven)
pub fn binary_search(arr: &[i32], target: i32) -> i32 {
    let mut left = 0i32;
    let mut right = (arr.len() as i32) - 1;
    
    while left <= right {
        let mid = (left + right) / 2;
        
        // Bounds check (Depyler: verified safe)
        if let Some(&arr_mid) = arr.get(mid as usize) {
            if arr_mid == target {
                return mid;
            } else if arr_mid < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        } else {
            // Unreachable: bounds maintained by algorithm
            unreachable!("Depyler: proven unreachable");
        }
    }
    
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    
    quickcheck! {
        fn prop_binary_search_finds_present(arr: Vec<i32>, idx: usize) -> TestResult {
            if arr.is_empty() {
                return TestResult::discard();
            }
            let mut sorted = arr.clone();
            sorted.sort();
            let idx = idx % sorted.len();
            let target = sorted[idx];
            let result = binary_search(&sorted, target);
            TestResult::from_bool(result >= 0 && sorted[result as usize] == target)
        }
    }
}
```

### Development Roadmap

**V1.0 (3 months): Core Transpilation**

- Safe subset transpilation with rustpython-parser
- PMAT integration for quality metrics
- Property-based test generation
- Basic MCP fallback for unsupported constructs

**V1.1 (6 months): Enhanced Type System**

- Lifetime inference for simple borrowing patterns
- `@dataclass` support with ownership inference
- Improved string handling (String vs &str)
- Contract-based verification

**V1.2 (9 months): Async & Advanced Patterns**

- `async`/`await` support
- Iterator protocol mapping
- Context managers → RAII
- Basic formal verification for critical properties

**V2.0 (12 months): Full Python Subset**

- Class inheritance (single)
- Generator expressions
- Limited dynamic dispatch
- SMT-based verification for core properties

This pragmatic approach delivers value immediately while building toward the
ambitious verification goals incrementally.
