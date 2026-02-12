"""Numerical methods: Number theory computations.

Tests: modular arithmetic, prime testing, GCD/LCM, Euler totient,
modular exponentiation, integer factorization.
"""

from typing import List, Tuple


def gcd(a: int, b: int) -> int:
    """Euclidean GCD algorithm."""
    x: int = a
    y: int = b
    if x < 0:
        x = -x
    if y < 0:
        y = -y
    while y != 0:
        temp: int = y
        y = x % y
        x = temp
    return x


def lcm(a: int, b: int) -> int:
    """Least common multiple via GCD."""
    if a == 0 or b == 0:
        return 0
    g: int = gcd(a, b)
    return (a * b) // g


def is_prime(n: int) -> bool:
    """Trial division primality test."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    d: int = 3
    while d * d <= n:
        if n % d == 0:
            return False
        d += 2
    return True


def sieve_count(limit: int) -> int:
    """Count primes up to limit using sieve of Eratosthenes."""
    if limit < 2:
        return 0
    flags: List[int] = []
    i: int = 0
    while i <= limit:
        flags.append(1)
        i += 1
    flags[0] = 0
    flags[1] = 0
    p: int = 2
    while p * p <= limit:
        if flags[p] == 1:
            j: int = p * p
            while j <= limit:
                flags[j] = 0
                j += p
        p += 1
    count: int = 0
    for f in flags:
        count += f
    return count


def mod_pow(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation by repeated squaring."""
    if mod == 1:
        return 0
    result: int = 1
    b: int = base % mod
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = (result * b) % mod
        e = e // 2
        b = (b * b) % mod
    return result


def euler_totient(n: int) -> int:
    """Compute Euler's totient function phi(n)."""
    result: int = n
    p: int = 2
    val: int = n
    while p * p <= val:
        if val % p == 0:
            while val % p == 0:
                val = val // p
            result = result - result // p
        p += 1
    if val > 1:
        result = result - result // val
    return result


def extended_gcd(a: int, b: int) -> Tuple[int, int, int]:
    """Extended GCD: returns (gcd, x, y) such that a*x + b*y = gcd."""
    if b == 0:
        return (a, 1, 0)
    old_r: int = a
    r: int = b
    old_s: int = 1
    s: int = 0
    old_t: int = 0
    t: int = 1
    while r != 0:
        quotient: int = old_r // r
        temp_r: int = r
        r = old_r - quotient * r
        old_r = temp_r
        temp_s: int = s
        s = old_s - quotient * s
        old_s = temp_s
        temp_t: int = t
        t = old_t - quotient * t
        old_t = temp_t
    return (old_r, old_s, old_t)


def factorize(n: int) -> List[int]:
    """Return prime factors of n in ascending order."""
    factors: List[int] = []
    if n <= 1:
        return factors
    d: int = 2
    val: int = n
    while d * d <= val:
        while val % d == 0:
            factors.append(d)
            val = val // d
        d += 1
    if val > 1:
        factors.append(val)
    return factors


def test_number_theory() -> bool:
    """Test number theory functions."""
    ok: bool = True
    g: int = gcd(12, 8)
    if g != 4:
        ok = False
    l: int = lcm(4, 6)
    if l != 12:
        ok = False
    if not is_prime(17):
        ok = False
    if is_prime(15):
        ok = False
    pc: int = sieve_count(100)
    if pc != 25:
        ok = False
    mp: int = mod_pow(2, 10, 1000)
    if mp != 24:
        ok = False
    return ok
