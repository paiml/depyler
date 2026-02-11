# Mobius function, Mobius inversion


def mobius(n: int) -> int:
    # mu(n): 0 if n has squared prime factor, (-1)^k if n is product of k distinct primes
    if n == 1:
        return 1
    prime_count: int = 0
    m: int = n
    p: int = 2
    while p * p <= m:
        if m % p == 0:
            m = m // p
            if m % p == 0:
                return 0
            prime_count = prime_count + 1
        p = p + 1
    if m > 1:
        prime_count = prime_count + 1
    if prime_count % 2 == 0:
        return 1
    return -1


def mobius_sieve(n: int) -> list[int]:
    # Compute mu(0), mu(1), ..., mu(n) using sieve
    mu: list[int] = []
    i: int = 0
    while i <= n:
        mu.append(0)
        i = i + 1
    mu[1] = 1
    k: int = 1
    while k <= n:
        if mu[k] != 0:
            j: int = 2 * k
            while j <= n:
                mu[j] = mu[j] - mu[k]
                j = j + k
        k = k + 1
    return mu


def sum_mobius(n: int) -> int:
    # Mertens function M(n) = sum of mu(k) for k=1..n
    total: int = 0
    k: int = 1
    while k <= n:
        total = total + mobius(k)
        k = k + 1
    return total


def divisor_sum(n: int) -> int:
    # Sum of divisors of n
    total: int = 0
    d: int = 1
    while d * d <= n:
        if n % d == 0:
            total = total + d
            if d != n // d:
                total = total + n // d
        d = d + 1
    return total


def euler_totient_via_mobius(n: int) -> int:
    # phi(n) = sum over d|n of mu(d) * (n/d)
    total: int = 0
    d: int = 1
    while d <= n:
        if n % d == 0:
            total = total + mobius(d) * (n // d)
        d = d + 1
    return total


def test_module() -> int:
    passed: int = 0

    # Test 1: mu(1) = 1
    if mobius(1) == 1:
        passed = passed + 1

    # Test 2: mu(prime) = -1
    if mobius(7) == -1:
        passed = passed + 1

    # Test 3: mu(4) = 0 (has squared factor)
    if mobius(4) == 0:
        passed = passed + 1

    # Test 4: mu(6) = 1 (2*3, two distinct primes)
    if mobius(6) == 1:
        passed = passed + 1

    # Test 5: mu(30) = -1 (2*3*5, three distinct primes)
    if mobius(30) == -1:
        passed = passed + 1

    # Test 6: Mertens function M(10)
    # mu: 1,1,-1,-1,0,-1,1,-1,0,0,-1 -> sum = -1
    if sum_mobius(10) == -1:
        passed = passed + 1

    # Test 7: sieve matches direct
    sieve: list[int] = mobius_sieve(10)
    ok: int = 1
    j: int = 1
    while j <= 10:
        if sieve[j] != mobius(j):
            ok = 0
        j = j + 1
    if ok == 1:
        passed = passed + 1

    # Test 8: phi via Mobius inversion
    if euler_totient_via_mobius(12) == 4:
        passed = passed + 1

    return passed
