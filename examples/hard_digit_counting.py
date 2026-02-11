"""Digit counting and frequency operations.

Tests: digit frequency, digit sum, specific digit count.
"""


def digit_frequency(n: int) -> list[int]:
    """Count frequency of each digit 0-9 in number n. Returns list of 10 counts."""
    freq: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    val: int = n
    if val < 0:
        val = -val
    if val == 0:
        freq[0] = 1
        return freq
    while val > 0:
        d: int = val % 10
        freq[d] = freq[d] + 1
        val = val // 10
    return freq


def digit_sum(n: int) -> int:
    """Sum of all digits of n."""
    val: int = n
    if val < 0:
        val = -val
    total: int = 0
    while val > 0:
        total = total + val % 10
        val = val // 10
    return total


def count_specific_digit(n: int, d: int) -> int:
    """Count occurrences of digit d in number n."""
    val: int = n
    if val < 0:
        val = -val
    if val == 0:
        if d == 0:
            return 1
        return 0
    count: int = 0
    while val > 0:
        if val % 10 == d:
            count = count + 1
        val = val // 10
    return count


def digital_root(n: int) -> int:
    """Compute digital root (repeated digit sum until single digit)."""
    val: int = n
    if val < 0:
        val = -val
    while val >= 10:
        val = digit_sum(val)
    return val


def test_module() -> int:
    """Test digit counting operations."""
    ok: int = 0
    freq: list[int] = digit_frequency(112233)
    if freq[1] == 2:
        ok = ok + 1
    if freq[2] == 2:
        ok = ok + 1
    if freq[3] == 2:
        ok = ok + 1
    if digit_sum(123) == 6:
        ok = ok + 1
    if count_specific_digit(111222, 1) == 3:
        ok = ok + 1
    if digital_root(493) == 7:
        ok = ok + 1
    return ok
