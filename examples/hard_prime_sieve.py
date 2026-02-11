"""Prime number operations.

Tests: sieve, primality check, nth prime, prime factorization count.
"""


def sieve_count(limit: int) -> int:
    """Count primes up to limit using sieve."""
    if limit < 2:
        return 0
    is_prime: list[bool] = [True] * (limit + 1)
    is_prime[0] = False
    is_prime[1] = False
    i: int = 2
    while i * i <= limit:
        if is_prime[i]:
            j: int = i * i
            while j <= limit:
                is_prime[j] = False
                j = j + i
        i = i + 1
    count: int = 0
    k: int = 2
    while k <= limit:
        if is_prime[k]:
            count = count + 1
        k = k + 1
    return count


def is_prime_check(n: int) -> int:
    """Check if n is prime. Returns 1 for prime, 0 otherwise."""
    if n < 2:
        return 0
    if n == 2:
        return 1
    if n % 2 == 0:
        return 0
    i: int = 3
    while i * i <= n:
        if n % i == 0:
            return 0
        i = i + 2
    return 1


def nth_prime(n: int) -> int:
    """Find the nth prime number (1-indexed)."""
    if n <= 0:
        return 0
    count: int = 0
    candidate: int = 2
    while count < n:
        if is_prime_check(candidate) == 1:
            count = count + 1
            if count == n:
                return candidate
        candidate = candidate + 1
    return candidate - 1


def prime_factor_count(n: int) -> int:
    """Count the number of prime factors (with multiplicity)."""
    if n <= 1:
        return 0
    count: int = 0
    d: int = 2
    val: int = n
    while d * d <= val:
        while val % d == 0:
            count = count + 1
            val = val // d
        d = d + 1
    if val > 1:
        count = count + 1
    return count


def test_module() -> int:
    """Test prime operations."""
    ok: int = 0
    if sieve_count(10) == 4:
        ok = ok + 1
    if sieve_count(30) == 10:
        ok = ok + 1
    if is_prime_check(17) == 1:
        ok = ok + 1
    if is_prime_check(15) == 0:
        ok = ok + 1
    if nth_prime(1) == 2:
        ok = ok + 1
    if nth_prime(5) == 11:
        ok = ok + 1
    if prime_factor_count(12) == 3:
        ok = ok + 1
    if prime_factor_count(7) == 1:
        ok = ok + 1
    return ok
