def mat2_determinant(a: int, b: int, c: int, d: int) -> int:
    return a * d - b * c


def mat2_inverse_scaled(a: int, b: int, c: int, d: int) -> list[int]:
    det: int = mat2_determinant(a, b, c, d)
    if det == 0:
        return [0, 0, 0, 0]
    return [d, -b, -c, a]


def mat2_is_identity(a: int, b: int, c: int, d: int) -> int:
    if a == 1 and b == 0 and c == 0 and d == 1:
        return 1
    return 0


def mat2_multiply(a1: int, b1: int, c1: int, d1: int, a2: int, b2: int, c2: int, d2: int) -> list[int]:
    ra: int = a1 * a2 + b1 * c2
    rb: int = a1 * b2 + b1 * d2
    rc: int = c1 * a2 + d1 * c2
    rd: int = c1 * b2 + d1 * d2
    return [ra, rb, rc, rd]


def mat2_trace(a: int, b: int, c: int, d: int) -> int:
    return a + d


def test_module() -> int:
    passed: int = 0
    if mat2_determinant(1, 2, 3, 4) == -2:
        passed = passed + 1
    if mat2_determinant(2, 0, 0, 3) == 6:
        passed = passed + 1
    inv: list[int] = mat2_inverse_scaled(1, 2, 3, 4)
    if inv == [4, -2, -3, 1]:
        passed = passed + 1
    if mat2_inverse_scaled(1, 1, 1, 1) == [0, 0, 0, 0]:
        passed = passed + 1
    if mat2_is_identity(1, 0, 0, 1) == 1:
        passed = passed + 1
    if mat2_is_identity(1, 1, 0, 1) == 0:
        passed = passed + 1
    prod: list[int] = mat2_multiply(1, 0, 0, 1, 5, 6, 7, 8)
    if prod == [5, 6, 7, 8]:
        passed = passed + 1
    if mat2_trace(3, 1, 2, 7) == 10:
        passed = passed + 1
    return passed
