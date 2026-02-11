"""Perfect number detection and related operations.

Tests: is perfect, sum of divisors, count perfect in range, deficiency.
"""


def sum_of_divisors(n: int) -> int:
    """Sum of proper divisors of n."""
    if n <= 1:
        return 0
    total: int = 1
    i: int = 2
    while i * i <= n:
        if n % i == 0:
            total = total + i
            if i != n // i:
                total = total + n // i
        i = i + 1
    return total


def is_perfect(n: int) -> int:
    """Returns 1 if n is a perfect number, 0 otherwise."""
    if n <= 1:
        return 0
    if sum_of_divisors(n) == n:
        return 1
    return 0


def count_perfect_in_range(lo: int, hi: int) -> int:
    """Count perfect numbers in [lo, hi]."""
    count: int = 0
    n: int = lo
    while n <= hi:
        if is_perfect(n) == 1:
            count = count + 1
        n = n + 1
    return count


def deficiency(n: int) -> int:
    """Deficiency: n - sum_of_divisors(n). Positive means deficient."""
    if n <= 0:
        return 0
    return n - sum_of_divisors(n)


def is_abundant(n: int) -> int:
    """Returns 1 if n is abundant (sum of divisors > n)."""
    if n <= 1:
        return 0
    if sum_of_divisors(n) > n:
        return 1
    return 0


def test_module() -> int:
    """Test perfect numbers."""
    ok: int = 0
    if is_perfect(6) == 1:
        ok = ok + 1
    if is_perfect(28) == 1:
        ok = ok + 1
    if is_perfect(12) == 0:
        ok = ok + 1
    if sum_of_divisors(12) == 16:
        ok = ok + 1
    if count_perfect_in_range(1, 30) == 2:
        ok = ok + 1
    if deficiency(8) == 1:
        ok = ok + 1
    if is_abundant(12) == 1:
        ok = ok + 1
    return ok
