"""Interval scheduling: select non-overlapping intervals and merge operations."""


def intervals_overlap(s1: int, e1: int, s2: int, e2: int) -> int:
    """Check if two intervals overlap. Returns 1 if overlapping, 0 otherwise."""
    if s1 >= e2:
        return 0
    if s2 >= e1:
        return 0
    return 1


def count_non_overlapping(starts: list[int], ends: list[int]) -> int:
    """Count maximum non-overlapping intervals using greedy by end time.
    Assumes intervals are sorted by end time."""
    n: int = len(starts)
    if n == 0:
        return 0
    count: int = 1
    last_end: int = ends[0]
    i: int = 1
    while i < n:
        if starts[i] >= last_end:
            count = count + 1
            last_end = ends[i]
        i = i + 1
    return count


def total_covered_length(starts: list[int], ends: list[int]) -> int:
    """Calculate total length covered by intervals (with merging).
    Assumes intervals sorted by start time."""
    n: int = len(starts)
    if n == 0:
        return 0
    total: int = 0
    cur_start: int = starts[0]
    cur_end: int = ends[0]
    i: int = 1
    while i < n:
        if starts[i] <= cur_end:
            if ends[i] > cur_end:
                cur_end = ends[i]
        else:
            total = total + (cur_end - cur_start)
            cur_start = starts[i]
            cur_end = ends[i]
        i = i + 1
    total = total + (cur_end - cur_start)
    return total


def max_overlap_count(starts: list[int], ends: list[int]) -> int:
    """Find maximum number of simultaneously overlapping intervals.
    Uses event sweep approach with sorted events."""
    n: int = len(starts)
    if n == 0:
        return 0
    events: list[int] = []
    i: int = 0
    while i < n:
        events.append(starts[i] * 2)
        events.append(ends[i] * 2 + 1)
        i = i + 1
    # Simple sort
    sz: int = len(events)
    j: int = 0
    while j < sz:
        k: int = j + 1
        while k < sz:
            if events[k] < events[j]:
                tmp: int = events[j]
                events[j] = events[k]
                events[k] = tmp
            k = k + 1
        j = j + 1
    max_count: int = 0
    current: int = 0
    idx: int = 0
    while idx < sz:
        if events[idx] % 2 == 0:
            current = current + 1
        else:
            current = current - 1
        if current > max_count:
            max_count = current
        idx = idx + 1
    return max_count


def test_module() -> int:
    """Test interval scheduling functions."""
    ok: int = 0

    if intervals_overlap(1, 5, 3, 7) == 1:
        ok = ok + 1

    if intervals_overlap(1, 3, 5, 7) == 0:
        ok = ok + 1

    starts1: list[int] = [1, 2, 4, 6]
    ends1: list[int] = [3, 5, 7, 8]
    if count_non_overlapping(starts1, ends1) == 2:
        ok = ok + 1

    starts2: list[int] = [1, 3, 7]
    ends2: list[int] = [5, 6, 10]
    if total_covered_length(starts2, ends2) == 9:
        ok = ok + 1

    starts3: list[int] = [1, 2, 3]
    ends3: list[int] = [5, 6, 7]
    if max_overlap_count(starts3, ends3) == 3:
        ok = ok + 1

    starts4: list[int] = [1, 10]
    ends4: list[int] = [5, 15]
    if max_overlap_count(starts4, ends4) == 1:
        ok = ok + 1

    return ok
