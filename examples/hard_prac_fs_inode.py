"""Inode allocation simulation.

Manages an inode table with allocation, deallocation,
and metadata tracking (size, link count, permissions).
"""


def inode_init(capacity: int) -> list[int]:
    """Initialize with -1 (free)."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def inode_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def inode_alloc(owners: list[int], sizes: list[int], links: list[int],
                capacity: int, owner: int) -> int:
    """Allocate a free inode. Returns inode number or -1 if full."""
    i: int = 0
    while i < capacity:
        o: int = owners[i]
        if o == (0 - 1):
            owners[i] = owner
            sizes[i] = 0
            links[i] = 1
            return i
        i = i + 1
    return 0 - 1


def inode_free(owners: list[int], sizes: list[int], links: list[int],
               capacity: int, ino: int) -> int:
    """Free an inode. Returns 1 on success, 0 if already free."""
    o: int = owners[ino]
    if o == (0 - 1):
        return 0
    owners[ino] = 0 - 1
    sizes[ino] = 0
    links[ino] = 0
    return 1


def inode_set_size(sizes: list[int], ino: int, new_size: int) -> int:
    """Set file size. Returns old size."""
    old: int = sizes[ino]
    sizes[ino] = new_size
    return old


def inode_add_link(links: list[int], ino: int) -> int:
    """Increment link count. Returns new count."""
    links[ino] = links[ino] + 1
    result: int = links[ino]
    return result


def inode_remove_link(links: list[int], ino: int) -> int:
    """Decrement link count. Returns new count."""
    links[ino] = links[ino] - 1
    result: int = links[ino]
    return result


def inode_count_allocated(owners: list[int], capacity: int) -> int:
    """Count allocated inodes."""
    count: int = 0
    i: int = 0
    while i < capacity:
        o: int = owners[i]
        if o != (0 - 1):
            count = count + 1
        i = i + 1
    return count


def inode_total_size(sizes: list[int], owners: list[int], capacity: int) -> int:
    """Sum of sizes of all allocated inodes."""
    total: int = 0
    i: int = 0
    while i < capacity:
        o: int = owners[i]
        if o != (0 - 1):
            s: int = sizes[i]
            total = total + s
        i = i + 1
    return total


def test_module() -> int:
    """Test inode allocation."""
    passed: int = 0
    cap: int = 8
    owners: list[int] = inode_init(cap)
    sizes: list[int] = inode_init_zeros(cap)
    links: list[int] = inode_init_zeros(cap)

    # Test 1: allocate inode
    ino1: int = inode_alloc(owners, sizes, links, cap, 1000)
    if ino1 == 0:
        passed = passed + 1

    # Test 2: set size and verify
    inode_set_size(sizes, ino1, 4096)
    s: int = sizes[ino1]
    if s == 4096:
        passed = passed + 1

    # Test 3: link count management
    inode_add_link(links, ino1)
    lc: int = links[ino1]
    if lc == 2:
        passed = passed + 1

    # Test 4: allocate multiple and count
    inode_alloc(owners, sizes, links, cap, 1001)
    inode_alloc(owners, sizes, links, cap, 1002)
    cnt: int = inode_count_allocated(owners, cap)
    if cnt == 3:
        passed = passed + 1

    # Test 5: free and recount
    inode_free(owners, sizes, links, cap, ino1)
    cnt2: int = inode_count_allocated(owners, cap)
    if cnt2 == 2:
        passed = passed + 1

    return passed
