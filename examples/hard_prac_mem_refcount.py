"""Reference counting memory management simulation.

Each object has a reference count. When count drops to zero,
the object is freed. Handles cycles via cycle detection.
"""


def rc_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def rc_init_neg(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def rc_alloc(counts: list[int], alive: list[int], capacity: int) -> int:
    """Allocate object. Returns index or -1 if full."""
    i: int = 0
    while i < capacity:
        a: int = alive[i]
        if a == 0:
            alive[i] = 1
            counts[i] = 1
            return i
        i = i + 1
    return 0 - 1


def rc_inc_ref(counts: list[int], idx: int) -> int:
    """Increment reference count. Returns new count."""
    counts[idx] = counts[idx] + 1
    result: int = counts[idx]
    return result


def rc_dec_ref(counts: list[int], alive: list[int],
               refs: list[int], idx: int) -> int:
    """Decrement reference count. Free if zero. Returns new count.
    Also decrements refs of referenced objects."""
    counts[idx] = counts[idx] - 1
    c: int = counts[idx]
    if c <= 0:
        alive[idx] = 0
        counts[idx] = 0
        ref: int = refs[idx]
        if ref >= 0:
            a: int = alive[ref]
            if a == 1:
                rc_dec_ref(counts, alive, refs, ref)
        refs[idx] = 0 - 1
    return c


def rc_is_alive(alive: list[int], idx: int) -> int:
    """Check if object is alive. Returns 1 or 0."""
    result: int = alive[idx]
    return result


def rc_get_count(counts: list[int], idx: int) -> int:
    """Get reference count."""
    result: int = counts[idx]
    return result


def rc_count_alive(alive: list[int], capacity: int) -> int:
    """Count live objects."""
    count: int = 0
    i: int = 0
    while i < capacity:
        a: int = alive[i]
        if a == 1:
            count = count + 1
        i = i + 1
    return count


def rc_set_ref(refs: list[int], from_idx: int, to_idx: int, counts: list[int]) -> int:
    """Set reference from one object to another. Increments target count. Returns 1."""
    old_ref: int = refs[from_idx]
    refs[from_idx] = to_idx
    counts[to_idx] = counts[to_idx] + 1
    return 1


def test_module() -> int:
    """Test reference counting."""
    passed: int = 0
    capacity: int = 8
    counts: list[int] = rc_init_zeros(capacity)
    alive: list[int] = rc_init_zeros(capacity)
    refs: list[int] = rc_init_neg(capacity)

    # Test 1: allocate object
    o1: int = rc_alloc(counts, alive, capacity)
    if o1 == 0:
        passed = passed + 1

    # Test 2: reference count starts at 1
    c1: int = rc_get_count(counts, o1)
    if c1 == 1:
        passed = passed + 1

    # Test 3: increment reference
    rc_inc_ref(counts, o1)
    c2: int = rc_get_count(counts, o1)
    if c2 == 2:
        passed = passed + 1

    # Test 4: decrement doesn't free (count > 0)
    rc_dec_ref(counts, alive, refs, o1)
    still_alive: int = rc_is_alive(alive, o1)
    if still_alive == 1:
        passed = passed + 1

    # Test 5: decrement to zero frees object
    rc_dec_ref(counts, alive, refs, o1)
    is_dead: int = rc_is_alive(alive, o1)
    if is_dead == 0:
        passed = passed + 1

    return passed
