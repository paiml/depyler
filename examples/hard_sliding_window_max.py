"""Sliding window maximum using deque simulation."""


def sliding_window_max(arr: list[int], k: int) -> list[int]:
    """Find maximum in each sliding window of size k."""
    n: int = len(arr)
    if n == 0 or k == 0:
        empty: list[int] = []
        return empty
    result: list[int] = []
    deque_idx: list[int] = []
    i: int = 0
    while i < n:
        while len(deque_idx) > 0 and deque_idx[0] < i - k + 1:
            deque_idx.pop(0)
        while len(deque_idx) > 0 and arr[deque_idx[len(deque_idx) - 1]] <= arr[i]:
            deque_idx.pop()
        deque_idx.append(i)
        if i >= k - 1:
            val: int = arr[deque_idx[0]]
            result.append(val)
        i = i + 1
    return result


def sliding_window_min(arr: list[int], k: int) -> list[int]:
    """Find minimum in each sliding window of size k."""
    n: int = len(arr)
    if n == 0 or k == 0:
        empty: list[int] = []
        return empty
    result: list[int] = []
    deque_idx: list[int] = []
    i: int = 0
    while i < n:
        while len(deque_idx) > 0 and deque_idx[0] < i - k + 1:
            deque_idx.pop(0)
        while len(deque_idx) > 0 and arr[deque_idx[len(deque_idx) - 1]] >= arr[i]:
            deque_idx.pop()
        deque_idx.append(i)
        if i >= k - 1:
            val: int = arr[deque_idx[0]]
            result.append(val)
        i = i + 1
    return result


def max_of_mins(arr: list[int], k: int) -> int:
    """Maximum of all sliding window minimums."""
    mins: list[int] = sliding_window_min(arr, k)
    n: int = len(mins)
    if n == 0:
        return 0
    best: int = mins[0]
    i: int = 1
    while i < n:
        if mins[i] > best:
            best = mins[i]
        i = i + 1
    return best


def test_module() -> int:
    """Test sliding window maximum."""
    passed: int = 0

    a1: list[int] = [1, 3, -1, -3, 5, 3, 6, 7]
    r1: list[int] = sliding_window_max(a1, 3)
    if len(r1) == 6:
        passed = passed + 1

    if r1[0] == 3 and r1[1] == 3 and r1[2] == 5:
        passed = passed + 1

    if r1[5] == 7:
        passed = passed + 1

    r2: list[int] = sliding_window_min(a1, 3)
    if r2[0] == 0 - 1:
        passed = passed + 1

    r3: list[int] = sliding_window_max([5], 1)
    if len(r3) == 1 and r3[0] == 5:
        passed = passed + 1

    if max_of_mins(a1, 3) == 5:
        passed = passed + 1

    return passed
