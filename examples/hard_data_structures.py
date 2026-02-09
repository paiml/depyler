"""Pathological data structure patterns for transpiler stress testing.

Tests: deeply nested dicts, dict comprehension-like patterns, lists of tuples
as records, chained set operations, mixed-type containers (list of dicts,
dict of lists), defaultdict-like patterns, sliding windows, graph adjacency
lists, priority queues via sorted lists, sparse matrix representations,
and multi-key indexing patterns.
"""


def build_nested_config() -> dict[str, dict[str, dict[str, int]]]:
    """Build a 3-level nested dict representing hierarchical configuration.

    Tests transpiler's ability to handle deeply nested generic types
    and nested dict literal construction with mutation.
    """
    config: dict[str, dict[str, dict[str, int]]] = {}

    db: dict[str, dict[str, int]] = {}
    db_primary: dict[str, int] = {}
    db_primary["port"] = 5432
    db_primary["max_connections"] = 100
    db_primary["timeout"] = 30
    db["primary"] = db_primary

    db_replica: dict[str, int] = {}
    db_replica["port"] = 5433
    db_replica["max_connections"] = 50
    db_replica["timeout"] = 15
    db["replica"] = db_replica
    config["database"] = db

    cache_cfg: dict[str, dict[str, int]] = {}
    redis_cfg: dict[str, int] = {}
    redis_cfg["port"] = 6379
    redis_cfg["db"] = 0
    redis_cfg["ttl"] = 3600
    cache_cfg["redis"] = redis_cfg
    config["cache"] = cache_cfg

    return config


def deep_dict_access(config: dict[str, dict[str, dict[str, int]]], section: str, subsection: str, key: str) -> int:
    """Access a value 3 levels deep with safe fallbacks at each level.

    Tests chained dict.get() with default values, simulating
    the common Python pattern of safe nested access.
    """
    if section not in config:
        return -1
    level1: dict[str, dict[str, int]] = config[section]
    if subsection not in level1:
        return -1
    level2: dict[str, int] = level1[subsection]
    if key not in level2:
        return -1
    return level2[key]


def invert_dict(d: dict[str, int]) -> dict[int, list[str]]:
    """Invert a dict so values become keys mapping to lists of original keys.

    Tests building dict[int, list[str]] from dict[str, int], requiring
    the transpiler to handle dict-of-lists construction with conditional append.
    """
    inverted: dict[int, list[str]] = {}
    for key in d:
        val: int = d[key]
        if val not in inverted:
            inverted[val] = []
        inverted[val].append(key)
    return inverted


def records_sort_by_field(records: list[list[int]], field_index: int) -> list[list[int]]:
    """Sort a list of fixed-size int records by a specific field.

    Uses insertion sort to avoid needing lambda/key functions.
    Tests list-of-lists with index-based access and in-place-like sorting.
    """
    n: int = len(records)
    result: list[list[int]] = []
    for r in records:
        copy: list[int] = []
        for v in r:
            copy.append(v)
        result.append(copy)

    for i in range(1, n):
        j: int = i
        while j > 0 and result[j][field_index] < result[j - 1][field_index]:
            temp: list[int] = result[j]
            result[j] = result[j - 1]
            result[j - 1] = temp
            j -= 1
    return result


def set_chain_operations(a: set[int], b: set[int], c: set[int]) -> list[int]:
    """Chain multiple set operations and collect results.

    Tests: union, intersection, difference, symmetric_difference in sequence.
    Returns sorted list of results for deterministic testing.
    """
    # (a | b) & c - gives elements in c that are also in a or b
    union_ab: set[int] = set()
    for x in a:
        union_ab.add(x)
    for x in b:
        union_ab.add(x)

    intersect_with_c: set[int] = set()
    for x in union_ab:
        if x in c:
            intersect_with_c.add(x)

    # (a & b) - c - gives elements in both a and b but not in c
    intersect_ab: set[int] = set()
    for x in a:
        if x in b:
            intersect_ab.add(x)
    diff_with_c: set[int] = set()
    for x in intersect_ab:
        if x not in c:
            diff_with_c.add(x)

    # Combine both results
    combined: set[int] = set()
    for x in intersect_with_c:
        combined.add(x)
    for x in diff_with_c:
        combined.add(x)

    # Sort for deterministic output
    result: list[int] = []
    for x in combined:
        result.append(x)
    result.sort()
    return result


def group_by_key(pairs: list[list[int]]) -> dict[int, list[int]]:
    """Group values by key from a list of [key, value] pairs.

    Simulates defaultdict(list) pattern using dict.get/setdefault logic.
    Tests the common Python pattern of building grouped data.
    """
    groups: dict[int, list[int]] = {}
    for pair in pairs:
        k: int = pair[0]
        v: int = pair[1]
        if k not in groups:
            groups[k] = []
        groups[k].append(v)
    return groups


def sliding_window_max(nums: list[int], window_size: int) -> list[int]:
    """Find maximum in each sliding window position.

    Brute-force O(n*k) approach that tests nested loop with
    dynamic window bounds and conditional max tracking.
    """
    if not nums or window_size <= 0:
        return []
    n: int = len(nums)
    if window_size > n:
        return []
    result: list[int] = []
    for i in range(n - window_size + 1):
        window_max: int = nums[i]
        for j in range(i + 1, i + window_size):
            if nums[j] > window_max:
                window_max = nums[j]
        result.append(window_max)
    return result


def adjacency_list_bfs(adj: dict[int, list[int]], start: int) -> list[int]:
    """BFS traversal of a graph given as adjacency list.

    Uses a list as a queue (pop from front), set for visited tracking.
    Tests dict[int, list[int]] access, set operations, and queue simulation.
    """
    visited: set[int] = set()
    queue: list[int] = [start]
    order: list[int] = []
    visited.add(start)

    while len(queue) > 0:
        current: int = queue[0]
        queue = queue[1:]
        order.append(current)

        if current in adj:
            neighbors: list[int] = adj[current]
            for neighbor in neighbors:
                if neighbor not in visited:
                    visited.add(neighbor)
                    queue.append(neighbor)
    return order


def sparse_matrix_multiply(
    a_rows: list[int], a_cols: list[int], a_vals: list[int],
    b_rows: list[int], b_cols: list[int], b_vals: list[int],
    size: int
) -> list[int]:
    """Multiply two sparse matrices stored in COO format.

    Inputs are parallel arrays of (row, col, value) triples.
    Returns dense result as flat list (row-major).
    Tests complex multi-array indexing and nested dict accumulation.
    """
    # Build lookup for matrix B: b_lookup[row][col] = val
    b_lookup: dict[int, dict[int, int]] = {}
    for idx in range(len(b_rows)):
        r: int = b_rows[idx]
        c: int = b_cols[idx]
        v: int = b_vals[idx]
        if r not in b_lookup:
            b_lookup[r] = {}
        b_lookup[r][c] = v

    # Accumulate into result dict
    result_map: dict[int, dict[int, int]] = {}
    for idx in range(len(a_rows)):
        ar: int = a_rows[idx]
        ac: int = a_cols[idx]
        av: int = a_vals[idx]
        if ac in b_lookup:
            b_row: dict[int, int] = b_lookup[ac]
            for bc in b_row:
                bv: int = b_row[bc]
                product: int = av * bv
                if ar not in result_map:
                    result_map[ar] = {}
                if bc not in result_map[ar]:
                    result_map[ar][bc] = 0
                result_map[ar][bc] += product

    # Convert to dense flat list
    result: list[int] = []
    for i in range(size):
        for j in range(size):
            val: int = 0
            if i in result_map:
                if j in result_map[i]:
                    val = result_map[i][j]
            result.append(val)
    return result


def frequency_histogram(values: list[int]) -> list[list[int]]:
    """Build a sorted frequency histogram as list of [value, count] pairs.

    First counts frequencies, then sorts by count descending, then by value ascending.
    Tests multi-step data pipeline: count -> convert -> sort.
    """
    counts: dict[int, int] = {}
    for v in values:
        if v in counts:
            counts[v] += 1
        else:
            counts[v] = 1

    # Convert to list of [value, count]
    pairs: list[list[int]] = []
    for key in counts:
        pair: list[int] = [key, counts[key]]
        pairs.append(pair)

    # Sort by count descending (negate for ascending sort), then value ascending
    # Using insertion sort with compound comparison
    for i in range(1, len(pairs)):
        j: int = i
        while j > 0:
            # Compare: higher count first, then lower value first
            should_swap: bool = False
            if pairs[j][1] > pairs[j - 1][1]:
                should_swap = True
            elif pairs[j][1] == pairs[j - 1][1] and pairs[j][0] < pairs[j - 1][0]:
                should_swap = True
            if should_swap:
                temp: list[int] = pairs[j]
                pairs[j] = pairs[j - 1]
                pairs[j - 1] = temp
                j -= 1
            else:
                break
    return pairs


def trie_operations(words: list[str]) -> int:
    """Simulate trie insert and prefix search using nested dicts.

    Uses dict[str, dict] pattern to build a trie-like structure.
    Since we cannot have recursive types, we flatten to string keys.
    Returns count of words that share a common prefix.
    """
    # Build prefix count map
    prefix_counts: dict[str, int] = {}
    for word in words:
        for end in range(1, len(word) + 1):
            prefix: str = word[:end]
            if prefix in prefix_counts:
                prefix_counts[prefix] += 1
            else:
                prefix_counts[prefix] = 1

    # Count prefixes shared by more than one word
    shared: int = 0
    for prefix in prefix_counts:
        if prefix_counts[prefix] > 1:
            shared += 1
    return shared


def lru_cache_simulation(capacity: int, accesses: list[int]) -> list[int]:
    """Simulate an LRU cache using a list for ordering and dict for lookup.

    On hit: move to front. On miss: evict from back if full, add to front.
    Returns the cache state (front to back) after all accesses.
    Tests complex list manipulation with conditional removal and insertion.
    """
    cache_order: list[int] = []
    cache_set: set[int] = set()

    for key in accesses:
        if key in cache_set:
            # Remove from current position
            new_order: list[int] = []
            for item in cache_order:
                if item != key:
                    new_order.append(item)
            cache_order = new_order
            # Add to front
            cache_order = [key] + cache_order
        else:
            # Evict if at capacity
            if len(cache_order) >= capacity:
                evicted: int = cache_order[len(cache_order) - 1]
                cache_order = cache_order[:len(cache_order) - 1]
                cache_set.discard(evicted)
            # Add to front
            cache_order = [key] + cache_order
            cache_set.add(key)

    return cache_order


def test_nested_config() -> int:
    """Test 3-level nested dict construction and access."""
    config: dict[str, dict[str, dict[str, int]]] = build_nested_config()
    port: int = deep_dict_access(config, "database", "primary", "port")
    missing: int = deep_dict_access(config, "database", "primary", "nonexistent")
    missing2: int = deep_dict_access(config, "nonexistent", "x", "y")
    ttl: int = deep_dict_access(config, "cache", "redis", "ttl")
    return port + missing + missing2 + ttl  # 5432 + (-1) + (-1) + 3600 = 9030


def test_invert_dict() -> int:
    """Test dict inversion with duplicate values."""
    d: dict[str, int] = {}
    d["alice"] = 90
    d["bob"] = 85
    d["carol"] = 90
    d["dave"] = 85
    d["eve"] = 100
    inverted: dict[int, list[str]] = invert_dict(d)
    # 90 -> ["alice", "carol"], 85 -> ["bob", "dave"], 100 -> ["eve"]
    count: int = 0
    for key in inverted:
        count += len(inverted[key])
    return count  # 5


def test_records_sort() -> int:
    """Test sorting list of records by different fields."""
    records: list[list[int]] = [[3, 100, 1], [1, 300, 2], [2, 200, 3]]
    sorted_by_first: list[list[int]] = records_sort_by_field(records, 0)
    # Should be [[1,300,2], [2,200,3], [3,100,1]]
    return sorted_by_first[0][0] + sorted_by_first[1][0] + sorted_by_first[2][0]  # 1+2+3=6


def test_set_chains() -> int:
    """Test chained set operations."""
    a: set[int] = {1, 2, 3, 4, 5}
    b: set[int] = {3, 4, 5, 6, 7}
    c: set[int] = {5, 6, 7, 8, 9}
    result: list[int] = set_chain_operations(a, b, c)
    # union(a,b) & c = {5,6,7}; intersect(a,b) - c = {3,4}
    # combined sorted = [3,4,5,6,7]
    total: int = 0
    for v in result:
        total += v
    return total  # 3+4+5+6+7 = 25


def test_group_by() -> int:
    """Test grouping pairs by key."""
    pairs: list[list[int]] = [[1, 10], [2, 20], [1, 30], [2, 40], [3, 50], [1, 60]]
    groups: dict[int, list[int]] = group_by_key(pairs)
    # 1 -> [10,30,60], 2 -> [20,40], 3 -> [50]
    total: int = 0
    for key in groups:
        total += len(groups[key])
    return total  # 6


def test_sliding_window() -> int:
    """Test sliding window maximum."""
    nums: list[int] = [1, 3, -1, -3, 5, 3, 6, 7]
    result: list[int] = sliding_window_max(nums, 3)
    # windows: [1,3,-1]->3, [3,-1,-3]->3, [-1,-3,5]->5, [-3,5,3]->5, [5,3,6]->6, [3,6,7]->7
    total: int = 0
    for v in result:
        total += v
    return total  # 3+3+5+5+6+7 = 29


def test_bfs() -> int:
    """Test BFS traversal on a graph."""
    adj: dict[int, list[int]] = {
        0: [1, 2],
        1: [3],
        2: [4],
        3: [5],
        4: [5],
        5: [],
    }
    order: list[int] = adjacency_list_bfs(adj, 0)
    return len(order)  # 6 nodes visited


def test_sparse_matrix() -> int:
    """Test sparse matrix multiplication.

    A = [[1, 0], [0, 2]]  (entries: (0,0,1), (1,1,2))
    B = [[3, 0], [0, 4]]  (entries: (0,0,3), (1,1,4))
    A*B = [[3, 0], [0, 8]]
    """
    a_rows: list[int] = [0, 1]
    a_cols: list[int] = [0, 1]
    a_vals: list[int] = [1, 2]
    b_rows: list[int] = [0, 1]
    b_cols: list[int] = [0, 1]
    b_vals: list[int] = [3, 4]
    result: list[int] = sparse_matrix_multiply(a_rows, a_cols, a_vals, b_rows, b_cols, b_vals, 2)
    # result = [3, 0, 0, 8]
    return result[0] + result[3]  # 3 + 8 = 11


def test_frequency_histogram() -> int:
    """Test frequency histogram building and sorting."""
    values: list[int] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
    hist: list[list[int]] = frequency_histogram(values)
    # 5 appears 3x, 1 appears 2x, 3 appears 2x, then 2,4,6,9 appear 1x each
    # First entry should be [5, 3]
    return hist[0][0] * 10 + hist[0][1]  # 5*10 + 3 = 53


def test_trie_operations() -> int:
    """Test trie-based prefix counting."""
    words: list[str] = ["apple", "app", "application", "banana", "band"]
    return trie_operations(words)  # shared prefixes count


def test_lru_cache() -> int:
    """Test LRU cache simulation."""
    accesses: list[int] = [1, 2, 3, 4, 1, 2, 5, 1, 2, 3]
    cache: list[int] = lru_cache_simulation(3, accesses)
    # After all accesses, cache front-to-back should be [3, 2, 1]
    return len(cache)  # 3
