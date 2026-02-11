"""Finite differences of sequences: first, second, cumulative, restoration."""


def first_difference(arr: list[int]) -> list[int]:
    """Compute first differences: d[i] = arr[i+1] - arr[i]."""
    result: list[int] = []
    idx: int = 0
    length: int = len(arr)
    limit: int = length - 1
    while idx < limit:
        next_idx: int = idx + 1
        result.append(arr[next_idx] - arr[idx])
        idx = idx + 1
    return result


def second_difference(arr: list[int]) -> list[int]:
    """Compute second differences (difference of differences)."""
    d1: list[int] = first_difference(arr)
    d2: list[int] = first_difference(d1)
    return d2


def cumulative_sum(arr: list[int]) -> list[int]:
    """Compute cumulative sum (prefix sum)."""
    result: list[int] = []
    total: int = 0
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        total = total + arr[idx]
        result.append(total)
        idx = idx + 1
    return result


def restore_from_diff(first_val: int, diffs: list[int]) -> list[int]:
    """Restore original sequence from first value and first differences."""
    result: list[int] = [first_val]
    idx: int = 0
    length: int = len(diffs)
    while idx < length:
        prev_val: int = result[idx]
        result.append(prev_val + diffs[idx])
        idx = idx + 1
    return result


def is_polynomial_degree(arr: list[int], degree: int) -> int:
    """Check if sequence is polynomial of given degree by checking constant nth differences.
    Returns 1 if yes, 0 if no."""
    current: list[int] = []
    ci: int = 0
    while ci < len(arr):
        current.append(arr[ci])
        ci = ci + 1
    d: int = 0
    while d < degree:
        current = first_difference(current)
        d = d + 1
    idx: int = 1
    length: int = len(current)
    while idx < length:
        if current[idx] != current[0]:
            return 0
        idx = idx + 1
    return 1


def test_module() -> int:
    passed: int = 0

    d1: list[int] = first_difference([1, 4, 9, 16, 25])
    if d1[0] == 3:
        passed = passed + 1
    if d1[1] == 5:
        passed = passed + 1

    d2: list[int] = second_difference([1, 4, 9, 16, 25])
    if d2[0] == 2:
        passed = passed + 1

    cs: list[int] = cumulative_sum([1, 2, 3, 4, 5])
    if cs[4] == 15:
        passed = passed + 1

    restored: list[int] = restore_from_diff(1, [3, 5, 7, 9])
    if restored[4] == 25:
        passed = passed + 1

    if is_polynomial_degree([1, 4, 9, 16, 25], 2) == 1:
        passed = passed + 1

    if is_polynomial_degree([1, 2, 4, 8], 2) == 0:
        passed = passed + 1

    return passed
