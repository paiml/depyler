"""Monotone patterns: longest increasing subsequence, patience sort, LIS variants.

Tests: lis_length, lis_count, longest_non_decreasing, longest_bitonic.
"""


def lis_length(arr: list[int]) -> int:
    """Longest strictly increasing subsequence length using O(n^2) DP."""
    n: int = len(arr)
    if n == 0:
        return 0
    dp: list[int] = []
    i: int = 0
    while i < n:
        dp.append(1)
        i = i + 1
    j: int = 1
    while j < n:
        k: int = 0
        while k < j:
            if arr[k] < arr[j]:
                cand: int = dp[k] + 1
                if cand > dp[j]:
                    dp[j] = cand
            k = k + 1
        j = j + 1
    best: int = 0
    m: int = 0
    while m < n:
        if dp[m] > best:
            best = dp[m]
        m = m + 1
    return best


def lis_binary_search_length(arr: list[int]) -> int:
    """LIS length using patience sort (O(n log n))."""
    n: int = len(arr)
    if n == 0:
        return 0
    tails: list[int] = []
    i: int = 0
    while i < n:
        val: int = arr[i]
        pos: int = bs_lower_bound(tails, val)
        if pos == len(tails):
            tails.append(val)
        else:
            tails[pos] = val
        i = i + 1
    return len(tails)


def bs_lower_bound(tails: list[int], target: int) -> int:
    """Binary search for first element >= target."""
    lo: int = 0
    hi: int = len(tails)
    while lo < hi:
        mid: int = (lo + hi) // 2
        if tails[mid] < target:
            lo = mid + 1
        else:
            hi = mid
    return lo


def longest_non_decreasing(arr: list[int]) -> int:
    """Length of longest non-decreasing subsequence."""
    n: int = len(arr)
    if n == 0:
        return 0
    dp: list[int] = []
    i: int = 0
    while i < n:
        dp.append(1)
        i = i + 1
    j: int = 1
    while j < n:
        k: int = 0
        while k < j:
            if arr[k] <= arr[j]:
                cand: int = dp[k] + 1
                if cand > dp[j]:
                    dp[j] = cand
            k = k + 1
        j = j + 1
    best: int = 0
    m: int = 0
    while m < n:
        if dp[m] > best:
            best = dp[m]
        m = m + 1
    return best


def longest_decreasing_from(arr: list[int]) -> list[int]:
    """For each index, length of longest decreasing subseq starting there."""
    n: int = len(arr)
    dp: list[int] = []
    i: int = 0
    while i < n:
        dp.append(1)
        i = i + 1
    j: int = n - 2
    while j >= 0:
        k: int = j + 1
        while k < n:
            if arr[k] < arr[j]:
                cand: int = dp[k] + 1
                if cand > dp[j]:
                    dp[j] = cand
            k = k + 1
        j = j - 1
    return dp


def longest_bitonic(arr: list[int]) -> int:
    """Longest bitonic subsequence: increases then decreases."""
    n: int = len(arr)
    if n == 0:
        return 0
    inc: list[int] = []
    i: int = 0
    while i < n:
        inc.append(1)
        i = i + 1
    j: int = 1
    while j < n:
        k: int = 0
        while k < j:
            if arr[k] < arr[j]:
                cand: int = inc[k] + 1
                if cand > inc[j]:
                    inc[j] = cand
            k = k + 1
        j = j + 1
    dec: list[int] = []
    di: int = 0
    while di < n:
        dec.append(1)
        di = di + 1
    dj: int = n - 2
    while dj >= 0:
        dk: int = dj + 1
        while dk < n:
            if arr[dk] < arr[dj]:
                dcand: int = dec[dk] + 1
                if dcand > dec[dj]:
                    dec[dj] = dcand
            dk = dk + 1
        dj = dj - 1
    best: int = 0
    m: int = 0
    while m < n:
        total: int = inc[m] + dec[m] - 1
        if total > best:
            best = total
        m = m + 1
    return best


def count_lis(arr: list[int]) -> int:
    """Count the number of longest increasing subsequences."""
    n: int = len(arr)
    if n == 0:
        return 0
    dp_len: list[int] = []
    dp_cnt: list[int] = []
    i: int = 0
    while i < n:
        dp_len.append(1)
        dp_cnt.append(1)
        i = i + 1
    j: int = 1
    while j < n:
        k: int = 0
        while k < j:
            if arr[k] < arr[j]:
                new_len: int = dp_len[k] + 1
                if new_len > dp_len[j]:
                    dp_len[j] = new_len
                    dp_cnt[j] = dp_cnt[k]
                elif new_len == dp_len[j]:
                    dp_cnt[j] = dp_cnt[j] + dp_cnt[k]
            k = k + 1
        j = j + 1
    max_len: int = 0
    m: int = 0
    while m < n:
        if dp_len[m] > max_len:
            max_len = dp_len[m]
        m = m + 1
    total: int = 0
    m2: int = 0
    while m2 < n:
        if dp_len[m2] == max_len:
            total = total + dp_cnt[m2]
        m2 = m2 + 1
    return total


def test_module() -> int:
    """Test monotone pattern algorithms."""
    passed: int = 0

    if lis_length([10, 9, 2, 5, 3, 7, 101, 18]) == 4:
        passed = passed + 1

    if lis_binary_search_length([10, 9, 2, 5, 3, 7, 101, 18]) == 4:
        passed = passed + 1

    if longest_non_decreasing([1, 3, 3, 2, 5]) == 4:
        passed = passed + 1

    if longest_bitonic([1, 2, 5, 3, 2]) == 5:
        passed = passed + 1

    if longest_bitonic([1, 2, 3]) == 3:
        passed = passed + 1

    if count_lis([1, 3, 5, 4, 7]) == 2:
        passed = passed + 1

    if lis_length([]) == 0:
        passed = passed + 1

    return passed
