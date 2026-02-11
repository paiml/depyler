"""Series computation: harmonic, geometric, alternating.

Tests: partial sums using integer arithmetic (scaled by 1000).
"""


def harmonic_sum_scaled(n: int) -> int:
    """Compute harmonic sum H(n) = 1 + 1/2 + ... + 1/n, scaled by 1000."""
    total: int = 0
    i: int = 1
    while i <= n:
        total = total + 1000 // i
        i = i + 1
    return total


def geometric_sum(a: int, r: int, n: int) -> int:
    """Compute geometric sum a + a*r + a*r^2 + ... + a*r^(n-1)."""
    total: int = 0
    current: int = a
    i: int = 0
    while i < n:
        total = total + current
        current = current * r
        i = i + 1
    return total


def alternating_sum(n: int) -> int:
    """Compute 1 - 2 + 3 - 4 + ... +/- n."""
    total: int = 0
    i: int = 1
    while i <= n:
        if i % 2 == 1:
            total = total + i
        else:
            total = total - i
        i = i + 1
    return total


def power_sum(n: int, p: int) -> int:
    """Compute 1^p + 2^p + ... + n^p."""
    total: int = 0
    i: int = 1
    while i <= n:
        val: int = 1
        j: int = 0
        while j < p:
            val = val * i
            j = j + 1
        total = total + val
        i = i + 1
    return total


def test_module() -> int:
    """Test series operations."""
    ok: int = 0
    if harmonic_sum_scaled(1) == 1000:
        ok = ok + 1
    if geometric_sum(1, 2, 4) == 15:
        ok = ok + 1
    if geometric_sum(3, 3, 3) == 39:
        ok = ok + 1
    if alternating_sum(4) == -2:
        ok = ok + 1
    if alternating_sum(5) == 3:
        ok = ok + 1
    if power_sum(3, 2) == 14:
        ok = ok + 1
    return ok
