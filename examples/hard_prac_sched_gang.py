"""Gang scheduler simulation.

Co-schedules groups of related tasks on multiple processors simultaneously.
All tasks in a gang run together or not at all.
"""


def gang_init(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def gang_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def gang_add_group(group_ids: list[int], group_sizes: list[int],
                   group_count: list[int], gid: int, size: int) -> int:
    """Add a gang group. Returns index."""
    idx: int = group_count[0]
    group_ids[idx] = gid
    group_sizes[idx] = size
    group_count[0] = idx + 1
    return idx


def gang_can_schedule(group_sizes: list[int], idx: int, num_cpus: int,
                      cpu_busy: list[int]) -> int:
    """Check if gang at idx can fit on available CPUs. Returns 1 if yes."""
    needed: int = group_sizes[idx]
    avail: int = 0
    c: int = 0
    while c < num_cpus:
        b: int = cpu_busy[c]
        if b == 0:
            avail = avail + 1
        c = c + 1
    if avail >= needed:
        return 1
    return 0


def gang_schedule(group_ids: list[int], group_sizes: list[int],
                  cpu_assign: list[int], cpu_busy: list[int],
                  num_cpus: int, group_idx: int) -> int:
    """Schedule gang onto CPUs. Returns number of CPUs assigned."""
    needed: int = group_sizes[group_idx]
    gid: int = group_ids[group_idx]
    assigned: int = 0
    c: int = 0
    while c < num_cpus:
        if assigned >= needed:
            c = num_cpus
        else:
            b: int = cpu_busy[c]
            if b == 0:
                cpu_assign[c] = gid
                cpu_busy[c] = 1
                assigned = assigned + 1
        c = c + 1
    return assigned


def gang_unschedule(cpu_assign: list[int], cpu_busy: list[int],
                    num_cpus: int, gid: int) -> int:
    """Remove gang from CPUs. Returns number of CPUs freed."""
    freed: int = 0
    c: int = 0
    while c < num_cpus:
        a: int = cpu_assign[c]
        if a == gid:
            cpu_assign[c] = 0 - 1
            cpu_busy[c] = 0
            freed = freed + 1
        c = c + 1
    return freed


def gang_count_busy(cpu_busy: list[int], num_cpus: int) -> int:
    """Count busy CPUs."""
    count: int = 0
    c: int = 0
    while c < num_cpus:
        b: int = cpu_busy[c]
        if b == 1:
            count = count + 1
        c = c + 1
    return count


def test_module() -> int:
    """Test gang scheduler."""
    passed: int = 0
    num_cpus: int = 4
    cap: int = 5
    gids: list[int] = gang_init(cap)
    gsizes: list[int] = gang_init_zeros(cap)
    gcnt: list[int] = [0]
    cpu_assign: list[int] = gang_init(num_cpus)
    cpu_busy: list[int] = gang_init_zeros(num_cpus)

    # Test 1: add gangs
    gang_add_group(gids, gsizes, gcnt, 10, 2)
    gang_add_group(gids, gsizes, gcnt, 20, 3)
    if gcnt[0] == 2:
        passed = passed + 1

    # Test 2: can schedule first gang
    can1: int = gang_can_schedule(gsizes, 0, num_cpus, cpu_busy)
    if can1 == 1:
        passed = passed + 1

    # Test 3: schedule first gang uses 2 CPUs
    assigned: int = gang_schedule(gids, gsizes, cpu_assign, cpu_busy, num_cpus, 0)
    if assigned == 2:
        passed = passed + 1

    # Test 4: second gang cannot fit (needs 3, only 2 free)
    can2: int = gang_can_schedule(gsizes, 1, num_cpus, cpu_busy)
    if can2 == 0:
        passed = passed + 1

    # Test 5: unschedule frees CPUs
    freed: int = gang_unschedule(cpu_assign, cpu_busy, num_cpus, 10)
    busy: int = gang_count_busy(cpu_busy, num_cpus)
    if freed == 2:
        if busy == 0:
            passed = passed + 1

    return passed
