"""Arithmetic and geometric progression operations.

Implements operations for generating and analyzing
arithmetic and geometric progressions.
"""


def arithmetic_sum(first: int, diff: int, n: int) -> int:
    """Compute sum of arithmetic progression: first, first+d, first+2d, ..."""
    last: int = first + (n - 1) * diff
    total: int = n * (first + last) // 2
    return total


def arithmetic_nth(first: int, diff: int, n: int) -> int:
    """Compute nth term (0-indexed) of arithmetic progression."""
    result: int = first + n * diff
    return result


def geometric_nth(first: int, ratio: int, n: int) -> int:
    """Compute nth term (0-indexed) of geometric progression."""
    result: int = first
    i: int = 0
    while i < n:
        result = result * ratio
        i = i + 1
    return result


def is_arithmetic(arr: list[int], size: int) -> int:
    """Check if array forms an arithmetic progression. Returns 1 if yes."""
    if size <= 2:
        return 1
    diff: int = arr[1] - arr[0]
    i: int = 2
    while i < size:
        prev_idx: int = i - 1
        if arr[i] - arr[prev_idx] != diff:
            return 0
        i = i + 1
    return 1


def is_geometric(arr: list[int], size: int) -> int:
    """Check if array forms a geometric progression. Returns 1 if yes.

    Only works with integer ratios and non-zero elements.
    """
    if size <= 2:
        return 1
    if arr[0] == 0:
        return 0
    ratio: int = arr[1] // arr[0]
    i: int = 2
    while i < size:
        prev_idx: int = i - 1
        if arr[prev_idx] == 0:
            return 0
        expected: int = arr[prev_idx] * ratio
        if arr[i] != expected:
            return 0
        i = i + 1
    return 1


def find_missing_arithmetic(arr: list[int], size: int) -> int:
    """Find missing element in arithmetic progression with one gap.

    Assumes sorted and exactly one element missing.
    """
    if size < 2:
        return 0
    last_idx: int = size - 1
    total_diff: int = arr[last_idx] - arr[0]
    expected_diff: int = total_diff // size
    i: int = 0
    while i < size - 1:
        next_idx: int = i + 1
        if arr[next_idx] - arr[i] != expected_diff:
            missing: int = arr[i] + expected_diff
            return missing
        i = i + 1
    return 0


def test_module() -> int:
    """Test progression operations."""
    ok: int = 0

    asum: int = arithmetic_sum(1, 2, 5)
    if asum == 25:
        ok = ok + 1

    nth: int = arithmetic_nth(3, 5, 4)
    if nth == 23:
        ok = ok + 1

    gnth: int = geometric_nth(2, 3, 3)
    if gnth == 54:
        ok = ok + 1

    arr_a: list[int] = [2, 4, 6, 8, 10]
    is_a: int = is_arithmetic(arr_a, 5)
    if is_a == 1:
        ok = ok + 1

    arr_g: list[int] = [2, 6, 18, 54]
    is_g: int = is_geometric(arr_g, 4)
    if is_g == 1:
        ok = ok + 1

    arr_m: list[int] = [2, 4, 8, 10]
    missing: int = find_missing_arithmetic(arr_m, 4)
    if missing == 6:
        ok = ok + 1

    return ok
