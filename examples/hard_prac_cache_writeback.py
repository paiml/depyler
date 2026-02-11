"""Write-back cache simulation.

Dirty-bit tracking: writes go to cache only, marked dirty.
Flushing writes dirty entries back to backing store.
"""


def wb_init(capacity: int) -> list[int]:
    """Initialize with -1 sentinels."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def wb_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def wb_find(keys: list[int], target: int, capacity: int) -> int:
    """Find key index or -1."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == target:
            return i
        i = i + 1
    return 0 - 1


def wb_first_empty(keys: list[int], capacity: int) -> int:
    """Find first empty slot or -1."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == (0 - 1):
            return i
        i = i + 1
    return 0 - 1


def wb_write(keys: list[int], vals: list[int], dirty: list[int],
             capacity: int, k: int, v: int) -> int:
    """Write to cache, mark dirty. Returns 1 on success, 0 if full."""
    idx: int = wb_find(keys, k, capacity)
    if idx >= 0:
        vals[idx] = v
        dirty[idx] = 1
        return 1
    slot: int = wb_first_empty(keys, capacity)
    if slot < 0:
        return 0
    keys[slot] = k
    vals[slot] = v
    dirty[slot] = 1
    return 1


def wb_read(keys: list[int], vals: list[int], capacity: int, k: int) -> int:
    """Read from cache. Returns -1 on miss."""
    idx: int = wb_find(keys, k, capacity)
    if idx < 0:
        return 0 - 1
    result: int = vals[idx]
    return result


def wb_count_dirty(dirty: list[int], capacity: int) -> int:
    """Count dirty entries."""
    count: int = 0
    i: int = 0
    while i < capacity:
        d: int = dirty[i]
        if d == 1:
            count = count + 1
        i = i + 1
    return count


def wb_flush(keys: list[int], vals: list[int], dirty: list[int],
             backing: list[int], capacity: int, backing_size: int) -> int:
    """Flush dirty entries to backing store. Returns count flushed."""
    flushed: int = 0
    i: int = 0
    while i < capacity:
        d: int = dirty[i]
        if d == 1:
            k: int = keys[i]
            v: int = vals[i]
            j: int = 0
            while j < backing_size:
                bk: int = backing[j * 2]
                if bk == k:
                    backing[j * 2 + 1] = v
                    dirty[i] = 0
                    flushed = flushed + 1
                    j = backing_size
                if bk == (0 - 1):
                    backing[j * 2] = k
                    backing[j * 2 + 1] = v
                    dirty[i] = 0
                    flushed = flushed + 1
                    j = backing_size
                j = j + 1
        i = i + 1
    return flushed


def wb_init_backing(size: int) -> list[int]:
    """Initialize backing store as flat key-value pairs."""
    result: list[int] = []
    i: int = 0
    while i < size * 2:
        result.append(0 - 1)
        i = i + 1
    return result


def test_module() -> int:
    """Test write-back cache operations."""
    passed: int = 0
    cap: int = 4
    keys: list[int] = wb_init(cap)
    vals: list[int] = wb_init(cap)
    dirty: list[int] = wb_init_zeros(cap)
    backing: list[int] = wb_init_backing(8)

    # Test 1: write and read
    wb_write(keys, vals, dirty, cap, 10, 100)
    got: int = wb_read(keys, vals, cap, 10)
    if got == 100:
        passed = passed + 1

    # Test 2: entry is dirty after write
    dc: int = wb_count_dirty(dirty, cap)
    if dc == 1:
        passed = passed + 1

    # Test 3: flush writes to backing store
    wb_write(keys, vals, dirty, cap, 20, 200)
    flushed: int = wb_flush(keys, vals, dirty, backing, cap, 8)
    if flushed == 2:
        passed = passed + 1

    # Test 4: dirty count is 0 after flush
    dc2: int = wb_count_dirty(dirty, cap)
    if dc2 == 0:
        passed = passed + 1

    # Test 5: backing store has correct value
    bv: int = backing[1]
    if bv == 100:
        passed = passed + 1

    return passed
