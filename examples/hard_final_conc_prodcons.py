"""Producer-consumer simulation with bounded buffer.

Simulates producer/consumer pattern using integer arrays.
Buffer is circular with head/tail pointers tracked explicitly.
"""


def create_buffer(capacity: int) -> list[int]:
    """Create a circular buffer with given capacity."""
    buf: list[int] = []
    i: int = 0
    while i < capacity:
        buf.append(0)
        i = i + 1
    return buf


def produce(buf: list[int], head: int, tail: int, capacity: int, item: int) -> list[int]:
    """Produce item into buffer. Returns [new_head, new_tail, success].

    success: 1 if produced, 0 if buffer full.
    """
    cur_size: int = tail - head
    if cur_size < 0:
        cur_size = cur_size + capacity
    if cur_size >= capacity - 1:
        return [head, tail, 0]
    buf[tail % capacity] = item
    new_tail: int = (tail + 1) % capacity
    return [head, new_tail, 1]


def consume(buf: list[int], head: int, tail: int, capacity: int) -> list[int]:
    """Consume item from buffer. Returns [new_head, new_tail, item, success]."""
    if head == tail:
        return [head, tail, 0, 0]
    item: int = buf[head % capacity]
    new_head: int = (head + 1) % capacity
    return [new_head, tail, item, 1]


def simulate_producers(buf: list[int], capacity: int, items: list[int]) -> list[int]:
    """Produce all items into buffer. Returns [head, tail, produced_count]."""
    head: int = 0
    tail: int = 0
    produced: int = 0
    i: int = 0
    while i < len(items):
        iv: int = items[i]
        result: list[int] = produce(buf, head, tail, capacity, iv)
        head = result[0]
        tail = result[1]
        success: int = result[2]
        if success == 1:
            produced = produced + 1
        i = i + 1
    return [head, tail, produced]


def simulate_consumers(buf: list[int], head: int, tail: int, capacity: int, count: int) -> list[int]:
    """Consume count items. Returns consumed items list."""
    consumed: list[int] = []
    i: int = 0
    while i < count:
        result: list[int] = consume(buf, head, tail, capacity)
        head = result[0]
        tail = result[1]
        item: int = result[2]
        success: int = result[3]
        if success == 1:
            consumed.append(item)
        i = i + 1
    return consumed


def buffer_size(head: int, tail: int, capacity: int) -> int:
    """Current number of items in circular buffer."""
    diff: int = tail - head
    if diff < 0:
        diff = diff + capacity
    return diff


def test_module() -> int:
    """Test producer-consumer."""
    ok: int = 0
    cap: int = 5
    buf: list[int] = create_buffer(cap)
    state: list[int] = simulate_producers(buf, cap, [10, 20, 30, 40])
    head: int = state[0]
    tail: int = state[1]
    produced: int = state[2]
    if produced == 4:
        ok = ok + 1
    sz: int = buffer_size(head, tail, cap)
    if sz == 4:
        ok = ok + 1
    consumed: list[int] = simulate_consumers(buf, head, tail, cap, 2)
    if len(consumed) == 2:
        ok = ok + 1
    c0: int = consumed[0]
    if c0 == 10:
        ok = ok + 1
    c1: int = consumed[1]
    if c1 == 20:
        ok = ok + 1
    return ok
