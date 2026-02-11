"""Simple query planner with cost estimation.

Plans selection, projection, and join operations with cost-based optimization.
Tables represented as flat arrays with known cardinalities.
"""


def estimate_select_cost(table_rows: int, selectivity_pct: int) -> int:
    """Estimate cost of selection: table_rows * selectivity / 100."""
    return table_rows * selectivity_pct // 100


def estimate_join_cost_nested(rows_a: int, rows_b: int) -> int:
    """Nested loop join cost: O(rows_a * rows_b)."""
    return rows_a * rows_b


def estimate_join_cost_hash(rows_a: int, rows_b: int) -> int:
    """Hash join cost: O(rows_a + rows_b) * constant factor 3."""
    return (rows_a + rows_b) * 3


def estimate_join_cost_merge(rows_a: int, rows_b: int) -> int:
    """Sort-merge join cost: sort both + merge."""
    cost_a: int = sort_cost(rows_a)
    cost_b: int = sort_cost(rows_b)
    return cost_a + cost_b + rows_a + rows_b


def sort_cost(n: int) -> int:
    """Approximate sort cost: n * log2(n)."""
    if n <= 1:
        return 0
    log2: int = 0
    temp: int = n
    while temp > 1:
        temp = temp // 2
        log2 = log2 + 1
    return n * log2


def best_join_strategy(rows_a: int, rows_b: int) -> int:
    """Choose best join: 0=nested, 1=hash, 2=merge. Returns strategy code."""
    c_nested: int = estimate_join_cost_nested(rows_a, rows_b)
    c_hash: int = estimate_join_cost_hash(rows_a, rows_b)
    c_merge: int = estimate_join_cost_merge(rows_a, rows_b)
    best: int = 0
    best_cost: int = c_nested
    if c_hash < best_cost:
        best = 1
        best_cost = c_hash
    if c_merge < best_cost:
        best = 2
    return best


def project_cost(rows: int, num_cols: int, selected_cols: int) -> int:
    """Cost of projection: proportional to rows * selected_cols."""
    return rows * selected_cols


def plan_total_cost(table_a_rows: int, table_b_rows: int, select_pct_a: int, select_pct_b: int) -> int:
    """Total cost of plan: select on both, then join filtered results."""
    filtered_a: int = estimate_select_cost(table_a_rows, select_pct_a)
    filtered_b: int = estimate_select_cost(table_b_rows, select_pct_b)
    if filtered_a < 1:
        filtered_a = 1
    if filtered_b < 1:
        filtered_b = 1
    strategy: int = best_join_strategy(filtered_a, filtered_b)
    join_cost: int = 0
    if strategy == 0:
        join_cost = estimate_join_cost_nested(filtered_a, filtered_b)
    if strategy == 1:
        join_cost = estimate_join_cost_hash(filtered_a, filtered_b)
    if strategy == 2:
        join_cost = estimate_join_cost_merge(filtered_a, filtered_b)
    return filtered_a + filtered_b + join_cost


def compare_plans(costs: list[int]) -> int:
    """Find index of plan with minimum cost."""
    if len(costs) == 0:
        return 0 - 1
    best_idx: int = 0
    best_c: int = costs[0]
    i: int = 1
    while i < len(costs):
        cv: int = costs[i]
        if cv < best_c:
            best_c = cv
            best_idx = i
        i = i + 1
    return best_idx


def test_module() -> int:
    """Test query planner."""
    ok: int = 0
    sel: int = estimate_select_cost(1000, 10)
    if sel == 100:
        ok = ok + 1
    nj: int = estimate_join_cost_nested(100, 200)
    if nj == 20000:
        ok = ok + 1
    hj: int = estimate_join_cost_hash(100, 200)
    if hj == 900:
        ok = ok + 1
    best: int = best_join_strategy(100, 200)
    if best == 1:
        ok = ok + 1
    bp: int = compare_plans([500, 200, 300])
    if bp == 1:
        ok = ok + 1
    return ok
