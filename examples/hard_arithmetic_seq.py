"""Arithmetic sequence operations: sum, nth term, detection."""


def arith_nth_term(first: int, diff: int, n: int) -> int:
    """Return the nth term of arithmetic sequence (0-indexed)."""
    return first + n * diff


def arith_sum(first: int, diff: int, count: int) -> int:
    """Sum of first 'count' terms of arithmetic sequence."""
    total: int = 0
    idx: int = 0
    while idx < count:
        total = total + first + idx * diff
        idx = idx + 1
    return total


def detect_arithmetic(arr: list[int]) -> int:
    """Return 1 if array forms an arithmetic sequence, else 0."""
    length: int = len(arr)
    if length <= 2:
        return 1
    diff: int = arr[1] - arr[0]
    idx: int = 2
    while idx < length:
        prev: int = idx - 1
        if arr[idx] - arr[prev] != diff:
            return 0
        idx = idx + 1
    return 1


def find_missing_arith(arr: list[int]) -> int:
    """Find the missing element in an arithmetic sequence with one gap.
    Assumes array has at least 3 elements and exactly one is missing."""
    length: int = len(arr)
    last_idx: int = length - 1
    total_diff: int = arr[last_idx] - arr[0]
    expected_diff: int = total_diff // length
    idx: int = 0
    while idx < last_idx:
        next_idx: int = idx + 1
        if arr[next_idx] - arr[idx] != expected_diff:
            return arr[idx] + expected_diff
        idx = idx + 1
    return -1


def test_module() -> int:
    passed: int = 0

    if arith_nth_term(2, 3, 4) == 14:
        passed = passed + 1
    if arith_sum(1, 1, 10) == 55:
        passed = passed + 1
    if arith_sum(2, 3, 4) == 26:
        passed = passed + 1
    if detect_arithmetic([2, 5, 8, 11]) == 1:
        passed = passed + 1
    if detect_arithmetic([1, 3, 7, 10]) == 0:
        passed = passed + 1
    if find_missing_arith([2, 4, 8, 10]) == 6:
        passed = passed + 1
    if arith_nth_term(0, 5, 0) == 0:
        passed = passed + 1

    return passed
