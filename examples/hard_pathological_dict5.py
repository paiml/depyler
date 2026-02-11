# Pathological dict: Dict-based caching/memoization
# Tests: dict used as memo table for recursive-style computations
# Workaround: avoid dict[expr - expr] pattern. Use local var for computed key
# since transpiler generates dp.get(&(a) - (coin)) instead of dp.get(&(a - coin))


def fibonacci_memo(n: int, memo: dict[int, int]) -> int:
    """Compute fibonacci with memoization dict."""
    if n in memo:
        return memo[n]
    if n <= 0:
        return 0
    if n == 1:
        return 1
    memo[0] = 0
    memo[1] = 1
    i: int = 2
    while i <= n:
        if i not in memo:
            prev_idx1: int = i - 1
            prev_idx2: int = i - 2
            prev1: int = memo[prev_idx1]
            prev2: int = memo[prev_idx2]
            memo[i] = prev1 + prev2
        i = i + 1
    return memo[n]


def tribonacci_memo(n: int, memo: dict[int, int]) -> int:
    """Compute tribonacci (sum of 3 previous) with memo."""
    memo[0] = 0
    memo[1] = 0
    memo[2] = 1
    i: int = 3
    while i <= n:
        if i not in memo:
            idx1: int = i - 1
            idx2: int = i - 2
            idx3: int = i - 3
            p1: int = memo[idx1]
            p2: int = memo[idx2]
            p3: int = memo[idx3]
            memo[i] = p1 + p2 + p3
        i = i + 1
    return memo[n]


def coin_change_count(amount: int, coins: list[int]) -> int:
    """Count number of ways to make change using DP with dict cache."""
    dp: dict[int, int] = {}
    dp[0] = 1
    c: int = 0
    while c < len(coins):
        coin: int = coins[c]
        a: int = coin
        while a <= amount:
            prev_amount: int = a - coin
            if prev_amount in dp:
                if a in dp:
                    dp[a] = dp[a] + dp[prev_amount]
                else:
                    dp[a] = dp[prev_amount]
            a = a + 1
        c = c + 1
    if amount in dp:
        return dp[amount]
    return 0


def stair_climb_ways(n: int) -> int:
    """Count ways to climb n stairs taking 1 or 2 steps at a time."""
    dp: dict[int, int] = {}
    dp[0] = 1
    dp[1] = 1
    i: int = 2
    while i <= n:
        idx1: int = i - 1
        idx2: int = i - 2
        dp[i] = dp[idx1] + dp[idx2]
        i = i + 1
    return dp[n]


def longest_increasing_subsequence_len(nums: list[int]) -> int:
    """Compute length of longest increasing subsequence using DP."""
    if len(nums) == 0:
        return 0
    dp: dict[int, int] = {}
    i: int = 0
    while i < len(nums):
        dp[i] = 1
        j: int = 0
        while j < i:
            if nums[j] < nums[i]:
                candidate: int = dp[j] + 1
                if candidate > dp[i]:
                    dp[i] = candidate
            j = j + 1
        i = i + 1
    best: int = 0
    k: int = 0
    while k < len(nums):
        if dp[k] > best:
            best = dp[k]
        k = k + 1
    return best


def test_module() -> int:
    passed: int = 0
    # Test 1: fibonacci
    memo: dict[int, int] = {}
    if fibonacci_memo(10, memo) == 55:
        passed = passed + 1
    # Test 2: fibonacci uses memo
    if fibonacci_memo(10, memo) == 55:
        passed = passed + 1
    # Test 3: tribonacci (0,0,1,1,2,4,7,13,24,44)
    tmemo: dict[int, int] = {}
    if tribonacci_memo(7, tmemo) == 13:
        passed = passed + 1
    # Test 4: coin change (1,2,5 for amount 5 -> 4 ways)
    if coin_change_count(5, [1, 2, 5]) == 4:
        passed = passed + 1
    # Test 5: stair climb (n=5 -> 8 ways)
    if stair_climb_ways(5) == 8:
        passed = passed + 1
    # Test 6: LIS
    if longest_increasing_subsequence_len([10, 9, 2, 5, 3, 7, 101, 18]) == 4:
        passed = passed + 1
    # Test 7: LIS empty
    if longest_increasing_subsequence_len([]) == 0:
        passed = passed + 1
    return passed
