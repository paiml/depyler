"""Capacitor computations using integer arithmetic.

Tests: capacitance, energy, parallel plate, dielectric.
Scale factor 1000 for fixed-point.
"""


def parallel_plate_capacitance(epsilon_r: int, area: int, separation: int) -> int:
    """Capacitance C = epsilon_r * epsilon_0 * A / d.
    epsilon_0 = 8854 (scale 10^-12 * 10^15 = 10^3). Scale 1000."""
    if separation == 0:
        return 0
    result: int = (epsilon_r * 8854 * area) // (separation * 1000 * 1000)
    return result


def capacitor_charge_stored(capacitance: int, volt: int) -> int:
    """Charge Q = C*V. Scale 1000."""
    result: int = (capacitance * volt) // 1000
    return result


def capacitor_energy_stored(capacitance: int, volt: int) -> int:
    """Energy E = 0.5*C*V^2. Scale 1000."""
    v_sq: int = (volt * volt) // 1000
    result: int = (capacitance * v_sq) // 2000
    return result


def electric_field_parallel_plate(volt: int, separation: int) -> int:
    """Electric field E = V/d. Scale 1000."""
    if separation == 0:
        return 0
    result: int = (volt * 1000) // separation
    return result


def series_capacitance(caps: list[int]) -> int:
    """Series capacitance: 1/C = 1/C1 + 1/C2 + ... Scale 1000."""
    inv_sum: int = 0
    i: int = 0
    while i < len(caps):
        val: int = caps[i]
        if val == 0:
            return 0
        inv_sum = inv_sum + (1000 * 1000) // val
        i = i + 1
    if inv_sum == 0:
        return 0
    result: int = (1000 * 1000) // inv_sum
    return result


def parallel_capacitance_sum(caps: list[int]) -> int:
    """Parallel capacitance: C = C1 + C2 + ..."""
    total: int = 0
    i: int = 0
    while i < len(caps):
        val: int = caps[i]
        total = total + val
        i = i + 1
    return total


def dielectric_capacitance(c_vacuum: int, epsilon_r: int) -> int:
    """Capacitance with dielectric: C = epsilon_r * C0. Scale 1000."""
    result: int = (epsilon_r * c_vacuum) // 1000
    return result


def time_to_charge_percent(tau: int, percent: int) -> int:
    """Time to charge to given percent.
    t = -tau * ln(1 - percent/1000).
    ln(1-x) ~ -x - x^2/2. Scale 1000."""
    x: int = percent
    if x >= 1000:
        return 5 * tau
    ln_val: int = 0 - x - (x * x) // 2000
    result: int = (0 - tau * ln_val) // 1000
    return result


def voltage_across_capacitor_series(c1: int, c2: int, v_total: int) -> int:
    """Voltage across C1 in series: V1 = V_total * C2/(C1+C2). Scale 1000."""
    denom: int = c1 + c2
    if denom == 0:
        return 0
    result: int = (v_total * c2) // denom
    return result


def test_module() -> int:
    """Test capacitor computations."""
    ok: int = 0
    q: int = capacitor_charge_stored(1000, 5000)
    if q == 5000:
        ok = ok + 1
    e: int = capacitor_energy_stored(1000, 5000)
    if e == 12500:
        ok = ok + 1
    ef: int = electric_field_parallel_plate(1000, 500)
    if ef == 2000:
        ok = ok + 1
    sc: int = series_capacitance([2000, 2000])
    if sc == 1000:
        ok = ok + 1
    pc: int = parallel_capacitance_sum([1000, 2000, 3000])
    if pc == 6000:
        ok = ok + 1
    vc: int = voltage_across_capacitor_series(1000, 1000, 10000)
    if vc == 5000:
        ok = ok + 1
    return ok
