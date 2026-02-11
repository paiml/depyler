"""0/1 Knapsack problem using dynamic programming.

Tests: max value computation, zero capacity, single item, exact fit, large inputs.
"""


def knapsack(weights: list[int], values: list[int], capacity: int) -> int:
    """Return maximum value achievable within given capacity."""
    n: int = len(weights)
    dp: list[list[int]] = []
    i: int = 0
    while i <= n:
        row: list[int] = []
        j: int = 0
        while j <= capacity:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    i = 1
    while i <= n:
        w: int = 0
        while w <= capacity:
            if weights[i - 1] <= w:
                include: int = values[i - 1] + dp[i - 1][w - weights[i - 1]]
                exclude: int = dp[i - 1][w]
                if include > exclude:
                    dp[i][w] = include
                else:
                    dp[i][w] = exclude
            else:
                dp[i][w] = dp[i - 1][w]
            w = w + 1
        i = i + 1
    return dp[n][capacity]


def knapsack_items(weights: list[int], values: list[int], capacity: int) -> list[int]:
    """Return indices of items selected in optimal solution."""
    n: int = len(weights)
    dp: list[list[int]] = []
    i: int = 0
    while i <= n:
        row: list[int] = []
        j: int = 0
        while j <= capacity:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    i = 1
    while i <= n:
        w: int = 0
        while w <= capacity:
            if weights[i - 1] <= w:
                include: int = values[i - 1] + dp[i - 1][w - weights[i - 1]]
                exclude: int = dp[i - 1][w]
                if include > exclude:
                    dp[i][w] = include
                else:
                    dp[i][w] = exclude
            else:
                dp[i][w] = dp[i - 1][w]
            w = w + 1
        i = i + 1
    result: list[int] = []
    w = capacity
    i = n
    while i > 0:
        if dp[i][w] != dp[i - 1][w]:
            result.append(i - 1)
            w = w - weights[i - 1]
        i = i - 1
    return result


def test_module() -> int:
    """Test 0/1 knapsack implementations."""
    ok: int = 0

    w1: list[int] = [2, 3, 4, 5]
    v1: list[int] = [3, 4, 5, 6]
    if knapsack(w1, v1, 5) == 7:
        ok = ok + 1

    if knapsack(w1, v1, 0) == 0:
        ok = ok + 1

    w2: list[int] = [10]
    v2: list[int] = [100]
    if knapsack(w2, v2, 5) == 0:
        ok = ok + 1
    if knapsack(w2, v2, 10) == 100:
        ok = ok + 1

    w3: list[int] = [1, 2, 3]
    v3: list[int] = [6, 10, 12]
    if knapsack(w3, v3, 5) == 22:
        ok = ok + 1

    items: list[int] = knapsack_items(w3, v3, 5)
    if len(items) == 2:
        ok = ok + 1

    w4: list[int] = [1, 1, 1]
    v4: list[int] = [10, 20, 30]
    if knapsack(w4, v4, 2) == 50:
        ok = ok + 1

    w5: list[int] = [3, 4, 2]
    v5: list[int] = [4, 5, 3]
    if knapsack(w5, v5, 7) == 9:
        ok = ok + 1

    empty_w: list[int] = []
    empty_v: list[int] = []
    if knapsack(empty_w, empty_v, 10) == 0:
        ok = ok + 1

    if knapsack(w1, v1, 14) == 18:
        ok = ok + 1

    return ok
