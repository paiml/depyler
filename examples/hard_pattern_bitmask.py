"""Bitmask DP: subset enumeration, assignment problem, set cover.

Tests: subset_enum, min_cost_assignment, count_set_bits, max_and_pair.
"""


def count_set_bits(n: int) -> int:
    """Count number of 1-bits in integer."""
    count: int = 0
    val: int = n
    while val > 0:
        count = count + (val & 1)
        val = val >> 1
    return count


def enumerate_subsets(n: int) -> list[int]:
    """Enumerate all subsets of {0..n-1} as bitmasks. Returns list of masks."""
    total: int = 1 << n
    result: list[int] = []
    mask: int = 0
    while mask < total:
        result.append(mask)
        mask = mask + 1
    return result


def subset_to_list(mask: int, n: int) -> list[int]:
    """Convert bitmask to list of set bit positions."""
    result: list[int] = []
    bit: int = 0
    while bit < n:
        if (mask >> bit) & 1 == 1:
            result.append(bit)
        bit = bit + 1
    return result


def bitmask_subset_sum(arr: list[int], target: int) -> int:
    """Check if any subset sums to target using bitmask enumeration. Returns 1 or 0."""
    n: int = len(arr)
    total: int = 1 << n
    mask: int = 0
    while mask < total:
        s: int = 0
        bit: int = 0
        while bit < n:
            if (mask >> bit) & 1 == 1:
                s = s + arr[bit]
            bit = bit + 1
        if s == target:
            return 1
        mask = mask + 1
    return 0


def min_cost_assignment(cost: list[list[int]], n: int) -> int:
    """Minimum cost assignment of n workers to n jobs using bitmask DP."""
    full_mask: int = (1 << n) - 1
    num_states: int = 1 << n
    dp: list[int] = []
    i: int = 0
    while i < num_states:
        dp.append(999999999)
        i = i + 1
    dp[0] = 0
    mask: int = 0
    while mask <= full_mask:
        if dp[mask] < 999999999:
            worker: int = count_set_bits(mask)
            if worker < n:
                job: int = 0
                while job < n:
                    bit: int = 1 << job
                    if (mask & bit) == 0:
                        new_mask: int = mask | bit
                        row: list[int] = cost[worker]
                        cand: int = dp[mask] + row[job]
                        if cand < dp[new_mask]:
                            dp[new_mask] = cand
                    job = job + 1
        mask = mask + 1
    return dp[full_mask]


def max_and_pair(arr: list[int]) -> int:
    """Find maximum AND value of any pair in array."""
    n: int = len(arr)
    if n < 2:
        return 0
    best: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            val: int = arr[i] & arr[j]
            if val > best:
                best = val
            j = j + 1
        i = i + 1
    return best


def max_xor_pair(arr: list[int]) -> int:
    """Find maximum XOR value of any pair in array."""
    n: int = len(arr)
    if n < 2:
        return 0
    best: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            val: int = arr[i] ^ arr[j]
            if val > best:
                best = val
            j = j + 1
        i = i + 1
    return best


def test_module() -> int:
    """Test bitmask algorithms."""
    passed: int = 0

    if count_set_bits(7) == 3:
        passed = passed + 1

    if count_set_bits(0) == 0:
        passed = passed + 1

    subs: list[int] = enumerate_subsets(3)
    if len(subs) == 8:
        passed = passed + 1

    sl: list[int] = subset_to_list(5, 3)
    if sl == [0, 2]:
        passed = passed + 1

    if bitmask_subset_sum([3, 7, 1, 8], 11) == 1:
        passed = passed + 1

    cost_m: list[list[int]] = [[9, 2, 7], [6, 4, 3], [5, 8, 1]]
    mc: int = min_cost_assignment(cost_m, 3)
    if mc == 7:
        passed = passed + 1

    if max_xor_pair([5, 3, 1, 6]) == 7:
        passed = passed + 1

    return passed
