"""Work stealing scheduler simulation.

Each worker has a local deque of tasks. When a worker's deque is empty,
it steals from another worker's deque (from the opposite end).
"""


def create_deques(num_workers: int, tasks_per: int) -> list[int]:
    """Create flat deque storage. deques[worker * max_tasks + i] = task value.

    sizes[worker] = current deque size.
    """
    total: int = num_workers * tasks_per
    storage: list[int] = []
    i: int = 0
    while i < total:
        storage.append(0)
        i = i + 1
    return storage


def push_task(storage: list[int], sizes: list[int], worker: int, max_tasks: int, task_val: int) -> int:
    """Push task to worker's deque (push to tail). Returns 1 if success."""
    sz: int = sizes[worker]
    if sz >= max_tasks:
        return 0
    storage[worker * max_tasks + sz] = task_val
    sizes[worker] = sz + 1
    return 1


def pop_task(storage: list[int], sizes: list[int], worker: int, max_tasks: int) -> int:
    """Pop task from worker's own deque (pop from tail). Returns task or -1."""
    sz: int = sizes[worker]
    if sz == 0:
        return 0 - 1
    sizes[worker] = sz - 1
    return storage[worker * max_tasks + sz - 1]


def steal_task(storage: list[int], sizes: list[int], victim: int, max_tasks: int) -> int:
    """Steal task from victim's deque (steal from head). Returns task or -1."""
    sz: int = sizes[victim]
    if sz == 0:
        return 0 - 1
    stolen: int = storage[victim * max_tasks]
    j: int = 0
    while j < sz - 1:
        storage[victim * max_tasks + j] = storage[victim * max_tasks + j + 1]
        j = j + 1
    sizes[victim] = sz - 1
    return stolen


def simulate_work_stealing(num_workers: int, max_tasks: int, all_tasks: list[int]) -> list[int]:
    """Distribute tasks round-robin, then simulate work stealing.

    Returns [total_executed, steal_count].
    """
    storage: list[int] = create_deques(num_workers, max_tasks)
    sizes: list[int] = []
    wi: int = 0
    while wi < num_workers:
        sizes.append(0)
        wi = wi + 1
    i: int = 0
    while i < len(all_tasks):
        worker: int = i % num_workers
        tv: int = all_tasks[i]
        push_task(storage, sizes, worker, max_tasks, tv)
        i = i + 1
    executed: int = 0
    steals: int = 0
    rounds: int = 0
    max_rounds: int = len(all_tasks) + num_workers
    while rounds < max_rounds:
        any_work: int = 0
        w: int = 0
        while w < num_workers:
            task_val: int = pop_task(storage, sizes, w, max_tasks)
            if task_val >= 0:
                executed = executed + 1
                any_work = 1
            else:
                victim: int = (w + 1) % num_workers
                stolen: int = steal_task(storage, sizes, victim, max_tasks)
                if stolen >= 0:
                    executed = executed + 1
                    steals = steals + 1
                    any_work = 1
            w = w + 1
        if any_work == 0:
            rounds = max_rounds
        rounds = rounds + 1
    return [executed, steals]


def test_module() -> int:
    """Test work stealing scheduler."""
    ok: int = 0
    mt: int = 10
    st: list[int] = create_deques(2, mt)
    sz: list[int] = [0, 0]
    push_task(st, sz, 0, mt, 42)
    push_task(st, sz, 0, mt, 99)
    v: int = pop_task(st, sz, 0, mt)
    if v == 99:
        ok = ok + 1
    push_task(st, sz, 1, mt, 50)
    stolen: int = steal_task(st, sz, 1, mt)
    if stolen == 50:
        ok = ok + 1
    tasks: list[int] = [1, 2, 3, 4, 5, 6]
    result: list[int] = simulate_work_stealing(2, mt, tasks)
    r0: int = result[0]
    if r0 == 6:
        ok = ok + 1
    empty: int = pop_task(st, sz, 1, mt)
    if empty == 0 - 1:
        ok = ok + 1
    s0: int = sz[0]
    if s0 == 1:
        ok = ok + 1
    return ok
