"""Deadline scheduler simulation.

Schedules tasks by earliest absolute deadline. Tasks with missed
deadlines are tracked for penalty accounting.
"""


def dl_init(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def dl_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def dl_add_task(task_ids: list[int], deadlines: list[int], durations: list[int],
                count_arr: list[int], tid: int, deadline: int, duration: int) -> int:
    """Add a task. count_arr[0] tracks count. Returns 1 on success."""
    idx: int = count_arr[0]
    task_ids[idx] = tid
    deadlines[idx] = deadline
    durations[idx] = duration
    count_arr[0] = idx + 1
    return 1


def dl_find_earliest(deadlines: list[int], done: list[int], count: int) -> int:
    """Find index of task with earliest deadline among undone tasks."""
    best_idx: int = 0 - 1
    best_dl: int = 2147483647
    i: int = 0
    while i < count:
        d_flag: int = done[i]
        if d_flag == 0:
            dl: int = deadlines[i]
            if dl < best_dl:
                best_dl = dl
                best_idx = i
        i = i + 1
    return best_idx


def dl_schedule(task_ids: list[int], deadlines: list[int], durations: list[int],
                done: list[int], count: int) -> int:
    """Schedule all tasks by earliest deadline. Returns count of missed deadlines."""
    current_time: int = 0
    missed: int = 0
    scheduled: int = 0
    while scheduled < count:
        idx: int = dl_find_earliest(deadlines, done, count)
        if idx < 0:
            scheduled = count
        else:
            dur: int = durations[idx]
            dl: int = deadlines[idx]
            finish: int = current_time + dur
            if finish > dl:
                missed = missed + 1
            current_time = finish
            done[idx] = 1
            scheduled = scheduled + 1
    return missed


def dl_count_done(done: list[int], count: int) -> int:
    """Count completed tasks."""
    total: int = 0
    i: int = 0
    while i < count:
        d: int = done[i]
        if d == 1:
            total = total + 1
        i = i + 1
    return total


def dl_total_duration(durations: list[int], count: int) -> int:
    """Sum of all task durations."""
    total: int = 0
    i: int = 0
    while i < count:
        d: int = durations[i]
        total = total + d
        i = i + 1
    return total


def test_module() -> int:
    """Test deadline scheduler."""
    passed: int = 0
    cap: int = 10
    tids: list[int] = dl_init(cap)
    dlines: list[int] = dl_init(cap)
    durs: list[int] = dl_init(cap)
    done: list[int] = dl_init_zeros(cap)
    cnt: list[int] = [0]

    # Test 1: add tasks and verify count
    dl_add_task(tids, dlines, durs, cnt, 1, 10, 3)
    dl_add_task(tids, dlines, durs, cnt, 2, 5, 2)
    dl_add_task(tids, dlines, durs, cnt, 3, 15, 4)
    if cnt[0] == 3:
        passed = passed + 1

    # Test 2: earliest deadline is task 2
    earliest: int = dl_find_earliest(dlines, done, cnt[0])
    eid: int = tids[earliest]
    if eid == 2:
        passed = passed + 1

    # Test 3: all tasks complete with EDF
    missed: int = dl_schedule(tids, dlines, durs, done, cnt[0])
    completed: int = dl_count_done(done, cnt[0])
    if completed == 3:
        passed = passed + 1

    # Test 4: no missed deadlines for feasible schedule
    if missed == 0:
        passed = passed + 1

    # Test 5: total duration is correct
    td: int = dl_total_duration(durs, cnt[0])
    if td == 9:
        passed = passed + 1

    return passed
