# decimal - Arbitrary Precision Decimal Arithmetic

Python's decimal module provides arbitrary-precision decimal arithmetic for financial and other applications requiring exact decimal representation. Depyler transpiles these to Rust's `rust_decimal` crate with full precision control.

## Python → Rust Mapping

| Python Module | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `from decimal import Decimal` | `use rust_decimal::Decimal` | Arbitrary precision |
| `Decimal("10.5")` | `Decimal::from_str("10.5")` | String construction |
| `getcontext().prec` | Precision methods | Context control |

## Basic Decimal Operations

### Decimal Creation and Arithmetic

Create decimals from strings for exact representation:

```python
from decimal import Decimal

def decimal_basic() -> Decimal:
    a: Decimal = Decimal("10.5")
    b: Decimal = Decimal("2.3")

    # Basic arithmetic
    sum_val = a + b        # → a + b
    diff_val = a - b       # → a - b
    prod_val = a * b       # → a * b
    quot_val = a / b       # → a / b

    return sum_val  # 12.8
```

**Generated Rust:**

```rust
use rust_decimal::Decimal;
use std::str::FromStr;

fn decimal_basic() -> Decimal {
    let a: Decimal = Decimal::from_str("10.5").unwrap();
    let b: Decimal = Decimal::from_str("2.3").unwrap();

    let sum_val = a + b;
    let diff_val = a - b;
    let prod_val = a * b;
    let quot_val = a / b;

    sum_val
}
```

## Precision Control

### getcontext() and Precision

Control decimal precision for calculations:

```python
from decimal import Decimal, getcontext

def decimal_precision() -> Decimal:
    # Set precision
    getcontext().prec = 10

    a: Decimal = Decimal("1.0")
    b: Decimal = Decimal("3.0")

    # Division with controlled precision
    result = a / b  # 0.3333333333

    return result
```

**Generated Rust:**

```rust
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::str::FromStr;

fn decimal_precision() -> Decimal {
    let a: Decimal = Decimal::from_str("1.0").unwrap();
    let b: Decimal = Decimal::from_str("3.0").unwrap();

    let result = a / b;
    // Precision handled by Decimal type

    result
}
```

## Comparison Operations

### Decimal Comparisons

Compare decimals with exact equality:

```python
from decimal import Decimal

def decimal_comparison() -> bool:
    a: Decimal = Decimal("10.5")
    b: Decimal = Decimal("10.50")
    c: Decimal = Decimal("10.51")

    # Comparisons
    equal = (a == b)        # True
    less = (a < c)          # True
    greater = (c > a)       # True

    return equal and less and greater
```

**Generated Rust:**

```rust
use rust_decimal::Decimal;
use std::str::FromStr;

fn decimal_comparison() -> bool {
    let a: Decimal = Decimal::from_str("10.5").unwrap();
    let b: Decimal = Decimal::from_str("10.50").unwrap();
    let c: Decimal = Decimal::from_str("10.51").unwrap();

    let equal = a == b;
    let less = a < c;
    let greater = c > a;

    equal && less && greater
}
```

## Rounding and Quantization

### quantize() - Control Decimal Places

Round decimals to a specific number of decimal places:

```python
from decimal import Decimal

def decimal_rounding() -> Decimal:
    a: Decimal = Decimal("10.567")

    # Quantize (round to 2 decimal places)
    rounded = a.quantize(Decimal("0.01"))  # 10.57

    return rounded
```

**Generated Rust:**

```rust
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::str::FromStr;

fn decimal_rounding() -> Decimal {
    let a: Decimal = Decimal::from_str("10.567").unwrap();

    // Round to 2 decimal places
    let rounded = a.round_dp(2);

    rounded
}
```

## String Conversion

### to/from String

Convert decimals to and from strings:

```python
from decimal import Decimal

def decimal_string_conversion() -> str:
    # Create from string
    a: Decimal = Decimal("123.456")

    # Convert to string
    s: str = str(a)  # "123.456"

    return s
```

**Generated Rust:**

```rust
use rust_decimal::Decimal;
use std::str::FromStr;

fn decimal_string_conversion() -> String {
    let a: Decimal = Decimal::from_str("123.456").unwrap();

    let s: String = a.to_string();

    s
}
```

## Complete Operation Coverage

All common decimal operations are supported:

| Python Operation | Rust Equivalent | Category |
|-----------------|-----------------|----------|
| `Decimal("10.5")` | `Decimal::from_str("10.5")` | Construction |
| `a + b` | `a + b` | Arithmetic |
| `a - b` | `a - b` | Arithmetic |
| `a * b` | `a * b` | Arithmetic |
| `a / b` | `a / b` | Arithmetic |
| `a == b` | `a == b` | Comparison |
| `a < b` | `a < b` | Comparison |
| `a.quantize(b)` | `a.round_dp(places)` | Rounding |
| `str(a)` | `a.to_string()` | Conversion |
| `getcontext().prec` | Precision control | Context |

## Precision Guarantees

**Depyler guarantees:**
- Exact decimal representation (no float rounding errors)
- Configurable precision
- Deterministic results
- No loss of precision in arithmetic

**Example: No Float Rounding Errors**

```python
from decimal import Decimal

def precision_demo() -> bool:
    # Float has rounding errors
    float_result = 0.1 + 0.2  # 0.30000000000000004

    # Decimal is exact
    a: Decimal = Decimal("0.1")
    b: Decimal = Decimal("0.2")
    decimal_result = a + b  # Exactly 0.3

    return decimal_result == Decimal("0.3")  # True
```

## Financial Calculations

Decimals are ideal for financial calculations:

```python
from decimal import Decimal

def financial_calc() -> Decimal:
    price: Decimal = Decimal("19.99")
    quantity: int = 100
    tax_rate: Decimal = Decimal("0.08")

    subtotal = price * Decimal(quantity)
    tax = subtotal * tax_rate
    total = subtotal + tax

    # Round to cents
    return total.quantize(Decimal("0.01"))
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| Construction | Software | Software | String parsing |
| Addition | O(n) | O(n) | n = digits |
| Multiplication | O(n²) | O(n²) | Arbitrary precision |
| Division | O(n) | O(n) | With rounding |
| Comparison | O(n) | O(n) | Digit-by-digit |

## Safety and Guarantees

**Decimal arithmetic safety:**
- No silent rounding errors
- Configurable precision
- Overflow detection
- Exact representation of decimal fractions

**Important Notes:**
- Decimals are slower than floats but exact
- Use Decimal for financial calculations
- Use f64 for scientific calculations
- String construction is preferred over float conversion

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_decimal.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_decimal.py -v
```
