"""Sliding window protocol simulation.

Implements sender-side sliding window for reliable data transfer
with sequence numbers, acknowledgments, and window management.
"""


def sw_init(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def sw_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def sw_send(buf: list[int], acked: list[int], window_size: int,
            send_base: int, next_seq: int, data: int, buf_size: int) -> int:
    """Send a packet if window allows. Returns new next_seq, or -1 if window full."""
    if next_seq - send_base >= window_size:
        return 0 - 1
    idx: int = next_seq % buf_size
    buf[idx] = data
    acked[idx] = 0
    return next_seq + 1


def sw_ack(acked: list[int], seq: int, buf_size: int) -> int:
    """Mark packet as acknowledged. Returns 1."""
    idx: int = seq % buf_size
    acked[idx] = 1
    return 1


def sw_advance_base(acked: list[int], send_base: int,
                    next_seq: int, buf_size: int) -> int:
    """Advance send_base past consecutive acked packets. Returns new send_base."""
    sb: int = send_base
    while sb < next_seq:
        idx: int = sb % buf_size
        a: int = acked[idx]
        if a == 1:
            sb = sb + 1
        else:
            return sb
    return sb


def sw_window_used(send_base: int, next_seq: int) -> int:
    """How many slots in use."""
    return next_seq - send_base


def sw_window_free(send_base: int, next_seq: int, window_size: int) -> int:
    """How many free slots in window."""
    return window_size - (next_seq - send_base)


def sw_count_unacked(acked: list[int], send_base: int,
                     next_seq: int, buf_size: int) -> int:
    """Count unacknowledged packets in window."""
    count: int = 0
    i: int = send_base
    while i < next_seq:
        idx: int = i % buf_size
        a: int = acked[idx]
        if a == 0:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test sliding window protocol."""
    passed: int = 0
    buf_size: int = 8
    win_size: int = 4
    buf: list[int] = sw_init(buf_size)
    acked: list[int] = sw_init_zeros(buf_size)
    send_base: int = 0
    next_seq: int = 0

    # Test 1: send within window
    next_seq = sw_send(buf, acked, win_size, send_base, next_seq, 100, buf_size)
    next_seq = sw_send(buf, acked, win_size, send_base, next_seq, 200, buf_size)
    if next_seq == 2:
        passed = passed + 1

    # Test 2: window usage tracking
    used: int = sw_window_used(send_base, next_seq)
    if used == 2:
        passed = passed + 1

    # Test 3: ack and advance
    sw_ack(acked, 0, buf_size)
    send_base = sw_advance_base(acked, send_base, next_seq, buf_size)
    if send_base == 1:
        passed = passed + 1

    # Test 4: fill window
    next_seq = sw_send(buf, acked, win_size, send_base, next_seq, 300, buf_size)
    next_seq = sw_send(buf, acked, win_size, send_base, next_seq, 400, buf_size)
    next_seq = sw_send(buf, acked, win_size, send_base, next_seq, 500, buf_size)
    free: int = sw_window_free(send_base, next_seq, win_size)
    if free == 0:
        passed = passed + 1

    # Test 5: window full rejects send
    reject: int = sw_send(buf, acked, win_size, send_base, next_seq, 600, buf_size)
    if reject == (0 - 1):
        passed = passed + 1

    return passed
