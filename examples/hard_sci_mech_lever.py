"""Lever and simple machine computations using integer arithmetic.

Tests: lever classes, mechanical advantage, equilibrium, work.
Scale factor 1000 for fixed-point.
"""


def lever_effort(load: int, load_arm: int, effort_arm: int) -> int:
    """Lever equilibrium: effort = load * load_arm / effort_arm. Scale 1000."""
    if effort_arm == 0:
        return 0
    result: int = (load * load_arm) // effort_arm
    return result


def lever_mechanical_advantage(effort_arm: int, load_arm: int) -> int:
    """MA = effort_arm / load_arm. Scale 1000."""
    if load_arm == 0:
        return 0
    result: int = (effort_arm * 1000) // load_arm
    return result


def lever_load_capacity(effort: int, effort_arm: int, load_arm: int) -> int:
    """Load = effort * effort_arm / load_arm. Scale 1000."""
    if load_arm == 0:
        return 0
    result: int = (effort * effort_arm) // load_arm
    return result


def torque_balance(forces: list[int], distances: list[int]) -> int:
    """Check torque balance: sum(F_i * d_i) should be 0.
    Returns absolute net torque. Scale 1000."""
    total: int = 0
    i: int = 0
    n: int = len(forces)
    if n > len(distances):
        n = len(distances)
    while i < n:
        f_val: int = forces[i]
        d_val: int = distances[i]
        total = total + (f_val * d_val) // 1000
        i = i + 1
    if total < 0:
        total = 0 - total
    return total


def wedge_mechanical_advantage(length_val: int, width: int) -> int:
    """Wedge MA = length/width. Scale 1000."""
    if width == 0:
        return 0
    result: int = (length_val * 1000) // width
    return result


def screw_mechanical_advantage(circumference: int, pitch: int) -> int:
    """Screw MA = circumference/pitch. Scale 1000."""
    if pitch == 0:
        return 0
    result: int = (circumference * 1000) // pitch
    return result


def wheel_axle_ma(wheel_radius: int, axle_radius: int) -> int:
    """Wheel and axle MA = wheel_radius/axle_radius. Scale 1000."""
    if axle_radius == 0:
        return 0
    result: int = (wheel_radius * 1000) // axle_radius
    return result


def inclined_plane_ma(length_val: int, height: int) -> int:
    """Inclined plane MA = length/height. Scale 1000."""
    if height == 0:
        return 0
    result: int = (length_val * 1000) // height
    return result


def work_input_equals_output(effort: int, effort_dist: int, load: int, load_dist: int) -> int:
    """Check work in = work out (ideal). Returns difference. Scale 1000."""
    w_in: int = (effort * effort_dist) // 1000
    w_out: int = (load * load_dist) // 1000
    diff: int = w_in - w_out
    if diff < 0:
        diff = 0 - diff
    return diff


def compound_ma(machines: list[int]) -> int:
    """Compound machine MA = product of individual MAs. Scale 1000."""
    if len(machines) == 0:
        return 1000
    result: int = machines[0]
    i: int = 1
    while i < len(machines):
        val: int = machines[i]
        result = (result * val) // 1000
        i = i + 1
    return result


def test_module() -> int:
    """Test lever computations."""
    ok: int = 0
    eff: int = lever_effort(1000, 2000, 4000)
    if eff == 500:
        ok = ok + 1
    ma: int = lever_mechanical_advantage(4000, 2000)
    if ma == 2000:
        ok = ok + 1
    lc: int = lever_load_capacity(500, 4000, 2000)
    if lc == 1000:
        ok = ok + 1
    wma: int = wedge_mechanical_advantage(10000, 2000)
    if wma == 5000:
        ok = ok + 1
    ip_ma: int = inclined_plane_ma(5000, 1000)
    if ip_ma == 5000:
        ok = ok + 1
    cma: int = compound_ma([2000, 3000])
    if cma == 6000:
        ok = ok + 1
    return ok
