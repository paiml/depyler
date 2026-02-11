"""TTL (Time-To-Live) cache simulation.

Each entry has an expiration time. Expired entries are treated as
misses and can be overwritten.
"""


def ttl_init(capacity: int) -> list[int]:
    """Initialize list with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def ttl_init_expiry(capacity: int) -> list[int]:
    """Initialize expiry times to 0."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def ttl_find_valid(keys: list[int], expiry: list[int],
                   target: int, capacity: int, now: int) -> int:
    """Find non-expired entry for target. Returns index or -1."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        exp: int = expiry[i]
        if elem == target:
            if exp > now:
                return i
        i = i + 1
    return 0 - 1


def ttl_find_slot(keys: list[int], expiry: list[int],
                  capacity: int, now: int) -> int:
    """Find an empty or expired slot. Returns index or -1."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == (0 - 1):
            return i
        exp: int = expiry[i]
        if exp <= now:
            return i
        i = i + 1
    return 0 - 1


def ttl_put(keys: list[int], vals: list[int], expiry: list[int],
            capacity: int, k: int, v: int, now: int, ttl_dur: int) -> int:
    """Insert with TTL. Returns 1 on success, 0 if no slot available."""
    idx: int = ttl_find_valid(keys, expiry, k, capacity, now)
    if idx >= 0:
        vals[idx] = v
        expiry[idx] = now + ttl_dur
        return 1
    slot: int = ttl_find_slot(keys, expiry, capacity, now)
    if slot < 0:
        return 0
    keys[slot] = k
    vals[slot] = v
    expiry[slot] = now + ttl_dur
    return 1


def ttl_get(keys: list[int], vals: list[int], expiry: list[int],
            capacity: int, k: int, now: int) -> int:
    """Get value if key exists and not expired. Returns -1 otherwise."""
    idx: int = ttl_find_valid(keys, expiry, k, capacity, now)
    if idx < 0:
        return 0 - 1
    result: int = vals[idx]
    return result


def ttl_purge_expired(keys: list[int], vals: list[int], expiry: list[int],
                      capacity: int, now: int) -> int:
    """Remove all expired entries. Returns count of purged entries."""
    purged: int = 0
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem != (0 - 1):
            exp: int = expiry[i]
            if exp <= now:
                keys[i] = 0 - 1
                vals[i] = 0
                expiry[i] = 0
                purged = purged + 1
        i = i + 1
    return purged


def test_module() -> int:
    """Test TTL cache operations."""
    passed: int = 0
    cap: int = 4
    keys: list[int] = ttl_init(cap)
    vals: list[int] = ttl_init(cap)
    expiry: list[int] = ttl_init_expiry(cap)

    # Test 1: insert and get before expiry
    ttl_put(keys, vals, expiry, cap, 10, 100, 0, 10)
    got: int = ttl_get(keys, vals, expiry, cap, 10, 5)
    if got == 100:
        passed = passed + 1

    # Test 2: expired entry returns miss
    expired: int = ttl_get(keys, vals, expiry, cap, 10, 15)
    if expired == (0 - 1):
        passed = passed + 1

    # Test 3: expired slot is reusable
    ttl_put(keys, vals, expiry, cap, 20, 200, 15, 10)
    ttl_put(keys, vals, expiry, cap, 30, 300, 15, 10)
    ttl_put(keys, vals, expiry, cap, 40, 400, 15, 10)
    ttl_put(keys, vals, expiry, cap, 50, 500, 15, 10)
    got50: int = ttl_get(keys, vals, expiry, cap, 50, 16)
    if got50 == 500:
        passed = passed + 1

    # Test 4: purge expired entries
    purged: int = ttl_purge_expired(keys, vals, expiry, cap, 30)
    if purged == 4:
        passed = passed + 1

    # Test 5: insert after purge works
    ok: int = ttl_put(keys, vals, expiry, cap, 60, 600, 30, 20)
    if ok == 1:
        passed = passed + 1

    return passed
