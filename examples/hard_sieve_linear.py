"""Linear sieve for finding prime numbers."""


def linear_sieve(limit: int) -> list[int]:
    """Find all primes up to limit using linear sieve."""
    is_composite: list[int] = []
    i: int = 0
    while i <= limit:
        is_composite.append(0)
        i = i + 1
    primes: list[int] = []
    i = 2
    while i <= limit:
        if is_composite[i] == 0:
            primes.append(i)
        j: int = 0
        while j < len(primes):
            product: int = i * primes[j]
            if product > limit:
                j = len(primes)
            else:
                is_composite[product] = 1
                if i % primes[j] == 0:
                    j = len(primes)
                else:
                    j = j + 1
        i = i + 1
    return primes


def count_primes(limit: int) -> int:
    """Count primes up to limit."""
    primes: list[int] = linear_sieve(limit)
    return len(primes)


def is_prime(n: int) -> int:
    """Check if n is prime. Returns 1 if prime."""
    if n < 2:
        return 0
    i: int = 2
    while i * i <= n:
        if n % i == 0:
            return 0
        i = i + 1
    return 1


def nth_prime(n: int) -> int:
    """Find the nth prime (1-indexed)."""
    count: int = 0
    candidate: int = 2
    while count < n:
        if is_prime(candidate) == 1:
            count = count + 1
            if count == n:
                return candidate
        candidate = candidate + 1
    return candidate - 1


def sum_primes(limit: int) -> int:
    """Sum all primes up to limit."""
    primes: list[int] = linear_sieve(limit)
    total: int = 0
    i: int = 0
    while i < len(primes):
        total = total + primes[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test linear sieve."""
    passed: int = 0

    primes: list[int] = linear_sieve(30)
    if len(primes) == 10:
        passed = passed + 1

    if primes[0] == 2 and primes[1] == 3 and primes[2] == 5:
        passed = passed + 1

    if count_primes(100) == 25:
        passed = passed + 1

    if is_prime(17) == 1:
        passed = passed + 1

    if is_prime(15) == 0:
        passed = passed + 1

    if sum_primes(10) == 17:
        passed = passed + 1

    return passed
