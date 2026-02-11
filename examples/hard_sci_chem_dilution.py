"""Dilution computations using integer arithmetic.

Tests: dilution formula, serial dilution, concentration factors.
Scale factor 1000 for fixed-point.
"""


def dilution_volume(c1: int, v1: int, c2: int) -> int:
    """Dilution formula C1*V1 = C2*V2. Solve for V2. Scale 1000."""
    if c2 == 0:
        return 0
    result: int = (c1 * v1) // c2
    return result


def dilution_concentration(c1: int, v1: int, v2: int) -> int:
    """Dilution formula: C2 = C1*V1/V2. Scale 1000."""
    if v2 == 0:
        return 0
    result: int = (c1 * v1) // v2
    return result


def volume_of_solvent_to_add(v1: int, v2: int) -> int:
    """Volume of solvent to add = V2 - V1."""
    result: int = v2 - v1
    if result < 0:
        result = 0
    return result


def concentration_factor(c_final: int, c_initial: int) -> int:
    """Concentration factor = C_final / C_initial. Scale 1000."""
    if c_initial == 0:
        return 0
    result: int = (c_final * 1000) // c_initial
    return result


def serial_dilution_conc(initial: int, dilution_ratio: int, steps: int) -> int:
    """Concentration after n serial dilutions. Scale 1000."""
    conc: int = initial
    i: int = 0
    while i < steps:
        conc = (conc * 1000) // dilution_ratio
        i = i + 1
    return conc


def serial_dilution_table(initial: int, dilution_ratio: int, num_steps: int) -> list[int]:
    """Generate concentration table for serial dilution."""
    result: list[int] = []
    conc: int = initial
    i: int = 0
    while i <= num_steps:
        result.append(conc)
        conc = (conc * 1000) // dilution_ratio
        i = i + 1
    return result


def dilution_fold(v_sample: int, v_total: int) -> int:
    """Dilution fold = V_total / V_sample. Scale 1000."""
    if v_sample == 0:
        return 0
    result: int = (v_total * 1000) // v_sample
    return result


def stock_volume_needed(desired_conc: int, desired_vol: int, stock_conc: int) -> int:
    """Volume of stock solution: V_stock = C_desired * V_desired / C_stock. Scale 1000."""
    if stock_conc == 0:
        return 0
    result: int = (desired_conc * desired_vol) // stock_conc
    return result


def mass_concentration(molarity_val: int, molar_mass: int) -> int:
    """Mass concentration g/L = M * MM. Scale 1000."""
    result: int = (molarity_val * molar_mass) // 1000
    return result


def test_module() -> int:
    """Test dilution computations."""
    ok: int = 0
    v2: int = dilution_volume(1000, 100, 500)
    if v2 == 200:
        ok = ok + 1
    c2: int = dilution_concentration(1000, 100, 200)
    if c2 == 500:
        ok = ok + 1
    vs: int = volume_of_solvent_to_add(100, 200)
    if vs == 100:
        ok = ok + 1
    cf: int = concentration_factor(2000, 1000)
    if cf == 2000:
        ok = ok + 1
    sd: int = serial_dilution_conc(1000, 10000, 1)
    if sd == 100:
        ok = ok + 1
    sv: int = stock_volume_needed(100, 1000, 1000)
    if sv == 100:
        ok = ok + 1
    df: int = dilution_fold(100, 1000)
    if df == 10000:
        ok = ok + 1
    return ok
