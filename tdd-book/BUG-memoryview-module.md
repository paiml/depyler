# BUG REPORT: memoryview/bytes literals - UNSUPPORTED CONSTANT TYPE (CRITICAL)

**Discovered**: 2025-10-23
**Test Suite**: tdd-book/tests/test_memoryview.py
**Severity**: P0 - CRITICAL (Complete Failure)
**Category**: type_system

## Problem

ALL memoryview operations fail with **transpilation error**:
```
Error: Unsupported constant type
```

## Test Evidence

**Test File**: tests/test_memoryview.py
**Results**: 0/6 passing (0% - COMPLETE FAILURE)

**All Failed** ❌:
- test_memoryview_from_bytes - Error
- test_memoryview_from_bytearray - Error
- test_memoryview_indexing - Error
- test_memoryview_slicing - Error
- test_memoryview_tobytes - Error
- test_memoryview_iteration - Error

## Failing Code

**memoryview from bytes**:
```python
def test_memview_bytes() -> int:
    # Create memoryview from bytes
    data = b"hello world"  # ❌ Error: Unsupported constant type
    view = memoryview(data)
    return len(view)
```

**memoryview from bytearray**:
```python
def test_memview_bytearray() -> int:
    # Create memoryview from bytearray
    data = bytearray(b"test")  # ❌ Error: Unsupported constant type
    view = memoryview(data)
    return len(view)
```

**memoryview indexing**:
```python
def test_memview_index() -> int:
    # Create and index memoryview
    data = b"hello"  # ❌ Error: Unsupported constant type
    view = memoryview(data)
    return view[0]
```

## Error Analysis

**Error**: `Error: Unsupported constant type`
**Return Code**: 1 (transpilation error, not panic)

**Root Cause**: The transpiler cannot handle **bytes literals**:
- `b"hello"` literal syntax not supported
- This is more fundamental than memoryview itself
- memoryview depends on bytes, so both are broken

**Pattern**: The error occurs at bytes literal parsing, before memoryview is even evaluated.

**Hypothesis**:
1. Bytes literals (`b"..."`) not recognized as valid constant type
2. Constant type system missing bytes/bytearray support
3. This blocks ALL buffer protocol operations (memoryview, bytearray, etc.)

## Impact

- **CRITICAL**: Complete module failure (0/6 tests)
- **FUNDAMENTAL**: bytes is a core Python type
- **BLOCKING**: All buffer protocol operations impossible
- **SEVERITY**: P0 - This is a fundamental type system gap
- bytes/memoryview used for: Binary data, I/O, network, buffer protocol
- Also affects: bytearray, buffer protocol, binary file operations

## Comparison with other bugs

1. **struct module**: TOTAL failure (0/6 tests, P0 CRITICAL) - module unimplemented
2. **memoryview/bytes**: TOTAL failure (0/6 tests, P0 CRITICAL) - type unsupported
3. **re module**: MAJOR failure (2/6 tests, P1 MAJOR) - Match object missing
4. **copy.copy() for lists**: Minor failure (5/6 tests, P1 MAJOR) - specific case

The memoryview/bytes bug is P0 CRITICAL because it's a fundamental type gap.

## Recommended Fix Priority

**P0 - CRITICAL - IMMEDIATE**

This should be fixed alongside struct module because:
1. Complete module failure (0% pass rate)
2. Fundamental type system gap (bytes literals)
3. Affects multiple modules (memoryview, bytearray, binary I/O)
4. More fundamental than re bug (which has 33% pass rate)

## Next Ticket

Should create: **DEPYLER-0XXX: Implement bytes literals and memoryview support**

This will require:
1. Add bytes literal (`b"..."`) to constant type system
2. Implement bytes → Rust mapping (`&[u8]` or `Vec<u8>`)
3. Implement bytearray() → Rust equivalent
4. Implement memoryview() → Rust equivalent (slice wrapper?)
5. Implement buffer protocol basics
6. Handle indexing, slicing, iteration on bytes/memoryview
7. Implement .tobytes() method

**Estimated Effort**: 12-20 hours (new fundamental type + buffer protocol)

---

**Discovery Method**: TDD Book validation (OPTION 1 strategy)
**Bug Severity Progression**: P1 (copy) → P0 (struct) → P1 (re) → P0 (bytes/memoryview)
**Pattern**: Fundamental type gaps are causing P0 bugs
