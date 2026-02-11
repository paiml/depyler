"""Narcissistic (Armstrong) number detection.

Tests: is narcissistic, count digits, power sum, count in range.
"""


def count_digits_of(n: int) -> int:
    """Count digits of n."""
    if n == 0:
        return 1
    val: int = n
    if val < 0:
        val = -val
    count: int = 0
    while val > 0:
        count = count + 1
        val = val // 10
    return count


def int_pow(base: int, exp: int) -> int:
    """Integer power."""
    result: int = 1
    i: int = 0
    while i < exp:
        result = result * base
        i = i + 1
    return result


def digit_power_sum(n: int) -> int:
    """Sum of each digit raised to the power of number of digits."""
    val: int = n
    if val < 0:
        val = -val
    num_digits: int = count_digits_of(val)
    total: int = 0
    temp: int = val
    while temp > 0:
        d: int = temp % 10
        total = total + int_pow(d, num_digits)
        temp = temp // 10
    return total


def is_narcissistic(n: int) -> int:
    """Returns 1 if n is a narcissistic number."""
    if n < 0:
        return 0
    if digit_power_sum(n) == n:
        return 1
    return 0


def count_narcissistic_in_range(lo: int, hi: int) -> int:
    """Count narcissistic numbers in [lo, hi]."""
    count: int = 0
    n: int = lo
    while n <= hi:
        if is_narcissistic(n) == 1:
            count = count + 1
        n = n + 1
    return count


def test_module() -> int:
    """Test narcissistic numbers."""
    ok: int = 0
    if is_narcissistic(153) == 1:
        ok = ok + 1
    if is_narcissistic(370) == 1:
        ok = ok + 1
    if is_narcissistic(100) == 0:
        ok = ok + 1
    if is_narcissistic(0) == 1:
        ok = ok + 1
    if digit_power_sum(153) == 153:
        ok = ok + 1
    if count_narcissistic_in_range(1, 500) == 4:
        ok = ok + 1
    return ok
