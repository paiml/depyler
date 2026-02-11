"""Extent-based allocation simulation.

Allocates contiguous blocks (extents) for files, preferring
contiguous allocation for better sequential read performance.
"""


def ext_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def ext_init_neg(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def ext_find_free_run(bitmap: list[int], total_blocks: int, needed: int) -> int:
    """Find first contiguous run of 'needed' free blocks. Returns start or -1."""
    start: int = 0
    while start <= total_blocks - needed:
        found: int = 1
        j: int = 0
        while j < needed:
            b: int = bitmap[start + j]
            if b != 0:
                found = 0
                j = needed
            j = j + 1
        if found == 1:
            return start
        start = start + 1
    return 0 - 1


def ext_allocate(bitmap: list[int], total_blocks: int,
                 ext_starts: list[int], ext_lengths: list[int],
                 ext_count: list[int], needed: int) -> int:
    """Allocate an extent of 'needed' blocks. Returns start block or -1."""
    start: int = ext_find_free_run(bitmap, total_blocks, needed)
    if start < 0:
        return 0 - 1
    j: int = 0
    while j < needed:
        bitmap[start + j] = 1
        j = j + 1
    idx: int = ext_count[0]
    ext_starts[idx] = start
    ext_lengths[idx] = needed
    ext_count[0] = idx + 1
    return start


def ext_deallocate(bitmap: list[int], ext_starts: list[int],
                   ext_lengths: list[int], ext_idx: int) -> int:
    """Free an extent. Returns number of blocks freed."""
    start: int = ext_starts[ext_idx]
    length: int = ext_lengths[ext_idx]
    j: int = 0
    while j < length:
        bitmap[start + j] = 0
        j = j + 1
    ext_starts[ext_idx] = 0 - 1
    ext_lengths[ext_idx] = 0
    return length


def ext_count_free(bitmap: list[int], total_blocks: int) -> int:
    """Count free blocks."""
    count: int = 0
    i: int = 0
    while i < total_blocks:
        b: int = bitmap[i]
        if b == 0:
            count = count + 1
        i = i + 1
    return count


def ext_largest_free_run(bitmap: list[int], total_blocks: int) -> int:
    """Find largest contiguous free run."""
    best: int = 0
    current: int = 0
    i: int = 0
    while i < total_blocks:
        b: int = bitmap[i]
        if b == 0:
            current = current + 1
            if current > best:
                best = current
        else:
            current = 0
        i = i + 1
    return best


def test_module() -> int:
    """Test extent allocation."""
    passed: int = 0
    total: int = 20
    bitmap: list[int] = ext_init_zeros(total)
    cap: int = 10
    starts: list[int] = ext_init_neg(cap)
    lengths: list[int] = ext_init_zeros(cap)
    cnt: list[int] = [0]

    # Test 1: allocate extent
    s1: int = ext_allocate(bitmap, total, starts, lengths, cnt, 5)
    if s1 == 0:
        passed = passed + 1

    # Test 2: second allocation starts after first
    s2: int = ext_allocate(bitmap, total, starts, lengths, cnt, 3)
    if s2 == 5:
        passed = passed + 1

    # Test 3: free blocks count
    free: int = ext_count_free(bitmap, total)
    if free == 12:
        passed = passed + 1

    # Test 4: deallocate and verify
    freed: int = ext_deallocate(bitmap, starts, lengths, 0)
    if freed == 5:
        passed = passed + 1

    # Test 5: largest free run after dealloc
    largest: int = ext_largest_free_run(bitmap, total)
    if largest == 12:
        passed = passed + 1

    return passed
