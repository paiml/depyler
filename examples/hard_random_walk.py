"""Deterministic pseudo-random walk simulation."""


def lcg_next(state: int) -> int:
    """Linear congruential generator: next pseudo-random value."""
    new_state: int = (state * 1103515245 + 12345) % 2147483648
    return new_state


def walk_1d(steps: int, seed: int) -> list[int]:
    """Simulate a 1D random walk for given steps using LCG."""
    positions: list[int] = [0]
    state: int = seed
    pos: int = 0
    i: int = 0
    while i < steps:
        state = lcg_next(state)
        direction: int = state % 2
        if direction == 0:
            pos = pos - 1
        else:
            pos = pos + 1
        positions.append(pos)
        i = i + 1
    return positions


def walk_distance(positions: list[int]) -> int:
    """Compute the final distance from origin."""
    last_idx: int = len(positions) - 1
    final: int = positions[last_idx]
    if final < 0:
        return -final
    return final


def max_displacement(positions: list[int]) -> int:
    """Find the maximum absolute displacement during walk."""
    max_abs: int = 0
    i: int = 0
    length: int = len(positions)
    while i < length:
        val: int = positions[i]
        abs_val: int = val
        if val < 0:
            abs_val = -val
        if abs_val > max_abs:
            max_abs = abs_val
        i = i + 1
    return max_abs


def count_origin_visits(positions: list[int]) -> int:
    """Count how many times the walk passes through origin."""
    count: int = 0
    i: int = 1
    length: int = len(positions)
    while i < length:
        if positions[i] == 0:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test random walk operations."""
    passed: int = 0

    r1: int = lcg_next(0)
    if r1 == 12345:
        passed = passed + 1

    r2: int = lcg_next(12345)
    if r2 > 0:
        passed = passed + 1

    walk: list[int] = walk_1d(10, 42)
    if len(walk) == 11:
        passed = passed + 1

    if walk[0] == 0:
        passed = passed + 1

    dist: int = walk_distance(walk)
    if dist >= 0:
        passed = passed + 1

    max_d: int = max_displacement(walk)
    if max_d >= dist:
        passed = passed + 1

    zero_walk: list[int] = [0, 1, 0, -1, 0]
    origins: int = count_origin_visits(zero_walk)
    if origins == 2:
        passed = passed + 1

    empty_walk: list[int] = walk_1d(0, 0)
    if len(empty_walk) == 1 and empty_walk[0] == 0:
        passed = passed + 1

    return passed
