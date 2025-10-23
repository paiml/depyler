# BUG REPORT: struct module - TRANSPILER PANIC (CRITICAL)

**Discovered**: 2025-10-23
**Test Suite**: tdd-book/tests/test_struct.py
**Severity**: P0 - CRITICAL (Transpiler Crash)
**Category**: transpiler_crash

## Problem

ALL struct module operations cause **transpiler panic**:
```
thread 'main' panicked at /home/noah/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/depyler-core-3.19.14/src/rust_gen/expr_gen.rs:34:16:
expected an expression
```

## Test Evidence

**Test File**: tests/test_struct.py
**Results**: 0/6 passing (0% - COMPLETE FAILURE)

**All Failed** ❌:
- test_struct_pack_integer - Panic
- test_struct_unpack_integer - Panic
- test_struct_pack_multiple - Panic
- test_struct_unpack_multiple - Panic
- test_struct_calcsize - Panic
- test_struct_roundtrip - Panic

## Failing Code

**Simple pack**:
```python
import struct

def test_pack_int() -> int:
    packed = struct.pack('i', 42)
    return len(packed)
```

**Simple unpack**:
```python
import struct

def test_unpack_int() -> int:
    packed = struct.pack('i', 42)
    unpacked = struct.unpack('i', packed)
    return unpacked[0]
```

**calcsize**:
```python
import struct

def test_calcsize() -> int:
    size = struct.calcsize('i')
    return size
```

## Error Analysis

**Panic Location**: `expr_gen.rs:34:16`
**Error**: `expected an expression`
**Return Code**: 101 (panic/crash)

**Root Cause**: The transpiler has NO support for `struct` module:
- `struct.pack()` not recognized
- `struct.unpack()` not recognized
- `struct.calcsize()` not recognized
- Expression generator expects an expression but gets nothing

**Module Status**: **COMPLETELY UNIMPLEMENTED**

## Impact

- **CRITICAL**: Transpiler crashes (panic) on ANY struct usage
- **BLOCKING**: Binary data operations impossible
- **SEVERITY**: P0 - This is not a partial implementation, it's a complete absence
- struct is a core Python stdlib module
- Used for: Binary protocols, file formats, network packets, C interop

## Comparison with other bugs

1. **copy.copy() for lists**: Partial failure (1/6 tests)
2. **struct module**: **TOTAL failure** (0/6 tests, transpiler crashes)

This is **much more severe** than the copy bug.

## Recommended Fix Priority

**P0 - CRITICAL - IMMEDIATE**

This should be fixed before copy.copy() because:
1. Transpiler crash vs simple failure
2. Complete module absence vs partial implementation
3. Affects ALL struct operations vs one specific operation

## Next Ticket

Should create: **DEPYLER-0XXX: Implement struct module support (pack, unpack, calcsize)**

This will require:
1. Add struct module to import resolution
2. Implement struct.pack() → Rust equivalent (byteorder crate?)
3. Implement struct.unpack() → Rust equivalent
4. Implement struct.calcsize() → compile-time calculation
5. Handle format strings ('i', 'ii', etc.)
6. Handle endianness markers ('>', '<', '@')

**Estimated Effort**: 8-16 hours (new module implementation)

---

**Discovery Method**: TDD Book validation (OPTION 1 strategy)
**Bug Severity Escalation**: From P1 (copy) to P0 (struct crash)
**Expected Outcome**: Will discover more critical bugs in re and memoryview modules
