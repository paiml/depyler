"""Array product operations: product except self, running product, partial products.

Tests: product_except_self, running_product, partial_products.
"""


def product_except_self(arr: list[int]) -> list[int]:
    """For each index, compute product of all other elements."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(1)
        i = i + 1
    left: int = 1
    i = 0
    while i < n:
        result[i] = left
        left = left * arr[i]
        i = i + 1
    right: int = 1
    i = n - 1
    while i >= 0:
        result[i] = result[i] * right
        right = right * arr[i]
        i = i - 1
    return result


def running_product(arr: list[int]) -> list[int]:
    """Compute running (cumulative) product."""
    result: list[int] = []
    prod: int = 1
    i: int = 0
    while i < len(arr):
        prod = prod * arr[i]
        result.append(prod)
        i = i + 1
    return result


def partial_products(arr: list[int], k: int) -> list[int]:
    """Compute product of each window of size k."""
    result: list[int] = []
    if k <= 0 or k > len(arr):
        return result
    i: int = 0
    while i <= len(arr) - k:
        prod: int = 1
        j: int = 0
        while j < k:
            prod = prod * arr[i + j]
            j = j + 1
        result.append(prod)
        i = i + 1
    return result


def array_product(arr: list[int]) -> int:
    """Compute total product of all elements."""
    prod: int = 1
    i: int = 0
    while i < len(arr):
        prod = prod * arr[i]
        i = i + 1
    return prod


def test_module() -> int:
    """Test array product operations."""
    ok: int = 0

    if product_except_self([1, 2, 3, 4]) == [24, 12, 8, 6]:
        ok = ok + 1

    if product_except_self([2, 3]) == [3, 2]:
        ok = ok + 1

    if running_product([1, 2, 3, 4]) == [1, 2, 6, 24]:
        ok = ok + 1

    if running_product([5]) == [5]:
        ok = ok + 1

    if partial_products([1, 2, 3, 4, 5], 2) == [2, 6, 12, 20]:
        ok = ok + 1

    if partial_products([1, 2, 3], 3) == [6]:
        ok = ok + 1

    if array_product([2, 3, 4]) == 24:
        ok = ok + 1

    return ok
