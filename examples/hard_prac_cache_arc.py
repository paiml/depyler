"""ARC (Adaptive Replacement Cache) simplified simulation.

Uses two lists (recent and frequent) to adaptively balance between
recency and frequency based eviction.
"""


def arc_init_list(capacity: int) -> list[int]:
    """Initialize a list of given capacity with -1 (empty)."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def arc_find_in(lst: list[int], target: int, size: int) -> int:
    """Find target in list, return index or -1."""
    i: int = 0
    while i < size:
        elem: int = lst[i]
        if elem == target:
            return i
        i = i + 1
    return 0 - 1


def arc_count_valid(lst: list[int], size: int) -> int:
    """Count non-empty slots."""
    count: int = 0
    i: int = 0
    while i < size:
        elem: int = lst[i]
        if elem != (0 - 1):
            count = count + 1
        i = i + 1
    return count


def arc_remove_at(lst: list[int], idx: int, size: int) -> int:
    """Remove element at idx by shifting left. Returns removed value."""
    removed: int = lst[idx]
    j: int = idx
    while j < size - 1:
        next_val: int = lst[j + 1]
        lst[j] = next_val
        j = j + 1
    lst[size - 1] = 0 - 1
    return removed


def arc_append(lst: list[int], val: int, size: int) -> int:
    """Append val to first empty slot. Returns slot index or -1."""
    i: int = 0
    while i < size:
        elem: int = lst[i]
        if elem == (0 - 1):
            lst[i] = val
            return i
        i = i + 1
    return 0 - 1


def arc_access(recent: list[int], frequent: list[int],
               capacity: int, item: int) -> int:
    """Access an item. Returns 1 for hit, 0 for miss.
    If in recent, promote to frequent. If in frequent, keep.
    If miss, add to recent with possible eviction."""
    freq_idx: int = arc_find_in(frequent, item, capacity)
    if freq_idx >= 0:
        return 1

    rec_idx: int = arc_find_in(recent, item, capacity)
    if rec_idx >= 0:
        arc_remove_at(recent, rec_idx, capacity)
        freq_count: int = arc_count_valid(frequent, capacity)
        if freq_count >= capacity:
            arc_remove_at(frequent, 0, capacity)
        arc_append(frequent, item, capacity)
        return 1

    rec_count: int = arc_count_valid(recent, capacity)
    if rec_count >= capacity:
        arc_remove_at(recent, 0, capacity)
    arc_append(recent, item, capacity)
    return 0


def arc_is_cached(recent: list[int], frequent: list[int],
                  capacity: int, item: int) -> int:
    """Check if item is in either list."""
    r: int = arc_find_in(recent, item, capacity)
    if r >= 0:
        return 1
    f: int = arc_find_in(frequent, item, capacity)
    if f >= 0:
        return 1
    return 0


def test_module() -> int:
    """Test ARC cache operations."""
    passed: int = 0
    cap: int = 3
    recent: list[int] = arc_init_list(cap)
    frequent: list[int] = arc_init_list(cap)

    # Test 1: first access is a miss
    r1: int = arc_access(recent, frequent, cap, 10)
    if r1 == 0:
        passed = passed + 1

    # Test 2: item is now in recent
    cached: int = arc_is_cached(recent, frequent, cap, 10)
    if cached == 1:
        passed = passed + 1

    # Test 3: second access promotes to frequent (hit)
    r2: int = arc_access(recent, frequent, cap, 10)
    if r2 == 1:
        passed = passed + 1

    # Test 4: item now in frequent, not recent
    in_freq: int = arc_find_in(frequent, 10, cap)
    in_rec: int = arc_find_in(recent, 10, cap)
    if in_freq >= 0:
        if in_rec < 0:
            passed = passed + 1

    # Test 5: fill recent list and verify eviction
    arc_access(recent, frequent, cap, 20)
    arc_access(recent, frequent, cap, 30)
    arc_access(recent, frequent, cap, 40)
    arc_access(recent, frequent, cap, 50)
    evicted: int = arc_find_in(recent, 20, cap)
    if evicted < 0:
        passed = passed + 1

    return passed
