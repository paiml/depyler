"""Product of array except self.

Tests: product except self, prefix products, suffix products.
"""


def product_except_self(arr: list[int]) -> list[int]:
    """Compute product of all elements except self at each position."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(1)
        i = i + 1
    left_prod: int = 1
    j: int = 0
    while j < n:
        result[j] = left_prod
        left_prod = left_prod * arr[j]
        j = j + 1
    right_prod: int = 1
    k: int = n - 1
    while k >= 0:
        result[k] = result[k] * right_prod
        right_prod = right_prod * arr[k]
        k = k - 1
    return result


def prefix_products(arr: list[int]) -> list[int]:
    """Compute prefix product array."""
    result: list[int] = []
    prod: int = 1
    i: int = 0
    while i < len(arr):
        prod = prod * arr[i]
        result.append(prod)
        i = i + 1
    return result


def suffix_products(arr: list[int]) -> list[int]:
    """Compute suffix product array."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(0)
        i = i + 1
    prod: int = 1
    j: int = n - 1
    while j >= 0:
        prod = prod * arr[j]
        result[j] = prod
        j = j - 1
    return result


def test_module() -> int:
    """Test product except self operations."""
    ok: int = 0
    arr: list[int] = [1, 2, 3, 4]
    res: list[int] = product_except_self(arr)
    if res[0] == 24:
        ok = ok + 1
    if res[1] == 12:
        ok = ok + 1
    if res[2] == 8:
        ok = ok + 1
    if res[3] == 6:
        ok = ok + 1
    pp: list[int] = prefix_products([1, 2, 3])
    if pp[2] == 6:
        ok = ok + 1
    sp: list[int] = suffix_products([1, 2, 3])
    if sp[0] == 6:
        ok = ok + 1
    return ok
