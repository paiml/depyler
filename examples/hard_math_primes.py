"""Sieve of Eratosthenes, prime counting, nth prime."""


def sieve_of_eratosthenes(limit: int) -> list[int]:
    """Return list of all primes up to limit."""
    if limit < 2:
        return []
    is_prime: list[int] = []
    i: int = 0
    while i <= limit:
        is_prime.append(1)
        i = i + 1
    is_prime[0] = 0
    is_prime[1] = 0
    p: int = 2
    while p * p <= limit:
        if is_prime[p] == 1:
            multiple: int = p * p
            while multiple <= limit:
                is_prime[multiple] = 0
                multiple = multiple + p
        p = p + 1
    primes: list[int] = []
    j: int = 2
    while j <= limit:
        if is_prime[j] == 1:
            primes.append(j)
        j = j + 1
    return primes


def count_primes(n: int) -> int:
    """Count primes less than n."""
    if n <= 2:
        return 0
    primes: list[int] = sieve_of_eratosthenes(n - 1)
    return len(primes)


def is_prime(n: int) -> int:
    """Check if n is prime. Returns 1 or 0."""
    if n < 2:
        return 0
    if n == 2:
        return 1
    if n % 2 == 0:
        return 0
    d: int = 3
    while d * d <= n:
        if n % d == 0:
            return 0
        d = d + 2
    return 1


def nth_prime(n: int) -> int:
    """Find the nth prime (1-indexed). nth_prime(1) = 2."""
    if n <= 0:
        return -1
    count: int = 0
    candidate: int = 2
    while True:
        if is_prime(candidate) == 1:
            count = count + 1
            if count == n:
                return candidate
        candidate = candidate + 1


def test_module() -> int:
    passed: int = 0

    primes10: list[int] = sieve_of_eratosthenes(10)
    if primes10 == [2, 3, 5, 7]:
        passed = passed + 1

    primes1: list[int] = sieve_of_eratosthenes(1)
    if primes1 == []:
        passed = passed + 1

    if count_primes(10) == 4:
        passed = passed + 1

    if is_prime(17) == 1:
        passed = passed + 1

    if is_prime(15) == 0:
        passed = passed + 1

    if nth_prime(1) == 2:
        passed = passed + 1

    if nth_prime(5) == 11:
        passed = passed + 1

    if is_prime(2) == 1:
        passed = passed + 1

    return passed
