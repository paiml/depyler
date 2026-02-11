"""Maximum sum increasing subsequence.

Tests: general arrays, all decreasing, all same, single element, known results.
"""


def max_increasing_sum(arr: list[int]) -> int:
    """Return maximum sum of an increasing subsequence."""
    n: int = len(arr)
    if n == 0:
        return 0
    dp: list[int] = []
    i: int = 0
    while i < n:
        dp.append(arr[i])
        i = i + 1
    i = 1
    while i < n:
        j: int = 0
        while j < i:
            if arr[j] < arr[i]:
                candidate: int = dp[j] + arr[i]
                if candidate > dp[i]:
                    dp[i] = candidate
            j = j + 1
        i = i + 1
    best: int = dp[0]
    i = 1
    while i < n:
        if dp[i] > best:
            best = dp[i]
        i = i + 1
    return best


def longest_increasing_subseq_len(arr: list[int]) -> int:
    """Return length of longest increasing subsequence."""
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
            if arr[j] < arr[i]:
                candidate: int = dp[j] + 1
                if candidate > dp[i]:
                    dp[i] = candidate
            j = j + 1
        i = i + 1
    best: int = 1
    i = 0
    while i < n:
        if dp[i] > best:
            best = dp[i]
        i = i + 1
    return best


def count_increasing_subseq(arr: list[int]) -> int:
    """Return number of increasing subsequences."""
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
            if arr[j] < arr[i]:
                dp[i] = dp[i] + dp[j]
            j = j + 1
        i = i + 1
    total: int = 0
    i = 0
    while i < n:
        total = total + dp[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test maximum sum increasing subsequence."""
    ok: int = 0

    arr1: list[int] = [1, 101, 2, 3, 100, 4, 5]
    if max_increasing_sum(arr1) == 106:
        ok = ok + 1

    arr2: list[int] = [3, 4, 5, 10]
    if max_increasing_sum(arr2) == 22:
        ok = ok + 1

    arr3: list[int] = [10, 5, 4, 3]
    if max_increasing_sum(arr3) == 10:
        ok = ok + 1

    arr4: list[int] = [5]
    if max_increasing_sum(arr4) == 5:
        ok = ok + 1

    empty: list[int] = []
    if max_increasing_sum(empty) == 0:
        ok = ok + 1

    arr5: list[int] = [10, 22, 9, 33, 21, 50, 41, 60]
    if longest_increasing_subseq_len(arr5) == 5:
        ok = ok + 1

    arr6: list[int] = [1, 2, 3]
    if count_increasing_subseq(arr6) == 7:
        ok = ok + 1

    arr7: list[int] = [5, 5, 5]
    if max_increasing_sum(arr7) == 5:
        ok = ok + 1

    arr8: list[int] = [1, 2, 3, 4, 5]
    if max_increasing_sum(arr8) == 15:
        ok = ok + 1

    if longest_increasing_subseq_len(arr8) == 5:
        ok = ok + 1

    return ok
