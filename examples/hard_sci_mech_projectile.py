"""Projectile motion computations using integer arithmetic.

Tests: range, height, time of flight, trajectory.
Scale factor 1000 for fixed-point.
"""


def projectile_range(speed: int, angle_sin: int, angle_cos: int, gravity: int) -> int:
    """Projectile range: R = v^2 * sin(2*theta) / g = v^2 * 2*sin*cos / g.
    Scale 1000."""
    if gravity == 0:
        return 0
    v_sq: int = (speed * speed) // 1000
    sin_2theta: int = (2 * angle_sin * angle_cos) // 1000
    result: int = (v_sq * sin_2theta) // gravity
    return result


def projectile_max_height(speed: int, angle_sin: int, gravity: int) -> int:
    """Max height: H = v^2 * sin^2(theta) / (2*g). Scale 1000."""
    if gravity == 0:
        return 0
    v_sq: int = (speed * speed) // 1000
    sin_sq: int = (angle_sin * angle_sin) // 1000
    result: int = (v_sq * sin_sq) // (2 * gravity)
    return result


def projectile_time_of_flight(speed: int, angle_sin: int, gravity: int) -> int:
    """Time of flight: T = 2*v*sin(theta)/g. Scale 1000."""
    if gravity == 0:
        return 0
    result: int = (2 * speed * angle_sin) // gravity
    return result


def horizontal_position(speed: int, angle_cos: int, elapsed: int) -> int:
    """Horizontal position: x = v*cos(theta)*t. Scale 1000."""
    result: int = (speed * angle_cos * elapsed) // (1000 * 1000)
    return result


def vertical_position(speed: int, angle_sin: int, gravity: int, elapsed: int) -> int:
    """Vertical position: y = v*sin(theta)*t - 0.5*g*t^2. Scale 1000."""
    v_term: int = (speed * angle_sin * elapsed) // (1000 * 1000)
    t_sq: int = (elapsed * elapsed) // 1000
    g_term: int = (gravity * t_sq) // 2000
    return v_term - g_term


def impact_speed(speed: int, gravity: int, launch_height: int) -> int:
    """Impact speed from height h: v_f = sqrt(v0^2 + 2*g*h). Scale 1000."""
    v_sq: int = (speed * speed) // 1000
    gh: int = (2 * gravity * launch_height) // 1000
    total: int = v_sq + gh
    if total <= 0:
        return 0
    guess: int = total
    iterations: int = 0
    target: int = total * 1000
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


def time_to_reach_height(speed: int, angle_sin: int, gravity: int, height: int) -> int:
    """Time to reach height h: solve h = v*sin*t - 0.5*g*t^2.
    First root (ascending): t = (v*sin - sqrt((v*sin)^2 - 2*g*h)) / g.
    Scale 1000."""
    if gravity == 0:
        return 0
    v_sin: int = (speed * angle_sin) // 1000
    v_sin_sq: int = (v_sin * v_sin) // 1000
    two_gh: int = (2 * gravity * height) // 1000
    discriminant: int = v_sin_sq - two_gh
    if discriminant < 0:
        return 0
    guess: int = discriminant
    if guess == 0:
        return (v_sin * 1000) // gravity
    iterations: int = 0
    target: int = discriminant * 1000
    while iterations < 50:
        if guess == 0:
            return (v_sin * 1000) // gravity
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            result: int = ((v_sin - next_g) * 1000) // gravity
            return result
        guess = next_g
        iterations = iterations + 1
    result2: int = ((v_sin - guess) * 1000) // gravity
    return result2


def test_module() -> int:
    """Test projectile computations."""
    ok: int = 0
    rng: int = projectile_range(10000, 707, 707, 9810)
    if rng > 10100 and rng < 10300:
        ok = ok + 1
    h: int = projectile_max_height(10000, 707, 9810)
    if h > 2540 and h < 2560:
        ok = ok + 1
    tof: int = projectile_time_of_flight(10000, 707, 9810)
    if tof > 1440 and tof < 1445:
        ok = ok + 1
    hp: int = horizontal_position(10000, 707, 1000)
    if hp == 7070:
        ok = ok + 1
    vp: int = vertical_position(10000, 1000, 9810, 1000)
    if vp > 5090 and vp < 5100:
        ok = ok + 1
    imp: int = impact_speed(0, 9810, 10000)
    if imp > 13900 and imp < 14100:
        ok = ok + 1
    return ok
