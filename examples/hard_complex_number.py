def complex_add(ar: int, ai: int, br: int, bi: int) -> list[int]:
    return [ar + br, ai + bi]


def complex_multiply(ar: int, ai: int, br: int, bi: int) -> list[int]:
    real: int = ar * br - ai * bi
    imag: int = ar * bi + ai * br
    return [real, imag]


def complex_conjugate(ar: int, ai: int) -> list[int]:
    return [ar, -ai]


def complex_magnitude_squared(ar: int, ai: int) -> int:
    return ar * ar + ai * ai


def complex_subtract(ar: int, ai: int, br: int, bi: int) -> list[int]:
    return [ar - br, ai - bi]


def complex_power(ar: int, ai: int, n: int) -> list[int]:
    rr: int = 1
    ri: int = 0
    count: int = 0
    while count < n:
        new_r: int = rr * ar - ri * ai
        new_i: int = rr * ai + ri * ar
        rr = new_r
        ri = new_i
        count = count + 1
    return [rr, ri]


def test_module() -> int:
    passed: int = 0
    if complex_add(1, 2, 3, 4) == [4, 6]:
        passed = passed + 1
    if complex_multiply(1, 2, 3, 4) == [-5, 10]:
        passed = passed + 1
    if complex_conjugate(3, 4) == [3, -4]:
        passed = passed + 1
    if complex_magnitude_squared(3, 4) == 25:
        passed = passed + 1
    if complex_subtract(5, 3, 2, 1) == [3, 2]:
        passed = passed + 1
    if complex_power(0, 1, 2) == [-1, 0]:
        passed = passed + 1
    if complex_power(1, 0, 5) == [1, 0]:
        passed = passed + 1
    if complex_multiply(0, 1, 0, 1) == [-1, 0]:
        passed = passed + 1
    return passed
