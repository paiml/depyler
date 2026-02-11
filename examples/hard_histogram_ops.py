"""Histogram operations on lists of integers.

Tests: counting, frequency analysis, mode finding.
"""


def count_occurrences(arr: list[int], target: int) -> int:
    """Count occurrences of target in array."""
    count: int = 0
    for v in arr:
        if v == target:
            count += 1
    return count


def find_mode(arr: list[int]) -> int:
    """Find the most frequent element (first if tied)."""
    if len(arr) == 0:
        return 0
    best_val: int = arr[0]
    best_count: int = 0
    i: int = 0
    while i < len(arr):
        c: int = count_occurrences(arr, arr[i])
        if c > best_count:
            best_count = c
            best_val = arr[i]
        i += 1
    return best_val


def value_range(arr: list[int]) -> int:
    """Compute range (max - min) of array."""
    if len(arr) == 0:
        return 0
    lo: int = arr[0]
    hi: int = arr[0]
    for v in arr:
        if v < lo:
            lo = v
        if v > hi:
            hi = v
    return hi - lo


def unique_count(arr: list[int]) -> int:
    """Count unique elements using nested loop."""
    seen: list[int] = []
    for v in arr:
        found: bool = False
        for s in seen:
            if s == v:
                found = True
        if not found:
            seen.append(v)
    return len(seen)


def cumulative_sum(arr: list[int]) -> list[int]:
    """Compute cumulative sum."""
    result: list[int] = []
    total: int = 0
    for v in arr:
        total += v
        result.append(total)
    return result


def test_module() -> int:
    """Test histogram operations."""
    ok: int = 0

    c: int = count_occurrences([1, 2, 3, 2, 1, 2], 2)
    if c == 3:
        ok += 1

    m: int = find_mode([1, 2, 2, 3, 3, 3])
    if m == 3:
        ok += 1

    r: int = value_range([5, 2, 8, 1, 9])
    if r == 8:
        ok += 1

    u: int = unique_count([1, 2, 2, 3, 3, 3])
    if u == 3:
        ok += 1

    cs: list[int] = cumulative_sum([1, 2, 3, 4])
    if cs == [1, 3, 6, 10]:
        ok += 1

    return ok
