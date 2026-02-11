def grav_force(m1: float, m2: float, dist: float) -> float:
    gc: float = 6.674e-11
    return gc * m1 * m2 / (dist * dist)

def grav_accel(mass: float, dist: float) -> float:
    gc: float = 6.674e-11
    return gc * mass / (dist * dist)

def orbital_velocity(mass: float, radius: float) -> float:
    gc: float = 6.674e-11
    return (gc * mass / radius) ** 0.5

def orbital_period(mass: float, radius: float) -> float:
    gc: float = 6.674e-11
    pi: float = 3.14159265
    v: float = (gc * mass / radius) ** 0.5
    circumference: float = 2.0 * pi * radius
    return circumference / v

def escape_velocity(mass: float, radius: float) -> float:
    gc: float = 6.674e-11
    return (2.0 * gc * mass / radius) ** 0.5

def weight_on_surface(obj_mass: float, planet_mass: float, planet_radius: float) -> float:
    gc: float = 6.674e-11
    return gc * obj_mass * planet_mass / (planet_radius * planet_radius)

def potential_energy(m1: float, m2: float, dist: float) -> float:
    gc: float = 6.674e-11
    return (0.0 - 1.0) * gc * m1 * m2 / dist

def test_module() -> int:
    passed: int = 0
    f: float = grav_force(1000.0, 1000.0, 1.0)
    diff: float = f - 0.00006674
    if diff < 0.0001 and diff > (0.0 - 0.0001):
        passed = passed + 1
    a: float = grav_accel(5.972e24, 6.371e6)
    diff2: float = a - 9.82
    if diff2 < 0.1 and diff2 > (0.0 - 0.1):
        passed = passed + 1
    ev: float = escape_velocity(5.972e24, 6.371e6)
    diff3: float = ev - 11186.0
    if diff3 < 100.0 and diff3 > (0.0 - 100.0):
        passed = passed + 1
    pe: float = potential_energy(1.0, 1.0, 1.0)
    if pe < 0.0:
        passed = passed + 1
    w: float = weight_on_surface(1.0, 5.972e24, 6.371e6)
    diff4: float = w - 9.82
    if diff4 < 0.1 and diff4 > (0.0 - 0.1):
        passed = passed + 1
    return passed
