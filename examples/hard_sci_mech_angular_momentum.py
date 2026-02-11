"""Angular momentum and conservation computations using integer arithmetic.

Tests: angular momentum, conservation, precession, gyroscopic effects.
Scale factor 1000 for fixed-point.
"""


def angular_momentum_calc(moment_inertia: int, omega: int) -> int:
    """Angular momentum L = I*omega. Scale 1000."""
    result: int = (moment_inertia * omega) // 1000
    return result


def conservation_omega(i1: int, omega1: int, i2: int) -> int:
    """Conservation of angular momentum: I1*omega1 = I2*omega2.
    Solve for omega2. Scale 1000."""
    if i2 == 0:
        return 0
    result: int = (i1 * omega1) // i2
    return result


def precession_rate(torque_val: int, ang_momentum: int) -> int:
    """Precession rate: Omega_p = tau/L. Scale 1000."""
    if ang_momentum == 0:
        return 0
    result: int = (torque_val * 1000) // ang_momentum
    return result


def gyroscopic_torque(moment_inertia: int, omega_spin: int, omega_prec: int) -> int:
    """Gyroscopic torque: tau = I*omega_spin*omega_prec. Scale 1000."""
    result: int = (moment_inertia * omega_spin * omega_prec) // (1000 * 1000)
    return result


def spin_angular_momentum(moment_inertia: int, rpm: int) -> int:
    """Angular momentum from RPM: L = I * (2*pi*rpm/60). Scale 1000."""
    omega: int = (6283 * rpm) // 60000
    result: int = (moment_inertia * omega) // 1000
    return result


def collision_angular(i1: int, omega1: int, i2: int, omega2: int) -> int:
    """Final angular velocity after perfectly inelastic collision.
    omega_f = (I1*omega1 + I2*omega2)/(I1+I2). Scale 1000."""
    denom: int = i1 + i2
    if denom == 0:
        return 0
    numer: int = (i1 * omega1 + i2 * omega2) // 1000
    result: int = (numer * 1000) // denom
    return result


def rotational_impulse(torque_val: int, duration: int) -> int:
    """Rotational impulse: J = tau*dt. Scale 1000."""
    result: int = (torque_val * duration) // 1000
    return result


def angular_velocity_from_energy(energy: int, moment_inertia: int) -> int:
    """omega = sqrt(2*E/I). Scale 1000."""
    if moment_inertia == 0:
        return 0
    ratio: int = (2 * energy * 1000) // moment_inertia
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
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def test_module() -> int:
    """Test angular momentum computations."""
    ok: int = 0
    am: int = angular_momentum_calc(5000, 2000)
    if am == 10000:
        ok = ok + 1
    omega2: int = conservation_omega(5000, 2000, 10000)
    if omega2 == 1000:
        ok = ok + 1
    pr: int = precession_rate(100, 10000)
    if pr == 10:
        ok = ok + 1
    gt: int = gyroscopic_torque(5000, 100, 10)
    if gt == 5:
        ok = ok + 1
    co: int = collision_angular(5000, 2000, 5000, 0)
    if co == 1000:
        ok = ok + 1
    ri: int = rotational_impulse(100, 5000)
    if ri == 500:
        ok = ok + 1
    return ok
