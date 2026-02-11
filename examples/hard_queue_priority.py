# Priority queue using sorted array


def pq_create() -> list[int]:
    result: list[int] = []
    return result


def pq_insert(queue: list[int], value: int) -> list[int]:
    new_queue: list[int] = []
    inserted: int = 0
    i: int = 0
    while i < len(queue):
        if inserted == 0 and value <= queue[i]:
            new_queue.append(value)
            inserted = 1
        new_queue.append(queue[i])
        i = i + 1
    if inserted == 0:
        new_queue.append(value)
    return new_queue


def pq_pop_min(queue: list[int]) -> int:
    if len(queue) == 0:
        return -1
    return queue[0]


def pq_remove_min(queue: list[int]) -> list[int]:
    if len(queue) == 0:
        return []
    result: list[int] = []
    i: int = 1
    while i < len(queue):
        result.append(queue[i])
        i = i + 1
    return result


def pq_pop_max(queue: list[int]) -> int:
    if len(queue) == 0:
        return -1
    return queue[len(queue) - 1]


def pq_remove_max(queue: list[int]) -> list[int]:
    if len(queue) == 0:
        return []
    result: list[int] = []
    i: int = 0
    while i < len(queue) - 1:
        result.append(queue[i])
        i = i + 1
    return result


def pq_size(queue: list[int]) -> int:
    return len(queue)


def test_module() -> int:
    passed: int = 0

    # Test 1: create empty queue
    q: list[int] = pq_create()
    if pq_size(q) == 0:
        passed = passed + 1

    # Test 2: insert and check min
    q = pq_insert(q, 5)
    q = pq_insert(q, 3)
    q = pq_insert(q, 7)
    if pq_pop_min(q) == 3:
        passed = passed + 1

    # Test 3: check max
    if pq_pop_max(q) == 7:
        passed = passed + 1

    # Test 4: remove min
    q = pq_remove_min(q)
    if pq_pop_min(q) == 5:
        passed = passed + 1

    # Test 5: remove max
    q = pq_remove_max(q)
    if pq_pop_max(q) == 5:
        passed = passed + 1

    # Test 6: size after operations
    if pq_size(q) == 1:
        passed = passed + 1

    # Test 7: insert maintains order
    q = pq_create()
    q = pq_insert(q, 10)
    q = pq_insert(q, 1)
    q = pq_insert(q, 5)
    q = pq_insert(q, 3)
    if pq_pop_min(q) == 1 and pq_pop_max(q) == 10:
        passed = passed + 1

    return passed
