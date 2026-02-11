"""Pendulum computations using integer arithmetic.

Tests: period, frequency, energy, small angle approximation.
Scale factor 1000 for fixed-point.
"""


def pendulum_period(length: int, gravity: int) -> int:
    """Period T = 2*pi*sqrt(L/g). 2*pi ~ 6283. Scale 1000."""
    if gravity == 0:
        return 0
    ratio: int = (length * 1000) // gravity
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
            return (6283 * next_g) // 1000
        guess = next_g
        iterations = iterations + 1
    return (6283 * guess) // 1000


def pendulum_frequency(length: int, gravity: int) -> int:
    """Frequency f = 1/(2*pi)*sqrt(g/L). Scale 1000."""
    period: int = pendulum_period(length, gravity)
    if period == 0:
        return 0
    result: int = (1000 * 1000) // period
    return result


def pendulum_max_speed(length: int, gravity: int, angle_rad: int) -> int:
    """Max speed at bottom: v = sqrt(2*g*L*(1-cos(theta))).
    cos(theta) ~ 1 - theta^2/2. Scale 1000."""
    cos_val: int = 1000 - (angle_rad * angle_rad) // 2000
    one_minus_cos: int = 1000 - cos_val
    product: int = (2 * gravity * length * one_minus_cos) // (1000 * 1000)
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


def pendulum_potential_energy(mass: int, gravity: int, length: int, angle_rad: int) -> int:
    """PE = m*g*L*(1-cos(theta)). cos(theta) ~ 1 - theta^2/2. Scale 1000."""
    cos_val: int = 1000 - (angle_rad * angle_rad) // 2000
    height: int = (length * (1000 - cos_val)) // 1000
    result: int = (mass * gravity * height) // (1000 * 1000)
    return result


def pendulum_kinetic_energy(mass: int, velocity: int) -> int:
    """KE = 0.5*m*v^2. Scale 1000."""
    v_sq: int = (velocity * velocity) // 1000
    result: int = (mass * v_sq) // 2000
    return result


def pendulum_length_from_period(period: int, gravity: int) -> int:
    """Length from period: L = g*(T/(2*pi))^2. Scale 1000."""
    t_over_2pi: int = (period * 1000) // 6283
    t_sq: int = (t_over_2pi * t_over_2pi) // 1000
    result: int = (gravity * t_sq) // 1000
    return result


def physical_pendulum_period(moment_of_inertia: int, mass: int, gravity: int, dist_cm: int) -> int:
    """Physical pendulum period: T = 2*pi*sqrt(I/(m*g*d)). Scale 1000."""
    denom: int = (mass * gravity * dist_cm) // (1000 * 1000)
    if denom == 0:
        return 0
    ratio: int = (moment_of_inertia * 1000) // denom
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
            return (6283 * next_g) // 1000
        guess = next_g
        iterations = iterations + 1
    return (6283 * guess) // 1000


def test_module() -> int:
    """Test pendulum computations."""
    ok: int = 0
    p: int = pendulum_period(1000, 9810)
    if p > 1980 and p < 2020:
        ok = ok + 1
    f: int = pendulum_frequency(1000, 9810)
    if f > 490 and f < 510:
        ok = ok + 1
    lp: int = pendulum_length_from_period(2006, 9810)
    if lp > 980 and lp < 1020:
        ok = ok + 1
    ke: int = pendulum_kinetic_energy(1000, 2000)
    if ke == 2000:
        ok = ok + 1
    ms: int = pendulum_max_speed(1000, 9810, 0)
    if ms == 0:
        ok = ok + 1
    pp: int = physical_pendulum_period(0, 1000, 9810, 500)
    if pp == 0:
        ok = ok + 1
    return ok
