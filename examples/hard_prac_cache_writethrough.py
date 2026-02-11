"""Write-through cache simulation.

Every write goes to both cache and backing store simultaneously.
No dirty tracking needed; backing store always consistent.
"""


def wt_init(capacity: int) -> list[int]:
    """Initialize with -1 sentinels."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def wt_find(keys: list[int], target: int, capacity: int) -> int:
    """Find key index or return -1."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == target:
            return i
        i = i + 1
    return 0 - 1


def wt_first_empty(keys: list[int], capacity: int) -> int:
    """Find first empty slot or return -1."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == (0 - 1):
            return i
        i = i + 1
    return 0 - 1


def wt_backing_write(store: list[int], store_size: int, k: int, v: int) -> int:
    """Write key-value to flat backing store. Returns 1 on success."""
    i: int = 0
    while i < store_size:
        sk: int = store[i * 2]
        if sk == k:
            store[i * 2 + 1] = v
            return 1
        if sk == (0 - 1):
            store[i * 2] = k
            store[i * 2 + 1] = v
            return 1
        i = i + 1
    return 0


def wt_backing_read(store: list[int], store_size: int, k: int) -> int:
    """Read from backing store. Returns -1 on miss."""
    i: int = 0
    while i < store_size:
        sk: int = store[i * 2]
        if sk == k:
            result: int = store[i * 2 + 1]
            return result
        i = i + 1
    return 0 - 1


def wt_write(keys: list[int], vals: list[int], capacity: int,
             store: list[int], store_size: int, k: int, v: int) -> int:
    """Write-through: write to cache AND backing store. Returns 1 on success."""
    idx: int = wt_find(keys, k, capacity)
    if idx >= 0:
        vals[idx] = v
        wt_backing_write(store, store_size, k, v)
        return 1
    slot: int = wt_first_empty(keys, capacity)
    if slot < 0:
        return 0
    keys[slot] = k
    vals[slot] = v
    wt_backing_write(store, store_size, k, v)
    return 1


def wt_read(keys: list[int], vals: list[int], capacity: int, k: int) -> int:
    """Read from cache. Returns -1 on miss."""
    idx: int = wt_find(keys, k, capacity)
    if idx < 0:
        return 0 - 1
    result: int = vals[idx]
    return result


def wt_invalidate(keys: list[int], vals: list[int], capacity: int, k: int) -> int:
    """Remove from cache only (backing store keeps data). Returns 1 if found."""
    idx: int = wt_find(keys, k, capacity)
    if idx < 0:
        return 0
    keys[idx] = 0 - 1
    vals[idx] = 0 - 1
    return 1


def test_module() -> int:
    """Test write-through cache operations."""
    passed: int = 0
    cap: int = 4
    keys: list[int] = wt_init(cap)
    vals: list[int] = wt_init(cap)
    store: list[int] = wt_init(8)

    # Test 1: write-through and cache read
    wt_write(keys, vals, cap, store, 4, 10, 100)
    got: int = wt_read(keys, vals, cap, 10)
    if got == 100:
        passed = passed + 1

    # Test 2: backing store also has data
    bg: int = wt_backing_read(store, 4, 10)
    if bg == 100:
        passed = passed + 1

    # Test 3: update goes to both
    wt_write(keys, vals, cap, store, 4, 10, 999)
    cache_val: int = wt_read(keys, vals, cap, 10)
    store_val: int = wt_backing_read(store, 4, 10)
    if cache_val == 999:
        if store_val == 999:
            passed = passed + 1

    # Test 4: invalidate removes from cache only
    wt_invalidate(keys, vals, cap, 10)
    miss: int = wt_read(keys, vals, cap, 10)
    still_in_store: int = wt_backing_read(store, 4, 10)
    if miss == (0 - 1):
        if still_in_store == 999:
            passed = passed + 1

    # Test 5: multiple writes work
    wt_write(keys, vals, cap, store, 4, 20, 200)
    wt_write(keys, vals, cap, store, 4, 30, 300)
    g20: int = wt_read(keys, vals, cap, 20)
    g30: int = wt_read(keys, vals, cap, 30)
    if g20 == 200:
        if g30 == 300:
            passed = passed + 1

    return passed
