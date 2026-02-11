"""Deterministic sampling patterns for testing (pseudo-random via LCG).

Tests: lcg_next, sample_k, reservoir_sample, weighted_select, shuffle.
"""


def lcg_next(seed: int) -> int:
    """Linear congruential generator: deterministic pseudo-random."""
    a: int = 1103515245
    c: int = 12345
    m: int = 2147483648
    result: int = (a * seed + c) % m
    return result


def lcg_range(seed: int, lo: int, hi: int) -> list[int]:
    """Return [random_value_in_range, new_seed]."""
    new_seed: int = lcg_next(seed)
    diff: int = hi - lo
    if diff <= 0:
        result: list[int] = [lo, new_seed]
        return result
    val: int = lo + (new_seed % diff)
    result2: list[int] = [val, new_seed]
    return result2


def reservoir_sample(arr: list[int], k_size: int, seed: int) -> list[int]:
    """Reservoir sampling: select k items from stream deterministically."""
    n: int = len(arr)
    if k_size >= n:
        result: list[int] = []
        i: int = 0
        while i < n:
            result.append(arr[i])
            i = i + 1
        return result
    reservoir: list[int] = []
    i2: int = 0
    while i2 < k_size:
        reservoir.append(arr[i2])
        i2 = i2 + 1
    cur_seed: int = seed
    j: int = k_size
    while j < n:
        rng: list[int] = lcg_range(cur_seed, 0, j + 1)
        rand_idx: int = rng[0]
        cur_seed = rng[1]
        if rand_idx < k_size:
            reservoir[rand_idx] = arr[j]
        j = j + 1
    return reservoir


def deterministic_shuffle(arr: list[int], seed: int) -> list[int]:
    """Fisher-Yates shuffle with deterministic LCG."""
    result: list[int] = []
    i: int = 0
    n: int = len(arr)
    while i < n:
        result.append(arr[i])
        i = i + 1
    cur_seed: int = seed
    j: int = n - 1
    while j > 0:
        rng: list[int] = lcg_range(cur_seed, 0, j + 1)
        swap_idx: int = rng[0]
        cur_seed = rng[1]
        tmp: int = result[j]
        result[j] = result[swap_idx]
        result[swap_idx] = tmp
        j = j - 1
    return result


def weighted_select(weights: list[int], seed: int) -> int:
    """Select index proportional to weight using deterministic random."""
    n: int = len(weights)
    total: int = 0
    i: int = 0
    while i < n:
        total = total + weights[i]
        i = i + 1
    if total == 0:
        return 0
    rng: list[int] = lcg_range(seed, 0, total)
    target: int = rng[0]
    cumulative: int = 0
    j: int = 0
    while j < n:
        cumulative = cumulative + weights[j]
        if cumulative > target:
            return j
        j = j + 1
    return n - 1


def sample_without_replacement(arr: list[int], k_size: int, seed: int) -> list[int]:
    """Sample k items without replacement using shuffling."""
    shuffled: list[int] = deterministic_shuffle(arr, seed)
    result: list[int] = []
    i: int = 0
    limit: int = k_size
    if limit > len(shuffled):
        limit = len(shuffled)
    while i < limit:
        result.append(shuffled[i])
        i = i + 1
    return result


def test_module() -> int:
    """Test sampling algorithms."""
    passed: int = 0

    s1: int = lcg_next(42)
    s2: int = lcg_next(42)
    if s1 == s2:
        passed = passed + 1

    s3: int = lcg_next(0)
    if s3 != 0:
        passed = passed + 1

    arr: list[int] = [10, 20, 30, 40, 50]
    rs: list[int] = reservoir_sample(arr, 3, 42)
    if len(rs) == 3:
        passed = passed + 1

    rs2: list[int] = reservoir_sample(arr, 10, 42)
    if len(rs2) == 5:
        passed = passed + 1

    shuf: list[int] = deterministic_shuffle([1, 2, 3, 4, 5], 99)
    if len(shuf) == 5:
        passed = passed + 1

    wi: int = weighted_select([10, 20, 70], 42)
    if wi >= 0:
        if wi <= 2:
            passed = passed + 1

    samp: list[int] = sample_without_replacement([1, 2, 3, 4, 5], 2, 42)
    if len(samp) == 2:
        passed = passed + 1

    return passed
