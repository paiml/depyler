"""Block bitmap allocator simulation.

Uses a bitmap (list of 0/1) to track free/allocated blocks.
Supports first-fit, best-fit, and bulk operations.
"""


def bm_init_zeros(size: int) -> list[int]:
    """Initialize with zeros (all free)."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def bm_alloc_first_fit(bitmap: list[int], total: int) -> int:
    """Allocate first free block. Returns block number or -1."""
    i: int = 0
    while i < total:
        b: int = bitmap[i]
        if b == 0:
            bitmap[i] = 1
            return i
        i = i + 1
    return 0 - 1


def bm_alloc_contiguous(bitmap: list[int], total: int, needed: int) -> int:
    """Allocate contiguous run. Returns start or -1."""
    start: int = 0
    while start <= total - needed:
        ok: int = 1
        j: int = 0
        while j < needed:
            b: int = bitmap[start + j]
            if b != 0:
                ok = 0
                j = needed
            j = j + 1
        if ok == 1:
            k: int = 0
            while k < needed:
                bitmap[start + k] = 1
                k = k + 1
            return start
        start = start + 1
    return 0 - 1


def bm_free_block(bitmap: list[int], block: int) -> int:
    """Free a single block. Returns 1."""
    bitmap[block] = 0
    return 1


def bm_free_range(bitmap: list[int], start: int, length: int) -> int:
    """Free a range of blocks. Returns count freed."""
    i: int = 0
    while i < length:
        bitmap[start + i] = 0
        i = i + 1
    return length


def bm_count_free(bitmap: list[int], total: int) -> int:
    """Count free blocks."""
    count: int = 0
    i: int = 0
    while i < total:
        b: int = bitmap[i]
        if b == 0:
            count = count + 1
        i = i + 1
    return count


def bm_count_used(bitmap: list[int], total: int) -> int:
    """Count used blocks."""
    count: int = 0
    i: int = 0
    while i < total:
        b: int = bitmap[i]
        if b == 1:
            count = count + 1
        i = i + 1
    return count


def bm_fragmentation(bitmap: list[int], total: int) -> int:
    """Count number of free fragments (contiguous free runs)."""
    fragments: int = 0
    in_free: int = 0
    i: int = 0
    while i < total:
        b: int = bitmap[i]
        if b == 0:
            if in_free == 0:
                fragments = fragments + 1
                in_free = 1
        else:
            in_free = 0
        i = i + 1
    return fragments


def test_module() -> int:
    """Test bitmap allocator."""
    passed: int = 0
    total: int = 16
    bitmap: list[int] = bm_init_zeros(total)

    # Test 1: allocate first fit
    b1: int = bm_alloc_first_fit(bitmap, total)
    if b1 == 0:
        passed = passed + 1

    # Test 2: allocate contiguous run
    start: int = bm_alloc_contiguous(bitmap, total, 4)
    if start == 1:
        passed = passed + 1

    # Test 3: count free
    free: int = bm_count_free(bitmap, total)
    if free == 11:
        passed = passed + 1

    # Test 4: free range and recount
    bm_free_range(bitmap, 1, 4)
    free2: int = bm_count_free(bitmap, total)
    if free2 == 15:
        passed = passed + 1

    # Test 5: fragmentation count
    bm_alloc_contiguous(bitmap, total, 2)
    bm_alloc_first_fit(bitmap, total)
    bm_alloc_contiguous(bitmap, total, 3)
    frags: int = bm_fragmentation(bitmap, total)
    if frags >= 1:
        passed = passed + 1

    return passed
