"""Fast exponentiation using binary method (repeated squaring)."""


def fast_power(base: int, exp: int) -> int:
    """Compute base^exp using binary exponentiation."""
    if exp == 0:
        return 1
    result: int = 1
    b: int = base
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = result * b
        b = b * b
        e = e // 2
    return result


def fast_power_mod(base: int, exp: int, mod: int) -> int:
    """Compute (base^exp) % mod using modular exponentiation."""
    if mod == 1:
        return 0
    result: int = 1
    b: int = base % mod
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = (result * b) % mod
        b = (b * b) % mod
        e = e // 2
    return result


def matrix_mult_2x2(a: list[int], b: list[int]) -> list[int]:
    """Multiply two 2x2 matrices stored as flat [a00,a01,a10,a11]."""
    c00: int = a[0] * b[0] + a[1] * b[2]
    c01: int = a[0] * b[1] + a[1] * b[3]
    c10: int = a[2] * b[0] + a[3] * b[2]
    c11: int = a[2] * b[1] + a[3] * b[3]
    result: list[int] = [c00, c01, c10, c11]
    return result


def fib_matrix(n: int) -> int:
    """Compute nth Fibonacci using matrix exponentiation."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    result: list[int] = [1, 0, 0, 1]
    base: list[int] = [1, 1, 1, 0]
    e: int = n
    while e > 0:
        if e % 2 == 1:
            result = matrix_mult_2x2(result, base)
        base = matrix_mult_2x2(base, base)
        e = e // 2
    return result[1]


def test_module() -> int:
    passed: int = 0

    if fast_power(2, 10) == 1024:
        passed = passed + 1

    if fast_power(3, 5) == 243:
        passed = passed + 1

    if fast_power_mod(2, 10, 1000) == 24:
        passed = passed + 1

    if fast_power_mod(3, 13, 7) == 3:
        passed = passed + 1

    if fib_matrix(10) == 55:
        passed = passed + 1

    if fib_matrix(1) == 1:
        passed = passed + 1

    if fast_power(5, 0) == 1:
        passed = passed + 1

    if fib_matrix(20) == 6765:
        passed = passed + 1

    return passed
