"""Chinese Remainder Theorem for system of congruences."""


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    while b != 0:
        t: int = b
        b = a % b
        a = t
    return a


def extended_gcd(a: int, b: int) -> list[int]:
    """Extended GCD: returns [gcd, x, y]."""
    if a == 0:
        return [b, 0, 1]
    r: list[int] = extended_gcd(b % a, a)
    g: int = r[0]
    x1: int = r[1]
    y1: int = r[2]
    x: int = y1 - (b // a) * x1
    return [g, x, x1]


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse of a mod m."""
    r: list[int] = extended_gcd(a % m, m)
    if r[0] != 1:
        return 0 - 1
    return ((r[1] % m) + m) % m


def crt_two(r1: int, m1: int, r2: int, m2: int) -> list[int]:
    """CRT for two congruences. Returns [solution, lcm] or [-1, 0]."""
    g: int = gcd(m1, m2)
    if (r2 - r1) % g != 0:
        return [0 - 1, 0]
    lcm_val: int = m1 // g * m2
    inv: int = mod_inverse(m1 // g, m2 // g)
    diff: int = (r2 - r1) // g
    x: int = (r1 + m1 * (diff * inv % (m2 // g))) % lcm_val
    return [x, lcm_val]


def crt_solve(remainders: list[int], moduli: list[int]) -> int:
    """Solve system of congruences using CRT."""
    n: int = len(remainders)
    if n == 0:
        return 0
    cur_r: int = remainders[0]
    cur_m: int = moduli[0]
    i: int = 1
    while i < n:
        ri: int = remainders[i]
        mi: int = moduli[i]
        result: list[int] = crt_two(cur_r, cur_m, ri, mi)
        if result[1] == 0:
            return 0 - 1
        cur_r = result[0]
        cur_m = result[1]
        i = i + 1
    return cur_r


def test_module() -> int:
    """Test CRT functions."""
    ok: int = 0
    r1: list[int] = crt_two(2, 3, 3, 5)
    if r1[0] == 8:
        ok = ok + 1
    r2: list[int] = crt_two(1, 2, 2, 3)
    if r2[0] == 5:
        ok = ok + 1
    rems: list[int] = [2, 3, 2]
    mods: list[int] = [3, 5, 7]
    if crt_solve(rems, mods) == 23:
        ok = ok + 1
    rems2: list[int] = [1, 2]
    mods2: list[int] = [3, 5]
    if crt_solve(rems2, mods2) == 7:
        ok = ok + 1
    rems3: list[int] = [0, 0]
    mods3: list[int] = [4, 6]
    if crt_solve(rems3, mods3) == 0:
        ok = ok + 1
    return ok
