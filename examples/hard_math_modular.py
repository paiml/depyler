"""Modular arithmetic: modular power, modular inverse, Fermat's check."""


def mod_pow(base: int, exp: int, mod: int) -> int:
    """Compute (base^exp) % mod using fast exponentiation."""
    result: int = 1
    b: int = base % mod
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = (result * b) % mod
        e = e // 2
        b = (b * b) % mod
    return result


def mod_inverse(a: int, mod: int) -> int:
    """Compute modular inverse of a mod m using extended Euclidean.
    Returns -1 if inverse doesn't exist.
    """
    old_r: int = a
    r: int = mod
    old_s: int = 1
    s: int = 0
    while r != 0:
        q: int = old_r // r
        temp_r: int = r
        r = old_r - q * r
        old_r = temp_r
        temp_s: int = s
        s = old_s - q * s
        old_s = temp_s
    if old_r != 1:
        return -1
    if old_s < 0:
        old_s = old_s + mod
    return old_s


def fermat_check(p: int) -> int:
    """Check Fermat's little theorem: a^(p-1) = 1 mod p for a=2.
    Returns 1 if holds (probable prime), 0 otherwise.
    """
    if p <= 1:
        return 0
    if p == 2:
        return 1
    result: int = mod_pow(2, p - 1, p)
    if result == 1:
        return 1
    return 0


def mod_multiply(a: int, b: int, mod: int) -> int:
    """Compute (a * b) % mod safely."""
    return ((a % mod) * (b % mod)) % mod


def test_module() -> int:
    passed: int = 0

    if mod_pow(2, 10, 1000) == 24:
        passed = passed + 1

    if mod_pow(3, 5, 13) == 9:
        passed = passed + 1

    inv1: int = mod_inverse(3, 7)
    if (3 * inv1) % 7 == 1:
        passed = passed + 1

    inv2: int = mod_inverse(2, 4)
    if inv2 == -1:
        passed = passed + 1

    if fermat_check(17) == 1:
        passed = passed + 1

    if fermat_check(4) == 0:
        passed = passed + 1

    if mod_multiply(123456, 789012, 1000000) == 265472:
        passed = passed + 1

    if mod_pow(2, 0, 5) == 1:
        passed = passed + 1

    return passed
