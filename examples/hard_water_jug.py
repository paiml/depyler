"""Water jug problem variants.

Tests: GCD-based solvability, min operations, reachable volumes.
"""


def gcd(a: int, b: int) -> int:
    """Compute greatest common divisor."""
    x: int = a
    y: int = b
    while y != 0:
        temp: int = y
        y = x % y
        x = temp
    return x


def can_measure(jug1: int, jug2: int, target: int) -> int:
    """Check if target volume is measurable. Returns 1 if yes."""
    if target < 0:
        return 0
    if target > jug1 + jug2:
        return 0
    if target == 0:
        return 1
    if target % gcd(jug1, jug2) == 0:
        return 1
    return 0


def min_operations_fill(capacity: int, target: int) -> int:
    """Minimum fill/pour operations to get target in one jug (simplified).
    
    Uses a single jug: fill adds capacity, pour removes 1.
    Returns steps to reach target mod capacity.
    """
    if target == 0:
        return 0
    if target > capacity:
        return -1
    steps: int = 0
    current: int = 0
    limit: int = capacity * capacity
    while steps < limit:
        if current == target:
            return steps
        if current == 0:
            current = capacity
            steps = steps + 1
        else:
            current = current - 1
            steps = steps + 1
    return -1


def count_reachable_volumes(jug1: int, jug2: int) -> int:
    """Count how many distinct volumes from 0 to jug1+jug2 are reachable."""
    total: int = jug1 + jug2
    g: int = gcd(jug1, jug2)
    count: int = 0
    v: int = 0
    while v <= total:
        if v % g == 0:
            count = count + 1
        v = v + 1
    return count


def test_module() -> int:
    """Test water jug operations."""
    ok: int = 0
    if gcd(12, 8) == 4:
        ok = ok + 1
    if can_measure(3, 5, 4) == 1:
        ok = ok + 1
    if can_measure(2, 6, 5) == 0:
        ok = ok + 1
    if can_measure(3, 5, 0) == 1:
        ok = ok + 1
    if min_operations_fill(5, 3) == 3:
        ok = ok + 1
    if min_operations_fill(5, 0) == 0:
        ok = ok + 1
    if count_reachable_volumes(3, 5) == 9:
        ok = ok + 1
    return ok
