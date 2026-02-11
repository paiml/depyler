"""Bisection method for root finding using integer-scaled arithmetic."""


def eval_poly(coeffs: list[int], x: int) -> int:
    """Evaluate polynomial. coeffs[i] = coeff of x^i."""
    n: int = len(coeffs)
    result: int = 0
    power: int = 1
    i: int = 0
    while i < n:
        result = result + coeffs[i] * power
        power = power * x
        i = i + 1
    return result


def sign_of(x: int) -> int:
    """Return sign: -1, 0, or 1."""
    if x > 0:
        return 1
    if x < 0:
        return 0 - 1
    return 0


def bisect_root(coeffs: list[int], lo: int, hi: int, iters: int) -> int:
    """Bisect to find root of polynomial in [lo, hi]."""
    i: int = 0
    while i < iters:
        mid: int = (lo + hi) // 2
        f_mid: int = eval_poly(coeffs, mid)
        if f_mid == 0:
            return mid
        f_lo: int = eval_poly(coeffs, lo)
        if sign_of(f_lo) * sign_of(f_mid) < 0:
            hi = mid
        else:
            lo = mid
        if hi - lo < 2:
            return mid
        i = i + 1
    return (lo + hi) // 2


def count_sign_changes(coeffs: list[int]) -> int:
    """Count sign changes in coefficients (Descartes rule bound)."""
    n: int = len(coeffs)
    changes: int = 0
    prev_sign: int = 0
    i: int = 0
    while i < n:
        if coeffs[i] != 0:
            cur: int = sign_of(coeffs[i])
            if prev_sign != 0 and cur != prev_sign:
                changes = changes + 1
            prev_sign = cur
        i = i + 1
    return changes


def test_module() -> int:
    """Test bisection method."""
    ok: int = 0
    coeffs: list[int] = [0 - 4, 0, 1]
    if eval_poly(coeffs, 0) == 0 - 4:
        ok = ok + 1
    root: int = bisect_root(coeffs, 0, 4, 50)
    if root == 2:
        ok = ok + 1
    if sign_of(0 - 5) == 0 - 1:
        ok = ok + 1
    if sign_of(0) == 0:
        ok = ok + 1
    if count_sign_changes([1, 0 - 1, 1]) == 2:
        ok = ok + 1
    return ok
