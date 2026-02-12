"""Systems: Ring buffer implementation.
Tests: circular indexing, overflow handling, capacity management.
"""
from typing import Dict, List, Tuple


def ring_write(buf: List[int], pos: int, val: int) -> int:
    """Write value at position in ring buffer, return next position."""
    buf[pos] = val
    return pos


def ring_next(pos: int, capacity: int) -> int:
    """Get next position in ring buffer."""
    return (pos + 1) % capacity


def ring_prev(pos: int, capacity: int) -> int:
    """Get previous position in ring buffer."""
    np: int = pos - 1
    if np < 0:
        np = capacity - 1
    return np


def ring_index(head: int, offset: int, capacity: int) -> int:
    """Get absolute index from head + offset."""
    return (head + offset) % capacity


def ring_fill(capacity: int, fill_val: int) -> List[int]:
    """Create a ring buffer of given capacity filled with fill_val."""
    buf: List[int] = []
    i: int = 0
    while i < capacity:
        buf.append(fill_val)
        i += 1
    return buf


def ring_push_state(head: int, count: int, capacity: int) -> Tuple[int, int, int]:
    """Compute new state after push. Returns (write_pos, new_head, new_count)."""
    write_pos: int = (head + count) % capacity
    if count < capacity:
        return (write_pos, head, count + 1)
    new_head: int = (head + 1) % capacity
    return (write_pos, new_head, count)


def ring_pop_state(head: int, count: int, capacity: int) -> Tuple[int, int, int]:
    """Compute new state after pop. Returns (read_pos, new_head, new_count)."""
    if count == 0:
        return (-1, head, 0)
    read_pos: int = head
    new_head: int = (head + 1) % capacity
    new_count: int = count - 1
    return (read_pos, new_head, new_count)


def ring_sum(buf: List[int], head: int, count: int, capacity: int) -> int:
    """Sum all elements in ring buffer."""
    total: int = 0
    i: int = 0
    while i < count:
        idx: int = (head + i) % capacity
        total += buf[idx]
        i += 1
    return total


def ring_max(buf: List[int], head: int, count: int, capacity: int) -> int:
    """Find maximum element in ring buffer."""
    if count == 0:
        return -1
    best: int = buf[head]
    i: int = 1
    while i < count:
        idx: int = (head + i) % capacity
        if buf[idx] > best:
            best = buf[idx]
        i += 1
    return best


def test_ring_buffer() -> bool:
    ok: bool = True
    capacity: int = 4
    buf: List[int] = ring_fill(capacity, 0)
    head: int = 0
    count: int = 0

    s1: Tuple[int, int, int] = ring_push_state(head, count, capacity)
    buf[s1[0]] = 10
    head = s1[1]
    count = s1[2]
    if count != 1:
        ok = False

    s2: Tuple[int, int, int] = ring_push_state(head, count, capacity)
    buf[s2[0]] = 20
    head = s2[1]
    count = s2[2]
    if count != 2:
        ok = False

    s3: Tuple[int, int, int] = ring_push_state(head, count, capacity)
    buf[s3[0]] = 30
    head = s3[1]
    count = s3[2]

    total: int = ring_sum(buf, head, count, capacity)
    if total != 60:
        ok = False

    mx: int = ring_max(buf, head, count, capacity)
    if mx != 30:
        ok = False

    p1: Tuple[int, int, int] = ring_pop_state(head, count, capacity)
    popped: int = buf[p1[0]]
    head = p1[1]
    count = p1[2]
    if popped != 10:
        ok = False
    if count != 2:
        ok = False

    return ok
