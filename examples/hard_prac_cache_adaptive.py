"""Adaptive cache simulation.

Monitors hit rates and dynamically adjusts between LRU and LFU
eviction strategies based on recent performance.
"""


def adap_init(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def adap_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def adap_find(keys: list[int], target: int, capacity: int) -> int:
    """Find target. Returns index or -1."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == target:
            return i
        i = i + 1
    return 0 - 1


def adap_victim_lru(times: list[int], keys: list[int], capacity: int) -> int:
    """LRU victim: oldest access time."""
    min_t: int = 2147483647
    min_idx: int = 0
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == (0 - 1):
            return i
        t: int = times[i]
        if t < min_t:
            min_t = t
            min_idx = i
        i = i + 1
    return min_idx


def adap_victim_lfu(freqs: list[int], keys: list[int], capacity: int) -> int:
    """LFU victim: lowest frequency."""
    min_f: int = 2147483647
    min_idx: int = 0
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == (0 - 1):
            return i
        f: int = freqs[i]
        if f < min_f:
            min_f = f
            min_idx = i
        i = i + 1
    return min_idx


def adap_access(keys: list[int], vals: list[int], times: list[int],
                freqs: list[int], capacity: int, k: int, v: int,
                clock: int, lru_hits: int, lfu_hits: int,
                stats: list[int]) -> int:
    """Access cache adaptively. stats[0]=lru_hits, stats[1]=lfu_hits.
    Returns clock+1."""
    new_clock: int = clock + 1
    idx: int = adap_find(keys, k, capacity)
    if idx >= 0:
        vals[idx] = v
        times[idx] = new_clock
        freqs[idx] = freqs[idx] + 1
        stats[0] = stats[0] + 1
        stats[1] = stats[1] + 1
        return new_clock
    s0: int = stats[0]
    s1: int = stats[1]
    victim: int = 0
    if s0 >= s1:
        victim = adap_victim_lru(times, keys, capacity)
    if s0 < s1:
        victim = adap_victim_lfu(freqs, keys, capacity)
    keys[victim] = k
    vals[victim] = v
    times[victim] = new_clock
    freqs[victim] = 1
    return new_clock


def adap_get(keys: list[int], vals: list[int], times: list[int],
             freqs: list[int], capacity: int, k: int, clock: int) -> int:
    """Get value. Returns -1 on miss. Updates time and freq on hit."""
    idx: int = adap_find(keys, k, capacity)
    if idx < 0:
        return 0 - 1
    times[idx] = clock + 1
    freqs[idx] = freqs[idx] + 1
    result: int = vals[idx]
    return result


def adap_total_freq(freqs: list[int], keys: list[int], capacity: int) -> int:
    """Sum of all frequencies for valid entries."""
    total: int = 0
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem != (0 - 1):
            f: int = freqs[i]
            total = total + f
        i = i + 1
    return total


def test_module() -> int:
    """Test adaptive cache."""
    passed: int = 0
    cap: int = 3
    keys: list[int] = adap_init(cap)
    vals: list[int] = adap_init(cap)
    times: list[int] = adap_init_zeros(cap)
    freqs: list[int] = adap_init_zeros(cap)
    stats: list[int] = [0, 0]

    # Test 1: insert and retrieve
    clk: int = adap_access(keys, vals, times, freqs, cap, 10, 100, 0, 0, 0, stats)
    got: int = adap_get(keys, vals, times, freqs, cap, 10, clk)
    if got == 100:
        passed = passed + 1

    # Test 2: miss returns -1
    miss: int = adap_get(keys, vals, times, freqs, cap, 99, clk)
    if miss == (0 - 1):
        passed = passed + 1

    # Test 3: frequency increments on access
    adap_get(keys, vals, times, freqs, cap, 10, clk + 1)
    tf: int = adap_total_freq(freqs, keys, cap)
    if tf >= 3:
        passed = passed + 1

    # Test 4: fill and evict
    clk = adap_access(keys, vals, times, freqs, cap, 20, 200, clk, 0, 0, stats)
    clk = adap_access(keys, vals, times, freqs, cap, 30, 300, clk, 0, 0, stats)
    clk = adap_access(keys, vals, times, freqs, cap, 40, 400, clk, 0, 0, stats)
    got40: int = adap_get(keys, vals, times, freqs, cap, 40, clk)
    if got40 == 400:
        passed = passed + 1

    # Test 5: re-access existing key triggers hit stat
    clk = adap_access(keys, vals, times, freqs, cap, 40, 400, clk, 0, 0, stats)
    s0: int = stats[0]
    s1: int = stats[1]
    total_stats: int = s0 + s1
    if total_stats > 0:
        passed = passed + 1

    return passed
