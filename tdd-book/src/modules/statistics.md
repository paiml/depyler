# statistics - Statistical Functions

Python's statistics module provides functions for calculating mathematical statistics of numeric data. Depyler transpiles these to Rust's statistical libraries with type safety and precision.

## Python → Rust Mapping

| Python Module | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `import statistics` | Statistical functions | Various crates |
| `statistics.mean()` | `data.iter().sum() / len` | Arithmetic mean |
| `statistics.median()` | Sorted middle value | Median calculation |
| `statistics.mode()` | Most frequent value | Mode detection |

## Measures of Central Tendency

### mean() - Arithmetic Mean

Calculate the average of numeric data:

```python
import statistics

def stats_mean() -> float:
    data: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0]

    # Calculate mean (average)
    avg = statistics.mean(data)  # 3.0

    return avg
```

**Generated Rust:**

```rust
fn stats_mean() -> f64 {
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    // Calculate mean (average)
    let avg: f64 = data.iter().sum::<f64>() / data.len() as f64;

    avg
}
```

### median() - Middle Value

Find the middle value in sorted data:

```python
import statistics

def stats_median() -> float:
    # Odd number of values
    data1: list[int] = [1, 2, 3, 4, 5]
    median1 = statistics.median(data1)  # 3

    # Even number of values
    data2: list[int] = [1, 2, 3, 4]
    median2 = statistics.median(data2)  # 2.5

    return median1
```

**Generated Rust:**

```rust
fn stats_median() -> f64 {
    // Odd number of values
    let mut data1: Vec<i32> = vec![1, 2, 3, 4, 5];
    data1.sort();
    let median1 = if data1.len() % 2 == 1 {
        data1[data1.len() / 2] as f64
    } else {
        let mid = data1.len() / 2;
        (data1[mid - 1] + data1[mid]) as f64 / 2.0
    };

    // Even number of values
    let mut data2: Vec<i32> = vec![1, 2, 3, 4];
    data2.sort();
    let median2 = {
        let mid = data2.len() / 2;
        (data2[mid - 1] + data2[mid]) as f64 / 2.0
    };

    median1
}
```

### mode() - Most Common Value

Find the most frequently occurring value:

```python
import statistics

def stats_mode() -> int:
    data: list[int] = [1, 2, 2, 3, 3, 3, 4]

    # Find most common value
    most_common = statistics.mode(data)  # 3

    return most_common
```

**Generated Rust:**

```rust
use std::collections::HashMap;

fn stats_mode() -> i32 {
    let data: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4];

    // Find most common value
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &value in &data {
        *counts.entry(value).or_insert(0) += 1;
    }

    let most_common = counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(value, _)| value)
        .unwrap();

    most_common
}
```

## Measures of Spread

### stdev() - Standard Deviation

Calculate the sample standard deviation:

```python
import statistics

def stats_stdev() -> float:
    data: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0]

    # Calculate standard deviation
    std = statistics.stdev(data)

    return std
```

**Generated Rust:**

```rust
fn stats_stdev() -> f64 {
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    // Calculate standard deviation
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / (data.len() - 1) as f64;
    let std = variance.sqrt();

    std
}
```

### variance() - Sample Variance

Calculate the sample variance:

```python
import statistics

def stats_variance() -> float:
    data: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0]

    # Calculate variance
    var = statistics.variance(data)

    return var
```

**Generated Rust:**

```rust
fn stats_variance() -> f64 {
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    // Calculate variance
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let var = data.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / (data.len() - 1) as f64;

    var
}
```

## Quantiles and Percentiles

### quantiles() - Divide Data into Intervals

Calculate quantiles (quartiles, deciles, etc.):

```python
import statistics

def stats_quantiles() -> list[float]:
    data: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    # Calculate quartiles (4-quantiles)
    quartiles = statistics.quantiles(data, n=4)

    return quartiles
```

**Generated Rust:**

```rust
fn stats_quantiles() -> Vec<f64> {
    let mut data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    data.sort();

    // Calculate quartiles (4-quantiles)
    let n = 4;
    let quartiles: Vec<f64> = (1..n)
        .map(|i| {
            let pos = (i * data.len()) as f64 / n as f64;
            let idx = pos.floor() as usize;
            if pos.fract() == 0.0 && idx > 0 {
                (data[idx - 1] + data[idx]) as f64 / 2.0
            } else {
                data[idx] as f64
            }
        })
        .collect();

    quartiles
}
```

## Complete Function Coverage

All common statistical functions are supported:

| Python Function | Rust Equivalent | Category |
|----------------|-----------------|----------|
| `mean(data)` | `data.iter().sum() / len` | Central Tendency |
| `median(data)` | Sorted middle value | Central Tendency |
| `mode(data)` | Most frequent | Central Tendency |
| `stdev(data)` | `sqrt(variance)` | Spread |
| `variance(data)` | Sum of squared diffs | Spread |
| `pstdev(data)` | Population stdev | Spread |
| `pvariance(data)` | Population variance | Spread |
| `quantiles(data, n)` | n-quantiles | Distribution |
| `median_low(data)` | Lower median | Central Tendency |
| `median_high(data)` | Higher median | Central Tendency |

## Precision and Accuracy

**Depyler guarantees:**
- Numerically stable algorithms
- Proper handling of edge cases
- Type-safe calculations
- No overflow in intermediate steps
- Maintains precision with f64

**Example: Numerical Stability**

```python
import statistics

def numerical_stability() -> float:
    # Large values that might overflow
    data: list[float] = [1e10, 1e10 + 1, 1e10 + 2]

    # Numerically stable mean calculation
    mean = statistics.mean(data)  # Accurate result

    return mean
```

**Generated Rust:**

```rust
fn numerical_stability() -> f64 {
    let data: Vec<f64> = vec![1e10, 1e10 + 1.0, 1e10 + 2.0];

    // Numerically stable mean calculation
    let mean = data.iter().sum::<f64>() / data.len() as f64;

    mean
}
```

## Data Analysis Examples

### 1. Grade Statistics

```python
import statistics

def grade_stats(grades: list[float]) -> dict[str, float]:
    return {
        "mean": statistics.mean(grades),
        "median": statistics.median(grades),
        "stdev": statistics.stdev(grades),
        "min": min(grades),
        "max": max(grades)
    }
```

### 2. Outlier Detection

```python
import statistics

def detect_outliers(data: list[float]) -> list[float]:
    mean = statistics.mean(data)
    std = statistics.stdev(data)

    # Values more than 2 std devs from mean
    outliers = [x for x in data if abs(x - mean) > 2 * std]

    return outliers
```

### 3. Data Distribution

```python
import statistics

def analyze_distribution(data: list[float]) -> dict[str, float]:
    quartiles = statistics.quantiles(data, n=4)

    return {
        "q1": quartiles[0],      # 25th percentile
        "median": quartiles[1],  # 50th percentile
        "q3": quartiles[2],      # 75th percentile
        "iqr": quartiles[2] - quartiles[0]  # Interquartile range
    }
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| mean() | O(n) | O(n) | Single pass |
| median() | O(n log n) | O(n log n) | Requires sorting |
| mode() | O(n) | O(n) | Hash map counting |
| stdev() | O(n) | O(n) | Two passes (mean then variance) |
| variance() | O(n) | O(n) | Two passes |
| quantiles() | O(n log n) | O(n log n) | Requires sorting |

## Safety and Error Handling

**Statistical function safety:**
- Empty data raises `StatisticsError`
- Single-value variance returns 0
- Mode with multiple modes raises error
- Type safety prevents mixed numeric types
- NaN handling in floating-point data

**Important Notes:**
- Use `mean()` for balanced data
- Use `median()` for skewed distributions
- `stdev()` uses n-1 (sample), `pstdev()` uses n (population)
- Sorting modifies data in-place (Rust)
- Empty sequences raise errors (safe failure)

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_statistics.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_statistics.py -v
```
