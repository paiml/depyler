"""Work-conserving scheduler simulation.

Never leaves a CPU idle if there is work to do. Implements work stealing
between queues to maintain high utilization.
"""


def wc_init(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def wc_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def wc_enqueue(queue: list[int], tail_arr: list[int],
               queue_idx: int, task_id: int, max_per_q: int) -> int:
    """Enqueue task to specific CPU queue. Returns 1 on success."""
    base_offset: int = queue_idx * max_per_q
    t: int = tail_arr[queue_idx]
    if t >= max_per_q:
        return 0
    queue[base_offset + t] = task_id
    tail_arr[queue_idx] = t + 1
    return 1


def wc_dequeue(queue: list[int], tail_arr: list[int],
               queue_idx: int, max_per_q: int) -> int:
    """Dequeue from front of queue. Returns task_id or -1."""
    t: int = tail_arr[queue_idx]
    if t == 0:
        return 0 - 1
    base_offset: int = queue_idx * max_per_q
    result: int = queue[base_offset]
    j: int = 0
    while j < t - 1:
        next_val: int = queue[base_offset + j + 1]
        queue[base_offset + j] = next_val
        j = j + 1
    queue[base_offset + t - 1] = 0 - 1
    tail_arr[queue_idx] = t - 1
    return result


def wc_queue_length(tail_arr: list[int], queue_idx: int) -> int:
    """Get length of specific queue."""
    result: int = tail_arr[queue_idx]
    return result


def wc_find_longest_queue(tail_arr: list[int], num_cpus: int) -> int:
    """Find index of queue with most tasks."""
    best: int = 0
    best_len: int = tail_arr[0]
    i: int = 1
    while i < num_cpus:
        l: int = tail_arr[i]
        if l > best_len:
            best_len = l
            best = i
        i = i + 1
    return best


def wc_steal(queue: list[int], tail_arr: list[int],
             from_q: int, to_q: int, max_per_q: int) -> int:
    """Steal one task from from_q to to_q. Returns 1 on success."""
    task: int = wc_dequeue(queue, tail_arr, from_q, max_per_q)
    if task == (0 - 1):
        return 0
    ok: int = wc_enqueue(queue, tail_arr, to_q, task, max_per_q)
    return ok


def wc_total_tasks(tail_arr: list[int], num_cpus: int) -> int:
    """Total tasks across all queues."""
    total: int = 0
    i: int = 0
    while i < num_cpus:
        t: int = tail_arr[i]
        total = total + t
        i = i + 1
    return total


def test_module() -> int:
    """Test work-conserving scheduler."""
    passed: int = 0
    num_cpus: int = 3
    max_per_q: int = 5
    total_slots: int = num_cpus * max_per_q
    queue: list[int] = wc_init(total_slots)
    tails: list[int] = wc_init_zeros(num_cpus)

    # Test 1: enqueue and dequeue
    wc_enqueue(queue, tails, 0, 100, max_per_q)
    wc_enqueue(queue, tails, 0, 200, max_per_q)
    got: int = wc_dequeue(queue, tails, 0, max_per_q)
    if got == 100:
        passed = passed + 1

    # Test 2: queue length tracking
    ln: int = wc_queue_length(tails, 0)
    if ln == 1:
        passed = passed + 1

    # Test 3: work stealing
    wc_enqueue(queue, tails, 0, 300, max_per_q)
    wc_enqueue(queue, tails, 0, 400, max_per_q)
    stolen: int = wc_steal(queue, tails, 0, 1, max_per_q)
    if stolen == 1:
        passed = passed + 1

    # Test 4: find longest queue
    longest: int = wc_find_longest_queue(tails, num_cpus)
    if longest == 0:
        passed = passed + 1

    # Test 5: total tasks
    total: int = wc_total_tasks(tails, num_cpus)
    if total == 3:
        passed = passed + 1

    return passed
