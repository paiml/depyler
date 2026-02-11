"""Phase change and latent heat computations using integer arithmetic.

Tests: latent heat, phase transition energy, Clausius-Clapeyron.
Scale factor 1000 for fixed-point.
"""


def heat_for_phase_change(mass: int, latent_heat: int) -> int:
    """Heat required for phase change: Q = m * L. Scale 1000."""
    result: int = (mass * latent_heat) // 1000
    return result


def heat_for_temp_change(mass: int, specific_heat: int, delta_t: int) -> int:
    """Heat for temperature change: Q = m * c * dT. Scale 1000."""
    result: int = (mass * specific_heat * delta_t) // (1000 * 1000)
    return result


def total_heat_ice_to_steam(mass: int) -> int:
    """Total heat to convert ice at 0C to steam at 100C.
    L_fusion = 334000, c_water = 4186, L_vap = 2260000. Scale 1000."""
    l_fusion: int = 334000
    c_water: int = 4186
    l_vap: int = 2260000
    q_melt: int = (mass * l_fusion) // 1000
    q_heat: int = (mass * c_water * 100) // (1000)
    q_vap: int = (mass * l_vap) // 1000
    total: int = q_melt + q_heat + q_vap
    return total


def clausius_clapeyron_slope(latent_heat: int, temp: int, delta_v: int) -> int:
    """Clausius-Clapeyron slope: dP/dT = L/(T*dV). Scale 1000."""
    denom: int = (temp * delta_v) // 1000
    if denom == 0:
        return 0
    result: int = (latent_heat * 1000) // denom
    return result


def boiling_point_elevation(kb_const: int, molality: int) -> int:
    """Boiling point elevation: dTb = Kb * m. Scale 1000."""
    result: int = (kb_const * molality) // 1000
    return result


def freezing_point_depression(kf_const: int, molality: int) -> int:
    """Freezing point depression: dTf = Kf * m. Scale 1000."""
    result: int = (kf_const * molality) // 1000
    return result


def vapor_pressure_ratio(latent_heat: int, t1: int, t2: int) -> int:
    """Ratio of vapor pressures P2/P1 using Clausius-Clapeyron.
    ln(P2/P1) = (L/R)*(1/T1 - 1/T2). R = 8314. Scale 1000.
    Returns the ratio * 1000."""
    if t1 == 0 or t2 == 0:
        return 1000
    r_const: int = 8314
    inv_t1: int = (1000 * 1000) // t1
    inv_t2: int = (1000 * 1000) // t2
    ln_ratio: int = (latent_heat * (inv_t1 - inv_t2)) // r_const
    exp_val: int = 1000 + ln_ratio + (ln_ratio * ln_ratio) // 2000
    if exp_val < 0:
        exp_val = 0
    return exp_val


def specific_heat_ratio(cp: int, cv: int) -> int:
    """Ratio of specific heats gamma = Cp/Cv. Scale 1000."""
    if cv == 0:
        return 0
    result: int = (cp * 1000) // cv
    return result


def test_module() -> int:
    """Test phase change computations."""
    ok: int = 0
    q_pc: int = heat_for_phase_change(1000, 334000)
    if q_pc == 334000:
        ok = ok + 1
    q_tc: int = heat_for_temp_change(1000, 4186, 10000)
    if q_tc > 41000 and q_tc < 42000:
        ok = ok + 1
    bpe: int = boiling_point_elevation(512, 1000)
    if bpe == 512:
        ok = ok + 1
    fpd: int = freezing_point_depression(1860, 500)
    if fpd == 930:
        ok = ok + 1
    gamma: int = specific_heat_ratio(1005, 718)
    if gamma > 1390 and gamma < 1410:
        ok = ok + 1
    zero_check: int = clausius_clapeyron_slope(1000, 0, 100)
    if zero_check == 0:
        ok = ok + 1
    return ok
