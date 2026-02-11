"""Deterministic probability simulations using seed-based pseudo-random.

Tests: linear congruential generator, Monte Carlo pi estimate,
random walk distance, birthday paradox simulation, dice roll distribution.
"""


def lcg_next(seed: int, a: int, c: int, m: int) -> int:
    """Linear congruential generator: next value."""
    return (a * seed + c) % m


def lcg_sequence(seed: int, n: int) -> list[int]:
    """Generate n pseudo-random numbers using LCG with standard parameters."""
    a: int = 1103515245
    c: int = 12345
    m: int = 2147483648
    result: list[int] = []
    current: int = seed
    i: int = 0
    while i < n:
        current = lcg_next(current, a, c, m)
        result.append(current)
        i = i + 1
    return result


def monte_carlo_pi_estimate(n: int, seed: int) -> int:
    """Estimate pi*1000 using Monte Carlo with deterministic LCG.
    
    Returns integer approximation of pi * 1000.
    """
    a: int = 1103515245
    c: int = 12345
    m: int = 2147483648
    inside: int = 0
    current: int = seed
    i: int = 0
    while i < n:
        current = lcg_next(current, a, c, m)
        x: int = current % 10000
        current = lcg_next(current, a, c, m)
        y: int = current % 10000
        dist_sq: int = x * x + y * y
        if dist_sq <= 10000 * 10000:
            inside = inside + 1
        i = i + 1
    return (4 * inside * 1000) // n


def random_walk_final_position(steps: int, seed: int) -> list[int]:
    """Simulate 2D random walk. Returns [final_x, final_y]."""
    a: int = 1103515245
    c: int = 12345
    m: int = 2147483648
    x: int = 0
    y: int = 0
    current: int = seed
    i: int = 0
    while i < steps:
        current = lcg_next(current, a, c, m)
        direction: int = current % 4
        if direction == 0:
            x = x + 1
        elif direction == 1:
            x = x - 1
        elif direction == 2:
            y = y + 1
        else:
            y = y - 1
        i = i + 1
    return [x, y]


def dice_distribution(rolls: int, seed: int) -> list[int]:
    """Simulate dice rolls, return frequency of each face (1-6)."""
    a: int = 1103515245
    c: int = 12345
    m: int = 2147483648
    freq: list[int] = [0, 0, 0, 0, 0, 0]
    current: int = seed
    i: int = 0
    while i < rolls:
        current = lcg_next(current, a, c, m)
        face: int = (current % 6)
        freq[face] = freq[face] + 1
        i = i + 1
    return freq


def test_module() -> bool:
    """Test all probability simulation functions."""
    ok: bool = True

    seq: list[int] = lcg_sequence(42, 5)
    if len(seq) != 5:
        ok = False
    if seq[0] == seq[1]:
        ok = False

    pi_est: int = monte_carlo_pi_estimate(10000, 12345)
    if pi_est < 2500 or pi_est > 3800:
        ok = False

    pos: list[int] = random_walk_final_position(100, 7)
    if len(pos) != 2:
        ok = False

    freq: list[int] = dice_distribution(6000, 99)
    total: int = 0
    i: int = 0
    while i < 6:
        total = total + freq[i]
        i = i + 1
    if total != 6000:
        ok = False

    return ok
