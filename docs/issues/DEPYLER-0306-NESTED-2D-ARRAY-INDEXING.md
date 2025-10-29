# DEPYLER-0306: Nested 2D Array Indexing - Malformed Code Generation

**Discovered**: 2025-10-29 during Example 12 (Control Flow) validation
**Status**: 🐛 **BUG** - Code generation creates syntax errors
**Priority**: P1 (affects common pattern - nested loops with 2D arrays)
**Estimate**: 4-6 hours (medium complexity, code generation fix)

## Overview

When transpiling Python code with nested list indexing `matrix[i][j]`, the transpiler generates **malformed Rust code** with syntax errors. The range expression in nested `for` loops is incorrectly split across lines, creating stray `. len() as i32 {` fragments.

## Discovery Context

**Example**: python-to-rust-conversion-examples/examples/12_control_flow/
**Functions**: 26 control flow functions tested
**Result**: 2 compilation errors in nested 2D array access
**Success Rate**: 24/26 functions (92%) compile correctly

**Error Message**:
```
error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `{`
  --> src/lib.rs:47:16
   |
47 | . len() as i32 {
   |                ^ expected one of 8 possible tokens
```

---

## Root Cause: Range Expression Split Across Lines

**Python Pattern**:
```python
def find_first_match(matrix: list[list[int]], target: int) -> tuple[int, int]:
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):  # ← Nested indexing
            if matrix[i][j] == target:
                return (i, j)
    return (-1, -1)
```

**Current Generated Code (BROKEN)**:
```rust
pub fn find_first_match(matrix: &Vec<Vec<i32>>, target: i32) -> Result<(i32, i32), IndexError> {
    for i in 0..matrix.len() as i32 {
    for j in 0..{
        let base = matrix;
        let idx: i32 = i;
        let actual_idx = if idx<0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    }
    . len() as i32 {  // ❌ SYNTAX ERROR: Stray `. len() as i32`
        if {
            // ... matrix[i][j] indexing ...
        } == target {
            return Ok((i, j));
        }
    }
    }
    Ok((-1, -1))
}
```

**Analysis**: The transpiler is generating the indexing logic for `matrix[i]` inside the range expression `0..`, then continuing on the next line with `. len() as i32`. This creates an incomplete expression `0..<complex block>` followed by a dangling `. len()`.

---

## Correct Translation Needed

**Expected Rust Code**:
```rust
pub fn find_first_match(matrix: &Vec<Vec<i32>>, target: i32) -> Result<(i32, i32), IndexError> {
    for i in 0..matrix.len() as i32 {
        // Extract matrix[i] into variable BEFORE for loop
        let row = matrix.get(i as usize)
            .ok_or_else(|| IndexError::new("index out of range"))?;

        for j in 0..row.len() as i32 {
            let value = row.get(j as usize)
                .ok_or_else(|| IndexError::new("index out of range"))?;

            if *value == target {
                return Ok((i, j));
            }
        }
    }
    Ok((-1, -1))
}
```

**Alternative** (more concise):
```rust
pub fn find_first_match(matrix: &Vec<Vec<i32>>, target: i32) -> Result<(i32, i32), IndexError> {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {  // Safe: i is guaranteed in bounds
            if matrix[i][j] == target {
                return Ok((i as i32, j as i32));
            }
        }
    }
    Ok((-1, -1))
}
```

---

## Affected Functions (2 of 26)

### 1. `find_first_match()` (lines 35-73)
**Python**:
```python
def find_first_match(matrix: list[list[int]], target: int) -> tuple[int, int]:
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                return (i, j)
    return (-1, -1)
```

**Error**: Line 47 - stray `. len() as i32 {`

### 2. `count_matches_in_matrix()` (lines 74-79)
**Python**:
```python
def count_matches_in_matrix(matrix: list[list[int]], target: int) -> int:
    count = 0
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                count = count + 1
    return count
```

**Error**: Line 87 - stray `. len() as i32 {`

---

## Functions That Work Correctly (24 of 26)

**Examples of functions that transpile correctly**:

```python
# Single indexing - works fine
def first_negative_index(numbers: list[int]) -> int:
    for i in range(len(numbers)):
        if numbers[i] < 0:
            return i
    return -1

# 2D diagonal access (matrix[i][i]) - works fine
def sum_diagonal(matrix: list[list[int]]) -> int:
    total = 0
    for i in range(len(matrix)):
        total = total + matrix[i][i]
    return total

# All control flow patterns work:
# - break, continue
# - nested conditionals
# - multiple return paths
# - complex boolean conditions
```

**Key Insight**: The issue is **specific to nested loops with 2D indexing in range expression**, not general 2D array access.

---

## Implementation Plan

### Phase 1: Root Cause Analysis ✅ COMPLETE (2025-10-29)

**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` lines 2290-2410 (`convert_index` function)

**Root Cause Identified**:
Indexing expressions (`matrix[i]`) generate **blocks with braces** for negative index handling:
```rust
{
    let base = &matrix;
    let idx: i32 = i;
    let actual_idx = if idx < 0 {
        base.len().saturating_sub(idx.abs() as usize)
    } else {
        idx as usize
    };
    base.get(actual_idx).cloned().unwrap_or_default()
}
```

When this appears in `range(len(matrix[i]))` context, it generates:
```rust
for j in 0..{
    // Complex indexing block
}
.len() as i32 {
    // ❌ SYNTAX ERROR: `.len()` after closing brace
}
```

**Why This Happens**:
- Indexing needs runtime negative index handling (`if idx < 0 ...`)
- This requires a block with `let` bindings
- Rust doesn't allow method calls (`.len()`) on block expressions in range contexts
- The parser sees `0..{ block }` as incomplete, then `. len()` as unexpected

**Architectural Issue**:
The indexing generation doesn't know if it's in a "range context" where blocks aren't allowed.
Would need to either:
1. Thread context through expr generation (complex, touches many functions)
2. Always generate simpler inline expressions (might break negative index handling)
3. Detect and extract complex expressions before for loops (attempted, but extraction still has same block issue)

### Phase 2: Fix Approaches Evaluated

**Attempted Fix #1: Extract to Variable Before For Loop** ❌ FAILED
- **Approach**: Detect complex indexing in iterator, extract to `let _loop_iter = ...` before loop
- **Implementation**: Modified `codegen_for_stmt` in `stmt_gen.rs`
- **Result**: Still generates block expressions in the extracted variable assignment
- **Problem**: `let _loop_iter = 0..{ block }.len()` has same syntax error
- **Conclusion**: Extraction doesn't help because the block issue persists

**Recommended Fix: Context-Aware Indexing Generation** (4-6 hours estimated)
**Option A: Thread "Range Context" Through Expression Generation**
```rust
// When generating `for j in 0..matrix[i].len()`:
// 1. Detect complex indexing in range expression
// 2. Extract to temporary variable BEFORE for loop
// 3. Use variable in range

// Generated:
let _temp_row = matrix.get(i as usize).ok_or(...)?;
for j in 0.._temp_row.len() as i32 {
    // ...
}
```

**Option B: Inline Safely**
```rust
// Ensure indexing expression stays on single line
for j in 0..matrix.get(i as usize).unwrap_or_default().len() as i32 {
    // ...
}
```

**Option C: Use Direct Indexing**
```rust
// When i is guaranteed in bounds (from parent for loop):
for j in 0..matrix[i].len() as i32 {
    // ...
}
```

### Phase 3: Add Test Cases (1 hour)

**Test Coverage**:
```rust
#[test]
fn test_nested_2d_indexing() {
    let python = r#"
def find_in_matrix(matrix: list[list[int]], target: int) -> tuple[int, int]:
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                return (i, j)
    return (-1, -1)
"#;

    let rust = transpile(python).unwrap();
    assert!(rust.contains("for i in 0..matrix.len()"));
    assert!(rust.contains("for j in 0..")); // Should be valid expression

    // Verify it compiles
    assert!(compile_rust(&rust).is_ok());
}

#[test]
fn test_3d_indexing() {
    let python = r#"
def sum_3d(cube: list[list[list[int]]]) -> int:
    total = 0
    for i in range(len(cube)):
        for j in range(len(cube[i])):
            for k in range(len(cube[i][j])):
                total += cube[i][j][k]
    return total
"#;

    let rust = transpile(python).unwrap();
    assert!(compile_rust(&rust).is_ok());
}
```

---

## Error Categorization

### Category: **Code Generation - Range Expressions**
- **Difficulty**: Medium (requires understanding expression generation flow)
- **Impact**: High (affects common nested loop pattern)
- **Frequency**: Medium (2D arrays used in ~20% of code)

### Technical Details:
- **Component**: `rust_gen/expr_gen.rs` (range expression generation)
- **Pattern**: Nested indexing in `range(len(matrix[i]))`
- **Symptom**: Expression split across multiple lines incorrectly
- **Fix Type**: Extract complex indexing to variable OR keep inline on single line

---

## Workaround (Until Fixed)

**Python Code**: Use temporary variable
```python
# Instead of:
for j in range(len(matrix[i])):
    # ...

# Use:
row = matrix[i]
for j in range(len(row)):
    # ...
```

This generates simpler, more readable Rust code anyway.

---

## Priority Justification

**Impact**: P1 (High Priority)
- Affects **nested loops** - extremely common pattern
- Affects **2D arrays** - used in ~20% of typical Python code
- **Blocks matrix operations, grid algorithms, game boards, image processing**

**Comparison**:
| Issue | Impact | Estimate | Priority |
|-------|--------|----------|----------|
| DEPYLER-0304 (Context managers) | 100% file I/O blocked | 11-13 hrs | P0 |
| DEPYLER-0305 (Classes) | 60-70% code blocked | 40-60 hrs | P0 |
| **DEPYLER-0306 (Nested indexing)** | **~20% code affected** | **4-6 hrs** | **P1** |
| DEPYLER-0302 (String methods) | High frequency | 6-8 hrs | P1 |
| DEPYLER-0303 (Dict methods) | High frequency | 4-6 hrs | P1 |

**ROI**: **High** - 4-6 hours to fix, unblocks common pattern

---

## Example 12 Overall Results

**Total Functions**: 26
**Functions Working**: 24 (92%)
**Functions Broken**: 2 (8%)

**Breakdown by Pattern**:
- ✅ Single loops with indexing: 100% working
- ✅ Nested loops without 2D indexing: 100% working
- ✅ 2D diagonal access (`matrix[i][i]`): 100% working
- ✅ Break/continue: 100% working
- ✅ Multiple return paths: 100% working
- ✅ Complex boolean conditions: 100% working
- ❌ Nested loops with 2D range indexing: 0% working (2/2 broken)

**Key Insight**: The transpiler handles **most control flow correctly**. This is a **specific bug in nested indexing**, not a systemic control flow issue.

---

## Recommendation

**Status**: **HIGH-PRIORITY BUG** - Common pattern completely broken

**Action**: Fix after P0 blockers (DEPYLER-0304, DEPYLER-0305)

**Rationale**:
1. **High ROI**: 4-6 hours investment, unblocks ~20% of code
2. **Common pattern**: Nested loops with 2D arrays extremely common
3. **Clean fix**: Extracting to variable improves code quality anyway
4. **Low risk**: Isolated to range expression generation

**Suggested Fix Order**:
1. DEPYLER-0304 (Context managers) - P0 blocker
2. DEPYLER-0305 (Classes) - P0 blocker
3. **DEPYLER-0306 (Nested indexing)** - **P1 high-ROI quick win**
4. DEPYLER-0302/0303 (String/Dict methods) - P1 batch fixes

---

## Conclusion

Example 12 reveals a **specific code generation bug in nested 2D array indexing**. The transpiler incorrectly splits range expressions when they contain nested indexing like `range(len(matrix[i]))`, creating syntax errors.

**Good News**: 92% of control flow patterns work correctly, indicating the transpiler's core control flow logic is solid.

**Fix**: Extract complex indexing expressions to temporary variables before `for` loops (4-6 hours).

**Impact**: Unblocks matrix operations, grid algorithms, and other 2D array patterns.

**Next Steps**:
1. ✅ Document finding (this ticket)
2. 🎯 Continue Matrix discovery (create more examples)
3. 📋 After Matrix complete: Fix in batch with other P1 issues
4. 🚀 Verify fix with Example 12 re-transpilation

**Status**: Documented, **P1 HIGH-PRIORITY BUG**
