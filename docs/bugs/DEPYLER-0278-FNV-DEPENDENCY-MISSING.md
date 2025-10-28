# DEPYLER-0278: Missing fnv Crate Dependency for FnvHashMap

**Status**: DISCOVERED
**Priority**: P2 (Medium - breaks compilation but has workaround)
**Discovered**: 2025-10-28
**Root Cause**: Transpiler generates code using `fnv::FnvHashMap` based on annotations, but doesn't ensure dependency is available

## Issue

When Python code includes `# @depyler: hash_strategy = "fnv"` annotation, the transpiler generates Rust code using `FnvHashMap`, but the generated code fails to compile because the `fnv` crate is not in scope.

### Example

**Python**:
```python
# @depyler: hash_strategy = "fnv"
# @depyler: ownership = "owned"
def count_words(text: str) -> Dict[str, int]:
    """Count word frequencies with FNV hash strategy."""
    word_count = {}
    words = text.split()
    for word in words:
        if word in word_count:
            word_count[word] += 1
        else:
            word_count[word] = 1
    return word_count
```

**Generated Rust (BROKEN)**:
```rust
use fnv::FnvHashMap;  // ERROR: unresolved import `fnv`

pub fn count_words(text: &str) -> Result<FnvHashMap<String, i32>, ...> {
    // ...
}
```

**Compilation Error**:
```
error[E0432]: unresolved import `fnv`
 --> examples/showcase/annotated_example.rs:1:5
  |
1 | use fnv::FnvHashMap;
  |     ^^^ use of unresolved module or unlinked crate `fnv`
  |
help: you might be missing a crate named `fnv`, add it to your project and import it in your code
```

## Root Cause

1. Annotation parsing recognizes `hash_strategy = "fnv"` and generates `FnvHashMap` usage
2. Import generation adds `use fnv::FnvHashMap;`
3. But the transpiler doesn't check if `fnv` crate is available
4. No mechanism to add external crate dependencies to generated code

## Solution Options

### Option 1: Add Dependency Declaration (Recommended)
Generate a comment at top of file instructing users to add dependency:
```rust
// Required dependencies (add to Cargo.toml):
// fnv = "1.0"

use fnv::FnvHashMap;
```

### Option 2: Feature Flag
Only use FnvHashMap if feature is enabled, otherwise fall back to std::HashMap:
```rust
#[cfg(feature = "fnv")]
use fnv::FnvHashMap as HashMap;
#[cfg(not(feature = "fnv"))]
use std::collections::HashMap;
```

### Option 3: Don't Use FnvHashMap in Standalone Files
Only use `FnvHashMap` when transpiling within a Cargo project context where dependencies can be verified.

### Option 4: Always Use std::HashMap
Ignore `hash_strategy` annotations and always use std::HashMap for standalone file transpilation.

## Implementation

**Short-term fix (Option 4)**: Disable FnvHashMap generation for showcase examples
- Modify annotation processing to ignore `hash_strategy` for standalone files
- Only use `std::collections::HashMap`

**Long-term solution (Option 1 or 2)**:
- Add dependency comment generation
- Or add feature flag support

### Code Changes

In `import_gen.rs` or annotation processing:
```rust
fn should_use_fnv_hashmap(&self, annotations: &Annotations) -> bool {
    // Check if in project context with Cargo.toml
    if !self.is_cargo_project_context() {
        return false;  // Use std::HashMap for standalone files
    }

    // Check annotation
    annotations.hash_strategy == Some("fnv")
}
```

## Test Case

```python
# @depyler: hash_strategy = "fnv"
def make_map() -> Dict[str, int]:
    return {"a": 1}
```

Should generate (standalone file):
```rust
use std::collections::HashMap;  // Not fnv::FnvHashMap

pub fn make_map() -> Result<HashMap<String, i32>, ...> {
    // ...
}
```

Or (with dependency comment):
```rust
// Required dependencies (add to Cargo.toml):
// fnv = "1.0"

use fnv::FnvHashMap;

pub fn make_map() -> Result<FnvHashMap<String, i32>, ...> {
    // ...
}
```

## Impact

- **Severity**: P2 - Breaks compilation but only for files with fnv annotations
- **Scope**: Any code using `# @depyler: hash_strategy = "fnv"` annotation
- **Examples Affected**: `annotated_example.py`
- **Workaround**: Remove annotation or manually add `fnv` crate dependency

## Related

- Annotation system design
- Import generation in `import_gen.rs`
- Dependency management strategy
- Similar issues might exist for other external crate features

## Extreme TDD Approach

- **RED**: annotated_example.rs fails to compile with unresolved import
- **GREEN**: Implement Option 4 (disable FnvHashMap for showcase examples)
- **REFACTOR**: Consider long-term solution with dependency comments or feature flags

## Additional Issue in annotated_example.rs

There's also a type mismatch error (similar to DEPYLER-0277):
```
error[E0308]: mismatched types
   --> examples/showcase/annotated_example.rs:79:19
    |
 79 |         return Ok(());
```

This suggests the same `None` → `()` bug exists in multiple functions.
