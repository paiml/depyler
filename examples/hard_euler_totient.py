# Euler's totient function, totient sum


def gcd(a: int, b: int) -> int:
    x: int = a
    y: int = b
    while y != 0:
        temp: int = y
        y = x % y
        x = temp
    return x


def euler_totient(n: int) -> int:
    if n <= 0:
        return 0
    if n == 1:
        return 1
    result: int = n
    p: int = 2
    m: int = n
    while p * p <= m:
        if m % p == 0:
            while m % p == 0:
                m = m // p
            result = result - result // p
        p = p + 1
    if m > 1:
        result = result - result // m
    return result


def totient_sum(n: int) -> int:
    # Sum of phi(k) for k = 1 to n
    total: int = 0
    k: int = 1
    while k <= n:
        total = total + euler_totient(k)
        k = k + 1
    return total


def totient_sieve(n: int) -> list[int]:
    # Compute phi(0), phi(1), ..., phi(n) using sieve
    phi: list[int] = []
    i: int = 0
    while i <= n:
        phi.append(i)
        i = i + 1
    p: int = 2
    while p <= n:
        if phi[p] == p:
            # p is prime
            j: int = p
            while j <= n:
                phi[j] = phi[j] - phi[j] // p
                j = j + p
        p = p + 1
    return phi


def coprime_count(n: int) -> int:
    # Count numbers 1..n-1 coprime to n (should equal totient)
    count: int = 0
    k: int = 1
    while k < n:
        if gcd(n, k) == 1:
            count = count + 1
        k = k + 1
    return count


def test_module() -> int:
    passed: int = 0

    # Test 1: phi(1) = 1
    if euler_totient(1) == 1:
        passed = passed + 1

    # Test 2: phi(prime) = prime - 1
    if euler_totient(7) == 6:
        passed = passed + 1

    # Test 3: phi(12) = 4
    if euler_totient(12) == 4:
        passed = passed + 1

    # Test 4: coprime count matches totient
    if coprime_count(12) == euler_totient(12):
        passed = passed + 1

    # Test 5: totient sum up to 5
    # phi(1)+phi(2)+phi(3)+phi(4)+phi(5) = 1+1+2+2+4 = 10
    if totient_sum(5) == 10:
        passed = passed + 1

    # Test 6: sieve matches direct
    sieve: list[int] = totient_sieve(10)
    if sieve[6] == euler_totient(6) and sieve[10] == euler_totient(10):
        passed = passed + 1

    # Test 7: phi(power of 2)
    # phi(8) = 4
    if euler_totient(8) == 4:
        passed = passed + 1

    return passed
