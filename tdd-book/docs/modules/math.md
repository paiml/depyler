# math

## math constants - Mathematical constants.

## Basic mathematical functions.

## Trigonometric functions.

## Hyperbolic functions.

## Angular conversion functions.

## Rounding and truncation functions.

## Integer mathematical functions.

## Floating point classification.

## Special mathematical functions.

## Edge cases and special scenarios.

### Basic: pi constant value.

```python
def test_pi_constant(self):
    """Basic: pi constant value."""
    assert abs(math.pi - 3.14159265359) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: e constant value.

```python
def test_e_constant(self):
    """Basic: e constant value."""
    assert abs(math.e - 2.71828182846) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: tau constant (2*pi).

```python
def test_tau_constant(self):
    """Basic: tau constant (2*pi)."""
    assert abs(math.tau - 6.28318530718) < 1e-10
    assert abs(math.tau - 2 * math.pi) < 1e-15
```

**Verification**: ✅ Tested in CI

### Basic: Infinity constant.

```python
def test_inf_constant(self):
    """Basic: Infinity constant."""
    assert math.isinf(math.inf)
    assert math.inf > 0
```

**Verification**: ✅ Tested in CI

### Basic: NaN constant.

```python
def test_nan_constant(self):
    """Basic: NaN constant."""
    assert math.isnan(math.nan)
```

**Verification**: ✅ Tested in CI

### Basic: Square root.

```python
def test_sqrt(self):
    """Basic: Square root."""
    assert math.sqrt(4) == 2.0
    assert math.sqrt(9) == 3.0
```

**Verification**: ✅ Tested in CI

### Feature: Square root of float.

```python
def test_sqrt_float(self):
    """Feature: Square root of float."""
    assert abs(math.sqrt(2) - 1.41421356237) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Power function.

```python
def test_pow(self):
    """Basic: Power function."""
    assert math.pow(2, 3) == 8.0
    assert math.pow(5, 2) == 25.0
```

**Verification**: ✅ Tested in CI

### Feature: Power with float exponent.

```python
def test_pow_float_exponent(self):
    """Feature: Power with float exponent."""
    assert abs(math.pow(4, 0.5) - 2.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Exponential (e^x).

```python
def test_exp(self):
    """Basic: Exponential (e^x)."""
    assert abs(math.exp(0) - 1.0) < 1e-10
    assert abs(math.exp(1) - math.e) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Natural logarithm.

```python
def test_log(self):
    """Basic: Natural logarithm."""
    assert abs(math.log(math.e) - 1.0) < 1e-10
    assert abs(math.log(1) - 0.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Base-10 logarithm.

```python
def test_log10(self):
    """Basic: Base-10 logarithm."""
    assert abs(math.log10(10) - 1.0) < 1e-10
    assert abs(math.log10(100) - 2.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Base-2 logarithm.

```python
def test_log2(self):
    """Basic: Base-2 logarithm."""
    assert abs(math.log2(2) - 1.0) < 1e-10
    assert abs(math.log2(8) - 3.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Feature: Logarithm with custom base.

```python
def test_log_with_base(self):
    """Feature: Logarithm with custom base."""
    assert abs(math.log(8, 2) - 3.0) < 1e-10
    assert abs(math.log(27, 3) - 3.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Error: Square root of negative number.

```python
def test_error_sqrt_negative(self):
    """Error: Square root of negative number."""
    with pytest.raises(ValueError):
        math.sqrt(-1)
```

**Verification**: ✅ Tested in CI

### Error: Logarithm of zero.

```python
def test_error_log_zero(self):
    """Error: Logarithm of zero."""
    with pytest.raises(ValueError):
        math.log(0)
```

**Verification**: ✅ Tested in CI

### Error: Logarithm of negative number.

```python
def test_error_log_negative(self):
    """Error: Logarithm of negative number."""
    with pytest.raises(ValueError):
        math.log(-1)
```

**Verification**: ✅ Tested in CI

### Basic: Sine function.

```python
def test_sin(self):
    """Basic: Sine function."""
    assert abs(math.sin(0) - 0.0) < 1e-10
    assert abs(math.sin(math.pi / 2) - 1.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Cosine function.

```python
def test_cos(self):
    """Basic: Cosine function."""
    assert abs(math.cos(0) - 1.0) < 1e-10
    assert abs(math.cos(math.pi) - -1.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Tangent function.

```python
def test_tan(self):
    """Basic: Tangent function."""
    assert abs(math.tan(0) - 0.0) < 1e-10
    assert abs(math.tan(math.pi / 4) - 1.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Arcsine function.

```python
def test_asin(self):
    """Basic: Arcsine function."""
    assert abs(math.asin(0) - 0.0) < 1e-10
    assert abs(math.asin(1) - math.pi / 2) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Arccosine function.

```python
def test_acos(self):
    """Basic: Arccosine function."""
    assert abs(math.acos(1) - 0.0) < 1e-10
    assert abs(math.acos(0) - math.pi / 2) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Arctangent function.

```python
def test_atan(self):
    """Basic: Arctangent function."""
    assert abs(math.atan(0) - 0.0) < 1e-10
    assert abs(math.atan(1) - math.pi / 4) < 1e-10
```

**Verification**: ✅ Tested in CI

### Feature: Two-argument arctangent.

```python
def test_atan2(self):
    """Feature: Two-argument arctangent."""
    assert abs(math.atan2(1, 1) - math.pi / 4) < 1e-10
    assert abs(math.atan2(1, 0) - math.pi / 2) < 1e-10
```

**Verification**: ✅ Tested in CI

### Error: Arcsine out of range.

```python
def test_error_asin_out_of_range(self):
    """Error: Arcsine out of range."""
    with pytest.raises(ValueError):
        math.asin(2)
```

**Verification**: ✅ Tested in CI

### Error: Arccosine out of range.

```python
def test_error_acos_out_of_range(self):
    """Error: Arccosine out of range."""
    with pytest.raises(ValueError):
        math.acos(-2)
```

**Verification**: ✅ Tested in CI

### Basic: Hyperbolic sine.

```python
def test_sinh(self):
    """Basic: Hyperbolic sine."""
    assert abs(math.sinh(0) - 0.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Hyperbolic cosine.

```python
def test_cosh(self):
    """Basic: Hyperbolic cosine."""
    assert abs(math.cosh(0) - 1.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Hyperbolic tangent.

```python
def test_tanh(self):
    """Basic: Hyperbolic tangent."""
    assert abs(math.tanh(0) - 0.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Inverse hyperbolic sine.

```python
def test_asinh(self):
    """Basic: Inverse hyperbolic sine."""
    assert abs(math.asinh(0) - 0.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Inverse hyperbolic cosine.

```python
def test_acosh(self):
    """Basic: Inverse hyperbolic cosine."""
    assert abs(math.acosh(1) - 0.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Inverse hyperbolic tangent.

```python
def test_atanh(self):
    """Basic: Inverse hyperbolic tangent."""
    assert abs(math.atanh(0) - 0.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Error: acosh requires input >= 1.

```python
def test_error_acosh_out_of_range(self):
    """Error: acosh requires input >= 1."""
    with pytest.raises(ValueError):
        math.acosh(0.5)
```

**Verification**: ✅ Tested in CI

### Error: atanh requires -1 < x < 1.

```python
def test_error_atanh_out_of_range(self):
    """Error: atanh requires -1 < x < 1."""
    with pytest.raises(ValueError):
        math.atanh(1)
```

**Verification**: ✅ Tested in CI

### Basic: Radians to degrees.

```python
def test_degrees(self):
    """Basic: Radians to degrees."""
    assert abs(math.degrees(math.pi) - 180.0) < 1e-10
    assert abs(math.degrees(math.pi / 2) - 90.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Degrees to radians.

```python
def test_radians(self):
    """Basic: Degrees to radians."""
    assert abs(math.radians(180) - math.pi) < 1e-10
    assert abs(math.radians(90) - math.pi / 2) < 1e-10
```

**Verification**: ✅ Tested in CI

### Property: Degrees/radians roundtrip.

```python
def test_roundtrip_conversion(self):
    """Property: Degrees/radians roundtrip."""
    angle = 45.0
    assert abs(math.degrees(math.radians(angle)) - angle) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Ceiling function.

```python
def test_ceil(self):
    """Basic: Ceiling function."""
    assert math.ceil(2.3) == 3
    assert math.ceil(2.0) == 2
    assert math.ceil(-2.3) == -2
```

**Verification**: ✅ Tested in CI

### Basic: Floor function.

```python
def test_floor(self):
    """Basic: Floor function."""
    assert math.floor(2.7) == 2
    assert math.floor(2.0) == 2
    assert math.floor(-2.3) == -3
```

**Verification**: ✅ Tested in CI

### Basic: Truncation toward zero.

```python
def test_trunc(self):
    """Basic: Truncation toward zero."""
    assert math.trunc(2.7) == 2
    assert math.trunc(-2.7) == -2
    assert math.trunc(2.0) == 2
```

**Verification**: ✅ Tested in CI

### Edge: Ceil/floor differ on negatives.

```python
def test_ceil_vs_floor_negative(self):
    """Edge: Ceil/floor differ on negatives."""
    x = -2.5
    assert math.ceil(x) == -2
    assert math.floor(x) == -3
```

**Verification**: ✅ Tested in CI

### Basic: Factorial function.

```python
def test_factorial(self):
    """Basic: Factorial function."""
    assert math.factorial(0) == 1
    assert math.factorial(1) == 1
    assert math.factorial(5) == 120
```

**Verification**: ✅ Tested in CI

### Basic: Greatest common divisor.

```python
def test_gcd(self):
    """Basic: Greatest common divisor."""
    assert math.gcd(12, 8) == 4
    assert math.gcd(17, 5) == 1
```

**Verification**: ✅ Tested in CI

### Edge: GCD with zero.

```python
def test_gcd_zero(self):
    """Edge: GCD with zero."""
    assert math.gcd(0, 5) == 5
    assert math.gcd(10, 0) == 10
```

**Verification**: ✅ Tested in CI

### Basic: Integer square root.

```python
def test_isqrt(self):
    """Basic: Integer square root."""
    assert math.isqrt(4) == 2
    assert math.isqrt(9) == 3
    assert math.isqrt(10) == 3
```

**Verification**: ✅ Tested in CI

### Error: Factorial of negative number.

```python
def test_error_factorial_negative(self):
    """Error: Factorial of negative number."""
    with pytest.raises(ValueError):
        math.factorial(-1)
```

**Verification**: ✅ Tested in CI

### Error: isqrt of negative number.

```python
def test_error_isqrt_negative(self):
    """Error: isqrt of negative number."""
    with pytest.raises(ValueError):
        math.isqrt(-1)
```

**Verification**: ✅ Tested in CI

### Basic: Check if finite.

```python
def test_isfinite(self):
    """Basic: Check if finite."""
    assert math.isfinite(1.0) is True
    assert math.isfinite(math.inf) is False
    assert math.isfinite(math.nan) is False
```

**Verification**: ✅ Tested in CI

### Basic: Check if infinite.

```python
def test_isinf(self):
    """Basic: Check if infinite."""
    assert math.isinf(math.inf) is True
    assert math.isinf(-math.inf) is True
    assert math.isinf(1.0) is False
```

**Verification**: ✅ Tested in CI

### Basic: Check if NaN.

```python
def test_isnan(self):
    """Basic: Check if NaN."""
    assert math.isnan(math.nan) is True
    assert math.isnan(1.0) is False
    assert math.isnan(math.inf) is False
```

**Verification**: ✅ Tested in CI

### Basic: Check if close.

```python
def test_isclose(self):
    """Basic: Check if close."""
    assert math.isclose(1.0, 1.00000000001)
    assert not math.isclose(1.0, 1.1)
```

**Verification**: ✅ Tested in CI

### Feature: isclose with relative tolerance.

```python
def test_isclose_rel_tol(self):
    """Feature: isclose with relative tolerance."""
    assert math.isclose(100, 101, rel_tol=0.02)
    assert not math.isclose(100, 101, rel_tol=0.005)
```

**Verification**: ✅ Tested in CI

### Feature: isclose with absolute tolerance.

```python
def test_isclose_abs_tol(self):
    """Feature: isclose with absolute tolerance."""
    assert math.isclose(0, 1e-05, abs_tol=0.0001)
    assert not math.isclose(0, 0.01, abs_tol=0.001)
```

**Verification**: ✅ Tested in CI

### Property: NaN is not equal to itself.

```python
def test_nan_not_equal_self(self):
    """Property: NaN is not equal to itself."""
    assert math.nan != math.nan
    assert not math.nan == math.nan
```

**Verification**: ✅ Tested in CI

### Basic: Floating absolute value.

```python
def test_fabs(self):
    """Basic: Floating absolute value."""
    assert math.fabs(-5.5) == 5.5
    assert math.fabs(3.2) == 3.2
```

**Verification**: ✅ Tested in CI

### Basic: Copy sign of a number.

```python
def test_copysign(self):
    """Basic: Copy sign of a number."""
    assert math.copysign(5, -1) == -5.0
    assert math.copysign(-5, 1) == 5.0
```

**Verification**: ✅ Tested in CI

### Basic: Floating point modulo.

```python
def test_fmod(self):
    """Basic: Floating point modulo."""
    assert abs(math.fmod(5.5, 2) - 1.5) < 1e-10
    assert abs(math.fmod(-5.5, 2) - -1.5) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: IEEE remainder.

```python
def test_remainder(self):
    """Basic: IEEE remainder."""
    assert abs(math.remainder(5, 2) - 1.0) < 1e-10
    assert abs(math.remainder(7, 3) - 1.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Accurate floating point sum.

```python
def test_fsum(self):
    """Basic: Accurate floating point sum."""
    values = [0.1] * 10
    result = math.fsum(values)
    assert abs(result - 1.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Split into fractional and integer parts.

```python
def test_modf(self):
    """Basic: Split into fractional and integer parts."""
    frac, integer = math.modf(3.75)
    assert abs(frac - 0.75) < 1e-10
    assert integer == 3.0
```

**Verification**: ✅ Tested in CI

### Basic: Split into mantissa and exponent.

```python
def test_frexp(self):
    """Basic: Split into mantissa and exponent."""
    mantissa, exponent = math.frexp(8.0)
    assert abs(mantissa - 0.5) < 1e-10
    assert exponent == 4
    assert abs(mantissa * 2 ** exponent - 8.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Compute mantissa * 2^exponent.

```python
def test_ldexp(self):
    """Basic: Compute mantissa * 2^exponent."""
    assert math.ldexp(0.5, 4) == 8.0
    assert math.ldexp(1.0, 3) == 8.0
```

**Verification**: ✅ Tested in CI

### Property: frexp/ldexp roundtrip.

```python
def test_frexp_ldexp_roundtrip(self):
    """Property: frexp/ldexp roundtrip."""
    x = 123.456
    m, e = math.frexp(x)
    assert abs(math.ldexp(m, e) - x) < 1e-10
```

**Verification**: ✅ Tested in CI

### Edge: Square root of zero.

```python
def test_sqrt_zero(self):
    """Edge: Square root of zero."""
    assert math.sqrt(0) == 0.0
```

**Verification**: ✅ Tested in CI

### Edge: Any number to power 0 is 1.

```python
def test_pow_zero_exponent(self):
    """Edge: Any number to power 0 is 1."""
    assert math.pow(5, 0) == 1.0
    assert math.pow(0, 0) == 1.0
```

**Verification**: ✅ Tested in CI

### Edge: Any number to power 1 is itself.

```python
def test_pow_one_exponent(self):
    """Edge: Any number to power 1 is itself."""
    assert math.pow(7, 1) == 7.0
```

**Verification**: ✅ Tested in CI

### Edge: Negative base with integer exponent.

```python
def test_pow_negative_base(self):
    """Edge: Negative base with integer exponent."""
    assert math.pow(-2, 3) == -8.0
    assert math.pow(-2, 2) == 4.0
```

**Verification**: ✅ Tested in CI

### Edge: Log of 1 is 0.

```python
def test_log_one(self):
    """Edge: Log of 1 is 0."""
    assert abs(math.log(1) - 0.0) < 1e-10
    assert abs(math.log10(1) - 0.0) < 1e-10
```

**Verification**: ✅ Tested in CI

### Performance: Factorial of large number.

```python
def test_factorial_large(self):
    """Performance: Factorial of large number."""
    result = math.factorial(20)
    assert result == 2432902008176640000
```

**Verification**: ✅ Tested in CI

### Edge: Infinity arithmetic.

```python
def test_inf_arithmetic(self):
    """Edge: Infinity arithmetic."""
    assert math.inf + 1 == math.inf
    assert math.inf * 2 == math.inf
    assert -math.inf < 0
```

**Verification**: ✅ Tested in CI

### Edge: NaN arithmetic.

```python
def test_nan_arithmetic(self):
    """Edge: NaN arithmetic."""
    assert math.isnan(math.nan + 1)
    assert math.isnan(math.nan * 2)
```

**Verification**: ✅ Tested in CI

### Basic: Euclidean distance.

```python
def test_hypot(self):
    """Basic: Euclidean distance."""
    assert math.hypot(3, 4) == 5.0
    assert abs(math.hypot(1, 1) - math.sqrt(2)) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Distance between points.

```python
def test_dist(self):
    """Basic: Distance between points."""
    p = [1, 2, 3]
    q = [4, 6, 8]
    assert abs(math.dist(p, q) - math.sqrt(50)) < 1e-10
```

**Verification**: ✅ Tested in CI

### Basic: Product of sequence.

```python
def test_prod(self):
    """Basic: Product of sequence."""
    assert math.prod([1, 2, 3, 4]) == 24
    assert math.prod([2, 3, 5]) == 30
```

**Verification**: ✅ Tested in CI

### Edge: Product of empty sequence.

```python
def test_prod_empty(self):
    """Edge: Product of empty sequence."""
    assert math.prod([]) == 1
```

**Verification**: ✅ Tested in CI

### Basic: Combinations (n choose k).

```python
def test_comb(self):
    """Basic: Combinations (n choose k)."""
    assert math.comb(5, 2) == 10
    assert math.comb(10, 3) == 120
```

**Verification**: ✅ Tested in CI

### Basic: Permutations.

```python
def test_perm(self):
    """Basic: Permutations."""
    assert math.perm(5, 2) == 20
    assert math.perm(5, 5) == 120
```

**Verification**: ✅ Tested in CI

### Feature: GCD with multiple arguments.

```python
def test_gcd_multiple_args(self):
    """Feature: GCD with multiple arguments."""
    assert math.gcd(12, 18, 24) == 6
    assert math.gcd(10, 15, 20, 25) == 5
```

**Verification**: ✅ Tested in CI

### Basic: Least common multiple.

```python
def test_lcm(self):
    """Basic: Least common multiple."""
    assert math.lcm(4, 6) == 12
    assert math.lcm(3, 5) == 15
```

**Verification**: ✅ Tested in CI

### Feature: LCM with multiple arguments.

```python
def test_lcm_multiple_args(self):
    """Feature: LCM with multiple arguments."""
    assert math.lcm(4, 6, 8) == 24
```

**Verification**: ✅ Tested in CI
