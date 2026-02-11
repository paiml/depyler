"""Electrical power computations using integer arithmetic.

Tests: real power, reactive power, apparent power, power factor.
Scale factor 1000 for fixed-point.
"""


def real_power(voltage_rms: int, current_rms: int, cos_phi: int) -> int:
    """Real power P = V*I*cos(phi). Scale 1000."""
    result: int = (voltage_rms * current_rms * cos_phi) // (1000 * 1000)
    return result


def reactive_power(voltage_rms: int, current_rms: int, sin_phi: int) -> int:
    """Reactive power Q = V*I*sin(phi). Scale 1000."""
    result: int = (voltage_rms * current_rms * sin_phi) // (1000 * 1000)
    return result


def apparent_power(voltage_rms: int, current_rms: int) -> int:
    """Apparent power S = V*I. Scale 1000."""
    result: int = (voltage_rms * current_rms) // 1000
    return result


def power_factor(real_p: int, apparent_p: int) -> int:
    """Power factor = P/S. Scale 1000."""
    if apparent_p == 0:
        return 0
    result: int = (real_p * 1000) // apparent_p
    return result


def power_triangle(real_p: int, reactive_p: int) -> int:
    """Apparent power from power triangle: S = sqrt(P^2 + Q^2). Scale 1000."""
    p_sq: int = (real_p * real_p) // 1000
    q_sq: int = (reactive_p * reactive_p) // 1000
    sum_sq: int = p_sq + q_sq
    if sum_sq <= 0:
        return 0
    guess: int = sum_sq
    iterations: int = 0
    target: int = sum_sq * 1000
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def three_phase_power(v_line: int, i_line: int, cos_phi: int) -> int:
    """Three-phase power: P = sqrt(3)*V*I*cos(phi).
    sqrt(3) ~ 1732. Scale 1000."""
    result: int = (1732 * v_line * i_line * cos_phi) // (1000 * 1000 * 1000)
    return result


def energy_consumed(power_watts: int, hours: int) -> int:
    """Energy in watt-hours: E = P * t. Scale 1000."""
    result: int = (power_watts * hours) // 1000
    return result


def transmission_loss(current_val: int, resistance: int) -> int:
    """Transmission line loss: P_loss = I^2 * R. Scale 1000."""
    i_sq: int = (current_val * current_val) // 1000
    result: int = (i_sq * resistance) // 1000
    return result


def efficiency_power(output_p: int, input_p: int) -> int:
    """Power efficiency = P_out / P_in * 1000. Scale 1000."""
    if input_p == 0:
        return 0
    result: int = (output_p * 1000) // input_p
    return result


def test_module() -> int:
    """Test power computations."""
    ok: int = 0
    rp: int = real_power(230000, 10000, 800)
    if rp > 1830000 and rp < 1850000:
        ok = ok + 1
    ap: int = apparent_power(230000, 10000)
    if ap == 2300000:
        ok = ok + 1
    pf: int = power_factor(1840000, 2300000)
    if pf == 800:
        ok = ok + 1
    pt: int = power_triangle(3000, 4000)
    if pt > 4990 and pt < 5010:
        ok = ok + 1
    ec: int = energy_consumed(1000, 5000)
    if ec == 5000:
        ok = ok + 1
    eff: int = efficiency_power(900, 1000)
    if eff == 900:
        ok = ok + 1
    return ok
