# Interprocedural Analysis for Depyler

**Status**: Design Document  
**Created**: 2025-11-21  
**Priority**: Medium  
**Complexity**: High  

## Executive Summary

This document outlines the need for and design of interprocedural analysis in Depyler. Currently, the transpiler analyzes functions in isolation, which limits its ability to infer optimal borrowing strategies and type information across function boundaries. This limitation has been identified in multiple issues and test failures.

## Problem Statement

### Current Limitation: Single-Function Analysis

Depyler currently performs **intraprocedural analysis** - each function is analyzed independently without knowledge of:
- How it's called by other functions
- What signatures other functions have
- How parameters flow through function call chains
- Whether called functions mutate their parameters

### Impact

This limitation causes several issues:

1. **Mutability Inference Failures** (5 failing tests in `mutability_test.rs`)
   - Cannot detect when a field is passed to a function that mutates it
   - Example: `use_helper(state: State)` calls `update_dict(state.data, ...)` where `update_dict` takes `&mut HashMap`
   - Expected: `state: &mut State` (because field is mutated)
   - Generated: `state: &State` (no cross-function analysis)

2. **Type Inference Limitations** (DEPYLER-0289, DEPYLER-0291)
   - Cannot infer collection element types from usage in other functions
   - Cannot refine generic types based on how functions call each other
   - Falls back to `Value` type unnecessarily

3. **Suboptimal Borrowing Strategies**
   - Cannot determine if a called function needs `&T`, `&mut T`, or `T`
   - May generate unnecessary clones or moves
   - Cannot auto-insert borrow operators at call sites

## Concrete Examples

### Example 1: Field Mutation Through Function Call

**Python Code:**
```python
from dataclasses import dataclass

@dataclass
class State:
    data: dict[str, int]

def update_dict(data: dict[str, int], key: str, value: int) -> None:
    data[key] = value

def use_helper(state: State) -> None:
    update_dict(state.data, "key1", 100)
```

**Current Output:**
```rust
pub fn update_dict(data: &mut HashMap<String, i32>, key: &str, value: i32) {
    data.insert(key.to_string(), value);
}

pub fn use_helper(state: &State) {  // ❌ Should be &mut State
    update_dict(state.data, "key1", 100);  // ❌ Won't compile
}
```

**Expected Output:**
```rust
pub fn update_dict(data: &mut HashMap<String, i32>, key: &str, value: i32) {
    data.insert(key.to_string(), value);
}

pub fn use_helper(state: &mut State) {  // ✅ Mutable because field is mutated
    update_dict(&mut state.data, "key1", 100);  // ✅ Auto-inserted &mut
}
```

### Example 2: Parameter Passed to Multiple Functions

**Python Code:**
```python
def increment(counter: Counter, amount: int) -> None:
    counter.value += amount
    counter.total += amount

def process(counter: Counter) -> None:
    increment(counter, 5)
    increment(counter, 10)
```

**Current Output:**
```rust
pub fn increment(counter: &mut Counter, amount: i32) {
    counter.value += amount;
    counter.total += amount;
}

pub fn process(mut counter: Counter) {  // ❌ Takes ownership
    increment(counter, 5);  // ❌ Won't compile (moved value)
    increment(counter, 10);
}
```

**Expected Output:**
```rust
pub fn increment(counter: &mut Counter, amount: i32) {
    counter.value += amount;
    counter.total += amount;
}

pub fn process(counter: &mut Counter) {  // ✅ Borrows mutably
    increment(counter, 5);  // ✅ Passes reference
    increment(counter, 10);
}
```

### Example 3: List Parameter Mutation

**Python Code:**
```python
def modify_list(items: list[int]) -> None:
    items.append(42)
    items.append(100)

def use_helper(state: State) -> None:
    modify_list(state.numbers)
```

**Current Output:**
```rust
pub fn modify_list(items: &mut Vec<i32>) {
    items.push(42);
    items.push(100);
}

pub fn use_helper(state: &State) {  // ❌ Should be &mut State
    modify_list(state.numbers);  // ❌ Won't compile
}
```

**Expected Output:**
```rust
pub fn modify_list(items: &mut Vec<i32>) {
    items.push(42);
    items.push(100);
}

pub fn use_helper(state: &mut State) {  // ✅ Mutable because field is mutated
    modify_list(&mut state.numbers);  // ✅ Auto-inserted &mut
}
```

## Related Issues and Test Failures

### Test Failures (as of 2025-11-21)

From `cargo test --test mutability_test`:
- ✅ 41 tests passing (basic mutability detection works)
- ✅ 8 tests fixed by lifetime elision improvements
- ❌ 5 tests failing (require interprocedural analysis):
  1. `test_dict_parameter_mutation` - Field passed to mutating function
  2. `test_iter_mut_pattern` - Iterator mutation pattern
  3. `test_mutable_list_parameter` - List field mutation
  4. `test_mutable_object_with_primitive_parameters` - Object field mutation with mixed params
  5. `test_passing_mutable_object_to_helper` - Object passed through helper chain

### Related Issues

1. **DEPYLER-0289, DEPYLER-0291**: Type inference limitations
   - Mentioned in: `docs/issues/DEPYLER-0289-0292-analysis.md`
   - Quote: "Would require whole-program analysis, which increases complexity and compile time significantly"

2. **DEPYLER-0318**: Function Parameter Mutability Inference
   - Document: `docs/issues/DEPYLER-0318-mutability-inference.md`
   - Current implementation: Single-function analysis only
   - Missing: Cross-function mutation propagation

3. **DEPYLER-0269**: Function Parameter Borrowing - Missing Borrow Operator
   - Missing: Function signature lookup for auto-borrow insertion

## Design Proposal

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Transpilation Pipeline                    │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              Phase 1: Module-Level Collection                │
│  • Parse all functions in module                             │
│  • Build call graph                                          │
│  • Extract function signatures (params, return types)        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│         Phase 2: Intraprocedural Analysis (Per-Function)     │
│  • Local variable tracking                                   │
│  • Direct mutation detection                                 │
│  • Method call analysis                                      │
│  • Basic borrowing inference                                 │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│          Phase 3: Interprocedural Analysis (NEW)             │
│  • Function signature registry                               │
│  • Call site analysis                                        │
│  • Mutation propagation through calls                        │
│  • Borrowing constraint propagation                          │
│  • Iterative refinement until fixpoint                       │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│               Phase 4: Code Generation                       │
│  • Apply inferred borrowing strategies                       │
│  • Insert borrow operators at call sites                     │
│  • Generate optimized Rust code                              │
└─────────────────────────────────────────────────────────────┘
```

### Core Components

#### 1. Function Signature Registry

**Location**: `crates/depyler-core/src/interprocedural/signature_registry.rs` (new)

```rust
/// Registry of function signatures for interprocedural analysis
pub struct FunctionSignatureRegistry {
    /// Map from function name to its signature
    signatures: HashMap<String, FunctionSignature>,
    /// Call graph edges (caller -> callees)
    call_graph: HashMap<String, HashSet<String>>,
    /// Reverse call graph (callee -> callers)
    reverse_call_graph: HashMap<String, HashSet<String>>,
}

/// Complete signature information for a function
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Function name
    pub name: String,
    /// Parameter information
    pub params: Vec<ParamSignature>,
    /// Return type
    pub return_type: RustType,
    /// Whether function can fail
    pub can_fail: bool,
}

/// Parameter signature with borrowing information
#[derive(Debug, Clone)]
pub struct ParamSignature {
    /// Parameter name
    pub name: String,
    /// Rust type
    pub rust_type: RustType,
    /// Borrowing strategy
    pub borrowing: BorrowingStrategy,
    /// Whether parameter is mutated
    pub is_mutated: bool,
}
```

#### 2. Call Site Analyzer

**Location**: `crates/depyler-core/src/interprocedural/call_analyzer.rs` (new)

```rust
/// Analyzes function calls to propagate borrowing requirements
pub struct CallSiteAnalyzer {
    /// Function signature registry
    registry: Arc<FunctionSignatureRegistry>,
    /// Mutation propagation results
    mutations: HashMap<String, MutationInfo>,
}

impl CallSiteAnalyzer {
    /// Analyze a function call expression
    pub fn analyze_call(
        &mut self,
        caller_func: &str,
        callee_func: &str,
        args: &[HirExpr],
    ) -> CallAnalysisResult {
        // 1. Look up callee signature
        // 2. Match arguments to parameters
        // 3. Propagate mutation requirements to caller
        // 4. Record borrowing needs
    }

    /// Propagate mutation through field access
    /// e.g., if `update_dict(state.data, ...)` and `data` param is &mut,
    /// then `state` must be &mut in caller
    pub fn propagate_field_mutation(
        &mut self,
        root_var: &str,
        field_path: &[String],
        requires_mut: bool,
    ) -> PropagationResult {
        // Track that root_var needs &mut because field is mutated
    }
}

#[derive(Debug, Clone)]
pub struct CallAnalysisResult {
    /// Variables that need to be mutable in caller
    pub required_mutable_vars: HashSet<String>,
    /// Borrow operators to insert at call site
    pub borrow_insertions: Vec<BorrowInsertion>,
}

#[derive(Debug, Clone)]
pub struct BorrowInsertion {
    pub arg_index: usize,
    pub kind: BorrowKind,  // & or &mut
}
```

#### 3. Mutation Propagation Engine

**Location**: `crates/depyler-core/src/interprocedural/mutation_propagation.rs` (new)

```rust
/// Propagates mutation information across function boundaries
pub struct MutationPropagator {
    /// Initial mutation info from intraprocedural analysis
    local_mutations: HashMap<String, HashSet<String>>,
    /// Propagated mutations from callees
    propagated_mutations: HashMap<String, HashSet<String>>,
    /// Call graph for traversal
    call_graph: Arc<CallGraph>,
}

impl MutationPropagator {
    /// Run fixpoint iteration to propagate mutations
    pub fn propagate(&mut self) -> PropagationResult {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            // For each function in topological order
            for func_name in self.call_graph.topological_order() {
                let new_mutations = self.analyze_function_calls(func_name);
                if !new_mutations.is_empty() {
                    changed = true;
                    self.propagated_mutations
                        .entry(func_name.clone())
                        .or_default()
                        .extend(new_mutations);
                }
            }
        }

        PropagationResult {
            mutations: self.propagated_mutations.clone(),
            iterations,
            converged: !changed,
        }
    }

    /// Analyze calls in a function to propagate mutations upward
    fn analyze_function_calls(&self, func_name: &str) -> HashSet<String> {
        // For each call in this function:
        // 1. Check if callee mutates parameters
        // 2. If yes, trace back to caller's variables
        // 3. Mark those variables as needing mutation
    }
}
```

#### 4. Integration with Existing Analysis

**Location**: Update `crates/depyler-core/src/borrowing_context.rs`

```rust
impl BorrowingContext {
    /// Analyze function with interprocedural context
    pub fn analyze_function_interprocedural(
        &mut self,
        func: &HirFunction,
        type_mapper: &TypeMapper,
        signature_registry: &FunctionSignatureRegistry,  // NEW
    ) -> BorrowingAnalysisResult {
        // Phase 1: Local analysis (existing code)
        self.analyze_function(func, type_mapper);

        // Phase 2: Interprocedural analysis (NEW)
        self.analyze_cross_function_mutations(func, signature_registry);

        // Phase 3: Merge results
        self.determine_strategies_with_interprocedural(func, type_mapper)
    }

    /// NEW: Analyze function calls to detect cross-function mutations
    fn analyze_cross_function_mutations(
        &mut self,
        func: &HirFunction,
        registry: &FunctionSignatureRegistry,
    ) {
        for stmt in &func.body {
            self.analyze_stmt_for_calls(stmt, registry);
        }
    }

    /// NEW: Analyze calls in statements
    fn analyze_stmt_for_calls(
        &mut self,
        stmt: &HirStmt,
        registry: &FunctionSignatureRegistry,
    ) {
        // Look for Call expressions
        // Check callee signature
        // If argument is a field access and parameter is &mut:
        //   - Mark root variable as mutated
    }
}
```

### Implementation Phases

#### Phase 1: Foundation (1-2 weeks)

**Deliverables:**
1. `FunctionSignatureRegistry` implementation
2. Call graph construction
3. Basic function signature extraction
4. Integration points in transpilation pipeline

**Testing:**
- Unit tests for signature registry
- Call graph construction tests
- Signature extraction for simple cases

#### Phase 2: Call Site Analysis (2-3 weeks)

**Deliverables:**
1. `CallSiteAnalyzer` implementation
2. Argument-to-parameter matching
3. Field access tracking through calls
4. Borrow insertion logic

**Testing:**
- Test field mutation propagation
- Test nested field access
- Test multiple call chains

#### Phase 3: Mutation Propagation (2-3 weeks)

**Deliverables:**
1. `MutationPropagator` implementation
2. Fixpoint iteration algorithm
3. Topological ordering of call graph
4. Convergence detection

**Testing:**
- Test simple call chains
- Test recursive calls (should detect and handle)
- Test complex call patterns

#### Phase 4: Integration & Refinement (1-2 weeks)

**Deliverables:**
1. Integration with `BorrowingContext`
2. Update `LifetimeInference` to use interprocedural info
3. Auto-borrow insertion in code generation
4. Fix all 5 failing tests

**Testing:**
- All `mutability_test.rs` tests pass
- Performance benchmarks (ensure no significant slowdown)
- Edge case testing

### Performance Considerations

#### Time Complexity

- **Single-function analysis**: O(n) per function, O(n*m) total for m functions
- **Call graph construction**: O(n*m) for m functions with average n calls
- **Fixpoint iteration**: O(k * m * n) where k is iterations to convergence
  - Expected k < 10 for most programs
  - k = 1 for acyclic call graphs
  - k bounded by recursion depth for cyclic graphs

#### Space Complexity

- **Signature registry**: O(m) functions × O(p) parameters = O(m*p)
- **Call graph**: O(e) edges, typically O(m*c) where c is avg calls per function
- **Mutation maps**: O(m * v) where v is avg variables per function

#### Optimization Strategies

1. **Lazy analysis**: Only analyze functions that are actually called
2. **Caching**: Cache analysis results for unchanged functions
3. **Incremental updates**: When a function changes, only re-analyze affected callers
4. **Parallel analysis**: Functions without dependencies can be analyzed in parallel
5. **Early termination**: Stop propagation when no changes detected

### Alternative Approaches Considered

#### 1. Annotation-Based Approach

**Pros:**
- Simple to implement
- User has full control
- No performance overhead

**Cons:**
- Requires manual annotations
- Error-prone
- Not automatic

**Decision:** Use as fallback/override mechanism, not primary solution

#### 2. Whole-Program Analysis

**Pros:**
- Most accurate
- Can optimize across entire program

**Cons:**
- Very expensive for large codebases
- Long compile times
- Difficult to implement incrementally

**Decision:** Too expensive, use targeted interprocedural analysis instead

#### 3. Type-Based Heuristics

**Pros:**
- Fast
- No call graph needed

**Cons:**
- Inaccurate (lots of false positives/negatives)
- Doesn't handle all cases

**Decision:** Use as quick filter, not replacement for proper analysis

### Success Metrics

1. **Correctness**: All 5 failing mutability tests pass
2. **Performance**: < 20% increase in transpilation time for typical programs
3. **Code Quality**: Generated code compiles without borrowing errors
4. **Coverage**: Handles 95%+ of common function call patterns

### Future Extensions

1. **Cross-module analysis**: Analyze calls across module boundaries
2. **Trait-based analysis**: Handle trait methods and dynamic dispatch
3. **Lifetime propagation**: Propagate lifetime relationships across calls
4. **Escape analysis**: Determine if parameters escape function scope
5. **Alias analysis**: Track aliases and indirect mutations

## References

### Existing Documentation

- `docs/issues/DEPYLER-0289-0292-analysis.md` - Type inference limitations
- `docs/issues/DEPYLER-0318-mutability-inference.md` - Mutability inference design
- `docs/architecture.md` - Overall Depyler architecture

### Academic Literature

1. **Interprocedural Analysis**: Aho, Sethi, Ullman - "Compilers: Principles, Techniques, and Tools"
2. **Alias Analysis**: Hind - "Pointer Analysis: Haven't We Solved This Problem Yet?"
3. **Rust Borrowing**: Matsakis, Klock - "The Rust Programming Language"
4. **Call Graph Construction**: Grove, Chambers - "A Framework for Call Graph Construction Algorithms"

### Similar Systems

1. **Rust Compiler (rustc)**: MIR borrow checker with interprocedural information
2. **C++ Static Analyzers**: Clang Static Analyzer, Coverity
3. **Java Escape Analysis**: JVM optimization for object allocation

## Open Questions

1. **How to handle recursive functions?**
   - Bounded iteration with conservative fallback?
   - Detect strongly connected components in call graph?

2. **How to handle higher-order functions?**
   - Track function pointers and closures?
   - Conservative approximation?

3. **How to handle external functions?**
   - Require signatures in stubs?
   - Conservative defaults?

4. **How to handle dynamic dispatch?**
   - Analyze all possible implementations?
   - Use type bounds information?

5. **What's the right balance between precision and performance?**
   - Start with conservative approach?
   - Add refinements based on benchmarks?

## Conclusion

Interprocedural analysis is essential for generating idiomatic Rust code from Python. The proposed design balances accuracy with performance, using targeted analysis focused on mutation propagation and borrowing inference. The phased implementation approach allows for incremental development and testing, with clear success metrics at each stage.

The immediate benefit is fixing the 5 failing mutability tests, but the long-term value is enabling more sophisticated optimizations and better code quality across all transpiled programs.

---

**Next Steps:**
1. Review this design with team
2. Create JIRA epic for tracking
3. Implement Phase 1 (Foundation)
4. Measure performance on benchmark suite
5. Iterate based on results
