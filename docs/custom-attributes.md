````markdown
# Custom Rust Attributes in Depyler

## Overview

Introduces support for custom Rust attributes, allowing developers to add Rust-specific compiler hints and directives (such as `#[inline]`, `#[must_use]`, `#[cold]`) directly from Python source code. This provides fine-grained control over code generation, compiler behavior, safety guarantees, and other Rust-specific features. 

## Usage

Add custom Rust attributes using the `@depyler: custom_attribute` annotation in Python comments:

```python
# @depyler: custom_attribute = "inline"
def fast_function(x: int) -> int:
    return x * 2
```

Transpiles to:

```rust
#[inline]
pub fn fast_function(x: i32) -> i32 {
    x * 2
}
```

## Supported Attributes

### 1. Compiler Directives

#### Inlining Control

```python
# Hint to inline (compiler decides)
# @depyler: custom_attribute = "inline"
def frequently_called(x: int) -> int:
    return x * 2

# Force aggressive inlining
# @depyler: custom_attribute = "inline(always)"
def critical_hot_path(x: int) -> int:
    return x + 1

# Prevent inlining
# @depyler: custom_attribute = "inline(never)"
def large_function(data: list[int]) -> int:
    # Complex processing...
    return sum(data)
```

#### Cold Path Marking

```python
# Mark rarely-executed error handlers
# @depyler: custom_attribute = "cold"
def handle_error(error_code: int, message: str) -> None:
    """Error handler - executed infrequently."""
    print(f"ERROR {error_code}: {message}")
```

### 2. Result Validation

```python
# Warn if return value is ignored
# @depyler: custom_attribute = "must_use"
def calculate_checksum(data: bytes) -> int:
    """Important calculation - result must be used."""
    checksum = 0
    for byte in data:
        checksum ^= byte
    return checksum
```

### 3. Compiler Warnings

```python
# Allow specific warnings
# @depyler: custom_attribute = "allow(unused_variables)"
def debug_function(x: int, debug_flag: bool) -> int:
    return x * 2  # debug_flag unused in production

# Deny unsafe code
# @depyler: custom_attribute = "deny(unsafe_code)"
def safe_function(data: list[int]) -> int:
    return sum(data)
```

### 4. Multiple Attributes

Specify multiple attributes by repeating the annotation:

```python
# @depyler: custom_attribute = "inline"
# @depyler: custom_attribute = "must_use"
def important_calculation(x: int, y: int) -> int:
    """Suggest inlining and ensure result is used."""
    return x * y + x - y
```

Transpiles to:

```rust
#[inline]
#[must_use]
pub fn important_calculation(x: i32, y: i32) -> i32 {
    x * y + x - y
}
```

### 5. Integration with Depyler Annotations

Custom Rust attributes work seamlessly with other Depyler annotations:

```python
# @depyler: optimization_level = "aggressive"
# @depyler: custom_attribute = "inline(always)"
# @depyler: performance_critical = "true"
# @depyler: bounds_checking = "explicit"
def optimized_hot_path(data: list[int]) -> int:
    total = 0
    for item in data:
        total += item
    return total
```

## Complete Examples

### Example 1: Hash Function with Compiler Hints

```python
# @depyler: custom_attribute = "inline(always)"
# @depyler: custom_attribute = "must_use"
# @depyler: optimization_level = "aggressive"
def compute_hash(data: bytes) -> int:
    """Hash computation with inline hint and must-use guarantee."""
    hash_val = 0
    for byte in data:
        hash_val = (hash_val * 31 + byte) % (2**32)
    return hash_val
```

Generated Rust:

```rust
#[inline(always)]
#[must_use]
pub fn compute_hash(data: &[u8]) -> i32 {
    let mut hash_val = 0i32;
    for byte in data {
        hash_val = (hash_val.wrapping_mul(31)
            .wrapping_add(*byte as i32)) % (2i32.pow(32));
    }
    hash_val
}
```

### Example 2: Error Handling with Cold Attribute

```python
# @depyler: custom_attribute = "cold"
# @depyler: error_strategy = "result_type"
def handle_critical_error(error_code: int, message: str) -> None:
    """Error handler - rarely executed, shouldn't be inlined."""
    print(f"CRITICAL ERROR {error_code}: {message}")
    # Log to file, send alert, etc.
```

Generated Rust:

```rust
#[cold]
pub fn handle_critical_error(error_code: i32, message: &str) {
    println!("CRITICAL ERROR {}: {}", error_code, message);
}
```

### Example 3: Multiple Functions with Different Attributes

```python
# @depyler: custom_attribute = "inline"
def add(a: int, b: int) -> int:
    """Simple function that may benefit from inlining."""
    return a + b

# @depyler: custom_attribute = "inline(never)"
def add_with_logging(a: int, b: int) -> int:
    """Debug version - preserve for stack traces."""
    print(f"Adding {a} + {b}")
    return a + b

# @depyler: custom_attribute = "cold"
def add_error_handler(a: int, b: int) -> int:
    """Error case - mark as infrequently executed."""
    print("Error: overflow detected")
    return 0
```

## Implementation Details

### Architecture

The custom attributes feature is implemented across multiple components:

#### 1. Annotation Parsing (`depyler-annotations`)

- **Field Addition**: `custom_attributes: Vec<String>` in
  `TranspilationAnnotations`
- **Parser Logic**: Special handling in `parse_annotations()` to accumulate
  multiple `custom_attribute` values
- **Validation**: Attributes are stored as strings and validated during code
  generation

#### 2. Code Generation (`depyler-core`)

- **Function Attributes**: `codegen_function_attrs()` extended to emit custom
  attributes
- **Token Stream Parsing**: Attributes parsed as `TokenStream` to support
  complex syntax like `inline(always)` or `repr(C)`
- **Ordering**: Custom attributes placed after doc comments, before function
  signature

#### 3. Transpilation Pipeline

```
Python Source
    ↓
Parse Annotations → Extract custom_attribute values
    ↓
Build HIR → Store in function.annotations.custom_attributes
    ↓
Code Generation → Parse as TokenStream and emit #[...] 
    ↓
Rust Output
```

### Testing Strategy

Comprehensive test coverage ensures correctness:

#### Unit Tests (`depyler-annotations`)

| Test | Purpose |
|------|---------|
| `test_custom_custom_attribute_single` | Single attribute parsing |
| `test_custom_custom_attribute_multiple` | Accumulation of multiple attributes |
| `test_custom_custom_attribute_with_other_annotations` | Integration |
| `test_custom_custom_attribute_empty` | Default behavior |

#### Integration Tests (`depyler-core`)

| Test | Coverage |
|------|----------|
| `test_single_custom_attribute` | Basic transpilation |
| `test_multiple_custom_attributes` | Multiple on same function |
| `test_custom_attribute_with_args` | Parameterized attributes |
| `test_custom_attribute_cold` | Specific attribute types |
| `test_custom_attribute_repr` | Complex syntax |
| `test_custom_attributes_with_other_annotations` | Combined usage |
| `test_custom_attribute_with_docstring` | Docstring interaction |
| `test_multiple_functions_different_attributes` | Multiple functions |
| `test_parse_to_hir_preserves_custom_attributes` | HIR preservation |

## Usage Patterns

### Frequently Called Functions

For functions that benefit from inlining:

```python
# @depyler: custom_attribute = "inline"
def compute_value(x: int, y: int) -> int:
    return x * y + x - y
```

### Error Handling

For error paths marked as infrequent:

```python
# @depyler: custom_attribute = "cold"
def handle_panic(msg: str) -> None:
    print(f"PANIC: {msg}")
```

### API Functions

For public API functions where results are critical:

```python
# @depyler: custom_attribute = "must_use"
def validate_input(data: str) -> bool:
    return len(data) > 0 and data.isalnum()
```

## CLI Usage

Transpile code with custom attributes:

```bash
# Basic transpilation
depyler transpile examples/custom_attributes_demo.py -o output.rs

# With verification
depyler transpile examples/custom_attributes_demo.py -o output.rs --verify

# View generated attributes
depyler transpile examples/custom_attributes_demo.py --stdout | grep "#\["
```

## Related Documentation

- [Annotation Syntax](annotation-syntax.md) - Complete annotation reference
- [Safety Guarantees](safety-guarantees.md) - Safety verification
- [Type Inference Hints](type-inference-hints.md) - Type system integration
- [User Guide](user-guide.md) - General usage information

**See Also**: `examples/custom_attributes_demo.py` for a complete working
example demonstrating all features.

````
