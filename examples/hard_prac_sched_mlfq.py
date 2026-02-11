"""Multi-Level Feedback Queue (MLFQ) scheduler simulation.

Multiple priority queues. Tasks start at highest priority.
If they use full quantum, they move down. Periodic boost moves all up.
"""


def mlfq_init(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def mlfq_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def mlfq_add_task(task_ids: list[int], levels: list[int],
                  allotments: list[int], count_arr: list[int],
                  tid: int, num_levels: int) -> int:
    """Add task at highest priority level (0). Returns index."""
    idx: int = count_arr[0]
    task_ids[idx] = tid
    levels[idx] = 0
    allotments[idx] = 0
    count_arr[0] = idx + 1
    return idx


def mlfq_find_highest(levels: list[int], task_ids: list[int], count: int) -> int:
    """Find task at highest priority (lowest level number). Returns index or -1."""
    best: int = 0 - 1
    best_level: int = 2147483647
    i: int = 0
    while i < count:
        tid: int = task_ids[i]
        if tid != (0 - 1):
            lv: int = levels[i]
            if lv < best_level:
                best_level = lv
                best = i
        i = i + 1
    return best


def mlfq_run_quantum(levels: list[int], allotments: list[int],
                     runs: list[int], idx: int, quantum: int,
                     max_allot: int, num_levels: int) -> int:
    """Run task for one quantum. Demote if allotment exceeded. Returns new level."""
    runs[idx] = runs[idx] + 1
    allotments[idx] = allotments[idx] + quantum
    allot: int = allotments[idx]
    if allot >= max_allot:
        current: int = levels[idx]
        if current < num_levels - 1:
            levels[idx] = current + 1
        allotments[idx] = 0
    result: int = levels[idx]
    return result


def mlfq_boost(levels: list[int], allotments: list[int], count: int) -> int:
    """Boost all tasks to highest priority. Returns count of boosted tasks."""
    boosted: int = 0
    i: int = 0
    while i < count:
        lv: int = levels[i]
        if lv > 0:
            levels[i] = 0
            allotments[i] = 0
            boosted = boosted + 1
        i = i + 1
    return boosted


def mlfq_count_at_level(levels: list[int], task_ids: list[int],
                        count: int, target_level: int) -> int:
    """Count tasks at a specific level."""
    total: int = 0
    i: int = 0
    while i < count:
        tid: int = task_ids[i]
        if tid != (0 - 1):
            lv: int = levels[i]
            if lv == target_level:
                total = total + 1
        i = i + 1
    return total


def test_module() -> int:
    """Test MLFQ scheduler."""
    passed: int = 0
    cap: int = 10
    num_levels: int = 3
    tids: list[int] = mlfq_init(cap)
    levels: list[int] = mlfq_init_zeros(cap)
    allots: list[int] = mlfq_init_zeros(cap)
    runs: list[int] = mlfq_init_zeros(cap)
    cnt: list[int] = [0]

    # Test 1: add tasks at highest priority
    mlfq_add_task(tids, levels, allots, cnt, 10, num_levels)
    mlfq_add_task(tids, levels, allots, cnt, 20, num_levels)
    at_top: int = mlfq_count_at_level(levels, tids, cnt[0], 0)
    if at_top == 2:
        passed = passed + 1

    # Test 2: highest priority selection
    best: int = mlfq_find_highest(levels, tids, cnt[0])
    if best == 0:
        passed = passed + 1

    # Test 3: demotion after allotment exceeded
    mlfq_run_quantum(levels, allots, runs, 0, 10, 20, num_levels)
    mlfq_run_quantum(levels, allots, runs, 0, 10, 20, num_levels)
    lv0: int = levels[0]
    if lv0 == 1:
        passed = passed + 1

    # Test 4: boost brings all back to top
    levels[1] = 2
    boosted: int = mlfq_boost(levels, allots, cnt[0])
    if boosted == 2:
        passed = passed + 1

    # Test 5: all at level 0 after boost
    at_top2: int = mlfq_count_at_level(levels, tids, cnt[0], 0)
    if at_top2 == 2:
        passed = passed + 1

    return passed
