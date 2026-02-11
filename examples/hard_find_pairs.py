"""Find pairs in arrays with specific properties.

Tests: pair sum, pair difference, count pairs, closest pair sum.
"""


def count_pair_sums(arr: list[int], target: int) -> int:
    """Count pairs (i,j) with i<j where arr[i]+arr[j] == target."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if arr[i] + arr[j] == target:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def count_pair_diffs(arr: list[int], target: int) -> int:
    """Count pairs (i,j) with i<j where |arr[i]-arr[j]| == target."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            diff: int = arr[i] - arr[j]
            if diff < 0:
                diff = -diff
            if diff == target:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def closest_pair_sum(arr: list[int], target: int) -> int:
    """Find pair sum closest to target. Returns that sum."""
    n: int = len(arr)
    if n < 2:
        return 0
    best: int = arr[0] + arr[1]
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            s: int = arr[i] + arr[j]
            diff_best: int = best - target
            if diff_best < 0:
                diff_best = -diff_best
            diff_s: int = s - target
            if diff_s < 0:
                diff_s = -diff_s
            if diff_s < diff_best:
                best = s
            j = j + 1
        i = i + 1
    return best


def max_pair_product(arr: list[int]) -> int:
    """Maximum product of any two elements."""
    n: int = len(arr)
    if n < 2:
        return 0
    best: int = arr[0] * arr[1]
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            prod: int = arr[i] * arr[j]
            if prod > best:
                best = prod
            j = j + 1
        i = i + 1
    return best


def test_module() -> int:
    """Test find pairs."""
    ok: int = 0
    if count_pair_sums([1, 2, 3, 4, 5], 5) == 2:
        ok = ok + 1
    if count_pair_diffs([1, 5, 3, 4, 2], 2) == 3:
        ok = ok + 1
    if closest_pair_sum([1, 3, 5, 7], 10) == 12:
        ok = ok + 1
    if max_pair_product([1, 5, 3, 7, 2]) == 35:
        ok = ok + 1
    if count_pair_sums([1, 1, 1], 2) == 3:
        ok = ok + 1
    return ok
