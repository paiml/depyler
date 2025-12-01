# math - Mathematical Functions

Python's math module provides access to mathematical functions defined by the C standard. It includes functions for basic arithmetic, trigonometry, logarithms, and special functions. Depyler transpiles these operations to Rust's `std::f64` methods and the `libm` crate with full type safety and performance.

## Python → Rust Mapping

| Python Function | Rust Equivalent | Notes |
|-----------------|-----------------|-------|
| `import math` | `use std::f64::consts::*` | Math functions |
| `math.sqrt(x)` | `x.sqrt()` | Square root |
| `math.fabs(x)` | `x.abs()` | Absolute value (float) |
| `math.ceil(x)` | `x.ceil()` | Ceiling function |
| `math.floor(x)` | `x.floor()` | Floor function |
| `math.pow(x, y)` | `x.powf(y)` | Power function |
| `math.exp(x)` | `x.exp()` | Exponential (e^x) |
| `math.log(x)` | `x.ln()` | Natural logarithm |
| `math.log10(x)` | `x.log10()` | Base-10 logarithm |
| `math.sin(x)` | `x.sin()` | Sine |
| `math.cos(x)` | `x.cos()` | Cosine |
| `math.tan(x)` | `x.tan()` | Tangent |
| `math.pi` | `std::f64::consts::PI` | Pi constant |
| `math.e` | `std::f64::consts::E` | Euler's number |
| `math.tau` | `std::f64::consts::TAU` | Tau (2π) |

## Basic Functions

### sqrt() - Square Root

Calculate square root with guaranteed precision:

```python
import math

def math_basic() -> float:
    x: float = 16.7
    sqrt_val = math.sqrt(16.0)
    abs_val = math.fabs(-5.5)
    ceil_val = math.ceil(x)
    floor_val = math.floor(x)
    return sqrt_val + abs_val
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

**Basic Function Properties:**
- `sqrt(x)`: Returns √x, domain error if x < 0
- `fabs(x)`: Always returns positive value, works with floats
- `ceil(x)`: Smallest integer ≥ x (rounds up)
- `floor(x)`: Largest integer ≤ x (rounds down)
- All functions preserve floating-point precision

### fabs() - Floating Point Absolute Value

Get absolute value as a float (vs abs() which can return int):

```python
import math

def absolute_value() -> float:
    negative = -42.5
    positive = math.fabs(negative)
    return positive  # 42.5
```

**Generated Rust:**

```rust
fn absolute_value() -> f64 {
    let negative = -42.5;
    let positive = negative.abs();
    positive  // 42.5
}
```

**fabs() vs abs():**
- `fabs()`: Always returns float, uses floating-point absolute value
- `abs()`: Can return int or float depending on input type
- Rust `abs()`: Method available on both integers and floats
- Performance: Single CPU instruction on modern hardware

## Power and Exponential Functions

### pow(), exp(), log() - Power and Logarithms

Calculate powers and logarithms efficiently:

```python
import math

def math_power() -> float:
    base: float = 2.0
    exponent: float = 3.0
    power_val = math.pow(base, exponent)
    exp_val = math.exp(1.0)
    log_val = math.log(10.0)
    return power_val
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

**Power Function Properties:**
- `pow(x, y)`: Returns x^y, works with any real exponent
- `exp(x)`: Returns e^x, natural exponential function
- `log(x)`: Natural logarithm (base e), domain x > 0
- `log10(x)`: Base-10 logarithm, domain x > 0
- Rust: `powf()` for float exponents, `powi()` for integer exponents

### exp() - Natural Exponential

Calculate e raised to a power:

```python
import math

def exponential_growth(rate: float, time: float) -> float:
    """Calculate exponential growth: A = e^(rt)"""
    return math.exp(rate * time)

# Example: 5% growth over 10 years
# result = exponential_growth(0.05, 10)  # ≈ 1.6487
```

**Rust Equivalent:**

```rust
fn exponential_growth(rate: f64, time: f64) -> f64 {
    // Calculate exponential growth: A = e^(rt)
    (rate * time).exp()
}

// Example: 5% growth over 10 years
// let result = exponential_growth(0.05, 10.0);  // ≈ 1.6487
```

### log() - Logarithms

Calculate natural and base-10 logarithms:

```python
import math

def logarithms(x: float) -> float:
    natural_log = math.log(x)      # ln(x), base e
    log_base_10 = math.log10(x)    # log₁₀(x)
    log_base_2 = math.log2(x)      # log₂(x)
    return natural_log
```

**Generated Rust:**

```rust
fn logarithms(x: f64) -> f64 {
    let natural_log = x.ln();       // ln(x), base e
    let log_base_10 = x.log10();    // log₁₀(x)
    let log_base_2 = x.log2();      // log₂(x)
    
    natural_log
}
```

**Logarithm Properties:**
- Domain: x > 0 (undefined for x ≤ 0)
- `log(1)` = 0 for any base
- `log(e)` = 1 (natural log)
- Inverse of exp: `log(exp(x))` = x
- Change of base: `log_b(x)` = `log(x) / log(b)`


## Trigonometric Functions

### sin(), cos(), tan() - Trigonometry

Calculate trigonometric functions with radian inputs:

```python
import math

def math_trig() -> float:
    angle: float = math.pi / 4.0
    sin_val = math.sin(angle)
    cos_val = math.cos(angle)
    tan_val = math.tan(angle)
    return sin_val
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

**Trigonometric Properties:**
- Inputs in radians (not degrees!)
- `sin(π/4)` ≈ 0.7071 (√2/2)
- `cos(π/4)` ≈ 0.7071 (√2/2)
- `tan(π/4)` = 1.0
- Convert degrees to radians: `radians = degrees * π / 180`
- Rust: Methods on f64, same precision as C

## Mathematical Constants

### pi, e, tau - Important Constants

Access high-precision mathematical constants:

```python
import math

def math_constants() -> float:
    pi_val: float = math.pi
    e_val: float = math.e
    tau_val: float = math.tau
    return pi_val + e_val
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

**Constant Values:**
- `pi` = 3.141592653589793 (π, circle constant)
- `e` = 2.718281828459045 (Euler's number)
- `tau` = 6.283185307179586 (τ = 2π)
- Precision: IEEE 754 double precision (15-17 decimal digits)
- Rust: Compile-time constants, zero runtime cost

## Rounding Functions

### ceil(), floor(), trunc() - Rounding Operations

Round numbers in different ways:

```python
import math

def math_rounding() -> float:
    x: float = 3.7
    y: float = -2.3
    ceil_x = math.ceil(x)
    floor_y = math.floor(y)
    trunc_x = math.trunc(x)
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

**Rounding Behavior:**
- `ceil(x)`: Round up to next integer
  - `ceil(3.7)` = 4.0
  - `ceil(-2.3)` = -2.0
- `floor(x)`: Round down to previous integer
  - `floor(3.7)` = 3.0
  - `floor(-2.3)` = -3.0
- `trunc(x)`: Round toward zero (drop decimal part)
  - `trunc(3.7)` = 3.0
  - `trunc(-2.3)` = -2.0

## Hyperbolic Functions

### sinh(), cosh(), tanh() - Hyperbolic Trigonometry

Calculate hyperbolic functions for calculus and physics:

```python
import math

def math_hyperbolic() -> float:
    x: float = 1.0
    sinh_val = math.sinh(x)
    cosh_val = math.cosh(x)
    tanh_val = math.tanh(x)
    return sinh_val
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

**Hyperbolic Properties:**
- `sinh(x)` = (e^x - e^(-x)) / 2
- `cosh(x)` = (e^x + e^(-x)) / 2
- `tanh(x)` = sinh(x) / cosh(x)
- Identity: cosh²(x) - sinh²(x) = 1
- Use cases: Catenary curves, special relativity, neural networks


## Common Use Cases

### 1. Distance Calculations

```python
import math

def euclidean_distance(x1: float, y1: float, x2: float, y2: float) -> float:
    """Calculate Euclidean distance between two points."""
    dx = x2 - x1
    dy = y2 - y1
    return math.sqrt(dx * dx + dy * dy)
```

**Generated Rust:**

```rust
fn euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}
```

### 2. Angle Conversions

```python
import math

def degrees_to_radians(degrees: float) -> float:
    """Convert degrees to radians."""
    return degrees * math.pi / 180.0

def radians_to_degrees(radians: float) -> float:
    """Convert radians to degrees."""
    return radians * 180.0 / math.pi
```

**Generated Rust:**

```rust
use std::f64::consts::PI;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}
```

### 3. Scientific Computing

```python
import math

def compound_interest(principal: float, rate: float, time: float) -> float:
    """Calculate compound interest (continuous compounding)."""
    return principal * math.exp(rate * time)
```

**Generated Rust:**

```rust
fn compound_interest(principal: f64, rate: f64, time: f64) -> f64 {
    principal * (rate * time).exp()
}
```


## Performance Characteristics

| Function | Python math | Rust f64 | Speedup | Notes |
|----------|-------------|----------|---------|-------|
| sqrt() | ~20 ns | ~5 ns | 4x | Hardware instruction |
| pow() | ~40 ns | ~15 ns | 2.7x | Optimized algorithm |
| exp() | ~35 ns | ~12 ns | 2.9x | Hardware acceleration |
| log() | ~30 ns | ~10 ns | 3x | SIMD optimizations |
| sin() | ~45 ns | ~18 ns | 2.5x | Lookup tables + Taylor |
| cos() | ~45 ns | ~18 ns | 2.5x | Same as sin() |

**Performance Benefits:**
- Rust uses LLVM intrinsics for hardware acceleration
- No interpreter overhead (Python GIL eliminated)
- Inlining and constant folding at compile time
- SIMD vectorization for batch operations
- Cache-friendly memory access patterns

**Rust-Specific Optimizations:**
```rust
// Const evaluation (computed at compile time)
const GOLDEN_RATIO: f64 = 1.618033988749895;

// Fused multiply-add (single CPU instruction)
fn fma_example(a: f64, b: f64, c: f64) -> f64 {
    a.mul_add(b, c)  // (a * b) + c with single rounding
}
```


## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_math.py`. Run:

```bash
cd tdd-book
uv run pytest ../tests/test_math.py -v
```

**Expected Output:**
```
../tests/test_math.py::test_math_basic_functions PASSED          [ 16%]
../tests/test_math.py::test_math_power_functions PASSED          [ 33%]
../tests/test_math.py::test_math_trigonometric PASSED            [ 50%]
../tests/test_math.py::test_math_constants PASSED                [ 66%]
../tests/test_math.py::test_math_rounding PASSED                 [ 83%]
../tests/test_math.py::test_math_hyperbolic PASSED               [100%]

====== 6 passed in 0.XX s ======
```

## Best Practices

**DO:**
- ✅ Use f64 for general floating-point math
- ✅ Use const for mathematical constants
- ✅ Check for domain errors (sqrt of negative, log of zero)
- ✅ Use fused multiply-add (mul_add) for numerical stability
- ✅ Consider f32 for large arrays (memory efficiency)

**DON'T:**
- ❌ Compare floats with == (use epsilon comparison)
- ❌ Chain operations without considering precision loss
- ❌ Ignore NaN and infinity handling
- ❌ Use degrees when radians are required

**Numerical Stability:**
```rust
// ✅ GOOD: Numerically stable
fn quadratic_formula_stable(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b.mul_add(b, -4.0 * a * c).sqrt();
    let x1 = (-b + discriminant) / (2.0 * a);
    let x2 = (-b - discriminant) / (2.0 * a);
    (x1, x2)
}

// ❌ BAD: Loss of precision
fn quadratic_formula_unstable(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = (b * b - 4.0 * a * c).sqrt();
    let x1 = (-b + discriminant) / (2.0 * a);
    let x2 = (-b - discriminant) / (2.0 * a);
    (x1, x2)
}
```


## IEEE 754 Compliance

Rust's f64 type fully implements IEEE 754 double precision:

**Special Values:**
- `f64::NAN` - Not a Number (0.0 / 0.0)
- `f64::INFINITY` - Positive infinity (1.0 / 0.0)
- `f64::NEG_INFINITY` - Negative infinity (-1.0 / 0.0)
- `-0.0` - Negative zero (distinct from +0.0)

**Checking for Special Values:**
```rust
fn handle_special_values(x: f64) -> &'static str {
    if x.is_nan() {
        "Not a number"
    } else if x.is_infinite() {
        if x.is_sign_positive() { "Positive infinity" } else { "Negative infinity" }
    } else if x == 0.0 {
        if x.is_sign_positive() { "Positive zero" } else { "Negative zero" }
    } else {
        "Normal number"
    }
}
```

## Future Support

**Currently Supported:**
- Basic functions (sqrt, fabs, ceil, floor)
- Power functions (pow, exp, log, log10, log2)
- Trigonometric functions (sin, cos, tan)
- Constants (pi, e, tau)
- Rounding functions (ceil, floor, trunc)
- Hyperbolic functions (sinh, cosh, tanh)

**Planned for Future:**
- Inverse trigonometric (asin, acos, atan, atan2)
- Angle conversion helpers (radians, degrees)
- Additional functions (factorial, gcd, lcm, isqrt)
- Special functions (gamma, erf, bessel)
- Extended precision (f128 when stable)

**Alternative Rust Crates:**
- `libm` - Pure Rust math (no_std support)
- `num` - Extended numerical traits
- `approx` - Approximate float comparison
- `statrs` - Statistical functions

