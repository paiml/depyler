"""TCP-like congestion control simulation.

Implements slow start, congestion avoidance, and fast recovery
phases of TCP congestion control using cwnd and ssthresh.
"""


def cc_slow_start(cwnd: int, mss: int) -> int:
    """Exponential growth: double cwnd each RTT. Returns new cwnd."""
    return cwnd + mss


def cc_cong_avoid(cwnd: int, mss: int) -> int:
    """Additive increase: add mss/cwnd per ACK. Approximated as +1 per RTT."""
    increment: int = (mss * mss) // cwnd
    if increment < 1:
        increment = 1
    return cwnd + increment


def cc_on_loss(cwnd: int, ssthresh: int) -> list[int]:
    """Handle packet loss. Returns [new_cwnd, new_ssthresh]."""
    new_ss: int = cwnd // 2
    if new_ss < 2:
        new_ss = 2
    result: list[int] = [2, new_ss]
    return result


def cc_on_fast_retransmit(cwnd: int) -> list[int]:
    """Handle triple duplicate ACK. Returns [new_cwnd, new_ssthresh]."""
    new_ss: int = cwnd // 2
    if new_ss < 2:
        new_ss = 2
    new_cwnd: int = new_ss + 3
    result: list[int] = [new_cwnd, new_ss]
    return result


def cc_simulate(initial_cwnd: int, mss: int, ssthresh: int,
                events: list[int], num_events: int) -> list[int]:
    """Simulate congestion control.
    events: 0=ack, 1=loss, 2=triple_dup_ack.
    Returns [final_cwnd, final_ssthresh, max_cwnd_reached]."""
    cwnd: int = initial_cwnd
    ss: int = ssthresh
    max_cwnd: int = cwnd
    i: int = 0
    while i < num_events:
        ev: int = events[i]
        if ev == 0:
            if cwnd < ss:
                cwnd = cc_slow_start(cwnd, mss)
            else:
                cwnd = cc_cong_avoid(cwnd, mss)
        if ev == 1:
            pair: list[int] = cc_on_loss(cwnd, ss)
            cwnd = pair[0]
            ss = pair[1]
        if ev == 2:
            pair2: list[int] = cc_on_fast_retransmit(cwnd)
            cwnd = pair2[0]
            ss = pair2[1]
        if cwnd > max_cwnd:
            max_cwnd = cwnd
        i = i + 1
    result: list[int] = [cwnd, ss, max_cwnd]
    return result


def cc_throughput(cwnd: int, rtt: int) -> int:
    """Estimate throughput as cwnd/rtt."""
    if rtt == 0:
        return 0
    return cwnd // rtt


def test_module() -> int:
    """Test congestion control simulation."""
    passed: int = 0

    # Test 1: slow start doubles cwnd
    c1: int = cc_slow_start(4, 2)
    if c1 == 6:
        passed = passed + 1

    # Test 2: congestion avoidance is additive
    c2: int = cc_cong_avoid(16, 4)
    if c2 == 17:
        passed = passed + 1

    # Test 3: loss halves ssthresh and resets cwnd
    pair: list[int] = cc_on_loss(20, 10)
    nc: int = pair[0]
    ns: int = pair[1]
    if nc == 2:
        if ns == 10:
            passed = passed + 1

    # Test 4: simulate growth then loss
    events: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0]
    result: list[int] = cc_simulate(2, 2, 16, events, 11)
    final_cwnd: int = result[0]
    max_cwnd: int = result[2]
    if max_cwnd > final_cwnd:
        passed = passed + 1

    # Test 5: throughput calculation
    tp: int = cc_throughput(100, 10)
    if tp == 10:
        passed = passed + 1

    return passed
