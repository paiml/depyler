# DEPYLER-0278: Missing fnv Crate Dependency for FnvHashMap

**Status**: FIXED ✅
**Priority**: P2 (Medium - breaks compilation but has workaround)
**Discovered**: 2025-10-28
**Fixed**: 2025-10-28
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

## Solution ✅

**Implemented Fix (Option 4)**: Disabled hash_strategy annotation for standalone transpilation

**Rationale**:
- Standalone files don't have Cargo.toml to declare dependencies
- Compilation success > optimization for standalone use case
- Projects can still optimize manually if they add fnv dependency

**Implementation** (annotation_aware_type_mapper.rs:102-114):
```rust
// DEPYLER-0278: Always use std::HashMap for standalone file transpilation
// FnvHashMap and AHashMap require external crate dependencies that may not be available
// For standalone files, we prioritize compilation success over optimization
// TODO: In the future, detect Cargo project context and use hash_strategy only within projects
let hash_map_type = "HashMap";

// Note: hash_strategy annotation is currently ignored for standalone transpilation
// Original logic (disabled):
// match annotations.hash_strategy {
//     depyler_annotations::HashStrategy::Standard => "HashMap",
//     depyler_annotations::HashStrategy::Fnv => "FnvHashMap",
//     depyler_annotations::HashStrategy::AHash => "AHashMap",
// }
```

**Changes**:
- Modified `map_dict_type()` to always return `HashMap`
- Updated tests to expect `HashMap` for all hash strategies
- Preserved hash_strategy enum for future use (Cargo project detection)

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

## Verification ✅

**Test Results**:
- `annotated_example.py` → transpiles successfully ✅
- Generated code uses `use std::collections::HashMap;` (not `fnv::FnvHashMap`) ✅
- No fnv references in generated code ✅

**Remaining Issues in annotated_example.rs** (separate bugs):
1. Unused `mut` warning (minor codegen issue)
2. Borrow after move error (separate ownership tracking bug)

**Note**: DEPYLER-0278 fix resolves the fnv import issue. The remaining errors are unrelated bugs.

## Related

- Annotation system design
- Import generation in `import_gen.rs`
- Dependency management strategy
- TODO: Future enhancement - detect Cargo project context and enable hash_strategy

## Extreme TDD Cycle ✅

- **RED**: annotated_example.rs failed with unresolved import `fnv`
- **GREEN**: Disabled hash_strategy annotation, always use `HashMap`
- **REFACTOR**: Updated tests, verified no fnv references in generated code

## Future Enhancement

When Cargo project detection is implemented:
1. Check for Cargo.toml in parent directories
2. If found, check for fnv/ahash dependencies in Cargo.toml
3. Only use FnvHashMap/AHashMap if dependencies are declared
4. For standalone files, always use HashMap
