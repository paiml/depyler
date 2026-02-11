"""Transformer computations using integer arithmetic.

Tests: turns ratio, voltage/current transformation, efficiency, regulation.
Scale factor 1000 for fixed-point.
"""


def turns_ratio(n_primary: int, n_secondary: int) -> int:
    """Turns ratio a = Np/Ns. Scale 1000."""
    if n_secondary == 0:
        return 0
    result: int = (n_primary * 1000) // n_secondary
    return result


def secondary_voltage(v_primary: int, n_primary: int, n_secondary: int) -> int:
    """Secondary voltage: Vs = Vp * Ns/Np. Scale 1000."""
    if n_primary == 0:
        return 0
    result: int = (v_primary * n_secondary) // n_primary
    return result


def secondary_current(i_primary: int, n_primary: int, n_secondary: int) -> int:
    """Secondary current: Is = Ip * Np/Ns. Scale 1000."""
    if n_secondary == 0:
        return 0
    result: int = (i_primary * n_primary) // n_secondary
    return result


def transformer_efficiency(p_output: int, p_input: int) -> int:
    """Transformer efficiency = Pout/Pin * 1000. Scale 1000."""
    if p_input == 0:
        return 0
    result: int = (p_output * 1000) // p_input
    return result


def voltage_regulation(v_no_load: int, v_full_load: int) -> int:
    """Voltage regulation = (Vnl - Vfl)/Vfl * 1000. Scale 1000."""
    if v_full_load == 0:
        return 0
    result: int = ((v_no_load - v_full_load) * 1000) // v_full_load
    return result


def referred_impedance(z_secondary: int, turns_ratio_val: int) -> int:
    """Impedance referred to primary: Z' = a^2 * Z. Scale 1000."""
    a_sq: int = (turns_ratio_val * turns_ratio_val) // 1000
    result: int = (a_sq * z_secondary) // 1000
    return result


def core_loss(hysteresis: int, eddy_current: int) -> int:
    """Total core loss = hysteresis + eddy current losses."""
    return hysteresis + eddy_current


def copper_loss(i_primary: int, r_primary: int, i_secondary: int, r_secondary: int) -> int:
    """Copper losses: Pcu = Ip^2*Rp + Is^2*Rs. Scale 1000."""
    ip_sq: int = (i_primary * i_primary) // 1000
    is_sq: int = (i_secondary * i_secondary) // 1000
    loss_p: int = (ip_sq * r_primary) // 1000
    loss_s: int = (is_sq * r_secondary) // 1000
    return loss_p + loss_s


def auto_transformer_ratio(v_common: int, v_series: int) -> int:
    """Auto-transformer turns ratio. Scale 1000."""
    total: int = v_common + v_series
    if total == 0:
        return 0
    result: int = (total * 1000) // v_common
    if v_common == 0:
        return 0
    return result


def test_module() -> int:
    """Test transformer computations."""
    ok: int = 0
    tr: int = turns_ratio(1000, 500)
    if tr == 2000:
        ok = ok + 1
    vs: int = secondary_voltage(240000, 1000, 500)
    if vs == 120000:
        ok = ok + 1
    ic: int = secondary_current(1000, 1000, 500)
    if ic == 2000:
        ok = ok + 1
    eff: int = transformer_efficiency(950, 1000)
    if eff == 950:
        ok = ok + 1
    vr: int = voltage_regulation(12500, 12000)
    if vr > 40 and vr < 43:
        ok = ok + 1
    cl: int = core_loss(100, 50)
    if cl == 150:
        ok = ok + 1
    return ok
