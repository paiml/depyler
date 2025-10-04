# decimal

## Decimal() - Create decimal numbers.

## Decimal arithmetic operations.

## Decimal comparison operations.

## Decimal precision and context.

## Decimal rounding modes.

## Decimal.quantize() - Set decimal places.

## Decimal mathematical functions.

## Decimal special values.

## Decimal conversion methods.

## Decimal properties and methods.

## Edge cases and special scenarios.

### Basic: Create from string.

```python
def test_create_from_string(self):
    """Basic: Create from string."""
    d = Decimal('3.14')
    assert str(d) == '3.14'
```

**Verification**: ✅ Tested in CI

### Basic: Create from integer.

```python
def test_create_from_int(self):
    """Basic: Create from integer."""
    d = Decimal(42)
    assert str(d) == '42'
```

**Verification**: ✅ Tested in CI

### Feature: Create from tuple (sign, digits, exponent).

```python
def test_create_from_tuple(self):
    """Feature: Create from tuple (sign, digits, exponent)."""
    d = Decimal((0, (3, 1, 4), -2))
    assert str(d) == '3.14'
```

**Verification**: ✅ Tested in CI

### Basic: Create negative decimal.

```python
def test_create_negative(self):
    """Basic: Create negative decimal."""
    d = Decimal('-10.5')
    assert str(d) == '-10.5'
```

**Verification**: ✅ Tested in CI

### Property: String precision preserved.

```python
def test_precision_preserved(self):
    """Property: String precision preserved."""
    d = Decimal('0.1')
    assert str(d) == '0.1'
```

**Verification**: ✅ Tested in CI

### Edge: Float conversion shows imprecision.

```python
def test_float_imprecision(self):
    """Edge: Float conversion shows imprecision."""
    d = Decimal(0.1)
    assert str(d).startswith('0.10000000000000000555')
```

**Verification**: ✅ Tested in CI

### Error: Invalid string raises InvalidOperation.

```python
def test_error_invalid_string(self):
    """Error: Invalid string raises InvalidOperation."""
    with pytest.raises(InvalidOperation):
        Decimal('invalid')
```

**Verification**: ✅ Tested in CI

### Basic: Addition.

```python
def test_addition(self):
    """Basic: Addition."""
    a = Decimal('1.1')
    b = Decimal('2.2')
    result = a + b
    assert str(result) == '3.3'
```

**Verification**: ✅ Tested in CI

### Basic: Subtraction.

```python
def test_subtraction(self):
    """Basic: Subtraction."""
    a = Decimal('5.5')
    b = Decimal('2.2')
    result = a - b
    assert str(result) == '3.3'
```

**Verification**: ✅ Tested in CI

### Basic: Multiplication.

```python
def test_multiplication(self):
    """Basic: Multiplication."""
    a = Decimal('2.5')
    b = Decimal('4')
    result = a * b
    assert str(result) == '10.0'
```

**Verification**: ✅ Tested in CI

### Basic: Division.

```python
def test_division(self):
    """Basic: Division."""
    a = Decimal('10')
    b = Decimal('4')
    result = a / b
    assert str(result) == '2.5'
```

**Verification**: ✅ Tested in CI

### Feature: Floor division.

```python
def test_floor_division(self):
    """Feature: Floor division."""
    a = Decimal('10')
    b = Decimal('3')
    result = a // b
    assert str(result) == '3'
```

**Verification**: ✅ Tested in CI

### Feature: Modulo operation.

```python
def test_modulo(self):
    """Feature: Modulo operation."""
    a = Decimal('10')
    b = Decimal('3')
    result = a % b
    assert str(result) == '1'
```

**Verification**: ✅ Tested in CI

### Feature: Exponentiation.

```python
def test_power(self):
    """Feature: Exponentiation."""
    a = Decimal('2')
    result = a ** 3
    assert str(result) == '8'
```

**Verification**: ✅ Tested in CI

### Basic: Unary negation.

```python
def test_negation(self):
    """Basic: Unary negation."""
    a = Decimal('3.14')
    result = -a
    assert str(result) == '-3.14'
```

**Verification**: ✅ Tested in CI

### Basic: Absolute value.

```python
def test_absolute(self):
    """Basic: Absolute value."""
    a = Decimal('-3.14')
    result = abs(a)
    assert str(result) == '3.14'
```

**Verification**: ✅ Tested in CI

### Error: Division by zero.

```python
def test_error_division_by_zero(self):
    """Error: Division by zero."""
    a = Decimal('10')
    b = Decimal('0')
    with pytest.raises(DivisionByZero):
        _ = a / b
```

**Verification**: ✅ Tested in CI

### Basic: Equality comparison.

```python
def test_equality(self):
    """Basic: Equality comparison."""
    a = Decimal('3.14')
    b = Decimal('3.14')
    assert a == b
```

**Verification**: ✅ Tested in CI

### Basic: Inequality comparison.

```python
def test_inequality(self):
    """Basic: Inequality comparison."""
    a = Decimal('3.14')
    b = Decimal('2.71')
    assert a != b
```

**Verification**: ✅ Tested in CI

### Basic: Less than comparison.

```python
def test_less_than(self):
    """Basic: Less than comparison."""
    a = Decimal('2.71')
    b = Decimal('3.14')
    assert a < b
```

**Verification**: ✅ Tested in CI

### Basic: Greater than comparison.

```python
def test_greater_than(self):
    """Basic: Greater than comparison."""
    a = Decimal('3.14')
    b = Decimal('2.71')
    assert a > b
```

**Verification**: ✅ Tested in CI

### Feature: Compare with integer.

```python
def test_compare_with_int(self):
    """Feature: Compare with integer."""
    a = Decimal('5')
    assert a == 5
```

**Verification**: ✅ Tested in CI

### Property: Different precision but equal value.

```python
def test_compare_different_precision(self):
    """Property: Different precision but equal value."""
    a = Decimal('1.0')
    b = Decimal('1.00')
    assert a == b
```

**Verification**: ✅ Tested in CI

### Basic: Default precision is 28.

```python
def test_default_precision(self):
    """Basic: Default precision is 28."""
    ctx = getcontext()
    assert ctx.prec >= 28
```

**Verification**: ✅ Tested in CI

### Feature: Set precision.

```python
def test_set_precision(self):
    """Feature: Set precision."""
    with localcontext() as ctx:
        ctx.prec = 4
        result = Decimal('1') / Decimal('3')
        assert str(result) == '0.3333'
```

**Verification**: ✅ Tested in CI

### Property: Precision affects result.

```python
def test_precision_affects_operations(self):
    """Property: Precision affects result."""
    with localcontext() as ctx:
        ctx.prec = 2
        result = Decimal('1') / Decimal('3')
        assert str(result) == '0.33'
```

**Verification**: ✅ Tested in CI

### Property: localcontext is isolated.

```python
def test_local_context_isolated(self):
    """Property: localcontext is isolated."""
    original_prec = getcontext().prec
    with localcontext() as ctx:
        ctx.prec = 10
        assert getcontext().prec == 10
    assert getcontext().prec == original_prec
```

**Verification**: ✅ Tested in CI

### Basic: ROUND_HALF_UP mode.

```python
def test_round_half_up(self):
    """Basic: ROUND_HALF_UP mode."""
    d = Decimal('2.5')
    result = d.quantize(Decimal('1'), rounding=ROUND_HALF_UP)
    assert str(result) == '3'
```

**Verification**: ✅ Tested in CI

### Feature: ROUND_DOWN mode.

```python
def test_round_down(self):
    """Feature: ROUND_DOWN mode."""
    d = Decimal('2.9')
    result = d.quantize(Decimal('1'), rounding=ROUND_DOWN)
    assert str(result) == '2'
```

**Verification**: ✅ Tested in CI

### Feature: ROUND_UP mode.

```python
def test_round_up(self):
    """Feature: ROUND_UP mode."""
    d = Decimal('2.1')
    result = d.quantize(Decimal('1'), rounding=ROUND_UP)
    assert str(result) == '3'
```

**Verification**: ✅ Tested in CI

### Feature: ROUND_CEILING mode (toward +inf).

```python
def test_round_ceiling(self):
    """Feature: ROUND_CEILING mode (toward +inf)."""
    d = Decimal('2.1')
    result = d.quantize(Decimal('1'), rounding=ROUND_CEILING)
    assert str(result) == '3'
```

**Verification**: ✅ Tested in CI

### Feature: ROUND_FLOOR mode (toward -inf).

```python
def test_round_floor(self):
    """Feature: ROUND_FLOOR mode (toward -inf)."""
    d = Decimal('2.9')
    result = d.quantize(Decimal('1'), rounding=ROUND_FLOOR)
    assert str(result) == '2'
```

**Verification**: ✅ Tested in CI

### Edge: ROUND_CEILING with negative.

```python
def test_round_negative_ceiling(self):
    """Edge: ROUND_CEILING with negative."""
    d = Decimal('-2.1')
    result = d.quantize(Decimal('1'), rounding=ROUND_CEILING)
    assert str(result) == '-2'
```

**Verification**: ✅ Tested in CI

### Edge: ROUND_FLOOR with negative.

```python
def test_round_negative_floor(self):
    """Edge: ROUND_FLOOR with negative."""
    d = Decimal('-2.1')
    result = d.quantize(Decimal('1'), rounding=ROUND_FLOOR)
    assert str(result) == '-3'
```

**Verification**: ✅ Tested in CI

### Basic: Quantize to 2 decimal places.

```python
def test_quantize_two_places(self):
    """Basic: Quantize to 2 decimal places."""
    d = Decimal('3.14159')
    result = d.quantize(Decimal('0.01'))
    assert str(result) == '3.14'
```

**Verification**: ✅ Tested in CI

### Feature: Quantize to integer.

```python
def test_quantize_no_places(self):
    """Feature: Quantize to integer."""
    d = Decimal('3.14159')
    result = d.quantize(Decimal('1'))
    assert str(result) == '3'
```

**Verification**: ✅ Tested in CI

### Use case: Financial calculations.

```python
def test_quantize_money(self):
    """Use case: Financial calculations."""
    price = Decimal('19.99')
    quantity = Decimal('3')
    total = (price * quantity).quantize(Decimal('0.01'))
    assert str(total) == '59.97'
```

**Verification**: ✅ Tested in CI

### Property: Quantize sets exact decimal places.

```python
def test_quantize_preserves_precision(self):
    """Property: Quantize sets exact decimal places."""
    d = Decimal('5')
    result = d.quantize(Decimal('0.00'))
    assert str(result) == '5.00'
```

**Verification**: ✅ Tested in CI

### Basic: Square root.

```python
def test_sqrt(self):
    """Basic: Square root."""
    d = Decimal('4')
    result = d.sqrt()
    assert str(result) == '2'
```

**Verification**: ✅ Tested in CI

### Feature: Square root with precision.

```python
def test_sqrt_precision(self):
    """Feature: Square root with precision."""
    with localcontext() as ctx:
        ctx.prec = 10
        d = Decimal('2')
        result = d.sqrt()
        assert str(result).startswith('1.414213562')
```

**Verification**: ✅ Tested in CI

### Basic: Exponential.

```python
def test_exp(self):
    """Basic: Exponential."""
    d = Decimal('0')
    result = d.exp()
    assert str(result) == '1'
```

**Verification**: ✅ Tested in CI

### Basic: Natural logarithm.

```python
def test_ln(self):
    """Basic: Natural logarithm."""
    d = Decimal('1')
    result = d.ln()
    assert str(result) == '0'
```

**Verification**: ✅ Tested in CI

### Basic: Base-10 logarithm.

```python
def test_log10(self):
    """Basic: Base-10 logarithm."""
    d = Decimal('10')
    result = d.log10()
    assert str(result) == '1'
```

**Verification**: ✅ Tested in CI

### Error: Square root of negative.

```python
def test_error_sqrt_negative(self):
    """Error: Square root of negative."""
    d = Decimal('-4')
    with pytest.raises(InvalidOperation):
        d.sqrt()
```

**Verification**: ✅ Tested in CI

### Basic: Positive infinity.

```python
def test_infinity_positive(self):
    """Basic: Positive infinity."""
    d = Decimal('Infinity')
    assert d.is_infinite()
    assert not d.is_finite()
```

**Verification**: ✅ Tested in CI

### Basic: Negative infinity.

```python
def test_infinity_negative(self):
    """Basic: Negative infinity."""
    d = Decimal('-Infinity')
    assert d.is_infinite()
```

**Verification**: ✅ Tested in CI

### Basic: Not a number.

```python
def test_nan(self):
    """Basic: Not a number."""
    d = Decimal('NaN')
    assert d.is_nan()
```

**Verification**: ✅ Tested in CI

### Edge: Infinity arithmetic.

```python
def test_infinity_arithmetic(self):
    """Edge: Infinity arithmetic."""
    inf = Decimal('Infinity')
    result = inf + 1
    assert result.is_infinite()
```

**Verification**: ✅ Tested in CI

### Edge: NaN propagates.

```python
def test_nan_propagation(self):
    """Edge: NaN propagates."""
    nan = Decimal('NaN')
    result = nan + 1
    assert result.is_nan()
```

**Verification**: ✅ Tested in CI

### Property: NaN not equal to itself.

```python
def test_nan_not_equal(self):
    """Property: NaN not equal to itself."""
    nan = Decimal('NaN')
    assert not nan == nan
```

**Verification**: ✅ Tested in CI

### Basic: Convert to int.

```python
def test_to_int(self):
    """Basic: Convert to int."""
    d = Decimal('3.14')
    result = int(d)
    assert result == 3
```

**Verification**: ✅ Tested in CI

### Basic: Convert to float.

```python
def test_to_float(self):
    """Basic: Convert to float."""
    d = Decimal('3.14')
    result = float(d)
    assert abs(result - 3.14) < 1e-10
```

**Verification**: ✅ Tested in CI

### Feature: Get tuple representation.

```python
def test_as_tuple(self):
    """Feature: Get tuple representation."""
    d = Decimal('3.14')
    sign, digits, exponent = d.as_tuple()
    assert sign == 0
    assert digits == (3, 1, 4)
    assert exponent == -2
```

**Verification**: ✅ Tested in CI

### Feature: Tuple for negative number.

```python
def test_as_tuple_negative(self):
    """Feature: Tuple for negative number."""
    d = Decimal('-3.14')
    sign, digits, exponent = d.as_tuple()
    assert sign == 1
```

**Verification**: ✅ Tested in CI

### Basic: Check if zero.

```python
def test_is_zero(self):
    """Basic: Check if zero."""
    d = Decimal('0')
    assert d.is_zero()
```

**Verification**: ✅ Tested in CI

### Feature: Check sign.

```python
def test_is_signed(self):
    """Feature: Check sign."""
    pos = Decimal('3.14')
    neg = Decimal('-3.14')
    assert pos.is_signed() == False
    assert neg.is_signed() == True
```

**Verification**: ✅ Tested in CI

### Feature: Copy absolute value.

```python
def test_copy_abs(self):
    """Feature: Copy absolute value."""
    d = Decimal('-3.14')
    result = d.copy_abs()
    assert str(result) == '3.14'
```

**Verification**: ✅ Tested in CI

### Feature: Copy negated value.

```python
def test_copy_negate(self):
    """Feature: Copy negated value."""
    d = Decimal('3.14')
    result = d.copy_negate()
    assert str(result) == '-3.14'
```

**Verification**: ✅ Tested in CI

### Feature: Copy with sign from another.

```python
def test_copy_sign(self):
    """Feature: Copy with sign from another."""
    d = Decimal('3.14')
    other = Decimal('-1')
    result = d.copy_sign(other)
    assert str(result) == '-3.14'
```

**Verification**: ✅ Tested in CI

### Feature: Total ordering comparison.

```python
def test_compare_total(self):
    """Feature: Total ordering comparison."""
    a = Decimal('1.0')
    b = Decimal('1.00')
    result = a.compare_total(b)
    assert result != 0
```

**Verification**: ✅ Tested in CI

### Edge: Division by zero with context.

```python
def test_zero_division_context(self):
    """Edge: Division by zero with context."""
    with localcontext() as ctx:
        ctx.traps[DivisionByZero] = False
        result = Decimal('1') / Decimal('0')
        assert result.is_infinite()
```

**Verification**: ✅ Tested in CI

### Edge: Overflow handling.

```python
def test_overflow_handling(self):
    """Edge: Overflow handling."""
    with localcontext() as ctx:
        ctx.traps[Overflow] = False
        ctx.Emax = 10
        result = Decimal('10') ** Decimal('20')
        assert result.is_infinite()
```

**Verification**: ✅ Tested in CI

### Feature: Normalize removes trailing zeros.

```python
def test_normalize(self):
    """Feature: Normalize removes trailing zeros."""
    d = Decimal('1.500')
    result = d.normalize()
    assert str(result) == '1.5'
```

**Verification**: ✅ Tested in CI

### Feature: Canonical representation.

```python
def test_canonical(self):
    """Feature: Canonical representation."""
    d = Decimal('1.500')
    result = d.canonical()
    assert isinstance(result, Decimal)
```

**Verification**: ✅ Tested in CI

### Feature: Adjusted exponent.

```python
def test_adjusted_exponent(self):
    """Feature: Adjusted exponent."""
    d = Decimal('123.45')
    assert d.adjusted() == 2
```

**Verification**: ✅ Tested in CI

### Feature: Check same quantum (exponent).

```python
def test_same_quantum(self):
    """Feature: Check same quantum (exponent)."""
    a = Decimal('1.0')
    b = Decimal('2.0')
    c = Decimal('1.00')
    assert a.same_quantum(b)
    assert not a.same_quantum(c)
```

**Verification**: ✅ Tested in CI

### Property: Count decimal places.

```python
def test_decimal_places(self):
    """Property: Count decimal places."""
    d = Decimal('3.14159')
    exponent = d.as_tuple().exponent
    assert exponent == -5
```

**Verification**: ✅ Tested in CI

### Feature: Min and max operations.

```python
def test_min_max(self):
    """Feature: Min and max operations."""
    a = Decimal('3.14')
    b = Decimal('2.71')
    assert a.max(b) == a
    assert a.min(b) == b
```

**Verification**: ✅ Tested in CI

### Feature: Next larger number.

```python
def test_next_plus(self):
    """Feature: Next larger number."""
    d = Decimal('1')
    with localcontext() as ctx:
        ctx.prec = 5
        result = d.next_plus()
        assert result > d
```

**Verification**: ✅ Tested in CI

### Feature: Next smaller number.

```python
def test_next_minus(self):
    """Feature: Next smaller number."""
    d = Decimal('1')
    with localcontext() as ctx:
        ctx.prec = 5
        result = d.next_minus()
        assert result < d
```

**Verification**: ✅ Tested in CI

### Use case: Financial calculation preserves precision.

```python
def test_financial_calculation(self):
    """Use case: Financial calculation preserves precision."""
    a = Decimal('0.1')
    b = Decimal('0.2')
    result = a + b
    assert str(result) == '0.3'
```

**Verification**: ✅ Tested in CI

### Property: Chained operations maintain precision.

```python
def test_chained_operations(self):
    """Property: Chained operations maintain precision."""
    d = Decimal('10.00')
    result = (d / 3 * 3).quantize(Decimal('0.00'))
    assert str(result) == '10.00'
```

**Verification**: ✅ Tested in CI

### Feature: from_float for exact conversion.

```python
def test_from_float_exact(self):
    """Feature: from_float for exact conversion."""
    f = 0.1
    d = Decimal.from_float(f)
    assert '0.10000000000000000555' in str(d)
```

**Verification**: ✅ Tested in CI

### Edge: Total order vs numeric comparison.

```python
def test_comparison_total_order(self):
    """Edge: Total order vs numeric comparison."""
    a = Decimal('1.0')
    b = Decimal('1.00')
    assert a == b
    assert a.compare_total(b) != 0
```

**Verification**: ✅ Tested in CI

### Feature: Scientific notation.

```python
def test_scientific_notation(self):
    """Feature: Scientific notation."""
    d = Decimal('1.23E+4')
    assert str(d) == '1.23E+4'
    assert d == 12300
```

**Verification**: ✅ Tested in CI
