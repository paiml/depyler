# DEPYLER-0552: Dict Access Type Inference

## Summary

Added type inference support for distinguishing dictionary access (string-keyed) from list access (integer-keyed) in function parameters.

## Problem Statement

The stdlib_integration example had 39 errors, with 15 related to `Vec` vs `HashMap` type inference. When Python code uses `info["key"]` patterns, the transpiler was incorrectly inferring `Vec<serde_json::Value>` instead of `HashMap<String, serde_json::Value>`.

Example from `format_output_text`:
```python
def format_output_text(info, include_hash):
    lines = []
    lines.append(f"Path: {info['path']}")  # String key access
    lines.append(f"Filename: {info['filename']}")  # String key access
```

Was generating:
```rust
pub fn format_output_text(
    info: &'a mut Vec<serde_json::Value>,  // WRONG - should be HashMap
    ...
)
```

## Root Cause Analysis

### Five Whys

1. **Why was `info` typed as `Vec`?**
   - The `UsagePattern::Container` pattern was triggered by indexing operations

2. **Why did `Container` imply `Vec`?**
   - The `add_container_evidence` function always adds `Type::List(...)` regardless of index type

3. **Why doesn't it check index type?**
   - The `analyze_indexing` function didn't distinguish between string keys (dict) and integer keys (list)

4. **Why wasn't this distinction made?**
   - Original implementation was optimized for list-heavy code patterns

5. **Why did this cause 15 errors?**
   - Every `info["key"]` access failed because Vec doesn't support string indexing

## Solution

### 1. New Usage Pattern

Added `UsagePattern::DictAccess` in `type_hints.rs`:
```rust
enum UsagePattern {
    Iterator,
    Numeric,
    StringLike,
    Container,      // List-like, integer indexing
    DictAccess,     // DEPYLER-0552: String-keyed access
    Callable,
}
```

### 2. Smart Indexing Analysis

Updated `analyze_indexing` to detect string-keyed access:
```rust
fn analyze_indexing(&mut self, base: &HirExpr, index: &HirExpr) -> Result<()> {
    if let HirExpr::Var(var) = base {
        // DEPYLER-0552: Check if index is a string literal (dict access)
        let is_string_key = matches!(
            index,
            HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::FString { .. }
        );
        // Also check for common string key variable names
        let is_likely_string_key = if let HirExpr::Var(idx_name) = index {
            idx_name == "key" || idx_name == "k" || idx_name.ends_with("_key")
        } else {
            false
        };

        if is_string_key || is_likely_string_key {
            self.record_usage_pattern(var, UsagePattern::DictAccess);
        } else {
            self.record_usage_pattern(var, UsagePattern::Container);
        }
    }
    ...
}
```

### 3. Dict Evidence Function

Added `add_dict_access_evidence`:
```rust
fn add_dict_access_evidence(&self, type_votes: &mut HashMap<Type, (u32, Vec<String>)>) {
    let (count, reasons) = type_votes
        .entry(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Custom("serde_json::Value".to_string())),
        ))
        .or_default();
    *count += 5; // Higher confidence than list
    reasons.push("string-keyed access (dict)".to_string());
}
```

### 4. Updated Pattern Handling

Updated all pattern handlers to include `DictAccess`:
- `add_pattern_evidence` - routes to `add_dict_access_evidence`
- `update_type_score` - adds dict type with +5 confidence
- `infer_non_optional_type` - prioritizes dict access over container

## Files Modified

- `crates/depyler-core/src/type_hints.rs` - Core changes:
  - Added `UsagePattern::DictAccess` variant
  - Updated `analyze_indexing` to detect string keys
  - Added `add_dict_access_evidence` function
  - Updated `add_pattern_evidence`, `update_type_score`, and `infer_non_optional_type`

- `crates/depyler-core/src/rust_gen/func_gen.rs` - Supporting changes:
  - Updated `infer_type_from_expr_usage` to detect string-keyed indexing

## Testing

### Before Fix
```
example_stdlib: 39 errors
- 15 errors: `[serde_json::Value]` cannot be indexed by `&str`
- Vec types incorrectly inferred for dict parameters
```

### After Fix
```
example_stdlib: 31 errors (8 fixed)
- Dict types correctly inferred: HashMap<String, serde_json::Value>
- String-keyed access now generates proper HashMap types
```

Type hints now correctly show:
```
Hint: dict[str, serde_json::Value] for parameter 'info' [Certain]
      (string-keyed access (dict), ...)
Applied type hint: info -> Dict(String, Custom("serde_json::Value"))
```

## Remaining Issues

The stdlib example still has 31 errors due to:
1. `DateTime.fromtimestamp()` - needs chrono mapping
2. File I/O patterns (`f.read(8192)` with walrus operator)
3. hashlib `hexdigest()` mapping
4. Various stdlib method mappings

These are tracked for future work and are candidates for the depyler-oracle ML approach (GH-105).

## Related

- DEPYLER-0550: CSV filter predicate dict type inference
- DEPYLER-0492: Container/iterator pattern detection
- GH-105: depyler-oracle ML-powered auto-fixer
- reprorusted-python-cli: 11/13 examples now compile (85%)

## Verification

```bash
cargo clippy -- -D warnings  # PASS
cargo test --workspace --lib  # PASS (118 tests)
```
