"""Numerical methods: Taylor series expansions.

Tests: factorial computation, power accumulation, alternating signs,
convergence by term magnitude, series truncation.
"""

from typing import List, Tuple


def taylor_exp(x: float, terms: int) -> float:
    """Compute e^x using Taylor series: sum(x^n/n!)."""
    result: float = 1.0
    term: float = 1.0
    n: int = 1
    while n < terms:
        term = term * x / float(n)
        result = result + term
        n += 1
    return result


def taylor_sin(x: float, terms: int) -> float:
    """Compute sin(x) using Taylor series."""
    result: float = x
    term: float = x
    n: int = 1
    while n < terms:
        d1: int = 2 * n
        d2: int = d1 + 1
        denom: float = float(d1) * float(d2)
        term = -term * x * x / denom
        result = result + term
        n += 1
    return result


def taylor_cos(x: float, terms: int) -> float:
    """Compute cos(x) using Taylor series."""
    result: float = 1.0
    term: float = 1.0
    n: int = 1
    while n < terms:
        d2: int = 2 * n
        d1: int = d2 - 1
        denom: float = float(d1) * float(d2)
        term = -term * x * x / denom
        result = result + term
        n += 1
    return result


def taylor_ln1px(x: float, terms: int) -> float:
    """Compute ln(1+x) using Taylor series for |x| < 1."""
    result: float = 0.0
    sign: float = 1.0
    power: float = x
    n: int = 1
    while n <= terms:
        result = result + sign * power / float(n)
        sign = -sign
        power = power * x
        n += 1
    return result


def taylor_atan(x: float, terms: int) -> float:
    """Compute arctan(x) using Taylor series for |x| <= 1."""
    result: float = 0.0
    sign: float = 1.0
    power: float = x
    n: int = 0
    while n < terms:
        divisor: int = 2 * n + 1
        result = result + sign * power / float(divisor)
        sign = -sign
        power = power * x * x
        n += 1
    return result


def taylor_sinh(x: float, terms: int) -> float:
    """Compute sinh(x) using Taylor series."""
    result: float = x
    term: float = x
    n: int = 1
    while n < terms:
        d1: int = 2 * n
        d2: int = d1 + 1
        denom: float = float(d1) * float(d2)
        term = term * x * x / denom
        result = result + term
        n += 1
    return result


def taylor_cosh(x: float, terms: int) -> float:
    """Compute cosh(x) using Taylor series."""
    result: float = 1.0
    term: float = 1.0
    n: int = 1
    while n < terms:
        d2: int = 2 * n
        d1: int = d2 - 1
        denom: float = float(d1) * float(d2)
        term = term * x * x / denom
        result = result + term
        n += 1
    return result


def pi_leibniz(terms: int) -> float:
    """Approximate pi using Leibniz formula: pi/4 = 1 - 1/3 + 1/5 - ..."""
    result: float = 0.0
    sign: float = 1.0
    n: int = 0
    while n < terms:
        divisor: int = 2 * n + 1
        result = result + sign / float(divisor)
        sign = -sign
        n += 1
    return result * 4.0


def test_taylor() -> bool:
    """Test Taylor series approximations."""
    ok: bool = True
    e: float = taylor_exp(1.0, 20)
    diff: float = e - 2.71828
    if diff < 0.0:
        diff = -diff
    if diff > 0.001:
        ok = False
    s: float = taylor_sin(0.0, 10)
    if s < -0.001:
        ok = False
    if s > 0.001:
        ok = False
    c: float = taylor_cos(0.0, 10)
    diff2: float = c - 1.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.001:
        ok = False
    return ok
