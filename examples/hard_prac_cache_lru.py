"""LRU cache eviction strategy simulation.

Simulates a Least Recently Used cache using parallel lists for keys,
values, and access timestamps.
"""


def lru_init_keys(capacity: int) -> list[int]:
    """Initialize empty key slots with -1."""
    keys: list[int] = []
    i: int = 0
    while i < capacity:
        keys.append(0 - 1)
        i = i + 1
    return keys


def lru_init_vals(capacity: int) -> list[int]:
    """Initialize empty value slots."""
    vals: list[int] = []
    i: int = 0
    while i < capacity:
        vals.append(0)
        i = i + 1
    return vals


def lru_init_times(capacity: int) -> list[int]:
    """Initialize access timestamps to 0."""
    times: list[int] = []
    i: int = 0
    while i < capacity:
        times.append(0)
        i = i + 1
    return times


def lru_find(keys: list[int], target: int, capacity: int) -> int:
    """Find index of target in keys, or -1 if not found."""
    i: int = 0
    while i < capacity:
        slot: int = keys[i]
        if slot == target:
            return i
        i = i + 1
    return 0 - 1


def lru_find_victim(times: list[int], keys: list[int], capacity: int) -> int:
    """Find the least recently used slot (oldest timestamp among valid entries, or first empty)."""
    i: int = 0
    while i < capacity:
        slot_key: int = keys[i]
        if slot_key == (0 - 1):
            return i
        i = i + 1
    min_time: int = times[0]
    min_idx: int = 0
    j: int = 1
    while j < capacity:
        t: int = times[j]
        if t < min_time:
            min_time = t
            min_idx = j
        j = j + 1
    return min_idx


def lru_put(keys: list[int], vals: list[int], times: list[int],
            capacity: int, k: int, v: int, clock: int) -> int:
    """Put k->v into cache, evicting LRU if needed. Returns new clock."""
    idx: int = lru_find(keys, k, capacity)
    new_clock: int = clock + 1
    if idx >= 0:
        vals[idx] = v
        times[idx] = new_clock
        return new_clock
    victim: int = lru_find_victim(times, keys, capacity)
    keys[victim] = k
    vals[victim] = v
    times[victim] = new_clock
    return new_clock


def lru_get(keys: list[int], vals: list[int], times: list[int],
            capacity: int, k: int, clock: int) -> int:
    """Get value for key k. Returns -1 if miss. Updates access time on hit."""
    idx: int = lru_find(keys, k, capacity)
    if idx < 0:
        return 0 - 1
    times[idx] = clock + 1
    result: int = vals[idx]
    return result


def lru_count_valid(keys: list[int], capacity: int) -> int:
    """Count how many slots are occupied."""
    count: int = 0
    i: int = 0
    while i < capacity:
        slot: int = keys[i]
        if slot != (0 - 1):
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test LRU cache operations."""
    passed: int = 0
    cap: int = 3
    keys: list[int] = lru_init_keys(cap)
    vals: list[int] = lru_init_vals(cap)
    times: list[int] = lru_init_times(cap)

    # Test 1: insert and retrieve
    clk: int = lru_put(keys, vals, times, cap, 10, 100, 0)
    got: int = lru_get(keys, vals, times, cap, 10, clk)
    if got == 100:
        passed = passed + 1

    # Test 2: miss returns -1
    miss: int = lru_get(keys, vals, times, cap, 99, clk)
    if miss == (0 - 1):
        passed = passed + 1

    # Test 3: fill cache
    clk = lru_put(keys, vals, times, cap, 20, 200, clk)
    clk = lru_put(keys, vals, times, cap, 30, 300, clk)
    valid: int = lru_count_valid(keys, cap)
    if valid == 3:
        passed = passed + 1

    # Test 4: eviction of LRU on overflow
    clk = lru_put(keys, vals, times, cap, 40, 400, clk)
    evicted: int = lru_get(keys, vals, times, cap, 10, clk)
    if evicted == (0 - 1):
        passed = passed + 1

    # Test 5: update existing key
    clk = lru_put(keys, vals, times, cap, 20, 999, clk)
    updated: int = lru_get(keys, vals, times, cap, 20, clk)
    if updated == 999:
        passed = passed + 1

    # Test 6: count still 3 after eviction + update
    valid2: int = lru_count_valid(keys, cap)
    if valid2 == 3:
        passed = passed + 1

    return passed
