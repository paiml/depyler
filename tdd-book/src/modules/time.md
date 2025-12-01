# time - Time Access and Conversions

Python's time module provides functions for working with time-related operations including timestamps, delays, and performance measurement. Depyler transpiles these to Rust's `std::time` and `std::thread` modules with precise timing guarantees.

## Python → Rust Mapping

| Python Module | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `import time` | `use std::time` | Time operations |
| `time.time()` | `SystemTime::now()` | Unix timestamp |
| `time.sleep(n)` | `thread::sleep()` | Pause execution |
| `time.perf_counter()` | `Instant::now()` | High-resolution timer |
| `time.monotonic()` | `Instant::now()` | Monotonic clock |

## Getting Timestamps

### time() - Current Unix Timestamp

Get the current time as seconds since the Unix epoch (January 1, 1970):

```python
import time

def get_timestamp() -> float:
    # Get current Unix timestamp
    timestamp = time.time()

    return timestamp
```

**Generated Rust:**

```rust
use std::time::{SystemTime, UNIX_EPOCH};

fn get_timestamp() -> f64 {
    // Get current Unix timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    timestamp
}
```

## Pausing Execution

### sleep() - Suspend Execution

Pause program execution for a specified duration:

```python
import time

def sleep_example() -> float:
    start = time.time()

    # Sleep for 0.1 seconds
    time.sleep(0.1)

    end = time.time()
    elapsed = end - start

    return elapsed
```

**Generated Rust:**

```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;

fn sleep_example() -> f64 {
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Sleep for 0.1 seconds
    thread::sleep(Duration::from_secs_f64(0.1));

    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let elapsed = end - start;

    elapsed
}
```

## Performance Measurement

### perf_counter() - High-Resolution Timer

Use a high-resolution performance counter for precise timing:

```python
import time

def measure_performance() -> float:
    # Start performance counter
    start = time.perf_counter()

    # Some operation
    result = sum(range(1000))

    # End performance counter
    end = time.perf_counter()
    elapsed = end - start

    return elapsed
```

**Generated Rust:**

```rust
use std::time::Instant;

fn measure_performance() -> f64 {
    // Start performance counter
    let start = Instant::now();

    // Some operation
    let result: i32 = (0..1000).sum();

    // End performance counter
    let end = Instant::now();
    let elapsed = end.duration_since(start).as_secs_f64();

    elapsed
}
```

## Monotonic Clock

### monotonic() - Clock That Never Goes Backward

Use a monotonic clock unaffected by system clock adjustments:

```python
import time

def monotonic_example() -> float:
    # Get monotonic time
    start = time.monotonic()

    # Some operation
    result = sum(range(100))

    end = time.monotonic()
    elapsed = end - start

    return elapsed
```

**Generated Rust:**

```rust
use std::time::Instant;

fn monotonic_example() -> f64 {
    // Get monotonic time
    let start = Instant::now();

    // Some operation
    let result: i32 = (0..100).sum();

    let end = Instant::now();
    let elapsed = end.duration_since(start).as_secs_f64();

    elapsed
}
```

## CPU Time Measurement

### process_time() - CPU Time Used by Process

Measure CPU time consumed by the current process:

```python
import time

def measure_cpu_time() -> float:
    # Measure CPU time used by process
    start = time.process_time()

    # CPU-intensive operation
    result = sum(range(10000))

    end = time.process_time()
    cpu_time = end - start

    return cpu_time
```

**Generated Rust:**

```rust
use std::time::Instant;

fn measure_cpu_time() -> f64 {
    // Measure CPU time used by process
    let start = Instant::now();

    // CPU-intensive operation
    let result: i32 = (0..10000).sum();

    let end = Instant::now();
    let cpu_time = end.duration_since(start).as_secs_f64();

    cpu_time
}
```

## Complete Function Coverage

All common time functions are supported:

| Python Function | Rust Equivalent | Category |
|----------------|-----------------|----------|
| `time.time()` | `SystemTime::now()` | Timestamps |
| `time.sleep(n)` | `thread::sleep()` | Delays |
| `time.perf_counter()` | `Instant::now()` | Performance |
| `time.monotonic()` | `Instant::now()` | Monotonic |
| `time.process_time()` | `Instant::now()` | CPU Time |

## Time Resolution and Accuracy

**Resolution characteristics:**
- `time()`: System-dependent, typically microsecond precision
- `sleep()`: System-dependent, minimum ~1ms on most platforms
- `perf_counter()`: Nanosecond resolution on modern systems
- `monotonic()`: Nanosecond resolution, unaffected by clock adjustments
- `process_time()`: CPU time with microsecond precision

## Common Use Cases

### 1. Simple Benchmarking

```python
import time

def benchmark_operation() -> float:
    start = time.perf_counter()

    # Operation to benchmark
    result = sum(range(1000000))

    elapsed = time.perf_counter() - start
    return elapsed
```

### 2. Rate Limiting

```python
import time

def rate_limited_operation(delay: float) -> None:
    # Ensure minimum time between operations
    time.sleep(delay)
```

### 3. Timeout Implementation

```python
import time

def operation_with_timeout(timeout: float) -> bool:
    start = time.monotonic()

    while time.monotonic() - start < timeout:
        # Check condition
        if condition_met():
            return True

    return False
```

### 4. Performance Profiling

```python
import time

def profile_sections() -> dict[str, float]:
    results = {}

    start = time.perf_counter()
    # Section 1
    section1()
    results["section1"] = time.perf_counter() - start

    start = time.perf_counter()
    # Section 2
    section2()
    results["section2"] = time.perf_counter() - start

    return results
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| `time()` | O(1) | O(1) | System call |
| `sleep()` | O(1) | O(1) | OS scheduler |
| `perf_counter()` | O(1) | O(1) | Hardware counter |
| `monotonic()` | O(1) | O(1) | Hardware counter |
| `process_time()` | O(1) | O(1) | Process stats |

## Safety and Guarantees

**Time operation safety:**
- `time()` returns seconds since epoch (never negative in practice)
- `sleep()` may sleep longer than requested (never shorter)
- `perf_counter()` has highest available resolution
- `monotonic()` guaranteed never to go backward
- `process_time()` excludes sleep time (CPU time only)

**Important Notes:**
- Use `perf_counter()` for precise timing measurements
- Use `monotonic()` when measuring elapsed time
- Use `time()` for timestamps and wall-clock time
- `sleep()` accuracy depends on OS scheduler (typically ~1ms minimum)
- Negative sleep values raise `ValueError`

## Clock Comparison

Different clocks serve different purposes:

```python
import time

def clock_comparison() -> dict[str, float]:
    # Wall-clock time (can jump forward/backward with system clock)
    wall_time = time.time()

    # Monotonic time (never goes backward, relative to system start)
    monotonic_time = time.monotonic()

    # Performance counter (highest resolution, for benchmarking)
    perf_time = time.perf_counter()

    # CPU time (excludes sleep/IO time, measures actual CPU usage)
    cpu_time = time.process_time()

    return {
        "wall": wall_time,
        "monotonic": monotonic_time,
        "perf": perf_time,
        "cpu": cpu_time
    }
```

**Generated Rust:**

```rust
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::collections::HashMap;

fn clock_comparison() -> HashMap<String, f64> {
    // Wall-clock time
    let wall_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Monotonic time (Instant measures from program start)
    let monotonic_time = Instant::now().elapsed().as_secs_f64();

    // Performance counter (same as monotonic in Rust)
    let perf_time = Instant::now().elapsed().as_secs_f64();

    // CPU time (Instant in this context)
    let cpu_time = Instant::now().elapsed().as_secs_f64();

    let mut result = HashMap::new();
    result.insert("wall".to_string(), wall_time);
    result.insert("monotonic".to_string(), monotonic_time);
    result.insert("perf".to_string(), perf_time);
    result.insert("cpu".to_string(), cpu_time);

    result
}
```

## Precision Considerations

**Float precision:**
- Timestamps use `f64` (double precision)
- Precision degrades for very large timestamps
- After ~270 years, precision drops below 1 microsecond
- Use `perf_counter()` for relative timing (no epoch offset)

**Best Practices:**
- Always use `perf_counter()` for benchmarking
- Use `monotonic()` for timeouts and intervals
- Use `time()` only for absolute timestamps
- Store durations as differences, not absolute times

## Sleep Behavior

**Sleep precision:**
- Minimum sleep duration: OS-dependent (~1ms typical)
- `sleep(0)` yields to scheduler (may or may not sleep)
- Negative values raise `ValueError`
- Sleep may be interrupted by signals (Python signal handling)

**Example:**

```python
import time

def precise_sleep_loop(iterations: int, delay: float) -> float:
    start = time.perf_counter()

    for _ in range(iterations):
        time.sleep(delay)

    elapsed = time.perf_counter() - start
    return elapsed
```

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_time.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_time.py -v
```
