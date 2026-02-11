"""Entropy computations using integer arithmetic.

Tests: entropy change, mixing entropy, statistical entropy, irreversibility.
Scale factor 1000 for fixed-point.
"""


def entropy_heating(mass: int, specific_heat: int, t_initial: int, t_final: int) -> int:
    """Entropy change for heating: dS = m*c*ln(Tf/Ti).
    ln approx: ln(1+x) ~ x - x^2/2 for small x.
    For larger ratios, use repeated halving. Scale 1000."""
    if t_initial <= 0 or t_final <= 0:
        return 0
    ratio: int = (t_final * 1000) // t_initial
    ln_val: int = 0
    temp_ratio: int = ratio
    scale_count: int = 0
    while temp_ratio > 2000:
        temp_ratio = temp_ratio // 2
        scale_count = scale_count + 1
    x: int = temp_ratio - 1000
    ln_val = x - (x * x) // 2000
    ln_val = ln_val + scale_count * 693
    result: int = (mass * specific_heat * ln_val) // (1000 * 1000)
    return result


def entropy_isothermal_expansion(moles: int, v_final: int, v_initial: int) -> int:
    """Entropy change for isothermal expansion: dS = nR*ln(Vf/Vi).
    R = 8314. Scale 1000."""
    if v_initial <= 0 or v_final <= 0:
        return 0
    ratio: int = (v_final * 1000) // v_initial
    x: int = ratio - 1000
    ln_val: int = x - (x * x) // 2000
    r_const: int = 8314
    result: int = (moles * r_const * ln_val) // (1000 * 1000)
    return result


def entropy_mixing_ideal(n1: int, n2: int) -> int:
    """Entropy of mixing for two ideal gases.
    dS_mix = -R*(n1*ln(x1) + n2*ln(x2)) where xi = ni/(n1+n2).
    Scale 1000. R = 8314."""
    total: int = n1 + n2
    if total == 0:
        return 0
    r_const: int = 8314
    x1: int = (n1 * 1000) // total
    x2: int = (n2 * 1000) // total
    ln_x1: int = 0
    if x1 > 0:
        arg1: int = x1 - 1000
        ln_x1 = arg1 - (arg1 * arg1) // 2000
    ln_x2: int = 0
    if x2 > 0:
        arg2: int = x2 - 1000
        ln_x2 = arg2 - (arg2 * arg2) // 2000
    ds: int = (0 - r_const) * (n1 * ln_x1 + n2 * ln_x2) // (1000 * 1000 * 1000)
    return ds


def clausius_inequality(q_hot: int, t_hot: int, q_cold: int, t_cold: int) -> int:
    """Clausius inequality: dS_univ = -Qh/Th + Qc/Tc.
    Returns dS * 1000. Positive means irreversible. Scale 1000."""
    if t_hot == 0 or t_cold == 0:
        return 0
    term_hot: int = (0 - q_hot * 1000) // t_hot
    term_cold: int = (q_cold * 1000) // t_cold
    result: int = term_hot + term_cold
    return result


def boltzmann_entropy(num_microstates: int) -> int:
    """Boltzmann entropy S = kB * ln(W).
    kB = 1381 (scale 10^-20). ln approx for integers.
    Scale 1000."""
    if num_microstates <= 0:
        return 0
    if num_microstates == 1:
        return 0
    ln_val: int = 0
    w: int = num_microstates
    while w > 2:
        w = w // 2
        ln_val = ln_val + 693
    if w == 2:
        ln_val = ln_val + 693
    result: int = (1381 * ln_val) // 1000
    return result


def gibbs_free_energy(enthalpy: int, temp: int, entropy: int) -> int:
    """Gibbs free energy G = H - TS. Scale 1000."""
    result: int = enthalpy - (temp * entropy) // 1000
    return result


def test_module() -> int:
    """Test entropy computations."""
    ok: int = 0
    ds_iso: int = entropy_isothermal_expansion(1000, 2000, 1000)
    if ds_iso > 4000 and ds_iso < 5000:
        ok = ok + 1
    ci: int = clausius_inequality(1000, 500, 1000, 500)
    if ci == 0:
        ok = ok + 1
    be: int = boltzmann_entropy(1)
    if be == 0:
        ok = ok + 1
    gfe: int = gibbs_free_energy(5000, 300, 10)
    if gfe > 4990 and gfe < 5010:
        ok = ok + 1
    ci2: int = clausius_inequality(1000, 600, 800, 300)
    ds_val: int = ci2
    if ds_val > 0:
        ok = ok + 1
    zero_check: int = entropy_isothermal_expansion(0, 2000, 1000)
    if zero_check == 0:
        ok = ok + 1
    return ok
