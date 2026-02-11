"""Heat engine cycle computations using integer arithmetic.

Tests: Otto cycle, Diesel cycle, efficiency, work output, MEP.
Scale factor 1000 for fixed-point.
"""


def otto_efficiency(compression_ratio: int, gamma: int) -> int:
    """Otto cycle efficiency = 1 - 1/r^(gamma-1).
    gamma for air ~ 1400/1000. Scale 1000."""
    if compression_ratio <= 0:
        return 0
    gamma_minus_1: int = gamma - 1000
    r_power: int = 1000
    i: int = 0
    steps: int = gamma_minus_1 // 100
    if steps <= 0:
        steps = 1
    while i < steps:
        r_power = (r_power * compression_ratio) // 1000
        i = i + 1
    if r_power == 0:
        return 0
    inv: int = (1000 * 1000) // r_power
    result: int = 1000 - inv
    if result < 0:
        result = 0
    return result


def diesel_efficiency(compression_ratio: int, cutoff_ratio: int, gamma: int) -> int:
    """Diesel cycle efficiency approximation.
    eta = 1 - (1/r^(gamma-1)) * (rc^gamma - 1)/(gamma*(rc-1)).
    Simplified integer approximation. Scale 1000."""
    if compression_ratio <= 0 or cutoff_ratio <= 1000:
        return 0
    otto_eff: int = otto_efficiency(compression_ratio, gamma)
    penalty: int = (cutoff_ratio - 1000) * 100 // 1000
    result: int = otto_eff - penalty
    if result < 0:
        result = 0
    return result


def heat_engine_work(q_in: int, q_out: int) -> int:
    """Net work from heat engine: W = Qin - Qout."""
    return q_in - q_out


def mean_effective_pressure(work: int, displacement: int) -> int:
    """Mean effective pressure = W / V_displacement. Scale 1000."""
    if displacement == 0:
        return 0
    result: int = (work * 1000) // displacement
    return result


def compression_ratio(v_max: int, v_min: int) -> int:
    """Compression ratio r = V_max / V_min. Scale 1000."""
    if v_min == 0:
        return 0
    result: int = (v_max * 1000) // v_min
    return result


def brake_power(torque: int, rpm: int) -> int:
    """Brake power = torque * 2*pi*rpm / 60.
    2*pi ~ 6283. Scale 1000."""
    result: int = (torque * 6283 * rpm) // (60 * 1000)
    return result


def specific_fuel_consumption(fuel_rate: int, power_output: int) -> int:
    """Specific fuel consumption = fuel_rate / power. Scale 1000."""
    if power_output == 0:
        return 0
    result: int = (fuel_rate * 1000) // power_output
    return result


def volumetric_efficiency(actual_air: int, theoretical_air: int) -> int:
    """Volumetric efficiency = actual/theoretical * 1000. Scale 1000."""
    if theoretical_air == 0:
        return 0
    result: int = (actual_air * 1000) // theoretical_air
    return result


def test_module() -> int:
    """Test heat engine computations."""
    ok: int = 0
    w: int = heat_engine_work(1000, 600)
    if w == 400:
        ok = ok + 1
    cr: int = compression_ratio(10000, 1000)
    if cr == 10000:
        ok = ok + 1
    mep: int = mean_effective_pressure(400, 2000)
    if mep == 200:
        ok = ok + 1
    bp: int = brake_power(100, 3000)
    if bp > 31000 and bp < 32000:
        ok = ok + 1
    sfc: int = specific_fuel_consumption(500, 1000)
    if sfc == 500:
        ok = ok + 1
    ve: int = volumetric_efficiency(850, 1000)
    if ve == 850:
        ok = ok + 1
    return ok
