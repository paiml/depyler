# DEPYLER-0273: Union Type Syntax Not Supported (PEP 604)

**Status**: 🛑 STOP THE LINE
**Severity**: P1 (Blocks modern Python, workaround exists)
**Discovery**: 2025-10-28 (Matrix Testing - Column A → B transpilation)
**Category**: Type System / Python 3.10+ Support

---

## Bug Description

### Issue
Depyler fails to transpile Python code using PEP 604 union type syntax (`int | None`), which was introduced in Python 3.10.

### Root Cause
**Location**: Type annotation parser in HIR builder

The transpiler encounters `int | None` syntax and throws:
```
Error: Unsupported type annotation
```

No detailed error message, no line number, no indication of which annotation failed.

### Impact
- **Cannot transpile modern Python 3.10+ code** using union syntax
- **Matrix testing blocked**: Column A uses `int | None` syntax
- **User experience**: Cryptic error message provides no actionable information

---

## Expected Output (After Fix)

### Python Input
```python
def is_none(value: int | None) -> bool:
    """Check if value is None."""
    return value is None

def optional_default(value: int | None, default: int = 42) -> int:
    """Return value or default if None."""
    if value is None:
        return default
    return value
```

### Expected Rust Output
```rust
pub fn is_none(value: Option<i32>) -> bool {
    value.is_none()
}

pub fn optional_default(value: Option<i32>, default: i32) -> i32 {
    value.unwrap_or(default)
}
```

---

## Actual Output (Current Behavior)

### Command
```bash
depyler transpile test_union.py
```

### Error
```
Error: Unsupported type annotation
```

**Problems**:
1. No line number
2. No indication of which type annotation failed
3. No suggestion to use `Optional[T]` instead
4. No documentation reference

---

## Workaround (Temporary)

Use `typing.Optional[T]` instead of `T | None`:

```python
from typing import Optional

def is_none(value: Optional[int]) -> bool:
    return value is None
```

This transpiles successfully to:
```rust
pub fn is_none(value: Option<i32>) -> bool {
    value.is_none()
}
```

---

## Test Cases

### Test 1: Simple Union Type
```python
def check_none(x: int | None) -> bool:
    return x is None
```

**Expected**: Transpiles to `Option<i32>`
**Actual**: `Error: Unsupported type annotation`

### Test 2: Multiple Union Types
```python
def process(value: int | str | None) -> str:
    if value is None:
        return "none"
    return str(value)
```

**Expected**: Transpiles to appropriate Rust enum or type
**Actual**: `Error: Unsupported type annotation`

### Test 3: Union in Return Type
```python
def maybe_int(flag: bool) -> int | None:
    return 42 if flag else None
```

**Expected**: Returns `Option<i32>`
**Actual**: `Error: Unsupported type annotation`

### Test 4: Optional (Works)
```python
from typing import Optional

def check_none(x: Optional[int]) -> bool:
    return x is None
```

**Expected**: Transpiles to `Option<i32>` ✅
**Actual**: Works correctly ✅

---

## Fix Strategy

### Analysis: What Needs to Change

1. **AST Parsing**: Recognize `BinOp` with `|` operator in type context
2. **Type Resolution**: Convert `int | None` → `Type::Optional(Box::new(Type::Int))`
3. **HIR Mapping**: Map union types to Rust's `Option<T>` when appropriate
4. **Error Messages**: Provide clear error with line number and suggestion

### Implementation Approach

#### Option 1: Support PEP 604 Union Syntax (Correct Solution)
Add support for parsing `T | None` in type annotations.

**Pros**: Supports modern Python 3.10+ code
**Cons**: Requires AST parsing changes
**Effort**: Medium (4-6 hours)

#### Option 2: Better Error Message (Quick Fix)
Detect union syntax and provide helpful error message.

**Pros**: Quick to implement, helps users
**Cons**: Doesn't fix the actual issue
**Effort**: Low (30 minutes)

**RECOMMENDATION**: Option 1 (proper support) for this fix, Option 2 as immediate improvement.

---

## Implementation Plan (TDD)

### Phase 1: RED (Create Failing Test)

```bash
# Create test file
touch tests/depyler_0273_union_types_test.rs

# Add test cases:
# 1. Simple union: int | None
# 2. Return type union: -> int | None
# 3. Multiple unions: int | str | None
# 4. Complex: dict[str, int | None]

cargo test depyler_0273  # Should FAIL (currently unsupported)
```

### Phase 2: GREEN (Implement Fix)

**Step 1: Improve Error Message (Quick Win)**

File: `crates/depyler-core/src/hir/builder.rs`

```rust
// When encountering unsupported annotation, provide details
fn resolve_type_annotation(&self, ann: &Expr) -> Result<Type> {
    match ann {
        Expr::BinOp { op: Operator::BitOr, left, right, .. } => {
            // Check if this is a union type (T | None pattern)
            bail!(
                "Union type syntax 'T | None' not yet supported (Python 3.10+ PEP 604).\n\
                 Workaround: Use 'Optional[T]' from typing module instead.\n\
                 Example: 'Optional[int]' instead of 'int | None'\n\
                 Location: line {}",
                self.get_line_number(ann)
            )
        }
        _ => // ... existing logic
    }
}
```

**Step 2: Add Union Type Support**

```rust
fn resolve_type_annotation(&self, ann: &Expr) -> Result<Type> {
    match ann {
        Expr::BinOp { op: Operator::BitOr, left, right, .. } => {
            // Check if right side is None (T | None pattern)
            if self.is_none_literal(right) {
                let inner_type = self.resolve_type_annotation(left)?;
                return Ok(Type::Optional(Box::new(inner_type)));
            }

            // Multiple unions: int | str | None
            // For now, only support T | None pattern
            bail!("Complex union types not yet supported. Use Optional[T] for now.")
        }
        _ => // ... existing logic
    }
}
```

### Phase 3: REFACTOR (Verify Quality)

```bash
# Run tests
cargo test --workspace

# Verify transpilation
depyler transpile /tmp/test_union.py
cat /tmp/test_union.rs  # Should have Option<i32>

# Check clippy
cargo clippy --all-targets --all-features -- -D warnings

# Manual validation
depyler transpile examples/01_basic_types/A_python/basic_types.py
```

---

## Quality Gate Impact

### Before Fix
❌ **Matrix Testing Blocked**: Cannot transpile Column A → Column B
❌ **User Experience**: Cryptic error messages
❌ **Python 3.10+ Support**: Modern syntax not supported

### After Fix
✅ **Matrix Testing Unblocked**: Column A transpiles successfully
✅ **Clear Error Messages**: Line numbers and suggestions provided
✅ **Python 3.10+ Support**: Union type syntax works

---

## Related Issues

- **PEP 604**: Union Types (Python 3.10+) - https://peps.python.org/pep-0604/
- **DEPYLER-0269**: isinstance() transpilation (FIXED) ✅
- **DEPYLER-0270**: Cow type inference (FIXED) ✅
- **DEPYLER-0271**: Unnecessary returns (FIXED) ✅
- **DEPYLER-0272**: Unnecessary casts (FIXED) ✅

---

## References

- PEP 604 Union Types: https://peps.python.org/pep-0604/
- Python 3.10 Release Notes: https://docs.python.org/3/whatsnew/3.10.html#pep-604-new-type-union-operator
- Rust Option<T>: https://doc.rust-lang.org/std/option/enum.Option.html

---

## Future Improvements

1. **Full Union Support**: Handle `int | str | None` → Rust enum
2. **Type Narrowing**: Use Rust's pattern matching for union type refinement
3. **Better Diagnostics**: Show both original and suggested syntax side-by-side

---

**Next Steps**: Proceed with TDD implementation (RED → GREEN → REFACTOR)
