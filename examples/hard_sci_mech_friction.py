"""Friction computations using integer arithmetic.

Tests: static/kinetic friction, normal force, incline, braking.
Scale factor 1000 for fixed-point.
"""


def normal_force_flat(mass: int, gravity: int) -> int:
    """Normal force on flat surface: N = m*g. Scale 1000."""
    result: int = (mass * gravity) // 1000
    return result


def normal_force_incline(mass: int, gravity: int, cos_angle: int) -> int:
    """Normal force on incline: N = m*g*cos(theta). Scale 1000."""
    result: int = (mass * gravity * cos_angle) // (1000 * 1000)
    return result


def static_friction_max(mu_s: int, normal: int) -> int:
    """Maximum static friction: f_s = mu_s * N. Scale 1000."""
    result: int = (mu_s * normal) // 1000
    return result


def kinetic_friction(mu_k: int, normal: int) -> int:
    """Kinetic friction: f_k = mu_k * N. Scale 1000."""
    result: int = (mu_k * normal) // 1000
    return result


def friction_deceleration(mu_k: int, gravity: int) -> int:
    """Deceleration due to friction: a = mu_k * g. Scale 1000."""
    result: int = (mu_k * gravity) // 1000
    return result


def braking_distance(speed: int, mu_k: int, gravity: int) -> int:
    """Braking distance: d = v^2 / (2*mu_k*g). Scale 1000."""
    denom: int = (2 * mu_k * gravity) // 1000
    if denom == 0:
        return 0
    v_sq: int = (speed * speed) // 1000
    result: int = (v_sq * 1000) // denom
    return result


def incline_acceleration(gravity: int, sin_angle: int, mu_k: int, cos_angle: int) -> int:
    """Acceleration on incline: a = g*(sin(theta) - mu_k*cos(theta)).
    Scale 1000."""
    sin_term: int = (gravity * sin_angle) // 1000
    cos_term: int = (gravity * mu_k * cos_angle) // (1000 * 1000)
    result: int = sin_term - cos_term
    return result


def angle_of_repose(mu_s: int) -> int:
    """Angle of repose: tan(theta) = mu_s.
    theta ~ atan(mu_s) ~ mu_s for small mu_s. Returns angle*1000 rad."""
    x: int = mu_s
    x3: int = (x * x * x) // (1000 * 1000)
    result: int = x - x3 // 3
    return result


def work_against_friction(friction_force: int, distance: int) -> int:
    """Work done against friction: W = f*d. Scale 1000."""
    result: int = (friction_force * distance) // 1000
    return result


def coefficient_from_angle(sin_angle: int, cos_angle: int) -> int:
    """Friction coefficient from angle: mu = tan(theta) = sin/cos. Scale 1000."""
    if cos_angle == 0:
        return 0
    result: int = (sin_angle * 1000) // cos_angle
    return result


def test_module() -> int:
    """Test friction computations."""
    ok: int = 0
    nf: int = normal_force_flat(10000, 9810)
    if nf == 98100:
        ok = ok + 1
    sf: int = static_friction_max(500, 98100)
    if sf == 49050:
        ok = ok + 1
    kf: int = kinetic_friction(400, 98100)
    if kf == 39240:
        ok = ok + 1
    bd: int = braking_distance(20000, 700, 9810)
    if bd > 29000 and bd < 29300:
        ok = ok + 1
    wf: int = work_against_friction(100, 5000)
    if wf == 500:
        ok = ok + 1
    ca: int = coefficient_from_angle(500, 866)
    if ca > 570 and ca < 580:
        ok = ok + 1
    return ok
