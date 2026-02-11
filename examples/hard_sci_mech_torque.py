"""Torque and rotational dynamics computations using integer arithmetic.

Tests: torque, moment of inertia, angular acceleration, rotational energy.
Scale factor 1000 for fixed-point.
"""


def torque_calc(force: int, radius: int, sin_angle: int) -> int:
    """Torque tau = F*r*sin(theta). Scale 1000."""
    result: int = (force * radius * sin_angle) // (1000 * 1000)
    return result


def angular_acceleration(torque_val: int, moment_of_inertia: int) -> int:
    """Angular acceleration alpha = tau/I. Scale 1000."""
    if moment_of_inertia == 0:
        return 0
    result: int = (torque_val * 1000) // moment_of_inertia
    return result


def rotational_kinetic_energy(moment_of_inertia: int, omega: int) -> int:
    """KE_rot = 0.5*I*omega^2. Scale 1000."""
    w_sq: int = (omega * omega) // 1000
    result: int = (moment_of_inertia * w_sq) // 2000
    return result


def moment_inertia_disk(mass: int, radius: int) -> int:
    """Moment of inertia of solid disk: I = 0.5*m*r^2. Scale 1000."""
    r_sq: int = (radius * radius) // 1000
    result: int = (mass * r_sq) // 2000
    return result


def moment_inertia_rod_center(mass: int, length: int) -> int:
    """Moment of inertia of rod about center: I = (1/12)*m*L^2. Scale 1000."""
    l_sq: int = (length * length) // 1000
    result: int = (mass * l_sq) // 12000
    return result


def moment_inertia_rod_end(mass: int, length: int) -> int:
    """Moment of inertia of rod about end: I = (1/3)*m*L^2. Scale 1000."""
    l_sq: int = (length * length) // 1000
    result: int = (mass * l_sq) // 3000
    return result


def parallel_axis(i_cm: int, mass: int, distance: int) -> int:
    """Parallel axis theorem: I = I_cm + m*d^2. Scale 1000."""
    d_sq: int = (distance * distance) // 1000
    md_sq: int = (mass * d_sq) // 1000
    return i_cm + md_sq


def angular_momentum(moment_of_inertia: int, omega: int) -> int:
    """Angular momentum L = I*omega. Scale 1000."""
    result: int = (moment_of_inertia * omega) // 1000
    return result


def final_angular_velocity(omega_0: int, alpha: int, elapsed: int) -> int:
    """Final angular velocity: omega = omega_0 + alpha*t. Scale 1000."""
    result: int = omega_0 + (alpha * elapsed) // 1000
    return result


def angular_displacement(omega_0: int, alpha: int, elapsed: int) -> int:
    """Angular displacement: theta = omega_0*t + 0.5*alpha*t^2. Scale 1000."""
    w_term: int = (omega_0 * elapsed) // 1000
    t_sq: int = (elapsed * elapsed) // 1000
    a_term: int = (alpha * t_sq) // 2000
    return w_term + a_term


def test_module() -> int:
    """Test torque computations."""
    ok: int = 0
    t: int = torque_calc(100, 500, 1000)
    if t == 50:
        ok = ok + 1
    aa: int = angular_acceleration(50000, 10000)
    if aa == 5000:
        ok = ok + 1
    ke: int = rotational_kinetic_energy(10000, 2000)
    if ke == 20000:
        ok = ok + 1
    id_val: int = moment_inertia_disk(2000, 1000)
    if id_val == 1000:
        ok = ok + 1
    am: int = angular_momentum(10000, 5000)
    if am == 50000:
        ok = ok + 1
    fav: int = final_angular_velocity(1000, 500, 2000)
    if fav == 2000:
        ok = ok + 1
    return ok
