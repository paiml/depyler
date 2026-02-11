"""Random eviction cache strategy simulation.

Uses a deterministic pseudo-random number generator (LCG) for
reproducible random eviction decisions.
"""


def rand_lcg(seed: int) -> int:
    """Linear congruential generator. Returns next pseudo-random value."""
    result: int = (seed * 1103515245 + 12345) % 2147483648
    return result


def rand_init_slots(capacity: int) -> list[int]:
    """Initialize slots with -1 sentinels."""
    slots: list[int] = []
    i: int = 0
    while i < capacity:
        slots.append(0 - 1)
        i = i + 1
    return slots


def rand_find(keys: list[int], target: int, capacity: int) -> int:
    """Find target in keys. Returns index or -1."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == target:
            return i
        i = i + 1
    return 0 - 1


def rand_count(keys: list[int], capacity: int) -> int:
    """Count occupied slots."""
    count: int = 0
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem != (0 - 1):
            count = count + 1
        i = i + 1
    return count


def rand_first_empty(keys: list[int], capacity: int) -> int:
    """Find first empty slot. Returns -1 if full."""
    i: int = 0
    while i < capacity:
        elem: int = keys[i]
        if elem == (0 - 1):
            return i
        i = i + 1
    return 0 - 1


def rand_put(keys: list[int], vals: list[int], capacity: int,
             k: int, v: int, seed: int) -> int:
    """Insert k->v with random eviction if full. Returns new seed."""
    idx: int = rand_find(keys, k, capacity)
    if idx >= 0:
        vals[idx] = v
        return seed
    empty: int = rand_first_empty(keys, capacity)
    if empty >= 0:
        keys[empty] = k
        vals[empty] = v
        return seed
    new_seed: int = rand_lcg(seed)
    victim: int = new_seed % capacity
    if victim < 0:
        victim = 0 - victim
    keys[victim] = k
    vals[victim] = v
    return new_seed


def rand_get(keys: list[int], vals: list[int], capacity: int, k: int) -> int:
    """Get value for key. Returns -1 on miss."""
    idx: int = rand_find(keys, k, capacity)
    if idx < 0:
        return 0 - 1
    result: int = vals[idx]
    return result


def test_module() -> int:
    """Test random eviction cache."""
    passed: int = 0
    cap: int = 3
    keys: list[int] = rand_init_slots(cap)
    vals: list[int] = rand_init_slots(cap)
    seed: int = 42

    # Test 1: insert and retrieve
    seed = rand_put(keys, vals, cap, 10, 100, seed)
    got: int = rand_get(keys, vals, cap, 10)
    if got == 100:
        passed = passed + 1

    # Test 2: LCG produces non-zero values
    r1: int = rand_lcg(42)
    if r1 > 0:
        passed = passed + 1

    # Test 3: fill cache without eviction
    seed = rand_put(keys, vals, cap, 20, 200, seed)
    seed = rand_put(keys, vals, cap, 30, 300, seed)
    cnt: int = rand_count(keys, cap)
    if cnt == 3:
        passed = passed + 1

    # Test 4: overflow triggers eviction (some key evicted)
    seed = rand_put(keys, vals, cap, 40, 400, seed)
    cnt2: int = rand_count(keys, cap)
    if cnt2 == 3:
        passed = passed + 1

    # Test 5: new key is present
    got40: int = rand_get(keys, vals, cap, 40)
    if got40 == 400:
        passed = passed + 1

    return passed
