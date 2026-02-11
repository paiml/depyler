"""Trial division factorization and factor counting."""


def smallest_factor(n: int) -> int:
    """Return smallest prime factor of n."""
    if n < 2:
        return n
    if n % 2 == 0:
        return 2
    i: int = 3
    while i * i <= n:
        if n % i == 0:
            return i
        i = i + 2
    return n


def count_prime_factors(n: int) -> int:
    """Count total prime factors with multiplicity."""
    if n < 2:
        return 0
    count: int = 0
    d: int = 2
    while d * d <= n:
        while n % d == 0:
            count = count + 1
            n = n // d
        d = d + 1
    if n > 1:
        count = count + 1
    return count


def count_distinct_factors(n: int) -> int:
    """Count distinct prime factors."""
    if n < 2:
        return 0
    count: int = 0
    d: int = 2
    while d * d <= n:
        if n % d == 0:
            count = count + 1
            while n % d == 0:
                n = n // d
        d = d + 1
    if n > 1:
        count = count + 1
    return count


def count_divisors(n: int) -> int:
    """Count all divisors of n."""
    if n <= 0:
        return 0
    count: int = 0
    i: int = 1
    while i * i <= n:
        if n % i == 0:
            count = count + 1
            if i != n // i:
                count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test factorization functions."""
    ok: int = 0
    if smallest_factor(15) == 3:
        ok = ok + 1
    if count_prime_factors(12) == 3:
        ok = ok + 1
    if count_distinct_factors(12) == 2:
        ok = ok + 1
    if count_divisors(12) == 6:
        ok = ok + 1
    if smallest_factor(97) == 97:
        ok = ok + 1
    return ok
