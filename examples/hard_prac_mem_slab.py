"""Slab allocator simulation.

Pre-allocates fixed-size object caches. Each slab holds objects
of the same size. Fast allocation with no fragmentation.
"""


def slab_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def slab_create(free_bitmap: list[int], obj_size: int, slab_capacity: int) -> int:
    """Create a slab. All slots free (0). Returns slab capacity."""
    i: int = 0
    while i < slab_capacity:
        free_bitmap[i] = 0
        i = i + 1
    return slab_capacity


def slab_alloc(free_bitmap: list[int], slab_capacity: int) -> int:
    """Allocate one object from slab. Returns slot index or -1."""
    i: int = 0
    while i < slab_capacity:
        f: int = free_bitmap[i]
        if f == 0:
            free_bitmap[i] = 1
            return i
        i = i + 1
    return 0 - 1


def slab_free_slot(free_bitmap: list[int], slot: int) -> int:
    """Free an object slot. Returns 1."""
    free_bitmap[slot] = 0
    return 1


def slab_count_free(free_bitmap: list[int], slab_capacity: int) -> int:
    """Count free slots."""
    count: int = 0
    i: int = 0
    while i < slab_capacity:
        f: int = free_bitmap[i]
        if f == 0:
            count = count + 1
        i = i + 1
    return count


def slab_count_used(free_bitmap: list[int], slab_capacity: int) -> int:
    """Count used slots."""
    count: int = 0
    i: int = 0
    while i < slab_capacity:
        f: int = free_bitmap[i]
        if f == 1:
            count = count + 1
        i = i + 1
    return count


def slab_is_full(free_bitmap: list[int], slab_capacity: int) -> int:
    """Check if slab is full. Returns 1 if full."""
    i: int = 0
    while i < slab_capacity:
        f: int = free_bitmap[i]
        if f == 0:
            return 0
        i = i + 1
    return 1


def slab_alloc_n(free_bitmap: list[int], slab_capacity: int,
                 n: int, result: list[int]) -> int:
    """Allocate n objects. Store slots in result. Returns count allocated."""
    allocated: int = 0
    while allocated < n:
        slot: int = slab_alloc(free_bitmap, slab_capacity)
        if slot < 0:
            return allocated
        result[allocated] = slot
        allocated = allocated + 1
    return allocated


def test_module() -> int:
    """Test slab allocator."""
    passed: int = 0
    capacity: int = 8
    bitmap: list[int] = slab_init_zeros(capacity)
    slab_create(bitmap, 64, capacity)

    # Test 1: allocate object
    s1: int = slab_alloc(bitmap, capacity)
    if s1 == 0:
        passed = passed + 1

    # Test 2: allocate multiple
    slots: list[int] = slab_init_zeros(4)
    got: int = slab_alloc_n(bitmap, capacity, 4, slots)
    if got == 4:
        passed = passed + 1

    # Test 3: count used
    used: int = slab_count_used(bitmap, capacity)
    if used == 5:
        passed = passed + 1

    # Test 4: free and recount
    slab_free_slot(bitmap, s1)
    free_cnt: int = slab_count_free(bitmap, capacity)
    if free_cnt == 4:
        passed = passed + 1

    # Test 5: fill slab and check full
    slab_alloc(bitmap, capacity)
    slab_alloc(bitmap, capacity)
    slab_alloc(bitmap, capacity)
    slab_alloc(bitmap, capacity)
    full: int = slab_is_full(bitmap, capacity)
    if full == 1:
        passed = passed + 1

    return passed
