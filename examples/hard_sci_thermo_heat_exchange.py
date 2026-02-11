"""Heat exchange and conduction computations using integer arithmetic.

Tests: conduction, convection, radiation, thermal resistance.
Scale factor 1000 for fixed-point.
"""


def heat_conduction(conductivity: int, area: int, delta_t: int, thickness: int) -> int:
    """Fourier's law: Q/t = k*A*dT/L. Scale 1000."""
    if thickness == 0:
        return 0
    ka: int = (conductivity * area) // 1000
    numer: int = (ka * delta_t) // 1000
    result: int = (numer * 1000) // thickness
    return result


def thermal_resistance(thickness: int, conductivity: int, area: int) -> int:
    """Thermal resistance R = L/(k*A). Scale 1000."""
    denom: int = (conductivity * area) // 1000
    if denom == 0:
        return 0
    result: int = (thickness * 1000) // denom
    return result


def series_thermal_resistance(resistances: list[int]) -> int:
    """Total resistance for series layers: R_total = sum(Ri)."""
    total: int = 0
    i: int = 0
    while i < len(resistances):
        val: int = resistances[i]
        total = total + val
        i = i + 1
    return total


def parallel_thermal_resistance(r1: int, r2: int) -> int:
    """Parallel thermal resistance: 1/R = 1/R1 + 1/R2. Scale 1000."""
    if r1 == 0 or r2 == 0:
        return 0
    inv1: int = (1000 * 1000) // r1
    inv2: int = (1000 * 1000) // r2
    inv_sum: int = inv1 + inv2
    if inv_sum == 0:
        return 0
    result: int = (1000 * 1000) // inv_sum
    return result


def convection_heat_transfer(h_coeff: int, area: int, t_surface: int, t_fluid: int) -> int:
    """Newton's law of cooling: Q = h*A*(Ts - Tf). Scale 1000."""
    dt: int = t_surface - t_fluid
    ha: int = (h_coeff * area) // 1000
    result: int = (ha * dt) // 1000
    return result


def radiation_heat_transfer(emissivity: int, area: int, t_surface: int, t_surround: int) -> int:
    """Stefan-Boltzmann: Q = eps*sigma*A*(Ts^4 - Tsur^4).
    sigma = 5670 (scale 10^-8 * 10^10 = 10^2 => /100 to normalize).
    Simplified: use T in relative units. Scale 1000."""
    t_s4: int = (t_surface * t_surface) // 1000
    t_s4 = (t_s4 * t_surface) // 1000
    t_s4 = (t_s4 * t_surface) // 1000
    t_r4: int = (t_surround * t_surround) // 1000
    t_r4 = (t_r4 * t_surround) // 1000
    t_r4 = (t_r4 * t_surround) // 1000
    diff_t4: int = t_s4 - t_r4
    ea: int = (emissivity * area) // 1000
    sigma_ea: int = (5670 * ea) // 1000
    numer: int = (sigma_ea * diff_t4) // 1000
    result: int = numer // 100
    return result


def overall_heat_transfer_coeff(h_inside: int, wall_r: int, h_outside: int) -> int:
    """Overall U = 1/(1/hi + R_wall + 1/ho). Scale 1000."""
    if h_inside == 0 or h_outside == 0:
        return 0
    inv_hi: int = (1000 * 1000) // h_inside
    inv_ho: int = (1000 * 1000) // h_outside
    inv_sum: int = inv_hi + wall_r + inv_ho
    if inv_sum == 0:
        return 0
    result: int = (1000 * 1000) // inv_sum
    return result


def log_mean_temp_diff(dt1: int, dt2: int) -> int:
    """LMTD = (dT1 - dT2) / ln(dT1/dT2).
    ln approx for ratio near 1. Scale 1000."""
    if dt1 <= 0 or dt2 <= 0:
        return 0
    if dt1 == dt2:
        return dt1
    diff: int = dt1 - dt2
    ratio: int = (dt1 * 1000) // dt2
    x: int = ratio - 1000
    ln_val: int = x - (x * x) // 2000
    if ln_val == 0:
        return (dt1 + dt2) // 2
    result: int = (diff * 1000) // ln_val
    return result


def test_module() -> int:
    """Test heat exchange computations."""
    ok: int = 0
    q_cond: int = heat_conduction(50000, 1000, 50000, 100)
    if q_cond > 24000 and q_cond < 26000:
        ok = ok + 1
    r: int = thermal_resistance(100, 50000, 1000)
    if r > 1 and r < 3:
        ok = ok + 1
    rs: int = series_thermal_resistance([100, 200, 300])
    if rs == 600:
        ok = ok + 1
    rp: int = parallel_thermal_resistance(1000, 1000)
    if rp == 500:
        ok = ok + 1
    lmtd: int = log_mean_temp_diff(1000, 1000)
    if lmtd == 1000:
        ok = ok + 1
    zero_t: int = thermal_resistance(100, 0, 1000)
    if zero_t == 0:
        ok = ok + 1
    return ok
