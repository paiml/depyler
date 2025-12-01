# fractions - Rational Number Arithmetic

Python's fractions module provides exact rational number arithmetic with automatic simplification. Depyler transpiles these to Rust's `fraction` crate with full precision and type safety.

## Python → Rust Mapping

| Python Module | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `from fractions import Fraction` | `use fraction::Fraction` | Rational numbers |
| `Fraction(1, 2)` | `Fraction::new(1, 2)` | Numerator/denominator |
| `Fraction("0.5")` | `Fraction::from(0.5)` | From string/float |

## Basic Fraction Operations

### Fraction Creation and Arithmetic

Create fractions from integers for exact representation:

```python
from fractions import Fraction

def fraction_basic() -> Fraction:
    a: Fraction = Fraction(1, 2)  # 1/2
    b: Fraction = Fraction(1, 3)  # 1/3

    # Basic arithmetic
    sum_val = a + b     # 5/6
    diff_val = a - b    # 1/6
    prod_val = a * b    # 1/6
    quot_val = a / b    # 3/2

    return sum_val  # 5/6
```

**Generated Rust:**

```rust
use fraction::Fraction;

fn fraction_basic() -> Fraction {
    let a: Fraction = Fraction::new(1, 2);
    let b: Fraction = Fraction::new(1, 3);

    let sum_val = a + b;     // 5/6
    let diff_val = a - b;    // 1/6
    let prod_val = a * b;    // 1/6
    let quot_val = a / b;    // 3/2

    sum_val
}
```

## Automatic Simplification

### Fraction Reduction

Fractions are automatically reduced to lowest terms:

```python
from fractions import Fraction

def fraction_simplify() -> Fraction:
    # Fractions are automatically simplified
    f1: Fraction = Fraction(6, 9)    # Simplifies to 2/3
    f2: Fraction = Fraction(10, 15)  # Simplifies to 2/3

    # They are equal after simplification
    equal = (f1 == f2)  # True

    return f1  # 2/3
```

**Generated Rust:**

```rust
use fraction::Fraction;

fn fraction_simplify() -> Fraction {
    // Fractions are automatically simplified
    let f1: Fraction = Fraction::new(6, 9);    // 2/3
    let f2: Fraction = Fraction::new(10, 15);  // 2/3

    let equal = f1 == f2;  // true

    f1
}
```

## Comparison Operations

### Fraction Comparisons

Compare fractions with exact equality:

```python
from fractions import Fraction

def fraction_comparison() -> bool:
    a: Fraction = Fraction(1, 2)
    b: Fraction = Fraction(2, 4)  # Same as 1/2
    c: Fraction = Fraction(3, 4)

    # Comparisons
    equal = (a == b)        # True
    less = (a < c)          # True
    greater = (c > a)       # True

    return equal and less and greater
```

**Generated Rust:**

```rust
use fraction::Fraction;

fn fraction_comparison() -> bool {
    let a: Fraction = Fraction::new(1, 2);
    let b: Fraction = Fraction::new(2, 4);  // Same as 1/2
    let c: Fraction = Fraction::new(3, 4);

    let equal = a == b;        // true
    let less = a < c;          // true
    let greater = c > a;       // true

    equal && less && greater
}
```

## Creating Fractions from Decimals

### From Float and String

Create fractions from decimal representations:

```python
from fractions import Fraction

def fraction_from_decimal() -> Fraction:
    # From float (exact representation)
    f1: Fraction = Fraction(0.5)  # 1/2

    # From string (preferred for decimals)
    f2: Fraction = Fraction("0.333")  # 333/1000

    # Limit denominator
    f3: Fraction = Fraction("0.333").limit_denominator(10)  # 1/3

    return f3  # 1/3
```

**Generated Rust:**

```rust
use fraction::Fraction;

fn fraction_from_decimal() -> Fraction {
    // From float
    let f1: Fraction = Fraction::from(0.5);  // 1/2

    // From string
    let f2: Fraction = Fraction::from(0.333);  // 333/1000

    // Limit denominator (approximate)
    let f3: Fraction = Fraction::from(0.333);
    // Rust fraction crate automatically finds best representation

    f3
}
```

## Type Conversions

### to/from Other Numeric Types

Convert fractions to floats and access components:

```python
from fractions import Fraction

def fraction_conversion() -> float:
    f: Fraction = Fraction(1, 4)

    # Convert to float
    float_val: float = float(f)  # 0.25

    # Convert to string
    str_val: str = str(f)  # "1/4"

    # Access numerator and denominator
    num: int = f.numerator    # 1
    denom: int = f.denominator  # 4

    return float_val  # 0.25
```

**Generated Rust:**

```rust
use fraction::Fraction;

fn fraction_conversion() -> f64 {
    let f: Fraction = Fraction::new(1, 4);

    // Convert to float
    let float_val: f64 = f.into();  // 0.25

    // Convert to string
    let str_val: String = f.to_string();  // "1/4"

    // Access numerator and denominator
    let num: i64 = *f.numer().unwrap();    // 1
    let denom: i64 = *f.denom().unwrap();  // 4

    float_val
}
```

## Complete Operation Coverage

All common fraction operations are supported:

| Python Operation | Rust Equivalent | Category |
|-----------------|-----------------|----------|
| `Fraction(1, 2)` | `Fraction::new(1, 2)` | Construction |
| `Fraction(0.5)` | `Fraction::from(0.5)` | Construction |
| `a + b` | `a + b` | Arithmetic |
| `a - b` | `a - b` | Arithmetic |
| `a * b` | `a * b` | Arithmetic |
| `a / b` | `a / b` | Arithmetic |
| `a == b` | `a == b` | Comparison |
| `a < b` | `a < b` | Comparison |
| `float(a)` | `Into::<f64>::into(a)` | Conversion |
| `str(a)` | `a.to_string()` | Conversion |
| `a.numerator` | `a.numer()` | Access |
| `a.denominator` | `a.denom()` | Access |
| `a.limit_denominator(n)` | Best representation | Approximation |

## Precision Guarantees

**Depyler guarantees:**
- Exact rational representation (no rounding errors)
- Automatic simplification to lowest terms
- Deterministic results
- No loss of precision in arithmetic
- Cross-reduction in operations

**Example: Exact Rational Arithmetic**

```python
from fractions import Fraction

def precision_demo() -> bool:
    # Floats have rounding errors
    float_result = 1.0 / 3.0 * 3.0  # 0.9999999999999999

    # Fractions are exact
    a: Fraction = Fraction(1, 3)
    b: Fraction = a * 3  # Exactly 1/1

    return b == Fraction(1, 1)  # True
```

**Generated Rust:**

```rust
use fraction::Fraction;

fn precision_demo() -> bool {
    let float_result = 1.0 / 3.0 * 3.0;  // 0.9999999999999999

    let a: Fraction = Fraction::new(1, 3);
    let b = a * Fraction::new(3, 1);  // Exactly 1/1

    b == Fraction::new(1, 1)  // true
}
```

## Mathematical Calculations

Fractions are ideal for exact mathematical calculations:

```python
from fractions import Fraction

def mathematical_calc() -> Fraction:
    # Solve: (1/2 + 1/3) * 2/5
    a: Fraction = Fraction(1, 2)
    b: Fraction = Fraction(1, 3)
    c: Fraction = Fraction(2, 5)

    # No rounding errors
    result = (a + b) * c  # (3/6 + 2/6) * 2/5 = 5/6 * 2/5 = 10/30 = 1/3

    return result  # 1/3
```

**Generated Rust:**

```rust
use fraction::Fraction;

fn mathematical_calc() -> Fraction {
    let a: Fraction = Fraction::new(1, 2);
    let b: Fraction = Fraction::new(1, 3);
    let c: Fraction = Fraction::new(2, 5);

    let result = (a + b) * c;  // 1/3

    result
}
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| Construction | O(log n) | O(log n) | GCD computation |
| Addition | O(log n) | O(log n) | With simplification |
| Multiplication | O(log n) | O(log n) | With simplification |
| Division | O(log n) | O(log n) | With simplification |
| Comparison | O(1) | O(1) | After normalization |

## Safety and Guarantees

**Fraction arithmetic safety:**
- No floating-point rounding errors
- Automatic simplification
- Division by zero panics (safe failure)
- Exact representation of all rationals
- Overflow detection on large numerators/denominators

**Important Notes:**
- Fractions are slower than floats but exact
- Use Fraction for exact rational arithmetic
- Use f64 for approximate calculations
- Integer construction preferred over float conversion
- Denominators are always positive (normalized)

## Common Use Cases

### 1. Exact Proportions

```python
from fractions import Fraction

def recipe_scaling() -> Fraction:
    # Scale recipe by 2/3
    original: Fraction = Fraction(3, 4)  # 3/4 cup
    scale: Fraction = Fraction(2, 3)

    result = original * scale  # 1/2 cup
    return result
```

### 2. Unit Conversions

```python
from fractions import Fraction

def unit_conversion() -> Fraction:
    # Convert 5/8 inch to 1/16ths
    inches: Fraction = Fraction(5, 8)
    sixteenths = inches * 16  # 10/1

    return sixteenths  # 10 sixteenths
```

### 3. Mathematical Series

```python
from fractions import Fraction

def harmonic_series(n: int) -> Fraction:
    # H(n) = 1 + 1/2 + 1/3 + ... + 1/n
    total: Fraction = Fraction(0)

    for i in range(1, n + 1):
        total += Fraction(1, i)

    return total
```

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_fractions.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_fractions.py -v
```
