# Matrix exponentiation by squaring (2x2 matrices as flat list)


def mat2_multiply(a: list[int], b: list[int]) -> list[int]:
    # 2x2 matrix multiply: [a00, a01, a10, a11]
    result: list[int] = [
        a[0] * b[0] + a[1] * b[2],
        a[0] * b[1] + a[1] * b[3],
        a[2] * b[0] + a[3] * b[2],
        a[2] * b[1] + a[3] * b[3],
    ]
    return result


def mat2_identity() -> list[int]:
    return [1, 0, 0, 1]


def mat2_power(m: list[int], n: int) -> list[int]:
    # Matrix exponentiation by squaring
    result: list[int] = mat2_identity()
    if n == 0:
        return result
    result = mat2_identity()
    base: list[int] = [m[0], m[1], m[2], m[3]]
    exp: int = n
    while exp > 0:
        if exp % 2 == 1:
            result = mat2_multiply(result, base)
        base = mat2_multiply(base, base)
        exp = exp // 2
    return result


def fibonacci_matrix(n: int) -> int:
    # F(n) using matrix exponentiation
    if n <= 0:
        return 0
    if n == 1:
        return 1
    m: list[int] = [1, 1, 1, 0]
    powered: list[int] = mat2_power(m, n - 1)
    return powered[0]


def mat2_trace(m: list[int]) -> int:
    return m[0] + m[3]


def mat2_determinant(m: list[int]) -> int:
    return m[0] * m[3] - m[1] * m[2]


def test_module() -> int:
    passed: int = 0

    # Test 1: identity multiply
    eye: list[int] = mat2_identity()
    m: list[int] = [1, 2, 3, 4]
    product: list[int] = mat2_multiply(eye, m)
    if product[0] == 1 and product[1] == 2 and product[2] == 3 and product[3] == 4:
        passed = passed + 1

    # Test 2: power 0 = identity
    p0: list[int] = mat2_power(m, 0)
    if p0[0] == 1 and p0[1] == 0 and p0[2] == 0 and p0[3] == 1:
        passed = passed + 1

    # Test 3: power 1 = self
    p1: list[int] = mat2_power(m, 1)
    if p1[0] == 1 and p1[1] == 2 and p1[2] == 3 and p1[3] == 4:
        passed = passed + 1

    # Test 4: power 2
    p2: list[int] = mat2_power(m, 2)
    expected: list[int] = mat2_multiply(m, m)
    if p2[0] == expected[0] and p2[1] == expected[1]:
        passed = passed + 1

    # Test 5: fibonacci via matrix
    if fibonacci_matrix(10) == 55:
        passed = passed + 1

    # Test 6: fibonacci small values
    if fibonacci_matrix(1) == 1 and fibonacci_matrix(2) == 1 and fibonacci_matrix(6) == 8:
        passed = passed + 1

    # Test 7: trace
    if mat2_trace(m) == 5:
        passed = passed + 1

    # Test 8: determinant
    if mat2_determinant(m) == -2:
        passed = passed + 1

    return passed
