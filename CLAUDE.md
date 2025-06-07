# CLAUDE.md - Depyler Development Guidelines

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
