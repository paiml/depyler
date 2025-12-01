# collections - Container Datatypes

Python's built-in collection types (list, dict, set) are fundamental to data manipulation. Depyler transpiles these to Rust's equivalent types with full type safety.

## Python → Rust Type Mapping

| Python Type | Rust Type | Notes |
|------------|-----------|-------|
| `list[T]` | `Vec<T>` | Dynamic array |
| `dict[K, V]` | `HashMap<K, V>` | Hash table |
| `set[T]` | `HashSet<T>` | Hash-based set |

## List Operations

### Basic List Methods

Depyler supports all common list methods with idiomatic Rust translations:

```python
def list_operations() -> list[int]:
    numbers: list[int] = [1, 2, 3]
    
    # Append element
    numbers.append(4)  # → numbers.push(4)
    
    # Extend with another list
    numbers.extend([5, 6])  # → numbers.extend([5, 6])
    
    # Remove first occurrence
    numbers.remove(3)  # → numbers.remove(pos)
    
    # Pop last element
    last = numbers.pop()  # → numbers.pop()
    
    return numbers  # [1, 2, 4, 5, 6]
```

**Generated Rust:**

```rust
fn list_operations() -> Vec<i32> {
    let mut numbers: Vec<i32> = vec![1, 2, 3];
    
    numbers.push(4);
    numbers.extend([5, 6]);
    
    if let Some(pos) = numbers.iter().position(|x| *x == 3) {
        numbers.remove(pos);
    }
    
    let last = numbers.pop();
    
    numbers
}
```

**Method Coverage:**
- ✅ `append()` → `push()`
- ✅ `extend()` → `extend()`
- ✅ `remove()` → `remove(pos)` with position lookup
- ✅ `pop()` → `pop()`
- ✅ `clear()` → `clear()`
- ✅ `insert()` → `insert()`

## Dictionary Operations

### Basic Dict Methods

```python
def dict_operations() -> dict[str, int]:
    scores: dict[str, int] = {"alice": 95, "bob": 87}
    
    # Get with default
    alice_score = scores.get("alice", 0)  # → get().unwrap_or(0)
    
    # Insert new key
    scores["charlie"] = 92  # → insert()
    
    # Pop key
    bob_score = scores.pop("bob")  # → remove()
    
    return scores  # {"alice": 95, "charlie": 92}
```

**Generated Rust:**

```rust
use std::collections::HashMap;

fn dict_operations() -> HashMap<String, i32> {
    let mut scores: HashMap<String, i32> = HashMap::from([
        ("alice".to_string(), 95),
        ("bob".to_string(), 87),
    ]);
    
    let alice_score = scores.get("alice").copied().unwrap_or(0);
    
    scores.insert("charlie".to_string(), 92);
    
    let bob_score = scores.remove("bob");
    
    scores
}
```

**Method Coverage:**
- ✅ `get()` → `get().copied().unwrap_or(default)`
- ✅ `pop()` → `remove()`
- ✅ `clear()` → `clear()`

## Set Operations

### Basic Set Methods

```python
def set_operations() -> set[int]:
    numbers: set[int] = {1, 2, 3}
    
    # Add element
    numbers.add(4)  # → insert()
    
    # Discard element (no error if missing)
    numbers.discard(2)  # → remove() without panic
    
    # Union with another set
    other: set[int] = {4, 5, 6}
    result = numbers.union(other)  # → union()
    
    return result  # {1, 3, 4, 5, 6}
```

**Generated Rust:**

```rust
use std::collections::HashSet;

fn set_operations() -> HashSet<i32> {
    let mut numbers: HashSet<i32> = HashSet::from([1, 2, 3]);
    
    numbers.insert(4);
    
    numbers.remove(&2);
    
    let other: HashSet<i32> = HashSet::from([4, 5, 6]);
    let result: HashSet<i32> = numbers.union(&other).copied().collect();
    
    result
}
```

**Method Coverage:**
- ✅ `add()` → `insert()`
- ✅ `remove()` → `remove()` with panic on missing
- ✅ `discard()` → `remove()` without panic
- ✅ `union()` → `union()`

## List Comprehensions

Depyler transpiles list comprehensions to efficient Rust iterators:

```python
def list_comp() -> list[int]:
    numbers: list[int] = [1, 2, 3, 4, 5]
    squares = [x * x for x in numbers if x % 2 == 0]
    return squares  # [4, 16]
```

**Generated Rust:**

```rust
fn list_comp() -> Vec<i32> {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect();
    squares
}
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| List append | O(1)* | O(1)* | Amortized |
| Dict lookup | O(1)* | O(1)* | Average case |
| Set membership | O(1)* | O(1)* | Average case |

\* Amortized or average case complexity

## Memory Safety Guarantees

Depyler's generated Rust code provides:

- **No null pointer dereferences**: All operations are type-safe
- **No buffer overflows**: Bounds checking on all array accesses  
- **No use-after-free**: Ownership system prevents dangling pointers
- **Thread safety**: Collections can be safely shared across threads when appropriate

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_collections.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_collections.py -v
```
