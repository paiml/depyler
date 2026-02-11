def gcd(a: int, b: int) -> int:
    x: int = a
    y: int = b
    while y != 0:
        temp: int = y
        y = x % y
        x = temp
    return x

def can_measure(jug1: int, jug2: int, target: int) -> int:
    if target > jug1 + jug2:
        return 0
    if target == 0:
        return 1
    g: int = gcd(jug1, jug2)
    if target % g == 0:
        return 1
    return 0

def min_steps_water_jug(cap1: int, cap2: int, target: int) -> int:
    chk: int = can_measure(cap1, cap2, target)
    if chk == 0:
        return 0 - 1
    if target == 0:
        return 0
    steps1: int = fill_from_first(cap1, cap2, target)
    steps2: int = fill_from_first(cap2, cap1, target)
    if steps1 < steps2:
        return steps1
    return steps2

def fill_from_first(cap_a: int, cap_b: int, target: int) -> int:
    a: int = 0
    b: int = 0
    steps: int = 0
    limit: int = 1000
    while steps < limit:
        if a == target or b == target or a + b == target:
            return steps
        if a == 0:
            a = cap_a
            steps = steps + 1
        elif b == cap_b:
            b = 0
            steps = steps + 1
        else:
            pour: int = cap_b - b
            if a < pour:
                pour = a
            a = a - pour
            b = b + pour
            steps = steps + 1
    return 0 - 1

def water_jug_states(cap1: int, cap2: int, target: int) -> int:
    chk: int = can_measure(cap1, cap2, target)
    if chk == 0:
        return 0
    return min_steps_water_jug(cap1, cap2, target) + 1

def die_hard_jugs(a: int, b: int, target: int) -> int:
    return can_measure(a, b, target)

def test_module() -> int:
    passed: int = 0
    r1: int = can_measure(3, 5, 4)
    if r1 == 1:
        passed = passed + 1
    r2: int = can_measure(2, 6, 5)
    if r2 == 0:
        passed = passed + 1
    r3: int = min_steps_water_jug(3, 5, 4)
    if r3 > 0:
        passed = passed + 1
    r4: int = gcd(12, 8)
    if r4 == 4:
        passed = passed + 1
    r5: int = die_hard_jugs(3, 5, 4)
    if r5 == 1:
        passed = passed + 1
    r6: int = can_measure(1, 1, 2)
    if r6 == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
