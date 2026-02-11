"""Integer factorization operations.

Implements algorithms for finding factors, prime factorization,
and related number theory operations.
"""


def count_factors(n: int) -> int:
    """Count the number of factors of n."""
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


def sum_of_factors(n: int) -> int:
    """Compute sum of all factors of n."""
    if n <= 0:
        return 0
    total: int = 0
    i: int = 1
    while i * i <= n:
        if n % i == 0:
            total = total + i
            other: int = n // i
            if other != i:
                total = total + other
        i = i + 1
    return total


def largest_prime_factor(n: int) -> int:
    """Find the largest prime factor of n."""
    if n <= 1:
        return 0
    largest: int = 0
    remaining: int = n
    d: int = 2
    while d * d <= remaining:
        while remaining % d == 0:
            largest = d
            remaining = remaining // d
        d = d + 1
    if remaining > 1:
        largest = remaining
    return largest


def prime_factorization_count(n: int) -> int:
    """Count the number of prime factors (with multiplicity)."""
    if n <= 1:
        return 0
    count: int = 0
    remaining: int = n
    d: int = 2
    while d * d <= remaining:
        while remaining % d == 0:
            count = count + 1
            remaining = remaining // d
        d = d + 1
    if remaining > 1:
        count = count + 1
    return count


def is_perfect_number(n: int) -> int:
    """Check if n is a perfect number. Returns 1 if yes."""
    if n <= 1:
        return 0
    factor_sum: int = sum_of_factors(n) - n
    if factor_sum == n:
        return 1
    return 0


def test_module() -> int:
    """Test factorization operations."""
    ok: int = 0

    fc: int = count_factors(12)
    if fc == 6:
        ok = ok + 1

    fs: int = sum_of_factors(12)
    if fs == 28:
        ok = ok + 1

    lpf: int = largest_prime_factor(60)
    if lpf == 5:
        ok = ok + 1

    pfc: int = prime_factorization_count(60)
    if pfc == 4:
        ok = ok + 1

    perfect: int = is_perfect_number(6)
    if perfect == 1:
        ok = ok + 1

    return ok
