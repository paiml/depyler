"""RC circuit computations using integer arithmetic.

Tests: time constant, charging, discharging, energy stored.
Scale factor 1000 for fixed-point.
"""


def rc_time_constant(resistance: int, capacitance: int) -> int:
    """Time constant tau = R*C. Scale 1000."""
    result: int = (resistance * capacitance) // 1000
    return result


def rc_charging_voltage(v_source: int, elapsed: int, tau: int) -> int:
    """Charging voltage: V(t) = Vs*(1 - exp(-t/tau)).
    exp(-x) ~ 1 - x + x^2/2 for small x. Scale 1000."""
    if tau == 0:
        return v_source
    x: int = (elapsed * 1000) // tau
    if x > 5000:
        return v_source
    exp_val: int = 1000 - x + (x * x) // 2000
    if exp_val < 0:
        exp_val = 0
    result: int = (v_source * (1000 - exp_val)) // 1000
    return result


def rc_discharging_voltage(v_initial: int, elapsed: int, tau: int) -> int:
    """Discharging voltage: V(t) = V0*exp(-t/tau).
    exp(-x) ~ 1 - x + x^2/2 for small x. Scale 1000."""
    if tau == 0:
        return 0
    x: int = (elapsed * 1000) // tau
    if x > 5000:
        return 0
    exp_val: int = 1000 - x + (x * x) // 2000
    if exp_val < 0:
        exp_val = 0
    result: int = (v_initial * exp_val) // 1000
    return result


def capacitor_energy(capacitance: int, volt: int) -> int:
    """Energy stored: E = 0.5*C*V^2. Scale 1000."""
    v_sq: int = (volt * volt) // 1000
    result: int = (capacitance * v_sq) // 2000
    return result


def capacitor_charge(capacitance: int, volt: int) -> int:
    """Charge stored: Q = C*V. Scale 1000."""
    result: int = (capacitance * volt) // 1000
    return result


def series_capacitance_two(c1: int, c2: int) -> int:
    """Series capacitance: 1/C = 1/C1 + 1/C2. Scale 1000."""
    if c1 == 0 or c2 == 0:
        return 0
    denom: int = (1000 * 1000) // c1 + (1000 * 1000) // c2
    if denom == 0:
        return 0
    result: int = (1000 * 1000) // denom
    return result


def parallel_capacitance(caps: list[int]) -> int:
    """Parallel capacitance: C = C1 + C2 + ..."""
    total: int = 0
    i: int = 0
    while i < len(caps):
        val: int = caps[i]
        total = total + val
        i = i + 1
    return total


def rc_cutoff_frequency(resistance: int, capacitance: int) -> int:
    """RC filter cutoff: f_c = 1/(2*pi*R*C).
    2*pi ~ 6283. Scale 1000."""
    denom: int = (6283 * resistance * capacitance) // (1000 * 1000)
    if denom == 0:
        return 0
    result: int = (1000 * 1000) // denom
    return result


def test_module() -> int:
    """Test RC circuit computations."""
    ok: int = 0
    tau: int = rc_time_constant(1000, 1000)
    if tau == 1000:
        ok = ok + 1
    v_charge: int = rc_charging_voltage(5000, 0, 1000)
    if v_charge == 0:
        ok = ok + 1
    v_full: int = rc_charging_voltage(5000, 10000, 1000)
    if v_full == 5000:
        ok = ok + 1
    v_dis: int = rc_discharging_voltage(5000, 0, 1000)
    if v_dis == 5000:
        ok = ok + 1
    energy: int = capacitor_energy(1000, 5000)
    if energy == 12500:
        ok = ok + 1
    pc: int = parallel_capacitance([100, 200, 300])
    if pc == 600:
        ok = ok + 1
    return ok
