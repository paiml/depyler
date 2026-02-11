"""Circular buffer operations using a flat list."""


def make_buffer(capacity: int) -> list[int]:
    """Create circular buffer: [capacity, head, tail, count, ...data...]."""
    buf: list[int] = []
    buf.append(capacity)
    buf.append(0)
    buf.append(0)
    buf.append(0)
    i: int = 0
    while i < capacity:
        buf.append(0)
        i = i + 1
    return buf


def buf_push(buf: list[int], val: int) -> int:
    """Push value into circular buffer. Returns 1 on success, 0 if full."""
    capacity: int = buf[0]
    count: int = buf[3]
    if count >= capacity:
        return 0
    tail: int = buf[2]
    buf[4 + tail] = val
    buf[2] = (tail + 1) % capacity
    buf[3] = count + 1
    return 1


def buf_pop(buf: list[int]) -> int:
    """Pop value from circular buffer. Returns value or -1 if empty."""
    count: int = buf[3]
    if count == 0:
        return 0 - 1
    head: int = buf[1]
    val: int = buf[4 + head]
    capacity: int = buf[0]
    buf[1] = (head + 1) % capacity
    buf[3] = count - 1
    return val


def buf_peek(buf: list[int]) -> int:
    """Peek at front element. Returns -1 if empty."""
    count: int = buf[3]
    if count == 0:
        return 0 - 1
    head: int = buf[1]
    return buf[4 + head]


def buf_size(buf: list[int]) -> int:
    """Return current count of elements."""
    return buf[3]


def buf_is_full(buf: list[int]) -> int:
    """Returns 1 if buffer is full."""
    if buf[3] >= buf[0]:
        return 1
    return 0


def test_module() -> int:
    """Test circular buffer operations."""
    ok: int = 0
    buf: list[int] = make_buffer(3)
    if buf_size(buf) == 0:
        ok = ok + 1
    buf_push(buf, 10)
    buf_push(buf, 20)
    buf_push(buf, 30)
    if buf_size(buf) == 3:
        ok = ok + 1
    if buf_is_full(buf) == 1:
        ok = ok + 1
    if buf_push(buf, 40) == 0:
        ok = ok + 1
    if buf_peek(buf) == 10:
        ok = ok + 1
    v1: int = buf_pop(buf)
    if v1 == 10:
        ok = ok + 1
    buf_push(buf, 40)
    v2: int = buf_pop(buf)
    if v2 == 20:
        ok = ok + 1
    return ok
