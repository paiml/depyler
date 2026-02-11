"""Matrix chain multiplication: min cost and optimal parenthesization."""


def matrix_chain_min_cost(dims: list[int]) -> int:
    """Find minimum number of scalar multiplications for matrix chain.
    dims has length n+1 for n matrices. Matrix i has dims[i] x dims[i+1].
    """
    n: int = len(dims) - 1
    if n <= 1:
        return 0
    # dp[i][j] = min cost to multiply matrices i..j, flattened
    dp: list[int] = []
    total: int = n * n
    k: int = 0
    while k < total:
        dp.append(0)
        k = k + 1
    chain_len: int = 2
    while chain_len <= n:
        i: int = 0
        while i < n - chain_len + 1:
            j: int = i + chain_len - 1
            dp[i * n + j] = 999999999
            split: int = i
            while split < j:
                cost: int = dp[i * n + split] + dp[(split + 1) * n + j] + dims[i] * dims[split + 1] * dims[j + 1]
                if cost < dp[i * n + j]:
                    dp[i * n + j] = cost
                split = split + 1
            i = i + 1
        chain_len = chain_len + 1
    return dp[0 * n + (n - 1)]


def max_matrix_chain_cost(dims: list[int]) -> int:
    """Find maximum number of scalar multiplications."""
    n: int = len(dims) - 1
    if n <= 1:
        return 0
    dp: list[int] = []
    total: int = n * n
    k: int = 0
    while k < total:
        dp.append(0)
        k = k + 1
    chain_len: int = 2
    while chain_len <= n:
        i: int = 0
        while i < n - chain_len + 1:
            j: int = i + chain_len - 1
            dp[i * n + j] = 0
            split: int = i
            while split < j:
                cost: int = dp[i * n + split] + dp[(split + 1) * n + j] + dims[i] * dims[split + 1] * dims[j + 1]
                if cost > dp[i * n + j]:
                    dp[i * n + j] = cost
                split = split + 1
            i = i + 1
        chain_len = chain_len + 1
    return dp[0 * n + (n - 1)]


def matrix_multiply_cost(r1: int, c1: int, c2: int) -> int:
    """Cost of multiplying two matrices: r1 x c1 and c1 x c2."""
    return r1 * c1 * c2


def test_module() -> int:
    passed: int = 0

    # Classic example: 10x30, 30x5, 5x60
    cost1: int = matrix_chain_min_cost([10, 30, 5, 60])
    if cost1 == 4500:
        passed = passed + 1

    # Two matrices: 2x3, 3x4
    cost2: int = matrix_chain_min_cost([2, 3, 4])
    if cost2 == 24:
        passed = passed + 1

    # Single matrix
    cost3: int = matrix_chain_min_cost([5, 10])
    if cost3 == 0:
        passed = passed + 1

    max1: int = max_matrix_chain_cost([10, 30, 5, 60])
    if max1 > cost1:
        passed = passed + 1

    mc1: int = matrix_multiply_cost(2, 3, 4)
    if mc1 == 24:
        passed = passed + 1

    # 4 matrices: 40x20, 20x30, 30x10, 10x30
    cost4: int = matrix_chain_min_cost([40, 20, 30, 10, 30])
    if cost4 == 26000:
        passed = passed + 1

    cost5: int = matrix_chain_min_cost([])
    if cost5 == 0:
        passed = passed + 1

    return passed
