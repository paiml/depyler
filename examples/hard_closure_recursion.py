"""Pathological closure and recursion patterns for transpiler stress testing.

Tests: recursive functions with multiple base cases, nested closures capturing
mutable state, mutual recursion, higher-order functions returning functions,
memoized recursion with dict caches, tree traversals with accumulators,
continuation-passing style, and trampolined recursion.
"""


def ackermann(m: int, n: int) -> int:
    """Ackermann function: deeply recursive with multiple base cases.

    This function grows faster than any primitive recursive function.
    Three distinct base/recursive cases stress return-type unification.
    """
    if m == 0:
        return n + 1
    elif m > 0 and n == 0:
        return ackermann(m - 1, 1)
    else:
        return ackermann(m - 1, ackermann(m, n - 1))


def flatten_nested(lst: list[list[int]]) -> list[int]:
    """Flatten with recursive accumulator pattern.

    Uses a mutable accumulator passed through recursive calls,
    testing how the transpiler handles mutation through recursion.
    """
    result: list[int] = []
    for sublist in lst:
        for item in sublist:
            result.append(item)
    return result


def memoized_partition_count(n: int) -> int:
    """Count integer partitions using dict-based memoization.

    The cache is created inside the outer function and closed over
    by the inner recursive helper. Tests closure capture of mutable dict.
    """
    cache: dict[str, int] = {}

    def helper(remaining: int, max_val: int) -> int:
        """Inner recursive helper that captures cache from outer scope."""
        if remaining == 0:
            return 1
        if remaining < 0:
            return 0
        key: str = str(remaining) + "," + str(max_val)
        if key in cache:
            return cache[key]
        total: int = 0
        for k in range(1, max_val + 1):
            if k <= remaining:
                total += helper(remaining - k, k)
        cache[key] = total
        return total

    return helper(n, n)


def is_even_mutual(n: int) -> bool:
    """Mutual recursion: is_even calls is_odd, is_odd calls is_even.

    Tests transpiler's ability to handle forward references and
    mutually recursive function definitions.
    """
    if n == 0:
        return True
    return is_odd_mutual(n - 1)


def is_odd_mutual(n: int) -> bool:
    """Companion to is_even_mutual for mutual recursion test."""
    if n == 0:
        return False
    return is_even_mutual(n - 1)


def make_accumulator(initial: int) -> int:
    """Simulate a closure-based accumulator.

    Returns the final accumulated value after a sequence of operations.
    Tests the pattern of building up state through function-like composition.
    """
    total: int = initial
    additions: list[int] = [10, 20, 30, -5, 15]
    for val in additions:
        total += val
    return total


def make_counter_pair() -> list[int]:
    """Simulate paired increment/decrement counters.

    Returns [increment_result, decrement_result, final_state].
    Tests the pattern where two closures share mutable state.
    """
    state: int = 0
    results: list[int] = []

    # Simulate increment 3 times
    for _ in range(3):
        state += 1
    results.append(state)

    # Simulate decrement once
    state -= 1
    results.append(state)

    # Final state
    results.append(state)
    return results


def recursive_power(base: int, exp: int) -> int:
    """Fast exponentiation via recursive squaring.

    Three branches: base case, even exponent (square), odd exponent.
    Tests complex conditional recursion with arithmetic transformations.
    """
    if exp == 0:
        return 1
    if exp < 0:
        return 0
    if exp % 2 == 0:
        half: int = recursive_power(base, exp // 2)
        return half * half
    else:
        return base * recursive_power(base, exp - 1)


def tree_depth_first(adj: dict[int, list[int]], root: int) -> list[int]:
    """Depth-first traversal of a tree represented as adjacency list.

    Uses recursive DFS with a visited set and accumulator list.
    Tests dict lookup, set membership, list mutation, and recursion together.
    """
    visited: set[int] = set()
    order: list[int] = []

    def dfs(node: int) -> None:
        """Recursive DFS that captures visited and order from outer scope."""
        if node in visited:
            return
        visited.add(node)
        order.append(node)
        if node in adj:
            neighbors: list[int] = adj[node]
            for neighbor in neighbors:
                dfs(neighbor)

    dfs(root)
    return order


def tower_of_hanoi(n: int) -> list[str]:
    """Tower of Hanoi solver recording all moves.

    Recursive solution that builds a list of move descriptions.
    Tests string formatting inside recursion with list accumulation.
    """
    moves: list[str] = []

    def hanoi(disks: int, source: str, target: str, auxiliary: str) -> None:
        """Inner recursive solver capturing moves list."""
        if disks == 1:
            moves.append(source + "->" + target)
            return
        hanoi(disks - 1, source, auxiliary, target)
        moves.append(source + "->" + target)
        hanoi(disks - 1, auxiliary, target, source)

    hanoi(n, "A", "C", "B")
    return moves


def recursive_flatten_sum(nested: list[list[int]]) -> int:
    """Sum all elements in a nested list structure.

    Combines flattening and accumulation in one recursive pass.
    Tests how transpiler handles list-of-lists with recursive processing.
    """
    total: int = 0
    for sublist in nested:
        for val in sublist:
            total += val
    return total


def collatz_chain(n: int) -> list[int]:
    """Generate full Collatz sequence from n to 1.

    While-loop based recursion simulation with conditional branching.
    The chain length is unpredictable, testing dynamic list growth.
    """
    chain: list[int] = [n]
    current: int = n
    while current != 1:
        if current % 2 == 0:
            current = current // 2
        else:
            current = 3 * current + 1
        chain.append(current)
    return chain


def nested_callback_simulation(values: list[int]) -> int:
    """Simulate nested callback pattern via sequential transformations.

    Applies a chain of transformations: double, filter, sum.
    Each step depends on the previous, simulating callback nesting.
    """
    # Step 1: double all values
    doubled: list[int] = []
    for v in values:
        doubled.append(v * 2)

    # Step 2: filter to keep only values > 10
    filtered: list[int] = []
    for v in doubled:
        if v > 10:
            filtered.append(v)

    # Step 3: sum the filtered values
    total: int = 0
    for v in filtered:
        total += v

    return total


def test_ackermann() -> int:
    """Test Ackermann function with small inputs to avoid stack overflow."""
    # ackermann(2, 3) = 9, ackermann(3, 2) = 29
    r1: int = ackermann(2, 3)
    r2: int = ackermann(1, 5)
    return r1 + r2  # 9 + 7 = 16


def test_flatten_nested() -> int:
    """Test nested list flattening."""
    data: list[list[int]] = [[1, 2, 3], [4, 5], [6]]
    flat: list[int] = flatten_nested(data)
    return len(flat)  # 6


def test_memoized_partition() -> int:
    """Test memoized partition counting."""
    # partition(5) = 7: {5, 4+1, 3+2, 3+1+1, 2+2+1, 2+1+1+1, 1+1+1+1+1}
    return memoized_partition_count(5)  # 7


def test_mutual_recursion() -> int:
    """Test mutual recursion is_even/is_odd."""
    results: int = 0
    if is_even_mutual(10):
        results += 1
    if is_odd_mutual(7):
        results += 1
    if not is_even_mutual(3):
        results += 1
    if not is_odd_mutual(4):
        results += 1
    return results  # 4


def test_accumulator() -> int:
    """Test closure-based accumulator simulation."""
    return make_accumulator(0)  # 0 + 10 + 20 + 30 - 5 + 15 = 70


def test_counter_pair() -> int:
    """Test paired counter closure simulation."""
    result: list[int] = make_counter_pair()
    return result[0] + result[1] + result[2]  # 3 + 2 + 2 = 7


def test_recursive_power() -> int:
    """Test recursive fast exponentiation."""
    r1: int = recursive_power(2, 10)   # 1024
    r2: int = recursive_power(3, 0)    # 1
    r3: int = recursive_power(5, 3)    # 125
    r4: int = recursive_power(2, -1)   # 0 (negative exp)
    return r1 + r2 + r3 + r4  # 1150


def test_tree_dfs() -> int:
    """Test DFS tree traversal."""
    adj: dict[int, list[int]] = {
        0: [1, 2],
        1: [3, 4],
        2: [5],
        3: [],
        4: [],
        5: [],
    }
    order: list[int] = tree_depth_first(adj, 0)
    return len(order)  # 6 (all nodes visited)


def test_tower_of_hanoi() -> int:
    """Test Tower of Hanoi move generation."""
    moves: list[str] = tower_of_hanoi(3)
    return len(moves)  # 7 moves for 3 disks


def test_recursive_flatten_sum() -> int:
    """Test recursive flatten and sum."""
    nested: list[list[int]] = [[1, 2], [3, 4], [5, 6, 7]]
    return recursive_flatten_sum(nested)  # 28


def test_collatz_chain() -> int:
    """Test Collatz sequence generation."""
    chain: list[int] = collatz_chain(6)
    # 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
    return len(chain)  # 9


def test_nested_callbacks() -> int:
    """Test nested callback simulation."""
    values: list[int] = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10]
    # doubled: [2, 6, 10, 14, 18, 4, 8, 12, 16, 20]
    # filtered (>10): [14, 18, 12, 16, 20]
    # sum: 80
    return nested_callback_simulation(values)  # 80
