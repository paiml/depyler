"""Rate Monotonic scheduler simulation.

Fixed-priority scheduling where shorter period = higher priority.
Uses Liu-Layland utilization bound for schedulability test.
"""


def rm_init(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def rm_add_task(periods: list[int], wcets: list[int],
                count_arr: list[int], period: int, wcet: int) -> int:
    """Add task. Returns index."""
    idx: int = count_arr[0]
    periods[idx] = period
    wcets[idx] = wcet
    count_arr[0] = idx + 1
    return idx


def rm_assign_priorities(periods: list[int], priorities: list[int], count: int) -> int:
    """Assign priorities: shorter period = higher priority number.
    Uses selection sort on periods to determine rank."""
    ranks: list[int] = []
    i: int = 0
    while i < count:
        ranks.append(i)
        i = i + 1
    a: int = 0
    while a < count - 1:
        min_idx: int = a
        b: int = a + 1
        while b < count:
            p_b: int = periods[ranks[b]]
            p_min: int = periods[ranks[min_idx]]
            if p_b < p_min:
                min_idx = b
            b = b + 1
        if min_idx != a:
            tmp: int = ranks[a]
            ranks[a] = ranks[min_idx]
            ranks[min_idx] = tmp
        a = a + 1
    j: int = 0
    while j < count:
        task_idx: int = ranks[j]
        priorities[task_idx] = count - j
        j = j + 1
    return 0


def rm_utilization_pct(periods: list[int], wcets: list[int], count: int) -> int:
    """Compute utilization as integer percentage."""
    total: int = 0
    i: int = 0
    while i < count:
        w: int = wcets[i]
        p: int = periods[i]
        total = total + (w * 1000) // p
        i = i + 1
    return (total + 5) // 10


def rm_liu_layland_bound_pct(n: int) -> int:
    """Approximate Liu-Layland bound * 100 for n tasks.
    n*(2^(1/n)-1). Approximation: n=1->100, n=2->82, n=3->78, n=inf->69."""
    if n == 1:
        return 100
    if n == 2:
        return 82
    if n == 3:
        return 78
    if n == 4:
        return 75
    return 69


def rm_is_schedulable(periods: list[int], wcets: list[int], count: int) -> int:
    """Returns 1 if task set passes Liu-Layland bound."""
    util: int = rm_utilization_pct(periods, wcets, count)
    bound: int = rm_liu_layland_bound_pct(count)
    if util <= bound:
        return 1
    return 0


def rm_get_priority(priorities: list[int], task_idx: int) -> int:
    """Get priority of a task."""
    result: int = priorities[task_idx]
    return result


def test_module() -> int:
    """Test rate monotonic scheduler."""
    passed: int = 0
    cap: int = 10
    periods: list[int] = rm_init(cap)
    wcets: list[int] = rm_init(cap)
    priorities: list[int] = rm_init(cap)
    cnt: list[int] = [0]

    # Test 1: add tasks
    rm_add_task(periods, wcets, cnt, 10, 2)
    rm_add_task(periods, wcets, cnt, 5, 1)
    rm_add_task(periods, wcets, cnt, 20, 3)
    if cnt[0] == 3:
        passed = passed + 1

    # Test 2: priority assignment (shorter period = higher priority)
    rm_assign_priorities(periods, priorities, cnt[0])
    p_short: int = rm_get_priority(priorities, 1)
    p_long: int = rm_get_priority(priorities, 2)
    if p_short > p_long:
        passed = passed + 1

    # Test 3: utilization computation
    util: int = rm_utilization_pct(periods, wcets, cnt[0])
    if util > 0:
        if util < 100:
            passed = passed + 1

    # Test 4: schedulability test
    sched: int = rm_is_schedulable(periods, wcets, cnt[0])
    if sched == 1:
        passed = passed + 1

    # Test 5: Liu-Layland bound values
    b2: int = rm_liu_layland_bound_pct(2)
    b3: int = rm_liu_layland_bound_pct(3)
    if b2 > b3:
        passed = passed + 1

    return passed
