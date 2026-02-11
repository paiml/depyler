"""Real-world LRU cache with access tracking and eviction.

Mimics: functools.lru_cache, Redis/Memcached eviction policies.
Implements get/put with access timestamps, hit/miss counters.
"""


def cache_create(max_size: int) -> list[list[int]]:
    """Create LRU cache. Each slot is [occupied, stored_id, stored_val, access_time].
    occupied: 0=empty, 1=filled."""
    slots: list[list[int]] = []
    idx: int = 0
    while idx < max_size:
        slots.append([0, 0, 0, 0])
        idx = idx + 1
    return slots


def cache_find(slots: list[list[int]], stored_id: int) -> int:
    """Find slot index for a given id. Returns -1 if not found."""
    idx: int = 0
    while idx < len(slots):
        if slots[idx][0] == 1 and slots[idx][1] == stored_id:
            return idx
        idx = idx + 1
    return -1


def cache_get(slots: list[list[int]], stored_id: int, clock: int) -> list[int]:
    """Get value from cache. Returns [found, value, new_clock].
    Updates access time on hit."""
    slot_idx: int = cache_find(slots, stored_id)
    if slot_idx == -1:
        return [0, 0, clock]
    slots[slot_idx][3] = clock
    return [1, slots[slot_idx][2], clock + 1]


def cache_find_empty(slots: list[list[int]]) -> int:
    """Find first empty slot. Returns -1 if none."""
    idx: int = 0
    while idx < len(slots):
        if slots[idx][0] == 0:
            return idx
        idx = idx + 1
    return -1


def cache_find_lru(slots: list[list[int]]) -> int:
    """Find least recently used slot (lowest access_time among occupied)."""
    lru_idx: int = -1
    lru_time: int = 999999999
    idx: int = 0
    while idx < len(slots):
        if slots[idx][0] == 1 and slots[idx][3] < lru_time:
            lru_time = slots[idx][3]
            lru_idx = idx
        idx = idx + 1
    return lru_idx


def cache_put(slots: list[list[int]], stored_id: int, stored_val: int, clock: int) -> list[int]:
    """Put value in cache. Returns [evicted_id, new_clock]. evicted_id=-1 if no eviction."""
    # Check if already exists - update
    existing: int = cache_find(slots, stored_id)
    if existing != -1:
        slots[existing][2] = stored_val
        slots[existing][3] = clock
        return [-1, clock + 1]
    # Find empty slot
    empty: int = cache_find_empty(slots)
    if empty != -1:
        slots[empty][0] = 1
        slots[empty][1] = stored_id
        slots[empty][2] = stored_val
        slots[empty][3] = clock
        return [-1, clock + 1]
    # Evict LRU
    lru: int = cache_find_lru(slots)
    evicted: int = slots[lru][1]
    slots[lru][1] = stored_id
    slots[lru][2] = stored_val
    slots[lru][3] = clock
    return [evicted, clock + 1]


def cache_size(slots: list[list[int]]) -> int:
    """Count occupied slots."""
    count: int = 0
    idx: int = 0
    while idx < len(slots):
        if slots[idx][0] == 1:
            count = count + 1
        idx = idx + 1
    return count


def cache_invalidate(slots: list[list[int]], stored_id: int) -> bool:
    """Remove a specific entry from cache. Returns True if found."""
    slot_idx: int = cache_find(slots, stored_id)
    if slot_idx == -1:
        return False
    slots[slot_idx][0] = 0
    slots[slot_idx][1] = 0
    slots[slot_idx][2] = 0
    slots[slot_idx][3] = 0
    return True


def cache_clear(slots: list[list[int]]) -> int:
    """Clear all entries. Returns number cleared."""
    cleared: int = 0
    idx: int = 0
    while idx < len(slots):
        if slots[idx][0] == 1:
            cleared = cleared + 1
            slots[idx][0] = 0
            slots[idx][1] = 0
            slots[idx][2] = 0
            slots[idx][3] = 0
        idx = idx + 1
    return cleared


def test_module() -> int:
    """Test LRU cache module."""
    passed: int = 0

    # Test 1: create and check empty
    slots: list[list[int]] = cache_create(3)
    if cache_size(slots) == 0:
        passed = passed + 1

    # Test 2: put and get
    clock: int = 0
    res: list[int] = cache_put(slots, 10, 100, clock)
    clock = res[1]
    g: list[int] = cache_get(slots, 10, clock)
    if g[0] == 1 and g[1] == 100:
        passed = passed + 1

    # Test 3: cache miss
    clock = g[2]
    miss: list[int] = cache_get(slots, 99, clock)
    if miss[0] == 0:
        passed = passed + 1

    # Test 4: fill cache
    clock = miss[2]
    cache_put(slots, 20, 200, clock)
    clock = clock + 1
    cache_put(slots, 30, 300, clock)
    clock = clock + 1
    if cache_size(slots) == 3:
        passed = passed + 1

    # Test 5: eviction on overflow
    evict_res: list[int] = cache_put(slots, 40, 400, clock)
    clock = evict_res[1]
    if evict_res[0] >= 0:
        passed = passed + 1

    # Test 6: invalidate
    if cache_invalidate(slots, 40):
        passed = passed + 1

    # Test 7: size after invalidate
    if cache_size(slots) == 2:
        passed = passed + 1

    # Test 8: clear
    cleared: int = cache_clear(slots)
    if cleared == 2 and cache_size(slots) == 0:
        passed = passed + 1

    return passed
