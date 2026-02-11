"""CFS-like (Completely Fair Scheduler) simulation.

Tracks virtual runtime per task. Always runs task with lowest
virtual runtime. Weight adjusts virtual runtime growth rate.
"""


def cfs_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def cfs_add_task(task_ids: list[int], weights: list[int], vruntimes: list[int],
                 count_arr: list[int], tid: int, weight: int) -> int:
    """Add task with weight. Returns index."""
    idx: int = count_arr[0]
    task_ids[idx] = tid
    weights[idx] = weight
    vruntimes[idx] = 0
    count_arr[0] = idx + 1
    return idx


def cfs_find_min_vruntime(vruntimes: list[int], count: int) -> int:
    """Find index of task with minimum virtual runtime."""
    if count == 0:
        return 0 - 1
    min_vr: int = vruntimes[0]
    min_idx: int = 0
    i: int = 1
    while i < count:
        vr: int = vruntimes[i]
        if vr < min_vr:
            min_vr = vr
            min_idx = i
        i = i + 1
    return min_idx


def cfs_run_tick(vruntimes: list[int], weights: list[int],
                 count: int, base_quantum: int) -> int:
    """Run one tick: pick min vruntime task, advance its vruntime.
    vruntime_delta = base_quantum * 1024 / weight. Returns task index."""
    idx: int = cfs_find_min_vruntime(vruntimes, count)
    if idx < 0:
        return 0 - 1
    w: int = weights[idx]
    delta: int = (base_quantum * 1024) // w
    vruntimes[idx] = vruntimes[idx] + delta
    return idx


def cfs_simulate(task_ids: list[int], vruntimes: list[int], weights: list[int],
                 runs: list[int], count: int, ticks: int, quantum: int) -> int:
    """Simulate CFS for given ticks. runs[i] counts how many times task i ran.
    Returns 0."""
    t: int = 0
    while t < ticks:
        idx: int = cfs_run_tick(vruntimes, weights, count, quantum)
        if idx >= 0:
            runs[idx] = runs[idx] + 1
        t = t + 1
    return 0


def cfs_vruntime_spread(vruntimes: list[int], count: int) -> int:
    """Max vruntime minus min vruntime."""
    if count == 0:
        return 0
    min_vr: int = vruntimes[0]
    max_vr: int = vruntimes[0]
    i: int = 1
    while i < count:
        vr: int = vruntimes[i]
        if vr < min_vr:
            min_vr = vr
        if vr > max_vr:
            max_vr = vr
        i = i + 1
    return max_vr - min_vr


def test_module() -> int:
    """Test CFS scheduler."""
    passed: int = 0
    cap: int = 5
    tids: list[int] = cfs_init_zeros(cap)
    weights: list[int] = cfs_init_zeros(cap)
    vrt: list[int] = cfs_init_zeros(cap)
    cnt: list[int] = [0]

    # Test 1: add tasks
    cfs_add_task(tids, weights, vrt, cnt, 10, 1024)
    cfs_add_task(tids, weights, vrt, cnt, 20, 512)
    cfs_add_task(tids, weights, vrt, cnt, 30, 2048)
    if cnt[0] == 3:
        passed = passed + 1

    # Test 2: min vruntime is initially 0 for first task
    min_idx: int = cfs_find_min_vruntime(vrt, cnt[0])
    if min_idx == 0:
        passed = passed + 1

    # Test 3: simulate and check all tasks ran
    runs: list[int] = cfs_init_zeros(cap)
    cfs_simulate(tids, vrt, weights, runs, cnt[0], 100, 4)
    r0: int = runs[0]
    r1: int = runs[1]
    r2: int = runs[2]
    if r0 > 0:
        if r1 > 0:
            if r2 > 0:
                passed = passed + 1

    # Test 4: higher weight gets more CPU
    if r2 > r1:
        passed = passed + 1

    # Test 5: vruntime spread is bounded (CFS is fair)
    spread: int = cfs_vruntime_spread(vrt, cnt[0])
    if spread < 100:
        passed = passed + 1

    return passed
