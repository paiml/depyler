"""Modular exponentiation and modular inverse computations."""


def mod_pow(bv: int, exp: int, mod: int) -> int:
    """Fast modular exponentiation: bv^exp mod mod."""
    if mod == 1:
        return 0
    result: int = 1
    bv = bv % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * bv) % mod
        exp = exp // 2
        bv = (bv * bv) % mod
    return result


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    while b != 0:
        t: int = b
        b = a % b
        a = t
    return a


def extended_gcd(a: int, b: int) -> list[int]:
    """Extended GCD: returns [gcd, x, y] such that a*x + b*y = gcd."""
    if a == 0:
        return [b, 0, 1]
    r: list[int] = extended_gcd(b % a, a)
    g: int = r[0]
    x1: int = r[1]
    y1: int = r[2]
    x: int = y1 - (b // a) * x1
    y: int = x1
    return [g, x, y]


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse of a mod m. Returns -1 if not exists."""
    r: list[int] = extended_gcd(a % m, m)
    g: int = r[0]
    x: int = r[1]
    if g != 1:
        return 0 - 1
    return ((x % m) + m) % m


def mod_multiply(a: int, b: int, m: int) -> int:
    """Modular multiplication."""
    return ((a % m) * (b % m)) % m


def test_module() -> int:
    """Test modular arithmetic functions."""
    ok: int = 0
    if mod_pow(2, 10, 1000) == 24:
        ok = ok + 1
    if mod_pow(3, 13, 100) == 23:
        ok = ok + 1
    if gcd(12, 8) == 4:
        ok = ok + 1
    if mod_inverse(3, 7) == 5:
        ok = ok + 1
    inv: int = mod_inverse(3, 7)
    chk: int = (3 * inv) % 7
    if chk == 1:
        ok = ok + 1
    if mod_inverse(2, 4) == 0 - 1:
        ok = ok + 1
    return ok
