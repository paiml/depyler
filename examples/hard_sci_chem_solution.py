"""Solution chemistry computations using integer arithmetic.

Tests: concentration, osmotic pressure, colligative properties.
Scale factor 1000 for fixed-point.
"""


def molarity(moles_solute: int, volume_liters: int) -> int:
    """Molarity M = moles/volume. Scale 1000."""
    if volume_liters == 0:
        return 0
    result: int = (moles_solute * 1000) // volume_liters
    return result


def molality(moles_solute: int, mass_solvent_kg: int) -> int:
    """Molality m = moles/mass_solvent_kg. Scale 1000."""
    if mass_solvent_kg == 0:
        return 0
    result: int = (moles_solute * 1000) // mass_solvent_kg
    return result


def mass_percent(mass_solute: int, mass_solution: int) -> int:
    """Mass percent = (mass_solute/mass_solution)*1000. Scale 1000."""
    if mass_solution == 0:
        return 0
    result: int = (mass_solute * 1000) // mass_solution
    return result


def parts_per_million(mass_solute: int, mass_solution: int) -> int:
    """PPM = (mass_solute/mass_solution)*10^6. Scale 1000."""
    if mass_solution == 0:
        return 0
    result: int = (mass_solute * 1000000) // mass_solution
    return result


def osmotic_pressure(molarity_val: int, temp: int, van_hoff_i: int) -> int:
    """Osmotic pressure pi = i*M*R*T. R = 8314. Scale 1000."""
    r_const: int = 8314
    result: int = (van_hoff_i * molarity_val * r_const * temp) // (1000 * 1000 * 1000)
    return result


def boiling_point_elevation_calc(kb: int, molality_val: int, van_hoff_i: int) -> int:
    """dTb = i * Kb * m. Scale 1000."""
    result: int = (van_hoff_i * kb * molality_val) // (1000 * 1000)
    return result


def freezing_point_depression_calc(kf: int, molality_val: int, van_hoff_i: int) -> int:
    """dTf = i * Kf * m. Scale 1000."""
    result: int = (van_hoff_i * kf * molality_val) // (1000 * 1000)
    return result


def raoults_law(x_solvent: int, p_pure: int) -> int:
    """Raoult's law: P = x_solvent * P_pure. Scale 1000."""
    result: int = (x_solvent * p_pure) // 1000
    return result


def henrys_law(kh: int, partial_pressure: int) -> int:
    """Henry's law: C = kH * P. Scale 1000."""
    result: int = (kh * partial_pressure) // 1000
    return result


def test_module() -> int:
    """Test solution chemistry computations."""
    ok: int = 0
    m_val: int = molarity(1000, 1000)
    if m_val == 1000:
        ok = ok + 1
    mp: int = mass_percent(50, 200)
    if mp == 250:
        ok = ok + 1
    ppm: int = parts_per_million(1, 1000)
    if ppm == 1000:
        ok = ok + 1
    rl: int = raoults_law(900, 1000)
    if rl == 900:
        ok = ok + 1
    hl: int = henrys_law(1000, 500)
    if hl == 500:
        ok = ok + 1
    zero_m: int = molarity(1000, 0)
    if zero_m == 0:
        ok = ok + 1
    return ok
