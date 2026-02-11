"""Rational number operations and GCD algorithms.

Tests: GCD computation, LCM, integer arithmetic, modular ops.
"""


def gcd(a: int, b: int) -> int:
    """Compute greatest common divisor."""
    if a < 0:
        a = -a
    if b < 0:
        b = -b
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    return a


def lcm(a: int, b: int) -> int:
    """Compute least common multiple."""
    g: int = gcd(a, b)
    if g == 0:
        return 0
    result: int = a // g * b
    if result < 0:
        result = -result
    return result


def simplify_num(num: int, den: int) -> int:
    """Simplify fraction and return numerator."""
    g: int = gcd(num, den)
    if g > 1:
        return num // g
    return num


def simplify_den(num: int, den: int) -> int:
    """Simplify fraction and return denominator."""
    g: int = gcd(num, den)
    if g > 1:
        return den // g
    return den


def fraction_add_num(n1: int, d1: int, n2: int, d2: int) -> int:
    """Compute numerator of sum of two fractions."""
    return n1 * d2 + n2 * d1


def fraction_add_den(d1: int, d2: int) -> int:
    """Compute denominator of sum of two fractions."""
    return d1 * d2


def fraction_mul_num(n1: int, n2: int) -> int:
    """Compute numerator of product."""
    return n1 * n2


def fraction_mul_den(d1: int, d2: int) -> int:
    """Compute denominator of product."""
    return d1 * d2


def test_module() -> int:
    """Test fraction operations."""
    ok: int = 0

    sn: int = simplify_num(6, 4)
    if sn == 3:
        ok += 1
    sd: int = simplify_den(6, 4)
    if sd == 2:
        ok += 1

    g: int = gcd(12, 8)
    if g == 4:
        ok += 1

    g2: int = gcd(7, 13)
    if g2 == 1:
        ok += 1

    an: int = fraction_add_num(1, 2, 1, 3)
    if an == 5:
        ok += 1

    l: int = lcm(4, 6)
    if l == 12:
        ok += 1

    mn: int = fraction_mul_num(3, 5)
    if mn == 15:
        ok += 1

    return ok
