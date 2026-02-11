"""Mathematical series computation.

Implements various mathematical series using integer arithmetic
including harmonic approximations and convergent sums.
"""


def partial_sum(n: int) -> int:
    """Compute sum of first n natural numbers: 1 + 2 + ... + n."""
    result: int = n * (n + 1) // 2
    return result


def sum_of_squares(n: int) -> int:
    """Compute sum of squares: 1^2 + 2^2 + ... + n^2."""
    result: int = n * (n + 1) * (2 * n + 1) // 6
    return result


def sum_of_cubes(n: int) -> int:
    """Compute sum of cubes: 1^3 + 2^3 + ... + n^3."""
    half: int = n * (n + 1) // 2
    result: int = half * half
    return result


def alternating_series_sum(n: int) -> int:
    """Compute 1 - 2 + 3 - 4 + ... up to n terms (scaled by 1000)."""
    total: int = 0
    i: int = 1
    while i <= n:
        if i % 2 == 1:
            total = total + i * 1000
        else:
            total = total - i * 1000
        i = i + 1
    return total


def geometric_series_int(first: int, ratio: int, n: int) -> int:
    """Compute geometric series sum: a + ar + ar^2 + ... + ar^(n-1)."""
    total: int = 0
    term: int = first
    i: int = 0
    while i < n:
        total = total + term
        term = term * ratio
        i = i + 1
    return total


def power_sum(base: int, exponent: int) -> int:
    """Compute base raised to exponent using repeated multiplication."""
    result: int = 1
    i: int = 0
    while i < exponent:
        result = result * base
        i = i + 1
    return result


def test_module() -> int:
    """Test mathematical series operations."""
    ok: int = 0

    ps: int = partial_sum(10)
    if ps == 55:
        ok = ok + 1

    sq: int = sum_of_squares(5)
    if sq == 55:
        ok = ok + 1

    cu: int = sum_of_cubes(3)
    if cu == 36:
        ok = ok + 1

    alt: int = alternating_series_sum(4)
    if alt == -2000:
        ok = ok + 1

    geo: int = geometric_series_int(1, 2, 5)
    if geo == 31:
        ok = ok + 1

    pw: int = power_sum(2, 10)
    if pw == 1024:
        ok = ok + 1

    return ok
