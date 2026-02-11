"""Molarity and concentration computations using integer arithmetic.

Tests: molarity, normality, formality, dilution.
Scale factor 1000 for fixed-point.
"""


def molarity_from_mass(mass_solute: int, molar_mass: int, volume_ml: int) -> int:
    """Molarity = (mass/MM) / (volume_mL/1000). Scale 1000."""
    if molar_mass == 0 or volume_ml == 0:
        return 0
    moles: int = (mass_solute * 1000) // molar_mass
    result: int = (moles * 1000000) // volume_ml
    return result


def normality(molarity_val: int, equiv_factor: int) -> int:
    """Normality N = M * n (equivalents per mole). Scale 1000."""
    result: int = (molarity_val * equiv_factor) // 1000
    return result


def mass_needed(molarity_val: int, volume_ml: int, molar_mass: int) -> int:
    """Mass of solute needed: m = M * V(L) * MM. Scale 1000."""
    vol_l: int = volume_ml
    moles: int = (molarity_val * vol_l) // (1000 * 1000)
    result: int = (moles * molar_mass) // 1000
    return result


def moles_in_solution(molarity_val: int, volume_ml: int) -> int:
    """Moles = M * V(L). volume_ml in mL. Scale 1000."""
    result: int = (molarity_val * volume_ml) // 1000000
    return result


def volume_for_moles(molarity_val: int, desired_moles: int) -> int:
    """Volume (mL) = moles * 1000 / M. Scale 1000."""
    if molarity_val == 0:
        return 0
    result: int = (desired_moles * 1000000) // molarity_val
    return result


def serial_dilution(initial_conc: int, dilution_factor: int, num_steps: int) -> int:
    """Concentration after serial dilutions. Scale 1000."""
    conc: int = initial_conc
    step: int = 0
    while step < num_steps:
        conc = (conc * 1000) // dilution_factor
        step = step + 1
    return conc


def mixing_concentration(c1: int, v1: int, c2: int, v2: int) -> int:
    """Concentration after mixing: C = (C1*V1 + C2*V2)/(V1+V2). Scale 1000."""
    total_v: int = v1 + v2
    if total_v == 0:
        return 0
    total_moles: int = (c1 * v1 + c2 * v2) // 1000
    result: int = (total_moles * 1000) // total_v
    return result


def ionic_strength(concentrations: list[int], charges: list[int]) -> int:
    """Ionic strength I = 0.5 * sum(ci * zi^2). Scale 1000."""
    total: int = 0
    i: int = 0
    n: int = len(concentrations)
    if n > len(charges):
        n = len(charges)
    while i < n:
        c_val: int = concentrations[i]
        z_val: int = charges[i]
        total = total + (c_val * z_val * z_val) // 1000
        i = i + 1
    result: int = total // 2
    return result


def test_module() -> int:
    """Test molarity computations."""
    ok: int = 0
    m: int = molarity_from_mass(58440, 58440, 1000000)
    if m == 1000:
        ok = ok + 1
    n: int = normality(1000, 2000)
    if n == 2000:
        ok = ok + 1
    mol: int = moles_in_solution(1000, 500000)
    if mol == 500:
        ok = ok + 1
    sd: int = serial_dilution(1000, 10000, 2)
    if sd == 10:
        ok = ok + 1
    mc: int = mixing_concentration(1000, 500, 2000, 500)
    if mc == 1500:
        ok = ok + 1
    vfm: int = volume_for_moles(1000, 0)
    if vfm == 0:
        ok = ok + 1
    return ok
