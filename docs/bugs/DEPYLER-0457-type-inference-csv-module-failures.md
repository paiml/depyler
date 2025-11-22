# DEPYLER-0457: Type Inference Failures in CSV Module Transpilation (P0 - STOP THE LINE)

**Status**: 🛑 BLOCKING - 17 compilation errors in example_csv_filter
**Severity**: P0 (Core type system failure)
**Component**: Type inference, CSV module mapping
**Discovered**: 2025-11-22
**Affects**: csv.DictReader, optional parameters, generator expressions

## Executive Summary

The type inference system defaults function parameters to `serde_json::Value` instead of inferring correct types. This cascades into 17 compilation errors when transpiling CSV file processing code.

## Error Breakdown

```
3× E0308: Type mismatches (wrong parameter types)
2× E0599: Missing `iter()` method on Map iterators
2× E0308: If/else type incompatibility
2× E0277: Trait bound failures (AsRef<Path>)
1× E0606: Invalid cast (&serde_json::Value as usize)
1× E0599: get() on unit type
1× E0434: Nested function can't capture environment
2× E0282: Type annotations needed
2× E0277: Display/comparison trait issues
```

## Root Causes

### 1. Function Parameter Type Inference Failure

**Pattern**: All optional parameters default to `serde_json::Value` or `bool`

```python
def filter_csv(input_file, column, value, output_file=None):
    ...
```

**Current (Broken)**:
```rust
pub fn filter_csv<'a, 'b>(
    input_file: String,
    column: &'a serde_json::Value,    // Should be: String
    value: &'b serde_json::Value,     // Should be: String
    output_file: bool,                 // Should be: Option<String>
) -> Result<(), std::io::Error>
```

**Root Cause**: Type inference defaults to conservative `serde_json::Value` for parameters without type annotations. Optional parameters incorrectly inferred as `bool` instead of `Option<T>`.

### 2. Generator Expression Failures

**Pattern**: `(row for row in reader if condition)`

```python
filtered_rows = (row for row in reader if row[column] == value)
```

**Current (Broken)**:
```rust
// Missing closure conversion
// Iterator types don't chain correctly
```

**Root Cause**: Generator expressions not converted to closures/iterators properly.

### 3. CSV DictReader Mapping Issues

**Pattern**: `csv.DictReader(f)` usage

**Root Cause**: Python csv module not fully mapped to Rust csv crate patterns.

## Impact

- **Affected Examples**: example_csv_filter (17 errors)
- **User Impact**: HIGH - CSV processing is common data science pattern
- **Workaround**: Manual type annotations (defeats purpose of transpiler)

## Solution Strategy

1. **Improve type inference heuristics**:
   - String literals in calls → infer String parameters
   - `default=None` → infer `Option<T>`
   - Dict indexing → infer string keys

2. **CSV module mapping**:
   - Map csv.DictReader to rust csv::Reader
   - Handle field access patterns

3. **Generator expression transformation**:
   - Convert to closures with proper captures
   - Map to iterator chains

## Estimated Fix Time

- Parameter type inference: 4-6 hours
- CSV module mapping: 3-4 hours
- Generator expressions: 2-3 hours
- **Total**: 9-13 hours with testing

## Priority

**P0 - STOP THE LINE**: Core type system issue affecting multiple language features beyond just CSV processing.

---
**Blocked By**: None
**Blocks**: Reprorusted data processing examples, type inference feature completeness
