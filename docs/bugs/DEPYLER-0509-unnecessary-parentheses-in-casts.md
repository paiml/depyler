# DEPYLER-0509: Unnecessary Parentheses in Integer Casts

## Summary

Generated Rust code has unnecessary double parentheses around integer literals when casting to i64, causing clippy warnings with `-D warnings`.

## Severity

**P1 (BLOCK RELEASE)** - Triggers `unnecessary_parentheses` warning which fails with `-D warnings`

## Symptoms

```bash
$ cargo run --release --bin depyler -- transpile examples/lambda_advanced_test.py
$ rustc --crate-type lib --deny warnings examples/lambda_advanced_test.rs

error: unnecessary parentheses around function argument
 --> examples/lambda_advanced_test.rs:6:24
  |
6 |     let result1 = calc(((2) as i64), ((3) as i64), ((4) as i64));
  |                        ^          ^
```

## Root Cause Analysis (Five Whys)

### Why 1: Why does rustc complain about unnecessary parentheses?
The generated code has `((2) as i64)` instead of `(2 as i64)` or `2_i64`.

### Why 2: Why are there double parentheses?
The codegen in `expr_gen.rs:2678-2683` wraps the expression in parentheses twice:
```rust
let paren_expr: syn::Expr = parse_quote! { (#final_expr) };
final_expr = parse_quote! { (#paren_expr as i64) };
```

### Why 3: Why does it wrap twice?
Step 1: `paren_expr = (2)` - wraps literal in parens
Step 2: `((2) as i64)` - wraps paren_expr in parens again for cast precedence

### Why 4: Why is the outer paren needed?
Comment says: "ensure correct precedence: (expr) as i64, not expr as i64"
This is only needed for complex expressions, not simple literals.

### Why 5: Why doesn't it detect simple literals?
No check for whether `final_expr` is already a simple literal that doesn't need parentheses.

## Correct Solution

Check if `final_expr` is a simple literal or already parenthesized:
- Literals: Use typed suffix `2_i64` or bare cast `2 as i64`
- Complex expressions: Use parenthesized cast `(complex_expr) as i64`

## Test Cases

### Failing Test (before fix)
```python
def test_call(x: int, y: int) -> int:
    return x + y

def main():
    result = test_call(2, 3)
```
Generated: `test_call(((2) as i64), ((3) as i64))`
Expected: `test_call(2_i64, 3_i64)` or `test_call(2 as i64, 3 as i64)`

## Implementation Plan

1. **Phase 1 (RED)**: Create failing test for unnecessary parentheses
2. **Phase 2 (GREEN)**: Detect simple literals and avoid double-wrapping
3. **Phase 3 (REFACTOR)**: Clean up, ensure all quality gates pass

## Files to Modify

- `crates/depyler-core/src/rust_gen/expr_gen.rs` lines 2670-2685

## Acceptance Criteria

- [ ] Integer literals cast with `2_i64` or `(2 as i64)` syntax
- [ ] Complex expressions still get proper parentheses
- [ ] `lambda_advanced_test.py` compiles with `rustc --deny warnings`
- [ ] No regression in existing tests
