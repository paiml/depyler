"""Balance array operations.

Implements operations for balancing arrays, finding equilibrium
points, and redistributing values.
"""


def find_equilibrium(arr: list[int], size: int) -> int:
    """Find index where left sum equals right sum. Returns -1 if none."""
    total: int = 0
    i: int = 0
    while i < size:
        total = total + arr[i]
        i = i + 1

    left_sum: int = 0
    j: int = 0
    while j < size:
        right_sum: int = total - left_sum - arr[j]
        if left_sum == right_sum:
            return j
        left_sum = left_sum + arr[j]
        j = j + 1
    return -1


def balance_redistribute(arr: list[int], size: int) -> list[int]:
    """Redistribute values so each element equals the average (integer division)."""
    total: int = 0
    i: int = 0
    while i < size:
        total = total + arr[i]
        i = i + 1
    avg: int = total // size
    remainder: int = total - avg * size
    result: list[int] = []
    j: int = 0
    while j < size:
        if j < remainder:
            result.append(avg + 1)
        else:
            result.append(avg)
        j = j + 1
    return result


def min_moves_to_equal(arr: list[int], size: int) -> int:
    """Compute minimum moves to make all elements equal to the median value."""
    sorted_arr: list[int] = []
    i: int = 0
    while i < size:
        sorted_arr.append(arr[i])
        i = i + 1

    si: int = 0
    while si < size - 1:
        sj: int = 0
        while sj < size - 1 - si:
            next_j: int = sj + 1
            if sorted_arr[sj] > sorted_arr[next_j]:
                tmp: int = sorted_arr[sj]
                sorted_arr[sj] = sorted_arr[next_j]
                sorted_arr[next_j] = tmp
            sj = sj + 1
        si = si + 1

    median_idx: int = size // 2
    median: int = sorted_arr[median_idx]
    moves: int = 0
    k: int = 0
    while k < size:
        diff: int = arr[k] - median
        if diff < 0:
            diff = -diff
        moves = moves + diff
        k = k + 1
    return moves


def is_balanced_partition(arr: list[int], size: int, split: int) -> int:
    """Check if splitting at index split gives equal sums. Returns 1 if yes."""
    left: int = 0
    i: int = 0
    while i < split:
        left = left + arr[i]
        i = i + 1
    right: int = 0
    j: int = split
    while j < size:
        right = right + arr[j]
        j = j + 1
    if left == right:
        return 1
    return 0


def test_module() -> int:
    """Test array balance operations."""
    ok: int = 0

    arr1: list[int] = [1, 3, 5, 2, 2]
    eq: int = find_equilibrium(arr1, 5)
    if eq == 2:
        ok = ok + 1

    arr2: list[int] = [10, 20, 30]
    tmp_result: list[int] = balance_redistribute(arr2, 3)
    if tmp_result[0] == 20 and tmp_result[1] == 20 and tmp_result[2] == 20:
        ok = ok + 1

    arr3: list[int] = [1, 2, 3]
    moves: int = min_moves_to_equal(arr3, 3)
    if moves == 2:
        ok = ok + 1

    arr4: list[int] = [1, 2, 3, 3, 2, 1]
    bal: int = is_balanced_partition(arr4, 6, 3)
    if bal == 1:
        ok = ok + 1

    return ok
