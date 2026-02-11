"""Longest increasing subsequence O(n^2)."""


def lis_length(arr: list[int]) -> int:
    """Find length of longest increasing subsequence."""
    n: int = len(arr)
    if n == 0:
        return 0
    dp: list[int] = []
    i: int = 0
    while i < n:
        dp.append(1)
        i = i + 1
    i = 1
    while i < n:
        j: int = 0
        while j < i:
            if arr[j] < arr[i] and dp[j] + 1 > dp[i]:
                dp[i] = dp[j] + 1
            j = j + 1
        i = i + 1
    best: int = 0
    k: int = 0
    while k < n:
        if dp[k] > best:
            best = dp[k]
        k = k + 1
    return best


def lis_sequence(arr: list[int]) -> list[int]:
    """Return one actual LIS (not just length)."""
    n: int = len(arr)
    if n == 0:
        return []
    dp: list[int] = []
    parent: list[int] = []
    i: int = 0
    while i < n:
        dp.append(1)
        parent.append(-1)
        i = i + 1
    i = 1
    while i < n:
        j: int = 0
        while j < i:
            if arr[j] < arr[i] and dp[j] + 1 > dp[i]:
                dp[i] = dp[j] + 1
                parent[i] = j
            j = j + 1
        i = i + 1
    # Find index of max
    best_idx: int = 0
    best_len: int = dp[0]
    k: int = 1
    while k < n:
        if dp[k] > best_len:
            best_len = dp[k]
            best_idx = k
        k = k + 1
    # Reconstruct
    result: list[int] = []
    idx: int = best_idx
    while idx != -1:
        result.append(arr[idx])
        idx = parent[idx]
    # Reverse
    left: int = 0
    right: int = len(result) - 1
    while left < right:
        tmp: int = result[left]
        result[left] = result[right]
        result[right] = tmp
        left = left + 1
        right = right - 1
    return result


def count_lis(arr: list[int]) -> int:
    """Count the number of longest increasing subsequences."""
    n: int = len(arr)
    if n == 0:
        return 0
    dp: list[int] = []
    cnt: list[int] = []
    i: int = 0
    while i < n:
        dp.append(1)
        cnt.append(1)
        i = i + 1
    i = 1
    while i < n:
        j: int = 0
        while j < i:
            if arr[j] < arr[i]:
                if dp[j] + 1 > dp[i]:
                    dp[i] = dp[j] + 1
                    cnt[i] = cnt[j]
                elif dp[j] + 1 == dp[i]:
                    cnt[i] = cnt[i] + cnt[j]
            j = j + 1
        i = i + 1
    best: int = 0
    k: int = 0
    while k < n:
        if dp[k] > best:
            best = dp[k]
        k = k + 1
    total: int = 0
    m: int = 0
    while m < n:
        if dp[m] == best:
            total = total + cnt[m]
        m = m + 1
    return total


def test_module() -> int:
    passed: int = 0

    if lis_length([10, 9, 2, 5, 3, 7, 101, 18]) == 4:
        passed = passed + 1

    if lis_length([0, 1, 0, 3, 2, 3]) == 4:
        passed = passed + 1

    if lis_length([7, 7, 7, 7]) == 1:
        passed = passed + 1

    if lis_length([]) == 0:
        passed = passed + 1

    seq1: list[int] = lis_sequence([10, 9, 2, 5, 3, 7, 101, 18])
    if len(seq1) == 4:
        passed = passed + 1

    c1: int = count_lis([1, 3, 5, 4, 7])
    if c1 == 2:
        passed = passed + 1

    if lis_length([1, 2, 3, 4, 5]) == 5:
        passed = passed + 1

    return passed
