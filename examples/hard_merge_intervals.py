"""Interval merging algorithms.

Tests: merge count, overlap check, total coverage, gap count.
"""


def count_overlapping_pairs(starts: list[int], ends: list[int]) -> int:
    """Count pairs of overlapping intervals."""
    n: int = len(starts)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if starts[i] < ends[j] and starts[j] < ends[i]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def total_coverage(starts: list[int], ends: list[int]) -> int:
    """Compute total coverage length of intervals (may overcount overlaps)."""
    n: int = len(starts)
    total: int = 0
    i: int = 0
    while i < n:
        length: int = ends[i] - starts[i]
        if length > 0:
            total = total + length
        i = i + 1
    return total


def max_overlap_depth(starts: list[int], ends: list[int], max_val: int) -> int:
    """Find maximum number of overlapping intervals at any point."""
    timeline: list[int] = [0] * (max_val + 2)
    n: int = len(starts)
    i: int = 0
    while i < n:
        if starts[i] >= 0 and starts[i] <= max_val:
            timeline[starts[i]] = timeline[starts[i]] + 1
        if ends[i] >= 0 and ends[i] <= max_val:
            timeline[ends[i]] = timeline[ends[i]] - 1
        i = i + 1
    best: int = 0
    curr: int = 0
    j: int = 0
    while j <= max_val:
        curr = curr + timeline[j]
        if curr > best:
            best = curr
        j = j + 1
    return best


def count_non_overlapping(starts: list[int], ends: list[int]) -> int:
    """Count intervals that do not overlap with any other interval."""
    n: int = len(starts)
    count: int = 0
    i: int = 0
    while i < n:
        has_overlap: int = 0
        j: int = 0
        while j < n:
            if i != j:
                if starts[i] < ends[j] and starts[j] < ends[i]:
                    has_overlap = 1
                    j = n
            j = j + 1
        if has_overlap == 0:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test interval operations."""
    ok: int = 0
    s1: list[int] = [1, 2, 5, 8]
    e1: list[int] = [4, 6, 7, 10]
    if count_overlapping_pairs(s1, e1) == 2:
        ok = ok + 1
    if total_coverage(s1, e1) == 11:
        ok = ok + 1
    if max_overlap_depth([1, 2, 3], [5, 6, 7], 10) == 3:
        ok = ok + 1
    if count_non_overlapping([1, 5, 10], [3, 7, 12]) == 3:
        ok = ok + 1
    if count_non_overlapping([1, 2, 5], [4, 6, 7]) == 1:
        ok = ok + 1
    return ok
