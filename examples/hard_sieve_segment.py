"""Segmented sieve of Eratosthenes for prime generation."""


def simple_sieve(limit: int) -> list[int]:
    """Return list of primes up to limit using basic sieve."""
    if limit < 2:
        result: list[int] = []
        return result
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
            mult: int = p * p
            while mult <= limit:
                is_prime[mult] = 0
                mult = mult + p
        p = p + 1
    primes: list[int] = []
    j: int = 2
    while j <= limit:
        if is_prime[j] == 1:
            primes.append(j)
        j = j + 1
    return primes


def count_primes_range(low: int, high: int) -> int:
    """Count primes in range [low, high] using segmented sieve approach."""
    if high < 2:
        return 0
    if low < 2:
        low = 2
    base_primes: list[int] = simple_sieve(150)
    size: int = high - low + 1
    segment: list[int] = []
    k: int = 0
    while k < size:
        segment.append(1)
        k = k + 1
    bp: int = 0
    num_base: int = len(base_primes)
    while bp < num_base:
        p: int = base_primes[bp]
        start: int = p * p
        if start < low:
            rem: int = low % p
            if rem == 0:
                start = low
            else:
                start = low + (p - rem)
        while start <= high:
            idx: int = start - low
            segment[idx] = 0
            start = start + p
        bp = bp + 1
    count: int = 0
    m: int = 0
    while m < size:
        if segment[m] == 1:
            count = count + 1
        m = m + 1
    return count


def is_prime(n: int) -> int:
    """Check if n is prime. Returns 1 for prime, 0 otherwise."""
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


def test_module() -> int:
    passed: int = 0

    primes10: list[int] = simple_sieve(10)
    if len(primes10) == 4:
        passed = passed + 1

    primes30: list[int] = simple_sieve(30)
    if len(primes30) == 10:
        passed = passed + 1

    if count_primes_range(10, 30) == 6:
        passed = passed + 1

    if count_primes_range(1, 10) == 4:
        passed = passed + 1

    if is_prime(17) == 1:
        passed = passed + 1

    if is_prime(15) == 0:
        passed = passed + 1

    if count_primes_range(100, 110) == 4:
        passed = passed + 1

    return passed
