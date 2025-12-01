# itertools - Iterator Building Blocks

Python's itertools module provides efficient, memory-conserving iterator building blocks inspired by functional programming constructs. Depyler transpiles these operations to Rust's zero-cost iterator adapters with compile-time optimization.

## Python → Rust Mapping

| Python Function | Rust Equivalent | Notes |
|-----------------|-----------------|-------|
| `import itertools` | `use std::iter::*` | Iterator adapters |
| `chain(a, b)` | `a.chain(b)` | Concatenate iterators |
| `islice(iter, n)` | `iter.take(n)` | Slice iterator |
| `repeat(x, n)` | `std::iter::repeat(x).take(n)` | Repeat element |
| `count(start)` | `(start..).into_iter()` | Infinite counter |
| `zip_longest(a, b)` | Custom zip with padding | Requires itertools crate |
| `product(a, b)` | `a.flat_map(\|x\| b.map(\|y\| (x, y)))` | Cartesian product |

## Chaining Iterators

### chain() - Concatenate Multiple Iterables

Combine multiple iterables into a single iterator:

```python
from itertools import chain

def test_chain() -> int:
    # Chain multiple lists together
    list1 = [1, 2, 3]
    list2 = [4, 5, 6]
    list3 = [7, 8, 9]

    result = list(chain(list1, list2, list3))

    # Return sum of all elements
    total = sum(result)

    return total
```

**Generated Rust:**

```rust
fn test_chain() -> i32 {
    // Chain multiple lists together
    let list1 = vec![1, 2, 3];
    let list2 = vec![4, 5, 6];
    let list3 = vec![7, 8, 9];

    let result: Vec<i32> = list1.into_iter()
        .chain(list2)
        .chain(list3)
        .collect();

    // Return sum of all elements
    let total: i32 = result.iter().sum();

    total
}
```

**chain() Properties:**
- Zero-cost abstraction: No runtime overhead
- Memory efficient: Lazy evaluation
- Type safe: Preserves element types
- Composable: Can chain chains

## Slicing Iterators

### islice() - Take First N Elements

Extract a slice from an iterable:

```python
from itertools import islice

def test_islice() -> int:
    # Take first 5 elements from range
    numbers = range(100)
    first_five = list(islice(numbers, 5))

    # Sum of first five: 0+1+2+3+4 = 10
    total = sum(first_five)

    return total
```

**Generated Rust:**

```rust
fn test_islice() -> i32 {
    // Take first 5 elements from range
    let numbers = 0..100;
    let first_five: Vec<i32> = numbers.take(5).collect();

    // Sum of first five: 0+1+2+3+4 = 10
    let total: i32 = first_five.iter().sum();

    total
}
```

**islice() Properties:**
- Lazy evaluation: Doesn't consume entire iterator
- Memory efficient: Only processes needed elements
- Works with infinite iterators
- Rust's `take(n)` is equivalent and optimized


## Repeating Elements

### repeat() - Repeat Element N Times

Generate repeated values efficiently:

```python
from itertools import repeat

def test_repeat() -> int:
    # Repeat value 5 times
    repeated = list(repeat(10, 5))

    # Sum: 10*5 = 50
    total = sum(repeated)

    return total
```

**Generated Rust:**

```rust
fn test_repeat() -> i32 {
    // Repeat value 5 times
    let repeated: Vec<i32> = std::iter::repeat(10)
        .take(5)
        .collect();

    // Sum: 10*5 = 50
    let total: i32 = repeated.iter().sum();

    total
}
```

**repeat() Properties:**
- Constant memory: Doesn't store duplicates
- Infinite by default: Use `take(n)` to limit
- Zero-copy: Same value referenced
- Useful for padding and initialization

## Infinite Iterators

### count() - Infinite Counter

Generate sequential numbers starting from a value:

```python
from itertools import count, islice

def test_count() -> int:
    # Create counter starting at 10
    counter = count(10)

    # Take first 5 values: 10, 11, 12, 13, 14
    first_five = list(islice(counter, 5))

    # Sum: 10+11+12+13+14 = 60
    total = sum(first_five)

    return total
```

**Generated Rust:**

```rust
fn test_count() -> i32 {
    // Create counter starting at 10
    let counter = 10..;

    // Take first 5 values: 10, 11, 12, 13, 14
    let first_five: Vec<i32> = counter.take(5).collect();

    // Sum: 10+11+12+13+14 = 60
    let total: i32 = first_five.iter().sum();

    total
}
```

**count() Properties:**
- Infinite iterator: Never terminates
- Must be limited: Use `take()` or similar
- Rust range syntax: `start..` is equivalent
- Memory efficient: Computed on demand

## Zipping with Padding

### zip_longest() - Zip Iterables with Fill Value

Zip iterables of different lengths, padding shorter ones:

```python
from itertools import zip_longest

def test_zip_longest() -> int:
    # Zip lists of different lengths
    list1 = [1, 2, 3]
    list2 = [10, 20]

    # zip_longest pads shorter list with None (or fillvalue)
    result = list(zip_longest(list1, list2, fillvalue=0))

    # Count total elements: [(1,10), (2,20), (3,0)]
    count = len(result)

    return count
```

**Generated Rust:**

```rust
use itertools::Itertools;  // From itertools crate

fn test_zip_longest() -> i32 {
    // Zip lists of different lengths
    let list1 = vec![1, 2, 3];
    let list2 = vec![10, 20];

    // zip_longest pads shorter list with fillvalue
    let result: Vec<_> = list1.into_iter()
        .zip_longest(list2)
        .map(|pair| match pair {
            itertools::EitherOrBoth::Both(a, b) => (a, b),
            itertools::EitherOrBoth::Left(a) => (a, 0),
            itertools::EitherOrBoth::Right(b) => (0, b),
        })
        .collect();

    // Count total elements
    result.len() as i32
}
```

**zip_longest() Properties:**
- Handles unequal lengths: Pads shorter iterables
- Requires itertools crate in Rust
- Fill value configurable
- Use case: Parallel processing with unequal data

## Combinatorics

### product() - Cartesian Product

Generate all combinations of elements from multiple iterables:

```python
from itertools import product

def test_product() -> int:
    # Cartesian product of two lists
    list1 = [1, 2]
    list2 = [10, 20]

    result = list(product(list1, list2))

    # Should have 2*2 = 4 combinations: [(1,10), (1,20), (2,10), (2,20)]
    count = len(result)

    return count
```

**Generated Rust:**

```rust
fn test_product() -> i32 {
    // Cartesian product of two lists
    let list1 = vec![1, 2];
    let list2 = vec![10, 20];

    let result: Vec<_> = list1.iter()
        .flat_map(|&x| {
            list2.iter().map(move |&y| (x, y))
        })
        .collect();

    // Should have 2*2 = 4 combinations
    result.len() as i32
}
```

**product() Properties:**
- Cartesian product: All combinations
- Size: product of input lengths
- Lazy evaluation: Iterator-based
- Rust uses `flat_map` for nested iteration


## Common Use Cases

### 1. Batching Data Processing

Process data in chunks efficiently:

```python
from itertools import islice

def process_in_batches(data, batch_size):
    """Process data in batches using islice."""
    iterator = iter(data)
    while batch := list(islice(iterator, batch_size)):
        # Process batch
        yield sum(batch)
```

**Rust Equivalent:**

```rust
fn process_in_batches(data: Vec<i32>, batch_size: usize) -> Vec<i32> {
    data.chunks(batch_size)
        .map(|batch| batch.iter().sum())
        .collect()
}
```

### 2. Infinite Sequences

Generate infinite sequences with termination:

```python
from itertools import count, takewhile

def fibonacci_under_1000():
    """Generate Fibonacci numbers under 1000."""
    a, b = 0, 1
    counter = count()
    result = []
    
    for _ in counter:
        if a >= 1000:
            break
        result.append(a)
        a, b = b, a + b
    
    return result
```

### 3. Pairwise Iteration

Iterate over adjacent pairs:

```python
from itertools import chain

def pairwise(iterable):
    """Iterate over adjacent pairs."""
    a = iter(iterable)
    b = iter(iterable)
    next(b, None)  # Advance b by one
    return zip(a, b)
```

**Rust Equivalent:**

```rust
fn pairwise<I>(iter: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I: Iterator,
    I::Item: Clone,
{
    let mut iter = iter.peekable();
    std::iter::from_fn(move || {
        let first = iter.next()?;
        let second = iter.peek()?.clone();
        Some((first, second))
    })
}
```

### 4. Combinatorial Generation

Generate all combinations for testing:

```python
from itertools import product

def test_all_combinations():
    """Test function with all input combinations."""
    inputs_a = [1, 2, 3]
    inputs_b = [True, False]
    
    results = []
    for a, b in product(inputs_a, inputs_b):
        result = process(a, b)
        results.append(result)
    
    return results
```

## Performance Characteristics

| Operation | Time Complexity | Space Complexity | Rust Advantage |
|-----------|----------------|------------------|----------------|
| `chain()` | O(n) | O(1) | Zero-cost iterator |
| `islice()` | O(k) | O(1) | No allocation |
| `repeat()` | O(1) per item | O(1) | Const propagation |
| `count()` | O(1) per item | O(1) | Range syntax |
| `zip_longest()` | O(n) | O(1) | Specialized impl |
| `product()` | O(n×m) | O(1) lazy | Iterator fusion |

**Performance Notes:**
- Rust iterators are zero-cost abstractions
- Lazy evaluation: Only compute what's needed
- Iterator fusion: Rust compiler optimizes chains
- No Python overhead: No GIL, no reference counting

**Rust Performance Advantages:**
- Compile-time optimization of iterator chains
- LLVM auto-vectorization (SIMD)
- No heap allocation for most operations
- Inlining eliminates function call overhead

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_itertools.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_itertools.py -v
```

**Expected Output:**
```
tests/test_itertools.py::test_itertools_chain PASSED         [ 16%]
tests/test_itertools.py::test_itertools_islice PASSED        [ 33%]
tests/test_itertools.py::test_itertools_repeat PASSED        [ 50%]
tests/test_itertools.py::test_itertools_count PASSED         [ 66%]
tests/test_itertools.py::test_itertools_zip_longest PASSED   [ 83%]
tests/test_itertools.py::test_itertools_product PASSED       [100%]

====== 6 passed in 0.XX s ======
```

## Alternative Rust Patterns

### Native Iterator Methods

Rust's standard library provides many iterator methods that overlap with itertools:

| Python itertools | Rust std::iter | Notes |
|-----------------|----------------|-------|
| `chain(a, b)` | `a.chain(b)` | Built-in |
| `islice(iter, n)` | `iter.take(n)` | Built-in |
| `islice(iter, n, m)` | `iter.skip(n).take(m-n)` | Composable |
| `repeat(x, n)` | `std::iter::repeat(x).take(n)` | Built-in |
| `count(n)` | `(n..).into_iter()` | Range syntax |

### itertools Crate

For more advanced operations, use the `itertools` crate:

```rust
use itertools::Itertools;

// Batching
iter.chunks(3);

// Unique elements
iter.unique();

// Cartesian product
iter.cartesian_product(other);

// Permutations
iter.permutations(k);

// Combinations
iter.combinations(k);
```

## Future Support

**Currently Supported:**
- ✅ `chain()` - Chaining iterators
- ✅ `islice()` - Slicing iterators
- ✅ `repeat()` - Repeating elements
- ✅ `count()` - Infinite counters
- ✅ `zip_longest()` - Zipping with padding
- ✅ `product()` - Cartesian products

**Planned Support:**
- 🔄 `permutations()` - Permutation generation
- 🔄 `combinations()` - Combination generation
- 🔄 `groupby()` - Grouping consecutive elements
- 🔄 `compress()` - Filtering by boolean mask
- 🔄 `accumulate()` - Running totals
- 🔄 `dropwhile()`, `takewhile()` - Conditional iteration

**Workarounds for Unsupported Features:**

```rust
// permutations() - Use itertools crate
use itertools::Itertools;
let perms: Vec<_> = vec![1, 2, 3].iter().permutations(2).collect();

// groupby() - Use chunk_by (nightly) or itertools
let groups = data.iter().group_by(|x| x / 10);

// accumulate() - Use scan()
let running_sum: Vec<_> = data.iter().scan(0, |acc, &x| {
    *acc += x;
    Some(*acc)
}).collect();
```

## Comparison: itertools vs for loops

**Memory Efficiency:**
```python
# BAD: Creates intermediate lists
result = list(chain(range(1000000), range(1000000)))

# GOOD: Lazy iterator
result = chain(range(1000000), range(1000000))
for item in result:
    process(item)
```

**Rust Equivalent:**
```rust
// Both are equally efficient in Rust
let result: Vec<_> = (0..1_000_000)
    .chain(0..1_000_000)
    .collect();  // Only allocates once

// Or fully lazy
(0..1_000_000)
    .chain(0..1_000_000)
    .for_each(|item| process(item));  // Zero allocation
```

## Iterator Composition

Combine multiple itertools operations:

```python
from itertools import chain, islice, repeat

def complex_pipeline():
    # Chain, repeat, slice
    a = [1, 2, 3]
    b = [4, 5, 6]
    padding = repeat(0, 3)
    
    result = list(islice(chain(a, b, padding), 10))
    return result
```

**Rust Equivalent:**

```rust
fn complex_pipeline() -> Vec<i32> {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    
    a.into_iter()
        .chain(b)
        .chain(std::iter::repeat(0).take(3))
        .take(10)
        .collect()
}
```

**Composition Benefits:**
- Readable: Left-to-right flow
- Efficient: Single-pass processing
- Type-safe: Compile-time checking
- Optimizable: Compiler can fuse operations

