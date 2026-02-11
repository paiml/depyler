"""Free list block allocator simulation.

Maintains a linked free list of disk blocks. Allocated blocks
are removed from the list; freed blocks are prepended.
"""


def fl_init_free(total_blocks: int) -> list[int]:
    """Initialize free list: each block points to next. Last points to -1."""
    nxt: list[int] = []
    i: int = 0
    while i < total_blocks:
        nxt.append(i + 1)
        i = i + 1
    nxt[total_blocks - 1] = 0 - 1
    return nxt


def fl_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def fl_alloc(nxt: list[int], head_arr: list[int]) -> int:
    """Allocate one block from free list. Returns block number or -1."""
    h: int = head_arr[0]
    if h < 0:
        return 0 - 1
    next_head: int = nxt[h]
    nxt[h] = 0 - 1
    head_arr[0] = next_head
    return h


def fl_free_block(nxt: list[int], head_arr: list[int], block: int) -> int:
    """Return block to free list (prepend). Returns 1."""
    h: int = head_arr[0]
    nxt[block] = h
    head_arr[0] = block
    return 1


def fl_count_free(nxt: list[int], head_arr: list[int]) -> int:
    """Count blocks in free list by traversal."""
    count: int = 0
    current: int = head_arr[0]
    while current >= 0:
        count = count + 1
        current = nxt[current]
    return count


def fl_alloc_n(nxt: list[int], head_arr: list[int], n: int,
               result: list[int]) -> int:
    """Allocate n blocks. Store in result. Returns count allocated."""
    allocated: int = 0
    while allocated < n:
        block: int = fl_alloc(nxt, head_arr)
        if block < 0:
            return allocated
        result[allocated] = block
        allocated = allocated + 1
    return allocated


def fl_free_n(nxt: list[int], head_arr: list[int],
              blocks: list[int], n: int) -> int:
    """Free n blocks. Returns count freed."""
    freed: int = 0
    while freed < n:
        b: int = blocks[freed]
        fl_free_block(nxt, head_arr, b)
        freed = freed + 1
    return freed


def test_module() -> int:
    """Test free list allocator."""
    passed: int = 0
    total: int = 10
    nxt: list[int] = fl_init_free(total)
    head: list[int] = [0]

    # Test 1: initial free count
    fc: int = fl_count_free(nxt, head)
    if fc == 10:
        passed = passed + 1

    # Test 2: allocate block
    b1: int = fl_alloc(nxt, head)
    if b1 == 0:
        passed = passed + 1

    # Test 3: allocate multiple
    result: list[int] = fl_init_zeros(5)
    got: int = fl_alloc_n(nxt, head, 5, result)
    if got == 5:
        passed = passed + 1

    # Test 4: free count after allocation
    fc2: int = fl_count_free(nxt, head)
    if fc2 == 4:
        passed = passed + 1

    # Test 5: free blocks and recount
    fl_free_n(nxt, head, result, 5)
    fc3: int = fl_count_free(nxt, head)
    if fc3 == 9:
        passed = passed + 1

    return passed
