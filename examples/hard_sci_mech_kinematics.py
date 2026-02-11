"""Kinematics computations using integer arithmetic.

Tests: displacement, velocity, acceleration, free fall.
Scale factor 1000 for fixed-point. Gravity g = 9810 (scale 1000).
"""


def displacement(v_initial: int, accel: int, elapsed: int) -> int:
    """Displacement s = v0*t + 0.5*a*t^2. Scale 1000."""
    v_term: int = (v_initial * elapsed) // 1000
    t_sq: int = (elapsed * elapsed) // 1000
    a_term: int = (accel * t_sq) // 2000
    return v_term + a_term


def final_velocity(v_initial: int, accel: int, elapsed: int) -> int:
    """Final velocity v = v0 + a*t. Scale 1000."""
    result: int = v_initial + (accel * elapsed) // 1000
    return result


def velocity_from_displacement(v_initial: int, accel: int, dist: int) -> int:
    """Final velocity: v^2 = v0^2 + 2*a*s. Returns v. Scale 1000."""
    v0_sq: int = (v_initial * v_initial) // 1000
    two_as: int = (2 * accel * dist) // 1000
    v_sq: int = v0_sq + two_as
    if v_sq < 0:
        return 0
    guess: int = v_sq
    if guess == 0:
        return 0
    iterations: int = 0
    target: int = v_sq * 1000
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


def time_of_flight(v_initial: int, accel: int) -> int:
    """Time to return to initial height (projectile up): t = -2*v0/a.
    Scale 1000."""
    if accel == 0:
        return 0
    result: int = (0 - 2 * v_initial * 1000) // accel
    return result


def max_height(v_initial: int, gravity: int) -> int:
    """Maximum height: h = v0^2 / (2*g). Scale 1000."""
    if gravity == 0:
        return 0
    v_sq: int = (v_initial * v_initial) // 1000
    result: int = (v_sq * 1000) // (2 * gravity)
    return result


def free_fall_time(height: int, gravity: int) -> int:
    """Free fall time: t = sqrt(2*h/g). Scale 1000."""
    if gravity == 0:
        return 0
    ratio: int = (2 * height * 1000) // gravity
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


def average_velocity(v_initial: int, v_final: int) -> int:
    """Average velocity = (v0 + v)/2."""
    return (v_initial + v_final) // 2


def relative_velocity(v_a: int, v_b: int) -> int:
    """Relative velocity of A with respect to B."""
    return v_a - v_b


def test_module() -> int:
    """Test kinematics computations."""
    ok: int = 0
    s: int = displacement(10000, 0, 5000)
    if s == 50000:
        ok = ok + 1
    vf: int = final_velocity(0, 9810, 1000)
    if vf == 9810:
        ok = ok + 1
    tof: int = time_of_flight(9810, 0 - 9810)
    if tof == 2000:
        ok = ok + 1
    mh: int = max_height(9810, 9810)
    if mh > 4900 and mh < 4920:
        ok = ok + 1
    avg: int = average_velocity(1000, 3000)
    if avg == 2000:
        ok = ok + 1
    rv: int = relative_velocity(5000, 3000)
    if rv == 2000:
        ok = ok + 1
    return ok
