"""Memory compaction simulation.

Moves allocated objects to eliminate fragmentation gaps,
updating references as objects move.
"""


def mc_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def mc_init_neg(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def mc_count_objects(heap: list[int], heap_size: int) -> int:
    """Count non-zero (allocated) entries."""
    count: int = 0
    i: int = 0
    while i < heap_size:
        h: int = heap[i]
        if h != 0:
            count = count + 1
        i = i + 1
    return count


def mc_count_gaps(heap: list[int], heap_size: int) -> int:
    """Count number of gaps (contiguous free runs)."""
    gaps: int = 0
    in_gap: int = 0
    i: int = 0
    while i < heap_size:
        h: int = heap[i]
        if h == 0:
            if in_gap == 0:
                gaps = gaps + 1
                in_gap = 1
        else:
            in_gap = 0
        i = i + 1
    return gaps


def mc_compact(heap: list[int], heap_size: int, forwarding: list[int]) -> int:
    """Compact heap: move all objects to front.
    forwarding[old_pos] = new_pos for each moved object.
    Returns number of moves."""
    moves: int = 0
    write_pos: int = 0
    read_pos: int = 0
    while read_pos < heap_size:
        h: int = heap[read_pos]
        if h != 0:
            if read_pos != write_pos:
                heap[write_pos] = h
                heap[read_pos] = 0
                forwarding[read_pos] = write_pos
                moves = moves + 1
            else:
                forwarding[read_pos] = write_pos
            write_pos = write_pos + 1
        read_pos = read_pos + 1
    return moves


def mc_update_refs(refs: list[int], num_refs: int,
                   forwarding: list[int], heap_size: int) -> int:
    """Update reference array using forwarding table. Returns count updated."""
    updated: int = 0
    i: int = 0
    while i < num_refs:
        old_ref: int = refs[i]
        if old_ref >= 0:
            if old_ref < heap_size:
                new_ref: int = forwarding[old_ref]
                if new_ref >= 0:
                    if new_ref != old_ref:
                        refs[i] = new_ref
                        updated = updated + 1
        i = i + 1
    return updated


def mc_free_at_end(heap: list[int], heap_size: int) -> int:
    """Count free cells at end of heap."""
    count: int = 0
    i: int = heap_size - 1
    while i >= 0:
        h: int = heap[i]
        if h == 0:
            count = count + 1
        else:
            return count
        i = i - 1
    return count


def test_module() -> int:
    """Test memory compaction."""
    passed: int = 0
    heap_size: int = 12
    heap: list[int] = [10, 0, 20, 0, 0, 30, 0, 40, 0, 0, 50, 0]
    forwarding: list[int] = mc_init_neg(heap_size)

    # Test 1: count objects before compaction
    obj_count: int = mc_count_objects(heap, heap_size)
    if obj_count == 5:
        passed = passed + 1

    # Test 2: count gaps before compaction
    gaps: int = mc_count_gaps(heap, heap_size)
    if gaps >= 3:
        passed = passed + 1

    # Test 3: compact
    moves: int = mc_compact(heap, heap_size, forwarding)
    if moves > 0:
        passed = passed + 1

    # Test 4: after compaction, objects at front
    first: int = heap[0]
    if first == 10:
        passed = passed + 1

    # Test 5: all free space at end
    end_free: int = mc_free_at_end(heap, heap_size)
    if end_free == 7:
        passed = passed + 1

    return passed
