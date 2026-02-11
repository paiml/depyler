"""Rational arithmetic with GCD normalization.

Tests: gcd, add, multiply, simplify fractions.
"""


def gcd(a: int, b: int) -> int:
    """Greatest common divisor using Euclidean algorithm."""
    x: int = a
    y: int = b
    if x < 0:
        x = -x
    if y < 0:
        y = -y
    while y != 0:
        tmp: int = y
        y = x % y
        x = tmp
    return x


def simplify_num(num: int, den: int) -> int:
    """Return simplified numerator."""
    if den == 0:
        return 0
    g: int = gcd(num, den)
    result: int = num // g
    if den < 0:
        result = -result
    return result


def simplify_den(num: int, den: int) -> int:
    """Return simplified denominator."""
    if den == 0:
        return 0
    g: int = gcd(num, den)
    result: int = den // g
    if result < 0:
        result = -result
    return result


def rat_add_num(n1: int, d1: int, n2: int, d2: int) -> int:
    """Numerator of rational addition: n1/d1 + n2/d2."""
    new_num: int = n1 * d2 + n2 * d1
    new_den: int = d1 * d2
    return simplify_num(new_num, new_den)


def rat_add_den(n1: int, d1: int, n2: int, d2: int) -> int:
    """Denominator of rational addition."""
    new_num: int = n1 * d2 + n2 * d1
    new_den: int = d1 * d2
    return simplify_den(new_num, new_den)


def rat_mul_num(n1: int, d1: int, n2: int, d2: int) -> int:
    """Numerator of rational multiplication."""
    new_num: int = n1 * n2
    new_den: int = d1 * d2
    return simplify_num(new_num, new_den)


def rat_mul_den(n1: int, d1: int, n2: int, d2: int) -> int:
    """Denominator of rational multiplication."""
    new_num: int = n1 * n2
    new_den: int = d1 * d2
    return simplify_den(new_num, new_den)


def test_module() -> None:
    assert gcd(12, 8) == 4
    assert gcd(7, 13) == 1
    assert gcd(0, 5) == 5
    assert simplify_num(4, 8) == 1
    assert simplify_den(4, 8) == 2
    assert simplify_num(6, 3) == 2
    assert simplify_den(6, 3) == 1
    assert rat_add_num(1, 2, 1, 3) == 5
    assert rat_add_den(1, 2, 1, 3) == 6
    assert rat_add_num(1, 4, 1, 4) == 1
    assert rat_add_den(1, 4, 1, 4) == 2
    assert rat_mul_num(2, 3, 3, 4) == 1
    assert rat_mul_den(2, 3, 3, 4) == 2
