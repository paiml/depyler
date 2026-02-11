"""Flow control simulation.

Implements token bucket and leaky bucket algorithms for
rate limiting and traffic shaping.
"""


def tb_add_tokens(bucket: int, rate: int, capacity: int, elapsed: int) -> int:
    """Add tokens based on elapsed time. Returns new bucket level."""
    added: int = rate * elapsed
    new_level: int = bucket + added
    if new_level > capacity:
        new_level = capacity
    return new_level


def tb_consume(bucket: int, tokens_needed: int) -> int:
    """Try to consume tokens. Returns new bucket level or -1 if insufficient."""
    if bucket < tokens_needed:
        return 0 - 1
    return bucket - tokens_needed


def tb_can_send(bucket: int, packet_size: int) -> int:
    """Check if packet can be sent. Returns 1 if yes."""
    if bucket >= packet_size:
        return 1
    return 0


def lb_process(queue_level: int, drain_rate: int, incoming: int,
               capacity: int) -> list[int]:
    """Leaky bucket: add incoming, drain at fixed rate.
    Returns [new_level, dropped]."""
    drained: int = queue_level - drain_rate
    if drained < 0:
        drained = 0
    new_level: int = drained + incoming
    dropped: int = 0
    if new_level > capacity:
        dropped = new_level - capacity
        new_level = capacity
    result: list[int] = [new_level, dropped]
    return result


def lb_simulate(drain_rate: int, capacity: int,
                arrivals: list[int], num_steps: int) -> list[int]:
    """Simulate leaky bucket. Returns [total_passed, total_dropped]."""
    level: int = 0
    total_passed: int = 0
    total_dropped: int = 0
    i: int = 0
    while i < num_steps:
        incoming: int = arrivals[i]
        pair: list[int] = lb_process(level, drain_rate, incoming, capacity)
        level = pair[0]
        dropped: int = pair[1]
        total_dropped = total_dropped + dropped
        passed: int = incoming - dropped
        total_passed = total_passed + passed
        i = i + 1
    result: list[int] = [total_passed, total_dropped]
    return result


def fc_weighted_fair(weights: list[int], bandwidth: int, count: int,
                     alloc: list[int]) -> int:
    """Allocate bandwidth proportionally to weights. Returns 0."""
    total_w: int = 0
    i: int = 0
    while i < count:
        w: int = weights[i]
        total_w = total_w + w
        i = i + 1
    if total_w == 0:
        return 0
    j: int = 0
    while j < count:
        w2: int = weights[j]
        alloc[j] = (w2 * bandwidth) // total_w
        j = j + 1
    return 0


def test_module() -> int:
    """Test flow control algorithms."""
    passed: int = 0

    # Test 1: token bucket add and consume
    bucket: int = tb_add_tokens(0, 10, 100, 5)
    if bucket == 50:
        passed = passed + 1

    # Test 2: token consumption
    after: int = tb_consume(bucket, 30)
    if after == 20:
        passed = passed + 1

    # Test 3: insufficient tokens
    fail: int = tb_consume(after, 50)
    if fail == (0 - 1):
        passed = passed + 1

    # Test 4: leaky bucket drops excess
    arrivals: list[int] = [10, 20, 30, 5, 50]
    stats: list[int] = lb_simulate(10, 30, arrivals, 5)
    total_p: int = stats[0]
    total_d: int = stats[1]
    sum_arr: int = total_p + total_d
    if sum_arr == 115:
        passed = passed + 1

    # Test 5: weighted fair allocation
    weights: list[int] = [50, 30, 20]
    alloc: list[int] = [0, 0, 0]
    fc_weighted_fair(weights, 1000, 3, alloc)
    a0: int = alloc[0]
    if a0 == 500:
        passed = passed + 1

    return passed
