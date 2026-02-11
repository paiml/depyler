"""Impedance and AC circuit computations using integer arithmetic.

Tests: impedance magnitude/phase, admittance, complex power.
Scale factor 1000 for fixed-point.
"""


def capacitive_reactance(capacitance: int, frequency: int) -> int:
    """Capacitive reactance: XC = 1/(2*pi*f*C). 2*pi ~ 6283. Scale 1000."""
    denom: int = (6283 * frequency * capacitance) // (1000 * 1000)
    if denom == 0:
        return 0
    result: int = (1000 * 1000) // denom
    return result


def inductive_reactance(inductance: int, frequency: int) -> int:
    """Inductive reactance: XL = 2*pi*f*L. 2*pi ~ 6283. Scale 1000."""
    result: int = (6283 * frequency * inductance) // (1000 * 1000)
    return result


def impedance_magnitude(resistance: int, reactance: int) -> int:
    """Impedance magnitude: |Z| = sqrt(R^2 + X^2). Scale 1000."""
    r_sq: int = (resistance * resistance) // 1000
    x_sq: int = (reactance * reactance) // 1000
    sum_sq: int = r_sq + x_sq
    if sum_sq <= 0:
        return resistance
    guess: int = sum_sq
    iterations: int = 0
    target: int = sum_sq * 1000
    while iterations < 50:
        if guess == 0:
            return resistance
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def admittance(resistance: int, reactance: int) -> int:
    """Admittance magnitude: Y = 1/|Z|. Scale 1000."""
    z_mag: int = impedance_magnitude(resistance, reactance)
    if z_mag == 0:
        return 0
    result: int = (1000 * 1000) // z_mag
    return result


def phase_angle_approx(reactance: int, resistance: int) -> int:
    """Phase angle approx: atan(X/R) ~ X/R for small ratios.
    Better approx: atan(x) ~ x - x^3/3. Returns angle*1000 (radians)."""
    if resistance == 0:
        if reactance > 0:
            return 1571
        if reactance < 0:
            return 0 - 1571
        return 0
    x: int = (reactance * 1000) // resistance
    x3: int = (x * x * x) // (1000 * 1000)
    result: int = x - x3 // 3
    return result


def skin_depth(resistivity: int, frequency: int, permeability: int) -> int:
    """Skin depth: delta = sqrt(rho/(pi*f*mu)).
    pi ~ 3142. Scale 1000."""
    if frequency == 0 or permeability == 0:
        return 0
    denom: int = (3142 * frequency * permeability) // (1000 * 1000)
    if denom == 0:
        return 0
    ratio: int = (resistivity * 1000) // denom
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


def quality_factor_component(reactance: int, resistance: int) -> int:
    """Component Q factor = |X|/R. Scale 1000."""
    if resistance == 0:
        return 0
    x_abs: int = reactance
    if x_abs < 0:
        x_abs = 0 - x_abs
    result: int = (x_abs * 1000) // resistance
    return result


def test_module() -> int:
    """Test impedance computations."""
    ok: int = 0
    z: int = impedance_magnitude(3000, 4000)
    if z > 4990 and z < 5010:
        ok = ok + 1
    y: int = admittance(3000, 4000)
    if y > 195 and y < 205:
        ok = ok + 1
    pa: int = phase_angle_approx(0, 1000)
    if pa == 0:
        ok = ok + 1
    pa2: int = phase_angle_approx(1000, 0)
    if pa2 == 1571:
        ok = ok + 1
    qf: int = quality_factor_component(5000, 100)
    if qf == 50000:
        ok = ok + 1
    z_r_only: int = impedance_magnitude(1000, 0)
    if z_r_only == 1000:
        ok = ok + 1
    return ok
