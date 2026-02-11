"""Chemistry gas law computations using integer arithmetic.

Tests: ideal gas, Dalton's law, Graham's law, Van der Waals.
Scale factor 1000 for fixed-point.
"""


def ideal_gas_pv(moles: int, temp: int) -> int:
    """PV = nRT product. R = 8314 scale 1000. Scale 1000."""
    r_const: int = 8314
    result: int = (moles * r_const * temp) // (1000 * 1000)
    return result


def dalton_partial_pressure(total_p: int, mole_fraction: int) -> int:
    """Partial pressure: Pi = P_total * xi. Scale 1000."""
    result: int = (total_p * mole_fraction) // 1000
    return result


def mole_fraction(moles_component: int, total_moles: int) -> int:
    """Mole fraction: xi = ni/n_total. Scale 1000."""
    if total_moles == 0:
        return 0
    result: int = (moles_component * 1000) // total_moles
    return result


def grahams_law_rate_ratio(mm1: int, mm2: int) -> int:
    """Graham's law: rate1/rate2 = sqrt(MM2/MM1). Scale 1000."""
    if mm1 == 0:
        return 0
    ratio: int = (mm2 * 1000) // mm1
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


def total_pressure_from_partials(partials: list[int]) -> int:
    """Total pressure = sum of partial pressures."""
    total: int = 0
    i: int = 0
    while i < len(partials):
        val: int = partials[i]
        total = total + val
        i = i + 1
    return total


def gas_density_ideal(pressure: int, molar_mass: int, temp: int) -> int:
    """Gas density rho = PM/(RT). R=8314. Scale 1000."""
    if temp == 0:
        return 0
    r_const: int = 8314
    result: int = (pressure * molar_mass) // (r_const * temp)
    return result


def average_molar_mass(masses: list[int], fractions: list[int]) -> int:
    """Average molar mass = sum(xi * Mi). Scale 1000."""
    total: int = 0
    i: int = 0
    n: int = len(masses)
    if n > len(fractions):
        n = len(fractions)
    while i < n:
        m_val: int = masses[i]
        f_val: int = fractions[i]
        total = total + (m_val * f_val) // 1000
        i = i + 1
    return total


def compressibility_factor(pv: int, nrt: int) -> int:
    """Compressibility factor Z = PV/(nRT). Scale 1000."""
    if nrt == 0:
        return 0
    result: int = (pv * 1000) // nrt
    return result


def test_module() -> int:
    """Test gas law computations."""
    ok: int = 0
    mf: int = mole_fraction(1000, 4000)
    if mf == 250:
        ok = ok + 1
    pp: int = dalton_partial_pressure(100000, 250)
    if pp == 25000:
        ok = ok + 1
    tp: int = total_pressure_from_partials([25000, 50000, 25000])
    if tp == 100000:
        ok = ok + 1
    gr: int = grahams_law_rate_ratio(2000, 32000)
    if gr > 3990 and gr < 4010:
        ok = ok + 1
    z: int = compressibility_factor(1000, 1000)
    if z == 1000:
        ok = ok + 1
    amm: int = average_molar_mass([28000, 32000], [780, 210])
    if amm > 28000 and amm < 29000:
        ok = ok + 1
    return ok
