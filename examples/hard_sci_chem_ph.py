"""pH and acid-base computations using integer arithmetic.

Tests: pH, pOH, H+ concentration, buffer capacity.
Scale factor 1000 for fixed-point.
"""


def ph_from_h_concentration(h_conc_exp: int) -> int:
    """pH = -log10([H+]). h_conc_exp is -log10([H+]) * 1000 directly.
    For simplicity, pH is passed as the negative log value. Scale 1000."""
    return h_conc_exp


def poh_from_ph(ph_val: int) -> int:
    """pOH = 14000 - pH (at 25C). Scale 1000 (pH*1000)."""
    return 14000 - ph_val


def h_concentration_class(ph_val: int) -> int:
    """Classify by pH: 1=acidic (pH<7), 2=neutral (pH=7), 3=basic (pH>7).
    pH in scale 1000."""
    if ph_val < 7000:
        return 1
    if ph_val == 7000:
        return 2
    return 3


def is_strong_acid(ph_val: int) -> int:
    """Returns 1 if strongly acidic (pH < 3). Scale 1000."""
    if ph_val < 3000:
        return 1
    return 0


def is_strong_base(ph_val: int) -> int:
    """Returns 1 if strongly basic (pH > 11). Scale 1000."""
    if ph_val > 11000:
        return 1
    return 0


def buffer_ph(pka: int, conc_acid: int, conc_salt: int) -> int:
    """Henderson-Hasselbalch: pH = pKa + log10([A-]/[HA]).
    log10(x) ~ (x-1)/2.303 ~ (x-1000)*434/1000 for x near 1000.
    Scale 1000."""
    if conc_acid == 0:
        return pka + 5000
    ratio: int = (conc_salt * 1000) // conc_acid
    log_approx: int = ((ratio - 1000) * 434) // 1000
    return pka + log_approx


def dilution_ph_shift(initial_ph: int, dilution_factor: int) -> int:
    """Approximate pH shift on dilution of strong acid.
    New pH ~ initial_pH + log10(dilution_factor).
    log10(x) ~ (x-1000)*434/1000. Scale 1000."""
    log_val: int = ((dilution_factor - 1000) * 434) // 1000
    result: int = initial_ph + log_val
    if result > 7000:
        result = 7000
    return result


def neutralization_heat(moles_acid: int, moles_base: int) -> int:
    """Heat of neutralization (strong acid + strong base).
    dH ~ -57100 J/mol. Returns heat for limiting moles * 1000."""
    limiting: int = moles_acid
    if moles_base < moles_acid:
        limiting = moles_base
    result: int = (limiting * 57100) // 1000
    return result


def ka_from_pka(pka: int) -> int:
    """Ka = 10^(-pKa). For integer approx:
    10^(-x) ~ 1000 * exp(-2.303 * x/1000).
    exp(-u) ~ 1 - u + u^2/2. Scale 1000."""
    u: int = (2303 * pka) // 1000
    if u > 10000:
        return 0
    exp_val: int = 1000 - u + (u * u) // 2000
    if exp_val < 0:
        return 0
    return exp_val


def test_module() -> int:
    """Test pH computations."""
    ok: int = 0
    p: int = ph_from_h_concentration(7000)
    if p == 7000:
        ok = ok + 1
    po: int = poh_from_ph(7000)
    if po == 7000:
        ok = ok + 1
    cls: int = h_concentration_class(3000)
    if cls == 1:
        ok = ok + 1
    cls2: int = h_concentration_class(7000)
    if cls2 == 2:
        ok = ok + 1
    cls3: int = h_concentration_class(11000)
    if cls3 == 3:
        ok = ok + 1
    sa: int = is_strong_acid(2000)
    if sa == 1:
        ok = ok + 1
    sb: int = is_strong_base(12000)
    if sb == 1:
        ok = ok + 1
    return ok
