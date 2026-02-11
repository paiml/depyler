"""Partition into two subsets with equal sum using DP."""


def array_sum(arr: list[int]) -> int:
    """Sum of array elements."""
    total: int = 0
    i: int = 0
    while i < len(arr):
        total = total + arr[i]
        i = i + 1
    return total


def can_partition(arr: list[int]) -> int:
    """Returns 1 if array can be partitioned into two equal-sum subsets."""
    total: int = array_sum(arr)
    if total % 2 != 0:
        return 0
    target: int = total // 2
    n: int = len(arr)
    dp: list[int] = []
    i: int = 0
    while i <= target:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    idx: int = 0
    while idx < n:
        j: int = target
        while j >= arr[idx]:
            if dp[j - arr[idx]] == 1:
                dp[j] = 1
            j = j - 1
        idx = idx + 1
    return dp[target]


def subset_sum_exists(arr: list[int], target: int) -> int:
    """Returns 1 if a subset with given sum exists."""
    if target < 0:
        return 0
    dp: list[int] = []
    i: int = 0
    while i <= target:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    idx: int = 0
    while idx < len(arr):
        j: int = target
        while j >= arr[idx]:
            if dp[j - arr[idx]] == 1:
                dp[j] = 1
            j = j - 1
        idx = idx + 1
    return dp[target]


def min_subset_diff(arr: list[int]) -> int:
    """Minimum difference between two subset sums."""
    total: int = array_sum(arr)
    half: int = total // 2
    dp: list[int] = []
    i: int = 0
    while i <= half:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    idx: int = 0
    while idx < len(arr):
        j: int = half
        while j >= arr[idx]:
            if dp[j - arr[idx]] == 1:
                dp[j] = 1
            j = j - 1
        idx = idx + 1
    best: int = 0
    i = 0
    while i <= half:
        if dp[i] == 1:
            best = i
        i = i + 1
    return total - 2 * best


def test_module() -> int:
    """Test partition equal sum."""
    ok: int = 0
    a1: list[int] = [1, 5, 11, 5]
    if can_partition(a1) == 1:
        ok = ok + 1
    a2: list[int] = [1, 2, 3, 5]
    if can_partition(a2) == 0:
        ok = ok + 1
    a3: list[int] = [1, 2, 3, 4]
    if can_partition(a3) == 1:
        ok = ok + 1
    if subset_sum_exists(a1, 11) == 1:
        ok = ok + 1
    if subset_sum_exists(a1, 100) == 0:
        ok = ok + 1
    if min_subset_diff(a1) == 0:
        ok = ok + 1
    a4: list[int] = [1, 6, 11, 5]
    if min_subset_diff(a4) == 1:
        ok = ok + 1
    return ok
