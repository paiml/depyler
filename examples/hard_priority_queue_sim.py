"""Priority queue simulation using a sorted array approach."""


def pq_insert(priorities: list[int], values: list[int], priority: int, value: int) -> int:
    """Insert an element into priority queue. Returns new size."""
    priorities.append(priority)
    values.append(value)
    # Bubble into sorted position (insertion sort step)
    pos: int = len(priorities) - 1
    while pos > 0:
        prev: int = pos - 1
        if priorities[prev] > priorities[pos]:
            tmp_p: int = priorities[prev]
            priorities[prev] = priorities[pos]
            priorities[pos] = tmp_p
            tmp_v: int = values[prev]
            values[prev] = values[pos]
            values[pos] = tmp_v
            pos = pos - 1
        else:
            pos = 0
    return len(priorities)


def pq_extract_min(priorities: list[int], values: list[int]) -> int:
    """Extract the minimum priority element. Returns its value, or -1 if empty."""
    if len(priorities) == 0:
        return -1
    val: int = values[0]
    # Shift elements left
    i: int = 1
    while i < len(priorities):
        prev: int = i - 1
        priorities[prev] = priorities[i]
        values[prev] = values[i]
        i = i + 1
    priorities.pop()
    values.pop()
    return val


def pq_peek_min(priorities: list[int]) -> int:
    """Peek at minimum priority without removing. Returns -1 if empty."""
    if len(priorities) == 0:
        return -1
    return priorities[0]


def simulate_task_scheduler(task_priorities: list[int], task_ids: list[int]) -> list[int]:
    """Simulate a task scheduler that processes tasks by priority.
    Returns order of task IDs processed."""
    pq_p: list[int] = []
    pq_v: list[int] = []
    i: int = 0
    while i < len(task_priorities):
        pq_insert(pq_p, pq_v, task_priorities[i], task_ids[i])
        i = i + 1
    order: list[int] = []
    while len(pq_p) > 0:
        task_id: int = pq_extract_min(pq_p, pq_v)
        order.append(task_id)
    return order


def test_module() -> int:
    """Test priority queue simulation."""
    ok: int = 0

    pq_p: list[int] = []
    pq_v: list[int] = []
    pq_insert(pq_p, pq_v, 3, 30)
    pq_insert(pq_p, pq_v, 1, 10)
    pq_insert(pq_p, pq_v, 2, 20)

    if pq_peek_min(pq_p) == 1:
        ok = ok + 1

    val1: int = pq_extract_min(pq_p, pq_v)
    if val1 == 10:
        ok = ok + 1

    val2: int = pq_extract_min(pq_p, pq_v)
    if val2 == 20:
        ok = ok + 1

    priorities: list[int] = [5, 1, 3, 2]
    ids: list[int] = [50, 10, 30, 20]
    order: list[int] = simulate_task_scheduler(priorities, ids)
    if order[0] == 10 and order[1] == 20 and order[2] == 30 and order[3] == 50:
        ok = ok + 1

    empty_p: list[int] = []
    empty_v: list[int] = []
    if pq_extract_min(empty_p, empty_v) == -1:
        ok = ok + 1

    if pq_peek_min(empty_p) == -1:
        ok = ok + 1

    return ok
