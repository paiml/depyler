"""Happy number detection.

Tests: is happy, sum of squared digits, count happy in range, happy chain length.
"""


def sum_squared_digits(n: int) -> int:
    """Sum of squares of digits of n."""
    val: int = n
    if val < 0:
        val = -val
    total: int = 0
    while val > 0:
        d: int = val % 10
        total = total + d * d
        val = val // 10
    return total


def is_happy(n: int) -> int:
    """Returns 1 if n is a happy number."""
    if n <= 0:
        return 0
    slow: int = n
    fast: int = sum_squared_digits(n)
    while fast != 1 and slow != fast:
        slow = sum_squared_digits(slow)
        fast = sum_squared_digits(sum_squared_digits(fast))
    if fast == 1:
        return 1
    return 0


def happy_chain_length(n: int) -> int:
    """Number of steps to reach 1 (or cycle detection limit)."""
    if n <= 0:
        return 0
    val: int = n
    steps: int = 0
    limit: int = 200
    while val != 1 and steps < limit:
        val = sum_squared_digits(val)
        steps = steps + 1
    return steps


def count_happy_in_range(lo: int, hi: int) -> int:
    """Count happy numbers in [lo, hi]."""
    count: int = 0
    n: int = lo
    while n <= hi:
        if is_happy(n) == 1:
            count = count + 1
        n = n + 1
    return count


def test_module() -> int:
    """Test happy numbers."""
    ok: int = 0
    if is_happy(7) == 1:
        ok = ok + 1
    if is_happy(2) == 0:
        ok = ok + 1
    if is_happy(1) == 1:
        ok = ok + 1
    if sum_squared_digits(49) == 97:
        ok = ok + 1
    if happy_chain_length(7) > 0:
        ok = ok + 1
    if count_happy_in_range(1, 10) == 3:
        ok = ok + 1
    return ok
