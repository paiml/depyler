# Reservoir sampling (deterministic test with seed)
# Using a simple linear congruential generator for deterministic randomness


def lcg_next(state: int) -> int:
    # Linear congruential generator: (a*state + c) mod m
    # Using small prime constants for reproducibility
    return (1103515245 * state + 12345) % 2147483648


def reservoir_sample(data: list[int], k: int, seed: int) -> list[int]:
    # Reservoir sampling: select k items from data with equal probability
    reservoir: list[int] = []
    i: int = 0
    while i < k and i < len(data):
        reservoir.append(data[i])
        i = i + 1
    state: int = seed
    while i < len(data):
        state = lcg_next(state)
        j: int = state % (i + 1)
        if j < k:
            reservoir[j] = data[i]
        i = i + 1
    return reservoir


def reservoir_sample_indices(n: int, k: int, seed: int) -> list[int]:
    # Sample k indices from [0..n-1]
    reservoir: list[int] = []
    i: int = 0
    while i < k and i < n:
        reservoir.append(i)
        i = i + 1
    state: int = seed
    while i < n:
        state = lcg_next(state)
        j: int = state % (i + 1)
        if j < k:
            reservoir[j] = i
        i = i + 1
    return reservoir


def contains(arr: list[int], val: int) -> int:
    i: int = 0
    while i < len(arr):
        if arr[i] == val:
            return 1
        i = i + 1
    return 0


def all_unique(arr: list[int]) -> int:
    i: int = 0
    while i < len(arr):
        j: int = i + 1
        while j < len(arr):
            if arr[i] == arr[j]:
                return 0
            j = j + 1
        i = i + 1
    return 1


def min_val(arr: list[int]) -> int:
    if len(arr) == 0:
        return 0
    result: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] < result:
            result = arr[i]
        i = i + 1
    return result


def max_val(arr: list[int]) -> int:
    if len(arr) == 0:
        return 0
    result: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] > result:
            result = arr[i]
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0

    # Test 1: sample size correct
    data: list[int] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
    sample: list[int] = reservoir_sample(data, 3, 42)
    if len(sample) == 3:
        passed = passed + 1

    # Test 2: all sampled values are from data
    all_in: int = 1
    i: int = 0
    while i < len(sample):
        if contains(data, sample[i]) == 0:
            all_in = 0
        i = i + 1
    if all_in == 1:
        passed = passed + 1

    # Test 3: deterministic with same seed
    sample2: list[int] = reservoir_sample(data, 3, 42)
    same: int = 1
    j: int = 0
    while j < 3:
        if sample[j] != sample2[j]:
            same = 0
        j = j + 1
    if same == 1:
        passed = passed + 1

    # Test 4: different seed gives different result (usually)
    sample3: list[int] = reservoir_sample(data, 3, 123)
    differs: int = 0
    k: int = 0
    while k < 3:
        if sample[k] != sample3[k]:
            differs = 1
        k = k + 1
    if differs == 1:
        passed = passed + 1

    # Test 5: k >= n returns all elements
    small: list[int] = [1, 2, 3]
    full: list[int] = reservoir_sample(small, 5, 42)
    if len(full) == 3:
        passed = passed + 1

    # Test 6: index sampling in range
    indices: list[int] = reservoir_sample_indices(100, 5, 42)
    if len(indices) == 5 and min_val(indices) >= 0 and max_val(indices) < 100:
        passed = passed + 1

    # Test 7: LCG produces different values
    s1: int = lcg_next(42)
    s2: int = lcg_next(s1)
    s3: int = lcg_next(s2)
    if s1 != s2 and s2 != s3 and s1 != s3:
        passed = passed + 1

    return passed
