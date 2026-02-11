"""Sweep line: max overlapping intervals, event processing, merge intervals.

Tests: max_overlap, merge_intervals, interval_intersection, point_coverage.
"""


def max_overlap(starts: list[int], ends: list[int]) -> int:
    """Find maximum number of overlapping intervals using sweep line."""
    n: int = len(starts)
    events: list[int] = []
    i: int = 0
    while i < n:
        events.append(starts[i] * 2)
        events.append(ends[i] * 2 + 1)
        i = i + 1
    ne: int = len(events)
    ei: int = 0
    while ei < ne - 1:
        ej: int = ei + 1
        while ej < ne:
            if events[ej] < events[ei]:
                tmp: int = events[ei]
                events[ei] = events[ej]
                events[ej] = tmp
            ej = ej + 1
        ei = ei + 1
    current: int = 0
    max_count: int = 0
    k: int = 0
    while k < ne:
        if events[k] % 2 == 0:
            current = current + 1
        else:
            current = current - 1
        if current > max_count:
            max_count = current
        k = k + 1
    return max_count


def merge_intervals(starts: list[int], ends: list[int]) -> list[int]:
    """Merge overlapping intervals. Returns flat list [s1,e1,s2,e2,...]."""
    n: int = len(starts)
    if n == 0:
        return []
    order: list[int] = []
    i: int = 0
    while i < n:
        order.append(i)
        i = i + 1
    oi: int = 0
    while oi < n - 1:
        oj: int = oi + 1
        while oj < n:
            si: int = order[oi]
            sj: int = order[oj]
            if starts[sj] < starts[si]:
                tmp: int = order[oi]
                order[oi] = order[oj]
                order[oj] = tmp
            oj = oj + 1
        oi = oi + 1
    first_idx: int = order[0]
    cur_s: int = starts[first_idx]
    cur_e: int = ends[first_idx]
    result: list[int] = []
    k: int = 1
    while k < n:
        idx: int = order[k]
        s: int = starts[idx]
        e: int = ends[idx]
        if s <= cur_e:
            if e > cur_e:
                cur_e = e
        else:
            result.append(cur_s)
            result.append(cur_e)
            cur_s = s
            cur_e = e
        k = k + 1
    result.append(cur_s)
    result.append(cur_e)
    return result


def point_coverage(starts: list[int], ends: list[int], point: int) -> int:
    """Count how many intervals contain the given point."""
    n: int = len(starts)
    count: int = 0
    i: int = 0
    while i < n:
        if starts[i] <= point:
            if ends[i] >= point:
                count = count + 1
        i = i + 1
    return count


def total_covered_length(starts: list[int], ends: list[int]) -> int:
    """Total length covered by union of intervals."""
    merged: list[int] = merge_intervals(starts, ends)
    total: int = 0
    i: int = 0
    nm: int = len(merged)
    while i < nm:
        s: int = merged[i]
        e: int = merged[i + 1]
        total = total + e - s
        i = i + 2
    return total


def count_intervals_at_each_point(starts: list[int], ends: list[int], max_point: int) -> list[int]:
    """Count intervals covering each point from 0 to max_point."""
    result: list[int] = []
    p: int = 0
    while p <= max_point:
        cov: int = point_coverage(starts, ends, p)
        result.append(cov)
        p = p + 1
    return result


def test_module() -> int:
    """Test sweep line algorithms."""
    passed: int = 0

    s1: list[int] = [1, 2, 3, 5]
    e1: list[int] = [4, 6, 5, 8]
    if max_overlap(s1, e1) == 3:
        passed = passed + 1

    s2: list[int] = [1, 1, 6, 8]
    e2: list[int] = [3, 5, 7, 10]
    m: list[int] = merge_intervals(s2, e2)
    if m == [1, 5, 6, 7, 8, 10]:
        passed = passed + 1

    s3: list[int] = [1, 3]
    e3: list[int] = [5, 7]
    m2: list[int] = merge_intervals(s3, e3)
    if m2 == [1, 7]:
        passed = passed + 1

    pc: int = point_coverage([1, 3, 5], [4, 6, 8], 4)
    if pc == 2:
        passed = passed + 1

    tcl: int = total_covered_length([1, 3, 7], [5, 6, 9])
    if tcl == 7:
        passed = passed + 1

    empty_m: list[int] = merge_intervals([], [])
    if empty_m == []:
        passed = passed + 1

    return passed
