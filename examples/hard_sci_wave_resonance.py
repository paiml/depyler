"""Resonance frequency computations using integer arithmetic.

Tests: natural frequency, Q factor, bandwidth, resonance curves.
Scale factor 1000 for fixed-point.
"""


def natural_frequency(stiffness: int, mass: int) -> int:
    """Natural frequency omega_n = sqrt(k/m). Fixed-point scale 1000.
    Uses integer sqrt via Newton's method."""
    if mass == 0 or stiffness <= 0:
        return 0
    ratio: int = (stiffness * 1000) // mass
    guess: int = ratio
    iterations: int = 0
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + (ratio * 1000) // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def quality_factor(omega_n: int, damping: int) -> int:
    """Q factor = omega_n / (2 * damping). Fixed-point scale 1000."""
    if damping == 0:
        return 0
    result: int = (omega_n * 1000) // (2 * damping)
    return result


def bandwidth(omega_n: int, q_factor: int) -> int:
    """Bandwidth = omega_n / Q. Fixed-point scale 1000."""
    if q_factor == 0:
        return 0
    result: int = (omega_n * 1000) // q_factor
    return result


def damped_frequency(omega_n: int, damping_ratio: int) -> int:
    """Damped natural frequency omega_d = omega_n * sqrt(1 - zeta^2).
    damping_ratio is zeta*1000. Fixed-point scale 1000."""
    zeta_sq: int = (damping_ratio * damping_ratio) // 1000
    one_minus: int = 1000 - zeta_sq
    if one_minus <= 0:
        return 0
    product: int = (omega_n * omega_n * one_minus) // (1000 * 1000)
    if product <= 0:
        return 0
    guess: int = omega_n
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
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def resonance_amplitude(static_amp: int, freq_ratio: int, damping_ratio: int) -> int:
    """Amplitude at resonance: A = A0 / sqrt((1-r^2)^2 + (2*zeta*r)^2).
    freq_ratio = omega/omega_n * 1000, damping_ratio = zeta*1000.
    Fixed-point scale 1000."""
    r_sq: int = (freq_ratio * freq_ratio) // 1000
    one_minus_rsq: int = 1000 - r_sq
    term1: int = (one_minus_rsq * one_minus_rsq) // 1000
    zeta_r: int = (2 * damping_ratio * freq_ratio) // 1000
    term2: int = (zeta_r * zeta_r) // 1000
    denom_sq: int = term1 + term2
    if denom_sq <= 0:
        return static_amp * 100
    guess: int = denom_sq
    iterations: int = 0
    target: int = denom_sq * 1000
    while iterations < 50:
        if guess == 0:
            return static_amp * 100
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            if next_g == 0:
                return static_amp * 100
            result: int = (static_amp * 1000) // next_g
            return result
        guess = next_g
        iterations = iterations + 1
    if guess == 0:
        return static_amp * 100
    result2: int = (static_amp * 1000) // guess
    return result2


def half_power_frequencies(omega_n: int, q_factor: int) -> list[int]:
    """Half-power frequencies omega_1 and omega_2.
    omega_1 = omega_n * (1 - 1/(2Q)), omega_2 = omega_n * (1 + 1/(2Q)).
    Fixed-point scale 1000. Returns [omega_1, omega_2]."""
    if q_factor == 0:
        return [0, 0]
    half_bw: int = (omega_n * 500) // q_factor
    omega_1: int = omega_n - half_bw
    omega_2: int = omega_n + half_bw
    return [omega_1, omega_2]


def test_module() -> int:
    """Test resonance computations."""
    ok: int = 0
    q: int = quality_factor(1000, 100)
    if q == 5000:
        ok = ok + 1
    bw: int = bandwidth(1000, 5000)
    if bw == 200:
        ok = ok + 1
    hp: list[int] = half_power_frequencies(1000, 5000)
    hp0: int = hp[0]
    hp1: int = hp[1]
    if hp0 == 900 and hp1 == 1100:
        ok = ok + 1
    q_zero: int = quality_factor(1000, 0)
    if q_zero == 0:
        ok = ok + 1
    bw_zero: int = bandwidth(1000, 0)
    if bw_zero == 0:
        ok = ok + 1
    nf_zero: int = natural_frequency(0, 1000)
    if nf_zero == 0:
        ok = ok + 1
    return ok
