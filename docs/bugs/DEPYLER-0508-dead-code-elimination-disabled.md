# DEPYLER-0508: Dead Code Elimination Disabled - Unused Variables Fail Compilation

## Summary

Transpiled Rust code fails to compile with `-D warnings` due to unused variables. The root cause is that dead code elimination (DCE) is disabled in the optimizer, causing Python variables that are assigned but never read to be transpiled into Rust code that triggers `unused_variables` warnings.

## Severity

**P1 (BLOCK RELEASE)** - Generated code fails clippy/rustc with `-D warnings`

## Symptoms

```bash
$ cargo run --release --bin depyler -- transpile examples/simple_array_test.py
# ... transpiles successfully ...

$ rustc --crate-type lib --deny warnings examples/simple_array_test.rs
error: unused variable: `arr2`
error: unused variable: `zeros`
error: unused variable: `ones`
error: aborting due to 3 previous errors
```

## Root Cause Analysis (Five Whys)

### Why 1: Why does Rust compilation fail?
The generated code has unused variables: `arr2`, `zeros`, `ones`

### Why 2: Why are these variables unused?
In the source Python, these variables are assigned but never read:
```python
def test_arrays():
    arr2 = [0, 0, 0, 0]      # assigned, never read
    zeros = [0] * 10          # assigned, never read
    ones = [1] * 5            # assigned, never read
    sum_val = arr1[0] + arr1[1]
    return sum_val
```

### Why 3: Why doesn't dead code elimination (DCE) remove them?
DCE is **DISABLED** in the optimizer (optimizer.rs:38):
```rust
// DEPYLER-0363: Temporarily disable dead code elimination to debug argparse issue
eliminate_dead_code: false,
```

### Why 4: Why does CSE make the problem worse?
Common Subexpression Elimination (CSE) runs even with DCE disabled:
- Creates `_cse_temp_0` for `[0] * 10` → `[0; 10]`
- Assigns to unused `zeros = _cse_temp_0`
- Now we have TWO unused variables instead of one

### Why 5: Why was DCE disabled in the first place?
DEPYLER-0363 (argparse transformation) had an issue where DCE was removing `parser.add_argument()` calls because their return values aren't used. This was a legitimate concern but disabling DCE globally is the wrong fix.

## Correct Solution

Re-enable DCE globally but ensure argparse-related statements are preserved. The optimizer should:

1. **Mark side-effectful statements** - `parser.add_argument()` has side effects (modifies parser state)
2. **Preserve method calls on tracked objects** - Any call like `obj.method()` where `obj` is used later should be preserved
3. **Use proper liveness analysis** - A variable is "used" if:
   - Its value is read
   - A method is called on it that has side effects

## Test Cases

### Failing Test (before fix)
```python
def test_unused_variables():
    """Unused variables should be eliminated or prefixed with _"""
    x = 42        # unused
    y = [1, 2, 3] # unused
    z = x + 1     # x is used here, but z is unused
    return 0
```
Expected: Compiles without warnings

### Argparse Preservation Test
```python
def test_argparse_preserved():
    """add_argument calls must be preserved even though return value unused"""
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--name')  # return value unused but MUST be preserved
    args = parser.parse_args()
    return args.name
```
Expected: Both `add_argument` and `parse_args` calls preserved

## Implementation Plan

1. **Phase 1 (RED)**: Create failing test for unused variable elimination
2. **Phase 2 (GREEN)**: Re-enable DCE with side-effect preservation
3. **Phase 3 (REFACTOR)**: Clean up, ensure all quality gates pass

## Files to Modify

- `crates/depyler-core/src/optimizer.rs` - Re-enable DCE, add side-effect detection
- `crates/depyler-core/tests/` - Add regression tests

## Acceptance Criteria

- [ ] Unused variables are eliminated from generated code
- [ ] Argparse `add_argument()` calls are preserved
- [ ] All examples compile with `rustc --deny warnings`
- [ ] All existing tests pass
- [ ] No regression in argparse transformation

## Related Tickets

- DEPYLER-0363: ArgumentParser → Clap transformation (original cause of DCE disable)
