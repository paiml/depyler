def projectile_height(v0: float, angle_rad: float, t: float, g: float) -> float:
    vy: float = v0 * angle_rad
    h: float = vy * t - 0.5 * g * t * t
    if h < 0.0:
        return 0.0
    return h

def projectile_range(v0: float, sin2a: float, g: float) -> float:
    r: float = (v0 * v0 * sin2a) / g
    if r < 0.0:
        return 0.0
    return r

def time_of_flight(v0: float, sina: float, g: float) -> float:
    t: float = (2.0 * v0 * sina) / g
    return t

def max_height(v0: float, sina: float, g: float) -> float:
    h: float = (v0 * v0 * sina * sina) / (2.0 * g)
    return h

def projectile_x(v0: float, cosa: float, t: float) -> float:
    return v0 * cosa * t

def projectile_velocity(vx: float, vy: float) -> float:
    return (vx * vx + vy * vy) ** 0.5

def test_module() -> int:
    passed: int = 0
    h: float = projectile_height(10.0, 0.5, 0.0, 9.8)
    if h == 0.0:
        passed = passed + 1
    r: float = projectile_range(10.0, 1.0, 9.8)
    diff: float = r - 10.204081632653061
    if diff < 0.001 and diff > (0.0 - 0.001):
        passed = passed + 1
    t: float = time_of_flight(10.0, 0.5, 9.8)
    diff2: float = t - 1.0204081632653061
    if diff2 < 0.001 and diff2 > (0.0 - 0.001):
        passed = passed + 1
    mh: float = max_height(10.0, 0.5, 9.8)
    diff3: float = mh - 1.2755102040816326
    if diff3 < 0.001 and diff3 > (0.0 - 0.001):
        passed = passed + 1
    x: float = projectile_x(10.0, 0.866, 1.0)
    diff4: float = x - 8.66
    if diff4 < 0.01 and diff4 > (0.0 - 0.01):
        passed = passed + 1
    v: float = projectile_velocity(3.0, 4.0)
    diff5: float = v - 5.0
    if diff5 < 0.001 and diff5 > (0.0 - 0.001):
        passed = passed + 1
    return passed
