"""Two-level cache simulation (L1 + L2).

L1 is small and fast. L2 is larger and slower.
On L1 miss, check L2 and promote to L1 on hit.
"""


def tl_init(size: int) -> list[int]:
    """Initialize with -1 sentinels."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def tl_find(keys: list[int], target: int, size: int) -> int:
    """Find target in keys. Returns index or -1."""
    i: int = 0
    while i < size:
        elem: int = keys[i]
        if elem == target:
            return i
        i = i + 1
    return 0 - 1


def tl_first_empty(keys: list[int], size: int) -> int:
    """First empty slot or -1."""
    i: int = 0
    while i < size:
        elem: int = keys[i]
        if elem == (0 - 1):
            return i
        i = i + 1
    return 0 - 1


def tl_evict_first(keys: list[int], vals: list[int], size: int) -> int:
    """Evict slot 0, shift everything left. Returns evicted key."""
    evicted: int = keys[0]
    j: int = 0
    while j < size - 1:
        nk: int = keys[j + 1]
        nv: int = vals[j + 1]
        keys[j] = nk
        vals[j] = nv
        j = j + 1
    keys[size - 1] = 0 - 1
    vals[size - 1] = 0 - 1
    return evicted


def tl_insert(keys: list[int], vals: list[int], size: int, k: int, v: int) -> int:
    """Insert into level. Evicts oldest if full. Returns 1 if eviction."""
    idx: int = tl_find(keys, k, size)
    if idx >= 0:
        vals[idx] = v
        return 0
    slot: int = tl_first_empty(keys, size)
    if slot >= 0:
        keys[slot] = k
        vals[slot] = v
        return 0
    tl_evict_first(keys, vals, size)
    slot2: int = tl_first_empty(keys, size)
    keys[slot2] = k
    vals[slot2] = v
    return 1


def tl_remove(keys: list[int], vals: list[int], size: int, k: int) -> int:
    """Remove key from level. Returns value or -1."""
    idx: int = tl_find(keys, k, size)
    if idx < 0:
        return 0 - 1
    result: int = vals[idx]
    keys[idx] = 0 - 1
    vals[idx] = 0 - 1
    return result


def tl_lookup(l1k: list[int], l1v: list[int], l1_size: int,
              l2k: list[int], l2v: list[int], l2_size: int,
              target: int) -> int:
    """Lookup in two-level cache. L1 first, then L2 with promotion."""
    l1_idx: int = tl_find(l1k, target, l1_size)
    if l1_idx >= 0:
        result: int = l1v[l1_idx]
        return result
    l2_idx: int = tl_find(l2k, target, l2_size)
    if l2_idx >= 0:
        val: int = l2v[l2_idx]
        tl_remove(l2k, l2v, l2_size, target)
        tl_insert(l1k, l1v, l1_size, target, val)
        return val
    return 0 - 1


def tl_count(keys: list[int], size: int) -> int:
    """Count valid entries."""
    count: int = 0
    i: int = 0
    while i < size:
        elem: int = keys[i]
        if elem != (0 - 1):
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test two-level cache."""
    passed: int = 0
    l1_sz: int = 2
    l2_sz: int = 4
    l1k: list[int] = tl_init(l1_sz)
    l1v: list[int] = tl_init(l1_sz)
    l2k: list[int] = tl_init(l2_sz)
    l2v: list[int] = tl_init(l2_sz)

    # Test 1: insert to L1 and find
    tl_insert(l1k, l1v, l1_sz, 10, 100)
    got: int = tl_lookup(l1k, l1v, l1_sz, l2k, l2v, l2_sz, 10)
    if got == 100:
        passed = passed + 1

    # Test 2: L2 lookup and promotion
    tl_insert(l2k, l2v, l2_sz, 20, 200)
    got2: int = tl_lookup(l1k, l1v, l1_sz, l2k, l2v, l2_sz, 20)
    if got2 == 200:
        passed = passed + 1

    # Test 3: after promotion, 20 is in L1
    in_l1: int = tl_find(l1k, 20, l1_sz)
    if in_l1 >= 0:
        passed = passed + 1

    # Test 4: miss returns -1
    miss: int = tl_lookup(l1k, l1v, l1_sz, l2k, l2v, l2_sz, 99)
    if miss == (0 - 1):
        passed = passed + 1

    # Test 5: L1 eviction when full
    tl_insert(l1k, l1v, l1_sz, 30, 300)
    cnt: int = tl_count(l1k, l1_sz)
    if cnt == 2:
        passed = passed + 1

    return passed
