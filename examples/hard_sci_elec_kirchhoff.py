"""Kirchhoff's laws computations using integer arithmetic.

Tests: KVL, KCL, node voltages, mesh currents, superposition.
Scale factor 1000 for fixed-point.
"""


def kvl_check(voltages: list[int]) -> int:
    """Check KVL: sum of voltages around loop should be 0.
    Returns absolute sum (0 means KVL satisfied)."""
    total: int = 0
    i: int = 0
    while i < len(voltages):
        val: int = voltages[i]
        total = total + val
        i = i + 1
    if total < 0:
        total = 0 - total
    return total


def kcl_check(currents: list[int]) -> int:
    """Check KCL: sum of currents at node should be 0.
    Returns absolute sum (0 means KCL satisfied)."""
    total: int = 0
    i: int = 0
    while i < len(currents):
        val: int = currents[i]
        total = total + val
        i = i + 1
    if total < 0:
        total = 0 - total
    return total


def two_loop_solve_i1(v1: int, r1: int, r2: int, r3: int, v2: int) -> int:
    """Solve 2-loop circuit for I1.
    Loop 1: V1 = I1*R1 + (I1-I2)*R2
    Loop 2: V2 = I2*R3 + (I2-I1)*R2
    I1 = (V1*(R2+R3) + V2*R2) / (R1*R2 + R1*R3 + R2*R3). Scale 1000."""
    denom: int = r1 * r2 + r1 * r3 + r2 * r3
    if denom == 0:
        return 0
    numer: int = v1 * (r2 + r3) + v2 * r2
    result: int = (numer * 1000) // denom
    return result


def two_loop_solve_i2(v1: int, r1: int, r2: int, r3: int, v2: int) -> int:
    """Solve 2-loop circuit for I2.
    I2 = (V2*(R1+R2) + V1*R2) / (R1*R2 + R1*R3 + R2*R3). Scale 1000."""
    denom: int = r1 * r2 + r1 * r3 + r2 * r3
    if denom == 0:
        return 0
    numer: int = v2 * (r1 + r2) + v1 * r2
    result: int = (numer * 1000) // denom
    return result


def node_voltage_two_source(v1: int, r1: int, v2: int, r2: int, r_load: int) -> int:
    """Node voltage with two sources: V_node = (V1/R1 + V2/R2) / (1/R1 + 1/R2 + 1/RL).
    Scale 1000."""
    if r1 == 0 or r2 == 0 or r_load == 0:
        return 0
    numer: int = (v1 * 1000) // r1 + (v2 * 1000) // r2
    denom: int = (1000 * 1000) // r1 + (1000 * 1000) // r2 + (1000 * 1000) // r_load
    if denom == 0:
        return 0
    result: int = (numer * 1000) // denom
    return result


def thevenin_voltage(v_open: int) -> int:
    """Thevenin equivalent voltage = open circuit voltage."""
    return v_open


def thevenin_resistance(v_open: int, i_short: int) -> int:
    """Thevenin equivalent resistance = Voc/Isc. Scale 1000."""
    if i_short == 0:
        return 0
    result: int = (v_open * 1000) // i_short
    return result


def norton_current(i_short: int) -> int:
    """Norton equivalent current = short circuit current."""
    return i_short


def max_power_transfer(v_th: int, r_th: int) -> int:
    """Maximum power transfer: P_max = Vth^2 / (4*Rth). Scale 1000."""
    if r_th == 0:
        return 0
    v_sq: int = (v_th * v_th) // 1000
    result: int = (v_sq * 1000) // (4 * r_th)
    return result


def test_module() -> int:
    """Test Kirchhoff's law computations."""
    ok: int = 0
    kvl: int = kvl_check([5000, 0 - 3000, 0 - 2000])
    if kvl == 0:
        ok = ok + 1
    kcl: int = kcl_check([3000, 0 - 1000, 0 - 2000])
    if kcl == 0:
        ok = ok + 1
    kvl_bad: int = kvl_check([5000, 0 - 3000])
    if kvl_bad == 2000:
        ok = ok + 1
    r_th: int = thevenin_resistance(10000, 2000)
    if r_th == 5000:
        ok = ok + 1
    mp: int = max_power_transfer(10000, 5000)
    if mp == 5000:
        ok = ok + 1
    nv: int = node_voltage_two_source(12000, 1000, 6000, 1000, 0)
    if nv == 0:
        ok = ok + 1
    return ok
