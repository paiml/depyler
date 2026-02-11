"""Pressure and fluid statics computations using integer arithmetic.

Tests: hydrostatic pressure, gauge pressure, Pascal's law, manometer.
Scale factor 1000 for fixed-point.
"""


def hydrostatic_pressure(density: int, gravity: int, height: int) -> int:
    """Hydrostatic pressure P = rho*g*h. Scale 1000."""
    result: int = (density * gravity * height) // (1000 * 1000)
    return result


def absolute_pressure(gauge: int, atmospheric: int) -> int:
    """Absolute pressure = gauge + atmospheric."""
    return gauge + atmospheric


def gauge_pressure(absolute_p: int, atmospheric: int) -> int:
    """Gauge pressure = absolute - atmospheric."""
    return absolute_p - atmospheric


def pascal_force(pressure: int, area: int) -> int:
    """Force from pressure: F = P*A. Scale 1000."""
    result: int = (pressure * area) // 1000
    return result


def hydraulic_advantage(area_large: int, area_small: int) -> int:
    """Hydraulic advantage = A_large / A_small. Scale 1000."""
    if area_small == 0:
        return 0
    result: int = (area_large * 1000) // area_small
    return result


def manometer_pressure_diff(density: int, gravity: int, height_diff: int) -> int:
    """Pressure difference from manometer: dP = rho*g*dh. Scale 1000."""
    result: int = (density * gravity * height_diff) // (1000 * 1000)
    return result


def barometric_formula(p0: int, molar_mass: int, gravity: int, height: int, temp: int) -> int:
    """Barometric formula: P = P0 * exp(-Mgh/RT).
    R = 8314. exp(-x) ~ 1 - x for small x. Scale 1000."""
    if temp == 0:
        return p0
    r_const: int = 8314
    exponent: int = (molar_mass * gravity * height) // (r_const * temp)
    exp_val: int = 1000 - exponent + (exponent * exponent) // 2000
    if exp_val < 0:
        exp_val = 0
    result: int = (p0 * exp_val) // 1000
    return result


def buoyant_force(fluid_density: int, gravity: int, displaced_volume: int) -> int:
    """Buoyant force F = rho*g*V. Scale 1000."""
    result: int = (fluid_density * gravity * displaced_volume) // (1000 * 1000)
    return result


def pressure_at_depth(surface_p: int, density: int, gravity: int, depth: int) -> int:
    """Total pressure at depth = P_surface + rho*g*h. Scale 1000."""
    hydro: int = (density * gravity * depth) // (1000 * 1000)
    result: int = surface_p + hydro
    return result


def test_module() -> int:
    """Test pressure computations."""
    ok: int = 0
    hp: int = hydrostatic_pressure(1000, 9810, 10000)
    if hp > 97000 and hp < 99000:
        ok = ok + 1
    ap: int = absolute_pressure(50000, 101325)
    if ap == 151325:
        ok = ok + 1
    gp: int = gauge_pressure(151325, 101325)
    if gp == 50000:
        ok = ok + 1
    pf: int = pascal_force(100000, 500)
    if pf == 50000:
        ok = ok + 1
    ha: int = hydraulic_advantage(10000, 100)
    if ha == 100000:
        ok = ok + 1
    zero_check: int = hydraulic_advantage(1000, 0)
    if zero_check == 0:
        ok = ok + 1
    return ok
