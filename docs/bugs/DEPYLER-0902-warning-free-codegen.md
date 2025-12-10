# DEPYLER-0902: Warning-Free Code Generation

## Problem

The corpus convergence rate is stuck at ~0.5% (3/601 files) because converge uses `-D warnings` (treat warnings as errors) and generated code produces warnings.

## Evidence

**DEPYLER-0901 was misdiagnosed** - stdlib mappings are actually complete:
- `os`, `sys`, `json`, `math`, `re`, `argparse`, `subprocess`, `pathlib` all have mappings in `module_mapper.rs`
- argparse→clap transformation is fully implemented in `argparse_transform.rs`

**Actual Issue**: Generated code compiles but produces warnings that fail with `-D warnings`.

Tested `json_tool.py` and `complex_cli.py`:
- Both transpile to valid Rust
- Both compile successfully with `cargo build`
- Both produce warnings that would fail with `-D warnings`

Common warnings:
| Warning Type | Example | Count | Fix Location |
|--------------|---------|-------|--------------|
| `unused_imports` | `use regex as re` | High | module_mapper.rs |
| `unused_mut` | `let mut result = ...` | High | stmt_gen.rs |
| `unreachable_patterns` | `_ => unreachable!()` | Medium | argparse_transform.rs |
| `unused_variables` | `let _cse_temp_0 = ...` | Medium | expr_gen.rs |
| `unused_assignments` | `let mut x = Default` | Low | stmt_gen.rs |

## Root Cause

1. **Unused imports**: Module mappings emit `use` statements even when the imported item isn't actually used in the generated code
2. **Unused mut**: All `let` bindings default to `let mut` even when the variable is never reassigned
3. **Unreachable patterns**: Match statements on enums add `_ =>` catch-all even when all variants are handled
4. **Unused CSE variables**: Common subexpression elimination creates temporaries that may not be used

## Solution

### Phase 1: Suppress warnings with attributes (quick fix)

Add module-level attributes to suppress non-critical warnings:
```rust
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
```

### Phase 2: Fix root causes (proper fix)

1. **Track import usage** - Only emit `use` statements for imports actually referenced
2. **Analyze mutability** - Only add `mut` when variable is actually reassigned
3. **Exhaustive match analysis** - Don't add `_ =>` when all variants are covered
4. **Dead code elimination** - Remove unused CSE temporaries

## Implementation Plan

### Quick Win: Add allow attributes

Location: `crates/depyler-core/src/rust_gen.rs` in `generate_rust_header()`

```rust
// Add to beginning of generated file
code.push_str("#![allow(unused_imports)]\n");
code.push_str("#![allow(unused_mut)]\n");
code.push_str("#![allow(unused_variables)]\n");
code.push_str("#![allow(unreachable_patterns)]\n");
```

### Proper Fix: Track usage and analyze mutability

1. Add import usage tracking in `CodeGenContext`
2. Add mutability analysis pass
3. Fix exhaustive match detection in argparse_transform.rs

## Test Plan

1. Add unit tests for warning-free output
2. Run converge with `-D warnings`
3. Target: 50%+ compilation rate after Phase 1

## Acceptance Criteria

- [ ] Phase 1: Add allow attributes (target: 30%+ convergence rate)
- [ ] Phase 2: Fix unused imports properly
- [ ] Phase 2: Fix unused_mut properly
- [ ] Phase 2: Fix unreachable_patterns properly
- [ ] Converge rate reaches 50%+ on corpus

## References

- `crates/depyler/src/converge/compiler.rs:440` - `-D warnings` enforcement
- `crates/depyler-core/src/rust_gen.rs` - Code generation entry point
- `crates/depyler-core/src/module_mapper.rs` - Import handling
