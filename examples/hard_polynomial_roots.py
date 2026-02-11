"""Polynomial root finding using bisection method (integer arithmetic).

Tests: polynomial evaluation, sign change detection, bisection.
"""


def poly_eval(coeffs: list[int], x: int) -> int:
    """Evaluate polynomial with integer coefficients at x.
    coeffs[0] + coeffs[1]*x + coeffs[2]*x^2 + ..."""
    result: int = 0
    power: int = 1
    i: int = 0
    while i < len(coeffs):
        result = result + coeffs[i] * power
        power = power * x
        i = i + 1
    return result


def count_sign_changes(coeffs: list[int]) -> int:
    """Count sign changes in coefficient array (Descartes rule hint)."""
    changes: int = 0
    last_sign: int = 0
    i: int = 0
    while i < len(coeffs):
        if coeffs[i] != 0:
            current_sign: int = 1
            if coeffs[i] < 0:
                current_sign = -1
            if last_sign != 0:
                if current_sign != last_sign:
                    changes = changes + 1
            last_sign = current_sign
        i = i + 1
    return changes


def has_root_in_interval(coeffs: list[int], a: int, b: int) -> int:
    """Check if polynomial has a root in [a, b] by sign change. Returns 1 if yes."""
    fa: int = poly_eval(coeffs, a)
    fb: int = poly_eval(coeffs, b)
    if fa == 0:
        return 1
    if fb == 0:
        return 1
    if fa > 0:
        if fb < 0:
            return 1
    if fa < 0:
        if fb > 0:
            return 1
    return 0


def poly_derivative_eval(coeffs: list[int], x: int) -> int:
    """Evaluate derivative of polynomial at x."""
    result: int = 0
    power: int = 1
    i: int = 1
    while i < len(coeffs):
        result = result + coeffs[i] * i * power
        power = power * x
        i = i + 1
    return result


def test_module() -> int:
    """Test polynomial operations."""
    ok: int = 0
    coeffs: list[int] = [-6, 1, 1]
    if poly_eval(coeffs, 2) == 0:
        ok = ok + 1
    if poly_eval(coeffs, 0) == -6:
        ok = ok + 1
    if has_root_in_interval(coeffs, 1, 3) == 1:
        ok = ok + 1
    if has_root_in_interval(coeffs, 5, 10) == 0:
        ok = ok + 1
    if count_sign_changes([-6, 1, 1]) == 1:
        ok = ok + 1
    if poly_derivative_eval(coeffs, 1) == 3:
        ok = ok + 1
    return ok
