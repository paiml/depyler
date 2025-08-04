# Function Inlining Heuristics in Depyler

## Overview

Depyler v2.0.0 implements sophisticated function inlining heuristics to optimize
generated Rust code by replacing function calls with their bodies when
beneficial. This reduces function call overhead and enables further
optimizations.

## Inlining Decisions

The inlining analyzer makes decisions based on multiple factors:

### 1. Function Characteristics

#### Trivial Functions

Functions with a single return statement are always inlined:

```python
def square(x: int) -> int:
    return x * x  # Trivial - will be inlined
```

#### Single-Use Functions

Functions called only once are inlined to eliminate overhead:

```python
def process_once(x: int) -> int:
    temp = x * 2
    return temp + 10

result = process_once(5)  # Only call - will be inlined
```

#### Size Constraints

Functions exceeding the size threshold (default: 20 HIR nodes) are not inlined:

```python
def large_function(x, y, z):
    # Many operations...
    # Size > 20 - won't be inlined
```

### 2. Cost-Benefit Analysis

The analyzer calculates a cost-benefit ratio considering:

- **Call frequency**: More calls = higher benefit
- **Function size**: Larger functions = higher cost
- **Loops**: 10x cost multiplier
- **Side effects**: 2x cost multiplier
- **Multiple returns**: Additional complexity cost

### 3. Safety Constraints

Functions are NOT inlined if they:

- Are recursive (prevents infinite expansion)
- Have side effects (unless pure)
- Contain loops (configurable)
- Would exceed maximum inline depth

## Inlining Process

### 1. Call Graph Analysis

Build a complete call graph to:

- Detect recursive functions
- Track call frequencies
- Identify dependencies

### 2. Metrics Collection

For each function, calculate:

- Size in HIR nodes
- Parameter count
- Return statement count
- Loop presence
- Side effect analysis

### 3. Decision Making

Apply heuristics to determine:

- Should inline (yes/no)
- Reason (trivial, single-use, cost-effective)
- Cost-benefit ratio

### 4. Transformation

When inlining:

- Replace parameters with arguments
- Rename local variables to avoid conflicts
- Convert returns to assignments
- Recursively inline nested calls

## Configuration

```rust
pub struct InliningConfig {
    pub max_inline_size: usize,      // Default: 20
    pub max_inline_depth: usize,     // Default: 3
    pub inline_single_use: bool,     // Default: true
    pub inline_trivial: bool,        // Default: true
    pub cost_threshold: f64,         // Default: 1.5
    pub inline_loops: bool,          // Default: false
}
```

## Example Results

Input Python:

```python
def add_one(n: int) -> int:
    return n + 1

def compute(x: int) -> int:
    step1 = add_one(x)
    step2 = add_one(step1)
    return step2
```

Output Rust (with inlining):

```rust
pub fn compute(x: i32) -> i32 {
    let _inline_n = x;
    let step1 = _inline_n + 1;
    let _inline_n = step1;
    let step2 = _inline_n + 1;
    return step2;
}
```

## Benefits

1. **Performance**: Eliminates function call overhead
2. **Optimization**: Enables further constant propagation and dead code
   elimination
3. **Code Size**: Can reduce size by eliminating single-use functions
4. **Cache Efficiency**: Better instruction cache utilization

## Limitations

1. **Code Bloat**: Aggressive inlining can increase binary size
2. **Compilation Time**: More code to compile per function
3. **Debugging**: Inlined code is harder to debug
4. **Recursion**: Cannot inline recursive functions

## Best Practices

1. Mark pure functions appropriately for better inlining decisions
2. Keep utility functions small and focused
3. Use configuration to balance performance vs code size
4. Monitor binary size when enabling aggressive inlining
5. Consider profiling to identify hot functions for inlining

## Future Enhancements

- Profile-guided inlining based on runtime data
- Partial inlining for functions with early returns
- Cross-module inlining optimization
- Machine learning-based cost models
- Incremental inlining with feedback
