"""Ohm's law and basic circuit computations using integer arithmetic.

Tests: voltage, current, resistance, power, series/parallel.
Scale factor 1000 for fixed-point.
"""


def voltage(current: int, resistance: int) -> int:
    """Ohm's law: V = I*R. Scale 1000."""
    result: int = (current * resistance) // 1000
    return result


def current_from_vr(volt: int, resistance: int) -> int:
    """Current from Ohm's law: I = V/R. Scale 1000."""
    if resistance == 0:
        return 0
    result: int = (volt * 1000) // resistance
    return result


def resistance_from_vi(volt: int, current_val: int) -> int:
    """Resistance from Ohm's law: R = V/I. Scale 1000."""
    if current_val == 0:
        return 0
    result: int = (volt * 1000) // current_val
    return result


def power_vi(volt: int, current_val: int) -> int:
    """Electrical power: P = V*I. Scale 1000."""
    result: int = (volt * current_val) // 1000
    return result


def power_i2r(current_val: int, resistance: int) -> int:
    """Power: P = I^2 * R. Scale 1000."""
    i_sq: int = (current_val * current_val) // 1000
    result: int = (i_sq * resistance) // 1000
    return result


def power_v2r(volt: int, resistance: int) -> int:
    """Power: P = V^2 / R. Scale 1000."""
    if resistance == 0:
        return 0
    v_sq: int = (volt * volt) // 1000
    result: int = (v_sq * 1000) // resistance
    return result


def series_resistance(resistances: list[int]) -> int:
    """Total resistance in series: R = R1 + R2 + ..."""
    total: int = 0
    i: int = 0
    while i < len(resistances):
        val: int = resistances[i]
        total = total + val
        i = i + 1
    return total


def parallel_resistance_two(r1: int, r2: int) -> int:
    """Parallel resistance: R = R1*R2/(R1+R2). Scale 1000."""
    denom: int = r1 + r2
    if denom == 0:
        return 0
    result: int = (r1 * r2) // denom
    return result


def voltage_divider(v_in: int, r1: int, r2: int) -> int:
    """Voltage divider: V_out = V_in * R2 / (R1+R2). Scale 1000."""
    denom: int = r1 + r2
    if denom == 0:
        return 0
    result: int = (v_in * r2) // denom
    return result


def current_divider(i_total: int, r1: int, r2: int) -> int:
    """Current through R1 in parallel: I1 = I_total * R2/(R1+R2). Scale 1000."""
    denom: int = r1 + r2
    if denom == 0:
        return 0
    result: int = (i_total * r2) // denom
    return result


def test_module() -> int:
    """Test Ohm's law computations."""
    ok: int = 0
    v: int = voltage(2000, 5000)
    if v == 10000:
        ok = ok + 1
    i_val: int = current_from_vr(10000, 5000)
    if i_val == 2000:
        ok = ok + 1
    r: int = resistance_from_vi(10000, 2000)
    if r == 5000:
        ok = ok + 1
    p: int = power_vi(10000, 2000)
    if p == 20000:
        ok = ok + 1
    sr: int = series_resistance([1000, 2000, 3000])
    if sr == 6000:
        ok = ok + 1
    pr: int = parallel_resistance_two(2000, 2000)
    if pr == 1000:
        ok = ok + 1
    vd: int = voltage_divider(12000, 6000, 6000)
    if vd == 6000:
        ok = ok + 1
    return ok
