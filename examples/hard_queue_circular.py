"""Queue operations using list append and index reading.

Tests: enqueue/dequeue simulation, queue drain, queue sum, max in queue.
"""


def queue_drain_sum(values: list[int]) -> int:
    """Simulate enqueue all then dequeue all, summing results."""
    queue: list[int] = []
    for v in values:
        queue.append(v)
    total: int = 0
    i: int = 0
    while i < len(queue):
        total = total + queue[i]
        i = i + 1
    return total


def queue_max(values: list[int]) -> int:
    """Find maximum in a queue (list)."""
    if len(values) == 0:
        return 0
    best: int = values[0]
    i: int = 1
    while i < len(values):
        if values[i] > best:
            best = values[i]
        i = i + 1
    return best


def queue_reverse(values: list[int]) -> list[int]:
    """Reverse a queue."""
    result: list[int] = []
    i: int = len(values) - 1
    while i >= 0:
        result.append(values[i])
        i = i - 1
    return result


def queue_rotate(values: list[int], k: int) -> list[int]:
    """Rotate queue: move first k elements to end."""
    n: int = len(values)
    if n == 0:
        return values
    kk: int = k % n
    result: list[int] = []
    i: int = kk
    while i < n:
        result.append(values[i])
        i = i + 1
    i = 0
    while i < kk:
        result.append(values[i])
        i = i + 1
    return result


def queue_count_above(values: list[int], threshold: int) -> int:
    """Count elements above threshold."""
    count: int = 0
    for v in values:
        if v > threshold:
            count = count + 1
    return count


def test_module() -> int:
    """Test queue operations."""
    ok: int = 0
    if queue_drain_sum([10, 20, 30]) == 60:
        ok = ok + 1
    if queue_max([5, 99, 3, 50]) == 99:
        ok = ok + 1
    r: list[int] = queue_reverse([1, 2, 3])
    if r[0] == 3 and r[1] == 2 and r[2] == 1:
        ok = ok + 1
    rot: list[int] = queue_rotate([1, 2, 3, 4, 5], 2)
    if rot[0] == 3 and rot[1] == 4:
        ok = ok + 1
    if queue_count_above([1, 5, 10, 15], 7) == 2:
        ok = ok + 1
    if queue_drain_sum([]) == 0:
        ok = ok + 1
    return ok
