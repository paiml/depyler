# Circular queue with wrap-around using fixed-size array


def cq_create(capacity: int) -> list[int]:
    # Layout: [capacity, head, tail, count, ...slots...]
    result: list[int] = [capacity, 0, 0, 0]
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def cq_enqueue(cq: list[int], value: int) -> int:
    capacity: int = cq[0]
    count: int = cq[3]
    if count >= capacity:
        return 0
    tail: int = cq[2]
    cq[4 + tail] = value
    cq[2] = (tail + 1) % capacity
    cq[3] = count + 1
    return 1


def cq_dequeue(cq: list[int]) -> int:
    count: int = cq[3]
    if count == 0:
        return -1
    head: int = cq[1]
    capacity: int = cq[0]
    value: int = cq[4 + head]
    cq[1] = (head + 1) % capacity
    cq[3] = count - 1
    return value


def cq_peek(cq: list[int]) -> int:
    if cq[3] == 0:
        return -1
    return cq[4 + cq[1]]


def cq_count(cq: list[int]) -> int:
    return cq[3]


def cq_is_full(cq: list[int]) -> int:
    if cq[3] == cq[0]:
        return 1
    return 0


def test_module() -> int:
    passed: int = 0

    # Test 1: create and check empty
    cq: list[int] = cq_create(4)
    if cq_count(cq) == 0:
        passed = passed + 1

    # Test 2: enqueue
    cq_enqueue(cq, 10)
    cq_enqueue(cq, 20)
    if cq_count(cq) == 2:
        passed = passed + 1

    # Test 3: peek
    if cq_peek(cq) == 10:
        passed = passed + 1

    # Test 4: dequeue
    val: int = cq_dequeue(cq)
    if val == 10:
        passed = passed + 1

    # Test 5: wrap-around
    cq_enqueue(cq, 30)
    cq_enqueue(cq, 40)
    cq_enqueue(cq, 50)
    if cq_is_full(cq) == 1:
        passed = passed + 1

    # Test 6: dequeue after wrap
    v1: int = cq_dequeue(cq)
    if v1 == 20:
        passed = passed + 1

    # Test 7: enqueue after dequeue wraps tail
    cq_enqueue(cq, 60)
    if cq_count(cq) == 4 and cq_is_full(cq) == 1:
        passed = passed + 1

    # Test 8: full queue rejects
    result: int = cq_enqueue(cq, 99)
    if result == 0:
        passed = passed + 1

    return passed
