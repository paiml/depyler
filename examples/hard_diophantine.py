def ext_gcd(a: int, b: int) -> list[int]:
    if b == 0:
        return [a, 1, 0]
    r: list[int] = ext_gcd(b, a % b)
    g: int = r[0]
    x1: int = r[1]
    y1: int = r[2]
    x: int = y1
    y: int = x1 - (a // b) * y1
    return [g, x, y]


def solve_linear_diophantine(a: int, b: int, c: int) -> list[int]:
    r: list[int] = ext_gcd(a, b)
    g: int = r[0]
    if g == 0:
        return [0, 0, 0]
    if c % g != 0:
        return [0, 0, 0]
    scale: int = c // g
    x0: int = r[1] * scale
    y0: int = r[2] * scale
    return [1, x0, y0]


def gcd_simple(a: int, b: int) -> int:
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
    if a == 0 or b == 0:
        return 0
    g: int = gcd_simple(a, b)
    val_a: int = a
    if val_a < 0:
        val_a = -val_a
    val_b: int = b
    if val_b < 0:
        val_b = -val_b
    return (val_a // g) * val_b


def test_module() -> int:
    passed: int = 0
    r: list[int] = ext_gcd(12, 8)
    if r[0] == 4:
        passed = passed + 1
    s: list[int] = solve_linear_diophantine(2, 3, 1)
    if s[0] == 1:
        passed = passed + 1
    check: int = 2 * s[1] + 3 * s[2]
    if check == 1:
        passed = passed + 1
    no_sol: list[int] = solve_linear_diophantine(4, 6, 3)
    if no_sol[0] == 0:
        passed = passed + 1
    if gcd_simple(48, 18) == 6:
        passed = passed + 1
    if lcm(4, 6) == 12:
        passed = passed + 1
    if lcm(0, 5) == 0:
        passed = passed + 1
    return passed
