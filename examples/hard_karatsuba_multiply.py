"""Karatsuba multiplication algorithm for large integers."""


def num_digits(n: int) -> int:
    """Count number of digits in n."""
    if n == 0:
        return 1
    val: int = n
    if val < 0:
        val = 0 - val
    count: int = 0
    while val > 0:
        val = val // 10
        count = count + 1
    return count


def power_of_10(exp: int) -> int:
    """Compute 10^exp."""
    result: int = 1
    i: int = 0
    while i < exp:
        result = result * 10
        i = i + 1
    return result


def abs_val(n: int) -> int:
    """Absolute value."""
    if n < 0:
        return 0 - n
    return n


def karatsuba(x: int, y: int) -> int:
    """Multiply x and y using Karatsuba algorithm."""
    ax: int = abs_val(x)
    ay: int = abs_val(y)
    if ax < 10 or ay < 10:
        return x * y
    dx: int = num_digits(ax)
    dy: int = num_digits(ay)
    nd: int = dx
    if dy > nd:
        nd = dy
    half: int = nd // 2
    p: int = power_of_10(half)
    high_x: int = x // p
    low_x: int = x % p
    high_y: int = y // p
    low_y: int = y % p
    z0: int = karatsuba(low_x, low_y)
    z2: int = karatsuba(high_x, high_y)
    sum_x: int = high_x + low_x
    sum_y: int = high_y + low_y
    z1: int = karatsuba(sum_x, sum_y) - z2 - z0
    result: int = z2 * power_of_10(2 * half) + z1 * power_of_10(half) + z0
    return result


def simple_multiply(a: int, b: int) -> int:
    """Simple multiplication for verification."""
    return a * b


def test_module() -> int:
    """Test Karatsuba multiplication."""
    passed: int = 0

    if karatsuba(12, 34) == 408:
        passed = passed + 1

    if karatsuba(123, 456) == 56088:
        passed = passed + 1

    if karatsuba(0, 999) == 0:
        passed = passed + 1

    if karatsuba(7, 8) == 56:
        passed = passed + 1

    if karatsuba(1000, 1000) == 1000000:
        passed = passed + 1

    if karatsuba(99, 99) == 9801:
        passed = passed + 1

    return passed
