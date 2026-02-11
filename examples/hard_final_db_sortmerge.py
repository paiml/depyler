"""External sort-merge algorithm simulation.

Implements multi-way merge sort for data that doesn't fit in memory.
Simulates runs, merge passes, and replacement selection.
"""


def create_run(data: list[int], start: int, length: int) -> list[int]:
    """Extract and sort a run of data."""
    run_data: list[int] = []
    i: int = 0
    while i < length:
        if start + i < len(data):
            dv: int = data[start + i]
            run_data.append(dv)
        i = i + 1
    j: int = 0
    while j < len(run_data):
        k: int = j + 1
        while k < len(run_data):
            vj: int = run_data[j]
            vk: int = run_data[k]
            if vj > vk:
                run_data[j] = vk
                run_data[k] = vj
            k = k + 1
        j = j + 1
    return run_data


def merge_two_runs(run_a: list[int], run_b: list[int]) -> list[int]:
    """Merge two sorted runs into one sorted run."""
    result: list[int] = []
    ia: int = 0
    ib: int = 0
    while ia < len(run_a):
        if ib < len(run_b):
            va: int = run_a[ia]
            vb: int = run_b[ib]
            if va <= vb:
                result.append(va)
                ia = ia + 1
            else:
                result.append(vb)
                ib = ib + 1
        else:
            va2: int = run_a[ia]
            result.append(va2)
            ia = ia + 1
    while ib < len(run_b):
        vb2: int = run_b[ib]
        result.append(vb2)
        ib = ib + 1
    return result


def external_sort(data: list[int], run_size: int) -> list[int]:
    """External sort: create runs of run_size, then merge pairwise."""
    runs: list[list[int]] = []
    pos: int = 0
    while pos < len(data):
        remaining: int = len(data) - pos
        actual_size: int = run_size
        if remaining < run_size:
            actual_size = remaining
        run_data: list[int] = create_run(data, pos, actual_size)
        runs.append(run_data)
        pos = pos + run_size
    while len(runs) > 1:
        new_runs: list[list[int]] = []
        ri: int = 0
        while ri < len(runs):
            if ri + 1 < len(runs):
                merged: list[int] = merge_two_runs(runs[ri], runs[ri + 1])
                new_runs.append(merged)
            else:
                new_runs.append(runs[ri])
            ri = ri + 2
        runs = new_runs
    if len(runs) == 0:
        return []
    return runs[0]


def is_sorted(arr: list[int]) -> int:
    """Check if array is sorted. Returns 1 if sorted."""
    i: int = 1
    while i < len(arr):
        prev: int = arr[i - 1]
        curr: int = arr[i]
        if prev > curr:
            return 0
        i = i + 1
    return 1


def count_inversions(arr: list[int]) -> int:
    """Count number of inversions (pairs where a[i] > a[j] and i < j)."""
    cnt: int = 0
    i: int = 0
    while i < len(arr):
        j: int = i + 1
        while j < len(arr):
            vi: int = arr[i]
            vj: int = arr[j]
            if vi > vj:
                cnt = cnt + 1
            j = j + 1
        i = i + 1
    return cnt


def test_module() -> int:
    """Test sort-merge algorithm."""
    ok: int = 0
    data: list[int] = [8, 3, 7, 1, 5, 2, 6, 4]
    sorted_data: list[int] = external_sort(data, 2)
    if is_sorted(sorted_data) == 1:
        ok = ok + 1
    if len(sorted_data) == 8:
        ok = ok + 1
    v0: int = sorted_data[0]
    v7: int = sorted_data[7]
    if v0 == 1:
        ok = ok + 1
    if v7 == 8:
        ok = ok + 1
    m: list[int] = merge_two_runs([1, 3, 5], [2, 4, 6])
    if is_sorted(m) == 1:
        ok = ok + 1
    return ok
