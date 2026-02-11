"""Interval scheduling maximization: select maximum non-overlapping intervals."""


def sort_intervals_by_end(starts: list[int], ends: list[int]) -> list[int]:
    """Return indices sorted by end time ascending."""
    length: int = len(starts)
    indices: list[int] = []
    idx: int = 0
    while idx < length:
        indices.append(idx)
        idx = idx + 1
    i: int = 0
    while i < length:
        j: int = i + 1
        while j < length:
            if ends[indices[j]] < ends[indices[i]]:
                temp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = temp
            j = j + 1
        i = i + 1
    return indices


def max_non_overlapping(starts: list[int], ends: list[int]) -> int:
    """Maximum number of non-overlapping intervals."""
    length: int = len(starts)
    if length == 0:
        return 0
    order: list[int] = sort_intervals_by_end(starts, ends)
    count: int = 1
    last_end: int = ends[order[0]]
    idx: int = 1
    while idx < length:
        curr: int = order[idx]
        if starts[curr] >= last_end:
            count = count + 1
            last_end = ends[curr]
        idx = idx + 1
    return count


def count_overlaps(starts: list[int], ends: list[int]) -> int:
    """Count total number of pairwise overlapping interval pairs."""
    length: int = len(starts)
    count: int = 0
    i: int = 0
    while i < length:
        j: int = i + 1
        while j < length:
            if starts[i] < ends[j] and starts[j] < ends[i]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def merge_overlapping_count(starts: list[int], ends: list[int]) -> int:
    """Count how many merged intervals result after merging all overlapping ones."""
    length: int = len(starts)
    if length == 0:
        return 0
    order: list[int] = sort_intervals_by_end(starts, ends)
    merged_count: int = 1
    current_end: int = ends[order[0]]
    current_start: int = starts[order[0]]
    idx: int = 1
    while idx < length:
        ci: int = order[idx]
        if starts[ci] < current_end and starts[ci] >= current_start:
            if ends[ci] > current_end:
                current_end = ends[ci]
        else:
            merged_count = merged_count + 1
            current_start = starts[ci]
            current_end = ends[ci]
        idx = idx + 1
    return merged_count


def test_module() -> int:
    passed: int = 0

    s: list[int] = [1, 3, 0, 5, 8, 5]
    e: list[int] = [2, 4, 6, 7, 9, 9]
    if max_non_overlapping(s, e) == 4:
        passed = passed + 1

    if max_non_overlapping([], []) == 0:
        passed = passed + 1

    s2: list[int] = [1, 2, 3]
    e2: list[int] = [3, 5, 4]
    if count_overlaps(s2, e2) == 2:
        passed = passed + 1

    s3: list[int] = [1, 2, 6]
    e3: list[int] = [5, 3, 8]
    if merge_overlapping_count(s3, e3) == 2:
        passed = passed + 1

    s4: list[int] = [0]
    e4: list[int] = [1]
    if max_non_overlapping(s4, e4) == 1:
        passed = passed + 1

    if count_overlaps(s4, e4) == 0:
        passed = passed + 1

    return passed
