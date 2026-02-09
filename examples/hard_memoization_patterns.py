"""Hard memoization, lookup, and DP patterns that stress transpiler codegen.

Tests: mutable dict params (&mut HashMap), typed dict lookups with .get(),
nested list indexing (list[list[int]]), graph adjacency dicts, sliding window
counters, bottom-up DP tables, and matrix chain operations.

These patterns specifically target:
- Dict passed as mutable reference and mutated inside callee
- Complement-based hash lookups (two-sum family)
- Nested list-of-list indexing with computed indices
- Dict[int, List[int]] adjacency lists for BFS/DFS
- Sliding window dict counters with add/remove cycles
"""


def coin_change_memo(amount: int, coins: list[int], memo: dict[int, int]) -> int:
    """Minimum coins needed for amount using memoized recursion.

    Passes memo dict mutably to recursive calls. Tests &mut HashMap<i64,i64>
    parameter threading through recursion with dict mutation in callee.
    """
    if amount == 0:
        return 0
    if amount < 0:
        return -1
    if amount in memo:
        return memo[amount]
    min_coins: int = amount + 1
    for coin in coins:
        sub: int = coin_change_memo(amount - coin, coins, memo)
        if sub >= 0 and sub + 1 < min_coins:
            min_coins = sub + 1
    result: int = min_coins if min_coins <= amount else -1
    memo[amount] = result
    return result


def two_sum_indices(nums: list[int], target: int) -> list[int]:
    """Find pair of indices summing to target using complement lookup.

    Tests typed dict[int,int] creation, complement computation, dict
    containment check, and dict value retrieval all in one loop body.
    """
    seen: dict[int, int] = {}
    for i in range(len(nums)):
        complement: int = target - nums[i]
        if complement in seen:
            return [seen[complement], i]
        seen[nums[i]] = i
    return [-1, -1]


def three_sum_count(nums: list[int], target: int) -> int:
    """Count triplets summing to target using nested loops with dict lookup.

    Tests O(n^2) loop with inner dict probe. The pair_sums dict maps
    sum-values to their count, requiring dict mutation with += on existing keys.
    """
    n: int = len(nums)
    count: int = 0
    pair_sums: dict[int, int] = {}
    for i in range(n):
        for j in range(i + 1, n):
            s: int = nums[i] + nums[j]
            if s in pair_sums:
                pair_sums[s] += 1
            else:
                pair_sums[s] = 1
    for k in range(n):
        need: int = target - nums[k]
        if need in pair_sums:
            count += pair_sums[need]
    return count


def longest_increasing_subseq(nums: list[int]) -> int:
    """Longest increasing subsequence length via bottom-up DP.

    Tests list-of-int DP table with nested loop access using computed
    indices and conditional max tracking across the table.
    """
    n: int = len(nums)
    if n == 0:
        return 0
    dp: list[int] = []
    for i in range(n):
        dp.append(1)
    for i in range(1, n):
        for j in range(i):
            if nums[j] < nums[i]:
                candidate: int = dp[j] + 1
                if candidate > dp[i]:
                    dp[i] = candidate
    best: int = 0
    for val in dp:
        if val > best:
            best = val
    return best


def edit_distance_dp(word1: str, word2: str) -> int:
    """Levenshtein edit distance using 2D DP table.

    Tests list[list[int]] creation, nested indexing dp[i][j],
    row initialization via inner loop, and min-of-three computation
    across adjacent cells in the matrix.
    """
    m: int = len(word1)
    n: int = len(word2)
    dp: list[list[int]] = []
    for i in range(m + 1):
        row: list[int] = []
        for j in range(n + 1):
            row.append(0)
        dp.append(row)
    for i in range(m + 1):
        dp[i][0] = i
    for j in range(n + 1):
        dp[0][j] = j
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if word1[i - 1] == word2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1]
            else:
                replace_cost: int = dp[i - 1][j - 1] + 1
                delete_cost: int = dp[i - 1][j] + 1
                insert_cost: int = dp[i][j - 1] + 1
                best: int = replace_cost
                if delete_cost < best:
                    best = delete_cost
                if insert_cost < best:
                    best = insert_cost
                dp[i][j] = best
    return dp[m][n]


def knapsack_01(weights: list[int], values: list[int], capacity: int) -> int:
    """0/1 knapsack via bottom-up 2D DP table.

    Tests list[list[int]] with (n+1)x(capacity+1) dimensions,
    conditional cell updates referencing previous row, and nested
    max comparisons across the DP table.
    """
    n: int = len(weights)
    dp: list[list[int]] = []
    for i in range(n + 1):
        row: list[int] = []
        for w in range(capacity + 1):
            row.append(0)
        dp.append(row)
    for i in range(1, n + 1):
        for w in range(1, capacity + 1):
            dp[i][w] = dp[i - 1][w]
            if weights[i - 1] <= w:
                with_item: int = dp[i - 1][w - weights[i - 1]] + values[i - 1]
                if with_item > dp[i][w]:
                    dp[i][w] = with_item
    return dp[n][capacity]


def matrix_chain_order(dims: list[int]) -> int:
    """Minimum scalar multiplications for matrix chain via interval DP.

    Tests 2D DP table indexed by interval endpoints, triple-nested loop
    with computed split point, and min-tracking across partition choices.
    """
    n: int = len(dims) - 1
    dp: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(n):
            row.append(0)
        dp.append(row)
    for length in range(2, n + 1):
        for i in range(n - length + 1):
            j: int = i + length - 1
            dp[i][j] = 999999999
            for k in range(i, j):
                cost: int = dp[i][k] + dp[k + 1][j] + dims[i] * dims[k + 1] * dims[j + 1]
                if cost < dp[i][j]:
                    dp[i][j] = cost
    return dp[0][n - 1]


def bfs_shortest_path(adj: dict[int, list[int]], start: int, end: int) -> int:
    """BFS shortest path in unweighted graph using adjacency dict.

    Tests Dict[int, List[int]] adjacency list, dict containment,
    list-valued dict access, queue-based BFS with visited set tracking.
    """
    if start == end:
        return 0
    visited: dict[int, bool] = {}
    dist: dict[int, int] = {}
    queue: list[int] = [start]
    visited[start] = True
    dist[start] = 0
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head += 1
        if node in adj:
            neighbors: list[int] = adj[node]
            for neighbor in neighbors:
                if neighbor not in visited:
                    visited[neighbor] = True
                    dist[neighbor] = dist[node] + 1
                    if neighbor == end:
                        return dist[neighbor]
                    queue.append(neighbor)
    return -1


def dfs_count_components(n: int, edges: list[list[int]]) -> int:
    """Count connected components via iterative DFS.

    Tests building Dict[int, List[int]] from edge list, then iterative
    DFS with stack, visited tracking via dict[int, bool], and component
    counting across multiple DFS launches.
    """
    adj: dict[int, list[int]] = {}
    for i in range(n):
        adj[i] = []
    for edge in edges:
        u: int = edge[0]
        v: int = edge[1]
        adj[u].append(v)
        adj[v].append(u)
    visited: dict[int, bool] = {}
    components: int = 0
    for node in range(n):
        if node not in visited:
            components += 1
            stack: list[int] = [node]
            while len(stack) > 0:
                current: int = stack.pop()
                if current not in visited:
                    visited[current] = True
                    if current in adj:
                        for neighbor in adj[current]:
                            if neighbor not in visited:
                                stack.append(neighbor)
    return components


def sliding_window_distinct(nums: list[int], k: int) -> int:
    """Count windows of size k with all distinct elements.

    Tests dict[int, int] as frequency counter with increment on add,
    decrement on remove, key deletion when count reaches zero, and
    window-size tracking against dict length.
    """
    n: int = len(nums)
    if k > n or k <= 0:
        return 0
    freq: dict[int, int] = {}
    distinct_count: int = 0
    result: int = 0
    for i in range(k):
        val: int = nums[i]
        if val in freq:
            freq[val] += 1
        else:
            freq[val] = 1
            distinct_count += 1
    if distinct_count == k:
        result += 1
    for i in range(k, n):
        new_val: int = nums[i]
        if new_val in freq:
            freq[new_val] += 1
        else:
            freq[new_val] = 1
            distinct_count += 1
        old_val: int = nums[i - k]
        freq[old_val] -= 1
        if freq[old_val] == 0:
            distinct_count -= 1
        if distinct_count == k:
            result += 1
    return result


def max_sum_subarray_of_size_k(nums: list[int], k: int) -> int:
    """Maximum sum of any contiguous subarray of size k.

    Tests sliding window with running sum, element removal by index
    offset, and max tracking across window positions.
    """
    n: int = len(nums)
    if k > n or k <= 0:
        return 0
    window_sum: int = 0
    for i in range(k):
        window_sum += nums[i]
    max_sum: int = window_sum
    for i in range(k, n):
        window_sum += nums[i] - nums[i - k]
        if window_sum > max_sum:
            max_sum = window_sum
    return max_sum


def group_anagrams_count(words: list[str]) -> int:
    """Count distinct anagram groups using sorted-key dict lookup.

    Tests string sorting as dict key, dict[str, int] group counting,
    and string manipulation within a collection iteration.
    """
    groups: dict[str, int] = {}
    for word in words:
        chars: list[str] = []
        for ch in word:
            chars.append(ch)
        chars.sort()
        key: str = "".join(chars)
        if key in groups:
            groups[key] += 1
        else:
            groups[key] = 1
    count: int = 0
    for key in groups:
        count += 1
    return count


def climb_stairs_memo(n: int, memo: dict[int, int]) -> int:
    """Count ways to climb n stairs (1 or 2 steps) with memoization.

    Tests dict[int,int] memo parameter passed mutably through recursion,
    with base case returns and memo lookup/store in recursive case.
    """
    if n <= 0:
        return 0
    if n == 1:
        return 1
    if n == 2:
        return 2
    if n in memo:
        return memo[n]
    result: int = climb_stairs_memo(n - 1, memo) + climb_stairs_memo(n - 2, memo)
    memo[n] = result
    return result


def rob_houses_dp(houses: list[int]) -> int:
    """House robber: max sum of non-adjacent elements via DP.

    Tests DP with two running variables updated each iteration,
    conditional max selection, and edge-case handling for small inputs.
    """
    n: int = len(houses)
    if n == 0:
        return 0
    if n == 1:
        return houses[0]
    prev2: int = houses[0]
    prev1: int = houses[1]
    if houses[0] > houses[1]:
        prev1 = houses[0]
    for i in range(2, n):
        take: int = prev2 + houses[i]
        skip: int = prev1
        current: int = take
        if skip > take:
            current = skip
        prev2 = prev1
        prev1 = current
    return prev1


def lcs_length(s1: str, s2: str) -> int:
    """Longest common subsequence length via 2D DP.

    Tests list[list[int]] DP table with string character comparison
    at computed indices, max-of-two cell references, and diagonal
    cell propagation.
    """
    m: int = len(s1)
    n: int = len(s2)
    dp: list[list[int]] = []
    for i in range(m + 1):
        row: list[int] = []
        for j in range(n + 1):
            row.append(0)
        dp.append(row)
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
            else:
                above: int = dp[i - 1][j]
                left: int = dp[i][j - 1]
                if above > left:
                    dp[i][j] = above
                else:
                    dp[i][j] = left
    return dp[m][n]


def topological_sort_kahn(n: int, edges: list[list[int]]) -> list[int]:
    """Topological sort using Kahn's algorithm (BFS-based).

    Tests Dict[int, List[int]] adjacency list construction, dict[int,int]
    in-degree tracking, queue-based processing with dict mutation,
    and degree decrement with zero-check triggering queue insertion.
    """
    adj: dict[int, list[int]] = {}
    in_degree: dict[int, int] = {}
    for i in range(n):
        adj[i] = []
        in_degree[i] = 0
    for edge in edges:
        src: int = edge[0]
        dst: int = edge[1]
        adj[src].append(dst)
        in_degree[dst] += 1
    queue: list[int] = []
    for i in range(n):
        if in_degree[i] == 0:
            queue.append(i)
    result: list[int] = []
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head += 1
        result.append(node)
        if node in adj:
            for neighbor in adj[node]:
                in_degree[neighbor] -= 1
                if in_degree[neighbor] == 0:
                    queue.append(neighbor)
    return result


def min_path_sum_grid(grid: list[list[int]]) -> int:
    """Minimum path sum from top-left to bottom-right in a grid.

    Tests 2D list indexing with dp[i][j] referencing dp[i-1][j] and
    dp[i][j-1], row/column boundary initialization, and min-of-two
    path comparisons at each interior cell.
    """
    m: int = len(grid)
    if m == 0:
        return 0
    n: int = len(grid[0])
    dp: list[list[int]] = []
    for i in range(m):
        row: list[int] = []
        for j in range(n):
            row.append(0)
        dp.append(row)
    dp[0][0] = grid[0][0]
    for i in range(1, m):
        dp[i][0] = dp[i - 1][0] + grid[i][0]
    for j in range(1, n):
        dp[0][j] = dp[0][j - 1] + grid[0][j]
    for i in range(1, m):
        for j in range(1, n):
            from_above: int = dp[i - 1][j]
            from_left: int = dp[i][j - 1]
            if from_above < from_left:
                dp[i][j] = from_above + grid[i][j]
            else:
                dp[i][j] = from_left + grid[i][j]
    return dp[m - 1][n - 1]


def frequency_sort_score(nums: list[int]) -> int:
    """Score based on frequency-sorted elements.

    Tests dict[int,int] frequency building, sorting values by frequency
    using the dict, and score accumulation weighted by position.
    """
    freq: dict[int, int] = {}
    for num in nums:
        if num in freq:
            freq[num] += 1
        else:
            freq[num] = 1
    unique: list[int] = []
    for key in freq:
        unique.append(key)
    for i in range(len(unique)):
        for j in range(i + 1, len(unique)):
            if freq[unique[i]] > freq[unique[j]]:
                temp: int = unique[i]
                unique[i] = unique[j]
                unique[j] = temp
            elif freq[unique[i]] == freq[unique[j]] and unique[i] > unique[j]:
                temp2: int = unique[i]
                unique[i] = unique[j]
                unique[j] = temp2
    score: int = 0
    position: int = 1
    for val in unique:
        count: int = freq[val]
        score += val * count * position
        position += 1
    return score
