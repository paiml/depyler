"""Reaction rate computations using integer arithmetic.

Tests: rate law, half-life, rate constant, order determination.
Scale factor 1000 for fixed-point.
"""


def first_order_rate(rate_constant: int, concentration: int) -> int:
    """First order rate: r = k * [A]. Scale 1000."""
    result: int = (rate_constant * concentration) // 1000
    return result


def second_order_rate(rate_constant: int, concentration: int) -> int:
    """Second order rate: r = k * [A]^2. Scale 1000."""
    c_sq: int = (concentration * concentration) // 1000
    result: int = (rate_constant * c_sq) // 1000
    return result


def zero_order_rate(rate_constant: int) -> int:
    """Zero order rate: r = k."""
    return rate_constant


def first_order_half_life(rate_constant: int) -> int:
    """Half-life for first order: t1/2 = ln(2)/k = 693/k. Scale 1000."""
    if rate_constant == 0:
        return 0
    result: int = (693 * 1000) // rate_constant
    return result


def second_order_half_life(rate_constant: int, initial_conc: int) -> int:
    """Half-life for second order: t1/2 = 1/(k*[A]0). Scale 1000."""
    denom: int = (rate_constant * initial_conc) // 1000
    if denom == 0:
        return 0
    result: int = (1000 * 1000) // denom
    return result


def concentration_first_order(c0: int, rate_constant: int, elapsed: int) -> int:
    """First order: [A] = [A]0 * exp(-k*t).
    exp(-x) ~ 1 - x + x^2/2. Scale 1000."""
    x: int = (rate_constant * elapsed) // 1000
    if x > 5000:
        return 0
    exp_val: int = 1000 - x + (x * x) // 2000
    if exp_val < 0:
        exp_val = 0
    result: int = (c0 * exp_val) // 1000
    return result


def concentration_zero_order(c0: int, rate_constant: int, elapsed: int) -> int:
    """Zero order: [A] = [A]0 - k*t. Scale 1000."""
    result: int = c0 - (rate_constant * elapsed) // 1000
    if result < 0:
        result = 0
    return result


def average_rate(c_initial: int, c_final: int, delta_time: int) -> int:
    """Average rate = -d[A]/dt = (c_initial - c_final)/dt. Scale 1000."""
    if delta_time == 0:
        return 0
    dc: int = c_initial - c_final
    result: int = (dc * 1000) // delta_time
    return result


def test_module() -> int:
    """Test reaction rate computations."""
    ok: int = 0
    r1: int = first_order_rate(100, 2000)
    if r1 == 200:
        ok = ok + 1
    r2: int = second_order_rate(100, 2000)
    if r2 == 400:
        ok = ok + 1
    hl: int = first_order_half_life(100)
    if hl == 6930:
        ok = ok + 1
    hl2: int = second_order_half_life(100, 1000)
    if hl2 == 10000:
        ok = ok + 1
    c_fo: int = concentration_first_order(1000, 0, 1000)
    if c_fo == 1000:
        ok = ok + 1
    c_zo: int = concentration_zero_order(1000, 100, 5000)
    if c_zo == 500:
        ok = ok + 1
    ar: int = average_rate(1000, 800, 2000)
    if ar == 100:
        ok = ok + 1
    return ok
