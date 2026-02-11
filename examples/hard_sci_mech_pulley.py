"""Pulley system computations using integer arithmetic.

Tests: mechanical advantage, force, acceleration, Atwood machine.
Scale factor 1000 for fixed-point.
"""


def single_fixed_pulley(effort: int) -> int:
    """Single fixed pulley changes direction only. MA=1."""
    return effort


def single_movable_pulley(load: int) -> int:
    """Single movable pulley: effort = load/2. Scale 1000."""
    return load // 2


def compound_pulley_effort(load: int, num_ropes: int) -> int:
    """Compound pulley: effort = load/n. Scale 1000."""
    if num_ropes == 0:
        return 0
    return load // num_ropes


def mechanical_advantage_pulley(num_supporting_ropes: int) -> int:
    """Mechanical advantage = number of supporting ropes. Scale 1000."""
    return num_supporting_ropes * 1000


def velocity_ratio(num_ropes: int) -> int:
    """Velocity ratio = n (distance pulled / distance lifted). Scale 1000."""
    return num_ropes * 1000


def pulley_efficiency(ma_actual: int, vr: int) -> int:
    """Efficiency = MA/VR * 1000. Scale 1000."""
    if vr == 0:
        return 0
    result: int = (ma_actual * 1000) // vr
    return result


def atwood_acceleration(m1: int, m2: int, gravity: int) -> int:
    """Atwood machine acceleration: a = g*(m1-m2)/(m1+m2). Scale 1000.
    m1 > m2."""
    denom: int = m1 + m2
    if denom == 0:
        return 0
    diff: int = m1 - m2
    result: int = (gravity * diff) // denom
    return result


def atwood_tension(m1: int, m2: int, gravity: int) -> int:
    """Atwood machine tension: T = 2*m1*m2*g/(m1+m2). Scale 1000."""
    denom: int = m1 + m2
    if denom == 0:
        return 0
    result: int = (2 * m1 * m2 * gravity) // (denom * 1000)
    return result


def rope_tension_incline(mass: int, gravity: int, sin_angle: int, mu_k: int, cos_angle: int) -> int:
    """Tension to pull mass up incline at constant velocity.
    T = m*g*(sin(theta) + mu_k*cos(theta)). Scale 1000."""
    sin_term: int = (mass * gravity * sin_angle) // (1000 * 1000)
    cos_term: int = (mass * gravity * mu_k * cos_angle) // (1000 * 1000 * 1000)
    return sin_term + cos_term


def work_done_pulley(effort: int, rope_pulled: int) -> int:
    """Work done pulling rope: W = F*d. Scale 1000."""
    result: int = (effort * rope_pulled) // 1000
    return result


def test_module() -> int:
    """Test pulley computations."""
    ok: int = 0
    sfp: int = single_fixed_pulley(100)
    if sfp == 100:
        ok = ok + 1
    smp: int = single_movable_pulley(200)
    if smp == 100:
        ok = ok + 1
    cpe: int = compound_pulley_effort(600, 3)
    if cpe == 200:
        ok = ok + 1
    ma: int = mechanical_advantage_pulley(4)
    if ma == 4000:
        ok = ok + 1
    aa: int = atwood_acceleration(3000, 1000, 9810)
    if aa == 4905:
        ok = ok + 1
    wd: int = work_done_pulley(100, 5000)
    if wd == 500:
        ok = ok + 1
    eff: int = pulley_efficiency(3800, 4000)
    if eff == 950:
        ok = ok + 1
    return ok
