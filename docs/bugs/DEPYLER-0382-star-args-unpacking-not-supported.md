# DEPYLER-0382: *args Unpacking in Function Calls Not Supported

**Ticket**: DEPYLER-0382
**Severity**: P0 (CRITICAL - Blocks transpilation)
**Status**: 🛑 STOP THE LINE
**Created**: 2025-11-17
**Component**: Expression Generator / HIR Converter
**Affects**: Variadic argument unpacking in function calls

## Problem Statement

The transpiler fails with "Expression type not yet supported" when encountering `*args` unpacking syntax in function calls. This blocks transpilation of common Python patterns where a sequence is unpacked into function arguments.

**Examples**:
- `os.path.join(*parts)` - Unpack list into path.join()
- `print(*items)` - Unpack list into print()
- `func(*args)` - Forward variadic arguments
- `max(*values)` - Unpack sequence into max()

## Reproduction

**Input** (`test_star_args.py`):
```python
import os

def join_paths(*parts):
    result = os.path.join(*parts)
    return result
```

**Current Error**:
```
Error: Expression type not yet supported
```

**Expected Behavior**: Should transpile to Rust equivalent that handles variadic arguments.

## Root Cause Analysis

### Location
Expression generator in HIR conversion phase does not recognize or handle the `Starred` AST node type when used in function call arguments.

### Python AST Structure
```python
# Python code: os.path.join(*parts)
# AST structure:
Call(
    func=Attribute(value=Attribute(value=Name(id='os'), attr='path'), attr='join'),
    args=[Starred(value=Name(id='parts'))],  # ← This is not handled
    keywords=[]
)
```

### Why It's Wrong
1. The `Starred` expression type is valid Python for unpacking sequences
2. Common pattern in idiomatic Python code (especially with `*args`)
3. Required for forwarding variadic arguments
4. Used extensively in stdlib functions (os.path.join, zip, etc.)

### Semantic Differences: Python vs Rust

**Python**:
- `*args` unpacks a sequence/iterable into individual arguments
- Works with any iterable: `func(*[1, 2, 3])` → `func(1, 2, 3)`
- Can mix with positional args: `func(a, *rest, b)`
- Runtime operation - sequence length can vary

**Rust**:
- No direct equivalent to `*args` unpacking
- Variadic functions use macros (`println!`) or slices (`&[T]`)
- Argument count must be known at compile time (except macros)
- Can use `.iter().copied()` or destructuring for fixed-size arrays

**Key Difference**: Python's dynamic unpacking vs Rust's static function signatures.

## Impact Assessment

**Affected Code Patterns**:
1. `os.path.join(*parts)` - Path construction from list
2. `print(*items)` - Printing variable number of items
3. `max(*values)`, `min(*values)` - Aggregation over unpacked sequence
4. `func(*args)` - Forwarding variadic arguments
5. `zip(*matrix)` - Transposing 2D sequences
6. Any stdlib function that accepts variadic arguments

**Severity**: **P0 CRITICAL**
- Blocks transpilation of example_environment
- Common Python idiom used throughout standard library
- Required for path manipulation (os.path.join is fundamental)
- Affects ~15% of Python codebases (estimate)

**Blocked Examples**:
- example_environment: Uses `os.path.join(*parts)` on line 97
- Likely affects other examples with path operations

## Solution Design

### Approach Options

#### Option 1: Convert to Slice-Based Functions
When the function accepts variadic args, pass as slice:

```python
# Python: os.path.join(*parts)
# Rust: std::path::PathBuf::from(parts.join(std::path::MAIN_SEPARATOR_STR))
```

**Pros**: Straightforward, type-safe
**Cons**: Requires knowing which functions accept variadic args

#### Option 2: Macro Generation
Generate Rust macros for functions with unpacking:

```rust
// Python: print(*items)
// Rust: macro that expands to individual print! calls
```

**Pros**: Flexible, handles unknown arg counts
**Cons**: Complex code generation, harder to debug

#### Option 3: Function-Specific Mappings
Special-case common functions with unpacking:

```python
# Python: os.path.join(*parts)
# Rust:
if parts.is_empty() {
    String::new()
} else {
    parts.join(std::path::MAIN_SEPARATOR_STR)
}
```

**Pros**: Idiomatic Rust for each case
**Cons**: Requires maintaining mappings for each function

### Recommended Approach: Option 3 (Function-Specific Mappings)

**Rationale**:
- Most common: `os.path.join(*parts)` - high-value, single fix
- Can add mappings incrementally
- Generates idiomatic Rust
- Avoids complex macro generation

### Algorithm

**Step 1: Detect `Starred` in Call Arguments**
```rust
// In convert_call() or convert_expr()
fn handle_starred_args(call: &Call) -> Result<syn::Expr> {
    for arg in &call.args {
        if matches!(arg, Expr::Starred(_)) {
            return convert_starred_call(call);
        }
    }
    // ... normal call handling
}
```

**Step 2: Map to Rust Equivalent**
```rust
fn convert_starred_call(call: &Call) -> Result<syn::Expr> {
    // Get the function being called
    let func_name = extract_function_name(&call.func)?;

    match func_name.as_str() {
        "os.path.join" => {
            let starred_arg = extract_starred_arg(&call.args)?;
            Ok(parse_quote! {
                {
                    let parts = #starred_arg;
                    if parts.is_empty() {
                        String::new()
                    } else {
                        parts.join(std::path::MAIN_SEPARATOR_STR)
                    }
                }
            })
        }
        "print" => {
            let starred_arg = extract_starred_arg(&call.args)?;
            Ok(parse_quote! {
                {
                    for item in #starred_arg {
                        print!("{} ", item);
                    }
                    println!();
                }
            })
        }
        _ => bail!("*args unpacking for {} not yet supported", func_name)
    }
}
```

### Implementation Strategy

**Phase 1: Core Infrastructure** (1-2 hours)
- Add `Starred` expression type recognition
- Implement starred argument extraction
- Add error message: "Function {name} does not support *args unpacking"

**Phase 2: os.path.join Support** (1 hour)
- Implement os.path.join(*parts) transpilation
- Use `parts.join(std::path::MAIN_SEPARATOR_STR)`
- Handle empty sequence case

**Phase 3: Common Functions** (2-3 hours)
- `print(*items)` - iterate and print
- `max(*values)`, `min(*values)` - use iterator methods
- `zip(*matrix)` - more complex, may defer

**Phase 4: General Forwarding** (future work)
- `func(*args)` where func is user-defined
- May require function signature analysis

### Complexity Considerations
- Function mapping: O(1) lookup, ≤ 10 complexity
- Code generation: Keep each mapping ≤ 15 lines
- Target: TDG ≤ 2.0 (A- or better)

## Test Plan

### Test Case 1: os.path.join(*parts) - Single Starred Arg
**Input**:
```python
import os

def join_paths(*parts: str) -> str:
    return os.path.join(*parts)
```

**Expected Rust**:
```rust
pub fn join_paths(parts: &[String]) -> String {
    if parts.is_empty() {
        String::new()
    } else {
        parts.join(std::path::MAIN_SEPARATOR_STR)
    }
}
```

### Test Case 2: os.path.join(*parts) - With List Literal
**Input**:
```python
import os

def test() -> str:
    parts = ["home", "user", "docs"]
    return os.path.join(*parts)
```

**Expected Rust**:
```rust
pub fn test() -> String {
    let parts = vec!["home".to_string(), "user".to_string(), "docs".to_string()];
    if parts.is_empty() {
        String::new()
    } else {
        parts.join(std::path::MAIN_SEPARATOR_STR)
    }
}
```

### Test Case 3: print(*items)
**Input**:
```python
def print_all(*items):
    print(*items)
```

**Expected Rust**:
```rust
pub fn print_all(items: &[String]) {
    for item in items {
        print!("{} ", item);
    }
    println!();
}
```

### Test Case 4: Mixed Args and Starred
**Input**:
```python
def test(prefix: str, *parts: str) -> str:
    import os
    return os.path.join(prefix, *parts)
```

**Expected Rust**:
```rust
pub fn test(prefix: String, parts: &[String]) -> String {
    let mut all_parts = vec![prefix];
    all_parts.extend_from_slice(parts);
    if all_parts.is_empty() {
        String::new()
    } else {
        all_parts.join(std::path::MAIN_SEPARATOR_STR)
    }
}
```

### Test Case 5: Error Case - Unsupported Function
**Input**:
```python
def test(*args):
    some_custom_func(*args)
```

**Expected Error**:
```
Error: *args unpacking for some_custom_func not yet supported
```

## Verification Steps

1. **Unit Tests**: Add tests for `Starred` expression detection
2. **Integration Tests**: Test os.path.join with various inputs
3. **Property Tests**: QuickCheck with random path lists (1000 iterations)
4. **Compilation Test**: Verify generated Rust compiles with `rustc --deny warnings`
5. **Runtime Test**: Execute transpiled binary with sample paths
6. **Regression Test**: Ensure example_environment transpiles after fix

## Related Issues

- **example_environment**: Blocked by this issue (line 97: `os.path.join(*parts)`)
- **DEPYLER-0381**: Similar sys module attribute access (completed)
- **Future**: `**kwargs` unpacking (separate ticket)

## Implementation Checklist

- [ ] Add `Starred` expression type to HIR
- [ ] Implement starred argument detection in convert_call()
- [ ] Create extract_starred_arg() helper function
- [ ] Add function mapping for os.path.join(*parts)
- [ ] Add function mapping for print(*items)
- [ ] Add function mapping for max(*values), min(*values)
- [ ] Handle mixed positional + starred arguments
- [ ] Add comprehensive unit tests (≥10 tests)
- [ ] Add property-based tests (QuickCheck, 1000 iterations)
- [ ] Test with example_environment
- [ ] Verify transpiled code compiles
- [ ] Run full test suite for regressions
- [ ] Update documentation

## Estimated Complexity

**TDG Target**: ≤ 2.0 (A- or better)
**Cyclomatic Complexity**: ≤ 10 per function
**Test Coverage**: ≥ 85%
**Implementation Time**: 4-6 hours (EXTREME TDD + function mappings)

## Notes

**Key Insight**: Python's `*args` unpacking is fundamentally dynamic, but most real-world usage can be mapped to static Rust patterns (especially os.path.join).

**Incremental Approach**: Start with high-value functions (os.path.join) and add mappings as needed, rather than attempting full generality.

**Future Work**: Full variadic function support would require:
1. Function signature analysis to determine arity
2. Macro generation for unknown arg counts
3. Type inference for starred expressions
4. This is a much larger effort - defer to DEPYLER-0400+

**Performance**: Rust's `join()` is typically faster than Python's os.path.join due to:
- No dynamic dispatch
- Better memory locality
- Compile-time string optimizations

**Cross-Platform**: Use `std::path::MAIN_SEPARATOR_STR` for platform-specific separators (Windows vs Unix).
