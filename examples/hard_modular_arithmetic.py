def mod_exp(base: int, exp: int, mod: int) -> int:
    result: int = 1
    b: int = base % mod
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = (result * b) % mod
        e = e // 2
        b = (b * b) % mod
    return result


def extended_gcd(a: int, b: int) -> list[int]:
    if b == 0:
        return [a, 1, 0]
    r: list[int] = extended_gcd(b, a % b)
    g: int = r[0]
    x1: int = r[1]
    y1: int = r[2]
    x: int = y1
    y: int = x1 - (a // b) * y1
    return [g, x, y]


def mod_inverse(a: int, m: int) -> int:
    r: list[int] = extended_gcd(a, m)
    if r[0] != 1:
        return -1
    result: int = r[1] % m
    if result < 0:
        result = result + m
    return result


def chinese_remainder_two(r1: int, m1: int, r2: int, m2: int) -> int:
    inv: int = mod_inverse(m1, m2)
    if inv == -1:
        return -1
    result: int = r1 + m1 * ((r2 - r1) * inv % m2)
    if result < 0:
        result = result + m1 * m2
    return result % (m1 * m2)


def test_module() -> int:
    passed: int = 0
    if mod_exp(2, 10, 1000) == 1024:
        passed = passed + 1
    if mod_exp(3, 5, 13) == 9:
        passed = passed + 1
    if mod_inverse(3, 7) == 5:
        passed = passed + 1
    if mod_inverse(2, 4) == -1:
        passed = passed + 1
    r: list[int] = extended_gcd(12, 8)
    if r[0] == 4:
        passed = passed + 1
    if chinese_remainder_two(2, 3, 3, 5) == 8:
        passed = passed + 1
    if mod_exp(5, 0, 7) == 1:
        passed = passed + 1
    return passed
