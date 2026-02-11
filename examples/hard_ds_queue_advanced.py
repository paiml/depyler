"""Circular queue and deque operations using flat lists.

Tests: circular enqueue/dequeue, deque push/pop front/back, full/empty checks.
"""


def cq_create(capacity: int) -> list[int]:
    """Create circular queue. Layout: [head, tail, size, capacity, ...data...]."""
    result: list[int] = [0, 0, 0, capacity]
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def cq_enqueue(cq: list[int], val: int) -> int:
    """Enqueue to circular queue. Returns 1 on success, 0 if full."""
    sz: int = cq[2]
    cap: int = cq[3]
    if sz == cap:
        return 0
    tail: int = cq[1]
    data_offset: int = 4 + tail
    cq[data_offset] = val
    cq[1] = (tail + 1) % cap
    cq[2] = sz + 1
    return 1


def cq_dequeue(cq: list[int]) -> int:
    """Dequeue from circular queue. Returns value or -1 if empty."""
    sz: int = cq[2]
    if sz == 0:
        return -1
    head: int = cq[0]
    data_offset: int = 4 + head
    val: int = cq[data_offset]
    cap: int = cq[3]
    cq[0] = (head + 1) % cap
    cq[2] = sz - 1
    return val


def cq_peek(cq: list[int]) -> int:
    """Peek at front without removing. Returns -1 if empty."""
    sz: int = cq[2]
    if sz == 0:
        return -1
    head: int = cq[0]
    data_offset: int = 4 + head
    return cq[data_offset]


def cq_size(cq: list[int]) -> int:
    """Return current size of circular queue."""
    return cq[2]


def cq_is_full(cq: list[int]) -> int:
    """Return 1 if full, 0 otherwise."""
    if cq[2] == cq[3]:
        return 1
    return 0


def deque_create() -> list[int]:
    """Create a deque using a list. Simple implementation."""
    return []


def deque_push_back(dq: list[int], val: int) -> int:
    """Push to back. Returns new size."""
    dq.append(val)
    return len(dq)


def deque_push_front(dq: list[int], val: int) -> int:
    """Push to front by shifting all elements. Returns new size."""
    sz: int = len(dq)
    dq.append(0)
    i: int = sz
    while i > 0:
        dq[i] = dq[i - 1]
        i = i - 1
    dq[0] = val
    return len(dq)


def deque_pop_back(dq: list[int]) -> int:
    """Pop from back. Returns value or -1 if empty."""
    sz: int = len(dq)
    if sz == 0:
        return -1
    return dq.pop()


def deque_pop_front(dq: list[int]) -> int:
    """Pop from front by shifting. Returns value or -1 if empty."""
    sz: int = len(dq)
    if sz == 0:
        return -1
    val: int = dq[0]
    i: int = 0
    while i < sz - 1:
        dq[i] = dq[i + 1]
        i = i + 1
    dq.pop()
    return val


def deque_peek_front(dq: list[int]) -> int:
    """Peek front. Returns -1 if empty."""
    if len(dq) == 0:
        return -1
    return dq[0]


def deque_peek_back(dq: list[int]) -> int:
    """Peek back. Returns -1 if empty."""
    sz: int = len(dq)
    if sz == 0:
        return -1
    return dq[sz - 1]


def test_module() -> int:
    """Test circular queue and deque."""
    passed: int = 0

    cq: list[int] = cq_create(3)
    cq_enqueue(cq, 10)
    cq_enqueue(cq, 20)
    cq_enqueue(cq, 30)

    if cq_is_full(cq) == 1:
        passed = passed + 1

    r1: int = cq_enqueue(cq, 40)
    if r1 == 0:
        passed = passed + 1

    v1: int = cq_dequeue(cq)
    if v1 == 10:
        passed = passed + 1

    cq_enqueue(cq, 40)
    if cq_peek(cq) == 20:
        passed = passed + 1

    if cq_size(cq) == 3:
        passed = passed + 1

    dq: list[int] = deque_create()
    deque_push_back(dq, 1)
    deque_push_back(dq, 2)
    deque_push_front(dq, 0)
    if deque_peek_front(dq) == 0:
        passed = passed + 1

    if deque_peek_back(dq) == 2:
        passed = passed + 1

    fv: int = deque_pop_front(dq)
    if fv == 0:
        passed = passed + 1

    bv: int = deque_pop_back(dq)
    if bv == 2:
        passed = passed + 1

    return passed
