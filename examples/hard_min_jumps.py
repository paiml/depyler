"""Minimum jumps to reach end of array.

Tests: small arrays, single element, unreachable, known results, greedy vs DP.
"""


def min_jumps_dp(arr: list[int]) -> int:
    """Return minimum jumps to reach last index, or -1 if impossible. Uses DP."""
    n: int = len(arr)
    if n <= 1:
        return 0
    if arr[0] == 0:
        return -1
    dp: list[int] = []
    i: int = 0
    while i < n:
        dp.append(999999999)
        i = i + 1
    dp[0] = 0
    i = 1
    while i < n:
        j: int = 0
        while j < i:
            if dp[j] != 999999999 and j + arr[j] >= i:
                candidate: int = dp[j] + 1
                if candidate < dp[i]:
                    dp[i] = candidate
            j = j + 1
        i = i + 1
    if dp[n - 1] == 999999999:
        return -1
    return dp[n - 1]


def min_jumps_greedy(arr: list[int]) -> int:
    """Return minimum jumps using greedy approach."""
    n: int = len(arr)
    if n <= 1:
        return 0
    if arr[0] == 0:
        return -1
    jumps: int = 0
    current_end: int = 0
    farthest: int = 0
    i: int = 0
    while i < n - 1:
        candidate: int = i + arr[i]
        if candidate > farthest:
            farthest = candidate
        if i == current_end:
            jumps = jumps + 1
            current_end = farthest
            if current_end >= n - 1:
                return jumps
        i = i + 1
    if current_end < n - 1:
        return -1
    return jumps


def can_reach_end(arr: list[int]) -> int:
    """Return 1 if last index is reachable, 0 otherwise."""
    n: int = len(arr)
    if n <= 1:
        return 1
    max_reach: int = 0
    i: int = 0
    while i < n:
        if i > max_reach:
            return 0
        candidate: int = i + arr[i]
        if candidate > max_reach:
            max_reach = candidate
        if max_reach >= n - 1:
            return 1
        i = i + 1
    return 0


def test_module() -> int:
    """Test minimum jumps algorithms."""
    ok: int = 0

    arr1: list[int] = [2, 3, 1, 1, 4]
    if min_jumps_dp(arr1) == 2:
        ok = ok + 1
    if min_jumps_greedy(arr1) == 2:
        ok = ok + 1

    arr2: list[int] = [1, 1, 1, 1]
    if min_jumps_dp(arr2) == 3:
        ok = ok + 1

    arr3: list[int] = [5]
    if min_jumps_dp(arr3) == 0:
        ok = ok + 1

    arr4: list[int] = [0, 1, 2]
    if min_jumps_dp(arr4) == -1:
        ok = ok + 1

    arr5: list[int] = [2, 3, 0, 1, 4]
    if min_jumps_greedy(arr5) == 2:
        ok = ok + 1

    if can_reach_end(arr1) == 1:
        ok = ok + 1
    if can_reach_end(arr4) == 0:
        ok = ok + 1

    arr6: list[int] = [1, 3, 5, 8, 9, 2, 6, 7, 6, 8, 9]
    if min_jumps_greedy(arr6) == 3:
        ok = ok + 1

    arr7: list[int] = [10, 0, 0, 0, 0]
    if min_jumps_dp(arr7) == 1:
        ok = ok + 1

    return ok
