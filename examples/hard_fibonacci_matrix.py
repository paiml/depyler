"""Fibonacci using matrix exponentiation with manual 2x2 multiply.

Tests: small fib numbers, larger fib, base cases, matrix multiply correctness.
"""


def mat_mult_2x2(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Multiply two 2x2 matrices represented as list of lists."""
    r00: int = a[0][0] * b[0][0] + a[0][1] * b[1][0]
    r01: int = a[0][0] * b[0][1] + a[0][1] * b[1][1]
    r10: int = a[1][0] * b[0][0] + a[1][1] * b[1][0]
    r11: int = a[1][0] * b[0][1] + a[1][1] * b[1][1]
    row0: list[int] = [r00, r01]
    row1: list[int] = [r10, r11]
    result: list[list[int]] = [row0, row1]
    return result


def mat_pow_2x2(m: list[list[int]], power: int) -> list[list[int]]:
    """Raise a 2x2 matrix to given power using repeated squaring."""
    row0: list[int] = [1, 0]
    row1: list[int] = [0, 1]
    result: list[list[int]] = [row0, row1]
    base_r0: list[int] = [m[0][0], m[0][1]]
    base_r1: list[int] = [m[1][0], m[1][1]]
    base: list[list[int]] = [base_r0, base_r1]
    p: int = power
    while p > 0:
        if p % 2 == 1:
            result = mat_mult_2x2(result, base)
        base = mat_mult_2x2(base, base)
        p = p // 2
    return result


def fibonacci_matrix(n: int) -> int:
    """Return nth Fibonacci number using matrix exponentiation. F(0)=0, F(1)=1."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    row0: list[int] = [1, 1]
    row1: list[int] = [1, 0]
    fib_mat: list[list[int]] = [row0, row1]
    result: list[list[int]] = mat_pow_2x2(fib_mat, n - 1)
    return result[0][0]


def fibonacci_iterative(n: int) -> int:
    """Return nth Fibonacci number using simple iteration."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        temp: int = a + b
        a = b
        b = temp
        i = i + 1
    return b


def is_fibonacci(num: int) -> int:
    """Return 1 if num is a Fibonacci number, 0 otherwise."""
    if num < 0:
        return 0
    a: int = 0
    b: int = 1
    while a < num:
        temp: int = a + b
        a = b
        b = temp
    if a == num:
        return 1
    return 0


def test_module() -> int:
    """Test Fibonacci matrix exponentiation."""
    ok: int = 0

    if fibonacci_matrix(0) == 0:
        ok = ok + 1
    if fibonacci_matrix(1) == 1:
        ok = ok + 1
    if fibonacci_matrix(10) == 55:
        ok = ok + 1
    if fibonacci_matrix(20) == 6765:
        ok = ok + 1

    if fibonacci_iterative(10) == 55:
        ok = ok + 1
    if fibonacci_matrix(10) == fibonacci_iterative(10):
        ok = ok + 1

    identity: list[list[int]] = [[1, 0], [0, 1]]
    test_mat: list[list[int]] = [[2, 3], [4, 5]]
    product: list[list[int]] = mat_mult_2x2(identity, test_mat)
    if product[0][0] == 2 and product[1][1] == 5:
        ok = ok + 1

    if is_fibonacci(8) == 1:
        ok = ok + 1
    if is_fibonacci(9) == 0:
        ok = ok + 1

    if fibonacci_matrix(15) == 610:
        ok = ok + 1

    return ok
