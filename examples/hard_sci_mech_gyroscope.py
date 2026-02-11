"""Gyroscope and precession computations using integer arithmetic.

Tests: precession, nutation, stability, gyroscopic effects.
Scale factor 1000 for fixed-point.
"""


def gyro_precession_rate(mass: int, gravity: int, dist_cm: int, spin_momentum: int) -> int:
    """Precession rate: Omega_p = m*g*d/L. Scale 1000."""
    if spin_momentum == 0:
        return 0
    numer: int = (mass * gravity * dist_cm) // (1000 * 1000)
    result: int = (numer * 1000) // spin_momentum
    return result


def gyro_nutation_frequency(spin_momentum: int, moment_inertia: int) -> int:
    """Nutation frequency: omega_n = L/(I_perp). Scale 1000."""
    if moment_inertia == 0:
        return 0
    result: int = (spin_momentum * 1000) // moment_inertia
    return result


def gyro_stability_criterion(spin_rate: int, gravity: int, radius: int) -> int:
    """Stability: spin must be > sqrt(4*g*r). Returns min spin rate. Scale 1000."""
    product: int = (4 * gravity * radius) // 1000
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


def gyro_is_stable(spin_rate: int, min_spin: int) -> int:
    """Returns 1 if gyro spin exceeds minimum for stability."""
    if spin_rate >= min_spin:
        return 1
    return 0


def gyro_torque_required(moment_inertia: int, spin_rate: int, prec_rate: int) -> int:
    """Torque for precession: tau = I*omega_spin*Omega_p. Scale 1000."""
    result: int = (moment_inertia * spin_rate * prec_rate) // (1000 * 1000)
    return result


def gyro_kinetic_energy(moment_inertia: int, spin_rate: int) -> int:
    """Rotational KE = 0.5*I*omega^2. Scale 1000."""
    w_sq: int = (spin_rate * spin_rate) // 1000
    result: int = (moment_inertia * w_sq) // 2000
    return result


def gyro_angular_momentum(moment_inertia: int, spin_rate: int) -> int:
    """Angular momentum L = I*omega. Scale 1000."""
    result: int = (moment_inertia * spin_rate) // 1000
    return result


def gyro_drift_rate(bias: int, elapsed: int) -> int:
    """Gyro drift = bias * time. Scale 1000."""
    result: int = (bias * elapsed) // 1000
    return result


def gyro_spin_down_time(initial_spin: int, friction_torque: int, moment_inertia: int) -> int:
    """Time to stop: t = I*omega/tau. Scale 1000."""
    if friction_torque == 0:
        return 0
    numer: int = (moment_inertia * initial_spin) // 1000
    result: int = (numer * 1000) // friction_torque
    return result


def test_module() -> int:
    """Test gyroscope computations."""
    ok: int = 0
    am: int = gyro_angular_momentum(5000, 100000)
    if am == 500000:
        ok = ok + 1
    ke: int = gyro_kinetic_energy(5000, 100000)
    if ke == 25000000:
        ok = ok + 1
    dr: int = gyro_drift_rate(10, 60000)
    if dr == 600:
        ok = ok + 1
    sd: int = gyro_spin_down_time(100000, 1000, 5000)
    if sd == 500000:
        ok = ok + 1
    stable: int = gyro_is_stable(10000, 5000)
    if stable == 1:
        ok = ok + 1
    unstable: int = gyro_is_stable(3000, 5000)
    if unstable == 0:
        ok = ok + 1
    return ok
