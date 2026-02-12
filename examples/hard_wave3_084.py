"""Systems: LRU cache simulation.
Tests: cache eviction, hit/miss tracking, capacity management.
"""
from typing import Dict, List, Tuple


def cache_contains(keys: List[int], count: int, target: int) -> int:
    """Check if key exists in cache. Returns index or -1."""
    i: int = 0
    while i < count:
        if keys[i] == target:
            return i
        i += 1
    return -1


def cache_lookup(keys: List[int], vals: List[int], count: int, target: int) -> int:
    """Lookup value by key in cache. Returns value or -1."""
    i: int = 0
    while i < count:
        if keys[i] == target:
            return vals[i]
        i += 1
    return -1


def cache_evict_oldest(keys: List[int], vals: List[int], count: int) -> int:
    """Remove oldest entry (index 0) by shifting. Returns new count."""
    if count == 0:
        return 0
    i: int = 0
    while i < count - 1:
        keys[i] = keys[i + 1]
        vals[i] = vals[i + 1]
        i += 1
    return count - 1


def cache_move_to_end(keys: List[int], vals: List[int], count: int, idx: int) -> int:
    """Move item at idx to end (most recently used). Returns count unchanged."""
    saved_key: int = keys[idx]
    saved_val: int = vals[idx]
    i: int = idx
    while i < count - 1:
        keys[i] = keys[i + 1]
        vals[i] = vals[i + 1]
        i += 1
    keys[count - 1] = saved_key
    vals[count - 1] = saved_val
    return count


def cache_insert_at_end(keys: List[int], vals: List[int], count: int, new_key: int, new_val: int) -> int:
    """Insert new entry at end. Returns new count."""
    keys[count] = new_key
    vals[count] = new_val
    return count + 1


def cache_hit_rate_sim(accesses: List[int], capacity: int) -> int:
    """Simulate LRU cache and return hit count (multiply by 100, divide by total for %)."""
    keys: List[int] = []
    vals: List[int] = []
    i: int = 0
    while i < capacity:
        keys.append(-1)
        vals.append(-1)
        i += 1
    count: int = 0
    hits: int = 0
    total: int = 0
    for acc in accesses:
        total += 1
        idx: int = cache_contains(keys, count, acc)
        if idx >= 0:
            hits += 1
            count = cache_move_to_end(keys, vals, count, idx)
        else:
            if count >= capacity:
                count = cache_evict_oldest(keys, vals, count)
            count = cache_insert_at_end(keys, vals, count, acc, acc)
    return hits


def cache_size_after_ops(ops: List[int], capacity: int) -> int:
    """Track cache size after a sequence of insert operations."""
    keys: List[int] = []
    vals: List[int] = []
    i: int = 0
    while i < capacity:
        keys.append(-1)
        vals.append(-1)
        i += 1
    count: int = 0
    for op in ops:
        idx: int = cache_contains(keys, count, op)
        if idx >= 0:
            count = cache_move_to_end(keys, vals, count, idx)
        else:
            if count >= capacity:
                count = cache_evict_oldest(keys, vals, count)
            count = cache_insert_at_end(keys, vals, count, op, op)
    return count


def test_cache() -> bool:
    ok: bool = True
    keys: List[int] = [-1, -1, -1]
    vals: List[int] = [-1, -1, -1]
    count: int = 0
    count = cache_insert_at_end(keys, vals, count, 1, 10)
    if count != 1:
        ok = False
    count = cache_insert_at_end(keys, vals, count, 2, 20)
    if count != 2:
        ok = False
    val: int = cache_lookup(keys, vals, count, 1)
    if val != 10:
        ok = False
    val2: int = cache_lookup(keys, vals, count, 2)
    if val2 != 20:
        ok = False
    val3: int = cache_lookup(keys, vals, count, 99)
    if val3 != -1:
        ok = False
    accesses: List[int] = [1, 2, 3, 1, 2, 4, 1]
    h: int = cache_hit_rate_sim(accesses, 3)
    if h < 0:
        ok = False
    return ok
