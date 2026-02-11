"""FAT (File Allocation Table) simulation.

Each entry in the FAT points to the next cluster in the chain.
Special values: -1 = free, -2 = end of chain.
"""


def fat_init(num_clusters: int) -> list[int]:
    """Initialize FAT with all clusters free (-1)."""
    result: list[int] = []
    i: int = 0
    while i < num_clusters:
        result.append(0 - 1)
        i = i + 1
    return result


def fat_find_free(fat: list[int], num_clusters: int) -> int:
    """Find first free cluster. Returns cluster number or -1 if full."""
    i: int = 0
    while i < num_clusters:
        f: int = fat[i]
        if f == (0 - 1):
            return i
        i = i + 1
    return 0 - 1


def fat_alloc_chain(fat: list[int], num_clusters: int, needed: int) -> int:
    """Allocate a chain of clusters. Returns first cluster or -1."""
    if needed == 0:
        return 0 - 1
    first: int = 0 - 1
    prev: int = 0 - 1
    allocated: int = 0
    i: int = 0
    while i < num_clusters:
        if allocated >= needed:
            i = num_clusters
        else:
            f: int = fat[i]
            if f == (0 - 1):
                if first < 0:
                    first = i
                if prev >= 0:
                    fat[prev] = i
                prev = i
                fat[i] = 0 - 2
                allocated = allocated + 1
        i = i + 1
    if allocated < needed:
        return 0 - 1
    return first


def fat_chain_length(fat: list[int], start: int) -> int:
    """Follow chain and return length."""
    if start < 0:
        return 0
    count: int = 0
    current: int = start
    while current >= 0:
        count = count + 1
        nxt: int = fat[current]
        if nxt == (0 - 2):
            return count
        current = nxt
    return count


def fat_free_chain(fat: list[int], start: int) -> int:
    """Free entire chain. Returns count of clusters freed."""
    if start < 0:
        return 0
    count: int = 0
    current: int = start
    while current >= 0:
        nxt: int = fat[current]
        fat[current] = 0 - 1
        count = count + 1
        if nxt == (0 - 2):
            return count
        current = nxt
    return count


def fat_count_free(fat: list[int], num_clusters: int) -> int:
    """Count free clusters."""
    count: int = 0
    i: int = 0
    while i < num_clusters:
        f: int = fat[i]
        if f == (0 - 1):
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test FAT operations."""
    passed: int = 0
    num_cl: int = 16
    fat: list[int] = fat_init(num_cl)

    # Test 1: all clusters initially free
    free: int = fat_count_free(fat, num_cl)
    if free == 16:
        passed = passed + 1

    # Test 2: allocate chain
    chain1: int = fat_alloc_chain(fat, num_cl, 4)
    if chain1 == 0:
        passed = passed + 1

    # Test 3: chain length
    ln: int = fat_chain_length(fat, chain1)
    if ln == 4:
        passed = passed + 1

    # Test 4: free count after allocation
    free2: int = fat_count_free(fat, num_cl)
    if free2 == 12:
        passed = passed + 1

    # Test 5: free chain and verify
    freed: int = fat_free_chain(fat, chain1)
    free3: int = fat_count_free(fat, num_cl)
    if freed == 4:
        if free3 == 16:
            passed = passed + 1

    return passed
