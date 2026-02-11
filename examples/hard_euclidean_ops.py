"""Extended Euclidean algorithm and modular arithmetic.

Tests: extended GCD, modular inverse, Chinese remainder (2 equations), modular exponentiation.
"""


def extended_gcd(a: int, b: int) -> list[int]:
    """Extended Euclidean algorithm returning [gcd, x, y] where a*x + b*y = gcd."""
    if b == 0:
        return [a, 1, 0]
    old_r: int = a
    r: int = b
    old_s: int = 1
    s: int = 0
    old_t: int = 0
    t: int = 1
    while r != 0:
        q: int = old_r // r
        temp_r: int = r
        r = old_r - q * r
        old_r = temp_r
        temp_s: int = s
        s = old_s - q * s
        old_s = temp_s
        temp_t: int = t
        t = old_t - q * t
        old_t = temp_t
    return [old_r, old_s, old_t]


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse of a mod m. Returns -1 if no inverse exists."""
    result: list[int] = extended_gcd(a, m)
    g: int = result[0]
    x: int = result[1]
    if g != 1:
        return -1
    return ((x % m) + m) % m


def mod_pow(base: int, exp: int, modulus: int) -> int:
    """Modular exponentiation: base^exp mod modulus."""
    result: int = 1
    b: int = base % modulus
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = (result * b) % modulus
        b = (b * b) % modulus
        e = e // 2
    return result


def crt_two(r1: int, m1: int, r2: int, m2: int) -> int:
    """Chinese Remainder Theorem for two equations.
    
    Find x such that x = r1 (mod m1) and x = r2 (mod m2).
    Returns x or -1 if no solution.
    """
    result: list[int] = extended_gcd(m1, m2)
    g: int = result[0]
    p: int = result[1]
    if (r2 - r1) % g != 0:
        return -1
    lcm: int = m1 * (m2 // g)
    x: int = (r1 + m1 * ((r2 - r1) // g) * p) % lcm
    if x < 0:
        x = x + lcm
    return x


def test_module() -> int:
    """Test extended Euclidean operations."""
    ok: int = 0
    res: list[int] = extended_gcd(35, 15)
    if res[0] == 5:
        ok = ok + 1
    if mod_inverse(3, 11) == 4:
        ok = ok + 1
    if mod_inverse(2, 4) == -1:
        ok = ok + 1
    if mod_pow(2, 10, 1000) == 24:
        ok = ok + 1
    if mod_pow(3, 5, 13) == 9:
        ok = ok + 1
    if crt_two(2, 3, 3, 5) == 8:
        ok = ok + 1
    if crt_two(1, 4, 3, 6) == 9:
        ok = ok + 1
    return ok
