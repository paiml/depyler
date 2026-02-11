"""Partition into equal sum subsets using DP."""


def array_sum(arr: list[int]) -> int:
    """Compute sum of array elements."""
    total: int = 0
    i: int = 0
    n: int = len(arr)
    while i < n:
        total = total + arr[i]
        i = i + 1
    return total


def can_partition(arr: list[int]) -> int:
    """Check if array can be split into two equal-sum subsets. Returns 1/0."""
    total: int = array_sum(arr)
    if total % 2 != 0:
        return 0
    target: int = total // 2
    dp: list[int] = []
    j: int = 0
    while j <= target:
        dp.append(0)
        j = j + 1
    dp[0] = 1
    i: int = 0
    n: int = len(arr)
    while i < n:
        s: int = target
        while s >= arr[i]:
            if dp[s - arr[i]] == 1:
                dp[s] = 1
            s = s - 1
        i = i + 1
    return dp[target]


def subset_sum_count(arr: list[int], target: int) -> int:
    """Count number of subsets that sum to target."""
    dp: list[int] = []
    j: int = 0
    while j <= target:
        dp.append(0)
        j = j + 1
    dp[0] = 1
    i: int = 0
    n: int = len(arr)
    while i < n:
        s: int = target
        while s >= arr[i]:
            dp[s] = dp[s] + dp[s - arr[i]]
            s = s - 1
        i = i + 1
    return dp[target]


def min_subset_diff(arr: list[int]) -> int:
    """Minimum difference between two subset sums."""
    total: int = array_sum(arr)
    half: int = total // 2
    dp: list[int] = []
    j: int = 0
    while j <= half:
        dp.append(0)
        j = j + 1
    dp[0] = 1
    i: int = 0
    n: int = len(arr)
    while i < n:
        s: int = half
        while s >= arr[i]:
            if dp[s - arr[i]] == 1:
                dp[s] = 1
            s = s - 1
        i = i + 1
    best: int = 0
    k: int = 0
    while k <= half:
        if dp[k] == 1:
            best = k
        k = k + 1
    return total - 2 * best


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [1, 5, 11, 5]
    if can_partition(arr1) == 1:
        passed = passed + 1

    arr2: list[int] = [1, 2, 3, 5]
    if can_partition(arr2) == 0:
        passed = passed + 1

    arr3: list[int] = [1, 2, 3]
    if subset_sum_count(arr3, 3) == 2:
        passed = passed + 1

    arr4: list[int] = [1, 1, 1, 1]
    if subset_sum_count(arr4, 2) == 6:
        passed = passed + 1

    arr5: list[int] = [3, 1, 4, 2, 2, 1]
    if min_subset_diff(arr5) == 1:
        passed = passed + 1

    if array_sum(arr1) == 22:
        passed = passed + 1

    arr6: list[int] = [1, 2, 3, 4, 5, 6, 7]
    if can_partition(arr6) == 1:
        passed = passed + 1

    return passed
