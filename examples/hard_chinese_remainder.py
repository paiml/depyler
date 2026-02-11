# Chinese remainder theorem solver


def gcd(a: int, b: int) -> int:
    x: int = a
    y: int = b
    while y != 0:
        temp: int = y
        y = x % y
        x = temp
    return x


def extended_gcd(a: int, b: int) -> list[int]:
    # Returns [gcd, x, y] such that a*x + b*y = gcd(a,b)
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
    result: list[int] = extended_gcd(a % m, m)
    if result[0] != 1:
        return -1
    return result[1] % m


def crt_two(r1: int, m1: int, r2: int, m2: int) -> list[int]:
    # Solve x = r1 (mod m1), x = r2 (mod m2)
    # Returns [solution, lcm(m1, m2)] or [-1, -1] if no solution
    g: int = gcd(m1, m2)
    if (r2 - r1) % g != 0:
        return [-1, -1]
    lcm: int = m1 * (m2 // g)
    inv: int = mod_inverse(m1 // g, m2 // g)
    diff: int = (r2 - r1) // g
    x: int = (r1 + m1 * (diff * inv % (m2 // g))) % lcm
    return [x, lcm]


def crt_solve(remainders: list[int], moduli: list[int]) -> int:
    # Solve system of congruences
    if len(remainders) == 0:
        return 0
    current_r: int = remainders[0]
    current_m: int = moduli[0]
    i: int = 1
    while i < len(remainders):
        result: list[int] = crt_two(current_r, current_m, remainders[i], moduli[i])
        if result[0] == -1:
            return -1
        current_r = result[0]
        current_m = result[1]
        i = i + 1
    return current_r


def test_module() -> int:
    passed: int = 0

    # Test 1: gcd
    if gcd(12, 18) == 6:
        passed = passed + 1

    # Test 2: extended gcd
    res: list[int] = extended_gcd(35, 15)
    if res[0] == 5:
        passed = passed + 1

    # Test 3: mod inverse
    if mod_inverse(3, 7) == 5:
        passed = passed + 1

    # Test 4: x = 2 mod 3, x = 3 mod 5 => x = 8
    sol: list[int] = crt_two(2, 3, 3, 5)
    if sol[0] == 8:
        passed = passed + 1

    # Test 5: classic CRT: x=2(mod3), x=3(mod5), x=2(mod7) => x=23
    rems: list[int] = [2, 3, 2]
    mods: list[int] = [3, 5, 7]
    if crt_solve(rems, mods) == 23:
        passed = passed + 1

    # Test 6: single congruence
    rems2: list[int] = [5]
    mods2: list[int] = [11]
    if crt_solve(rems2, mods2) == 5:
        passed = passed + 1

    # Test 7: mod inverse verification
    inv: int = mod_inverse(3, 7)
    if (3 * inv) % 7 == 1:
        passed = passed + 1

    return passed
