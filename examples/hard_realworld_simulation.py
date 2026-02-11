"""Real-world simulation patterns.

Mimics: Monte Carlo methods, population models, random walk simulations.
Uses deterministic pseudo-random number generator for reproducibility.
"""


def lcg_next(seed: int) -> int:
    """Linear congruential generator. Returns next pseudo-random value."""
    # Parameters from Numerical Recipes
    a: int = 1664525
    c: int = 1013904223
    m: int = 2147483647
    return (a * seed + c) % m


def lcg_range(seed: int, low: int, high: int) -> list[int]:
    """Generate random value in [low, high] from seed. Returns [value, next_seed]."""
    next_seed: int = lcg_next(seed)
    if next_seed < 0:
        next_seed = 0 - next_seed
    span: int = high - low + 1
    value: int = low + (next_seed % span)
    return [value, next_seed]


def random_walk_1d(steps: int, seed: int) -> list[int]:
    """Simulate 1D random walk. Each step is +1 or -1."""
    positions: list[int] = [0]
    current_pos: int = 0
    current_seed: int = seed
    idx: int = 0
    while idx < steps:
        rng: list[int] = lcg_range(current_seed, 0, 1)
        if rng[0] == 0:
            current_pos = current_pos + 1
        else:
            current_pos = current_pos - 1
        positions.append(current_pos)
        current_seed = rng[1]
        idx = idx + 1
    return positions


def max_displacement(positions: list[int]) -> int:
    """Find maximum absolute displacement from origin."""
    max_abs: int = 0
    idx: int = 0
    while idx < len(positions):
        val: int = positions[idx]
        if val < 0:
            val = 0 - val
        if val > max_abs:
            max_abs = val
        idx = idx + 1
    return max_abs


def population_growth(initial: int, growth_rate: int, carrying_capacity: int, steps: int) -> list[int]:
    """Logistic growth model. growth_rate is scaled by 100 (e.g., 110 = 1.10x).
    Returns population at each step."""
    populations: list[int] = [initial]
    pop: int = initial
    idx: int = 0
    while idx < steps:
        # Logistic: pop * rate * (1 - pop/capacity)
        # All integer math: pop * growth_rate * (capacity - pop) / (100 * capacity)
        new_pop: int = (pop * growth_rate * (carrying_capacity - pop)) // (100 * carrying_capacity)
        if new_pop < 0:
            new_pop = 0
        if new_pop > carrying_capacity:
            new_pop = carrying_capacity
        pop = new_pop
        populations.append(pop)
        idx = idx + 1
    return populations


def dice_roll_histogram(num_rolls: int, seed: int) -> list[int]:
    """Simulate rolling two dice num_rolls times. Returns histogram of sums (indices 2-12)."""
    hist: list[int] = []
    hi: int = 0
    while hi < 13:
        hist.append(0)
        hi = hi + 1
    current_seed: int = seed
    idx: int = 0
    while idx < num_rolls:
        d1_res: list[int] = lcg_range(current_seed, 1, 6)
        d1: int = d1_res[0]
        current_seed = d1_res[1]
        d2_res: list[int] = lcg_range(current_seed, 1, 6)
        d2: int = d2_res[0]
        current_seed = d2_res[1]
        total: int = d1 + d2
        hist[total] = hist[total] + 1
        idx = idx + 1
    return hist


def estimate_pi_x1000(num_samples: int, seed: int) -> int:
    """Estimate pi using Monte Carlo (quarter circle method).
    Returns pi*1000 as integer. Uses grid_size=10000."""
    inside: int = 0
    current_seed: int = seed
    idx: int = 0
    while idx < num_samples:
        rx: list[int] = lcg_range(current_seed, 0, 9999)
        x: int = rx[0]
        current_seed = rx[1]
        ry: list[int] = lcg_range(current_seed, 0, 9999)
        y: int = ry[0]
        current_seed = ry[1]
        dist_sq: int = x * x + y * y
        if dist_sq <= 99980001:
            inside = inside + 1
        idx = idx + 1
    return (4 * inside * 1000) // num_samples


def final_position(walk: list[int]) -> int:
    """Get final position of a random walk."""
    if len(walk) == 0:
        return 0
    return walk[len(walk) - 1]


def count_returns_to_origin(walk: list[int]) -> int:
    """Count how many times walk returns to position 0."""
    count: int = 0
    idx: int = 1
    while idx < len(walk):
        if walk[idx] == 0:
            count = count + 1
        idx = idx + 1
    return count


def walk_variance_x100(walk: list[int]) -> int:
    """Compute variance of walk positions * 100."""
    if len(walk) <= 1:
        return 0
    total: int = 0
    idx: int = 0
    while idx < len(walk):
        total = total + walk[idx]
        idx = idx + 1
    mean100: int = (total * 100) // len(walk)
    sum_sq: int = 0
    idx2: int = 0
    while idx2 < len(walk):
        diff: int = walk[idx2] * 100 - mean100
        sum_sq = sum_sq + diff * diff
        idx2 = idx2 + 1
    return sum_sq // (len(walk) * 100)


def test_module() -> int:
    """Test simulation module."""
    passed: int = 0

    # Test 1: lcg produces different values
    s1: int = lcg_next(42)
    s2: int = lcg_next(s1)
    if s1 != s2 and s1 != 42:
        passed = passed + 1

    # Test 2: lcg_range within bounds
    rng: list[int] = lcg_range(42, 1, 6)
    if rng[0] >= 1 and rng[0] <= 6:
        passed = passed + 1

    # Test 3: random walk starts at 0
    walk: list[int] = random_walk_1d(100, 42)
    if walk[0] == 0 and len(walk) == 101:
        passed = passed + 1

    # Test 4: max displacement > 0
    md: int = max_displacement(walk)
    if md > 0:
        passed = passed + 1

    # Test 5: population growth doesn't exceed capacity
    pops: list[int] = population_growth(100, 150, 1000, 20)
    all_under: bool = True
    pi: int = 0
    while pi < len(pops):
        if pops[pi] > 1000:
            all_under = False
        pi = pi + 1
    if all_under:
        passed = passed + 1

    # Test 6: dice histogram sums to num_rolls
    hist: list[int] = dice_roll_histogram(1000, 42)
    total: int = 0
    hi: int = 2
    while hi <= 12:
        total = total + hist[hi]
        hi = hi + 1
    if total == 1000:
        passed = passed + 1

    # Test 7: pi estimate in reasonable range
    pi_est: int = estimate_pi_x1000(5000, 42)
    if pi_est > 2500 and pi_est < 4000:
        passed = passed + 1

    # Test 8: walk variance non-negative
    v: int = walk_variance_x100(walk)
    if v >= 0:
        passed = passed + 1

    return passed
