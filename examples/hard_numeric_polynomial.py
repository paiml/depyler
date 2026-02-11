"""Polynomial evaluation using Horner's method and operations."""


def horner_eval(coeffs: list[int], x: int) -> int:
    """Evaluate polynomial using Horner's method.
    coeffs[0] is highest degree coefficient."""
    n: int = len(coeffs)
    if n == 0:
        return 0
    result: int = coeffs[0]
    i: int = 1
    while i < n:
        result = result * x + coeffs[i]
        i = i + 1
    return result


def poly_eval_ascending(coeffs: list[int], x: int) -> int:
    """Evaluate polynomial with ascending powers. coeffs[i] = coeff of x^i."""
    n: int = len(coeffs)
    result: int = 0
    power: int = 1
    i: int = 0
    while i < n:
        result = result + coeffs[i] * power
        power = power * x
        i = i + 1
    return result


def poly_add(a: list[int], b: list[int]) -> list[int]:
    """Add two polynomials (ascending power representation)."""
    na: int = len(a)
    nb: int = len(b)
    maxn: int = na
    if nb > maxn:
        maxn = nb
    result: list[int] = []
    i: int = 0
    while i < maxn:
        va: int = 0
        if i < na:
            va = a[i]
        vb: int = 0
        if i < nb:
            vb = b[i]
        result.append(va + vb)
        i = i + 1
    return result


def poly_degree(coeffs: list[int]) -> int:
    """Return degree of polynomial (ascending powers)."""
    n: int = len(coeffs)
    i: int = n - 1
    while i >= 0:
        if coeffs[i] != 0:
            return i
        i = i - 1
    return 0


def test_module() -> int:
    """Test polynomial functions."""
    ok: int = 0
    h: list[int] = [1, 0 - 1]
    if horner_eval(h, 3) == 2:
        ok = ok + 1
    asc: list[int] = [1, 2, 3]
    if poly_eval_ascending(asc, 2) == 17:
        ok = ok + 1
    b: list[int] = [0, 1, 0]
    ab: list[int] = poly_add(asc, b)
    if ab[1] == 3:
        ok = ok + 1
    if poly_degree(asc) == 2:
        ok = ok + 1
    if horner_eval([2, 3, 1], 0) == 1:
        ok = ok + 1
    return ok
