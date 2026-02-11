"""Abundant number detection and counting.

Tests: is abundant, abundance value, count abundant, sum abundant in range.
"""


def proper_divisor_sum(n: int) -> int:
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


def is_abundant_number(n: int) -> int:
    """Returns 1 if n is abundant."""
    if n <= 1:
        return 0
    if proper_divisor_sum(n) > n:
        return 1
    return 0


def abundance_value(n: int) -> int:
    """How much the divisor sum exceeds n. 0 if not abundant."""
    if n <= 1:
        return 0
    s: int = proper_divisor_sum(n)
    if s > n:
        return s - n
    return 0


def count_abundant_in_range(lo: int, hi: int) -> int:
    """Count abundant numbers in [lo, hi]."""
    count: int = 0
    n: int = lo
    while n <= hi:
        if is_abundant_number(n) == 1:
            count = count + 1
        n = n + 1
    return count


def sum_abundant_in_range(lo: int, hi: int) -> int:
    """Sum of all abundant numbers in [lo, hi]."""
    total: int = 0
    n: int = lo
    while n <= hi:
        if is_abundant_number(n) == 1:
            total = total + n
        n = n + 1
    return total


def test_module() -> int:
    """Test abundant numbers."""
    ok: int = 0
    if is_abundant_number(12) == 1:
        ok = ok + 1
    if is_abundant_number(7) == 0:
        ok = ok + 1
    if abundance_value(12) == 4:
        ok = ok + 1
    if abundance_value(8) == 0:
        ok = ok + 1
    if count_abundant_in_range(1, 20) == 1:
        ok = ok + 1
    if sum_abundant_in_range(1, 20) == 12:
        ok = ok + 1
    return ok
