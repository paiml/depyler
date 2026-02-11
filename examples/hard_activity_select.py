"""Activity selection problem: greedy algorithm on intervals."""


def sort_by_finish(starts: list[int], finishes: list[int]) -> list[int]:
    """Return indices sorted by finish time (bubble sort on finish times)."""
    length: int = len(finishes)
    indices: list[int] = []
    idx: int = 0
    while idx < length:
        indices.append(idx)
        idx = idx + 1
    i: int = 0
    while i < length:
        j: int = i + 1
        while j < length:
            if finishes[indices[j]] < finishes[indices[i]]:
                temp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = temp
            j = j + 1
        i = i + 1
    return indices


def max_activities(starts: list[int], finishes: list[int]) -> int:
    """Return maximum number of non-overlapping activities."""
    length: int = len(starts)
    if length == 0:
        return 0
    order: list[int] = sort_by_finish(starts, finishes)
    count: int = 1
    last_finish: int = finishes[order[0]]
    idx: int = 1
    while idx < length:
        curr: int = order[idx]
        if starts[curr] >= last_finish:
            count = count + 1
            last_finish = finishes[curr]
        idx = idx + 1
    return count


def selected_activities(starts: list[int], finishes: list[int]) -> list[int]:
    """Return indices of selected activities (original indices)."""
    length: int = len(starts)
    if length == 0:
        result_empty: list[int] = []
        return result_empty
    order: list[int] = sort_by_finish(starts, finishes)
    result: list[int] = [order[0]]
    last_finish: int = finishes[order[0]]
    idx: int = 1
    while idx < length:
        curr: int = order[idx]
        if starts[curr] >= last_finish:
            result.append(curr)
            last_finish = finishes[curr]
        idx = idx + 1
    return result


def test_module() -> int:
    passed: int = 0

    starts: list[int] = [1, 3, 0, 5, 8, 5]
    finishes: list[int] = [2, 4, 6, 7, 9, 9]

    if max_activities(starts, finishes) == 4:
        passed = passed + 1

    sel: list[int] = selected_activities(starts, finishes)
    if len(sel) == 4:
        passed = passed + 1

    if max_activities([], []) == 0:
        passed = passed + 1

    s2: list[int] = [0, 1]
    f2: list[int] = [10, 2]
    if max_activities(s2, f2) == 1:
        passed = passed + 1

    s3: list[int] = [1, 2, 3]
    f3: list[int] = [2, 3, 4]
    if max_activities(s3, f3) == 3:
        passed = passed + 1

    sel3: list[int] = selected_activities(s3, f3)
    if len(sel3) == 3:
        passed = passed + 1

    return passed
