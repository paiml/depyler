"""Spring-mass system computations using integer arithmetic.

Tests: Hooke's law, natural frequency, energy, damped oscillation.
Scale factor 1000 for fixed-point.
"""


def hookes_law_force(stiffness: int, displacement_val: int) -> int:
    """Spring force F = -k*x. Returns magnitude. Scale 1000."""
    result: int = (stiffness * displacement_val) // 1000
    if result < 0:
        result = 0 - result
    return result


def spring_potential_energy(stiffness: int, displacement_val: int) -> int:
    """PE = 0.5*k*x^2. Scale 1000."""
    x_sq: int = (displacement_val * displacement_val) // 1000
    result: int = (stiffness * x_sq) // 2000
    return result


def spring_natural_frequency(stiffness: int, mass: int) -> int:
    """omega_n = sqrt(k/m). Scale 1000."""
    if mass == 0:
        return 0
    ratio: int = (stiffness * 1000) // mass
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


def spring_period(stiffness: int, mass: int) -> int:
    """Period T = 2*pi*sqrt(m/k). Scale 1000."""
    omega: int = spring_natural_frequency(stiffness, mass)
    if omega == 0:
        return 0
    result: int = (6283 * 1000) // omega
    return result


def series_spring_constant(k1: int, k2: int) -> int:
    """Series springs: 1/k = 1/k1 + 1/k2 => k = k1*k2/(k1+k2). Scale 1000."""
    denom: int = k1 + k2
    if denom == 0:
        return 0
    result: int = (k1 * k2) // denom
    return result


def parallel_spring_constant(k1: int, k2: int) -> int:
    """Parallel springs: k = k1 + k2."""
    return k1 + k2


def damped_amplitude(initial_amp: int, damping: int, elapsed: int) -> int:
    """Damped amplitude: A(t) = A0 * exp(-gamma*t).
    exp(-x) ~ 1 - x + x^2/2. Scale 1000."""
    x: int = (damping * elapsed) // 1000
    if x > 5000:
        return 0
    exp_val: int = 1000 - x + (x * x) // 2000
    if exp_val < 0:
        exp_val = 0
    result: int = (initial_amp * exp_val) // 1000
    return result


def critical_damping(stiffness: int, mass: int) -> int:
    """Critical damping coefficient: c_c = 2*sqrt(k*m). Scale 1000."""
    product: int = (stiffness * mass) // 1000
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
            return 2 * next_g
        guess = next_g
        iterations = iterations + 1
    return 2 * guess


def static_deflection(mass: int, gravity: int, stiffness: int) -> int:
    """Static deflection: delta = m*g/k. Scale 1000."""
    if stiffness == 0:
        return 0
    result: int = (mass * gravity) // stiffness
    return result


def test_module() -> int:
    """Test spring-mass computations."""
    ok: int = 0
    f: int = hookes_law_force(1000, 500)
    if f == 500:
        ok = ok + 1
    pe: int = spring_potential_energy(1000, 2000)
    if pe == 2000:
        ok = ok + 1
    sk_s: int = series_spring_constant(2000, 2000)
    if sk_s == 1000:
        ok = ok + 1
    sk_p: int = parallel_spring_constant(2000, 3000)
    if sk_p == 5000:
        ok = ok + 1
    da: int = damped_amplitude(1000, 0, 1000)
    if da == 1000:
        ok = ok + 1
    sd: int = static_deflection(1000, 9810, 1000)
    if sd == 9810:
        ok = ok + 1
    return ok
