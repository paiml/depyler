# Extract & Test Private Modules - Idempotent Prompt

## Prerequisites
```bash
pmat work start DEPYLER-COVERAGE-95
cargo build --package depyler-core --lib
```

## Phase 1: Find Duplicates & Pure Functions

### 1.1 Detect Duplicates (CRITICAL FIRST STEP)
```bash
# Find duplicate code - these are FREE coverage wins
pmat analyze duplicates --path crates/depyler-core/src/rust_gen --threshold 0.7
```

### 1.2 Find Pure Static Functions to Extract
```bash
# Find standalone functions (no &self) - easiest to extract
grep -n "^fn [a-z_]\+(" crates/depyler-core/src/rust_gen/expr_gen.rs | head -20
grep -n "^fn [a-z_]\+(" crates/depyler-core/src/rust_gen/stmt_gen.rs | head -20

# Find impl methods that are static (can be extracted)
grep -n "^    fn [a-z_]\+([^&]" crates/depyler-core/src/rust_gen/expr_gen.rs | head -20
```

## Phase 2: Extreme TDD Extraction Loop

### 2.1 Pattern: Add to EXISTING Modules (Preferred)
```rust
// ADD pure functions to existing helper modules:
// - expr_analysis.rs   - HIR expression analysis
// - keywords.rs        - Rust keyword handling
// - name_heuristics.rs - Variable name inference
// - json_helpers.rs    - JSON/serde_json helpers
// - borrowing_helpers.rs - Borrow/reference helpers

// Example: Add to expr_analysis.rs
pub fn is_file_creating_expr(expr: &HirExpr) -> bool { ... }
pub fn is_stdio_expr(expr: &HirExpr) -> bool { ... }
pub fn extract_string_literal(expr: &HirExpr) -> String { ... }
```

### 2.2 Pattern: Remove Duplicates, Add Imports
```rust
// BEFORE: Duplicate in expr_gen.rs (60 lines, untested)
impl ExpressionConverter {
    fn is_rust_keyword(name: &str) -> bool { ... }
}

// AFTER: Import from centralized module
use crate::rust_gen::keywords;
// Replace: Self::is_rust_keyword -> keywords::is_rust_keyword
// Delete: The duplicate function definition
```

### 2.3 Write Tests INLINE with Module
```rust
// In the helper module (e.g., expr_analysis.rs), add tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_file_creating_open_call() {
        let expr = HirExpr::Call { func: "open".to_string(), ... };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_other_call() {
        let expr = HirExpr::Call { func: "print".to_string(), ... };
        assert!(!is_file_creating_expr(&expr));
    }
}
```

### 2.4 Fast Test Iteration
```bash
# Test specific functions (~2s)
cargo test --package depyler-core --lib -- is_file_creating is_stdio extract_string

# Test full module (~3s)
cargo test --package depyler-core --lib -- expr_analysis::tests
```

## Phase 3: Bulk Update Call Sites

```bash
# Use Edit with replace_all=true for bulk updates
# Example: Replace all Self::is_rust_keyword with keywords::is_rust_keyword
```

```rust
// Add import at top of file
use crate::rust_gen::keywords;

// Then use Edit tool with replace_all=true:
// old_string: "Self::is_rust_keyword"
// new_string: "keywords::is_rust_keyword"
```

## Phase 4: Validate

```bash
# Verify compilation
cargo check --package depyler-core --lib

# Run all tests (~50s)
cargo test --package depyler-core --lib

# Final coverage (only at end, ~90s)
cargo llvm-cov --package depyler-core --lib --summary-only
```

## Extraction Priority Order

1. **Duplicates** - Find with pmat, delete & import (FREE coverage)
2. **Pure functions** - `fn foo(...)` without `&self` (easy to test)
3. **Static methods** - `fn foo(...)` in impl without `&self`
4. **Methods with simple context** - Can mock context for testing

## Existing Helper Modules (ADD TO THESE)

| Module | Purpose | Add Functions Like |
|--------|---------|-------------------|
| `expr_analysis.rs` | HIR expression analysis | `is_*_expr`, `extract_*` |
| `keywords.rs` | Rust keyword handling | `is_*_keyword`, `safe_ident` |
| `name_heuristics.rs` | Name-based type inference | `is_*_var_name` |
| `json_helpers.rs` | JSON type detection | `is_json_*` |
| `borrowing_helpers.rs` | Reference handling | `borrow_*`, `wrap_*` |
| `precedence.rs` | Operator precedence | `get_*_precedence` |
| `walrus_helpers.rs` | Walrus operator analysis | `collect_walrus_*` |

## Anti-Patterns (AVOID)

❌ Creating new helper files when existing ones fit
❌ Running coverage after every small change
❌ Testing through integration when unit tests suffice
❌ Keeping duplicate implementations
❌ Writing tests for already-tested duplicates
❌ Complex mocking when pure function extraction is possible

## Success Metrics

| Metric | Target |
|--------|--------|
| Tests pass | 7000+ |
| Test time | <60s |
| Coverage | 95%+ |
| Duplicates | 0 |

## Quick Reference

| Action | Command | Time |
|--------|---------|------|
| Find duplicates | `pmat analyze duplicates --path crates/depyler-core/src/rust_gen` | ~5s |
| Fast test | `cargo test --lib -- [pattern]` | ~2s |
| Full test | `cargo test --package depyler-core --lib` | ~50s |
| Coverage | `cargo llvm-cov --lib --summary-only` | ~90s |
