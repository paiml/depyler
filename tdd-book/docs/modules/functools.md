# functools - Higher-Order Functions

Python's functools module provides tools for working with functions and callable objects. Depyler transpiles these operations to Rust's functional programming features, including closures and iterators.

## Python → Rust Mapping

| Python Function | Rust Equivalent | Notes |
|-----------------|-----------------|-------|
| `import functools` | `use std::iter::*` | Functional tools |
| `functools.reduce(f, seq)` | `seq.iter().fold()` | Reduce sequence |
| `functools.reduce(f, seq, init)` | `seq.iter().fold(init, f)` | With initial value |

**Note**: `functools.partial()` and `@lru_cache` are not yet supported in transpilation.

## reduce() - Sequence Reduction

### Basic Reduce

Apply a function cumulatively to reduce a sequence to a single value:

```python
from functools import reduce

def test_reduce() -> int:
    # Reduce list to sum
    numbers = [1, 2, 3, 4, 5]
    total = reduce(lambda x, y: x + y, numbers)

    return total
```

**Generated Rust:**

```rust
fn test_reduce() -> i32 {
    // Reduce list to sum
    let numbers = vec![1, 2, 3, 4, 5];
    let total = numbers.iter().fold(0, |x, y| x + y);

    total
}
```

**reduce() Behavior:**
- Takes first element as initial value (if no init provided)
- Applies function cumulatively from left to right
- Returns single accumulated value
- Equivalent to: `f(f(f(f(1, 2), 3), 4), 5)` for sum

### Reduce with Initial Value

Provide an explicit initial value for the accumulator:

```python
from functools import reduce

def test_reduce_initial() -> int:
    # Reduce with initial value
    numbers = [1, 2, 3, 4]
    total = reduce(lambda x, y: x + y, numbers, 10)

    return total
```

**Generated Rust:**

```rust
fn test_reduce_initial() -> i32 {
    // Reduce with initial value
    let numbers = vec![1, 2, 3, 4];
    let total = numbers.iter().fold(10, |x, y| x + y);

    total
}
```

**Initial Value Benefits:**
- Handles empty sequences gracefully
- Allows different return type than sequence elements
- Makes intent explicit
- Required for empty sequences

### Finding Maximum with Reduce

Use reduce to find the maximum element:

```python
from functools import reduce

def test_reduce_max() -> int:
    # Find maximum using reduce
    numbers = [5, 2, 9, 1, 7]
    maximum = reduce(lambda x, y: x if x > y else y, numbers)

    return maximum
```

**Generated Rust:**

```rust
fn test_reduce_max() -> i32 {
    // Find maximum using reduce
    let numbers = vec![5, 2, 9, 1, 7];
    let maximum = numbers.iter().fold(numbers[0], |x, y| {
        if x > *y { x } else { *y }
    });

    maximum
}
```

**Alternative (More Idiomatic Rust):**
```rust
let maximum = *numbers.iter().max().unwrap();
```

### Product with Reduce

Calculate the product of all elements:

```python
from functools import reduce

def test_reduce_product() -> int:
    # Calculate product using reduce
    numbers = [2, 3, 4]
    product = reduce(lambda x, y: x * y, numbers)

    return product
```

**Generated Rust:**

```rust
fn test_reduce_product() -> i32 {
    // Calculate product using reduce
    let numbers = vec![2, 3, 4];
    let product = numbers.iter().fold(1, |x, y| x * y);

    product
}
```

**reduce() vs fold():**
- Python's `reduce(f, [a,b,c])` starts with `a`
- Rust's `fold(init, f)` always needs `init`
- Transpiler infers appropriate initial value

## Common Use Cases

### 1. Sum of List

```python
from functools import reduce

def sum_list(numbers: list) -> int:
    """Calculate sum of numbers."""
    return reduce(lambda x, y: x + y, numbers, 0)
```

### 2. Flatten Nested Lists

```python
from functools import reduce

def flatten(nested: list) -> list:
    """Flatten one level of nesting."""
    return reduce(lambda x, y: x + y, nested, [])

# Example: [[1,2], [3,4], [5]] -> [1,2,3,4,5]
```

### 3. String Concatenation

```python
from functools import reduce

def join_strings(strings: list, sep: str = "") -> str:
    """Join strings with separator."""
    if not strings:
        return ""
    return reduce(lambda x, y: x + sep + y, strings)
```

### 4. Compose Functions

```python
from functools import reduce

def compose(*functions):
    """Compose functions right-to-left."""
    return reduce(lambda f, g: lambda x: f(g(x)), functions)
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| `reduce(+, list)` | O(n) | O(n) | Linear scan |
| `reduce(*,list)` | O(n) | O(n) | Linear scan |
| `reduce(max, list)` | O(n) | O(n) | Single pass |

**Performance Notes:**
- Rust's `fold()` is zero-cost abstraction
- No heap allocations for primitive types
- Rust can auto-vectorize simple operations
- Both lazily evaluate (no intermediate collections)

**Rust Optimizations:**
- LLVM inlining of closures
- Loop unrolling for fixed-size arrays
- SIMD for arithmetic operations
- No function call overhead

## Alternative Rust Patterns

**Instead of reduce for common operations:**

```rust
// Sum
let sum: i32 = vec.iter().sum();  // More idiomatic

// Product  
let product: i32 = vec.iter().product();  // Built-in

// Maximum
let max = vec.iter().max().unwrap();  // Standard method

// Minimum
let min = vec.iter().min().unwrap();  // Standard method
```

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_functools.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_functools.py -v
```

**Expected Output:**
```
tests/test_functools.py::test_functools_reduce PASSED                    [ 25%]
tests/test_functools.py::test_functools_reduce_with_initial PASSED       [ 50%]
tests/test_functools.py::test_functools_reduce_max PASSED                [ 75%]
tests/test_functools.py::test_functools_reduce_multiply PASSED           [100%]

====== 4 passed in 0.XX s ======
```

## Functional Programming in Rust

**Python functools** provides higher-order functions.
**Rust** has functional programming built into the language:

| Concept | Python | Rust |
|---------|--------|------|
| Map | `map(f, seq)` | `seq.iter().map(f)` |
| Filter | `filter(f, seq)` | `seq.iter().filter(f)` |
| Reduce | `reduce(f, seq)` | `seq.iter().fold(init, f)` |
| Compose | `functools.compose` | Closures + traits |
| Partial | `functools.partial` | Closures capture |

**Rust Advantages:**
- Zero-cost abstractions
- No runtime overhead
- Compile-time optimizations
- Memory safety guarantees
- Lazy evaluation by default

## Future Support

**Currently Unsupported** (planned for future releases):
- `functools.partial()` - Partial function application
- `@functools.lru_cache` - Memoization decorator
- `@functools.cache` - Unbounded cache
- `functools.wraps()` - Decorator helper

**Workarounds:**
```rust
// Partial application via closures
let double = |x| multiply(2, x);

// Memoization via lazy_static + HashMap
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<HashMap<i32, i32>> = Mutex::new(HashMap::new());
}
```

## Comparison: reduce() vs for loop

**reduce():**
```python
total = reduce(lambda x, y: x + y, numbers)
```

**for loop:**
```python
total = 0
for num in numbers:
    total += num
```

**When to use reduce():**
- Functional programming style preferred
- Expressing intent clearly (aggregation)
- Chaining with other functional operations
- No early exit needed

**When to use for loop:**
- Need early exit (break)
- More complex accumulation logic
- Better readability for simple cases
- Need to modify external state

