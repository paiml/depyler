"""Modular exponentiation variants.

Tests: modpow, modular inverse, power tower mod, repeated squaring sum.
"""


def mod_pow(base: int, exp: int, modulus: int) -> int:
    """Compute (base^exp) % modulus using repeated squaring."""
    if modulus == 1:
        return 0
    result: int = 1
    b: int = base % modulus
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = (result * b) % modulus
        e = e // 2
        b = (b * b) % modulus
    return result


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse of a mod m using extended Euclidean. Returns -1 if none."""
    g: int = a
    x: int = 1
    y: int = 0
    g2: int = m
    x2: int = 0
    y2: int = 1
    while g2 > 0:
        q: int = g // g2
        tg: int = g2
        tx: int = x2
        ty: int = y2
        g2 = g - q * g2
        x2 = x - q * x2
        y2 = y - q * y2
        g = tg
        x = tx
        y = ty
    if g != 1:
        return -1
    result: int = x % m
    if result < 0:
        result = result + m
    return result


def sum_of_powers_mod(n: int, exp: int, modulus: int) -> int:
    """Sum of (i^exp) mod modulus for i from 1 to n."""
    total: int = 0
    i: int = 1
    while i <= n:
        total = (total + mod_pow(i, exp, modulus)) % modulus
        i = i + 1
    return total


def power_tower_mod(base: int, height: int, modulus: int) -> int:
    """Compute base^base^...^base (height times) mod modulus (simplified)."""
    if height == 0:
        return 1 % modulus
    result: int = base % modulus
    i: int = 1
    while i < height:
        result = mod_pow(base, result, modulus)
        i = i + 1
    return result


def test_module() -> int:
    """Test modular exponentiation operations."""
    ok: int = 0
    if mod_pow(2, 10, 1000) == 24:
        ok = ok + 1
    if mod_pow(3, 5, 13) == 9:
        ok = ok + 1
    if mod_inverse(3, 7) == 5:
        ok = ok + 1
    if mod_inverse(2, 6) == -1:
        ok = ok + 1
    if sum_of_powers_mod(5, 2, 100) == 55:
        ok = ok + 1
    if power_tower_mod(2, 2, 1000) == 4:
        ok = ok + 1
    return ok
