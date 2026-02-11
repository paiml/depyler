"""Pseudo-Random Number Generator implementations.

Multiple PRNG algorithms: LCG, Xorshift, Middle Square, combined generators.
All operate on integers with defined periods and statistical properties.
"""


def lcg_generate(seed: int, a_val: int, c_val: int, mod_val: int, count: int) -> list[int]:
    """Linear Congruential Generator sequence."""
    result: list[int] = []
    state: int = seed
    i: int = 0
    while i < count:
        state = (a_val * state + c_val) % mod_val
        result.append(state)
        i = i + 1
    return result


def xorshift32(state: int) -> int:
    """One step of xorshift32 PRNG (using 16-bit arithmetic)."""
    state = state % 65536
    shifted_left: int = (state * 8) % 65536
    state = xor_vals(state, shifted_left)
    shifted_right: int = state // 4
    state = xor_vals(state, shifted_right)
    shifted_left2: int = (state * 32) % 65536
    state = xor_vals(state, shifted_left2)
    return state % 65536


def xor_vals(a: int, b: int) -> int:
    """XOR two 16-bit values using arithmetic."""
    result: int = 0
    bit: int = 1
    pos: int = 0
    while pos < 16:
        ab: int = (a // bit) % 2
        bb: int = (b // bit) % 2
        if ab != bb:
            result = result + bit
        bit = bit * 2
        pos = pos + 1
    return result


def xorshift_generate(seed: int, count: int) -> list[int]:
    """Generate sequence using xorshift."""
    result: list[int] = []
    state: int = seed
    if state == 0:
        state = 1
    i: int = 0
    while i < count:
        state = xorshift32(state)
        result.append(state)
        i = i + 1
    return result


def middle_square(seed: int, count: int) -> list[int]:
    """Middle square PRNG. Operates on 4-digit numbers."""
    result: list[int] = []
    state: int = seed % 10000
    i: int = 0
    while i < count:
        squared: int = state * state
        state = (squared // 100) % 10000
        result.append(state)
        i = i + 1
    return result


def chi_squared_uniformity(values: list[int], num_bins: int) -> int:
    """Chi-squared test for uniformity * 1000. Lower = more uniform."""
    bins: list[int] = []
    bi: int = 0
    while bi < num_bins:
        bins.append(0)
        bi = bi + 1
    n: int = len(values)
    i: int = 0
    while i < n:
        vv: int = values[i]
        bucket: int = vv % num_bins
        old: int = bins[bucket]
        bins[bucket] = old + 1
        i = i + 1
    expected: int = n // num_bins
    if expected == 0:
        expected = 1
    chi_sq: int = 0
    j: int = 0
    while j < num_bins:
        observed: int = bins[j]
        diff: int = observed - expected
        chi_sq = chi_sq + diff * diff * 1000 // expected
        j = j + 1
    return chi_sq


def runs_test(values: list[int], threshold: int) -> int:
    """Count number of runs (consecutive values above/below threshold)."""
    if len(values) == 0:
        return 0
    runs: int = 1
    i: int = 1
    while i < len(values):
        prev: int = values[i - 1]
        curr: int = values[i]
        prev_above: int = 0
        curr_above: int = 0
        if prev >= threshold:
            prev_above = 1
        if curr >= threshold:
            curr_above = 1
        if prev_above != curr_above:
            runs = runs + 1
        i = i + 1
    return runs


def test_module() -> int:
    """Test PRNG implementations."""
    ok: int = 0
    lcg_seq: list[int] = lcg_generate(1, 1103, 12345, 65536, 10)
    if len(lcg_seq) == 10:
        ok = ok + 1
    xor_seq: list[int] = xorshift_generate(42, 10)
    if len(xor_seq) == 10:
        ok = ok + 1
    v0: int = xor_seq[0]
    v1: int = xor_seq[1]
    if v0 != v1:
        ok = ok + 1
    ms_seq: list[int] = middle_square(1234, 5)
    if len(ms_seq) == 5:
        ok = ok + 1
    runs: int = runs_test(lcg_seq, 32768)
    if runs >= 2:
        ok = ok + 1
    return ok
