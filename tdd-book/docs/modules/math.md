# math - Mathematical Functions

Python's math module provides access to mathematical functions. Depyler transpiles these to Rust's `f64` methods and constants, delivering high-performance numeric computations with IEEE 754 compliance.

## Python → Rust Mapping

| Python Module | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `import math` | `use std::f64::consts` | Constants |
| `math.function()` | `f64::method()` | Instance methods |

## Basic Functions

### sqrt(), abs(), ceil(), floor()

Common mathematical operations:

```python
import math

def math_basic() -> float:
    x: float = 16.7
    
    # Square root
    sqrt_val = math.sqrt(16.0)  # → f64::sqrt()
    
    # Absolute value
    abs_val = math.fabs(-5.5)  # → f64::abs()
    
    # Ceiling (round up)
    ceil_val = math.ceil(x)  # → f64::ceil()
    
    # Floor (round down)
    floor_val = math.floor(x)  # → f64::floor()
    
    return sqrt_val + abs_val  # 4.0 + 5.5 = 9.5
```

**Generated Rust:**

```rust
fn math_basic() -> f64 {
    let x: f64 = 16.7;
    
    let sqrt_val = 16.0_f64.sqrt();
    let abs_val = (-5.5_f64).abs();
    let ceil_val = x.ceil();
    let floor_val = x.floor();
    
    sqrt_val + abs_val
}
```

## Power and Exponential Functions

### pow(), exp(), log()

Exponentiation and logarithms:

```python
import math

def math_power() -> float:
    base: float = 2.0
    exponent: float = 3.0
    
    # Power function
    power_val = math.pow(base, exponent)  # → f64::powf()
    
    # Natural exponential (e^x)
    exp_val = math.exp(1.0)  # → f64::exp()
    
    # Natural logarithm
    log_val = math.log(10.0)  # → f64::ln()
    
    return power_val  # 8.0
```

**Generated Rust:**

```rust
fn math_power() -> f64 {
    let base: f64 = 2.0;
    let exponent: f64 = 3.0;
    
    let power_val = base.powf(exponent);
    let exp_val = 1.0_f64.exp();
    let log_val = 10.0_f64.ln();
    
    power_val
}
```

## Trigonometric Functions

### sin(), cos(), tan()

Standard trigonometric functions (angles in radians):

```python
import math

def math_trig() -> float:
    angle: float = math.pi / 4.0  # 45 degrees
    
    # Sine
    sin_val = math.sin(angle)  # → f64::sin()
    
    # Cosine
    cos_val = math.cos(angle)  # → f64::cos()
    
    # Tangent
    tan_val = math.tan(angle)  # → f64::tan()
    
    return sin_val  # ~0.707
```

**Generated Rust:**

```rust
use std::f64::consts::PI;

fn math_trig() -> f64 {
    let angle: f64 = PI / 4.0;
    
    let sin_val = angle.sin();
    let cos_val = angle.cos();
    let tan_val = angle.tan();
    
    sin_val
}
```

## Mathematical Constants

### pi, e, tau

Pre-defined mathematical constants:

```python
import math

def math_constants() -> float:
    # Pi (π ≈ 3.14159...)
    pi_val: float = math.pi  # → std::f64::consts::PI
    
    # Euler's number (e ≈ 2.71828...)
    e_val: float = math.e  # → std::f64::consts::E
    
    # Tau (τ = 2π ≈ 6.28318...)
    tau_val: float = math.tau  # → std::f64::consts::TAU
    
    return pi_val + e_val  # ~5.85987
```

**Generated Rust:**

```rust
use std::f64::consts::{PI, E, TAU};

fn math_constants() -> f64 {
    let pi_val: f64 = PI;
    let e_val: f64 = E;
    let tau_val: f64 = TAU;
    
    pi_val + e_val
}
```

## Rounding Functions

### ceil(), floor(), trunc()

Different rounding strategies:

```python
import math

def math_rounding() -> float:
    x: float = 3.7
    y: float = -2.3
    
    # Round up (ceiling)
    ceil_x = math.ceil(x)  # → f64::ceil() → 4.0
    
    # Round down (floor)
    floor_y = math.floor(y)  # → f64::floor() → -3.0
    
    # Truncate towards zero
    trunc_x = math.trunc(x)  # → f64::trunc() → 3.0
    
    return float(ceil_x)
```

**Generated Rust:**

```rust
fn math_rounding() -> f64 {
    let x: f64 = 3.7;
    let y: f64 = -2.3;
    
    let ceil_x = x.ceil();
    let floor_y = y.floor();
    let trunc_x = x.trunc();
    
    ceil_x
}
```

## Hyperbolic Functions

### sinh(), cosh(), tanh()

Hyperbolic trigonometric functions:

```python
import math

def math_hyperbolic() -> float:
    x: float = 1.0
    
    # Hyperbolic sine
    sinh_val = math.sinh(x)  # → f64::sinh()
    
    # Hyperbolic cosine
    cosh_val = math.cosh(x)  # → f64::cosh()
    
    # Hyperbolic tangent
    tanh_val = math.tanh(x)  # → f64::tanh()
    
    return sinh_val  # ~1.175
```

**Generated Rust:**

```rust
fn math_hyperbolic() -> f64 {
    let x: f64 = 1.0;
    
    let sinh_val = x.sinh();
    let cosh_val = x.cosh();
    let tanh_val = x.tanh();
    
    sinh_val
}
```

## Complete Function Coverage

All common math functions are supported:

| Python Function | Rust Equivalent | Category |
|----------------|-----------------|----------|
| `sqrt(x)` | `x.sqrt()` | Basic |
| `fabs(x)` | `x.abs()` | Basic |
| `ceil(x)` | `x.ceil()` | Rounding |
| `floor(x)` | `x.floor()` | Rounding |
| `trunc(x)` | `x.trunc()` | Rounding |
| `pow(x, y)` | `x.powf(y)` | Power |
| `exp(x)` | `x.exp()` | Exponential |
| `log(x)` | `x.ln()` | Logarithm |
| `log10(x)` | `x.log10()` | Logarithm |
| `sin(x)` | `x.sin()` | Trigonometric |
| `cos(x)` | `x.cos()` | Trigonometric |
| `tan(x)` | `x.tan()` | Trigonometric |
| `asin(x)` | `x.asin()` | Inverse Trig |
| `acos(x)` | `x.acos()` | Inverse Trig |
| `atan(x)` | `x.atan()` | Inverse Trig |
| `sinh(x)` | `x.sinh()` | Hyperbolic |
| `cosh(x)` | `x.cosh()` | Hyperbolic |
| `tanh(x)` | `x.tanh()` | Hyperbolic |
| `pi` | `PI` | Constant |
| `e` | `E` | Constant |
| `tau` | `TAU` | Constant |

## IEEE 754 Compliance

Rust's `f64` type provides full IEEE 754 double-precision floating-point arithmetic:

```python
import math

def ieee_754_example() -> float:
    # Special values
    inf = math.inf  # → f64::INFINITY
    nan = math.nan  # → f64::NAN
    
    # Checks
    is_finite = math.isfinite(3.14)  # → f64::is_finite()
    is_inf = math.isinf(inf)  # → f64::is_infinite()
    is_nan = math.isnan(nan)  # → f64::is_nan()
    
    return 3.14
```

**Generated Rust:**

```rust
fn ieee_754_example() -> f64 {
    let inf = f64::INFINITY;
    let nan = f64::NAN;
    
    let is_finite = 3.14_f64.is_finite();
    let is_inf = inf.is_infinite();
    let is_nan = nan.is_nan();
    
    3.14
}
```

## Performance Characteristics

Rust's math operations leverage CPU-native instructions:

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| Basic ops | Software | Hardware | LLVM optimization |
| Trigonometry | libm | libm/intrinsics | Platform-specific |
| Constants | Runtime | Compile-time | Zero-cost |
| Special values | Checked | Efficient | IEEE 754 native |

## Precision and Accuracy

**Depyler guarantees:**
- Same precision as Python (IEEE 754 double)
- Deterministic results across platforms
- NaN and infinity handling
- No implicit type conversions

**Example:**

```python
import math

def precision_demo() -> bool:
    x: float = 0.1 + 0.2
    # Both Python and Rust: 0.30000000000000004
    return math.isclose(x, 0.3)  # True (within tolerance)
```

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_math.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_math.py -v
```
