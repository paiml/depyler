"""Queue simulation operations.

Implements queue-based simulations using lists with
front pointer tracking for efficient dequeue operations.
"""


def simulate_queue(arrivals: list[int], service_times: list[int], count: int) -> int:
    """Simulate a simple queue and return total wait time.

    arrivals[i] is arrival time, service_times[i] is service duration.
    Both arrays are sorted by arrival time.
    """
    total_wait: int = 0
    current_time: int = 0
    i: int = 0
    while i < count:
        if current_time < arrivals[i]:
            current_time = arrivals[i]
        wait: int = current_time - arrivals[i]
        total_wait = total_wait + wait
        current_time = current_time + service_times[i]
        i = i + 1
    return total_wait


def circular_queue_ops(capacity: int, ops: list[int], op_count: int) -> int:
    """Simulate circular queue operations.

    ops values: positive = enqueue that value, -1 = dequeue.
    Returns sum of all dequeued values.
    """
    buffer: list[int] = []
    i: int = 0
    while i < capacity:
        buffer.append(0)
        i = i + 1
    head: int = 0
    tail: int = 0
    size: int = 0
    dequeue_sum: int = 0

    j: int = 0
    while j < op_count:
        op: int = ops[j]
        if op == -1:
            if size > 0:
                dequeue_sum = dequeue_sum + buffer[head]
                head = (head + 1) % capacity
                size = size - 1
        else:
            if size < capacity:
                buffer[tail] = op
                tail = (tail + 1) % capacity
                size = size + 1
        j = j + 1
    return dequeue_sum


def queue_max_length(arrivals: list[int], departures: list[int], event_count: int) -> int:
    """Find maximum queue length given arrival and departure events.

    Events are sorted by time. arrivals[i]=time, departures[i]=time.
    """
    max_len: int = 0
    current_len: int = 0
    ai: int = 0
    di: int = 0
    while ai < event_count or di < event_count:
        if ai < event_count and (di >= event_count or arrivals[ai] <= departures[di]):
            current_len = current_len + 1
            if current_len > max_len:
                max_len = current_len
            ai = ai + 1
        else:
            current_len = current_len - 1
            di = di + 1
    return max_len


def test_module() -> int:
    """Test queue simulation operations."""
    ok: int = 0

    arr_times: list[int] = [0, 2, 4]
    svc_times: list[int] = [3, 3, 3]
    wait: int = simulate_queue(arr_times, svc_times, 3)
    if wait == 3:
        ok = ok + 1

    ops: list[int] = [10, 20, 30, -1, -1]
    dq_sum: int = circular_queue_ops(5, ops, 5)
    if dq_sum == 30:
        ok = ok + 1

    arrivals: list[int] = [1, 2, 3]
    departures: list[int] = [4, 5, 6]
    ml: int = queue_max_length(arrivals, departures, 3)
    if ml == 3:
        ok = ok + 1

    return ok
