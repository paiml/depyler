"""Simple hash table simulation with chaining using parallel lists."""


def hash_func(key: int, capacity: int) -> int:
    """Simple modulo hash function."""
    result: int = key % capacity
    if result < 0:
        result = result + capacity
    return result


def ht_create(capacity: int) -> list[int]:
    """Create hash table metadata. Returns list of bucket sizes."""
    sizes: list[int] = []
    i: int = 0
    while i < capacity:
        sizes.append(0)
        i = i + 1
    return sizes


def ht_insert(keys: list[int], vals: list[int], bucket_ids: list[int], sizes: list[int], capacity: int, key: int, val: int) -> int:
    """Insert key-val pair. Returns 1 if new, 0 if updated."""
    h: int = hash_func(key, capacity)
    i: int = 0
    n: int = len(keys)
    while i < n:
        if bucket_ids[i] == h and keys[i] == key:
            vals[i] = val
            return 0
        i = i + 1
    keys.append(key)
    vals.append(val)
    bucket_ids.append(h)
    sizes[h] = sizes[h] + 1
    return 1


def ht_get(keys: list[int], vals: list[int], bucket_ids: list[int], capacity: int, key: int) -> int:
    """Get value for key. Returns -1 if not found."""
    h: int = hash_func(key, capacity)
    i: int = 0
    n: int = len(keys)
    while i < n:
        if bucket_ids[i] == h and keys[i] == key:
            return vals[i]
        i = i + 1
    return 0 - 1


def ht_contains(keys: list[int], bucket_ids: list[int], capacity: int, key: int) -> int:
    """Check if key exists. Returns 1 if found."""
    h: int = hash_func(key, capacity)
    i: int = 0
    n: int = len(keys)
    while i < n:
        if bucket_ids[i] == h and keys[i] == key:
            return 1
        i = i + 1
    return 0


def ht_size(keys: list[int]) -> int:
    """Return number of entries."""
    return len(keys)


def test_module() -> int:
    """Test hash table simulation."""
    passed: int = 0
    cap: int = 8
    sizes: list[int] = ht_create(cap)
    ks: list[int] = []
    vs: list[int] = []
    bs: list[int] = []

    ht_insert(ks, vs, bs, sizes, cap, 10, 100)
    ht_insert(ks, vs, bs, sizes, cap, 20, 200)
    ht_insert(ks, vs, bs, sizes, cap, 30, 300)

    if ht_get(ks, vs, bs, cap, 10) == 100:
        passed = passed + 1

    if ht_get(ks, vs, bs, cap, 20) == 200:
        passed = passed + 1

    if ht_contains(ks, bs, cap, 30) == 1:
        passed = passed + 1

    if ht_contains(ks, bs, cap, 99) == 0:
        passed = passed + 1

    if ht_size(ks) == 3:
        passed = passed + 1

    ht_insert(ks, vs, bs, sizes, cap, 10, 999)
    if ht_get(ks, vs, bs, cap, 10) == 999:
        passed = passed + 1

    return passed
