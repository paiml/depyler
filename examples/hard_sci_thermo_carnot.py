"""Carnot cycle and heat engine efficiency computations.

Tests: Carnot efficiency, work output, COP, reversibility.
Scale factor 1000 for fixed-point.
"""


def carnot_efficiency(temp_hot: int, temp_cold: int) -> int:
    """Carnot efficiency = 1 - Tc/Th. Fixed-point scale 1000.
    Returns efficiency * 1000."""
    if temp_hot <= 0:
        return 0
    if temp_cold >= temp_hot:
        return 0
    result: int = 1000 - (temp_cold * 1000) // temp_hot
    return result


def carnot_work(q_hot: int, temp_hot: int, temp_cold: int) -> int:
    """Work output of Carnot engine = Qh * efficiency. Scale 1000."""
    eff: int = carnot_efficiency(temp_hot, temp_cold)
    result: int = (q_hot * eff) // 1000
    return result


def carnot_heat_rejected(q_hot: int, temp_hot: int, temp_cold: int) -> int:
    """Heat rejected Qc = Qh - W = Qh * Tc/Th. Scale 1000."""
    if temp_hot <= 0:
        return q_hot
    result: int = (q_hot * temp_cold) // temp_hot
    return result


def cop_refrigerator(temp_hot: int, temp_cold: int) -> int:
    """COP of Carnot refrigerator = Tc/(Th - Tc). Scale 1000."""
    diff: int = temp_hot - temp_cold
    if diff <= 0:
        return 0
    result: int = (temp_cold * 1000) // diff
    return result


def cop_heat_pump(temp_hot: int, temp_cold: int) -> int:
    """COP of Carnot heat pump = Th/(Th - Tc). Scale 1000."""
    diff: int = temp_hot - temp_cold
    if diff <= 0:
        return 0
    result: int = (temp_hot * 1000) // diff
    return result


def entropy_change_isothermal(heat: int, temp: int) -> int:
    """Entropy change for isothermal process: dS = Q/T. Scale 1000."""
    if temp == 0:
        return 0
    result: int = (heat * 1000) // temp
    return result


def is_reversible(entropy_change: int) -> int:
    """Check if process is reversible (dS_total = 0). Returns 1 or 0."""
    if entropy_change == 0:
        return 1
    return 0


def thermal_efficiency(work_out: int, heat_in: int) -> int:
    """Thermal efficiency = W/Qh. Scale 1000."""
    if heat_in == 0:
        return 0
    result: int = (work_out * 1000) // heat_in
    return result


def test_module() -> int:
    """Test Carnot cycle computations."""
    ok: int = 0
    eff: int = carnot_efficiency(600, 300)
    if eff == 500:
        ok = ok + 1
    w: int = carnot_work(1000, 600, 300)
    if w == 500:
        ok = ok + 1
    qc: int = carnot_heat_rejected(1000, 600, 300)
    if qc == 500:
        ok = ok + 1
    cop_r: int = cop_refrigerator(300, 270)
    if cop_r == 9000:
        ok = ok + 1
    cop_h: int = cop_heat_pump(300, 270)
    if cop_h == 10000:
        ok = ok + 1
    ds: int = entropy_change_isothermal(1000, 500)
    if ds == 2000:
        ok = ok + 1
    rev: int = is_reversible(0)
    if rev == 1:
        ok = ok + 1
    return ok
