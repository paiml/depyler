"""TLB (Translation Lookaside Buffer) simulation.

Caches recent virtual-to-physical page translations.
Tracks hit/miss rates for performance analysis.
"""


def tlb_init_neg(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def tlb_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def tlb_find(vpages: list[int], target: int, capacity: int) -> int:
    """Find virtual page in TLB. Returns index or -1."""
    i: int = 0
    while i < capacity:
        vp: int = vpages[i]
        if vp == target:
            return i
        i = i + 1
    return 0 - 1


def tlb_insert(vpages: list[int], frames: list[int], access_times: list[int],
               capacity: int, vpage: int, frame: int, clock: int) -> int:
    """Insert mapping into TLB. LRU eviction if full. Returns 1."""
    idx: int = tlb_find(vpages, vpage, capacity)
    if idx >= 0:
        frames[idx] = frame
        access_times[idx] = clock
        return 1
    empty: int = 0 - 1
    i: int = 0
    while i < capacity:
        vp: int = vpages[i]
        if vp == (0 - 1):
            empty = i
            i = capacity
        i = i + 1
    if empty >= 0:
        vpages[empty] = vpage
        frames[empty] = frame
        access_times[empty] = clock
        return 1
    min_time: int = access_times[0]
    min_idx: int = 0
    j: int = 1
    while j < capacity:
        t: int = access_times[j]
        if t < min_time:
            min_time = t
            min_idx = j
        j = j + 1
    vpages[min_idx] = vpage
    frames[min_idx] = frame
    access_times[min_idx] = clock
    return 1


def tlb_lookup(vpages: list[int], frames: list[int], access_times: list[int],
               capacity: int, vpage: int, clock: int, stats: list[int]) -> int:
    """Lookup virtual page. stats[0]=hits, stats[1]=misses. Returns frame or -1."""
    idx: int = tlb_find(vpages, vpage, capacity)
    if idx >= 0:
        access_times[idx] = clock
        stats[0] = stats[0] + 1
        result: int = frames[idx]
        return result
    stats[1] = stats[1] + 1
    return 0 - 1


def tlb_flush(vpages: list[int], frames: list[int], capacity: int) -> int:
    """Flush entire TLB. Returns count flushed."""
    flushed: int = 0
    i: int = 0
    while i < capacity:
        vp: int = vpages[i]
        if vp != (0 - 1):
            vpages[i] = 0 - 1
            frames[i] = 0 - 1
            flushed = flushed + 1
        i = i + 1
    return flushed


def tlb_hit_rate_pct(stats: list[int]) -> int:
    """Hit rate percentage (0-100)."""
    hits: int = stats[0]
    misses: int = stats[1]
    total: int = hits + misses
    if total == 0:
        return 0
    return (hits * 100) // total


def test_module() -> int:
    """Test TLB simulation."""
    passed: int = 0
    capacity: int = 4
    vpages: list[int] = tlb_init_neg(capacity)
    frames_arr: list[int] = tlb_init_neg(capacity)
    atimes: list[int] = tlb_init_zeros(capacity)
    stats: list[int] = [0, 0]

    # Test 1: insert and lookup (hit)
    tlb_insert(vpages, frames_arr, atimes, capacity, 100, 5, 1)
    frame: int = tlb_lookup(vpages, frames_arr, atimes, capacity, 100, 2, stats)
    if frame == 5:
        passed = passed + 1

    # Test 2: miss on unknown page
    miss: int = tlb_lookup(vpages, frames_arr, atimes, capacity, 999, 3, stats)
    if miss == (0 - 1):
        passed = passed + 1

    # Test 3: hit rate after 1 hit, 1 miss
    rate: int = tlb_hit_rate_pct(stats)
    if rate == 50:
        passed = passed + 1

    # Test 4: fill and verify eviction
    tlb_insert(vpages, frames_arr, atimes, capacity, 200, 10, 4)
    tlb_insert(vpages, frames_arr, atimes, capacity, 300, 15, 5)
    tlb_insert(vpages, frames_arr, atimes, capacity, 400, 20, 6)
    tlb_insert(vpages, frames_arr, atimes, capacity, 500, 25, 7)
    evicted: int = tlb_find(vpages, 100, capacity)
    if evicted < 0:
        passed = passed + 1

    # Test 5: flush
    flushed: int = tlb_flush(vpages, frames_arr, capacity)
    if flushed == 4:
        passed = passed + 1

    return passed
