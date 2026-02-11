"""Hazard pointer simulation for safe memory reclamation.

Tracks hazard pointers per thread and a retired list.
Objects can only be freed when no thread has a hazard pointer to them.
"""


def create_hazard_array(num_threads: int, ptrs_per_thread: int) -> list[int]:
    """Create hazard pointer array. -1 means no hazard pointer set."""
    arr: list[int] = []
    total: int = num_threads * ptrs_per_thread
    i: int = 0
    while i < total:
        arr.append(0 - 1)
        i = i + 1
    return arr


def set_hazard(haz_array: list[int], thread_id: int, slot: int, ptrs_per: int, pointer_val: int) -> int:
    """Set hazard pointer for thread. Returns 1."""
    haz_array[thread_id * ptrs_per + slot] = pointer_val
    return 1


def clear_hazard(haz_array: list[int], thread_id: int, slot: int, ptrs_per: int) -> int:
    """Clear hazard pointer. Returns 1."""
    haz_array[thread_id * ptrs_per + slot] = 0 - 1
    return 1


def is_hazardous(haz_array: list[int], num_threads: int, ptrs_per: int, pointer_val: int) -> int:
    """Check if any thread has a hazard pointer to this value. Returns 1 if hazardous."""
    i: int = 0
    total: int = num_threads * ptrs_per
    while i < total:
        hv: int = haz_array[i]
        if hv == pointer_val:
            return 1
        i = i + 1
    return 0


def retire_node(retired_list: list[int], pointer_val: int) -> int:
    """Add pointer to retired list. Returns new size."""
    retired_list.append(pointer_val)
    return len(retired_list)


def scan_and_reclaim(retired_list: list[int], haz_array: list[int], num_threads: int, ptrs_per: int) -> list[int]:
    """Scan retired list and reclaim non-hazardous entries. Returns [reclaimed_count, remaining]."""
    reclaimed: int = 0
    remaining: list[int] = []
    i: int = 0
    while i < len(retired_list):
        rv: int = retired_list[i]
        if is_hazardous(haz_array, num_threads, ptrs_per, rv) == 0:
            reclaimed = reclaimed + 1
        else:
            remaining.append(rv)
        i = i + 1
    while len(retired_list) > 0:
        retired_list.pop()
    j: int = 0
    while j < len(remaining):
        rj: int = remaining[j]
        retired_list.append(rj)
        j = j + 1
    return [reclaimed, len(retired_list)]


def count_active_hazards(haz_array: list[int], num_threads: int, ptrs_per: int) -> int:
    """Count non-null hazard pointers."""
    cnt: int = 0
    total: int = num_threads * ptrs_per
    i: int = 0
    while i < total:
        hv: int = haz_array[i]
        if hv >= 0:
            cnt = cnt + 1
        i = i + 1
    return cnt


def test_module() -> int:
    """Test hazard pointer simulation."""
    ok: int = 0
    nt: int = 3
    pp: int = 2
    haz: list[int] = create_hazard_array(nt, pp)
    if len(haz) == 6:
        ok = ok + 1
    set_hazard(haz, 0, 0, pp, 100)
    set_hazard(haz, 1, 0, pp, 200)
    if is_hazardous(haz, nt, pp, 100) == 1:
        ok = ok + 1
    if is_hazardous(haz, nt, pp, 999) == 0:
        ok = ok + 1
    retired: list[int] = []
    retire_node(retired, 100)
    retire_node(retired, 300)
    retire_node(retired, 400)
    result: list[int] = scan_and_reclaim(retired, haz, nt, pp)
    reclaimed: int = result[0]
    if reclaimed == 2:
        ok = ok + 1
    active: int = count_active_hazards(haz, nt, pp)
    if active == 2:
        ok = ok + 1
    return ok
