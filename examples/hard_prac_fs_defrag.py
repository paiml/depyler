"""File system defragmentation simulation.

Moves allocated blocks to eliminate gaps, creating contiguous
free space at the end of the disk.
"""


def df_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def df_count_fragments(disk: list[int], total: int) -> int:
    """Count number of separate allocated fragments on disk.
    A fragment is a contiguous run of non-zero blocks."""
    frags: int = 0
    in_alloc: int = 0
    i: int = 0
    while i < total:
        b: int = disk[i]
        if b != 0:
            if in_alloc == 0:
                frags = frags + 1
                in_alloc = 1
        else:
            in_alloc = 0
        i = i + 1
    return frags


def df_count_used(disk: list[int], total: int) -> int:
    """Count allocated blocks."""
    count: int = 0
    i: int = 0
    while i < total:
        b: int = disk[i]
        if b != 0:
            count = count + 1
        i = i + 1
    return count


def df_count_free(disk: list[int], total: int) -> int:
    """Count free blocks."""
    count: int = 0
    i: int = 0
    while i < total:
        b: int = disk[i]
        if b == 0:
            count = count + 1
        i = i + 1
    return count


def df_compact(disk: list[int], total: int) -> int:
    """Compact disk: move all allocated blocks to front. Returns moves made."""
    moves: int = 0
    write_pos: int = 0
    read_pos: int = 0
    while read_pos < total:
        b: int = disk[read_pos]
        if b != 0:
            if read_pos != write_pos:
                disk[write_pos] = b
                disk[read_pos] = 0
                moves = moves + 1
            write_pos = write_pos + 1
        read_pos = read_pos + 1
    return moves


def df_largest_free_run(disk: list[int], total: int) -> int:
    """Find largest contiguous free run."""
    best: int = 0
    current: int = 0
    i: int = 0
    while i < total:
        b: int = disk[i]
        if b == 0:
            current = current + 1
            if current > best:
                best = current
        else:
            current = 0
        i = i + 1
    return best


def df_free_space_at_end(disk: list[int], total: int) -> int:
    """Count free blocks at end of disk."""
    count: int = 0
    i: int = total - 1
    while i >= 0:
        b: int = disk[i]
        if b == 0:
            count = count + 1
        else:
            return count
        i = i - 1
    return count


def test_module() -> int:
    """Test defragmentation."""
    passed: int = 0
    total: int = 16
    # Create fragmented disk: blocks scattered
    disk: list[int] = [1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 0, 0, 6, 0, 0, 0]

    # Test 1: count fragments before defrag
    frags: int = df_count_fragments(disk, total)
    if frags == 6:
        passed = passed + 1

    # Test 2: count used blocks
    used: int = df_count_used(disk, total)
    if used == 6:
        passed = passed + 1

    # Test 3: compact the disk
    moves: int = df_compact(disk, total)
    if moves > 0:
        passed = passed + 1

    # Test 4: after compaction, 1 fragment
    frags2: int = df_count_fragments(disk, total)
    if frags2 == 1:
        passed = passed + 1

    # Test 5: all free space at end
    end_free: int = df_free_space_at_end(disk, total)
    if end_free == 10:
        passed = passed + 1

    return passed
