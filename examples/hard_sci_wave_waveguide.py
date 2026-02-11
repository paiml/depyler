"""Waveguide mode computations using integer arithmetic.

Tests: cutoff frequency, propagation constant, mode count, attenuation.
Scale factor 1000 for fixed-point.
"""


def cutoff_frequency_rect(mode_m: int, mode_n: int, width: int, height: int, speed: int) -> int:
    """Cutoff frequency for rectangular waveguide TE/TM mode.
    f_c = (c/2)*sqrt((m/a)^2 + (n/b)^2). Fixed-point scale 1000."""
    if width == 0 or height == 0:
        return 0
    term_m: int = (mode_m * 1000) // width
    term_n: int = (mode_n * 1000) // height
    sum_sq: int = (term_m * term_m + term_n * term_n) // 1000
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
            return (speed * next_g) // (2 * 1000)
        guess = next_g
        iterations = iterations + 1
    return (speed * guess) // (2 * 1000)


def cutoff_wavelength_rect(mode_m: int, mode_n: int, width: int, height: int) -> int:
    """Cutoff wavelength for rectangular waveguide.
    lambda_c = 2/sqrt((m/a)^2 + (n/b)^2). Fixed-point scale 1000."""
    if width == 0 or height == 0:
        return 0
    term_m: int = (mode_m * 1000) // width
    term_n: int = (mode_n * 1000) // height
    sum_sq: int = (term_m * term_m + term_n * term_n) // 1000
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
            if next_g == 0:
                return 0
            return (2 * 1000 * 1000) // next_g
        guess = next_g
        iterations = iterations + 1
    if guess == 0:
        return 0
    return (2 * 1000 * 1000) // guess


def propagation_constant(wavenumber: int, cutoff_wn: int) -> int:
    """Propagation constant beta = sqrt(k^2 - kc^2). Fixed-point scale 1000."""
    k_sq: int = (wavenumber * wavenumber) // 1000
    kc_sq: int = (cutoff_wn * cutoff_wn) // 1000
    diff: int = k_sq - kc_sq
    if diff <= 0:
        return 0
    guess: int = diff
    iterations: int = 0
    target: int = diff * 1000
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + target // guess) // 2
        d: int = next_g - guess
        if d < 0:
            d = 0 - d
        if d < 2:
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def is_propagating(frequency: int, cutoff_freq: int) -> int:
    """Returns 1 if mode propagates (f > fc), 0 otherwise."""
    if frequency > cutoff_freq:
        return 1
    return 0


def count_propagating_modes(frequency: int, width: int, height: int, speed: int) -> int:
    """Count number of propagating modes below given frequency."""
    count: int = 0
    m: int = 0
    while m <= 10:
        n: int = 0
        while n <= 10:
            if m == 0 and n == 0:
                n = n + 1
                continue
            fc: int = cutoff_frequency_rect(m, n, width, height, speed)
            if fc > 0 and frequency > fc:
                count = count + 1
            n = n + 1
        m = m + 1
    return count


def guide_wavelength(free_wavelength: int, cutoff_wavelength: int) -> int:
    """Guide wavelength = lambda / sqrt(1 - (lambda/lambda_c)^2).
    Fixed-point scale 1000."""
    if cutoff_wavelength == 0:
        return 0
    ratio: int = (free_wavelength * 1000) // cutoff_wavelength
    ratio_sq: int = (ratio * ratio) // 1000
    denom_sq: int = 1000 - ratio_sq
    if denom_sq <= 0:
        return 0
    guess: int = denom_sq
    iterations: int = 0
    target: int = denom_sq * 1000
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
            return (free_wavelength * 1000) // next_g
        guess = next_g
        iterations = iterations + 1
    if guess == 0:
        return 0
    return (free_wavelength * 1000) // guess


def test_module() -> int:
    """Test waveguide computations."""
    ok: int = 0
    prop1: int = is_propagating(5000, 3000)
    if prop1 == 1:
        ok = ok + 1
    prop2: int = is_propagating(2000, 3000)
    if prop2 == 0:
        ok = ok + 1
    beta: int = propagation_constant(5000, 3000)
    if beta > 3900 and beta < 4100:
        ok = ok + 1
    beta_zero: int = propagation_constant(3000, 5000)
    if beta_zero == 0:
        ok = ok + 1
    fc_zero: int = cutoff_frequency_rect(1, 0, 0, 1000, 300000)
    if fc_zero == 0:
        ok = ok + 1
    gw: int = guide_wavelength(500, 0)
    if gw == 0:
        ok = ok + 1
    return ok
