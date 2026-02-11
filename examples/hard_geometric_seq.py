"""Geometric sequence operations using integer arithmetic."""


def geo_nth_term(first: int, ratio: int, n: int) -> int:
    """Return nth term of geometric sequence (0-indexed)."""
    result: int = first
    idx: int = 0
    while idx < n:
        result = result * ratio
        idx = idx + 1
    return result


def geo_sum(first: int, ratio: int, count: int) -> int:
    """Sum of first 'count' terms of geometric sequence."""
    total: int = 0
    current: int = first
    idx: int = 0
    while idx < count:
        total = total + current
        current = current * ratio
        idx = idx + 1
    return total


def detect_geometric(arr: list[int]) -> int:
    """Return 1 if array forms a geometric sequence, else 0.
    Assumes no zeros in array."""
    length: int = len(arr)
    if length <= 1:
        return 1
    if arr[0] == 0:
        return 0
    idx: int = 1
    while idx < length:
        prev: int = idx - 1
        if arr[prev] == 0:
            return 0
        if arr[idx] * arr[0] != arr[1] * arr[prev]:
            return 0
        idx = idx + 1
    return 1


def geo_generate(first: int, ratio: int, count: int) -> list[int]:
    """Generate first 'count' terms of geometric sequence."""
    result: list[int] = []
    current: int = first
    idx: int = 0
    while idx < count:
        result.append(current)
        current = current * ratio
        idx = idx + 1
    return result


def test_module() -> int:
    passed: int = 0

    if geo_nth_term(2, 3, 3) == 54:
        passed = passed + 1
    if geo_sum(1, 2, 4) == 15:
        passed = passed + 1
    if detect_geometric([2, 6, 18, 54]) == 1:
        passed = passed + 1
    if detect_geometric([2, 4, 7]) == 0:
        passed = passed + 1

    gen: list[int] = geo_generate(3, 2, 4)
    if gen[0] == 3:
        passed = passed + 1
    if gen[3] == 24:
        passed = passed + 1
    if geo_nth_term(1, 1, 5) == 1:
        passed = passed + 1

    return passed
