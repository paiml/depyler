def elastic_v1(m1: float, m2: float, v1: float, v2: float) -> float:
    num: float = (m1 - m2) * v1 + 2.0 * m2 * v2
    denom: float = m1 + m2
    return num / denom

def elastic_v2(m1: float, m2: float, v1: float, v2: float) -> float:
    num: float = (m2 - m1) * v2 + 2.0 * m1 * v1
    denom: float = m1 + m2
    return num / denom

def kinetic_energy(mass: float, velocity: float) -> float:
    return 0.5 * mass * velocity * velocity

def momentum(mass: float, velocity: float) -> float:
    return mass * velocity

def coefficient_restitution(v1_before: float, v2_before: float, v1_after: float, v2_after: float) -> float:
    approach: float = v1_before - v2_before
    if approach == 0.0:
        return 0.0
    separation: float = v2_after - v1_after
    return separation / approach

def inelastic_velocity(m1: float, v1: float, m2: float, v2: float) -> float:
    return (m1 * v1 + m2 * v2) / (m1 + m2)

def impulse(mass: float, v_before: float, v_after: float) -> float:
    return mass * (v_after - v_before)

def test_module() -> int:
    passed: int = 0
    nv1: float = elastic_v1(1.0, 1.0, 5.0, 0.0)
    diff: float = nv1 - 0.0
    if diff < 0.001 and diff > (0.0 - 0.001):
        passed = passed + 1
    nv2: float = elastic_v2(1.0, 1.0, 5.0, 0.0)
    diff2: float = nv2 - 5.0
    if diff2 < 0.001 and diff2 > (0.0 - 0.001):
        passed = passed + 1
    ke: float = kinetic_energy(2.0, 3.0)
    if ke == 9.0:
        passed = passed + 1
    p: float = momentum(5.0, 10.0)
    if p == 50.0:
        passed = passed + 1
    iv: float = inelastic_velocity(1.0, 10.0, 1.0, 0.0)
    if iv == 5.0:
        passed = passed + 1
    imp: float = impulse(2.0, 3.0, 7.0)
    if imp == 8.0:
        passed = passed + 1
    return passed
