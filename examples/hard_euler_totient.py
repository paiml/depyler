"""Euler's totient function phi(n)."""


def gcd(a: int, b: int) -> int:
    """Compute greatest common divisor."""
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    return a


def euler_totient(n: int) -> int:
    """Compute Euler's totient phi(n)."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    result: int = n
    p: int = 2
    temp_n: int = n
    while p * p <= temp_n:
        if temp_n % p == 0:
            while temp_n % p == 0:
                temp_n = temp_n // p
            result = result - result // p
        p = p + 1
    if temp_n > 1:
        result = result - result // temp_n
    return result


def totient_brute(n: int) -> int:
    """Brute force totient by counting coprimes."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    count: int = 0
    i: int = 1
    while i < n:
        if gcd(i, n) == 1:
            count = count + 1
        i = i + 1
    return count


def sum_totients(n: int) -> int:
    """Sum of totients from 1 to n."""
    total: int = 0
    i: int = 1
    while i <= n:
        total = total + euler_totient(i)
        i = i + 1
    return total


def is_prime_via_totient(n: int) -> int:
    """If phi(n) == n-1, then n is prime. Returns 1 if prime."""
    if n <= 1:
        return 0
    if euler_totient(n) == n - 1:
        return 1
    return 0


def test_module() -> int:
    """Test Euler totient computations."""
    ok: int = 0
    if euler_totient(1) == 1:
        ok = ok + 1
    if euler_totient(2) == 1:
        ok = ok + 1
    if euler_totient(6) == 2:
        ok = ok + 1
    if euler_totient(10) == 4:
        ok = ok + 1
    if euler_totient(12) == 4:
        ok = ok + 1
    if is_prime_via_totient(7) == 1:
        ok = ok + 1
    if is_prime_via_totient(6) == 0:
        ok = ok + 1
    if totient_brute(10) == 4:
        ok = ok + 1
    return ok
