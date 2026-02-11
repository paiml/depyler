def gcd(a: int, b: int) -> int:
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


def frac_simplify(num: int, den: int) -> list[int]:
    if den == 0:
        return [0, 0]
    g: int = gcd(num, den)
    sn: int = num // g
    sd: int = den // g
    if sd < 0:
        sn = -sn
        sd = -sd
    return [sn, sd]


def frac_add(n1: int, d1: int, n2: int, d2: int) -> list[int]:
    num: int = n1 * d2 + n2 * d1
    den: int = d1 * d2
    return frac_simplify(num, den)


def frac_subtract(n1: int, d1: int, n2: int, d2: int) -> list[int]:
    num: int = n1 * d2 - n2 * d1
    den: int = d1 * d2
    return frac_simplify(num, den)


def frac_multiply(n1: int, d1: int, n2: int, d2: int) -> list[int]:
    num: int = n1 * n2
    den: int = d1 * d2
    return frac_simplify(num, den)


def test_module() -> int:
    passed: int = 0
    if frac_simplify(4, 8) == [1, 2]:
        passed = passed + 1
    if frac_add(1, 3, 1, 6) == [1, 2]:
        passed = passed + 1
    if frac_subtract(1, 2, 1, 3) == [1, 6]:
        passed = passed + 1
    if frac_multiply(2, 3, 3, 4) == [1, 2]:
        passed = passed + 1
    if gcd(12, 8) == 4:
        passed = passed + 1
    if gcd(7, 13) == 1:
        passed = passed + 1
    if frac_add(1, 2, 1, 2) == [1, 1]:
        passed = passed + 1
    if frac_simplify(0, 5) == [0, 1]:
        passed = passed + 1
    return passed
