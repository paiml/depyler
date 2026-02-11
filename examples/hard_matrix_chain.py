"""Matrix chain multiplication order to minimize scalar multiplications.

Tests: optimal cost, two matrices, three matrices, four matrices, single matrix.
"""


def matrix_chain_order(dims: list[int]) -> int:
    """Return minimum number of scalar multiplications for matrix chain.

    dims has length n+1 for n matrices, where matrix i has dimensions dims[i] x dims[i+1].
    """
    n: int = len(dims) - 1
    if n <= 1:
        return 0
    dp: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    chain_len: int = 2
    while chain_len <= n:
        i = 0
        while i <= n - chain_len:
            j: int = i + chain_len - 1
            dp[i][j] = 999999999
            k: int = i
            while k < j:
                cost: int = dp[i][k] + dp[k + 1][j] + dims[i] * dims[k + 1] * dims[j + 1]
                if cost < dp[i][j]:
                    dp[i][j] = cost
                k = k + 1
            i = i + 1
        chain_len = chain_len + 1
    return dp[0][n - 1]


def matrix_multiply_cost(r1: int, c1: int, c2: int) -> int:
    """Return cost of multiplying two matrices of size r1xc1 and c1xc2."""
    return r1 * c1 * c2


def optimal_parens_count(dims: list[int]) -> int:
    """Return the number of ways to parenthesize n matrices (Catalan number)."""
    n: int = len(dims) - 1
    if n <= 1:
        return 1
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    dp[1] = 1
    i = 2
    while i <= n:
        k: int = 0
        while k < i:
            dp[i] = dp[i] + dp[k] * dp[i - 1 - k]
            k = k + 1
        i = i + 1
    return dp[n - 1]


def test_module() -> int:
    """Test matrix chain multiplication."""
    ok: int = 0

    dims1: list[int] = [10, 30, 5, 60]
    if matrix_chain_order(dims1) == 4500:
        ok = ok + 1

    dims2: list[int] = [40, 20, 30, 10, 30]
    if matrix_chain_order(dims2) == 26000:
        ok = ok + 1

    dims3: list[int] = [10, 20, 30]
    if matrix_chain_order(dims3) == 6000:
        ok = ok + 1

    dims4: list[int] = [10, 20]
    if matrix_chain_order(dims4) == 0:
        ok = ok + 1

    if matrix_multiply_cost(10, 20, 30) == 6000:
        ok = ok + 1

    dims5: list[int] = [1, 2, 3, 4]
    if matrix_chain_order(dims5) == 18:
        ok = ok + 1

    dims6: list[int] = [5, 10, 3, 12, 5, 50, 6]
    if matrix_chain_order(dims6) == 2010:
        ok = ok + 1

    if optimal_parens_count(dims1) == 2:
        ok = ok + 1

    dims_single: list[int] = [5, 10]
    if matrix_chain_order(dims_single) == 0:
        ok = ok + 1

    if optimal_parens_count(dims2) == 5:
        ok = ok + 1

    return ok
