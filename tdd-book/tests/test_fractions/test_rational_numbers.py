"""Test fractions module - Rational number arithmetic.

This module tests Fraction for exact rational number arithmetic without
floating point precision issues.
"""

from fractions import Fraction
from decimal import Decimal
import pytest


class TestFractionCreation:
    """Fraction() - Create rational numbers."""

    def test_create_from_integers(self):
        """Basic: Create from numerator and denominator."""
        f = Fraction(3, 4)
        assert f.numerator == 3
        assert f.denominator == 4

    def test_create_from_integer(self):
        """Basic: Create from single integer."""
        f = Fraction(5)
        assert f.numerator == 5
        assert f.denominator == 1

    def test_create_from_string(self):
        """Basic: Create from string."""
        f = Fraction('3/4')
        assert f.numerator == 3
        assert f.denominator == 4

    def test_create_from_decimal_string(self):
        """Feature: Create from decimal string."""
        f = Fraction('0.25')
        assert f.numerator == 1
        assert f.denominator == 4

    def test_create_from_float(self):
        """Feature: Create from float."""
        f = Fraction(0.5)
        assert f.numerator == 1
        assert f.denominator == 2

    def test_create_from_decimal(self):
        """Feature: Create from Decimal."""
        d = Decimal('0.25')
        f = Fraction(d)
        assert f.numerator == 1
        assert f.denominator == 4

    def test_automatic_reduction(self):
        """Property: Automatically reduces to lowest terms."""
        f = Fraction(6, 8)
        assert f.numerator == 3
        assert f.denominator == 4

    def test_negative_fraction(self):
        """Basic: Negative fractions."""
        f = Fraction(-3, 4)
        assert f.numerator == -3
        assert f.denominator == 4

    def test_negative_denominator(self):
        """Property: Negative in denominator moves to numerator."""
        f = Fraction(3, -4)
        assert f.numerator == -3
        assert f.denominator == 4

    def test_error_zero_denominator(self):
        """Error: Zero denominator raises ZeroDivisionError."""
        with pytest.raises(ZeroDivisionError):
            Fraction(1, 0)

    def test_error_invalid_string(self):
        """Error: Invalid string raises ValueError."""
        with pytest.raises(ValueError):
            Fraction('invalid')


class TestArithmetic:
    """Fraction arithmetic operations."""

    def test_addition(self):
        """Basic: Addition of fractions."""
        a = Fraction(1, 4)
        b = Fraction(1, 2)
        result = a + b
        assert result == Fraction(3, 4)

    def test_subtraction(self):
        """Basic: Subtraction of fractions."""
        a = Fraction(3, 4)
        b = Fraction(1, 4)
        result = a - b
        assert result == Fraction(1, 2)

    def test_multiplication(self):
        """Basic: Multiplication of fractions."""
        a = Fraction(2, 3)
        b = Fraction(3, 4)
        result = a * b
        assert result == Fraction(1, 2)

    def test_division(self):
        """Basic: Division of fractions."""
        a = Fraction(1, 2)
        b = Fraction(1, 4)
        result = a / b
        assert result == Fraction(2, 1)

    def test_floor_division(self):
        """Feature: Floor division."""
        a = Fraction(7, 4)
        b = Fraction(1, 2)
        result = a // b
        assert result == 3

    def test_modulo(self):
        """Feature: Modulo operation."""
        a = Fraction(7, 4)
        b = Fraction(1, 2)
        result = a % b
        assert result == Fraction(1, 4)

    def test_power(self):
        """Feature: Exponentiation."""
        f = Fraction(2, 3)
        result = f ** 2
        assert result == Fraction(4, 9)

    def test_negative_power(self):
        """Feature: Negative exponent."""
        f = Fraction(2, 3)
        result = f ** -1
        assert result == Fraction(3, 2)

    def test_negation(self):
        """Basic: Unary negation."""
        f = Fraction(3, 4)
        result = -f
        assert result == Fraction(-3, 4)

    def test_absolute(self):
        """Basic: Absolute value."""
        f = Fraction(-3, 4)
        result = abs(f)
        assert result == Fraction(3, 4)

    def test_add_with_integer(self):
        """Feature: Add fraction and integer."""
        f = Fraction(1, 4)
        result = f + 1
        assert result == Fraction(5, 4)

    def test_multiply_by_integer(self):
        """Feature: Multiply fraction by integer."""
        f = Fraction(2, 3)
        result = f * 3
        assert result == Fraction(2, 1)

    def test_error_division_by_zero(self):
        """Error: Division by zero."""
        f = Fraction(1, 2)
        with pytest.raises(ZeroDivisionError):
            _ = f / 0


class TestComparison:
    """Fraction comparison operations."""

    def test_equality(self):
        """Basic: Equality comparison."""
        a = Fraction(1, 2)
        b = Fraction(2, 4)
        assert a == b

    def test_inequality(self):
        """Basic: Inequality comparison."""
        a = Fraction(1, 2)
        b = Fraction(1, 3)
        assert a != b

    def test_less_than(self):
        """Basic: Less than comparison."""
        a = Fraction(1, 3)
        b = Fraction(1, 2)
        assert a < b

    def test_greater_than(self):
        """Basic: Greater than comparison."""
        a = Fraction(1, 2)
        b = Fraction(1, 3)
        assert a > b

    def test_compare_with_integer(self):
        """Feature: Compare with integer."""
        f = Fraction(4, 2)
        assert f == 2

    def test_compare_with_float(self):
        """Feature: Compare with float."""
        f = Fraction(1, 2)
        assert f == 0.5

    def test_compare_different_denominators(self):
        """Property: Compares correctly with different denominators."""
        a = Fraction(1, 2)
        b = Fraction(2, 3)
        assert a < b


class TestConversion:
    """Fraction conversion methods."""

    def test_to_float(self):
        """Basic: Convert to float."""
        f = Fraction(1, 2)
        result = float(f)
        assert result == 0.5

    def test_to_int(self):
        """Basic: Convert to int (truncates)."""
        f = Fraction(7, 4)
        result = int(f)
        assert result == 1

    def test_to_string(self):
        """Basic: String representation."""
        f = Fraction(3, 4)
        assert str(f) == '3/4'

    def test_repr(self):
        """Feature: Repr representation."""
        f = Fraction(3, 4)
        assert repr(f) == 'Fraction(3, 4)'

    def test_whole_number_string(self):
        """Edge: Whole number as string."""
        f = Fraction(4, 2)
        assert str(f) == '2'


class TestProperties:
    """Fraction properties and methods."""

    def test_numerator_property(self):
        """Basic: Numerator property."""
        f = Fraction(3, 4)
        assert f.numerator == 3

    def test_denominator_property(self):
        """Basic: Denominator property."""
        f = Fraction(3, 4)
        assert f.denominator == 4

    def test_limit_denominator(self):
        """Feature: Approximate with limited denominator."""
        f = Fraction(3.141592653589793)
        approx = f.limit_denominator(100)
        # Should be close to 22/7
        assert approx.denominator <= 100

    def test_limit_denominator_exact(self):
        """Property: Exact if within limit."""
        f = Fraction(22, 7)
        approx = f.limit_denominator(100)
        assert approx == f

    def test_from_float_method(self):
        """Feature: Explicit from_float constructor."""
        f = Fraction.from_float(0.25)
        assert f == Fraction(1, 4)

    def test_from_decimal_method(self):
        """Feature: Explicit from_decimal constructor."""
        d = Decimal('0.25')
        f = Fraction.from_decimal(d)
        assert f == Fraction(1, 4)

    def test_as_integer_ratio(self):
        """Feature: Get numerator and denominator tuple."""
        f = Fraction(3, 4)
        num, den = f.as_integer_ratio()
        assert num == 3
        assert den == 4


class TestMathematical:
    """Mathematical operations and properties."""

    def test_reciprocal(self):
        """Feature: Reciprocal via division."""
        f = Fraction(3, 4)
        reciprocal = 1 / f
        assert reciprocal == Fraction(4, 3)

    def test_mixed_operations(self):
        """Property: Mixed operations maintain exactness."""
        a = Fraction(1, 3)
        b = Fraction(1, 6)
        result = (a + b) * 2
        assert result == Fraction(1, 1)

    def test_repeated_division(self):
        """Property: Repeated operations exact."""
        f = Fraction(1, 1)
        for _ in range(10):
            f = f / 2
        assert f == Fraction(1, 1024)

    def test_zero_fraction(self):
        """Edge: Zero as fraction."""
        f = Fraction(0, 1)
        assert f.numerator == 0
        assert f.denominator == 1


class TestEdgeCases:
    """Edge cases and special scenarios."""

    def test_one_as_fraction(self):
        """Edge: One represented exactly."""
        f = Fraction(1, 1)
        assert f == 1

    def test_large_numerator_denominator(self):
        """Performance: Large numbers."""
        f = Fraction(123456789, 987654321)
        assert f.numerator == 13717421
        assert f.denominator == 109739369

    def test_float_precision_issue(self):
        """Property: Solves float precision problems."""
        # 0.1 + 0.2 != 0.3 in float, but exact in Fraction
        a = Fraction('0.1')
        b = Fraction('0.2')
        result = a + b
        assert result == Fraction('0.3')

    def test_reduce_common_factor(self):
        """Property: Always in lowest terms."""
        f = Fraction(100, 200)
        assert f.numerator == 1
        assert f.denominator == 2

    def test_negative_both(self):
        """Edge: Negative numerator and denominator."""
        f = Fraction(-3, -4)
        assert f.numerator == 3
        assert f.denominator == 4

    def test_mixed_number_string(self):
        """Error: Mixed number string not supported."""
        # Fractions don't parse mixed numbers like "1 1/2"
        with pytest.raises(ValueError):
            Fraction('1 1/2')

    def test_bool_conversion(self):
        """Feature: Bool conversion."""
        zero = Fraction(0, 1)
        nonzero = Fraction(1, 2)
        assert not zero
        assert nonzero

    def test_hash_consistency(self):
        """Property: Equal fractions have same hash."""
        a = Fraction(1, 2)
        b = Fraction(2, 4)
        assert hash(a) == hash(b)

    def test_hash_with_integer(self):
        """Property: Fraction(n, 1) hashes like int(n)."""
        f = Fraction(5, 1)
        assert hash(f) == hash(5)

    def test_gcd_in_reduction(self):
        """Property: Uses GCD for reduction."""
        f = Fraction(12, 18)
        # GCD(12, 18) = 6, so reduces to 2/3
        assert f.numerator == 2
        assert f.denominator == 3

    def test_compare_with_zero(self):
        """Edge: Compare with zero."""
        positive = Fraction(1, 2)
        negative = Fraction(-1, 2)
        zero = Fraction(0, 1)
        assert positive > zero
        assert negative < zero
        assert zero == 0

    def test_from_float_repeating_decimal(self):
        """Edge: Float with repeating decimal."""
        f = Fraction(1.0 / 3.0)
        # Won't be exactly 1/3 due to float precision
        approx = f.limit_denominator(10)
        assert approx == Fraction(1, 3)

    def test_string_with_whitespace(self):
        """Feature: String with whitespace."""
        f = Fraction('  3/4  ')
        assert f == Fraction(3, 4)

    def test_string_negative(self):
        """Feature: String with negative sign."""
        f = Fraction('-3/4')
        assert f.numerator == -3
        assert f.denominator == 4

    def test_power_zero(self):
        """Edge: Any fraction to power 0 is 1."""
        f = Fraction(3, 4)
        result = f ** 0
        assert result == 1

    def test_power_one(self):
        """Edge: Any fraction to power 1 is itself."""
        f = Fraction(3, 4)
        result = f ** 1
        assert result == f

    def test_zero_power_positive(self):
        """Edge: Zero to positive power is zero."""
        f = Fraction(0, 1)
        result = f ** 5
        assert result == 0

    def test_error_zero_power_zero(self):
        """Edge: Zero to power zero is 1 (by convention)."""
        f = Fraction(0, 1)
        result = f ** 0
        assert result == 1

    def test_from_another_fraction(self):
        """Feature: Create from another Fraction."""
        original = Fraction(3, 4)
        copy = Fraction(original)
        assert copy == original

    def test_exact_decimal_conversion(self):
        """Property: Exact conversion for terminating decimals."""
        f = Fraction('0.125')
        assert f == Fraction(1, 8)

    def test_simplification_maintains_sign(self):
        """Property: Sign preserved during simplification."""
        f = Fraction(-6, 8)
        assert f.numerator == -3
        assert f.denominator == 4
