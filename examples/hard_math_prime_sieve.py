"""Sieve of Eratosthenes and segmented sieve for prime generation."""


def sieve_of_eratosthenes(limit: int) -> list[int]:
    """Return all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []
    flags: list[int] = []
    i: int = 0
    while i <= limit:
        flags.append(1)
        i = i + 1
    flags[0] = 0
    flags[1] = 0
    p: int = 2
    while p * p <= limit:
        if flags[p] == 1:
            j: int = p * p
            while j <= limit:
                flags[j] = 0
                j = j + p
        p = p + 1
    primes: list[int] = []
    k: int = 2
    while k <= limit:
        if flags[k] == 1:
            primes.append(k)
        k = k + 1
    return primes


def count_primes_up_to(limit: int) -> int:
    """Count primes up to limit."""
    primes: list[int] = sieve_of_eratosthenes(limit)
    return len(primes)


def sum_primes_up_to(limit: int) -> int:
    """Sum all primes up to limit."""
    primes: list[int] = sieve_of_eratosthenes(limit)
    total: int = 0
    i: int = 0
    n: int = len(primes)
    while i < n:
        total = total + primes[i]
        i = i + 1
    return total


def nth_prime(n: int) -> int:
    """Return nth prime (1-indexed). Uses generous sieve bound."""
    if n <= 0:
        return 0
    bound: int = n * 12 + 100
    primes: list[int] = sieve_of_eratosthenes(bound)
    np: int = len(primes)
    if n <= np:
        return primes[n - 1]
    return 0


def test_module() -> int:
    """Test prime sieve functions."""
    ok: int = 0
    if count_primes_up_to(10) == 4:
        ok = ok + 1
    if count_primes_up_to(30) == 10:
        ok = ok + 1
    if sum_primes_up_to(10) == 17:
        ok = ok + 1
    if nth_prime(1) == 2:
        ok = ok + 1
    if nth_prime(5) == 11:
        ok = ok + 1
    if nth_prime(10) == 29:
        ok = ok + 1
    return ok
