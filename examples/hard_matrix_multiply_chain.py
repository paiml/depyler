"""Matrix chain multiplication DP to find optimal parenthesization cost."""


def matrix_chain_order(dims: list[int]) -> int:
    """Find minimum number of scalar multiplications for matrix chain.

    dims has length n+1 where matrix i has dimensions dims[i] x dims[i+1].
    """
    n: int = len(dims) - 1
    if n <= 1:
        return 0
    dp: list[int] = []
    i: int = 0
    while i < n * n:
        dp.append(0)
        i = i + 1
    chain_len: int = 2
    while chain_len <= n:
        i = 0
        while i < n - chain_len + 1:
            j: int = i + chain_len - 1
            dp[i * n + j] = 999999999
            k: int = i
            while k < j:
                cost: int = dp[i * n + k] + dp[(k + 1) * n + j] + dims[i] * dims[k + 1] * dims[j + 1]
                if cost < dp[i * n + j]:
                    dp[i * n + j] = cost
                k = k + 1
            i = i + 1
        chain_len = chain_len + 1
    return dp[0 * n + n - 1]


def matrix_multiply_flat(a: list[int], ar: int, ac: int, b: list[int], bc: int) -> list[int]:
    """Multiply two matrices stored as flat lists. a is ar x ac, b is ac x bc."""
    result: list[int] = []
    i: int = 0
    while i < ar * bc:
        result.append(0)
        i = i + 1
    i = 0
    while i < ar:
        j: int = 0
        while j < bc:
            total: int = 0
            k: int = 0
            while k < ac:
                total = total + a[i * ac + k] * b[k * bc + j]
                k = k + 1
            result[i * bc + j] = total
            j = j + 1
        i = i + 1
    return result


def test_module() -> int:
    """Test matrix chain multiplication."""
    passed: int = 0

    dims1: list[int] = [10, 30, 5, 60]
    if matrix_chain_order(dims1) == 4500:
        passed = passed + 1

    dims2: list[int] = [40, 20, 30, 10, 30]
    if matrix_chain_order(dims2) == 26000:
        passed = passed + 1

    dims3: list[int] = [10, 20]
    if matrix_chain_order(dims3) == 0:
        passed = passed + 1

    a: list[int] = [1, 2, 3, 4]
    b: list[int] = [5, 6, 7, 8]
    c: list[int] = matrix_multiply_flat(a, 2, 2, b, 2)
    if c[0] == 19 and c[1] == 22:
        passed = passed + 1

    if c[2] == 43 and c[3] == 50:
        passed = passed + 1

    dims4: list[int] = [5, 10]
    if matrix_chain_order(dims4) == 0:
        passed = passed + 1

    return passed
