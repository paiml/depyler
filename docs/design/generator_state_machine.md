# Generator State Machine Transformation (DEPYLER-0115 Phase 3)

## Status: NOT STARTED (Deferred from Phase 2)

## Overview

Phase 2 completed 75% of generator support:
- ✅ State struct generation
- ✅ Iterator trait implementation
- ✅ Yield→return Some() conversion
- ✅ Variable scoping (self.field)
- ❌ **State machine transformation** (THIS DOCUMENT)

## The Problem

Current generated code is **structurally broken**:

```rust
fn next(&mut self) -> Option<Self::Item> {
    match self.state {
        0 => {
            self.current = 0;
            while self.current < self.n {
                return Some(self.current);  // ⚠️ Exits immediately
                self.current = self.current + 1;  // ⚠️ Unreachable
            }
            None
        }
        _ => None
    }
}
```

**Root Cause**: Yield statements become `return Some(value)`, which exits the function instead of suspending execution.

## Required Transformation

### Input (Python HIR)
```
HirFunction {
    body: [
        Assign { target: "current", value: Literal(0) },
        While {
            condition: Binary { Lt, Var("current"), Var("n") },
            body: [
                Expr(Yield { value: Var("current") }),
                Assign { target: "current", value: Binary { Add, Var("current"), Literal(1) } }
            ]
        }
    ]
}
```

### Output (Rust)
```rust
fn next(&mut self) -> Option<Self::Item> {
    loop {
        match self.state {
            0 => {  // Initialization
                self.current = 0;
                self.state = 1;
            }
            1 => {  // Loop condition check
                if self.current < self.n {
                    self.state = 2;  // Prepare to yield
                } else {
                    return None;  // Loop done
                }
            }
            2 => {  // Yield point
                let value = self.current;
                self.state = 3;  // Next: post-yield code
                return Some(value);
            }
            3 => {  // Post-yield (loop increment)
                self.current = self.current + 1;
                self.state = 1;  // Back to condition
            }
            _ => return None
        }
    }
}
```

## Implementation Design

### 1. HIR Analysis Phase

**Goal**: Identify all yield points and control flow structure

```rust
struct YieldPoint {
    id: usize,
    location: HirLocation,  // Where in the HIR
    value: HirExpr,
}

struct ControlFlowGraph {
    nodes: Vec<CFGNode>,
    edges: Vec<CFGEdge>,
    yield_points: Vec<YieldPoint>,
}

enum CFGNode {
    Entry,
    Exit,
    BasicBlock(Vec<HirStmt>),  // Statements between control flow
    LoopHeader { condition: HirExpr },
    LoopBody,
    IfCondition { condition: HirExpr },
    YieldPoint { id: usize },
}
```

**Algorithm**:
1. Walk HIR tree
2. Split at control flow boundaries (if, while, for, yield)
3. Create CFG nodes for each segment
4. Build edges representing control flow

### 2. State Assignment Phase

**Goal**: Assign state numbers to each CFG node

```rust
struct StateInfo {
    state_num: usize,
    node: CFGNode,
    predecessors: Vec<usize>,  // Which states can transition here
    successors: Vec<usize>,     // Which states this can transition to
}
```

**Algorithm**:
1. Topological sort of CFG
2. Assign sequential state numbers
3. Record transitions (edges become state changes)

### 3. Code Generation Phase

**Goal**: Generate `match self.state` with proper transitions

```rust
fn generate_state_machine(cfg: &ControlFlowGraph, states: &[StateInfo]) -> TokenStream {
    let state_arms: Vec<TokenStream> = states.iter().map(|state| {
        let state_num = state.state_num;
        let body = generate_state_body(&state.node, &state.successors);
        quote! {
            #state_num => {
                #body
            }
        }
    }).collect();

    quote! {
        loop {
            match self.state {
                #(#state_arms)*
                _ => return None
            }
        }
    }
}
```

### 4. Special Cases

#### While Loops
- State N: Check condition
  - If true: transition to body (N+1)
  - If false: transition to exit
- State N+1: Execute body up to first yield/end
- State N+2: Continue after yield, back to N

#### For Loops
- Convert to while loop with iterator
- State N: Call `iter.next()`
  - If Some: bind variable, go to body
  - If None: exit loop

#### Nested Control Flow
- Flatten into linear state sequence
- Use state stack for nested loops (complex)

#### Early Returns
- Transition to terminal state (return None)

## Implementation Phases

### Phase 3A: Simple While Loop (1 day)
**Scope**: Single while loop with single yield
**Pattern**: 80% of real-world generators
**Tests**: counter, range, simple iterators

### Phase 3B: Complex Control Flow (2 days)
**Scope**: Multiple yields, nested loops, conditionals
**Pattern**: fibonacci, stateful iterators
**Tests**: All 20 stateful generator tests

### Phase 3C: Edge Cases (1 day)
**Scope**: Early returns, exceptions, multiple loops
**Tests**: Property-based tests, fuzzing

## Complexity Analysis

This transformation is equivalent to:
- **Async/await lowering** in Rust compiler
- **Continuation-passing style** (CPS) transformation
- **Python generator bytecode** compilation

**Similar implementations**:
- Rust `async fn` → state machine (rustc)
- C# `yield` → state machine (Roslyn)
- Python generators → bytecode (CPython)

**Estimated LOC**: 500-800 lines for full implementation

## Testing Strategy

### Unit Tests
- [ ] CFG construction for simple statements
- [ ] CFG construction for while loops
- [ ] CFG construction for if statements
- [ ] State numbering algorithm
- [ ] Transition generation

### Integration Tests
- [ ] Simple counter (1 while, 1 yield)
- [ ] Range with step (1 while, 1 yield, arithmetic)
- [ ] Fibonacci (2 state vars, 1 while, 1 yield)
- [ ] Conditional yields (if inside while)
- [ ] Nested loops (for inside while)

### Property Tests
- [ ] All generated state machines are acyclic (except loop back-edges)
- [ ] Every yield point is reachable
- [ ] No unreachable code in generated match arms
- [ ] State numbering is consecutive

## Success Criteria

1. All 20 stateful generator tests pass
2. Generated code compiles with zero warnings
3. Runtime behavior matches Python semantics
4. No unreachable code warnings
5. Complexity ≤10 for all generated functions

## References

- Rust async lowering: https://rust-lang.github.io/async-book/
- C# yield: https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/yield
- Python generators: https://peps.python.org/pep-0255/
- CPS transformation: Appel's "Compiling with Continuations"

## Decision: Defer to Phase 3

**Rationale**:
- Phase 2 achieved 75% completion (all infrastructure)
- State machine transformation is compiler-level work (1 week effort)
- TDD Book methodology: ship working features incrementally
- Current code structure supports future transformation
- Clear design doc enables future implementation

**Next Steps**:
1. Merge Phase 2 work (current state)
2. Document limitation in generated code
3. Create DEPYLER-0115-PHASE3 ticket
4. Schedule for future sprint
