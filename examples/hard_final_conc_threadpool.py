"""Thread pool simulation with task queue and worker scheduling.

Simulates a fixed-size thread pool processing a queue of tasks.
Each task has an id, duration, and priority. Workers pick tasks in order.
"""


def create_task_queue(ids: list[int], durations: list[int], priorities: list[int]) -> list[int]:
    """Create flat task queue: [id0, dur0, pri0, id1, dur1, pri1, ...]."""
    result: list[int] = []
    i: int = 0
    while i < len(ids):
        result.append(ids[i])
        result.append(durations[i])
        result.append(priorities[i])
        i = i + 1
    return result


def get_task_id(queue: list[int], idx: int) -> int:
    """Get task id at index."""
    return queue[idx * 3]


def get_task_dur(queue: list[int], idx: int) -> int:
    """Get task duration at index."""
    return queue[idx * 3 + 1]


def get_task_pri(queue: list[int], idx: int) -> int:
    """Get task priority at index."""
    return queue[idx * 3 + 2]


def num_tasks(queue: list[int]) -> int:
    """Number of tasks in queue."""
    return len(queue) // 3


def schedule_fifo(queue: list[int], num_workers: int) -> list[int]:
    """FIFO scheduling. Returns [total_time, tasks_completed].

    Workers process tasks in order. Total time = max of all worker finish times.
    """
    worker_times: list[int] = []
    w: int = 0
    while w < num_workers:
        worker_times.append(0)
        w = w + 1
    completed: int = 0
    ntasks: int = num_tasks(queue)
    i: int = 0
    while i < ntasks:
        min_worker: int = 0
        min_time: int = worker_times[0]
        j: int = 1
        while j < num_workers:
            wt: int = worker_times[j]
            if wt < min_time:
                min_time = wt
                min_worker = j
            j = j + 1
        dur: int = get_task_dur(queue, i)
        old: int = worker_times[min_worker]
        worker_times[min_worker] = old + dur
        completed = completed + 1
        i = i + 1
    max_time: int = 0
    k: int = 0
    while k < num_workers:
        wt2: int = worker_times[k]
        if wt2 > max_time:
            max_time = wt2
        k = k + 1
    return [max_time, completed]


def schedule_priority(queue: list[int], num_workers: int) -> list[int]:
    """Priority scheduling (higher priority first). Returns [total_time, tasks_completed]."""
    ntasks: int = num_tasks(queue)
    order: list[int] = []
    ti: int = 0
    while ti < ntasks:
        order.append(ti)
        ti = ti + 1
    si: int = 0
    while si < ntasks:
        sj: int = si + 1
        while sj < ntasks:
            pi: int = get_task_pri(queue, order[si])
            pj: int = get_task_pri(queue, order[sj])
            if pj > pi:
                tmp: int = order[si]
                order[si] = order[sj]
                order[sj] = tmp
            sj = sj + 1
        si = si + 1
    worker_times: list[int] = []
    w: int = 0
    while w < num_workers:
        worker_times.append(0)
        w = w + 1
    completed: int = 0
    oi: int = 0
    while oi < ntasks:
        task_idx: int = order[oi]
        min_worker: int = 0
        min_time: int = worker_times[0]
        j: int = 1
        while j < num_workers:
            wt: int = worker_times[j]
            if wt < min_time:
                min_time = wt
                min_worker = j
            j = j + 1
        dur: int = get_task_dur(queue, task_idx)
        old: int = worker_times[min_worker]
        worker_times[min_worker] = old + dur
        completed = completed + 1
        oi = oi + 1
    max_time: int = 0
    k: int = 0
    while k < num_workers:
        wt2: int = worker_times[k]
        if wt2 > max_time:
            max_time = wt2
        k = k + 1
    return [max_time, completed]


def test_module() -> int:
    """Test thread pool simulation."""
    ok: int = 0
    ids: list[int] = [1, 2, 3, 4]
    durs: list[int] = [10, 20, 10, 20]
    pris: list[int] = [1, 3, 2, 4]
    queue: list[int] = create_task_queue(ids, durs, pris)
    if num_tasks(queue) == 4:
        ok = ok + 1
    fifo: list[int] = schedule_fifo(queue, 2)
    f_time: int = fifo[0]
    f_done: int = fifo[1]
    if f_done == 4:
        ok = ok + 1
    if f_time == 40:
        ok = ok + 1
    pri_res: list[int] = schedule_priority(queue, 2)
    p_done: int = pri_res[1]
    if p_done == 4:
        ok = ok + 1
    tid: int = get_task_id(queue, 0)
    if tid == 1:
        ok = ok + 1
    return ok
