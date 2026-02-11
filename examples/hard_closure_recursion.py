"""Closure and recursion patterns for transpiler stress testing.

Tests: recursive functions with multiple base cases, mutual recursion,
memoized recursion, tree traversals, and trampolined recursion.
All nested functions extracted to top-level to avoid scope issues.
"""


def ackermann(m: int, n: int) -> int:
    """Ackermann function: deeply recursive with multiple base cases."""
    if m == 0:
        return n + 1
    elif m > 0 and n == 0:
        return ackermann(m - 1, 1)
    else:
        return ackermann(m - 1, ackermann(m, n - 1))


def flatten_nested(lst: list[list[int]]) -> list[int]:
    """Flatten a list of lists into a single list."""
    result: list[int] = []
    for sublist in lst:
        for item in sublist:
            result.append(item)
    return result


def partition_helper(remaining: int, max_val: int, cache_keys: list[str], cache_vals: list[int]) -> int:
    """Recursive partition counter with explicit cache arrays."""
    if remaining == 0:
        return 1
    if remaining < 0:
        return 0
    lookup: str = str(remaining) + "," + str(max_val)
    ci: int = 0
    while ci < len(cache_keys):
        if cache_keys[ci] == lookup:
            return cache_vals[ci]
        ci = ci + 1
    total: int = 0
    k: int = 1
    while k <= max_val:
        if k <= remaining:
            total = total + partition_helper(remaining - k, k, cache_keys, cache_vals)
        k = k + 1
    cache_keys.append(lookup)
    cache_vals.append(total)
    return total


def memoized_partition_count(n: int) -> int:
    """Count integer partitions using list-based memoization."""
    cache_keys: list[str] = []
    cache_vals: list[int] = []
    return partition_helper(n, n, cache_keys, cache_vals)


def is_even_mutual(n: int) -> bool:
    """Mutual recursion: is_even calls is_odd."""
    if n == 0:
        return True
    return is_odd_mutual(n - 1)


def is_odd_mutual(n: int) -> bool:
    """Companion to is_even_mutual for mutual recursion test."""
    if n == 0:
        return False
    return is_even_mutual(n - 1)


def make_accumulator(initial: int) -> int:
    """Simulate a closure-based accumulator."""
    total: int = initial
    additions: list[int] = [10, 20, 30, -5, 15]
    for val in additions:
        total += val
    return total


def make_counter_pair() -> list[int]:
    """Simulate paired increment/decrement counters."""
    state: int = 0
    results: list[int] = []
    i: int = 0
    while i < 3:
        state += 1
        i = i + 1
    results.append(state)
    state -= 1
    results.append(state)
    results.append(state)
    return results


def recursive_power(num: int, exp: int) -> int:
    """Fast exponentiation via recursive squaring."""
    if exp == 0:
        return 1
    if exp < 0:
        return 0
    if exp % 2 == 0:
        half: int = recursive_power(num, exp // 2)
        return half * half
    else:
        return num * recursive_power(num, exp - 1)


def dfs_iterative(adj_keys: list[int], adj_vals: list[list[int]], root: int) -> list[int]:
    """Depth-first traversal using iterative stack approach.

    adj_keys and adj_vals represent the adjacency list as parallel arrays.
    """
    order: list[int] = []
    visited: list[int] = []
    stack: list[int] = [root]
    while len(stack) > 0:
        last_idx: int = len(stack) - 1
        node: int = stack[last_idx]
        stack.pop()
        already: int = 0
        vi: int = 0
        while vi < len(visited):
            if visited[vi] == node:
                already = 1
            vi = vi + 1
        if already == 0:
            visited.append(node)
            order.append(node)
            ki: int = 0
            while ki < len(adj_keys):
                if adj_keys[ki] == node:
                    neighbors: list[int] = adj_vals[ki]
                    ni: int = len(neighbors) - 1
                    while ni >= 0:
                        n_node: int = neighbors[ni]
                        stack.append(n_node)
                        ni = ni - 1
                ki = ki + 1
    return order


def hanoi_solve(disks: int, src: int, tgt: int, aux: int, moves: list[int]) -> int:
    """Tower of Hanoi solver recording moves as int triples [src, tgt, ...]."""
    if disks == 1:
        moves.append(src)
        moves.append(tgt)
        return 0
    hanoi_solve(disks - 1, src, aux, tgt, moves)
    moves.append(src)
    moves.append(tgt)
    hanoi_solve(disks - 1, aux, tgt, src, moves)
    return 0


def tower_of_hanoi_count(n: int) -> int:
    """Get the number of moves for Tower of Hanoi with n disks."""
    moves: list[int] = []
    hanoi_solve(n, 1, 3, 2, moves)
    return len(moves) // 2


def recursive_flatten_sum(nested: list[list[int]]) -> int:
    """Sum all elements in a nested list structure."""
    total: int = 0
    for sublist in nested:
        for val in sublist:
            total += val
    return total


def collatz_chain(n: int) -> list[int]:
    """Generate full Collatz sequence from n to 1."""
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
    """Simulate nested callback pattern via sequential transformations."""
    doubled: list[int] = []
    for v in values:
        doubled.append(v * 2)
    filtered: list[int] = []
    for v in doubled:
        if v > 10:
            filtered.append(v)
    total: int = 0
    for v in filtered:
        total += v
    return total


def test_ackermann() -> int:
    """Test Ackermann function with small inputs."""
    r1: int = ackermann(2, 3)
    r2: int = ackermann(1, 5)
    return r1 + r2


def test_flatten_nested() -> int:
    """Test nested list flattening."""
    data: list[list[int]] = [[1, 2, 3], [4, 5], [6]]
    flat: list[int] = flatten_nested(data)
    return len(flat)


def test_memoized_partition() -> int:
    """Test memoized partition counting."""
    return memoized_partition_count(5)


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
    return results


def test_accumulator() -> int:
    """Test closure-based accumulator simulation."""
    return make_accumulator(0)


def test_counter_pair() -> int:
    """Test paired counter closure simulation."""
    result: list[int] = make_counter_pair()
    return result[0] + result[1] + result[2]


def test_recursive_power() -> int:
    """Test recursive fast exponentiation."""
    r1: int = recursive_power(2, 10)
    r2: int = recursive_power(3, 0)
    r3: int = recursive_power(5, 3)
    r4: int = recursive_power(2, -1)
    return r1 + r2 + r3 + r4


def test_tree_dfs() -> int:
    """Test DFS tree traversal."""
    adj_keys: list[int] = [0, 1, 2, 3, 4, 5]
    adj_vals: list[list[int]] = [[1, 2], [3, 4], [5], [], [], []]
    order: list[int] = dfs_iterative(adj_keys, adj_vals, 0)
    return len(order)


def test_tower_of_hanoi() -> int:
    """Test Tower of Hanoi move generation."""
    return tower_of_hanoi_count(3)


def test_recursive_flatten_sum() -> int:
    """Test recursive flatten and sum."""
    nested: list[list[int]] = [[1, 2], [3, 4], [5, 6, 7]]
    return recursive_flatten_sum(nested)


def test_collatz_chain() -> int:
    """Test Collatz sequence generation."""
    chain: list[int] = collatz_chain(6)
    return len(chain)


def test_nested_callbacks() -> int:
    """Test nested callback simulation."""
    values: list[int] = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10]
    return nested_callback_simulation(values)
