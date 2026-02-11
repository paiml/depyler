"""Buddy allocator simulation.

Power-of-2 block sizes. When a block is too large, split into buddies.
When both buddies are free, merge them back.
"""


def buddy_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def buddy_init_neg(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def buddy_next_pow2(n: int) -> int:
    """Find next power of 2 >= n."""
    p: int = 1
    while p < n:
        p = p * 2
    return p


def buddy_log2(n: int) -> int:
    """Floor of log base 2."""
    result: int = 0
    val: int = n
    while val > 1:
        val = val // 2
        result = result + 1
    return result


def buddy_alloc(free_map: list[int], total_size: int, requested: int) -> int:
    """Allocate block of at least requested size. Returns offset or -1.
    free_map[i]: 0=free, 1=allocated, 2=split-parent."""
    block_size: int = buddy_next_pow2(requested)
    if block_size > total_size:
        return 0 - 1
    offset: int = 0
    while offset < total_size:
        fm: int = free_map[offset]
        if fm == 0:
            avail: int = total_size - offset
            actual: int = buddy_next_pow2(avail)
            if actual < avail:
                actual = avail
            while actual > block_size:
                actual = actual // 2
                mid: int = offset + actual
                if mid < total_size:
                    free_map[mid] = 0
            if actual >= block_size:
                free_map[offset] = 1
                return offset
        offset = offset + block_size
    return 0 - 1


def buddy_free(free_map: list[int], offset: int) -> int:
    """Free block at offset. Returns 1."""
    free_map[offset] = 0
    return 1


def buddy_count_free(free_map: list[int], total_size: int) -> int:
    """Count free entries."""
    count: int = 0
    i: int = 0
    while i < total_size:
        fm: int = free_map[i]
        if fm == 0:
            count = count + 1
        i = i + 1
    return count


def buddy_count_alloc(free_map: list[int], total_size: int) -> int:
    """Count allocated entries."""
    count: int = 0
    i: int = 0
    while i < total_size:
        fm: int = free_map[i]
        if fm == 1:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test buddy allocator."""
    passed: int = 0
    total: int = 16
    free_map: list[int] = buddy_init_zeros(total)

    # Test 1: next power of 2
    p: int = buddy_next_pow2(5)
    if p == 8:
        passed = passed + 1

    # Test 2: log2
    l: int = buddy_log2(8)
    if l == 3:
        passed = passed + 1

    # Test 3: allocate
    off1: int = buddy_alloc(free_map, total, 4)
    if off1 >= 0:
        passed = passed + 1

    # Test 4: second allocation
    off2: int = buddy_alloc(free_map, total, 4)
    if off2 >= 0:
        if off2 != off1:
            passed = passed + 1

    # Test 5: free and recount
    buddy_free(free_map, off1)
    free_cnt: int = buddy_count_free(free_map, total)
    if free_cnt > 0:
        passed = passed + 1

    return passed
