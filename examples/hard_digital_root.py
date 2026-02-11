"""Digital root and digit operations.

Tests: digital root, digit sum, digit product, digit count, persistence.
"""


def digit_sum(n: int) -> int:
    """Sum of digits of n."""
    val: int = n
    if val < 0:
        val = -val
    total: int = 0
    while val > 0:
        total = total + val % 10
        val = val // 10
    return total


def digital_root(n: int) -> int:
    """Digital root: repeatedly sum digits until single digit."""
    val: int = n
    if val < 0:
        val = -val
    while val >= 10:
        val = digit_sum(val)
    return val


def digit_product(n: int) -> int:
    """Product of digits of n."""
    val: int = n
    if val < 0:
        val = -val
    if val == 0:
        return 0
    product: int = 1
    while val > 0:
        product = product * (val % 10)
        val = val // 10
    return product


def count_digits(n: int) -> int:
    """Count number of digits in n."""
    val: int = n
    if val < 0:
        val = -val
    if val == 0:
        return 1
    count: int = 0
    while val > 0:
        count = count + 1
        val = val // 10
    return count


def multiplicative_persistence(n: int) -> int:
    """Count steps to reach single digit by multiplying digits."""
    val: int = n
    if val < 0:
        val = -val
    steps: int = 0
    while val >= 10:
        val = digit_product(val)
        steps = steps + 1
    return steps


def test_module() -> int:
    """Test digital root and digit operations."""
    ok: int = 0
    if digit_sum(123) == 6:
        ok = ok + 1
    if digit_sum(9999) == 36:
        ok = ok + 1
    if digital_root(493) == 7:
        ok = ok + 1
    if digital_root(9) == 9:
        ok = ok + 1
    if digit_product(234) == 24:
        ok = ok + 1
    if count_digits(12345) == 5:
        ok = ok + 1
    if count_digits(0) == 1:
        ok = ok + 1
    if multiplicative_persistence(39) == 3:
        ok = ok + 1
    return ok
