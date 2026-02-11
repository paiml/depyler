"""Sliding window minimum using a monotonic deque (simulated with arrays)."""


def sliding_window_min(arr: list[int], win_size: int) -> list[int]:
    """Find minimum in each sliding window of size win_size."""
    n: int = len(arr)
    if n == 0 or win_size <= 0:
        result: list[int] = []
        return result
    if win_size > n:
        win_size = n
    deque_indices: list[int] = []
    idx: int = 0
    while idx < n:
        deque_indices.append(-1)
        idx = idx + 1
    dq_front: int = 0
    dq_back: int = 0
    result: list[int] = []
    i: int = 0
    while i < n:
        while dq_front < dq_back and deque_indices[dq_front] <= i - win_size:
            dq_front = dq_front + 1
        while dq_front < dq_back:
            tail_idx: int = dq_back - 1
            if arr[deque_indices[tail_idx]] >= arr[i]:
                dq_back = dq_back - 1
            else:
                break
        deque_indices[dq_back] = i
        dq_back = dq_back + 1
        if i >= win_size - 1:
            result.append(arr[deque_indices[dq_front]])
        i = i + 1
    return result


def sliding_window_max(arr: list[int], win_size: int) -> list[int]:
    """Find maximum in each sliding window of size win_size."""
    n: int = len(arr)
    if n == 0 or win_size <= 0:
        result: list[int] = []
        return result
    if win_size > n:
        win_size = n
    deque_indices: list[int] = []
    idx: int = 0
    while idx < n:
        deque_indices.append(-1)
        idx = idx + 1
    dq_front: int = 0
    dq_back: int = 0
    result: list[int] = []
    i: int = 0
    while i < n:
        while dq_front < dq_back and deque_indices[dq_front] <= i - win_size:
            dq_front = dq_front + 1
        while dq_front < dq_back:
            tail_idx: int = dq_back - 1
            if arr[deque_indices[tail_idx]] <= arr[i]:
                dq_back = dq_back - 1
            else:
                break
        deque_indices[dq_back] = i
        dq_back = dq_back + 1
        if i >= win_size - 1:
            result.append(arr[deque_indices[dq_front]])
        i = i + 1
    return result


def sliding_window_sum(arr: list[int], win_size: int) -> list[int]:
    """Sum of each sliding window."""
    n: int = len(arr)
    if n == 0 or win_size <= 0 or win_size > n:
        result: list[int] = []
        return result
    result: list[int] = []
    current_sum: int = 0
    i: int = 0
    while i < win_size:
        current_sum = current_sum + arr[i]
        i = i + 1
    result.append(current_sum)
    j: int = win_size
    while j < n:
        current_sum = current_sum + arr[j] - arr[j - win_size]
        result.append(current_sum)
        j = j + 1
    return result


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [1, 3, -1, -3, 5, 3, 6, 7]
    mins: list[int] = sliding_window_min(arr1, 3)
    if mins[0] == -1 and mins[1] == -3 and mins[2] == -3:
        passed = passed + 1

    maxs: list[int] = sliding_window_max(arr1, 3)
    if maxs[0] == 3 and maxs[1] == 3 and maxs[2] == 5:
        passed = passed + 1

    sums: list[int] = sliding_window_sum(arr1, 3)
    if sums[0] == 3 and sums[1] == -1:
        passed = passed + 1

    arr2: list[int] = [5, 4, 3, 2, 1]
    mins2: list[int] = sliding_window_min(arr2, 2)
    if mins2[0] == 4 and mins2[1] == 3 and mins2[2] == 2:
        passed = passed + 1

    arr3: list[int] = [1, 2, 3]
    maxs3: list[int] = sliding_window_max(arr3, 1)
    if maxs3[0] == 1 and maxs3[1] == 2 and maxs3[2] == 3:
        passed = passed + 1

    empty: list[int] = []
    empty_r: list[int] = sliding_window_min(empty, 3)
    if len(empty_r) == 0:
        passed = passed + 1

    sums2: list[int] = sliding_window_sum(arr2, 5)
    if sums2[0] == 15:
        passed = passed + 1

    return passed
