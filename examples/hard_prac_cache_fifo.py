"""FIFO cache eviction strategy simulation.

First-In First-Out cache: evicts the oldest inserted entry when full.
"""


def fifo_init(capacity: int) -> list[int]:
    """Initialize list with -1 sentinel values."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def fifo_find(keys: list[int], target: int, capacity: int) -> int:
    """Find target in keys. Returns index or -1."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == target:
            return i
        i = i + 1
    return 0 - 1


def fifo_count(keys: list[int], capacity: int) -> int:
    """Count occupied slots."""
    count: int = 0
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem != (0 - 1):
            count = count + 1
        i = i + 1
    return count


def fifo_put(keys: list[int], vals: list[int], head_ptr: list[int],
             capacity: int, k: int, v: int) -> int:
    """Insert k->v into FIFO cache. Returns 1 if eviction, 0 otherwise.
    head_ptr is a single-element list holding the write position."""
    idx: int = fifo_find(keys, k, capacity)
    if idx >= 0:
        vals[idx] = v
        return 0
    pos: int = head_ptr[0]
    evicted: int = 0
    old_key: int = keys[pos]
    if old_key != (0 - 1):
        evicted = 1
    keys[pos] = k
    vals[pos] = v
    next_pos: int = (pos + 1) % capacity
    head_ptr[0] = next_pos
    return evicted


def fifo_get(keys: list[int], vals: list[int], capacity: int, k: int) -> int:
    """Get value for key. Returns -1 on miss."""
    idx: int = fifo_find(keys, k, capacity)
    if idx < 0:
        return 0 - 1
    result: int = vals[idx]
    return result


def fifo_remove(keys: list[int], vals: list[int], capacity: int, k: int) -> int:
    """Remove key from cache. Returns 1 if found, 0 otherwise."""
    idx: int = fifo_find(keys, k, capacity)
    if idx < 0:
        return 0
    keys[idx] = 0 - 1
    vals[idx] = 0
    return 1


def test_module() -> int:
    """Test FIFO cache operations."""
    passed: int = 0
    cap: int = 3
    keys: list[int] = fifo_init(cap)
    vals: list[int] = fifo_init(cap)
    head_ptr: list[int] = [0]

    # Test 1: insert and get
    fifo_put(keys, vals, head_ptr, cap, 10, 100)
    got: int = fifo_get(keys, vals, cap, 10)
    if got == 100:
        passed = passed + 1

    # Test 2: miss returns -1
    miss: int = fifo_get(keys, vals, cap, 99)
    if miss == (0 - 1):
        passed = passed + 1

    # Test 3: FIFO eviction order
    fifo_put(keys, vals, head_ptr, cap, 20, 200)
    fifo_put(keys, vals, head_ptr, cap, 30, 300)
    ev: int = fifo_put(keys, vals, head_ptr, cap, 40, 400)
    if ev == 1:
        passed = passed + 1

    # Test 4: first inserted key was evicted
    evicted_val: int = fifo_get(keys, vals, cap, 10)
    if evicted_val == (0 - 1):
        passed = passed + 1

    # Test 5: removal works
    removed: int = fifo_remove(keys, vals, cap, 20)
    if removed == 1:
        passed = passed + 1

    # Test 6: count after removal
    cnt: int = fifo_count(keys, cap)
    if cnt == 2:
        passed = passed + 1

    return passed
