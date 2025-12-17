# Golden Example Codegen Defects Report

**Date**: 2025-12-14
**Source Files**:
- `examples/golden_exception_handling.py` (436 lines, 12KB)
- `examples/golden_async_concurrency.py` (355 lines, 13KB)

**Summary**: Both golden examples successfully transpile but fail compilation.
- Exception example: 53 errors
- Async example: 50 errors

---

## Exception Handling Defects (golden_exception_handling.py)

### DEPYLER-0985: Comparison Operator Spacing Bug

**Severity**: Critical (syntax error)
**Count**: 8 errors
**Pattern**: `= =` instead of `==` when comparing to negative numbers

**Evidence**:
```rust
// Generated (BROKEN):
assert!(get_with_key_error(& d, "missing".to_string()) = = (- 1));
assert!(exception_with_computation(10, 0, 3) = = (- 1));

// Expected:
assert!(get_with_key_error(&d, "missing".to_string()) == (-1));
```

**Root Cause**: Likely in expr_gen.rs where comparison expressions are generated. The tokenization is splitting `==` and `(-1)` incorrectly.

---

### DEPYLER-0986: Custom Exception Types Missing Error Trait

**Severity**: High
**Count**: 2 errors
**Pattern**: User-defined exceptions don't implement `std::error::Error`

**Evidence**:
```rust
// Generated:
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
}
impl ValidationError {
    pub fn new(message: String) -> Self { ... }
}
// Missing: impl std::error::Error for ValidationError {}
// Missing: impl std::fmt::Display for ValidationError {}
```

**Expected**: All exception classes should generate Error + Display impls.

---

### DEPYLER-0987: Result Return Wrapping Inconsistency

**Severity**: Critical
**Count**: 25 errors
**Pattern**: Return values inside try blocks not wrapped in `Ok()`

**Evidence**:
```rust
// Generated (BROKEN):
pub fn divide_safe(a: i64, b: i64) -> Result<i64, Box<dyn std::error::Error>> {
    if needs_adjustment {
        q - 1  // Missing Ok()
    } else {
        q      // Missing Ok()
    }
}

// Expected:
    if needs_adjustment {
        Ok(q - 1)
    } else {
        Ok(q)
    }
```

---

### DEPYLER-0988: Multiple Exception Handlers Collapsed

**Severity**: High
**Count**: Implicit (logic bug)
**Pattern**: Multiple `except` clauses become single catch-all

**Evidence**:
```python
# Python:
try:
    num = int(s)
    return d[str(num)]
except ValueError:
    return -1
except KeyError:
    return -2
```

```rust
// Generated (BROKEN):
match (|| { ... })() {
    Ok(_result) => return _result,
    Err(_) => return Ok(-1),  // KeyError case (-2) is lost!
}
```

---

### DEPYLER-0989: raise Statement Uses panic! Instead of Err()

**Severity**: High
**Count**: Multiple
**Pattern**: `raise ValueError("msg")` becomes `panic!` not `Err()`

**Evidence**:
```rust
// Generated (BROKEN):
if inner > 100 {
    panic!("{}", ValueError::new("Too large".to_string()));
}

// Expected:
if inner > 100 {
    return Err(Box::new(ValueError::new("Too large".to_string())));
}
```

---

### DEPYLER-0990: Result vs Value Comparison

**Severity**: High
**Count**: 10 errors
**Pattern**: Functions returning `Result<T>` compared directly to `T`

**Evidence**:
```rust
// Generated (BROKEN):
assert!(nested_try_except(10) == 22);  // nested_try_except returns Result<i64>

// Expected:
assert!(nested_try_except(10).unwrap() == 22);
// or
assert!(nested_try_except(10) == Ok(22));
```

---

## Async/Concurrency Defects (golden_async_concurrency.py)

### DEPYLER-0991: Async Iterator Not Properly Handled

**Severity**: Critical
**Count**: 2 errors
**Pattern**: `async for` generates non-async iterator code

**Evidence**:
```rust
// Generated (BROKEN):
for value in AsyncCounter::new(limit) {  // Not async!
    total += value;
}

// Expected:
while let Some(value) = counter.next().await {
    total += value;
}
```

**Additional Error**: `StopAsyncIteration` is not defined in Rust scope.

---

### DEPYLER-0992: Nested Functions/Closures Lose Scope

**Severity**: Critical
**Count**: 2 errors
**Pattern**: Nested `async def` inside functions become top-level

**Evidence**:
```python
async def concurrent_with_results(values: List[int]) -> List[int]:
    async def process(x: int) -> int:  # Nested function
        await asyncio.sleep(0.001)
        return x * 2
    tasks = [process(v) for v in values]
```

```rust
// Generated (BROKEN) - process is not in scope:
async fn concurrent_with_results(values: &[i64]) -> Vec<i64> {
    let tasks = values.iter().map(|v| process(v));  // process not found!
}
```

---

### DEPYLER-0993: await Outside Async Block

**Severity**: Critical
**Count**: 3 errors
**Pattern**: `.await` expressions appear outside async context

**Evidence**:
```rust
// Generated (BROKEN) - at module level:
tokio::time::sleep(...).await;  // Not inside async fn!
```

This happens when async class methods or nested functions are incorrectly hoisted.

---

### DEPYLER-0994: Async Context Manager Not Implemented

**Severity**: High
**Pattern**: `async with` generates no equivalent Rust code

**Evidence**: The `__aenter__` and `__aexit__` methods are not being transformed into proper Rust async resource management patterns.

---

### DEPYLER-0995: Tokio/Serde Dependencies Not in Scope

**Severity**: Medium (infrastructure)
**Count**: 20 errors
**Pattern**: Generated code references unlinked crates

**Note**: This is expected when compiling with `rustc` directly. The Cargo.toml was generated but not used.

---

## Summary Table

| Ticket | Severity | Category | Count | Description |
|--------|----------|----------|-------|-------------|
| DEPYLER-0985 | Critical | Syntax | 8 | `= =` spacing in comparisons |
| DEPYLER-0986 | High | Types | 2 | Custom exceptions missing Error trait |
| DEPYLER-0987 | Critical | Returns | 25 | Result wrapping inconsistency |
| DEPYLER-0988 | High | Logic | 1 | Multiple handlers collapsed |
| DEPYLER-0989 | High | Semantics | N/A | raise → panic! instead of Err() |
| DEPYLER-0990 | High | Types | 10 | Result vs value comparison |
| DEPYLER-0991 | Critical | Async | 2 | async for not handled |
| DEPYLER-0992 | Critical | Scope | 2 | Nested functions lose scope |
| DEPYLER-0993 | Critical | Async | 3 | await outside async blocks |
| DEPYLER-0994 | High | Async | N/A | async with not implemented |
| DEPYLER-0995 | Medium | Deps | 20 | Missing crate references |

---

## Prioritized Fix Order

1. **DEPYLER-0985** - Syntax error blocks all compilation
2. **DEPYLER-0987** - Result wrapping is fundamental to exception handling
3. **DEPYLER-0989** - raise semantics are broken
4. **DEPYLER-0988** - Multiple handlers is common pattern
5. **DEPYLER-0986** - Custom exceptions need Error trait
6. **DEPYLER-0991** - async for is common async pattern
7. **DEPYLER-0992** - Nested functions affect closures
8. **DEPYLER-0993** - await scoping is critical
9. **DEPYLER-0994** - async with is important pattern
10. **DEPYLER-0990** - Test assertion generation (lower priority)

---

## Conclusion

The golden examples successfully identified **11 distinct codegen defects** in the transpiler. The exception handling subsystem has fundamental issues with Result wrapping and error propagation. The async subsystem has significant gaps in async iterator, nested function, and context manager support.

These defects confirm that type inference was not the sole convergence blocker. Even with 100% type annotations, the codegen produces uncompilable Rust code.
