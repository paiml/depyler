"""Ring buffer simulation: fixed-size circular buffer operations."""


def ring_buffer_push(buf: list[int], capacity: int, head: int, size: int, value: int) -> list[int]:
    """Push a value into ring buffer. Returns [new_head, new_size]."""
    write_pos: int = (head + size) % capacity
    if size < capacity:
        buf[write_pos] = value
        result: list[int] = [head, size + 1]
        return result
    else:
        buf[head] = value
        new_head: int = (head + 1) % capacity
        result2: list[int] = [new_head, size]
        return result2


def ring_buffer_pop(buf: list[int], head: int, size: int, capacity: int) -> list[int]:
    """Pop oldest value from ring buffer. Returns [value, new_head, new_size]."""
    if size == 0:
        result: list[int] = [-1, head, 0]
        return result
    val: int = buf[head]
    new_head: int = (head + 1) % capacity
    result2: list[int] = [val, new_head, size - 1]
    return result2


def ring_buffer_read(buf: list[int], head: int, size: int, capacity: int, index: int) -> int:
    """Read element at logical index from ring buffer."""
    if index < 0 or index >= size:
        return -1
    physical: int = (head + index) % capacity
    return buf[physical]


def ring_buffer_sum(buf: list[int], head: int, size: int, capacity: int) -> int:
    """Sum all elements in the ring buffer."""
    total: int = 0
    i: int = 0
    while i < size:
        physical: int = (head + i) % capacity
        total = total + buf[physical]
        i = i + 1
    return total


def test_module() -> int:
    """Test ring buffer operations."""
    ok: int = 0

    cap: int = 4
    buf: list[int] = [0, 0, 0, 0]
    head: int = 0
    size: int = 0

    # Push 10, 20, 30
    state: list[int] = ring_buffer_push(buf, cap, head, size, 10)
    head = state[0]
    size = state[1]
    state = ring_buffer_push(buf, cap, head, size, 20)
    head = state[0]
    size = state[1]
    state = ring_buffer_push(buf, cap, head, size, 30)
    head = state[0]
    size = state[1]

    if size == 3:
        ok = ok + 1

    if ring_buffer_read(buf, head, size, cap, 0) == 10:
        ok = ok + 1

    if ring_buffer_read(buf, head, size, cap, 2) == 30:
        ok = ok + 1

    if ring_buffer_sum(buf, head, size, cap) == 60:
        ok = ok + 1

    pop_result: list[int] = ring_buffer_pop(buf, head, size, cap)
    if pop_result[0] == 10:
        ok = ok + 1

    # Out of range read
    if ring_buffer_read(buf, head, size, cap, 10) == -1:
        ok = ok + 1

    return ok
