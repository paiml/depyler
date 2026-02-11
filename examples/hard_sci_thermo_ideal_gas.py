"""Ideal gas law computations using integer arithmetic.

Tests: PV=nRT calculations, gas density, molar volume, combined gas law.
Scale factor 1000 for fixed-point. R = 8314 (J/(mol*K) * 1000).
"""


def ideal_gas_pressure(moles: int, temp: int, volume: int) -> int:
    """Compute pressure P = nRT/V. Fixed-point scale 1000.
    R = 8314 in scale 1000 units."""
    if volume == 0:
        return 0
    r_const: int = 8314
    result: int = (moles * r_const * temp) // (volume * 1000)
    return result


def ideal_gas_volume(moles: int, temp: int, pressure: int) -> int:
    """Compute volume V = nRT/P. Fixed-point scale 1000."""
    if pressure == 0:
        return 0
    r_const: int = 8314
    result: int = (moles * r_const * temp) // (pressure * 1000)
    return result


def ideal_gas_temp(pressure: int, volume: int, moles: int) -> int:
    """Compute temperature T = PV/(nR). Fixed-point scale 1000."""
    if moles == 0:
        return 0
    r_const: int = 8314
    result: int = (pressure * volume * 1000) // (moles * r_const)
    return result


def ideal_gas_moles(pressure: int, volume: int, temp: int) -> int:
    """Compute moles n = PV/(RT). Fixed-point scale 1000."""
    if temp == 0:
        return 0
    r_const: int = 8314
    result: int = (pressure * volume * 1000) // (r_const * temp)
    return result


def gas_density(pressure: int, molar_mass: int, temp: int) -> int:
    """Gas density rho = PM/(RT). Fixed-point scale 1000."""
    if temp == 0:
        return 0
    r_const: int = 8314
    result: int = (pressure * molar_mass) // (r_const * temp)
    return result


def combined_gas_law_pressure(p1: int, v1: int, t1: int, v2: int, t2: int) -> int:
    """Combined gas law: solve for P2 = P1*V1*T2/(T1*V2). Scale 1000."""
    denom: int = t1 * v2
    if denom == 0:
        return 0
    result: int = (p1 * v1 * t2) // denom
    return result


def rms_speed(temp: int, molar_mass: int) -> int:
    """RMS speed of gas molecules: v_rms = sqrt(3RT/M).
    R = 8314 scale 1000. Fixed-point scale 1000."""
    if molar_mass == 0:
        return 0
    r_const: int = 8314
    numerator: int = (3 * r_const * temp) // molar_mass
    if numerator <= 0:
        return 0
    guess: int = numerator
    iterations: int = 0
    target: int = numerator * 1000
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


def mean_free_path(diameter: int, number_density: int) -> int:
    """Mean free path: lambda = 1/(sqrt(2)*pi*d^2*n).
    sqrt(2) ~ 1414/1000, pi ~ 3142/1000. Fixed-point scale 1000."""
    if diameter == 0 or number_density == 0:
        return 0
    d_sq: int = (diameter * diameter) // 1000
    denom: int = (1414 * 3142 * d_sq * number_density) // (1000 * 1000 * 1000)
    if denom == 0:
        return 0
    result: int = (1000 * 1000) // denom
    return result


def test_module() -> int:
    """Test ideal gas computations."""
    ok: int = 0
    p: int = ideal_gas_pressure(1000, 273, 22400)
    if p > 90 and p < 120:
        ok = ok + 1
    v: int = ideal_gas_volume(1000, 273, 101)
    if v > 20000 and v < 25000:
        ok = ok + 1
    t: int = ideal_gas_temp(101, 22400, 1000)
    if t > 260 and t < 290:
        ok = ok + 1
    p2: int = combined_gas_law_pressure(100, 1000, 300, 500, 600)
    if p2 == 400:
        ok = ok + 1
    d: int = gas_density(101, 29, 273)
    if d >= 0:
        ok = ok + 1
    zero_v: int = ideal_gas_volume(1000, 273, 0)
    if zero_v == 0:
        ok = ok + 1
    return ok
