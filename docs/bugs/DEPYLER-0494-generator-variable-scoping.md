# DEPYLER-0494: Generator Variable Scoping Bug

**Status**: Open
**Priority**: P0 - CRITICAL
**Severity**: STOP THE LINE
**Created**: 2025-11-23
**Assigned**: Claude

## Problem Statement

Python generators with `yield` transpile to Rust state machines with **incorrect variable scoping**. Variables declared in one match arm (state:0) are not accessible in subsequent arms (state:1), causing "cannot find value in this scope" compiler errors.

**Impact**: Breaks ALL Python generators with yield - a fundamental language feature.

## Reproduction

**Python Input** (`fibonacci_generator` from fibonacci.py):
```python
def fibonacci_generator(limit=None):
    """Generate Fibonacci numbers as an iterator."""
    a, b = 0, 1
    count = 0
    while limit is None or count < limit:
        yield a
        a, b = b, a + b
        count += 1
```

**Generated Rust** (fibonacci.rs:74-112):
```rust
pub fn fibonacci_generator(limit: &Option<i32>) -> impl Iterator<Item = Iterator<i32>> {
    FibonacciGeneratorState {
        state: 0,
        count: 0,
        limit: limit,
    }
}

impl Iterator for FibonacciGeneratorState {
    type Item = Iterator<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                let (mut a, mut b) = (0, 1);  // ← DECLARED HERE
                self.count = 0;
                self.state = 1;
                self.next()
            }
            1 => {
                if (self.limit.is_none()) || (self.count < self.limit) {
                    let result = a;  // ← ERROR: `a` not in scope
                    (a, b) = (b, a + b);  // ← ERROR: `a`, `b` not in scope
                    self.count = self.count + 1;
                    return Some(result);
                } else {
                    self.state = 2;
                    None
                }
            }
            _ => None,
        }
    }
}
```

**Compilation Errors** (6 errors):
```
error[E0425]: cannot find value `a` in this scope
   --> fibonacci.rs:100:34
    |
100 |                     let result = a;
    |                                  ^
help: the binding `a` is available in a different scope in the same function
   --> fibonacci.rs:93:26
    |
 93 |                 let (mut a, mut b) = (0, 1);
    |                          ^
```

## Root Cause Analysis

### Current (Broken) Implementation

The generator transpiler creates a state machine but incorrectly places initialization variables in the wrong scope:

**State 0** (initialization):
```rust
let (mut a, mut b) = (0, 1);  // Local to this match arm only!
```

**State 1** (yield loop):
```rust
let result = a;  // ERROR: `a` doesn't exist here
```

### Why This Happens

Generator variables that need to persist across yields are being treated as **local variables** instead of **state machine fields**.

**Correct Architecture**:
- Variables that persist across yields → struct fields
- Variables local to one yield → local variables

**Current Bug**:
- ALL variables → local to first match arm
- No fields added to state struct

## Solution Design

### Phase 1: Identify Generator Variables

**Algorithm**:
1. Parse generator function body
2. Identify all variables assigned before first `yield`
3. Identify all variables used after any `yield`
4. Variables in both sets → **generator state variables** (must be struct fields)
5. Variables in only one set → local variables

**Example**:
```python
def fibonacci_generator(limit=None):
    a, b = 0, 1  # ← Used after yield → STATE VARIABLE
    count = 0    # ← Used after yield → STATE VARIABLE
    while limit is None or count < limit:
        yield a
        a, b = b, a + b  # ← Reassigned after yield → STATE VARIABLE
        count += 1       # ← Reassigned after yield → STATE VARIABLE
```

### Phase 2: Generate Correct State Struct

**Before** (broken):
```rust
#[derive(Debug)]
struct FibonacciGeneratorState {
    state: usize,
    count: i32,  // Only parameter captured
    limit: Option<i32>,
}
```

**After** (correct):
```rust
#[derive(Debug)]
struct FibonacciGeneratorState {
    state: usize,
    // Generator state variables
    a: i32,
    b: i32,
    count: i32,
    // Parameters
    limit: Option<i32>,
}
```

### Phase 3: Update State Machine Logic

**Before** (broken):
```rust
match self.state {
    0 => {
        let (mut a, mut b) = (0, 1);  // ← Local variable
        self.count = 0;
        self.state = 1;
        self.next()
    }
    1 => {
        let result = a;  // ← ERROR
    }
}
```

**After** (correct):
```rust
match self.state {
    0 => {
        // Initialize state fields
        self.a = 0;
        self.b = 1;
        self.count = 0;
        self.state = 1;
        self.next()
    }
    1 => {
        // Access state fields
        let result = self.a;
        let temp_b = self.b;
        self.b = self.a + self.b;
        self.a = temp_b;
        self.count = self.count + 1;
        return Some(result);
    }
}
```

## Affected Code Locations

**Primary**:
- `crates/depyler-core/src/rust_gen/stmt_gen.rs` - Generator transpilation logic
- `crates/depyler-core/src/hir.rs` - Generator HIR representation

**Secondary**:
- `crates/depyler-core/src/ast_bridge/converters.rs` - Yield expression handling

## Test Plan

### RED Phase: Create Failing Tests

```rust
// crates/depyler-core/tests/depyler_0494_generator_scoping.rs

#[test]
fn test_generator_variable_scoping() {
    let python = r#"
def fibonacci_gen():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should contain state struct with a and b fields
    assert!(
        rust.contains("a:") && rust.contains("b:"),
        "State struct must contain generator variables as fields"
    );

    // Should access via self.a, self.b
    assert!(
        rust.contains("self.a") && rust.contains("self.b"),
        "Must access generator variables via self"
    );

    // Should NOT declare as local variables in match arm
    assert!(
        !rust.contains("let (mut a, mut b) = (0, 1);") ||
        !rust.contains("match self.state"),
        "Must not declare generator variables as match arm locals"
    );
}

#[test]
fn test_generator_compiles() {
    let python = r#"
def fib_gen(limit=None):
    a, b = 0, 1
    count = 0
    while limit is None or count < limit:
        yield a
        a, b = b, a + b
        count += 1
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Write to temp file and compile
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(rust.as_bytes()).unwrap();

    // Must compile without errors
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--deny=warnings")
        .arg(file.path())
        .output()
        .expect("Failed to run rustc");

    assert!(
        output.status.success(),
        "Generated code must compile:\n{}\n\nErrors:\n{}",
        rust,
        String::from_utf8_lossy(&output.stderr)
    );
}
```

### GREEN Phase: Implementation

**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs`

**Algorithm**:
1. **Parse generator**: Identify all yield statements
2. **Collect state variables**: Find variables assigned before first yield and used after any yield
3. **Generate state struct**: Add state variables as fields
4. **Generate initialization**: Set fields in state:0
5. **Generate yield logic**: Access via self.field

### REFACTOR Phase: Quality Gates

- Cyclomatic complexity ≤10
- Test coverage ≥80%
- All examples re-transpile successfully
- No regression in existing tests

## Acceptance Criteria

✅ **Correctness**:
- [ ] fibonacci_generator compiles without errors
- [ ] Generated state struct contains a, b, count fields
- [ ] Variables accessed via self.a, self.b, self.count
- [ ] No "cannot find value in scope" errors

✅ **Quality**:
- [ ] Test coverage ≥80% for generator transpilation
- [ ] Cyclomatic complexity ≤10
- [ ] TDG grade ≥ A-

✅ **Regression Prevention**:
- [ ] All existing tests pass
- [ ] All 7 working examples still compile
- [ ] Property tests for generator variable analysis (5+ tests)

## Related Issues

- **DEPYLER-0492**: Type inference (completed) - may need integration
- **DEPYLER-0493**: Constructor patterns (completed) - unrelated

## References

- Python PEP 255: Simple Generators
- Rust Iterator trait documentation
- State machine patterns in Rust

## Estimated Effort

**4-6 hours**:
- Phase 1 (RED): 1 hour - Create comprehensive test suite
- Phase 2 (GREEN): 3-4 hours - Implement generator variable analysis + state struct generation
- Phase 3 (REFACTOR): 1 hour - Quality gates + regression testing
