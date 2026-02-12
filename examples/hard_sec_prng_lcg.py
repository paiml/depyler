from typing import List, Tuple

def lcg_next(state: int, a: int, c: int, m: int) -> int:
    return (a * state + c) % m

def lcg_generate(seed: int, a: int, c: int, m: int, count: int) -> List[int]:
    results: List[int] = []
    state: int = seed
    for i in range(count):
        state = (a * state + c) % m
        results.append(state)
    return results

def lcg_uniform_scaled(state: int, a: int, c: int, m: int) -> Tuple[int, int]:
    ns: int = (a * state + c) % m
    scaled: int = (ns * 10000) // m
    return (scaled, ns)

def test_period(seed: int, a: int, c: int, m: int, max_steps: int) -> int:
    state: int = (a * seed + c) % m
    initial: int = state
    count: int = 1
    while count < max_steps:
        state = (a * state + c) % m
        if state == initial:
            return count
        count = count + 1
    return max_steps

def chi_sq_scaled(values: List[int], bins: int, m: int) -> int:
    counts: List[int] = [0] * bins
    for v in values:
        idx: int = (v * bins) // m
        if idx >= bins:
            idx = bins - 1
        counts[idx] = counts[idx] + 1
    expected_times_bins: int = len(values)
    chi2: int = 0
    for b in counts:
        diff: int = b * bins - expected_times_bins
        chi2 = chi2 + (diff * diff) // (expected_times_bins * bins)
    return chi2
