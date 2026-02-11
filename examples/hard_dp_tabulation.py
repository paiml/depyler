"""Dynamic programming with tabulation patterns.

Tests: rod cutting, partition equal subset sum, longest common substring,
minimum jumps, and paint house coloring.
"""


def rod_cutting(prices: list[int], length: int) -> int:
    """Maximum revenue from cutting a rod of given length."""
    dp: list[int] = [0] * (length + 1)
    i: int = 1
    while i <= length:
        best: int = 0
        j: int = 1
        while j <= i and j <= len(prices):
            candidate: int = prices[j - 1] + dp[i - j]
            if candidate > best:
                best = candidate
            j = j + 1
        dp[i] = best
        i = i + 1
    return dp[length]


def can_partition(nums: list[int], n: int) -> bool:
    """Check if array can be partitioned into two subsets with equal sum."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + nums[i]
        i = i + 1
    if total % 2 != 0:
        return False
    target: int = total // 2
    dp: list[bool] = [False] * (target + 1)
    dp[0] = True
    i = 0
    while i < n:
        j: int = target
        while j >= nums[i]:
            if dp[j - nums[i]]:
                dp[j] = True
            j = j - 1
        i = i + 1
    return dp[target]


def longest_common_substr_len(s1: str, s2: str) -> int:
    """Length of longest common substring using DP tabulation."""
    m: int = len(s1)
    n: int = len(s2)
    best: int = 0
    prev: list[int] = [0] * (n + 1)
    i: int = 1
    while i <= m:
        curr: list[int] = [0] * (n + 1)
        j: int = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                curr[j] = prev[j - 1] + 1
                if curr[j] > best:
                    best = curr[j]
            j = j + 1
        prev = curr
        i = i + 1
    return best


def min_jumps(arr: list[int]) -> int:
    """Minimum jumps to reach end of array. Returns -1 if impossible."""
    n: int = len(arr)
    if n <= 1:
        return 0
    big: int = n + 1
    dp: list[int] = [big] * n
    dp[0] = 0
    i: int = 0
    while i < n:
        if dp[i] < big:
            j: int = 1
            while j <= arr[i] and i + j < n:
                candidate: int = dp[i] + 1
                if candidate < dp[i + j]:
                    dp[i + j] = candidate
                j = j + 1
        i = i + 1
    if dp[n - 1] >= big:
        return -1
    return dp[n - 1]


def paint_houses(costs: list[list[int]]) -> int:
    """Minimum cost to paint houses with 3 colors, no two adjacent same color."""
    n: int = len(costs)
    if n == 0:
        return 0
    prev_r: int = costs[0][0]
    prev_g: int = costs[0][1]
    prev_b: int = costs[0][2]
    i: int = 1
    while i < n:
        min_rg: int = prev_r
        if prev_g < min_rg:
            min_rg = prev_g
        min_rb: int = prev_r
        if prev_b < min_rb:
            min_rb = prev_b
        min_gb: int = prev_g
        if prev_b < min_gb:
            min_gb = prev_b
        curr_r: int = costs[i][0] + min_gb
        curr_g: int = costs[i][1] + min_rb
        curr_b: int = costs[i][2] + min_rg
        prev_r = curr_r
        prev_g = curr_g
        prev_b = curr_b
        i = i + 1
    result: int = prev_r
    if prev_g < result:
        result = prev_g
    if prev_b < result:
        result = prev_b
    return result


def test_module() -> bool:
    """Test all DP tabulation functions."""
    ok: bool = True

    if rod_cutting([1, 5, 8, 9, 10, 17, 17, 20], 8) != 22:
        ok = False
    if rod_cutting([3, 5, 8, 9], 4) != 12:
        ok = False

    if not can_partition([1, 5, 11, 5], 4):
        ok = False
    if can_partition([1, 2, 3, 5], 4):
        ok = False

    if longest_common_substr_len("abcdef", "zbcdf") != 3:
        ok = False
    if longest_common_substr_len("abc", "xyz") != 0:
        ok = False

    if min_jumps([2, 3, 1, 1, 4]) != 2:
        ok = False
    if min_jumps([1, 1, 1, 1]) != 3:
        ok = False

    costs: list[list[int]] = [[17, 2, 17], [16, 16, 5], [14, 3, 19]]
    if paint_houses(costs) != 10:
        ok = False

    return ok
