"""Bit shift patterns: left/right shift, power of 2 checks."""


def left_shift(n: int, amount: int) -> int:
    """Shift n left by amount positions."""
    return n << amount


def right_shift(n: int, amount: int) -> int:
    """Shift n right by amount positions."""
    return n >> amount


def is_power_of_two(n: int) -> int:
    """Return 1 if n is a power of 2, else 0."""
    if n <= 0:
        return 0
    if (n & (n - 1)) == 0:
        return 1
    return 0


def next_power_of_two(n: int) -> int:
    """Find the next power of two >= n."""
    if n <= 1:
        return 1
    val: int = 1
    while val < n:
        val = val << 1
    return val


def multiply_by_shift(a: int, b: int) -> int:
    """Multiply a by b using only shifts and adds (b >= 0)."""
    result: int = 0
    multiplicand: int = a
    multiplier: int = b
    while multiplier > 0:
        if (multiplier & 1) == 1:
            result = result + multiplicand
        multiplicand = multiplicand << 1
        multiplier = multiplier >> 1
    return result


def test_module() -> int:
    passed: int = 0

    if left_shift(1, 4) == 16:
        passed = passed + 1
    if right_shift(16, 2) == 4:
        passed = passed + 1
    if is_power_of_two(8) == 1:
        passed = passed + 1
    if is_power_of_two(6) == 0:
        passed = passed + 1
    if next_power_of_two(5) == 8:
        passed = passed + 1
    if next_power_of_two(8) == 8:
        passed = passed + 1
    if multiply_by_shift(7, 6) == 42:
        passed = passed + 1
    if multiply_by_shift(0, 100) == 0:
        passed = passed + 1

    return passed
