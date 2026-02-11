"""Amicable pair detection.

Tests: amicable check, find amicable partner, count amicable in range.
"""


def sum_proper_divisors(n: int) -> int:
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


def is_amicable_pair(a: int, b: int) -> int:
    """Returns 1 if a and b are an amicable pair."""
    if a == b:
        return 0
    if a <= 0 or b <= 0:
        return 0
    if sum_proper_divisors(a) == b and sum_proper_divisors(b) == a:
        return 1
    return 0


def find_amicable_partner(n: int) -> int:
    """Find amicable partner of n, or -1 if none."""
    if n <= 0:
        return -1
    partner: int = sum_proper_divisors(n)
    if partner == n or partner <= 0:
        return -1
    if sum_proper_divisors(partner) == n:
        return partner
    return -1


def count_amicable_in_range(hi: int) -> int:
    """Count numbers in [1, hi] that are part of an amicable pair."""
    count: int = 0
    n: int = 2
    while n <= hi:
        partner: int = find_amicable_partner(n)
        if partner > 0 and partner <= hi:
            count = count + 1
        n = n + 1
    return count


def test_module() -> int:
    """Test amicable pairs."""
    ok: int = 0
    if is_amicable_pair(220, 284) == 1:
        ok = ok + 1
    if is_amicable_pair(220, 220) == 0:
        ok = ok + 1
    if is_amicable_pair(10, 20) == 0:
        ok = ok + 1
    if find_amicable_partner(220) == 284:
        ok = ok + 1
    if find_amicable_partner(284) == 220:
        ok = ok + 1
    if find_amicable_partner(10) == -1:
        ok = ok + 1
    return ok
