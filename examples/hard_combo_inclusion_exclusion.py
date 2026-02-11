"""Inclusion-exclusion principle for counting."""


def count_divisible(n: int, d: int) -> int:
    """Count integers 1..n divisible by d."""
    if d <= 0:
        return 0
    return n // d


def count_div_by_a_or_b(n: int, a: int, b: int) -> int:
    """Count integers 1..n divisible by a or b (inclusion-exclusion).
    |A union B| = |A| + |B| - |A intersect B|."""
    lcm_ab: int = lcm(a, b)
    return n // a + n // b - n // lcm_ab


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    while b != 0:
        t: int = b
        b = a % b
        a = t
    return a


def lcm(a: int, b: int) -> int:
    """Least common multiple."""
    g: int = gcd(a, b)
    if g == 0:
        return 0
    return a // g * b


def count_div_by_a_or_b_or_c(n: int, a: int, b: int, c: int) -> int:
    """Count 1..n divisible by a, b, or c using inclusion-exclusion."""
    ab: int = lcm(a, b)
    ac: int = lcm(a, c)
    bc: int = lcm(b, c)
    abc: int = lcm(ab, c)
    result: int = n // a + n // b + n // c
    result = result - n // ab - n // ac - n // bc
    result = result + n // abc
    return result


def euler_totient_ie(n: int) -> int:
    """Euler totient via inclusion-exclusion on prime factors.
    phi(n) = n * prod(1 - 1/p) for prime p dividing n.
    Returns phi(n)."""
    result: int = n
    p: int = 2
    temp: int = n
    while p * p <= temp:
        if temp % p == 0:
            while temp % p == 0:
                temp = temp // p
            result = result - result // p
        p = p + 1
    if temp > 1:
        result = result - result // temp
    return result


def test_module() -> int:
    """Test inclusion-exclusion functions."""
    ok: int = 0
    if count_divisible(20, 3) == 6:
        ok = ok + 1
    if count_div_by_a_or_b(20, 2, 3) == 13:
        ok = ok + 1
    if count_div_by_a_or_b_or_c(30, 2, 3, 5) == 22:
        ok = ok + 1
    if euler_totient_ie(12) == 4:
        ok = ok + 1
    if lcm(4, 6) == 12:
        ok = ok + 1
    return ok
