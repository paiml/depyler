"""Calorimetry computations using integer arithmetic.

Tests: heat exchange, thermal equilibrium, calorimeter constant.
Scale factor 1000 for fixed-point.
"""


def equilibrium_temperature(m1: int, c1: int, t1: int, m2: int, c2: int, t2: int) -> int:
    """Equilibrium temperature when two bodies are mixed.
    T_eq = (m1*c1*T1 + m2*c2*T2) / (m1*c1 + m2*c2). Scale 1000."""
    numer: int = m1 * c1 * t1 + m2 * c2 * t2
    denom: int = m1 * c1 + m2 * c2
    if denom == 0:
        return 0
    result: int = numer // denom
    return result


def heat_gained(mass: int, specific_heat: int, t_initial: int, t_final: int) -> int:
    """Heat gained: Q = m*c*(Tf - Ti). Scale 1000."""
    dt: int = t_final - t_initial
    result: int = (mass * specific_heat * dt) // (1000 * 1000)
    return result


def calorimeter_constant(heat_released: int, delta_temp: int) -> int:
    """Calorimeter constant C_cal = Q / dT. Scale 1000."""
    if delta_temp == 0:
        return 0
    result: int = (heat_released * 1000) // delta_temp
    return result


def heat_with_calorimeter(mass: int, specific_heat: int, cal_const: int, delta_t: int) -> int:
    """Total heat including calorimeter: Q = (m*c + C_cal) * dT. Scale 1000."""
    mc: int = (mass * specific_heat) // 1000
    total_cap: int = mc + cal_const
    result: int = (total_cap * delta_t) // 1000
    return result


def water_equivalent(cal_const: int, c_water: int) -> int:
    """Water equivalent = C_cal / c_water. Scale 1000."""
    if c_water == 0:
        return 0
    result: int = (cal_const * 1000) // c_water
    return result


def heat_of_combustion(mass_fuel: int, cal_const: int, water_mass: int, c_water: int, delta_t: int) -> int:
    """Heat of combustion per unit mass.
    Q = (C_cal + m_w*c_w) * dT / m_fuel. Scale 1000."""
    if mass_fuel == 0:
        return 0
    mc_water: int = (water_mass * c_water) // 1000
    total_cap: int = cal_const + mc_water
    q_total: int = (total_cap * delta_t) // 1000
    result: int = (q_total * 1000) // mass_fuel
    return result


def specific_heat_from_experiment(heat: int, mass: int, delta_t: int) -> int:
    """Determine specific heat: c = Q / (m * dT). Scale 1000."""
    denom: int = (mass * delta_t) // 1000
    if denom == 0:
        return 0
    result: int = (heat * 1000) // denom
    return result


def test_module() -> int:
    """Test calorimetry computations."""
    ok: int = 0
    t_eq: int = equilibrium_temperature(1000, 4186, 80, 2000, 4186, 20)
    if t_eq > 39 and t_eq < 41:
        ok = ok + 1
    q_gain: int = heat_gained(1000, 4186, 20000, 30000)
    if q_gain > 41000 and q_gain < 42000:
        ok = ok + 1
    cal_c: int = calorimeter_constant(5000, 10000)
    if cal_c == 500:
        ok = ok + 1
    we: int = water_equivalent(4186, 4186)
    if we == 1000:
        ok = ok + 1
    sc: int = specific_heat_from_experiment(4186, 1000, 1000)
    if sc == 4186:
        ok = ok + 1
    zero_check: int = calorimeter_constant(1000, 0)
    if zero_check == 0:
        ok = ok + 1
    return ok
