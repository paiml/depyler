"""LRU-like cache simulation using dict and list.

Tests: cache put/get, cache eviction, hit rate computation,
most recently used tracking, and cache warmup.
"""


def cache_put(keys: list[int], vals: list[int], order: list[int],
              capacity: int, key: int, val: int) -> int:
    """Add key-val to cache. Evict LRU if full. Returns evicted key or -1."""
    idx: int = 0
    found: bool = False
    while idx < len(keys):
        if keys[idx] == key:
            found = True
            vals[idx] = val
            pos: int = 0
            while pos < len(order):
                if order[pos] == key:
                    order_new: list[int] = order[0:pos] + order[pos + 1:]
                    order_new.append(key)
                    i: int = 0
                    while i < len(order):
                        order[i] = order_new[i]
                        i = i + 1
                    if len(order_new) > len(order):
                        order.append(order_new[len(order_new) - 1])
                    break
                pos = pos + 1
            return -1
        idx = idx + 1
    evicted: int = -1
    if len(keys) >= capacity:
        evict_key: int = order[0]
        order_new2: list[int] = order[1:]
        ki: int = 0
        while ki < len(keys):
            if keys[ki] == evict_key:
                keys_new: list[int] = keys[0:ki] + keys[ki + 1:]
                vals_new: list[int] = vals[0:ki] + vals[ki + 1:]
                evicted = evict_key
                while len(keys) > len(keys_new):
                    keys.pop()
                    vals.pop()
                ki2: int = 0
                while ki2 < len(keys_new):
                    if ki2 < len(keys):
                        keys[ki2] = keys_new[ki2]
                        vals[ki2] = vals_new[ki2]
                    else:
                        keys.append(keys_new[ki2])
                        vals.append(vals_new[ki2])
                    ki2 = ki2 + 1
                break
            ki = ki + 1
        while len(order) > len(order_new2):
            order.pop()
        oi: int = 0
        while oi < len(order_new2):
            if oi < len(order):
                order[oi] = order_new2[oi]
            else:
                order.append(order_new2[oi])
            oi = oi + 1
    keys.append(key)
    vals.append(val)
    order.append(key)
    return evicted


def cache_get(keys: list[int], vals: list[int], order: list[int], key: int) -> int:
    """Get value from cache. Returns -1 if not found. Updates access order."""
    idx: int = 0
    while idx < len(keys):
        if keys[idx] == key:
            pos: int = 0
            while pos < len(order):
                if order[pos] == key:
                    new_order: list[int] = order[0:pos] + order[pos + 1:]
                    new_order.append(key)
                    i: int = 0
                    while i < len(new_order):
                        if i < len(order):
                            order[i] = new_order[i]
                        i = i + 1
                    break
                pos = pos + 1
            return vals[idx]
        idx = idx + 1
    return -1


def compute_hit_rate(accesses: list[int], capacity: int) -> int:
    """Simulate cache with given access pattern. Returns hit percentage (0-100)."""
    keys: list[int] = []
    vals: list[int] = []
    order: list[int] = []
    hits: int = 0
    total: int = len(accesses)
    i: int = 0
    while i < total:
        key: int = accesses[i]
        found: bool = False
        j: int = 0
        while j < len(keys):
            if keys[j] == key:
                found = True
                break
            j = j + 1
        if found:
            hits = hits + 1
            cache_get(keys, vals, order, key)
        else:
            cache_put(keys, vals, order, capacity, key, key)
        i = i + 1
    if total == 0:
        return 0
    return (hits * 100) // total


def test_module() -> bool:
    """Test cache simulation functions."""
    ok: bool = True

    keys: list[int] = []
    vals: list[int] = []
    order: list[int] = []
    ev1: int = cache_put(keys, vals, order, 2, 1, 10)
    if ev1 != -1:
        ok = False
    ev2: int = cache_put(keys, vals, order, 2, 2, 20)
    if ev2 != -1:
        ok = False

    v1: int = cache_get(keys, vals, order, 1)
    if v1 != 10:
        ok = False

    ev3: int = cache_put(keys, vals, order, 2, 3, 30)
    if ev3 != 2:
        ok = False

    v2: int = cache_get(keys, vals, order, 2)
    if v2 != -1:
        ok = False

    rate: int = compute_hit_rate([1, 2, 1, 3, 1, 2, 1], 2)
    if rate < 28:
        ok = False

    return ok
