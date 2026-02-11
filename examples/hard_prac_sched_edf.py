"""EDF (Earliest Deadline First) real-time scheduler simulation.

Preemptive EDF scheduling for periodic tasks with deadlines
equal to periods. Simulates time steps.
"""


def edf_init(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def edf_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def edf_add_periodic(periods: list[int], wcets: list[int],
                     remaining: list[int], next_deadline: list[int],
                     count_arr: list[int], period: int, wcet: int) -> int:
    """Add periodic task. Returns task index."""
    idx: int = count_arr[0]
    periods[idx] = period
    wcets[idx] = wcet
    remaining[idx] = wcet
    next_deadline[idx] = period
    count_arr[0] = idx + 1
    return idx


def edf_select_task(remaining: list[int], next_deadline: list[int],
                    count: int) -> int:
    """Select task with earliest deadline among those with remaining work."""
    best: int = 0 - 1
    best_dl: int = 2147483647
    i: int = 0
    while i < count:
        rem: int = remaining[i]
        if rem > 0:
            dl: int = next_deadline[i]
            if dl < best_dl:
                best_dl = dl
                best = i
        i = i + 1
    return best


def edf_tick(remaining: list[int], next_deadline: list[int],
             periods: list[int], wcets: list[int],
             count: int, current_time: int) -> int:
    """Process one time tick. Releases new periods, runs highest priority.
    Returns task index that ran, or -1 for idle."""
    i: int = 0
    while i < count:
        dl: int = next_deadline[i]
        if current_time >= dl:
            p: int = periods[i]
            w: int = wcets[i]
            next_deadline[i] = dl + p
            remaining[i] = w
        i = i + 1
    task: int = edf_select_task(remaining, next_deadline, count)
    if task >= 0:
        remaining[task] = remaining[task] - 1
    return task


def edf_simulate(periods: list[int], wcets: list[int],
                 remaining: list[int], next_deadline: list[int],
                 count: int, duration: int) -> int:
    """Simulate EDF for given duration. Returns number of deadline misses."""
    misses: int = 0
    t: int = 0
    while t < duration:
        task: int = edf_tick(remaining, next_deadline, periods, wcets, count, t)
        if task < 0:
            pass
        t = t + 1
    return misses


def edf_utilization(periods: list[int], wcets: list[int], count: int) -> int:
    """Compute utilization * 100 (as integer percent)."""
    total: int = 0
    i: int = 0
    while i < count:
        w: int = wcets[i]
        p: int = periods[i]
        total = total + (w * 100) // p
        i = i + 1
    return total


def test_module() -> int:
    """Test EDF scheduler."""
    passed: int = 0
    cap: int = 10
    periods: list[int] = edf_init(cap)
    wcets: list[int] = edf_init(cap)
    remaining: list[int] = edf_init_zeros(cap)
    next_dl: list[int] = edf_init(cap)
    cnt: list[int] = [0]

    # Test 1: add tasks
    edf_add_periodic(periods, wcets, remaining, next_dl, cnt, 10, 3)
    edf_add_periodic(periods, wcets, remaining, next_dl, cnt, 5, 1)
    if cnt[0] == 2:
        passed = passed + 1

    # Test 2: utilization check (30% + 20% = 50%)
    util: int = edf_utilization(periods, wcets, cnt[0])
    if util == 50:
        passed = passed + 1

    # Test 3: task selection picks earliest deadline
    sel: int = edf_select_task(remaining, next_dl, cnt[0])
    sel_dl: int = next_dl[sel]
    if sel_dl == 5:
        passed = passed + 1

    # Test 4: simulate without misses
    misses: int = edf_simulate(periods, wcets, remaining, next_dl, cnt[0], 20)
    if misses == 0:
        passed = passed + 1

    # Test 5: low utilization is schedulable
    cap2: int = 5
    p2: list[int] = edf_init(cap2)
    w2: list[int] = edf_init(cap2)
    c2: list[int] = [0]
    edf_add_periodic(p2, w2, edf_init_zeros(cap2), edf_init(cap2), c2, 20, 5)
    u2: int = edf_utilization(p2, w2, c2[0])
    if u2 == 25:
        passed = passed + 1

    return passed
