"""Number sequences: arithmetic, geometric, and harmonic series sums.

Tests: arithmetic_sum, geometric_sum, harmonic_sum_approx, triangular_number.
"""


def arithmetic_sum(a: int, d: int, n: int) -> int:
    """Sum of first n terms of arithmetic series: a, a+d, a+2d, ..."""
    total: int = 0
    i: int = 0
    val: int = a
    while i < n:
        total = total + val
        val = val + d
        i = i + 1
    return total


def geometric_sum(a: int, r: int, n: int) -> int:
    """Sum of first n terms of geometric series: a, a*r, a*r^2, ..."""
    total: int = 0
    i: int = 0
    val: int = a
    while i < n:
        total = total + val
        val = val * r
        i = i + 1
    return total


def harmonic_sum_times_1000(n: int) -> int:
    """Approximate harmonic sum H(n) * 1000 using integer arithmetic.
    
    H(n) = 1 + 1/2 + 1/3 + ... + 1/n, multiplied by 1000 for precision.
    """
    total: int = 0
    i: int = 1
    while i <= n:
        total = total + 1000 // i
        i = i + 1
    return total


def triangular_number(n: int) -> int:
    """Compute nth triangular number: 1 + 2 + ... + n."""
    return n * (n + 1) // 2


def sum_of_squares(n: int) -> int:
    """Compute sum of squares: 1^2 + 2^2 + ... + n^2."""
    total: int = 0
    i: int = 1
    while i <= n:
        total = total + i * i
        i = i + 1
    return total


def sum_of_cubes(n: int) -> int:
    """Compute sum of cubes: 1^3 + 2^3 + ... + n^3."""
    total: int = 0
    i: int = 1
    while i <= n:
        total = total + i * i * i
        i = i + 1
    return total


def test_module() -> int:
    """Test number sequence operations."""
    ok: int = 0

    # 1 + 3 + 5 + 7 + 9 = 25
    if arithmetic_sum(1, 2, 5) == 25:
        ok = ok + 1

    # 1 + 2 + 4 + 8 = 15
    if geometric_sum(1, 2, 4) == 15:
        ok = ok + 1

    # 3 + 6 + 12 = 21
    if geometric_sum(3, 2, 3) == 21:
        ok = ok + 1

    if triangular_number(10) == 55:
        ok = ok + 1

    if sum_of_squares(3) == 14:
        ok = ok + 1

    if sum_of_cubes(3) == 36:
        ok = ok + 1

    # H(1) * 1000 = 1000
    if harmonic_sum_times_1000(1) == 1000:
        ok = ok + 1

    return ok
