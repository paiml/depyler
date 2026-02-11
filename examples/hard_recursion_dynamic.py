"""Pathological recursion and dynamic programming patterns for transpiler stress testing.

Tests memoized recursion (manual dict cache), top-down/bottom-up DP,
tree recursion, backtracking, divide-and-conquer, and mutual recursion.
"""

from typing import List, Dict, Tuple, Optional


class BinaryTreeNode:
    """Binary tree node for tree recursion tests."""

    def __init__(self, value: int, left: Optional["BinaryTreeNode"] = None,
                 right: Optional["BinaryTreeNode"] = None):
        self.value = value
        self.left = left
        self.right = right


# --- Memoized recursion (manual dict cache, NOT functools) ---

def fib_memo(n: int, cache: Optional[Dict[int, int]] = None) -> int:
    """Fibonacci with manual memoization."""
    if cache is None:
        cache = {}
    if n in cache:
        return cache[n]
    if n <= 1:
        return n
    result = fib_memo(n - 1, cache) + fib_memo(n - 2, cache)
    cache[n] = result
    return result


def catalan_memo(n: int, cache: Optional[Dict[int, int]] = None) -> int:
    """Catalan number with manual memoization."""
    if cache is None:
        cache = {}
    if n in cache:
        return cache[n]
    if n <= 1:
        return 1
    result = 0
    for i in range(n):
        result += catalan_memo(i, cache) * catalan_memo(n - 1 - i, cache)
    cache[n] = result
    return result


def partition_count(n: int, k: int, cache: Optional[Dict[Tuple[int, int], int]] = None) -> int:
    """Count ways to partition n using parts up to k, with memoization."""
    if cache is None:
        cache = {}
    key = (n, k)
    if key in cache:
        return cache[key]
    if n == 0:
        return 1
    if n < 0 or k == 0:
        return 0
    result = partition_count(n, k - 1, cache) + partition_count(n - k, k, cache)
    cache[key] = result
    return result


# --- Top-down DP ---

def longest_common_subsequence(s1: str, s2: str) -> int:
    """LCS length using top-down DP with memoization."""
    cache: Dict[Tuple[int, int], int] = {}

    def helper(i: int, j: int) -> int:
        if (i, j) in cache:
            return cache[(i, j)]
        if i == len(s1) or j == len(s2):
            return 0
        if s1[i] == s2[j]:
            result = 1 + helper(i + 1, j + 1)
        else:
            result = max(helper(i + 1, j), helper(i, j + 1))
        cache[(i, j)] = result
        return result

    return helper(0, 0)


def edit_distance_td(s1: str, s2: str) -> int:
    """Edit distance using top-down DP."""
    cache: Dict[Tuple[int, int], int] = {}

    def helper(i: int, j: int) -> int:
        if (i, j) in cache:
            return cache[(i, j)]
        if i == 0:
            return j
        if j == 0:
            return i
        if s1[i - 1] == s2[j - 1]:
            result = helper(i - 1, j - 1)
        else:
            result = 1 + min(
                helper(i - 1, j),      # delete
                helper(i, j - 1),      # insert
                helper(i - 1, j - 1),  # replace
            )
        cache[(i, j)] = result
        return result

    return helper(len(s1), len(s2))


# --- Bottom-up DP ---

def knapsack_01(weights: List[int], values: List[int], capacity: int) -> int:
    """0/1 knapsack problem, bottom-up DP."""
    n = len(weights)
    dp: List[List[int]] = []
    for i in range(n + 1):
        row = [0] * (capacity + 1)
        dp.append(row)

    for i in range(1, n + 1):
        for w in range(capacity + 1):
            dp[i][w] = dp[i - 1][w]
            if weights[i - 1] <= w:
                include = dp[i - 1][w - weights[i - 1]] + values[i - 1]
                if include > dp[i][w]:
                    dp[i][w] = include

    return dp[n][capacity]


def coin_change(coins: List[int], amount: int) -> int:
    """Minimum coins to make amount, bottom-up DP. Returns -1 if impossible."""
    INF = amount + 1
    dp = [INF] * (amount + 1)
    dp[0] = 0
    for a in range(1, amount + 1):
        for coin in coins:
            if coin <= a and dp[a - coin] + 1 < dp[a]:
                dp[a] = dp[a - coin] + 1
    if dp[amount] >= INF:
        return -1
    return dp[amount]


def longest_increasing_subseq(arr: List[int]) -> int:
    """Length of longest increasing subsequence, bottom-up DP."""
    if not arr:
        return 0
    n = len(arr)
    dp = [1] * n
    for i in range(1, n):
        for j in range(i):
            if arr[j] < arr[i] and dp[j] + 1 > dp[i]:
                dp[i] = dp[j] + 1
    max_len = 0
    for v in dp:
        if v > max_len:
            max_len = v
    return max_len


def max_subarray_sum(arr: List[int]) -> int:
    """Kadane's algorithm for maximum subarray sum."""
    if not arr:
        return 0
    max_sum = arr[0]
    current = arr[0]
    for i in range(1, len(arr)):
        if current + arr[i] > arr[i]:
            current = current + arr[i]
        else:
            current = arr[i]
        if current > max_sum:
            max_sum = current
    return max_sum


# --- Tree recursion ---

def tree_height(node: Optional[BinaryTreeNode]) -> int:
    """Compute height of binary tree."""
    if node is None:
        return 0
    left_h = tree_height(node.left)
    right_h = tree_height(node.right)
    return 1 + (left_h if left_h > right_h else right_h)


def tree_sum(node: Optional[BinaryTreeNode]) -> int:
    """Sum all values in binary tree."""
    if node is None:
        return 0
    return node.value + tree_sum(node.left) + tree_sum(node.right)


def tree_inorder(node: Optional[BinaryTreeNode]) -> List[int]:
    """In-order traversal of binary tree."""
    if node is None:
        return []
    result: List[int] = []
    result.extend(tree_inorder(node.left))
    result.append(node.value)
    result.extend(tree_inorder(node.right))
    return result


def tree_mirror(node: Optional[BinaryTreeNode]) -> Optional[BinaryTreeNode]:
    """Create a mirrored copy of binary tree."""
    if node is None:
        return None
    return BinaryTreeNode(
        node.value,
        tree_mirror(node.right),
        tree_mirror(node.left)
    )


def tree_max_path_sum(node: Optional[BinaryTreeNode]) -> int:
    """Maximum path sum in binary tree (node to node)."""
    max_val = [-(10**9)]

    def helper(n: Optional[BinaryTreeNode]) -> int:
        if n is None:
            return 0
        left_gain = max(helper(n.left), 0)
        right_gain = max(helper(n.right), 0)
        path_sum = n.value + left_gain + right_gain
        if path_sum > max_val[0]:
            max_val[0] = path_sum
        return n.value + max(left_gain, right_gain)

    helper(node)
    return max_val[0]


# --- Backtracking ---

def n_queens(n: int) -> int:
    """Count solutions to N-queens problem."""
    count = [0]

    def is_safe(board: List[int], row: int, col: int) -> bool:
        for prev_row in range(row):
            prev_col = board[prev_row]
            if prev_col == col:
                return False
            if abs(prev_row - row) == abs(prev_col - col):
                return False
        return True

    def solve(board: List[int], row: int):
        if row == n:
            count[0] += 1
            return
        for col in range(n):
            if is_safe(board, row, col):
                board[row] = col
                solve(board, row + 1)
                board[row] = -1

    solve([-1] * n, 0)
    return count[0]


def subset_sum_exists(nums: List[int], target: int) -> bool:
    """Check if any subset sums to target using backtracking."""
    def backtrack(index: int, current_sum: int) -> bool:
        if current_sum == target:
            return True
        if index >= len(nums) or current_sum > target:
            return False
        # Include nums[index]
        if backtrack(index + 1, current_sum + nums[index]):
            return True
        # Exclude nums[index]
        return backtrack(index + 1, current_sum)

    return backtrack(0, 0)


def generate_parentheses(n: int) -> List[str]:
    """Generate all valid parentheses combinations."""
    result: List[str] = []

    def backtrack(current: str, open_count: int, close_count: int):
        if len(current) == 2 * n:
            result.append(current)
            return
        if open_count < n:
            backtrack(current + "(", open_count + 1, close_count)
        if close_count < open_count:
            backtrack(current + ")", open_count, close_count + 1)

    backtrack("", 0, 0)
    return result


# --- Divide and conquer ---

def merge_sort(arr):
    """Merge sort - untyped on purpose."""
    if len(arr) <= 1:
        return arr
    mid = len(arr) // 2
    left = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])
    return merge(left, right)


def merge(left, right):
    """Merge two sorted arrays - untyped."""
    result = []
    i = 0
    j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    while i < len(left):
        result.append(left[i])
        i += 1
    while j < len(right):
        result.append(right[j])
        j += 1
    return result


def quick_select(arr, k):
    """Find kth smallest element using quickselect - untyped."""
    if len(arr) == 1:
        return arr[0]
    pivot = arr[len(arr) // 2]
    lows = [x for x in arr if x < pivot]
    highs = [x for x in arr if x > pivot]
    pivots = [x for x in arr if x == pivot]

    if k < len(lows):
        return quick_select(lows, k)
    elif k < len(lows) + len(pivots):
        return pivot
    else:
        return quick_select(highs, k - len(lows) - len(pivots))


def power_fast(base, exp):
    """Fast exponentiation by squaring - untyped."""
    if exp == 0:
        return 1
    if exp < 0:
        return 1.0 / power_fast(base, -exp)
    if exp % 2 == 0:
        half = power_fast(base, exp // 2)
        return half * half
    else:
        return base * power_fast(base, exp - 1)


# --- Mutual recursion ---

def is_even_recursive(n: int) -> bool:
    """Check if n is even via mutual recursion."""
    if n == 0:
        return True
    return is_odd_recursive(n - 1)


def is_odd_recursive(n: int) -> bool:
    """Check if n is odd via mutual recursion."""
    if n == 0:
        return False
    return is_even_recursive(n - 1)


def hofstadter_female(n):
    """Hofstadter female sequence - mutual recursion - untyped."""
    if n == 0:
        return 1
    return n - hofstadter_male(hofstadter_female(n - 1))


def hofstadter_male(n):
    """Hofstadter male sequence - mutual recursion - untyped."""
    if n == 0:
        return 0
    return n - hofstadter_female(hofstadter_male(n - 1))


# --- Untyped DP helpers ---

def count_paths_grid(rows, cols):
    """Count paths in grid from top-left to bottom-right - untyped."""
    dp = []
    for i in range(rows):
        row = []
        for j in range(cols):
            if i == 0 or j == 0:
                row.append(1)
            else:
                row.append(dp[i-1][j] + row[j-1])
        dp.append(row)
    return dp[rows-1][cols-1]


def rod_cutting(prices, length):
    """Rod cutting problem - untyped."""
    dp = [0] * (length + 1)
    for l in range(1, length + 1):
        max_val = -(10**9)
        for cut in range(1, l + 1):
            if cut <= len(prices):
                val = prices[cut - 1] + dp[l - cut]
                if val > max_val:
                    max_val = val
        dp[l] = max_val
    return dp[length]


# --- Helper to build test trees ---

def build_test_tree() -> BinaryTreeNode:
    """Build a test binary tree.

    Tree structure: 5 at root, left=3(left=1,right=4), right=8(right=10).
    """
    return BinaryTreeNode(
        5,
        BinaryTreeNode(3, BinaryTreeNode(1), BinaryTreeNode(4)),
        BinaryTreeNode(8, None, BinaryTreeNode(10)),
    )


# --- Typed test functions ---

def test_fibonacci_memo():
    """Test memoized fibonacci."""
    assert fib_memo(0) == 0
    assert fib_memo(1) == 1
    assert fib_memo(10) == 55
    assert fib_memo(20) == 6765
    assert fib_memo(30) == 832040
    return True


def test_catalan():
    """Test memoized Catalan numbers."""
    assert catalan_memo(0) == 1
    assert catalan_memo(1) == 1
    assert catalan_memo(2) == 2
    assert catalan_memo(3) == 5
    assert catalan_memo(4) == 14
    assert catalan_memo(5) == 42
    return True


def test_partition_count():
    """Test partition counting."""
    assert partition_count(4, 4) == 5  # 4, 3+1, 2+2, 2+1+1, 1+1+1+1
    assert partition_count(5, 5) == 7
    assert partition_count(0, 0) == 1
    return True


def test_lcs():
    """Test longest common subsequence."""
    assert longest_common_subsequence("abcde", "ace") == 3
    assert longest_common_subsequence("abc", "abc") == 3
    assert longest_common_subsequence("abc", "def") == 0
    assert longest_common_subsequence("", "abc") == 0
    return True


def test_edit_distance():
    """Test top-down edit distance."""
    assert edit_distance_td("kitten", "sitting") == 3
    assert edit_distance_td("", "abc") == 3
    assert edit_distance_td("abc", "abc") == 0
    assert edit_distance_td("horse", "ros") == 3
    return True


def test_knapsack():
    """Test 0/1 knapsack."""
    weights = [2, 3, 4, 5]
    values = [3, 4, 5, 6]
    assert knapsack_01(weights, values, 5) == 7
    assert knapsack_01(weights, values, 8) == 10
    assert knapsack_01(weights, values, 0) == 0
    return True


def test_coin_change():
    """Test minimum coin change."""
    assert coin_change([1, 5, 10, 25], 30) == 2  # 25 + 5
    assert coin_change([1, 5, 10], 11) == 2  # 10 + 1
    assert coin_change([2], 3) == -1
    assert coin_change([1], 0) == 0
    return True


def test_lis():
    """Test longest increasing subsequence."""
    assert longest_increasing_subseq([10, 9, 2, 5, 3, 7, 101, 18]) == 4
    assert longest_increasing_subseq([0, 1, 0, 3, 2, 3]) == 4
    assert longest_increasing_subseq([7, 7, 7, 7]) == 1
    assert longest_increasing_subseq([]) == 0
    return True


def test_max_subarray():
    """Test Kadane's algorithm."""
    assert max_subarray_sum([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6
    assert max_subarray_sum([1]) == 1
    assert max_subarray_sum([-1, -2, -3]) == -1
    return True


def test_tree_operations():
    """Test binary tree recursive operations."""
    tree = build_test_tree()
    assert tree_height(tree) == 3
    assert tree_sum(tree) == 31  # 5+3+8+1+4+10
    assert tree_inorder(tree) == [1, 3, 4, 5, 8, 10]

    mirrored = tree_mirror(tree)
    assert tree_inorder(mirrored) == [10, 8, 5, 4, 3, 1]
    return True


def test_tree_max_path():
    """Test maximum path sum in tree."""
    tree = build_test_tree()
    assert tree_max_path_sum(tree) == 30  # 4 + 3 + 5 + 8 + 10 = 30... wait
    # helper(10) = 10, helper(8) = 8 + 10 = 18
    assert tree_max_path_sum(tree) == 30
    return True


def test_backtracking() -> bool:
    """Test backtracking algorithms."""
    assert n_queens(4) == 2
    assert n_queens(5) == 10
    assert n_queens(1) == 1

    assert subset_sum_exists([3, 34, 4, 12, 5, 2], 9)
    assert not subset_sum_exists([3, 34, 4, 12, 5, 2], 100)

    parens = generate_parentheses(3)
    assert len(parens) == 5
    assert "()()()" in parens
    assert "((()))" in parens
    return True


def test_divide_and_conquer() -> bool:
    """Test merge sort and quickselect."""
    assert merge_sort([3, 1, 4, 1, 5, 9, 2, 6]) == [1, 1, 2, 3, 4, 5, 6, 9]
    assert merge_sort([]) == []
    assert merge_sort([1]) == [1]

    assert quick_select([3, 1, 4, 1, 5, 9, 2, 6], 0) == 1
    assert quick_select([3, 1, 4, 1, 5, 9, 2, 6], 7) == 9
    assert quick_select([7, 3, 5, 1, 9], 2) == 5

    assert power_fast(2, 10) == 1024
    assert power_fast(3, 0) == 1
    assert abs(power_fast(2, -3) - 0.125) < 1e-9
    return True


def test_mutual_recursion() -> bool:
    """Test mutual recursion patterns."""
    assert is_even_recursive(0) == True
    assert is_even_recursive(4) == True
    assert is_even_recursive(7) == False
    assert is_odd_recursive(3) == True
    assert is_odd_recursive(6) == False

    # Hofstadter sequences (first few values)
    f_vals = [hofstadter_female(i) for i in range(8)]
    m_vals = [hofstadter_male(i) for i in range(8)]
    assert f_vals == [1, 1, 2, 2, 3, 3, 4, 5]
    assert m_vals == [0, 0, 1, 2, 2, 3, 4, 4]
    return True


def test_grid_and_rod() -> bool:
    """Test untyped DP functions."""
    assert count_paths_grid(3, 3) == 6
    assert count_paths_grid(1, 1) == 1
    assert count_paths_grid(2, 3) == 3

    prices = [1, 5, 8, 9, 10, 17, 17, 20]
    assert rod_cutting(prices, 4) == 10  # Two pieces of length 2
    assert rod_cutting(prices, 8) == 22
    return True


def test_all() -> bool:
    """Run all tests."""
    assert test_fibonacci_memo()
    assert test_catalan()
    assert test_partition_count()
    assert test_lcs()
    assert test_edit_distance()
    assert test_knapsack()
    assert test_coin_change()
    assert test_lis()
    assert test_max_subarray()
    assert test_tree_operations()
    assert test_tree_max_path()
    assert test_backtracking()
    assert test_divide_and_conquer()
    assert test_mutual_recursion()
    assert test_grid_and_rod()
    return True


def main():
    """Entry point."""
    if test_all():
        print("hard_recursion_dynamic: ALL TESTS PASSED")
    else:
        print("hard_recursion_dynamic: TESTS FAILED")


if __name__ == "__main__":
    main()
