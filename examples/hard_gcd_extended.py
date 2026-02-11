"""Extended GCD and modular inverse computation."""


def gcd(a: int, b: int) -> int:
    """Compute GCD using Euclidean algorithm."""
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    if a < 0:
        return -a
    return a


def extended_gcd(a: int, b: int) -> list[int]:
    """Extended GCD: returns [gcd, x, y] such that a*x + b*y = gcd."""
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
    result: list[int] = [old_r, old_s, old_t]
    return result


def mod_inverse(a: int, m: int) -> int:
    """Find modular inverse of a mod m. Returns -1 if none exists."""
    eg: list[int] = extended_gcd(a, m)
    g: int = eg[0]
    if g != 1:
        return -1
    x: int = eg[1]
    inv: int = ((x % m) + m) % m
    return inv


def lcm(a: int, b: int) -> int:
    """Compute LCM using GCD."""
    if a == 0 or b == 0:
        return 0
    g: int = gcd(a, b)
    abs_a: int = a
    if abs_a < 0:
        abs_a = -abs_a
    abs_b: int = b
    if abs_b < 0:
        abs_b = -abs_b
    return (abs_a // g) * abs_b


def test_module() -> int:
    passed: int = 0

    if gcd(48, 18) == 6:
        passed = passed + 1

    if gcd(100, 75) == 25:
        passed = passed + 1

    eg: list[int] = extended_gcd(35, 15)
    if eg[0] == 5:
        passed = passed + 1

    check: int = 35 * eg[1] + 15 * eg[2]
    if check == 5:
        passed = passed + 1

    if mod_inverse(3, 7) == 5:
        passed = passed + 1

    if mod_inverse(6, 9) == -1:
        passed = passed + 1

    if lcm(12, 18) == 36:
        passed = passed + 1

    if lcm(0, 5) == 0:
        passed = passed + 1

    return passed
