"""Titration computations using integer arithmetic.

Tests: equivalence point, titrant volume, endpoint detection.
Scale factor 1000 for fixed-point.
"""


def equivalence_volume(conc_analyte: int, vol_analyte: int, conc_titrant: int) -> int:
    """Volume of titrant at equivalence: V_t = C_a * V_a / C_t. Scale 1000."""
    if conc_titrant == 0:
        return 0
    result: int = (conc_analyte * vol_analyte) // conc_titrant
    return result


def moles_at_equivalence(conc_analyte: int, vol_analyte: int) -> int:
    """Moles at equivalence = C_a * V_a. Scale 1000 (vol in mL*1000)."""
    result: int = (conc_analyte * vol_analyte) // 1000
    return result


def percent_titrated(vol_added: int, equiv_vol: int) -> int:
    """Percent of equivalence reached. Scale 1000."""
    if equiv_vol == 0:
        return 0
    result: int = (vol_added * 1000) // equiv_vol
    return result


def is_before_equivalence(vol_added: int, equiv_vol: int) -> int:
    """Returns 1 if before equivalence point."""
    if vol_added < equiv_vol:
        return 1
    return 0


def is_at_equivalence(vol_added: int, equiv_vol: int, tolerance: int) -> int:
    """Returns 1 if at equivalence point within tolerance."""
    diff: int = vol_added - equiv_vol
    if diff < 0:
        diff = 0 - diff
    if diff <= tolerance:
        return 1
    return 0


def is_after_equivalence(vol_added: int, equiv_vol: int) -> int:
    """Returns 1 if past equivalence point."""
    if vol_added > equiv_vol:
        return 1
    return 0


def excess_moles(vol_added: int, equiv_vol: int, conc_titrant: int) -> int:
    """Moles of excess titrant after equivalence. Scale 1000."""
    if vol_added <= equiv_vol:
        return 0
    excess_vol: int = vol_added - equiv_vol
    result: int = (conc_titrant * excess_vol) // 1000
    return result


def back_titration_moles(moles_added: int, moles_back: int) -> int:
    """Moles of analyte from back titration: n_analyte = n_added - n_back."""
    result: int = moles_added - moles_back
    if result < 0:
        result = 0
    return result


def concentration_from_titration(equiv_vol: int, conc_titrant: int, vol_analyte: int) -> int:
    """Analyte concentration: C_a = C_t * V_t / V_a. Scale 1000."""
    if vol_analyte == 0:
        return 0
    result: int = (conc_titrant * equiv_vol) // vol_analyte
    return result


def test_module() -> int:
    """Test titration computations."""
    ok: int = 0
    ev: int = equivalence_volume(100, 50000, 200)
    if ev == 25000:
        ok = ok + 1
    pt: int = percent_titrated(12500, 25000)
    if pt == 500:
        ok = ok + 1
    bef: int = is_before_equivalence(10000, 25000)
    if bef == 1:
        ok = ok + 1
    at: int = is_at_equivalence(25000, 25000, 100)
    if at == 1:
        ok = ok + 1
    aft: int = is_after_equivalence(30000, 25000)
    if aft == 1:
        ok = ok + 1
    ca: int = concentration_from_titration(25000, 200, 50000)
    if ca == 100:
        ok = ok + 1
    return ok
