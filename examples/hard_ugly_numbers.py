"""Generate nth ugly number (numbers whose only prime factors are 2, 3, and 5).

Tests: first few ugly numbers, specific indices, sequence properties.
"""


def nth_ugly_number(n: int) -> int:
    """Return the nth ugly number (1-indexed). Ugly numbers: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12..."""
    if n <= 0:
        return 0
    ugly: list[int] = []
    ugly.append(1)
    i2: int = 0
    i3: int = 0
    i5: int = 0
    count: int = 1
    while count < n:
        next2: int = ugly[i2] * 2
        next3: int = ugly[i3] * 3
        next5: int = ugly[i5] * 5
        next_ugly: int = next2
        if next3 < next_ugly:
            next_ugly = next3
        if next5 < next_ugly:
            next_ugly = next5
        ugly.append(next_ugly)
        if next_ugly == next2:
            i2 = i2 + 1
        if next_ugly == next3:
            i3 = i3 + 1
        if next_ugly == next5:
            i5 = i5 + 1
        count = count + 1
    return ugly[n - 1]


def is_ugly(num: int) -> int:
    """Return 1 if num is an ugly number, 0 otherwise."""
    if num <= 0:
        return 0
    n: int = num
    while n % 2 == 0:
        n = n // 2
    while n % 3 == 0:
        n = n // 3
    while n % 5 == 0:
        n = n // 5
    if n == 1:
        return 1
    return 0


def ugly_numbers_up_to(limit: int) -> list[int]:
    """Return all ugly numbers up to limit."""
    result: list[int] = []
    i: int = 1
    while i <= limit:
        if is_ugly(i) == 1:
            result.append(i)
        i = i + 1
    return result


def count_ugly_up_to(limit: int) -> int:
    """Return count of ugly numbers up to limit."""
    count: int = 0
    i: int = 1
    while i <= limit:
        if is_ugly(i) == 1:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test ugly number generation."""
    ok: int = 0

    if nth_ugly_number(1) == 1:
        ok = ok + 1
    if nth_ugly_number(7) == 8:
        ok = ok + 1
    if nth_ugly_number(10) == 12:
        ok = ok + 1
    if nth_ugly_number(15) == 24:
        ok = ok + 1

    if is_ugly(6) == 1:
        ok = ok + 1
    if is_ugly(14) == 0:
        ok = ok + 1
    if is_ugly(1) == 1:
        ok = ok + 1

    uglies: list[int] = ugly_numbers_up_to(12)
    if len(uglies) == 10:
        ok = ok + 1

    if count_ugly_up_to(30) == 18:
        ok = ok + 1

    if nth_ugly_number(0) == 0:
        ok = ok + 1

    return ok
