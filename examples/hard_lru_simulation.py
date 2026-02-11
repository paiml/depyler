"""LRU cache simulation with fixed-size array.

Tests: lru_get, lru_put, eviction, capacity handling.
"""


def lru_create(capacity: int) -> list[list[int]]:
    """Create LRU cache. Each entry is [key, value, timestamp]. Empty = [-1,-1,0]."""
    cache: list[list[int]] = []
    i: int = 0
    while i < capacity:
        cache.append([-1, -1, 0])
        i = i + 1
    return cache


def lru_find(cache: list[list[int]], key: int) -> int:
    """Find index of key in cache. Returns -1 if not found."""
    i: int = 0
    while i < len(cache):
        if cache[i][0] == key:
            return i
        i = i + 1
    return -1


def lru_get(cache: list[list[int]], key: int, time: int) -> list[int]:
    """Get value from cache. Returns [value, new_time] or [-1, time] if miss."""
    idx: int = lru_find(cache, key)
    if idx == -1:
        return [-1, time]
    cache[idx][2] = time
    return [cache[idx][1], time + 1]


def lru_put(cache: list[list[int]], key: int, value: int, time: int) -> int:
    """Put key-value into cache, evicting LRU if full. Returns new time."""
    idx: int = lru_find(cache, key)
    if idx != -1:
        cache[idx][1] = value
        cache[idx][2] = time
        return time + 1
    empty: int = lru_find(cache, -1)
    if empty != -1:
        cache[empty][0] = key
        cache[empty][1] = value
        cache[empty][2] = time
        return time + 1
    min_time: int = cache[0][2]
    min_idx: int = 0
    i: int = 1
    while i < len(cache):
        if cache[i][2] < min_time:
            min_time = cache[i][2]
            min_idx = i
        i = i + 1
    cache[min_idx][0] = key
    cache[min_idx][1] = value
    cache[min_idx][2] = time
    return time + 1


def lru_size(cache: list[list[int]]) -> int:
    """Count number of occupied entries."""
    count: int = 0
    i: int = 0
    while i < len(cache):
        if cache[i][0] != -1:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test LRU cache simulation."""
    ok: int = 0

    c: list[list[int]] = lru_create(2)
    if lru_size(c) == 0:
        ok = ok + 1

    t: int = lru_put(c, 1, 10, 0)
    t = lru_put(c, 2, 20, t)
    if lru_size(c) == 2:
        ok = ok + 1

    r: list[int] = lru_get(c, 1, t)
    if r[0] == 10:
        ok = ok + 1

    t = r[1]
    t = lru_put(c, 3, 30, t)
    r2: list[int] = lru_get(c, 2, t)
    if r2[0] == -1:
        ok = ok + 1

    r3: list[int] = lru_get(c, 3, t)
    if r3[0] == 30:
        ok = ok + 1

    r4: list[int] = lru_get(c, 1, t + 1)
    if r4[0] == 10:
        ok = ok + 1

    return ok
