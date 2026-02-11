"""Wave dispersion computations using integer arithmetic.

Tests: group velocity, phase velocity, dispersion relation, wave packets.
Scale factor 1000 for fixed-point.
"""


def phase_velocity(omega: int, wavenumber: int) -> int:
    """Phase velocity v_p = omega / k. Fixed-point scale 1000."""
    if wavenumber == 0:
        return 0
    result: int = (omega * 1000) // wavenumber
    return result


def group_velocity_approx(omega1: int, omega2: int, k1: int, k2: int) -> int:
    """Group velocity v_g = d_omega/dk approx (omega2-omega1)/(k2-k1).
    Fixed-point scale 1000."""
    dk: int = k2 - k1
    if dk == 0:
        return 0
    d_omega: int = omega2 - omega1
    result: int = (d_omega * 1000) // dk
    return result


def dispersion_relation_deep_water(wavenumber: int, gravity: int) -> int:
    """Deep water waves: omega^2 = g*k. Returns omega. Fixed-point scale 1000.
    gravity in scale 1000."""
    product: int = (gravity * wavenumber) // 1000
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
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def dispersion_relation_string(wavenumber: int, tension: int, density: int) -> int:
    """String waves: omega = k * sqrt(T/rho). Fixed-point scale 1000."""
    if density == 0:
        return 0
    ratio: int = (tension * 1000) // density
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
            return (wavenumber * next_g) // 1000
        guess = next_g
        iterations = iterations + 1
    return (wavenumber * guess) // 1000


def wave_packet_width(dk: int) -> int:
    """Spatial width of wave packet ~ 2*pi/dk. Fixed-point scale 1000."""
    if dk == 0:
        return 0
    result: int = (6283 * 1000) // dk
    return result


def refractive_index(c_vacuum: int, v_medium: int) -> int:
    """Refractive index n = c/v. Fixed-point scale 1000."""
    if v_medium == 0:
        return 0
    result: int = (c_vacuum * 1000) // v_medium
    return result


def snells_law_sin(n1: int, sin_theta1: int, n2: int) -> int:
    """Snell's law: n1*sin(theta1) = n2*sin(theta2).
    Returns sin(theta2) * 1000. Fixed-point scale 1000."""
    if n2 == 0:
        return 0
    result: int = (n1 * sin_theta1) // n2
    return result


def critical_angle_sin(n1: int, n2: int) -> int:
    """Critical angle: sin(theta_c) = n2/n1. Fixed-point scale 1000.
    Returns 0 if total internal reflection not possible (n1 < n2)."""
    if n1 == 0:
        return 0
    if n1 < n2:
        return 0
    result: int = (n2 * 1000) // n1
    return result


def test_module() -> int:
    """Test dispersion computations."""
    ok: int = 0
    vp: int = phase_velocity(6000, 3000)
    if vp == 2000:
        ok = ok + 1
    vg: int = group_velocity_approx(5000, 6000, 2000, 3000)
    if vg == 1000:
        ok = ok + 1
    ri: int = refractive_index(3000, 2000)
    if ri == 1500:
        ok = ok + 1
    sn: int = snells_law_sin(1500, 500, 1000)
    if sn == 750:
        ok = ok + 1
    ca: int = critical_angle_sin(1500, 1000)
    if ca == 666:
        ok = ok + 1
    ca_none: int = critical_angle_sin(1000, 1500)
    if ca_none == 0:
        ok = ok + 1
    wp: int = wave_packet_width(1000)
    if wp == 6283:
        ok = ok + 1
    return ok
