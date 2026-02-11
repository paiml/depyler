"""Wave pulse propagation computations using integer arithmetic.

Tests: Gaussian pulse, pulse velocity, pulse broadening, reflection.
Scale factor 1000 for fixed-point.
"""


def gaussian_pulse(position: int, center: int, width: int, amplitude: int) -> int:
    """Gaussian pulse: A * exp(-(x-x0)^2 / (2*sigma^2)).
    exp approx: exp(-u) ~ 1000 - u + u^2/2 for small u.
    Fixed-point scale 1000."""
    if width == 0:
        return 0
    diff: int = position - center
    u_num: int = diff * diff
    u_den: int = 2 * width * width
    if u_den == 0:
        return 0
    u: int = (u_num * 1000) // u_den
    if u > 5000:
        return 0
    exp_val: int = 1000 - u + (u * u) // 2000
    if exp_val < 0:
        exp_val = 0
    if exp_val > 1000:
        exp_val = 1000
    result: int = (amplitude * exp_val) // 1000
    return result


def pulse_position(initial_pos: int, velocity: int, elapsed: int) -> int:
    """Position of pulse center after time t. Fixed-point scale 1000."""
    result: int = initial_pos + (velocity * elapsed) // 1000
    return result


def pulse_energy(amplitude: int, width: int) -> int:
    """Energy of Gaussian pulse proportional to A^2 * sigma * sqrt(pi).
    sqrt(pi) ~ 1772/1000. Fixed-point scale 1000."""
    a_sq: int = (amplitude * amplitude) // 1000
    result: int = (a_sq * width * 1772) // (1000 * 1000)
    return result


def reflected_amplitude(incident: int, impedance1: int, impedance2: int) -> int:
    """Reflected amplitude: A_r = A_i * (Z2 - Z1) / (Z2 + Z1).
    Fixed-point scale 1000."""
    denom: int = impedance2 + impedance1
    if denom == 0:
        return 0
    numer: int = impedance2 - impedance1
    result: int = (incident * numer) // denom
    return result


def transmitted_amplitude(incident: int, impedance1: int, impedance2: int) -> int:
    """Transmitted amplitude: A_t = A_i * 2*Z2 / (Z2 + Z1).
    Fixed-point scale 1000."""
    denom: int = impedance2 + impedance1
    if denom == 0:
        return 0
    result: int = (incident * 2 * impedance2) // denom
    return result


def reflection_coefficient(impedance1: int, impedance2: int) -> int:
    """Reflection coefficient R = (Z2-Z1)/(Z2+Z1). Fixed-point scale 1000."""
    denom: int = impedance2 + impedance1
    if denom == 0:
        return 0
    numer: int = impedance2 - impedance1
    result: int = (numer * 1000) // denom
    return result


def transmission_coefficient(impedance1: int, impedance2: int) -> int:
    """Transmission coefficient T = 2*Z2/(Z2+Z1). Fixed-point scale 1000."""
    denom: int = impedance2 + impedance1
    if denom == 0:
        return 0
    result: int = (2 * impedance2 * 1000) // denom
    return result


def pulse_broadening(initial_width: int, dispersion: int, distance: int) -> int:
    """Pulse broadening due to dispersion.
    sigma(z) = sqrt(sigma0^2 + (D*z)^2). Fixed-point scale 1000."""
    w_sq: int = (initial_width * initial_width) // 1000
    dz: int = (dispersion * distance) // 1000
    dz_sq: int = (dz * dz) // 1000
    total_sq: int = w_sq + dz_sq
    if total_sq <= 0:
        return initial_width
    guess: int = total_sq
    iterations: int = 0
    target: int = total_sq * 1000
    while iterations < 50:
        if guess == 0:
            return initial_width
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def test_module() -> int:
    """Test pulse computations."""
    ok: int = 0
    gp: int = gaussian_pulse(1000, 1000, 500, 1000)
    if gp == 1000:
        ok = ok + 1
    pp: int = pulse_position(0, 2000, 3000)
    if pp == 6000:
        ok = ok + 1
    ra: int = reflected_amplitude(1000, 1000, 3000)
    if ra == 500:
        ok = ok + 1
    ta: int = transmitted_amplitude(1000, 1000, 3000)
    if ta == 1500:
        ok = ok + 1
    rc: int = reflection_coefficient(1000, 1000)
    if rc == 0:
        ok = ok + 1
    tc: int = transmission_coefficient(1000, 1000)
    if tc == 1000:
        ok = ok + 1
    pe: int = pulse_energy(1000, 1000)
    if pe > 0:
        ok = ok + 1
    return ok
