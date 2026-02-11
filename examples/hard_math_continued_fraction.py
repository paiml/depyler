"""Continued fraction representation of rational numbers."""


def cf_expand(numerator: int, denominator: int) -> list[int]:
    """Expand a rational number as continued fraction coefficients."""
    result: list[int] = []
    a: int = numerator
    b: int = denominator
    while b != 0:
        q: int = a // b
        result.append(q)
        t: int = b
        b = a - q * b
        a = t
    return result


def cf_evaluate(coeffs: list[int]) -> list[int]:
    """Evaluate continued fraction to rational [numerator, denominator]."""
    n: int = len(coeffs)
    if n == 0:
        return [0, 1]
    num: int = coeffs[n - 1]
    den: int = 1
    i: int = n - 2
    while i >= 0:
        t: int = num
        num = coeffs[i] * num + den
        den = t
        i = i - 1
    return [num, den]


def cf_convergent(coeffs: list[int], depth: int) -> list[int]:
    """Compute the convergent at given depth."""
    n: int = len(coeffs)
    if depth > n:
        depth = n
    sub: list[int] = []
    i: int = 0
    while i < depth:
        sub.append(coeffs[i])
        i = i + 1
    return cf_evaluate(sub)


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    while b != 0:
        t: int = b
        b = a % b
        a = t
    return a


def cf_length(numerator: int, denominator: int) -> int:
    """Length of continued fraction expansion."""
    coeffs: list[int] = cf_expand(numerator, denominator)
    return len(coeffs)


def test_module() -> int:
    """Test continued fraction functions."""
    ok: int = 0
    cf: list[int] = cf_expand(355, 113)
    if cf[0] == 3:
        ok = ok + 1
    r: list[int] = cf_evaluate(cf)
    if r[0] == 355:
        ok = ok + 1
    if r[1] == 113:
        ok = ok + 1
    c1: list[int] = cf_convergent(cf, 1)
    if c1[0] == 3:
        ok = ok + 1
    if cf_length(22, 7) == 2:
        ok = ok + 1
    cf2: list[int] = cf_expand(22, 7)
    r2: list[int] = cf_evaluate(cf2)
    if r2[0] == 22:
        ok = ok + 1
    return ok
