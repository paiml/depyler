"""Euler's totient function and related number theory functions."""


def euler_phi(n: int) -> int:
    """Euler's totient function: count integers 1..n coprime to n."""
    if n <= 0:
        return 0
    result: int = n
    p: int = 2
    temp: int = n
    while p * p <= temp:
        if temp % p == 0:
            while temp % p == 0:
                temp = temp // p
            result = result - result // p
        p = p + 1
    if temp > 1:
        result = result - result // temp
    return result


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    while b != 0:
        t: int = b
        b = a % b
        a = t
    return a


def phi_brute(n: int) -> int:
    """Brute force totient for verification."""
    if n <= 0:
        return 0
    count: int = 0
    i: int = 1
    while i <= n:
        if gcd(i, n) == 1:
            count = count + 1
        i = i + 1
    return count


def sum_totient_range(lo: int, hi: int) -> int:
    """Sum of euler_phi for all integers in [lo, hi]."""
    total: int = 0
    n: int = lo
    while n <= hi:
        total = total + euler_phi(n)
        n = n + 1
    return total


def is_coprime(a: int, b: int) -> int:
    """Returns 1 if a and b are coprime."""
    if gcd(a, b) == 1:
        return 1
    return 0


def test_module() -> int:
    """Test Euler totient functions."""
    ok: int = 0
    if euler_phi(1) == 1:
        ok = ok + 1
    if euler_phi(10) == 4:
        ok = ok + 1
    if euler_phi(12) == 4:
        ok = ok + 1
    if euler_phi(10) == phi_brute(10):
        ok = ok + 1
    if is_coprime(8, 15) == 1:
        ok = ok + 1
    if is_coprime(6, 9) == 0:
        ok = ok + 1
    return ok
