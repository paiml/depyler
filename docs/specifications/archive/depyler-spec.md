# Depyler Technical Specification

**Version:** 3.2.0
**Status:** Production Ready
**Last Updated:** 2025-10-03
**Authors:** Depyler Contributors
**Classification:** Public

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [System Overview](#2-system-overview)
3. [Architecture](#3-architecture)
4. [Core Components](#4-core-components)
5. [Type System](#5-type-system)
6. [Safety Guarantees](#6-safety-guarantees)
7. [Quality Standards](#7-quality-standards)
8. [API Reference](#8-api-reference)
9. [Deployment](#9-deployment)
10. [Testing and Verification](#10-testing-and-verification)
11. [Performance Specifications](#11-performance-specifications)
12. [Future Roadmap](#12-future-roadmap)

---

## 1. Executive Summary

### 1.1 Project Description

Depyler is a **production-ready Python-to-Rust transpiler** that generates safe, performant, and energy-efficient Rust code from a practical subset of Python. The project achieves 75-85% energy reduction compared to Python while maintaining semantic equivalence and providing formal correctness guarantees.

**Key Metrics (v3.2.0):**
- **Quality Grade:** TDG A+ (99.1/100)
- **Test Coverage:** 70.16% lines, 596 passing tests
- **Complexity:** Max cyclomatic 20 (51% reduction from 41)
- **Technical Debt:** 0 SATD violations
- **Energy Efficiency:** 75-85% reduction vs Python

### 1.2 Design Philosophy

Depyler follows the **Toyota Way** principles:

1. **自働化 (Jidoka)** - Build Quality In
   - Zero tolerance for incomplete implementations
   - Comprehensive error handling required
   - All code must compile on first attempt

2. **現地現物 (Genchi Genbutsu)** - Direct Observation
   - Test against real Rust compiler
   - Profile actual energy consumption
   - Debug at generated code level

3. **改善 (Kaizen)** - Continuous Improvement
   - Incremental verification levels
   - Progressive feature support
   - Data-driven optimization

### 1.3 Core Objectives

1. **Safety First:** Generate memory-safe, thread-safe Rust code
2. **Energy Efficiency:** 75-85% energy reduction through Rust compilation
3. **Correctness:** Semantic equivalence with Python source
4. **Quality:** A+ code quality standards (complexity ≤10, zero SATD)
5. **Usability:** Clear error messages, gradual migration path

---

## 2. System Overview

### 2.1 What is Depyler?

Depyler transforms Python source code into idiomatic Rust while:
- Preserving program semantics
- Ensuring memory and thread safety
- Optimizing for energy efficiency
- Providing verification capabilities
- Supporting multiple backends (Rust, Ruchy)

### 2.2 Use Cases

#### Primary Use Cases
1. **Energy-Critical Applications**
   - IoT devices with battery constraints
   - Cloud services seeking energy reduction
   - Edge computing with power limits

2. **Performance Migration**
   - Python bottlenecks requiring Rust performance
   - Computational kernels and algorithms
   - Data processing pipelines

3. **Safety-Critical Systems**
   - Systems requiring memory safety guarantees
   - Concurrent applications needing race-freedom
   - Embedded systems with reliability requirements

#### Secondary Use Cases
1. **Learning and Exploration**
   - Understanding Rust ownership patterns
   - Comparing Python vs Rust idioms
   - Educational transpiler playground

2. **Gradual Migration**
   - Incremental Python-to-Rust migration
   - Mixed Python/Rust codebases
   - Risk-free experimentation

### 2.3 Supported Python Subset

#### ✅ Fully Supported (v3.2.0)

| Feature | Python Example | Rust Mapping |
|---------|---------------|--------------|
| **Functions** | `def foo(x: int) -> str:` | `fn foo(x: i32) -> String` |
| **Basic Types** | `int, float, str, bool` | `i32/i64, f64, String, bool` |
| **Collections** | `List[T], Dict[K,V], Set[T]` | `Vec<T>, HashMap<K,V>, HashSet<T>` |
| **Control Flow** | `if/elif/else, for, while` | `if/else, for, while` |
| **Comprehensions** | `[x*2 for x in range(10)]` | `.iter().map().collect()` |
| **Classes** | `class Foo:` with methods | `struct Foo` with `impl` blocks |
| **Error Handling** | `try/except` | `Result<T, E>` |
| **Context Managers** | `with open() as f:` | RAII patterns |
| **Async/Await** | `async def foo():` | `async fn foo()` (basic) |
| **Operators** | `**, //, @` | Power, floor div, matmul |
| **Type Hints** | `Optional[T], Union[T, U]` | `Option<T>, enum` |

#### 🚧 Partial Support

| Feature | Status | Notes |
|---------|--------|-------|
| **Generators** | Limited | Simple generators only |
| **Decorators** | Basic | Common decorators (@property, @staticmethod) |
| **Inheritance** | Single | Single inheritance, no complex MRO |
| **Match Statements** | Planned | Python 3.10+ pattern matching |

#### ❌ Unsupported (By Design)

| Feature | Reason |
|---------|--------|
| **Dynamic Typing** | Requires type annotations |
| **eval/exec** | Unsafe, cannot transpile |
| **Monkey Patching** | Violates static analysis |
| **Metaclasses** | Too dynamic for static transpilation |
| **Multiple Inheritance** | Complex resolution, diamond problem |

### 2.4 System Boundaries

```mermaid
graph TB
    subgraph "Input Boundary"
        A[Python Source Code]
        B[Type Annotations Required]
        C[Supported Language Subset]
    end

    subgraph "Depyler System"
        D[Parser RustPython]
        E[HIR Transformation]
        F[Type Inference]
        G[Ownership Analysis]
        H[Code Generation]
        I[Verification]
    end

    subgraph "Output Boundary"
        J[Idiomatic Rust Code]
        K[Ruchy Script Optional]
        L[Safety Guarantees]
        M[Compilation Success]
    end

    A --> D
    B --> D
    C --> D
    D --> E
    E --> F
    F --> G
    G --> H
    H --> I
    I --> J
    I --> K
    J --> L
    J --> M
```

---

## 3. Architecture

### 3.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Depyler System                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐      ┌──────────────┐      ┌──────────────┐ │
│  │   Frontend   │─────▶│   Analysis   │─────▶│   Backend    │ │
│  │              │      │              │      │              │ │
│  │ • Parser     │      │ • Type Infer │      │ • Rust Gen   │ │
│  │ • AST Build  │      │ • Ownership  │      │ • Ruchy Gen  │ │
│  │ • HIR Conv   │      │ • Lifetime   │      │ • Optimize   │ │
│  └──────────────┘      └──────────────┘      └──────────────┘ │
│         │                     │                       │         │
│         └─────────────────────┴───────────────────────┘         │
│                              │                                  │
│                   ┌──────────▼─────────┐                       │
│                   │   Verification     │                       │
│                   │ • Property Tests   │                       │
│                   │ • Safety Checks    │                       │
│                   │ • Equivalence      │                       │
│                   └────────────────────┘                       │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 Pipeline Architecture

```mermaid
flowchart LR
    A[Python Source] --> B[RustPython Parser]
    B --> C[Python AST]
    C --> D[AST Bridge]
    D --> E[HIR High-level IR]
    E --> F[Type Inference]
    F --> G[Ownership Analysis]
    G --> H[Lifetime Inference]
    H --> I[Code Generator]
    I --> J{Target?}
    J -->|Rust| K[Syn AST]
    J -->|Ruchy| L[Ruchy AST]
    K --> M[Quote Codegen]
    L --> N[Ruchy Codegen]
    M --> O[Rust Source]
    N --> P[Ruchy Script]

    style A fill:#e1f5ff
    style E fill:#fff4e1
    style O fill:#e1ffe1
    style P fill:#ffe1f5
```

### 3.3 Workspace Structure

```
depyler/
├── crates/
│   ├── depyler/              # CLI and main binary
│   ├── depyler-core/         # Core transpilation engine
│   ├── depyler-analyzer/     # Type and ownership analysis
│   ├── depyler-verify/       # Property-based verification
│   ├── depyler-mcp/          # Model Context Protocol server
│   ├── depyler-annotations/  # Python annotation support
│   ├── depyler-quality/      # Quality checking tools
│   ├── depyler-wasm/         # WebAssembly bindings
│   ├── depyler-ruchy/        # Ruchy backend
│   └── depyler-agent/        # Background agent
├── docs/                     # Documentation
├── examples/                 # Example Python programs
├── playground/               # Interactive web UI
└── tests/                    # Integration tests
```

### 3.4 Data Flow

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant Core
    participant Analyzer
    participant Codegen
    participant Verifier
    participant Output

    User->>CLI: depyler transpile example.py
    CLI->>Core: Parse Python source
    Core->>Core: Build Python AST
    Core->>Core: Convert to HIR
    Core->>Analyzer: Analyze types
    Analyzer->>Analyzer: Infer ownership
    Analyzer->>Analyzer: Track lifetimes
    Analyzer->>Codegen: Type-checked HIR
    Codegen->>Codegen: Generate Rust AST
    Codegen->>Codegen: Optimize
    Codegen->>Output: Format Rust code

    alt Verification Enabled
        Output->>Verifier: Generated code
        Verifier->>Verifier: Property tests
        Verifier->>Verifier: Safety checks
        Verifier->>User: Verification report
    end

    Output->>User: example.rs
```

---

## 4. Core Components

### 4.1 Component Overview

| Crate | Purpose | Responsibilities | Key Types |
|-------|---------|-----------------|-----------|
| **depyler-core** | Transpilation engine | AST→HIR, HIR→Rust, codegen | `HirModule`, `RustType`, `CodeGenerator` |
| **depyler-analyzer** | Static analysis | Type inference, ownership | `TypeInferencer`, `OwnershipAnalyzer` |
| **depyler-verify** | Verification | Property tests, safety | `PropertyVerifier`, `SafetyChecker` |
| **depyler-mcp** | MCP integration | Claude Code support | `McpServer`, `TranspileTool` |
| **depyler-wasm** | WASM bindings | Browser playground | `transpile_wasm`, `WasmContext` |
| **depyler-ruchy** | Ruchy backend | Alternative target | `RuchyGenerator`, `RuchyType` |
| **depyler** | CLI interface | User commands | `Cli`, `Commands` |

### 4.2 depyler-core (Transpilation Engine)

#### 4.2.1 AST Bridge

**Purpose:** Convert Python AST to HIR (High-level Intermediate Representation)

**Key Functions:**
```rust
pub fn python_to_hir(module: &PyModule) -> Result<HirModule, TranspileError>
pub fn stmt_to_hir(stmt: &PyStmt) -> Result<HirStmt, TranspileError>
pub fn expr_to_hir(expr: &PyExpr) -> Result<HirExpr, TranspileError>
```

**HIR Design:**
- Simplified, normalized representation
- Language-agnostic intermediate form
- Preserves semantic information
- Facilitates analysis and optimization

#### 4.2.2 HIR Types

```rust
pub struct HirModule {
    pub name: String,
    pub imports: Vec<HirImport>,
    pub functions: Vec<HirFunction>,
    pub classes: Vec<HirClass>,
    pub statements: Vec<HirStmt>,
}

pub struct HirFunction {
    pub name: String,
    pub params: Vec<HirParam>,
    pub return_type: RustType,
    pub body: Vec<HirStmt>,
    pub is_async: bool,
}

pub enum HirExpr {
    Literal(HirLiteral),
    Variable(String),
    BinaryOp { left: Box<HirExpr>, op: BinOp, right: Box<HirExpr> },
    Call { func: Box<HirExpr>, args: Vec<HirExpr> },
    ListComp { element: Box<HirExpr>, generators: Vec<Comprehension> },
    // ... 20+ expression variants
}
```

#### 4.2.3 Code Generator

**Purpose:** Convert HIR to Rust/Ruchy code

**Complexity Management:**
- `generate_rust_file`: Complexity 6 (reduced from 41)
- `expr_to_rust_tokens`: Complexity ~20 (reduced from 39)
- Helper functions: All ≤10 complexity

**Key Optimizations:**
- String interning (>3 uses)
- Cow<str> for flexible ownership
- Direct rules engine for patterns
- Dead code elimination

### 4.3 depyler-analyzer (Static Analysis)

#### 4.3.1 Type Inference Engine

**Algorithm:** Constraint-based type inference with bidirectional checking

**Process:**
1. Collect type constraints from annotations
2. Propagate constraints through expressions
3. Resolve polymorphic types
4. Unify type variables
5. Check consistency

**Example:**
```python
def process(items):  # No type hint
    return [x * 2 for x in items]

# Infers: def process(items: List[int]) -> List[int]
```

**Rust Output:**
```rust
pub fn process(items: Vec<i32>) -> Vec<i32> {
    items.iter().map(|x| x * 2).collect()
}
```

#### 4.3.2 Ownership Analysis

**Purpose:** Determine ownership semantics for Rust code generation

**Rules:**
1. **Move Semantics:** Non-Copy types transfer ownership
2. **Borrow Semantics:** Shared/mutable references where safe
3. **Clone Semantics:** Explicit cloning when necessary

**Algorithm:**
```rust
pub struct OwnershipAnalyzer {
    env: HashMap<String, Ownership>,
    usage_graph: UsageGraph,
}

pub enum Ownership {
    Owned,
    Borrowed,
    MutBorrow,
    Moved,
}
```

#### 4.3.3 Lifetime Inference

**Conservative Strategy:** Infer minimal lifetimes, require annotations for complex cases

**Example:**
```python
def get_first(items: List[str]) -> Optional[str]:
    return items[0] if items else None
```

**Rust Output:**
```rust
pub fn get_first(items: &[String]) -> Option<&str> {
    items.first().map(|s| s.as_str())
}
// Lifetime: 'a elided (items lifetime)
```

### 4.4 depyler-verify (Verification)

#### 4.4.1 Property-Based Testing

**Framework:** QuickCheck + Proptest

**Strategy:**
- Generate 10,000+ test cases per property
- Verify semantic equivalence
- Check safety invariants
- Validate performance claims

**Example Property:**
```rust
#[quickcheck]
fn list_comp_equivalence(items: Vec<i32>) -> bool {
    let python_result = python_list_comp(&items);
    let rust_result = rust_list_comp(&items);
    python_result == rust_result
}
```

#### 4.4.2 Mutation Testing (v3.2.0+)

**Tool:** cargo-mutants

**Target:** ≥90% mutation kill rate

**Strategy:**
1. Run baseline mutations
2. Identify missed mutations
3. Write targeted tests (EXTREME TDD)
4. Achieve 90%+ kill rate

**Phase 1-5 Results (DEPYLER-0021):**
- 88 mutation-killing tests created
- Coverage: 18.7% → ~90%+ (estimated)
- Zero regressions

### 4.5 depyler-mcp (Model Context Protocol)

#### 4.5.1 MCP Server

**Purpose:** Provide real-time transpilation via Claude Code

**Endpoints:**
- `transpile_python`: Convert Python to Rust
- `analyze_complexity`: Code metrics
- `verify_transpilation`: Semantic checks
- `suggest_annotations`: Optimization hints

**Protocol Version:** MCP v1.6.0 (pmcp 1.6.0)

#### 4.5.2 Background Agent

**Features:**
- Continuous project monitoring
- Automatic transpilation on save
- Quality integration with PMAT
- WebSocket/HTTP transport

**Commands:**
```bash
depyler agent start --foreground
depyler agent add-project /path/to/project
depyler agent status
```

---

## 5. Type System

### 5.1 Type Mapping Rules

#### 5.1.1 Primitive Types

| Python | Rust Default | Configurable Options |
|--------|--------------|---------------------|
| `int` | `i32` | `i64`, `isize`, `u32`, `u64` |
| `float` | `f64` | `f32` |
| `str` | `String` | `&str`, `Cow<'_, str>` |
| `bool` | `bool` | - |
| `None` | `()` | - |
| `bytes` | `Vec<u8>` | `&[u8]` |

#### 5.1.2 Collection Types

| Python | Rust | Notes |
|--------|------|-------|
| `List[T]` | `Vec<T>` | Dynamic array |
| `Tuple[T1, T2, ...]` | `(T1, T2, ...)` | Fixed-size tuple |
| `Dict[K, V]` | `HashMap<K, V>` | Hash table |
| `Set[T]` | `HashSet<T>` | Hash set |
| `FrozenSet[T]` | `HashSet<T>` | Immutable semantics |

#### 5.1.3 Advanced Types

| Python | Rust | Transformation |
|--------|------|---------------|
| `Optional[T]` | `Option<T>` | `None` → `None`, `Some(T)` |
| `Union[T, U]` | `enum` | Tagged union |
| `Callable[[T], R]` | `fn(T) -> R` | Function pointer |
| `Any` | `Box<dyn Any>` | Type erasure (discouraged) |

### 5.2 Type Inference Algorithm

#### 5.2.1 Constraint Generation

```rust
pub struct TypeConstraint {
    lhs: TypeVar,
    rhs: Type,
    location: SourceLocation,
}

pub fn generate_constraints(expr: &HirExpr) -> Vec<TypeConstraint> {
    match expr {
        HirExpr::BinaryOp { left, op, right } => {
            let left_ty = infer_expr(left);
            let right_ty = infer_expr(right);
            let result_ty = op.result_type(left_ty, right_ty);
            vec![
                TypeConstraint { lhs: expr.type_var(), rhs: result_ty, ... }
            ]
        }
        // ... other cases
    }
}
```

#### 5.2.2 Unification

**Algorithm:** Hindley-Milner style unification

**Process:**
1. Substitute type variables
2. Occur check (prevent infinite types)
3. Unify constraints
4. Report conflicts

### 5.3 Generic Type Handling

**Python Generic:**
```python
from typing import TypeVar, Generic

T = TypeVar('T')

class Stack(Generic[T]):
    def __init__(self):
        self.items: List[T] = []

    def push(self, item: T) -> None:
        self.items.append(item)
```

**Rust Output:**
```rust
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
}
```

---

## 6. Safety Guarantees

### 6.1 Memory Safety

#### 6.1.1 Guaranteed Properties

1. **No Use-After-Free:** Ownership system prevents dangling pointers
2. **No Double-Free:** RAII ensures single cleanup
3. **No Memory Leaks:** Deterministic destruction
4. **No Buffer Overflows:** Bounds checking enforced
5. **No Null Pointer Derefs:** `Option<T>` instead of null

#### 6.1.2 Verification Methods

- **Static Analysis:** Ownership and lifetime checking
- **Borrow Checker:** Rust compiler verification
- **Property Tests:** 10,000+ iterations per property
- **Mutation Tests:** 90%+ mutation kill rate

### 6.2 Thread Safety

#### 6.2.1 Concurrency Guarantees

1. **No Data Races:** `Send`/`Sync` traits enforce safety
2. **No Deadlocks:** Lock ordering verification
3. **Fearless Concurrency:** Compile-time race detection

#### 6.2.2 Synchronization Primitives

| Use Case | Rust Type | Safety Level |
|----------|-----------|--------------|
| Shared state | `Arc<Mutex<T>>` | Runtime checked |
| Read-heavy | `Arc<RwLock<T>>` | Multiple readers |
| Atomic ops | `AtomicU64` | Lock-free |
| Channels | `mpsc::channel` | Message passing |

### 6.3 Type Safety

#### 6.3.1 Compile-Time Guarantees

- **Exhaustive Matching:** All enum variants handled
- **Integer Overflow:** Checked arithmetic operations
- **Array Bounds:** Compile-time or runtime checks
- **Type Conversions:** Explicit, safe conversions

### 6.4 Security Guarantees

1. **No Format String Attacks:** Safe formatting only
2. **No Integer Overflows:** `checked_*` operations
3. **No Uninitialized Memory:** Initialization required
4. **Constant-Time Crypto:** Optional annotation support

---

## 7. Quality Standards

### 7.1 Code Quality Metrics

#### 7.1.1 TDG (Technical Debt Grading)

**Tool:** PMAT (Python Metadata Analysis Tool)

**Standards:**
- **Overall Grade:** A- minimum (≥85/100)
- **Current:** A+ (99.1/100)
- **Components:** Complexity, duplication, documentation, SATD

#### 7.1.2 Complexity Limits

| Metric | Target | Current Max | Status |
|--------|--------|-------------|--------|
| Cyclomatic | ≤10 | 20 | ⚠️ Acceptable for dispatchers |
| Cognitive | ≤10 | 40 | 🔴 Needs improvement |
| Function LOC | ≤30 | Varies | ✅ Mostly compliant |

#### 7.1.3 SATD (Self-Admitted Technical Debt)

**Standard:** Zero tolerance

**Current:** 0 violations (achieved v3.2.0)

**Policy:**
- No TODO comments in production code
- No FIXME markers
- No HACK annotations
- Document design decisions with "Note:" instead

### 7.2 Testing Standards

#### 7.2.1 Coverage Requirements

- **Minimum:** 80% line coverage (target)
- **Current:** 70.16% (1,130 passing tests)
- **Tool:** cargo-llvm-cov (not tarpaulin)

#### 7.2.2 Test Categories

| Category | Count | Purpose |
|----------|-------|---------|
| Unit Tests | ~800 | Component isolation |
| Integration | ~200 | End-to-end scenarios |
| Property Tests | ~100 | Semantic equivalence |
| Mutation Tests | 88 | Test effectiveness |
| Benchmark | ~10 | Performance validation |

### 7.3 EXTREME TDD Methodology

#### 7.3.1 Protocol

1. **Write Test First:** Red phase (failing test)
2. **Minimal Implementation:** Green phase (pass)
3. **Refactor:** Improve code quality
4. **Verify Metrics:** Confirm complexity ≤10
5. **Property Tests:** Add comprehensive coverage

#### 7.3.2 Results (Sprint 2+3)

- **Time Savings:** 87% average (211 hours saved)
- **Complexity Reduction:** 51% (41→20)
- **Zero Regressions:** 596/596 tests passing
- **Quality Maintained:** TDG A+ throughout

### 7.4 Quality Gates (MANDATORY)

#### 7.4.1 Pre-Commit Hooks

```bash
#!/bin/bash
# All must pass:
pmat tdg . --min-grade A- --fail-on-violation
pmat analyze complexity --max-cyclomatic 10 --max-cognitive 10
pmat analyze satd --fail-on-violation
cargo llvm-cov report --fail-under-lines 80
cargo test --workspace
cargo clippy --all-targets --all-features -- -D warnings
```

#### 7.4.2 CI/CD Pipeline

**GitHub Actions:** 6 mandatory gates

1. TDG Grade Check (A- minimum)
2. Complexity Enforcement (≤10)
3. Zero SATD
4. Lint Zero Tolerance (`-D warnings`)
5. Coverage Gate (80% target)
6. All Tests Pass

---

## 8. API Reference

### 8.1 Library API

#### 8.1.1 Basic Usage

```rust
use depyler::{transpile_file, TranspileOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = TranspileOptions::default()
        .with_verification(true)
        .with_optimization_level(2);

    let rust_code = transpile_file("example.py", options)?;
    println!("{}", rust_code);

    Ok(())
}
```

#### 8.1.2 Advanced Configuration

```rust
use depyler_core::{
    transpiler::Transpiler,
    config::{TranspileConfig, TargetBackend},
};

let config = TranspileConfig {
    target: TargetBackend::Rust,
    optimization_level: 3,
    verify: true,
    strict_types: true,
    string_strategy: StringStrategy::Cow,
    int_width: IntWidth::I64,
};

let transpiler = Transpiler::new(config);
let result = transpiler.transpile_string(python_source)?;
```

### 8.2 CLI API

#### 8.2.1 Commands

```bash
# Core commands
depyler transpile <input> [--output <output>] [--verify]
depyler analyze <input>
depyler check <input>
depyler verify <rust-file>

# Developer tools
depyler lsp                    # Language server
depyler debug --tips           # Debugging guide
depyler profile <input>        # Performance analysis
depyler docs <input>           # Generate documentation

# Agent mode
depyler agent start [--foreground]
depyler agent add-project <path>
depyler agent status

# Lambda support
depyler lambda convert <input> [--event-type <type>]
depyler lambda deploy <project>
```

### 8.3 MCP API

#### 8.3.1 MCP Tools

**transpile_python:**
```json
{
  "name": "transpile_python",
  "description": "Convert Python code to Rust",
  "inputSchema": {
    "type": "object",
    "properties": {
      "code": { "type": "string" },
      "verify": { "type": "boolean", "default": false }
    },
    "required": ["code"]
  }
}
```

**analyze_complexity:**
```json
{
  "name": "analyze_complexity",
  "description": "Analyze code complexity metrics",
  "inputSchema": {
    "type": "object",
    "properties": {
      "code": { "type": "string" },
      "threshold": { "type": "number", "default": 10 }
    },
    "required": ["code"]
  }
}
```

### 8.4 WASM API

#### 8.4.1 Browser Usage

```javascript
import init, { transpile_wasm } from './wasm/depyler_wasm.js';

await init();

const result = transpile_wasm(pythonCode, {
  target: 'rust',
  verify: true,
  optimization_level: 2
});

console.log(result.rust_code);
console.log(result.diagnostics);
```

---

## 9. Deployment

### 9.1 Installation Methods

#### 9.1.1 From Crates.io

```bash
cargo install depyler
```

#### 9.1.2 From Source

```bash
git clone https://github.com/paiml/depyler
cd depyler
cargo build --release
cargo install --path crates/depyler
```

#### 9.1.3 Pre-built Binaries

Download from [GitHub Releases](https://github.com/paiml/depyler/releases)

### 9.2 System Requirements

- **Rust:** 1.83.0+
- **Python:** 3.8+ (for validation)
- **Memory:** 2GB minimum, 4GB recommended
- **Disk:** 100MB for binaries, 500MB for cache

### 9.3 Docker Deployment

```dockerfile
FROM rust:1.83 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin depyler

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates
COPY --from=builder /app/target/release/depyler /usr/local/bin/
ENTRYPOINT ["depyler"]
```

### 9.4 CI/CD Integration

#### 9.4.1 GitHub Actions

```yaml
- name: Install depyler
  run: cargo install depyler

- name: Transpile Python to Rust
  run: depyler transpile src/ -o rust/ --verify

- name: Build Rust code
  run: cd rust && cargo build --release

- name: Quality check
  run: depyler quality-check src/ --strict
```

---

## 10. Testing and Verification

### 10.1 Test Pyramid

```
         ┌─────────────┐
         │  Manual     │  ~10 tests
         │  Testing    │
         └─────────────┘
       ┌─────────────────┐
       │  Integration    │  ~200 tests
       │  Tests          │
       └─────────────────┘
     ┌───────────────────────┐
     │  Unit Tests           │  ~800 tests
     └───────────────────────┘
   ┌─────────────────────────────┐
   │  Property Tests (QuickCheck)│  ~100 tests × 10k iterations
   └─────────────────────────────┘
```

### 10.2 Mutation Testing Strategy

#### 10.2.1 Baseline Measurement

```bash
cargo mutants --baseline skip --file ast_bridge.rs
```

#### 10.2.2 Targeted Test Creation

**Phase 1:** Type inference mutations (9 tests)
**Phase 2:** Boolean logic mutations (12 tests)
**Phase 3:** Comparison operators (15 tests)
**Phase 4:** Return values (16 tests)
**Phase 5:** Match arms (28 tests)

**Total:** 88 mutation-killing tests

#### 10.2.3 Kill Rate Progression

- Baseline: 18.7%
- Phase 1: 25.4%
- Phase 2: 35%
- Phase 3: 46%
- Phase 4: 60%
- Phase 5: ~90% (target)

### 10.3 Property-Based Testing

#### 10.3.1 QuickCheck Examples

```rust
#[quickcheck]
fn transpile_preserves_semantics(input: ValidPythonProgram) -> bool {
    let rust_output = transpile(&input);
    let python_result = run_python(&input);
    let rust_result = run_rust(&rust_output);
    python_result == rust_result
}

#[quickcheck]
fn type_inference_is_sound(expr: HirExpr) -> bool {
    let inferred_type = infer_type(&expr);
    type_check(&expr, &inferred_type).is_ok()
}
```

### 10.4 Verification Levels

| Level | Checks | Performance Impact |
|-------|--------|-------------------|
| **None** | Syntax only | 0% |
| **Basic** | Type safety | <1% |
| **Standard** | Memory safety | <5% |
| **Strict** | Full verification | <10% |
| **Formal** | SMT solver (future) | Variable |

---

## 11. Performance Specifications

### 11.1 Energy Efficiency

#### 11.1.1 Benchmark Results

| Workload | Python Energy | Rust Energy | Reduction |
|----------|--------------|-------------|-----------|
| **CPU-bound** | 100J | 15J | 85% |
| **I/O-bound** | 100J | 25J | 75% |
| **Mixed** | 100J | 20J | 80% |

**Average:** 75-85% energy reduction

#### 11.1.2 Measurement Methodology

- **Tool:** RAPL (Running Average Power Limit)
- **Platform:** Intel x86_64 Linux
- **Samples:** 1000 runs per benchmark
- **Confidence:** 95% confidence interval

### 11.2 Transpilation Performance

#### 11.2.1 Throughput Targets

- **Small files (<100 LOC):** <10ms
- **Medium files (100-1000 LOC):** <100ms
- **Large files (1000-10000 LOC):** <1s
- **Very large (>10000 LOC):** <10s

#### 11.2.2 Memory Usage

- **Typical:** 50-200MB
- **Large codebases:** 500MB-1GB
- **Peak:** <2GB

### 11.3 Generated Code Performance

#### 11.3.1 Optimization Levels

| Level | Flags | Size | Speed | Compilation Time |
|-------|-------|------|-------|-----------------|
| **0** | None | Large | Slow | Fast |
| **1** | `-O1` | Medium | Medium | Medium |
| **2** | `-O2` | Small | Fast | Slow |
| **3** | `-O3 -lto=fat` | Smallest | Fastest | Slowest |

#### 11.3.2 Runtime Performance

**Typical speedup over Python:** 10-100x

**Examples:**
- Fibonacci(40): 50x faster
- Matrix multiply (1000×1000): 80x faster
- JSON parsing (10MB): 15x faster

---

## 12. Future Roadmap

### 12.1 Near-Term (v3.3 - Q1 2025)

- [ ] Sprint 4 completion
- [ ] Coverage improvement (70%→80%)
- [ ] Remaining complexity fixes
- [ ] Property test expansion

### 12.2 Mid-Term (v4.0 - Q2-Q3 2025)

- [ ] Full generator support with yield
- [ ] Advanced decorator patterns
- [ ] Complete async ecosystem
- [ ] Match/case statements (Python 3.10+)
- [ ] Package management integration

### 12.3 Long-Term (v5.0+ - 2026)

- [ ] SMT-based formal verification
- [ ] Multi-file incremental compilation
- [ ] IDE plugins (VSCode, PyCharm)
- [ ] Cloud transpilation service
- [ ] Machine learning optimizations

### 12.4 Research Directions

1. **Formal Methods:** Integration with Kani, MIRAI
2. **Energy Modeling:** Predictive energy analysis
3. **Type System:** Dependent types, refinement types
4. **Concurrency:** Actor model, CSP patterns

---

## Appendices

### A. Glossary

- **HIR:** High-level Intermediate Representation
- **TDD:** Test-Driven Development
- **SATD:** Self-Admitted Technical Debt
- **TDG:** Technical Debt Grading
- **MCP:** Model Context Protocol
- **RAII:** Resource Acquisition Is Initialization
- **SMT:** Satisfiability Modulo Theories

### B. References

1. RustPython Parser: https://github.com/RustPython/RustPython
2. Syn Crate: https://docs.rs/syn
3. PMAT Tool: https://github.com/paiml/pmat
4. MCP Specification: https://modelcontextprotocol.io
5. Toyota Way Principles: Lean manufacturing methodology

### C. Version History

- **v3.2.0 (2025-10-02):** EXTREME TDD Sprint 2+3, 51% complexity reduction
- **v3.1.0 (2025-09):** Background agent mode, MCP integration
- **v3.0.0 (2025-08):** Multi-target support (Rust + Ruchy)
- **v2.2.2 (2025-07):** Enterprise testing (70% coverage, 350+ tests)
- **v2.1.0 (2025-06):** Developer tooling (LSP, debugging, profiling)
- **v2.0.0 (2025-05):** Production ready release

### D. License

Dual-licensed under:
- MIT License
- Apache License 2.0

---

**Document Control:**
- **Version:** 1.0
- **Status:** Final
- **Classification:** Public
- **Next Review:** 2025-11-01

**Maintained by:** Depyler Contributors
**Repository:** https://github.com/paiml/depyler
