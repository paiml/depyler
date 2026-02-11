"""Polynomial evaluation algorithms.

Tests: Horner's method, polynomial derivative, coefficient operations.
"""


def horner_eval(coeffs: list[int], x: int) -> int:
    """Evaluate polynomial using Horner's method.
    coeffs[0] is the highest degree coefficient.
    """
    if len(coeffs) == 0:
        return 0
    result: int = coeffs[0]
    i: int = 1
    while i < len(coeffs):
        result = result * x + coeffs[i]
        i += 1
    return result


def poly_derivative(coeffs: list[int]) -> list[int]:
    """Compute derivative coefficients.
    coeffs[0] is the highest degree coefficient.
    """
    n: int = len(coeffs)
    if n <= 1:
        return [0]
    result: list[int] = []
    i: int = 0
    degree: int = n - 1
    while i < n - 1:
        result.append(coeffs[i] * (degree - i))
        i += 1
    return result


def poly_add(a: list[int], b: list[int]) -> list[int]:
    """Add two polynomials (same length, padded with zeros)."""
    la: int = len(a)
    lb: int = len(b)
    length: int = la
    if lb > length:
        length = lb
    result: list[int] = []
    i: int = 0
    while i < length:
        va: int = 0
        if i < la:
            va = a[i]
        vb: int = 0
        if i < lb:
            vb = b[i]
        result.append(va + vb)
        i += 1
    return result


def poly_scale(coeffs: list[int], factor: int) -> list[int]:
    """Scale all coefficients by a factor."""
    result: list[int] = []
    for c in coeffs:
        result.append(c * factor)
    return result


def poly_degree(coeffs: list[int]) -> int:
    """Find the degree of the polynomial."""
    i: int = 0
    while i < len(coeffs):
        if coeffs[i] != 0:
            return len(coeffs) - 1 - i
        i += 1
    return 0


def test_module() -> int:
    """Test polynomial operations."""
    ok: int = 0

    # 2x^2 + 3x + 1 at x=2 => 8+6+1 = 15
    v: int = horner_eval([2, 3, 1], 2)
    if v == 15:
        ok += 1

    # x^3 at x=3 => 27
    v2: int = horner_eval([1, 0, 0, 0], 3)
    if v2 == 27:
        ok += 1

    # derivative of 3x^2 + 2x + 1 => 6x + 2 => [6, 2]
    d: list[int] = poly_derivative([3, 2, 1])
    if d == [6, 2]:
        ok += 1

    # add [1, 2, 3] + [4, 5, 6] = [5, 7, 9]
    s: list[int] = poly_add([1, 2, 3], [4, 5, 6])
    if s == [5, 7, 9]:
        ok += 1

    # scale [1, 2, 3] by 2 = [2, 4, 6]
    sc: list[int] = poly_scale([1, 2, 3], 2)
    if sc == [2, 4, 6]:
        ok += 1

    deg: int = poly_degree([0, 0, 3, 2, 1])
    if deg == 2:
        ok += 1

    return ok
