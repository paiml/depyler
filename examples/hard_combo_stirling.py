"""Stirling numbers of the first and second kind."""


def stirling_second(n: int, k: int) -> int:
    """Stirling number of second kind S(n,k)."""
    if n == 0 and k == 0:
        return 1
    if n == 0 or k == 0 or k > n:
        return 0
    if k == 1 or k == n:
        return 1
    sz: int = (n + 1) * (k + 1)
    dp: list[int] = []
    i: int = 0
    while i < sz:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    ni: int = 1
    while ni <= n:
        ki: int = 1
        while ki <= k and ki <= ni:
            idx: int = ni * (k + 1) + ki
            idx1: int = (ni - 1) * (k + 1) + ki
            idx2: int = (ni - 1) * (k + 1) + (ki - 1)
            dp[idx] = ki * dp[idx1] + dp[idx2]
            ki = ki + 1
        ni = ni + 1
    return dp[n * (k + 1) + k]


def stirling_first_unsigned(n: int, k: int) -> int:
    """Unsigned Stirling number of first kind |s(n,k)|."""
    if n == 0 and k == 0:
        return 1
    if n == 0 or k == 0 or k > n:
        return 0
    sz: int = (n + 1) * (k + 1)
    dp: list[int] = []
    i: int = 0
    while i < sz:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    ni: int = 1
    while ni <= n:
        ki: int = 1
        while ki <= k and ki <= ni:
            idx: int = ni * (k + 1) + ki
            idx1: int = (ni - 1) * (k + 1) + ki
            idx2: int = (ni - 1) * (k + 1) + (ki - 1)
            dp[idx] = (ni - 1) * dp[idx1] + dp[idx2]
            ki = ki + 1
        ni = ni + 1
    return dp[n * (k + 1) + k]


def test_module() -> int:
    """Test Stirling numbers."""
    ok: int = 0
    if stirling_second(4, 2) == 7:
        ok = ok + 1
    if stirling_second(5, 3) == 25:
        ok = ok + 1
    if stirling_first_unsigned(4, 2) == 11:
        ok = ok + 1
    if stirling_first_unsigned(3, 1) == 2:
        ok = ok + 1
    if stirling_second(3, 3) == 1:
        ok = ok + 1
    return ok
