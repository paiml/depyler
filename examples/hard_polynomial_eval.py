"""Polynomial evaluation using Horner's method and derivative computation.

Tests: horner_eval, polynomial_derivative, polynomial_add.
"""


def horner_eval(coeffs: list[int], x: int) -> int:
    """Evaluate polynomial using Horner's method.
    
    coeffs[0] is highest degree coefficient.
    E.g., [2, 3, 1] represents 2x^2 + 3x + 1.
    """
    if len(coeffs) == 0:
        return 0
    result: int = coeffs[0]
    i: int = 1
    while i < len(coeffs):
        result = result * x + coeffs[i]
        i = i + 1
    return result


def polynomial_derivative(coeffs: list[int]) -> list[int]:
    """Compute derivative of polynomial.
    
    coeffs[0] is highest degree. Returns derivative coefficients.
    """
    n: int = len(coeffs)
    if n <= 1:
        return [0]
    result: list[int] = []
    i: int = 0
    degree: int = n - 1
    while i < n - 1:
        result.append(coeffs[i] * (degree - i))
        i = i + 1
    return result


def polynomial_add(a: list[int], b: list[int]) -> list[int]:
    """Add two polynomials (highest degree first)."""
    la: int = len(a)
    lb: int = len(b)
    max_len: int = la
    if lb > max_len:
        max_len = lb
    result: list[int] = []
    i: int = 0
    while i < max_len:
        va: int = 0
        vb: int = 0
        if i < la:
            va = a[la - 1 - i]
        if i < lb:
            vb = b[lb - 1 - i]
        result.append(va + vb)
        i = i + 1
    # Reverse result
    reversed_result: list[int] = []
    j: int = len(result) - 1
    while j >= 0:
        reversed_result.append(result[j])
        j = j - 1
    return reversed_result


def polynomial_degree(coeffs: list[int]) -> int:
    """Return degree of polynomial."""
    if len(coeffs) == 0:
        return -1
    return len(coeffs) - 1


def test_module() -> int:
    """Test polynomial operations."""
    ok: int = 0

    # 2x^2 + 3x + 1 at x=2 => 8 + 6 + 1 = 15
    if horner_eval([2, 3, 1], 2) == 15:
        ok = ok + 1

    # x^3 at x=3 => 27
    if horner_eval([1, 0, 0, 0], 3) == 27:
        ok = ok + 1

    if horner_eval([], 5) == 0:
        ok = ok + 1

    # d/dx (2x^2 + 3x + 1) = 4x + 3
    d: list[int] = polynomial_derivative([2, 3, 1])
    if d == [4, 3]:
        ok = ok + 1

    # d/dx (5) = 0
    if polynomial_derivative([5]) == [0]:
        ok = ok + 1

    # (x + 1) + (2x + 3) = 3x + 4
    s: list[int] = polynomial_add([1, 1], [2, 3])
    if s == [3, 4]:
        ok = ok + 1

    if polynomial_degree([2, 3, 1]) == 2:
        ok = ok + 1

    return ok
