# random - Random Number Generation

Python's random module provides pseudo-random number generation for various distributions. Depyler transpiles these to Rust's `rand` crate with full reproducibility and type safety.

## Python → Rust Mapping

| Python Module | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `import random` | `use rand::prelude::*` | RNG imports |
| `random.random()` | `rng.gen::<f64>()` | Float [0.0, 1.0) |
| `random.randint()` | `rng.gen_range()` | Inclusive range |

## Basic Random Functions

### random(), randint(), choice()

Generate random numbers and select random elements:

```python
import random

def random_basic() -> float:
    # Random float [0.0, 1.0)
    rand_val = random.random()  # → rng.gen::<f64>()

    # Random integer [1, 10]
    rand_int = random.randint(1, 10)  # → rng.gen_range(1..=10)

    # Random choice from list
    choices: list[str] = ["apple", "banana", "cherry"]
    choice = random.choice(choices)  # → rng.choose()

    return rand_val
```

**Generated Rust:**

```rust
use rand::prelude::*;

fn random_basic() -> f64 {
    let mut rng = thread_rng();

    let rand_val = rng.gen::<f64>();
    let rand_int = rng.gen_range(1..=10);

    let choices: Vec<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string()
    ];
    let choice = choices.choose(&mut rng).unwrap();

    rand_val
}
```

## Floating-Point Ranges

### uniform()

Generate random floats in a specific range:

```python
import random

def random_uniform() -> float:
    # Random float in range [1.0, 10.0]
    val = random.uniform(1.0, 10.0)  # → rng.gen_range()
    return val
```

**Generated Rust:**

```rust
use rand::prelude::*;

fn random_uniform() -> f64 {
    let mut rng = thread_rng();
    let val = rng.gen_range(1.0..=10.0);
    val
}
```

## Sequence Operations

### shuffle()

Randomly shuffle a list in-place:

```python
import random

def random_shuffle() -> list[int]:
    numbers: list[int] = [1, 2, 3, 4, 5]
    random.shuffle(numbers)  # → numbers.shuffle()
    return numbers
```

**Generated Rust:**

```rust
use rand::prelude::*;

fn random_shuffle() -> Vec<i32> {
    let mut rng = thread_rng();
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers.shuffle(&mut rng);
    numbers
}
```

## Reproducible Randomness

### seed()

Set the random seed for reproducible results:

```python
import random

def random_seed() -> float:
    random.seed(42)  # → StdRng::seed_from_u64(42)
    val = random.random()
    return val
```

**Generated Rust:**

```rust
use rand::prelude::*;
use rand::rngs::StdRng;

fn random_seed() -> f64 {
    let mut rng = StdRng::seed_from_u64(42);
    let val = rng.gen::<f64>();
    val
}
```

## Sampling Without Replacement

### sample()

Select multiple random elements without replacement:

```python
import random

def random_sample() -> list[int]:
    numbers: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sample = random.sample(numbers, 3)  # → sample_iter().take(3)
    return sample
```

**Generated Rust:**

```rust
use rand::prelude::*;

fn random_sample() -> Vec<i32> {
    let mut rng = thread_rng();
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sample: Vec<i32> = numbers
        .choose_multiple(&mut rng, 3)
        .cloned()
        .collect();

    sample
}
```

## Complete Function Coverage

All common random functions are supported:

| Python Function | Rust Equivalent | Category |
|----------------|-----------------|----------|
| `random()` | `rng.gen::<f64>()` | Basic |
| `randint(a, b)` | `rng.gen_range(a..=b)` | Basic |
| `choice(seq)` | `seq.choose(&mut rng)` | Sequence |
| `shuffle(seq)` | `seq.shuffle(&mut rng)` | Sequence |
| `sample(seq, k)` | `seq.choose_multiple(&mut rng, k)` | Sequence |
| `uniform(a, b)` | `rng.gen_range(a..=b)` | Continuous |
| `seed(a)` | `StdRng::seed_from_u64(a)` | Control |
| `randrange(start, stop)` | `rng.gen_range(start..stop)` | Basic |
| `getstate()` | RNG state methods | Advanced |
| `setstate(state)` | RNG state methods | Advanced |

## RNG Types and Performance

Rust's `rand` crate provides multiple RNG algorithms:

| RNG Type | Python Equivalent | Use Case |
|----------|------------------|----------|
| `thread_rng()` | `random` default | General purpose |
| `StdRng` | Seeded RNG | Reproducible |
| `SmallRng` | Fast RNG | Performance-critical |
| `OsRng` | `os.urandom()` | Cryptographic |

## Reproducibility Guarantees

**Depyler guarantees:**
- Same seed → identical sequence (with StdRng)
- Cross-platform determinism
- No hidden state
- Explicit RNG threading

**Example:**

```python
import random

def reproducible_demo() -> list[float]:
    random.seed(42)
    vals = [random.random() for _ in range(5)]
    return vals
```

**Generated Rust:**

```rust
use rand::prelude::*;
use rand::rngs::StdRng;

fn reproducible_demo() -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(42);
    let vals: Vec<f64> = (0..5)
        .map(|_| rng.gen::<f64>())
        .collect();
    vals
}
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| random() | O(1) | O(1) | Hardware entropy |
| randint() | O(1) | O(1) | Fast rejection |
| choice() | O(1) | O(1) | Direct indexing |
| shuffle() | O(n) | O(n) | Fisher-Yates |
| sample() | O(k) | O(k) | Optimized selection |

## Safety and Security

**Random number safety:**
- No undefined behavior from bad seeds
- Thread-safe RNG access
- No global state races
- Cryptographic RNG available (OsRng)

**Important Notes:**
- `thread_rng()` is NOT cryptographically secure
- Use `OsRng` for security-critical applications
- Seeded RNGs are deterministic (not secure)

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_random.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_random.py -v
```
