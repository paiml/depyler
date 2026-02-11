def sort_intervals(starts: list[int], ends: list[int]) -> list[list[int]]:
    n: int = len(starts)
    indices: list[int] = []
    i: int = 0
    while i < n:
        indices.append(i)
        i = i + 1
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if starts[indices[j]] < starts[indices[i]]:
                tmp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = tmp
            j = j + 1
        i = i + 1
    sorted_starts: list[int] = []
    sorted_ends: list[int] = []
    i = 0
    while i < n:
        sorted_starts.append(starts[indices[i]])
        sorted_ends.append(ends[indices[i]])
        i = i + 1
    return [sorted_starts, sorted_ends]


def merge_intervals(starts: list[int], ends: list[int]) -> list[list[int]]:
    if len(starts) == 0:
        return [[], []]
    sorted_pair: list[list[int]] = sort_intervals(starts, ends)
    ss: list[int] = sorted_pair[0]
    se: list[int] = sorted_pair[1]
    ms: list[int] = [ss[0]]
    me: list[int] = [se[0]]
    i: int = 1
    while i < len(ss):
        if ss[i] <= me[len(me) - 1]:
            if se[i] > me[len(me) - 1]:
                me[len(me) - 1] = se[i]
        else:
            ms.append(ss[i])
            me.append(se[i])
        i = i + 1
    return [ms, me]


def find_gaps(starts: list[int], ends: list[int]) -> list[list[int]]:
    merged: list[list[int]] = merge_intervals(starts, ends)
    ms: list[int] = merged[0]
    me: list[int] = merged[1]
    gs: list[int] = []
    ge: list[int] = []
    i: int = 0
    while i < len(ms) - 1:
        gs.append(me[i])
        ge.append(ms[i + 1])
        i = i + 1
    return [gs, ge]


def test_module() -> int:
    passed: int = 0
    r: list[list[int]] = merge_intervals([1, 3, 2, 6], [3, 5, 4, 8])
    if r[0] == [1, 6]:
        passed = passed + 1
    if r[1] == [5, 8]:
        passed = passed + 1
    r2: list[list[int]] = merge_intervals([1, 5], [3, 7])
    if r2[0] == [1]:
        passed = passed + 1
    if r2[1] == [7]:
        passed = passed + 1
    g: list[list[int]] = find_gaps([1, 5], [3, 7])
    if g[0] == [3]:
        passed = passed + 1
    if g[1] == [5]:
        passed = passed + 1
    empty: list[list[int]] = merge_intervals([], [])
    if empty[0] == []:
        passed = passed + 1
    return passed
