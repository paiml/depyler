"""Deque operations using array simulation.

Tests: deque push/pop from both ends, sliding window max.
"""


def deque_front_back_ops(ops: list[int], vals: list[int]) -> list[int]:
    """Simulate deque operations. 1=push_back, 2=push_front, 3=pop_back, 4=pop_front.
    Returns final deque contents."""
    deque: list[int] = []
    i: int = 0
    while i < len(ops):
        if ops[i] == 1:
            deque.append(vals[i])
        elif ops[i] == 2:
            new_deque: list[int] = [vals[i]]
            j: int = 0
            while j < len(deque):
                new_deque.append(deque[j])
                j = j + 1
            deque = new_deque
        elif ops[i] == 3:
            if len(deque) > 0:
                deque.pop()
        elif ops[i] == 4:
            if len(deque) > 0:
                new_deque2: list[int] = []
                k: int = 1
                while k < len(deque):
                    new_deque2.append(deque[k])
                    k = k + 1
                deque = new_deque2
        i = i + 1
    return deque


def sliding_window_max(arr: list[int], w: int) -> list[int]:
    """Compute max in each sliding window of size w."""
    result: list[int] = []
    n: int = len(arr)
    i: int = 0
    while i <= n - w:
        max_val: int = arr[i]
        j: int = i + 1
        while j < i + w:
            if arr[j] > max_val:
                max_val = arr[j]
            j = j + 1
        result.append(max_val)
        i = i + 1
    return result


def sliding_window_sum(arr: list[int], w: int) -> list[int]:
    """Compute sum in each sliding window of size w."""
    result: list[int] = []
    n: int = len(arr)
    if n < w:
        return result
    window_sum: int = 0
    i: int = 0
    while i < w:
        window_sum = window_sum + arr[i]
        i = i + 1
    result.append(window_sum)
    j: int = w
    while j < n:
        window_sum = window_sum + arr[j] - arr[j - w]
        result.append(window_sum)
        j = j + 1
    return result


def test_module() -> int:
    """Test deque operations."""
    ok: int = 0
    ops: list[int] = [1, 1, 2, 1]
    vals: list[int] = [1, 2, 0, 3]
    d: list[int] = deque_front_back_ops(ops, vals)
    if d[0] == 0:
        ok = ok + 1
    if d[1] == 1:
        ok = ok + 1
    if d[2] == 2:
        ok = ok + 1
    swm: list[int] = sliding_window_max([1, 3, -1, -3, 5, 3, 6, 7], 3)
    if swm[0] == 3:
        ok = ok + 1
    if swm[4] == 6:
        ok = ok + 1
    sws: list[int] = sliding_window_sum([1, 2, 3, 4, 5], 3)
    if sws[0] == 6:
        ok = ok + 1
    return ok
