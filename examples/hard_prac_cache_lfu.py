"""LFU cache eviction strategy simulation.

Simulates a Least Frequently Used cache using parallel lists for keys,
values, and frequency counters.
"""


def lfu_init_keys(capacity: int) -> list[int]:
    """Initialize empty key slots."""
    keys: list[int] = []
    i: int = 0
    while i < capacity:
        keys.append(0 - 1)
        i = i + 1
    return keys


def lfu_init_vals(capacity: int) -> list[int]:
    """Initialize value slots."""
    vals: list[int] = []
    i: int = 0
    while i < capacity:
        vals.append(0)
        i = i + 1
    return vals


def lfu_init_freqs(capacity: int) -> list[int]:
    """Initialize frequency counters."""
    freqs: list[int] = []
    i: int = 0
    while i < capacity:
        freqs.append(0)
        i = i + 1
    return freqs


def lfu_find(keys: list[int], target: int, capacity: int) -> int:
    """Find index of target key, or -1."""
    i: int = 0
    while i < capacity:
        slot: int = keys[i]
        if slot == target:
            return i
        i = i + 1
    return 0 - 1


def lfu_find_victim(freqs: list[int], keys: list[int], capacity: int) -> int:
    """Find slot with lowest frequency among valid entries, or first empty."""
    i: int = 0
    while i < capacity:
        slot_key: int = keys[i]
        if slot_key == (0 - 1):
            return i
        i = i + 1
    min_freq: int = freqs[0]
    min_idx: int = 0
    j: int = 1
    while j < capacity:
        f: int = freqs[j]
        if f < min_freq:
            min_freq = f
            min_idx = j
        j = j + 1
    return min_idx


def lfu_put(keys: list[int], vals: list[int], freqs: list[int],
            capacity: int, k: int, v: int) -> int:
    """Insert k->v. Returns 1 if eviction happened, 0 otherwise."""
    idx: int = lfu_find(keys, k, capacity)
    if idx >= 0:
        vals[idx] = v
        freqs[idx] = freqs[idx] + 1
        return 0
    victim: int = lfu_find_victim(freqs, keys, capacity)
    evicted: int = 0
    victim_key: int = keys[victim]
    if victim_key != (0 - 1):
        evicted = 1
    keys[victim] = k
    vals[victim] = v
    freqs[victim] = 1
    return evicted


def lfu_get(keys: list[int], vals: list[int], freqs: list[int],
            capacity: int, k: int) -> int:
    """Get value for key. Returns -1 on miss. Increments freq on hit."""
    idx: int = lfu_find(keys, k, capacity)
    if idx < 0:
        return 0 - 1
    freqs[idx] = freqs[idx] + 1
    result: int = vals[idx]
    return result


def lfu_get_freq(keys: list[int], freqs: list[int], capacity: int, k: int) -> int:
    """Get frequency count for a key. Returns 0 if not found."""
    idx: int = lfu_find(keys, k, capacity)
    if idx < 0:
        return 0
    result: int = freqs[idx]
    return result


def test_module() -> int:
    """Test LFU cache operations."""
    passed: int = 0
    cap: int = 3
    keys: list[int] = lfu_init_keys(cap)
    vals: list[int] = lfu_init_vals(cap)
    freqs: list[int] = lfu_init_freqs(cap)

    # Test 1: insert and retrieve
    lfu_put(keys, vals, freqs, cap, 10, 100)
    got: int = lfu_get(keys, vals, freqs, cap, 10)
    if got == 100:
        passed = passed + 1

    # Test 2: frequency tracking
    lfu_get(keys, vals, freqs, cap, 10)
    lfu_get(keys, vals, freqs, cap, 10)
    freq10: int = lfu_get_freq(keys, freqs, cap, 10)
    if freq10 == 4:
        passed = passed + 1

    # Test 3: fill and evict least frequent
    lfu_put(keys, vals, freqs, cap, 20, 200)
    lfu_put(keys, vals, freqs, cap, 30, 300)
    ev: int = lfu_put(keys, vals, freqs, cap, 40, 400)
    if ev == 1:
        passed = passed + 1

    # Test 4: most frequent key survives eviction
    survived: int = lfu_get(keys, vals, freqs, cap, 10)
    if survived == 100:
        passed = passed + 1

    # Test 5: miss on evicted key
    miss: int = lfu_get(keys, vals, freqs, cap, 20)
    if miss == (0 - 1):
        passed = passed + 1

    return passed
