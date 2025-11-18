# Depyler Annotation Syntax Specification

## Overview

Depyler annotations are special comments that guide the transpilation process,
providing hints about type strategies, memory management, performance
optimization, and safety requirements. Annotations use the `@depyler:` prefix
and follow a key-value syntax.

## Syntax Format

```python
# @depyler: key = "value"
```

- Annotations must be comments starting with `#`
- The `@depyler:` prefix identifies the comment as a transpilation annotation
- Key-value pairs use `=` as the separator
- String values should be quoted
- Boolean values can be `"true"` or `"false"`
- Numeric values don't require quotes

## Annotation Categories

### 1. Type Strategy Annotations

Control how types are inferred and mapped from Python to Rust.

#### `type_strategy`

- **Values**: `"conservative"` | `"aggressive"` | `"zero_copy"` |
  `"always_owned"`
- **Default**: `"conservative"`
- **Description**: Controls the overall type mapping strategy
- **Example**:
  ```python
  # @depyler: type_strategy = "zero_copy"
  def process_data(data: bytes) -> bytes:
      return data[10:20]
  ```

#### `string_strategy`

- **Values**: `"conservative"` | `"always_owned"` | `"zero_copy"`
- **Default**: `"conservative"`
- **Description**: Specific strategy for string handling
- **Example**:
  ```python
  # @depyler: string_strategy = "zero_copy"
  def get_substring(s: str, start: int, end: int) -> str:
      return s[start:end]
  ```

### 2. Memory Management Annotations

Control ownership and memory safety patterns.

#### `ownership`

- **Values**: `"owned"` | `"borrowed"` | `"shared"`
- **Default**: `"owned"`
- **Description**: Specifies the ownership model for function parameters and
  return values
- **Example**:
  ```python
  # @depyler: ownership = "borrowed"
  def calculate_sum(numbers: List[int]) -> int:
      return sum(numbers)
  ```

#### `interior_mutability`

- **Values**: `"none"` | `"arc_mutex"` | `"ref_cell"` | `"cell"`
- **Default**: `"none"`
- **Description**: Specifies interior mutability pattern for shared state
- **Example**:
  ```python
  # @depyler: interior_mutability = "arc_mutex"
  class SharedCounter:
      def __init__(self):
          self.count = 0
  ```

### 3. Safety Annotations

Control safety checks and error handling.

#### `safety_level`

- **Values**: `"safe"` | `"unsafe_allowed"`
- **Default**: `"safe"`
- **Description**: Whether to allow unsafe Rust code generation
- **Example**:
  ```python
  # @depyler: safety_level = "unsafe_allowed"
  def raw_memory_access(ptr: int) -> int:
      # Direct memory access
      pass
  ```

#### `bounds_checking`

- **Values**: `"explicit"` | `"implicit"` | `"disabled"`
- **Default**: `"explicit"`
- **Description**: Controls array bounds checking behavior
- **Example**:
  ```python
  # @depyler: bounds_checking = "explicit"
  def safe_access(arr: List[int], idx: int) -> Optional[int]:
      if 0 <= idx < len(arr):
          return arr[idx]
      return None
  ```

#### `panic_behavior`

- **Values**: `"propagate"` | `"return_error"` | `"abort"`
- **Default**: `"propagate"`
- **Description**: How to handle panic situations
- **Example**:
  ```python
  # @depyler: panic_behavior = "return_error"
  def divide(a: int, b: int) -> Optional[float]:
      if b == 0:
          return None
      return a / b
  ```

#### `error_strategy`

- **Values**: `"panic"` | `"result_type"` | `"option_type"`
- **Default**: `"panic"`
- **Description**: Error handling approach
- **Example**:
  ```python
  # @depyler: error_strategy = "result_type"
  def parse_number(s: str) -> Union[int, str]:
      try:
          return int(s)
      except ValueError:
          return "Invalid number"
  ```

### 4. Performance Annotations

Guide optimization decisions.

#### `optimization_level`

- **Values**: `"standard"` | `"aggressive"` | `"conservative"`
- **Default**: `"standard"`
- **Description**: Overall optimization aggressiveness
- **Example**:
  ```python
  # @depyler: optimization_level = "aggressive"
  def compute_intensive_task(data: List[float]) -> float:
      # Complex computation
      pass
  ```

#### `performance_critical`

- **Values**: `"true"` | `"false"`
- **Default**: `"false"`
- **Description**: Marks functions as performance critical
- **Example**:
  ```python
  # @depyler: performance_critical = "true"
  def hot_path_function(x: int) -> int:
      return x * 2
  ```

#### `optimization_hint`

- **Values**: `"vectorize"` | `"latency"` | `"throughput"` | `"async_ready"`
- **Description**: Specific optimization hints
- **Example**:
  ```python
  # @depyler: optimization_hint = "vectorize"
  def process_array(arr: List[float]) -> List[float]:
      return [x * 2.0 for x in arr]
  ```

#### `vectorize`

- **Values**: `"true"` | `"false"`
- **Default**: `"false"`
- **Description**: Enable SIMD vectorization
- **Example**:
  ```python
  # @depyler: vectorize = "true"
  def dot_product(a: List[float], b: List[float]) -> float:
      return sum(x * y for x, y in zip(a, b))
  ```

#### `unroll_loops`

- **Values**: Numeric (e.g., `"4"`, `"8"`)
- **Description**: Loop unrolling factor
- **Example**:
  ```python
  # @depyler: unroll_loops = "4"
  def sum_array(arr: List[int]) -> int:
      total = 0
      for x in arr:
          total += x
      return total
  ```

### 5. Concurrency Annotations

Control thread safety and concurrency patterns.

#### `thread_safety`

- **Values**: `"required"` | `"not_required"`
- **Default**: `"not_required"`
- **Description**: Whether the code must be thread-safe
- **Example**:
  ```python
  # @depyler: thread_safety = "required"
  def concurrent_update(shared_data: Dict[str, int], key: str, value: int):
      shared_data[key] = value
  ```

### 6. Verification Annotations

Guide formal verification and property checking.

#### `termination`

- **Values**: `"unknown"` | `"proven"` | `"bounded_N"` (where N is a number)
- **Default**: `"unknown"`
- **Description**: Termination guarantee
- **Example**:
  ```python
  # @depyler: termination = "proven"
  def factorial(n: int) -> int:
      if n <= 1:
          return 1
      return n * factorial(n - 1)
  ```

#### `invariant`

- **Values**: String expression
- **Description**: Loop or function invariants
- **Example**:
  ```python
  # @depyler: invariant = "left <= right"
  def binary_search(arr: List[int], target: int) -> int:
      left, right = 0, len(arr) - 1
      while left <= right:
          # ...
  ```

#### `verify_bounds`

- **Values**: `"true"` | `"false"`
- **Default**: `"false"`
- **Description**: Enable bounds verification
- **Example**:
  ```python
  # @depyler: verify_bounds = "true"
  def access_matrix(matrix: List[List[int]], i: int, j: int) -> int:
      return matrix[i][j]
  ```

### 7. Architecture Annotations

Guide architectural decisions.

#### `service_type`

- **Values**: `"web_api"` | `"cli"` | `"library"`
- **Description**: Type of service being built
- **Example**:
  ```python
  # @depyler: service_type = "web_api"
  def handle_request(request: Dict[str, Any]) -> Dict[str, Any]:
      # Process web request
      pass
  ```

#### `global_strategy`

- **Values**: `"none"` | `"lazy_static"` | `"once_cell"`
- **Default**: `"none"`
- **Description**: Strategy for handling global state
- **Example**:
  ```python
  # @depyler: global_strategy = "lazy_static"
  GLOBAL_CONFIG = {"debug": True}
  ```

#### `hash_strategy`

- **Values**: `"standard"` | `"fnv"` | `"ahash"`
- **Default**: `"standard"`
- **Description**: Hash function strategy for dictionaries
- **Example**:
  ```python
  # @depyler: hash_strategy = "fnv"
  def create_lookup_table() -> Dict[str, int]:
      return {"a": 1, "b": 2}
  ```

### 8. Migration Annotations

Control migration strategy from Python to Rust.

#### `migration_strategy`

- **Values**: `"incremental"` | `"big_bang"` | `"hybrid"`
- **Description**: Overall migration approach
- **Example**:
  ```python
  # @depyler: migration_strategy = "incremental"
  # @depyler: compatibility_layer = "pyo3"
  class LegacySystem:
      pass
  ```

#### `compatibility_layer`

- **Values**: `"pyo3"` | `"ctypes"` | `"none"`
- **Description**: Python-Rust interop mechanism
- **Example**:
  ```python
  # @depyler: compatibility_layer = "pyo3"
  def rust_callable_function(x: int) -> int:
      return x * 2
  ```

### 9. Fallback Annotations

Control fallback behavior for complex constructs.

#### `fallback`

- **Values**: `"mcp"` | `"manual"` | `"error"`
- **Default**: `"error"`
- **Description**: What to do when automatic transpilation fails
- **Example**:
  ```python
  # @depyler: fallback = "mcp"
  def complex_dynamic_function(*args, **kwargs):
      # Complex dynamic behavior
      pass
  ```

#### `pattern`

- **Values**: String (e.g., `"builder"`, `"singleton"`)
- **Description**: Design pattern hint for MCP
- **Example**:
  ```python
  # @depyler: pattern = "builder"
  class ConfigBuilder:
      def __init__(self):
          self.config = {}
  ```

### 10. Custom Rust Attributes

Add custom Rust attributes directly to generated code.

#### `custom_attribute`

- **Values**: Any valid Rust attribute string
- **Default**: None
- **Description**: Adds custom Rust attributes to the generated function or class. Can be specified multiple times to add multiple attributes. Supports simple attributes (e.g., `inline`, `must_use`) and attributes with parameters (e.g., `inline(always)`, `repr(C)`).
- **Examples**:
  ```python
  # Simple attribute
  # @depyler: custom_attribute = "inline"
  def fast_function(x: int) -> int:
      return x * 2
  
  # Generated Rust:
  # #[inline]
  # pub fn fast_function(x: i32) -> i32 {
  #     x * 2
  # }
  
  # Attribute with parameters
  # @depyler: custom_attribute = "inline(always)"
  def critical_path(x: int) -> int:
      return x + 1
  
  # Generated Rust:
  # #[inline(always)]
  # pub fn critical_path(x: i32) -> i32 {
  #     x + 1
  # }
  
  # Multiple attributes
  # @depyler: custom_attribute = "inline"
  # @depyler: custom_attribute = "must_use"
  def important_calc(x: int) -> int:
      return x * 2
  
  # Generated Rust:
  # #[inline]
  # #[must_use]
  # pub fn important_calc(x: i32) -> i32 {
  #     x * 2
  # }
  
  # Cold function hint
  # @depyler: custom_attribute = "cold"
  def error_handler(msg: str) -> None:
      print(f"Error: {msg}")
  
  # Generated Rust:
  # #[cold]
  # pub fn error_handler(msg: &str) {
  #     println!("Error: {}", msg);
  # }
  ```

**Common Rust Attributes**:
- `inline` - Hint to inline the function
- `inline(always)` - Force function inlining
- `inline(never)` - Prevent function inlining
- `must_use` - Warn if function result is not used
- `cold` - Mark function as rarely executed (for error paths)
- `repr(C)` - Use C representation (for structs)
- `derive(Debug, Clone)` - Derive traits (for structs/enums)
- `allow(unused_variables)` - Suppress specific warnings
- `cfg(test)` - Compile only in test mode

## Annotation Placement Rules

1. **Function Annotations**: Place directly above the function definition
   ```python
   # @depyler: optimization_level = "aggressive"
   # @depyler: thread_safety = "required"
   def critical_function():
       pass
   ```

2. **Class Annotations**: Place directly above the class definition
   ```python
   # @depyler: fallback = "mcp"
   # @depyler: pattern = "singleton"
   class DatabaseConnection:
       pass
   ```

3. **Module Annotations**: Place at the top of the file
   ```python
   # @depyler: migration_strategy = "incremental"
   # @depyler: service_type = "web_api"

   import typing
   ```

4. **Inline Annotations**: Can be placed within function bodies
   ```python
   def process_data(items: List[int]) -> List[int]:
       # @depyler: vectorize = "true"
       result = [x * 2 for x in items]
       return result
   ```

## Annotation Validation

The annotation parser performs validation to ensure:

- No conflicting annotations (e.g., `string_strategy = "zero_copy"` with
  `ownership = "owned"`)
- Valid values for each annotation key
- Appropriate combinations (e.g., `thread_safety = "required"` requires
  thread-safe interior mutability)

## Best Practices

1. **Start Conservative**: Begin with conservative defaults and add annotations
   as needed
2. **Profile First**: Use performance annotations only after profiling
   identifies bottlenecks
3. **Document Intent**: Annotations serve as documentation for transpilation
   decisions
4. **Incremental Adoption**: Add annotations gradually as you understand their
   impact
5. **Validate Early**: Run validation checks to catch annotation conflicts early

## Examples

### Example 1: High-Performance Numeric Computation

```python
# @depyler: performance_critical = "true"
# @depyler: optimization_level = "aggressive"
# @depyler: vectorize = "true"
# @depyler: bounds_checking = "disabled"
def matrix_multiply(a: List[List[float]], b: List[List[float]]) -> List[List[float]]:
    rows_a, cols_a = len(a), len(a[0])
    cols_b = len(b[0])
    result = [[0.0] * cols_b for _ in range(rows_a)]
    
    # @depyler: unroll_loops = "4"
    for i in range(rows_a):
        for j in range(cols_b):
            for k in range(cols_a):
                result[i][j] += a[i][k] * b[k][j]
    
    return result
```

### Example 2: Thread-Safe Web Service

```python
# @depyler: service_type = "web_api"
# @depyler: thread_safety = "required"
# @depyler: optimization_hint = "latency"
# @depyler: error_strategy = "result_type"
def handle_api_request(request: Dict[str, Any]) -> Union[Dict[str, Any], str]:
    # @depyler: ownership = "shared"
    # @depyler: interior_mutability = "arc_mutex"
    try:
        validate_request(request)
        result = process_request(request)
        return {"status": "success", "data": result}
    except ValueError as e:
        return f"Error: {str(e)}"
```

### Example 3: Zero-Copy String Processing

```python
# @depyler: string_strategy = "zero_copy"
# @depyler: ownership = "borrowed"
# @depyler: bounds_checking = "explicit"
def extract_fields(data: str, delimiter: str = ",") -> List[str]:
    # @depyler: verify_bounds = "true"
    fields = []
    start = 0
    
    while start < len(data):
        end = data.find(delimiter, start)
        if end == -1:
            fields.append(data[start:])
            break
        fields.append(data[start:end])
        start = end + 1
    
    return fields
```

### Example 4: Custom Rust Attributes for Performance

```python
# @depyler: custom_attribute = "inline(always)"
# @depyler: custom_attribute = "must_use"
# @depyler: performance_critical = "true"
def compute_hash(data: bytes) -> int:
    """Fast hash computation for hot path."""
    hash_val = 0
    for byte in data:
        hash_val = (hash_val * 31 + byte) % (2**32)
    return hash_val

# @depyler: custom_attribute = "cold"
# @depyler: error_strategy = "result_type"
def handle_error(error_code: int, message: str) -> None:
    """Error handler - rarely executed."""
    print(f"ERROR {error_code}: {message}")
    # Log to file, send alert, etc.
```

Generated Rust:
```rust
#[inline(always)]
#[must_use]
pub fn compute_hash(data: &[u8]) -> i32 {
    let mut hash_val = 0i32;
    for byte in data {
        hash_val = (hash_val.wrapping_mul(31).wrapping_add(*byte as i32)) % (2i32.pow(32));
    }
    hash_val
}

#[cold]
pub fn handle_error(error_code: i32, message: &str) {
    println!("ERROR {}: {}", error_code, message);
}
```

## Future Extensions

The annotation system is designed to be extensible. Future versions may add:

- Async/await annotations
- GPU computation hints
- Distributed computing annotations
- Custom annotation validators
- IDE integration for annotation suggestions
