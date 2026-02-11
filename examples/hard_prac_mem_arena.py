"""Arena allocator simulation.

Bump-pointer allocation within a fixed region. Very fast allocation
(just increment pointer). Free all at once (reset pointer).
"""


def arena_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def arena_alloc(ptr_arr: list[int], capacity: int, requested: int) -> int:
    """Allocate requested bytes. ptr_arr[0]=current position.
    Returns start offset or -1 if insufficient space."""
    pos: int = ptr_arr[0]
    new_pos: int = pos + requested
    if new_pos > capacity:
        return 0 - 1
    ptr_arr[0] = new_pos
    return pos


def arena_alloc_aligned(ptr_arr: list[int], capacity: int,
                        requested: int, alignment: int) -> int:
    """Allocate with alignment. Returns aligned start or -1."""
    pos: int = ptr_arr[0]
    remainder: int = pos % alignment
    if remainder != 0:
        pos = pos + (alignment - remainder)
    new_pos: int = pos + requested
    if new_pos > capacity:
        return 0 - 1
    ptr_arr[0] = new_pos
    return pos


def arena_used(ptr_arr: list[int]) -> int:
    """How many bytes used."""
    result: int = ptr_arr[0]
    return result


def arena_remaining(ptr_arr: list[int], capacity: int) -> int:
    """How many bytes remaining."""
    pos: int = ptr_arr[0]
    return capacity - pos


def arena_reset(ptr_arr: list[int]) -> int:
    """Reset arena (free everything). Returns 0."""
    ptr_arr[0] = 0
    return 0


def arena_usage_pct(ptr_arr: list[int], capacity: int) -> int:
    """Usage as percentage."""
    pos: int = ptr_arr[0]
    return (pos * 100) // capacity


def arena_mark(ptr_arr: list[int]) -> int:
    """Save current position for later rollback. Returns position."""
    result: int = ptr_arr[0]
    return result


def arena_rollback(ptr_arr: list[int], mark_pos: int) -> int:
    """Rollback to saved mark position. Returns bytes freed."""
    pos: int = ptr_arr[0]
    freed: int = pos - mark_pos
    ptr_arr[0] = mark_pos
    return freed


def test_module() -> int:
    """Test arena allocator."""
    passed: int = 0
    capacity: int = 1024
    ptr_arr: list[int] = [0]

    # Test 1: basic allocation
    off1: int = arena_alloc(ptr_arr, capacity, 100)
    if off1 == 0:
        passed = passed + 1

    # Test 2: second allocation starts after first
    off2: int = arena_alloc(ptr_arr, capacity, 200)
    if off2 == 100:
        passed = passed + 1

    # Test 3: remaining space
    rem: int = arena_remaining(ptr_arr, capacity)
    if rem == 724:
        passed = passed + 1

    # Test 4: mark and rollback
    mk: int = arena_mark(ptr_arr)
    arena_alloc(ptr_arr, capacity, 500)
    freed: int = arena_rollback(ptr_arr, mk)
    if freed == 500:
        passed = passed + 1

    # Test 5: reset frees everything
    arena_reset(ptr_arr)
    used: int = arena_used(ptr_arr)
    if used == 0:
        passed = passed + 1

    return passed
