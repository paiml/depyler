"""RLC circuit computations using integer arithmetic.

Tests: resonant frequency, Q factor, impedance, damping.
Scale factor 1000 for fixed-point.
"""


def resonant_frequency_rlc(inductance: int, capacitance: int) -> int:
    """Resonant frequency: f0 = 1/(2*pi*sqrt(L*C)).
    2*pi ~ 6283. Scale 1000."""
    product: int = (inductance * capacitance) // 1000
    if product <= 0:
        return 0
    guess: int = product
    iterations: int = 0
    target: int = product * 1000
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            if next_g == 0:
                return 0
            return (1000 * 1000) // (6283 * next_g // 1000)
        guess = next_g
        iterations = iterations + 1
    if guess == 0:
        return 0
    return (1000 * 1000) // (6283 * guess // 1000)


def rlc_impedance(resistance: int, x_l: int, x_c: int) -> int:
    """RLC impedance magnitude: Z = sqrt(R^2 + (XL-XC)^2). Scale 1000."""
    x_diff: int = x_l - x_c
    r_sq: int = (resistance * resistance) // 1000
    x_sq: int = (x_diff * x_diff) // 1000
    sum_sq: int = r_sq + x_sq
    if sum_sq <= 0:
        return resistance
    guess: int = sum_sq
    iterations: int = 0
    target: int = sum_sq * 1000
    while iterations < 50:
        if guess == 0:
            return resistance
        next_g: int = (guess + target // guess) // 2
        d: int = next_g - guess
        if d < 0:
            d = 0 - d
        if d < 2:
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def quality_factor_rlc(inductance: int, capacitance: int, resistance: int) -> int:
    """Q factor = (1/R)*sqrt(L/C). Scale 1000."""
    if resistance == 0 or capacitance == 0:
        return 0
    ratio: int = (inductance * 1000) // capacitance
    if ratio <= 0:
        return 0
    guess: int = ratio
    iterations: int = 0
    target: int = ratio * 1000
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return (next_g * 1000) // resistance
        guess = next_g
        iterations = iterations + 1
    return (guess * 1000) // resistance


def damping_ratio_rlc(resistance: int, inductance: int, capacitance: int) -> int:
    """Damping ratio: zeta = (R/2)*sqrt(C/L). Scale 1000."""
    if inductance == 0:
        return 0
    ratio: int = (capacitance * 1000) // inductance
    if ratio <= 0:
        return 0
    guess: int = ratio
    iterations: int = 0
    target: int = ratio * 1000
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return (resistance * next_g) // 2000
        guess = next_g
        iterations = iterations + 1
    return (resistance * guess) // 2000


def bandwidth_rlc(resistance: int, inductance: int) -> int:
    """Bandwidth BW = R/L. Scale 1000."""
    if inductance == 0:
        return 0
    result: int = (resistance * 1000) // inductance
    return result


def is_underdamped(zeta: int) -> int:
    """Returns 1 if underdamped (zeta < 1), 0 otherwise."""
    if zeta < 1000:
        return 1
    return 0


def is_overdamped(zeta: int) -> int:
    """Returns 1 if overdamped (zeta > 1), 0 otherwise."""
    if zeta > 1000:
        return 1
    return 0


def is_critically_damped(zeta: int) -> int:
    """Returns 1 if critically damped (zeta == 1), 0 otherwise."""
    if zeta == 1000:
        return 1
    return 0


def test_module() -> int:
    """Test RLC circuit computations."""
    ok: int = 0
    z_at_res: int = rlc_impedance(1000, 5000, 5000)
    if z_at_res == 1000:
        ok = ok + 1
    bw: int = bandwidth_rlc(100, 1000)
    if bw == 100:
        ok = ok + 1
    ud: int = is_underdamped(500)
    if ud == 1:
        ok = ok + 1
    od: int = is_overdamped(1500)
    if od == 1:
        ok = ok + 1
    cd: int = is_critically_damped(1000)
    if cd == 1:
        ok = ok + 1
    bw_zero: int = bandwidth_rlc(100, 0)
    if bw_zero == 0:
        ok = ok + 1
    return ok
