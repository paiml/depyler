"""Memory pool allocator simulation.

Fixed-size pool with free list for O(1) allocation and deallocation.
No fragmentation, no per-object overhead.
"""


def pool_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def pool_create(free_list: list[int], capacity: int) -> int:
    """Create pool: build free list chain. free_list[i] = next free.
    Returns head index (0)."""
    i: int = 0
    while i < capacity - 1:
        free_list[i] = i + 1
        i = i + 1
    free_list[capacity - 1] = 0 - 1
    return 0


def pool_alloc(free_list: list[int], head_arr: list[int]) -> int:
    """Allocate from pool. Returns slot index or -1."""
    h: int = head_arr[0]
    if h < 0:
        return 0 - 1
    nxt: int = free_list[h]
    free_list[h] = 0 - 2
    head_arr[0] = nxt
    return h


def pool_dealloc(free_list: list[int], head_arr: list[int], slot: int) -> int:
    """Return slot to pool. Returns 1."""
    h: int = head_arr[0]
    free_list[slot] = h
    head_arr[0] = slot
    return 1


def pool_count_free(free_list: list[int], head_arr: list[int]) -> int:
    """Count free slots by traversing free list."""
    count: int = 0
    current: int = head_arr[0]
    while current >= 0:
        count = count + 1
        current = free_list[current]
    return count


def pool_count_alloc(free_list: list[int], capacity: int) -> int:
    """Count allocated slots (those with -2 marker)."""
    count: int = 0
    i: int = 0
    while i < capacity:
        f: int = free_list[i]
        if f == (0 - 2):
            count = count + 1
        i = i + 1
    return count


def pool_is_allocated(free_list: list[int], slot: int) -> int:
    """Check if slot is allocated. Returns 1 if yes."""
    f: int = free_list[slot]
    if f == (0 - 2):
        return 1
    return 0


def pool_reset(free_list: list[int], head_arr: list[int], capacity: int) -> int:
    """Reset pool to all free. Returns capacity."""
    pool_create(free_list, capacity)
    head_arr[0] = 0
    return capacity


def test_module() -> int:
    """Test memory pool."""
    passed: int = 0
    capacity: int = 8
    free_list: list[int] = pool_init_zeros(capacity)
    head_arr: list[int] = [0]
    pool_create(free_list, capacity)

    # Test 1: allocate from pool
    s1: int = pool_alloc(free_list, head_arr)
    if s1 == 0:
        passed = passed + 1

    # Test 2: is allocated
    is_alloc: int = pool_is_allocated(free_list, s1)
    if is_alloc == 1:
        passed = passed + 1

    # Test 3: allocate more and count
    pool_alloc(free_list, head_arr)
    pool_alloc(free_list, head_arr)
    alloc_cnt: int = pool_count_alloc(free_list, capacity)
    if alloc_cnt == 3:
        passed = passed + 1

    # Test 4: dealloc and verify
    pool_dealloc(free_list, head_arr, s1)
    free_cnt: int = pool_count_free(free_list, head_arr)
    if free_cnt == 6:
        passed = passed + 1

    # Test 5: reset pool
    pool_reset(free_list, head_arr, capacity)
    all_free: int = pool_count_free(free_list, head_arr)
    if all_free == 8:
        passed = passed + 1

    return passed
