"""Circular buffer operations using fixed-size array.

Tests: enqueue, dequeue, is full, peek.
"""


def circular_buffer_ops(capacity: int, ops: list[int], vals: list[int]) -> list[int]:
    """Simulate circular buffer. ops: 1=enqueue, 2=dequeue, 3=peek.
    Returns list of peek/dequeue results (-1 if empty)."""
    buf: list[int] = []
    i: int = 0
    while i < capacity:
        buf.append(0)
        i = i + 1
    head: int = 0
    tail: int = 0
    size: int = 0
    results: list[int] = []
    j: int = 0
    while j < len(ops):
        if ops[j] == 1:
            if size < capacity:
                buf[tail] = vals[j]
                tail = (tail + 1) % capacity
                size = size + 1
        elif ops[j] == 2:
            if size > 0:
                results.append(buf[head])
                head = (head + 1) % capacity
                size = size - 1
            else:
                results.append(-1)
        elif ops[j] == 3:
            if size > 0:
                results.append(buf[head])
            else:
                results.append(-1)
        j = j + 1
    return results


def ring_buffer_sum(buf: list[int], start: int, count: int) -> int:
    """Sum count elements from circular buffer starting at start."""
    n: int = len(buf)
    total: int = 0
    i: int = 0
    while i < count:
        idx: int = (start + i) % n
        total = total + buf[idx]
        i = i + 1
    return total


def rotate_array(arr: list[int], k: int) -> list[int]:
    """Rotate array right by k positions (circular shift)."""
    n: int = len(arr)
    if n == 0:
        return []
    k2: int = k % n
    result: list[int] = []
    i: int = 0
    while i < n:
        src: int = (i - k2 + n) % n
        result.append(arr[src])
        i = i + 1
    return result


def test_module() -> int:
    """Test circular buffer operations."""
    ok: int = 0
    ops: list[int] = [1, 1, 1, 3, 2, 3]
    vals: list[int] = [10, 20, 30, 0, 0, 0]
    results: list[int] = circular_buffer_ops(3, ops, vals)
    if results[0] == 10:
        ok = ok + 1
    if results[1] == 10:
        ok = ok + 1
    if results[2] == 20:
        ok = ok + 1
    buf: list[int] = [1, 2, 3, 4, 5]
    if ring_buffer_sum(buf, 3, 3) == 12:
        ok = ok + 1
    rot: list[int] = rotate_array([1, 2, 3, 4, 5], 2)
    if rot[0] == 4:
        ok = ok + 1
    if rot[1] == 5:
        ok = ok + 1
    return ok
