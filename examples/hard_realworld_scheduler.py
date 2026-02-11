"""Real-world task scheduler with priority queue.

Mimics: job schedulers, cron-like systems, work queue processors.
Uses sorted list as priority queue with deadline tracking.
"""


def create_task(task_id: int, priority: int, deadline: int, duration: int) -> list[int]:
    """Create a task as [task_id, priority, deadline, duration, status].
    Status: 0=pending, 1=running, 2=done, 3=missed."""
    return [task_id, priority, deadline, duration, 0]


def insert_sorted(queue: list[list[int]], tid: int, pri: int, dl: int, dur: int) -> int:
    """Insert task maintaining priority order (higher first). Returns queue size."""
    insert_pos: int = len(queue)
    idx: int = 0
    while idx < len(queue):
        if pri > queue[idx][1]:
            insert_pos = idx
            idx = len(queue)
        else:
            idx = idx + 1
    new_entry: list[int] = [tid, pri, dl, dur, 0]
    queue.append(new_entry)
    pos: int = len(queue) - 1
    while pos > insert_pos:
        queue[pos] = queue[pos - 1]
        pos = pos - 1
    queue[insert_pos] = [tid, pri, dl, dur, 0]
    return len(queue)


def pop_highest(queue: list[list[int]]) -> list[int]:
    """Remove and return highest priority task."""
    if len(queue) == 0:
        return [-1, -1, -1, -1, -1]
    tid: int = queue[0][0]
    pri: int = queue[0][1]
    dl: int = queue[0][2]
    dur: int = queue[0][3]
    stat: int = queue[0][4]
    idx: int = 0
    while idx < len(queue) - 1:
        queue[idx] = queue[idx + 1]
        idx = idx + 1
    queue.pop()
    return [tid, pri, dl, dur, stat]


def execute_task(task_id: int, task_deadline: int, task_duration: int, current_time: int) -> list[int]:
    """Execute a task. Returns [task_id, finish_time, status]."""
    finish_time: int = current_time + task_duration
    if current_time > task_deadline:
        return [task_id, finish_time, 3]
    return [task_id, finish_time, 2]


def run_scheduler(task_ids: list[int], priorities: list[int], deadlines: list[int], durations: list[int]) -> list[list[int]]:
    """Run all tasks through scheduler. Returns execution results."""
    queue: list[list[int]] = []
    idx: int = 0
    while idx < len(task_ids):
        insert_sorted(queue, task_ids[idx], priorities[idx], deadlines[idx], durations[idx])
        idx = idx + 1
    results: list[list[int]] = []
    current_time: int = 0
    while len(queue) > 0:
        task: list[int] = pop_highest(queue)
        res: list[int] = execute_task(task[0], task[2], task[3], current_time)
        current_time = res[1]
        results.append(res)
    return results


def count_completed(results: list[list[int]]) -> int:
    """Count tasks that completed successfully (status=2)."""
    count: int = 0
    idx: int = 0
    while idx < len(results):
        if results[idx][2] == 2:
            count = count + 1
        idx = idx + 1
    return count


def count_missed(results: list[list[int]]) -> int:
    """Count tasks that missed their deadline (status=3)."""
    count: int = 0
    idx: int = 0
    while idx < len(results):
        if results[idx][2] == 3:
            count = count + 1
        idx = idx + 1
    return count


def total_exec_time(results: list[list[int]]) -> int:
    """Get total time to execute all tasks."""
    if len(results) == 0:
        return 0
    return results[len(results) - 1][1]


def scheduler_summary(results: list[list[int]]) -> list[int]:
    """Return [completed, missed, total_time]."""
    comp: int = count_completed(results)
    miss: int = count_missed(results)
    tt: int = total_exec_time(results)
    return [comp, miss, tt]


def test_module() -> int:
    """Test scheduler module."""
    passed: int = 0

    # Test 1: create task
    t: list[int] = create_task(1, 5, 100, 10)
    if t[0] == 1 and t[1] == 5 and t[4] == 0:
        passed = passed + 1

    # Test 2: insert by priority
    q: list[list[int]] = []
    insert_sorted(q, 1, 3, 100, 10)
    insert_sorted(q, 2, 7, 100, 5)
    insert_sorted(q, 3, 5, 100, 8)
    if q[0][0] == 2:
        passed = passed + 1

    # Test 3: pop highest
    top: list[int] = pop_highest(q)
    if top[0] == 2 and len(q) == 2:
        passed = passed + 1

    # Test 4: execute on time
    res: list[int] = execute_task(10, 50, 10, 5)
    if res[0] == 10 and res[1] == 15 and res[2] == 2:
        passed = passed + 1

    # Test 5: missed deadline
    res2: list[int] = execute_task(11, 3, 10, 5)
    if res2[2] == 3:
        passed = passed + 1

    # Test 6: run scheduler
    ids: list[int] = [1, 2, 3]
    pris: list[int] = [3, 7, 5]
    dls: list[int] = [100, 50, 80]
    durs: list[int] = [10, 5, 8]
    results: list[list[int]] = run_scheduler(ids, pris, dls, durs)
    if len(results) == 3:
        passed = passed + 1

    # Test 7: count completed
    comp: int = count_completed(results)
    if comp >= 1:
        passed = passed + 1

    # Test 8: total time
    tt: int = total_exec_time(results)
    if tt == 23:
        passed = passed + 1

    return passed
