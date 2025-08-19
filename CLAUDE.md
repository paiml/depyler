# CLAUDE.md - Depyler Compiler Implementation Protocol

## Prime Directive

**Generate correct Rust code that compiles on first attempt. Quality is built-in, not bolted-on.**

## Project Context

Depyler is a Python-to-Rust transpiler focusing on energy-efficient, safe code
generation with progressive verification. The system must produce idiomatic Rust
code with formal correctness guarantees for a practical Python subset.

## Development Principles

### 自働化 (Jidoka) - Build Quality In

- **Never ship incomplete transpilation**: All HIR transformations must include
  complete error handling paths
- **Verification-first development**: Every new AST-to-Rust mapping requires
  corresponding property verification
- **Example**: When implementing control flow transpilation:
  ```rust
  // CORRECT: Complete error handling
  match stmt {
      While { test, body, .. } => {
          verify_termination_bounds(&test)?;
          emit_rust_while(test, body)
      }
      _ => Err(TranspileError::UnsupportedStatement(stmt.clone()))
  }
  // NEVER: Partial implementations with TODO
  ```

### 現地現物 (Genchi Genbutsu) - Direct Observation

- **Test against real Rust**: Don't rely on syn parsing alone; test generated
  code with `cargo check`
- **Profile actual compilation**: Measure transpilation time/memory on realistic
  Python codebases
- **Debug at the Rust level**: When transpilation fails, examine the actual
  generated Rust code, not just the HIR

### 反省 (Hansei) - Fix Before Adding

- **Current broken functionality to prioritize**:
  1. Type inference generates incorrect ownership patterns
  2. String handling creates unnecessary allocations
  3. Property verification doesn't catch all lifetime violations
- **Do not add**: Advanced async support, class inheritance, or SMT verification
  until core function transpilation is bulletproof

### 改善 (Kaizen) - Continuous Improvement

- **Incremental verification**: Start with `--verify basic`, achieve 100%
  coverage on V1 subset, then advance to `strict`
- **Performance baselines**: Generated Rust must compile in <500ms for typical
  functions
- **Code quality targets**: Output should pass `clippy::pedantic` without
  warnings

## Critical Invariants

1. **Type safety**: Every generated Rust program must pass `cargo check` without
   errors
2. **Determinism**: Same Python input must produce identical Rust output across
   runs
3. **Memory safety**: No generated code can cause undefined behavior or memory
   leaks

## Build Commands

```bash
# Run full test suite with property verification
cargo test --workspace

# Transpile with verification
cargo run -- transpile examples/showcase/binary_search.py --verify

# Run benchmarks
cargo bench

# Check generated code quality
cargo clippy --workspace -- -D warnings
```

## Testing Strategy

```bash
# Verify core transpilation pipeline
cargo test -p depyler-core

# Run property-based tests on generated code
cargo test -p depyler-verify

# Test MCP fallback integration  
cargo test -p depyler-mcp
```

## MANDATORY Quality Gates (BLOCKING - Not Advisory)

**CRITICAL**: Quality gates are now BLOCKING and ENFORCED. No commit shall pass without meeting all gates.

### Pre-commit Hooks (MANDATORY)
```bash
#!/bin/bash
# .git/hooks/pre-commit - BLOCKS commits that violate quality
set -e

echo "🔒 MANDATORY Quality Gates for Depyler..."

# GATE 1: Core transpilation must work
echo 'print("Hello")' | cargo run --quiet -- transpile --stdin || {
    echo "❌ FATAL: Basic transpilation broken"
    echo "Fix core transpiler before ANY commits"
    exit 1
}

# GATE 2: Complexity enforcement
cargo clippy --all-targets --all-features -- -D clippy::cognitive_complexity || {
    echo "❌ BLOCKED: Complexity exceeds limits"
    echo "Refactor before committing"
    exit 1
}

# GATE 3: Zero SATD policy
! grep -r "TODO\|FIXME\|HACK\|XXX" crates/ --include="*.rs" || {
    echo "❌ BLOCKED: Technical debt comments found"
    echo "Fix issues or file GitHub issues, don't commit debt"
    exit 1
}

# GATE 4: Lint zero tolerance
cargo clippy --all-targets --all-features -- -D warnings || {
    echo "❌ BLOCKED: Lint warnings found" 
    exit 1
}

# GATE 5: Test coverage threshold
cargo tarpaulin --min 70 --fail-under || {
    echo "❌ BLOCKED: Coverage below 70%"
    exit 1
}

echo "✅ All quality gates passed"
```

### CI/CD Pipeline Enforcement
```yaml
# .github/workflows/quality-gates.yml
name: MANDATORY Quality Gates
on: [push, pull_request]

jobs:
  quality-gates:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
          
      - name: GATE 1 - Basic Transpilation Function
        run: |
          echo 'print("CI Test")' | timeout 10s cargo run --quiet -- transpile --stdin
          
      - name: GATE 2 - Complexity Check  
        run: |
          cargo clippy --all-targets --all-features -- -D clippy::cognitive_complexity
          
      - name: GATE 3 - Zero SATD
        run: |
          ! grep -r "TODO\|FIXME\|HACK" crates/ --include="*.rs"
          
      - name: GATE 4 - Lint Zero Tolerance
        run: |
          cargo clippy --all-targets --all-features -- -D warnings
          
      - name: GATE 5 - Coverage Gate
        run: |
          cargo tarpaulin --min 70 --fail-under
```

## The Make Lint Contract (Zero Warnings Allowed)
```bash
# make lint command from Makefile:
cargo clippy --all-targets --all-features -- -D warnings
```

**Critical**: The `-D warnings` flag treats EVERY clippy warning as a hard error. This ensures zero technical debt accumulation.

### What This Means for Your Code

```rust
// Standard clippy: These would be warnings
x.to_string();           // Warning: could use .into()
&vec![1, 2, 3];         // Warning: could use slice
if x == true { }        // Warning: could omit == true

// With make lint: These FAIL the build
x.to_string();          // ERROR - build fails
&vec![1, 2, 3];        // ERROR - build fails  
if x == true { }       // ERROR - build fails
```

### Surviving -D warnings

```rust
// Write defensive code from the start:
x.into();               // Prefer into() over to_string()
&[1, 2, 3];            // Use slice literals
if x { }               // Omit redundant comparisons

// For unavoidable warnings, be explicit:
#[allow(clippy::specific_lint)]  // Document why
fn special_case() { }
```

## Performance Invariants

### Transpilation Throughput

```rust
#[bench]
fn bench_transpile_throughput(b: &mut Bencher) {
    let input = include_str!("../corpus/large.py"); // 10K LOC Python
    b.iter(|| {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(input)
    });
    
    // Invariant: >10MB/s transpilation
    assert!(b.bytes_per_second() > 10_000_000);
}
```

### Memory Safety Verification

```rust
#[bench]
fn bench_verification_latency(b: &mut Bencher) {
    let hir = test_hir_module();
    b.iter(|| {
        let verifier = PropertyVerifier::new();
        verifier.verify(&hir)
    });
    
    // Invariant: <50ms for typical module
    assert!(b.ns_per_iter() < 50_000_000);
}
```

## Architectural Patterns

### HIR Pattern - Immutable Transformations

```rust
impl HirModule {
    fn transform<F>(&self, f: F) -> Self 
    where 
        F: Fn(&HirExpr) -> HirExpr
    {
        // Immutable transformation preserving structure
        self.map_expressions(f)
    }
}
```

### Ownership Inference - Conservative Defaults

```rust
impl OwnershipInferencer {
    fn infer_ownership(&mut self, expr: &HirExpr) -> Ownership {
        match expr {
            HirExpr::Variable(name) => {
                // Conservative: default to borrowed
                self.env.lookup(name)
                    .unwrap_or(Ownership::Borrowed)
            }
            HirExpr::FunctionCall(func, args) => {
                // Move semantics for non-Copy types
                if self.is_move_required(func) {
                    Ownership::Owned
                } else {
                    Ownership::Borrowed
                }
            }
            _ => Ownership::Borrowed
        }
    }
}
```

## Error Diagnostics Quality

### Elm-Level Error Messages

```rust
impl ErrorReporter {
    fn render(&self, error: &TranspileError) -> String {
        let mut output = String::new();
        
        // Source context with highlighting
        writeln!(output, "{}", self.source_snippet(error.span));
        writeln!(output, "{}", "^".repeat(error.span.len()).red());
        
        // Primary message
        writeln!(output, "\n{}: {}", "Error".red().bold(), error.message);
        
        // Type mismatch details
        if let Some(expected) = &error.expected_type {
            writeln!(output, "  {} {}", "Expected:".yellow(), expected);
            writeln!(output, "  {} {}", "Found:".yellow(), error.found_type);
        }
        
        // Actionable suggestion
        if let Some(suggestion) = &error.suggestion {
            writeln!(output, "\n{}: {}", "Hint".green(), suggestion);
            writeln!(output, "{}", self.render_suggestion_diff(suggestion));
        }
        
        output
    }
}
```

## Sprint Hygiene Protocol

### Pre-Sprint Cleanup (MANDATORY)
```bash
# Remove all debug artifacts before starting sprint
rm -f test_* debug_* 
find . -type f -executable -not -path "./target/*" -not -path "./.git/*" -delete

# Verify no large files
find . -type f -size +10M -not -path "./target/*" -not -path "./.git/*"

# Clean build artifacts
cargo clean
```

### Post-Sprint Checklist
```bash
# 1. Remove debug artifacts
rm -f test_* debug_* *.o *.a

# 2. Run all quality gates
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace

# 3. Verify no cruft
git status --ignored

# 4. Push with clean history
git push origin main
```

## The Development Flow

```
1. IDENTIFY Python feature to support
2. WRITE property test first
3. IMPLEMENT with <10 cognitive complexity
4. VERIFY generated Rust compiles
5. VALIDATE performance invariants
6. COMMIT with quality gates passed
```

## Architecture Patterns

### HIR Builder Pattern

```rust
impl HirBuilder {
    fn build_function(&mut self, func: &ast::FunctionDef) -> Result<HirFunction> {
        // Type annotations are mandatory for transpilation
        let params = func.args.args.iter()
            .map(|arg| self.build_param(arg))
            .collect::<Result<Vec<_>>>()?;
            
        let ret_type = func.returns
            .as_ref()
            .map(|ann| self.resolve_type(ann))
            .unwrap_or(Ok(Type::Unit))?;
            
        let body = self.build_block(&func.body)?;
        
        Ok(HirFunction {
            name: func.name.clone(),
            params,
            ret_type,
            body,
            properties: self.infer_properties(&body)?,
        })
    }
}
```

### Ownership Inference

```rust
impl OwnershipAnalyzer {
    fn infer_ownership(&mut self, expr: &HirExpr) -> Result<Ownership> {
        match expr {
            HirExpr::Variable(name) => {
                let usage = self.track_usage(name)?;
                match usage {
                    Usage::MovedFrom => Ok(Ownership::Moved),
                    Usage::BorrowedMut => Ok(Ownership::MutBorrow),
                    Usage::BorrowedShared => Ok(Ownership::SharedBorrow),
                    Usage::Owned => Ok(Ownership::Owned),
                }
            }
            HirExpr::MethodCall { receiver, method, .. } => {
                // Check if method moves or borrows
                let method_sig = self.lookup_method(receiver, method)?;
                self.apply_method_semantics(receiver, method_sig)
            }
            _ => self.default_ownership(expr)
        }
    }
}
```

## Memory Management

### String Interning for Optimization

```rust
pub struct StringInterner {
    strings: FxHashMap<String, InternedString>,
    usage_counts: FxHashMap<InternedString, usize>,
}

impl StringInterner {
    pub fn should_intern(&self, s: &str) -> bool {
        // Intern strings used more than 3 times
        self.usage_counts.get(s).map_or(false, |&count| count > 3)
    }
    
    pub fn intern(&mut self, s: String) -> InternedString {
        *self.strings.entry(s.clone())
            .or_insert_with(|| {
                let id = InternedString(self.strings.len());
                *self.usage_counts.entry(id).or_insert(0) += 1;
                id
            })
    }
}
```

## Release Checklist

Before ANY release:

- [ ] All examples transpile and run correctly
- [ ] Property tests achieve 100% coverage on supported features
- [ ] Generated code passes `cargo clippy -- -D warnings`
- [ ] Benchmarks show no performance regression
- [ ] Documentation examples are tested
- [ ] CHANGELOG updated with breaking changes
- [ ] Version bump follows semver

---

**Remember**: Perfect transpilation is better than feature-complete transpilation. Every line of generated Rust must be idiomatic. Every error must guide the user to a solution. Ship nothing that doesn't meet these standards.