"""Sparse vector operations.

Tests: dot product, addition, magnitude squared, nonzero count.
"""


def sparse_dot_product(keys1: list[int], vals1: list[int], keys2: list[int], vals2: list[int]) -> int:
    """Dot product of two sparse vectors given as (index, value) pairs."""
    result: int = 0
    i: int = 0
    j: int = 0
    n1: int = len(keys1)
    n2: int = len(keys2)
    while i < n1 and j < n2:
        if keys1[i] == keys2[j]:
            result = result + vals1[i] * vals2[j]
            i = i + 1
            j = j + 1
        elif keys1[i] < keys2[j]:
            i = i + 1
        else:
            j = j + 1
    return result


def sparse_add(keys1: list[int], vals1: list[int], keys2: list[int], vals2: list[int]) -> list[int]:
    """Add two sparse vectors, return values at merged indices."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    n1: int = len(keys1)
    n2: int = len(keys2)
    while i < n1 and j < n2:
        if keys1[i] == keys2[j]:
            result.append(vals1[i] + vals2[j])
            i = i + 1
            j = j + 1
        elif keys1[i] < keys2[j]:
            result.append(vals1[i])
            i = i + 1
        else:
            result.append(vals2[j])
            j = j + 1
    while i < n1:
        result.append(vals1[i])
        i = i + 1
    while j < n2:
        result.append(vals2[j])
        j = j + 1
    return result


def sparse_magnitude_sq(vals: list[int]) -> int:
    """Sum of squares of values (magnitude squared)."""
    total: int = 0
    i: int = 0
    while i < len(vals):
        total = total + vals[i] * vals[i]
        i = i + 1
    return total


def sparse_nonzero_count(vals: list[int]) -> int:
    """Count non-zero elements."""
    count: int = 0
    i: int = 0
    while i < len(vals):
        if vals[i] != 0:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test sparse vector operations."""
    ok: int = 0
    k1: list[int] = [0, 2, 5]
    v1: list[int] = [1, 3, 5]
    k2: list[int] = [0, 1, 5]
    v2: list[int] = [2, 4, 6]
    if sparse_dot_product(k1, v1, k2, v2) == 32:
        ok = ok + 1
    added: list[int] = sparse_add(k1, v1, k2, v2)
    if added[0] == 3:
        ok = ok + 1
    if len(added) == 4:
        ok = ok + 1
    if sparse_magnitude_sq([3, 4]) == 25:
        ok = ok + 1
    if sparse_nonzero_count([0, 1, 0, 3, 0]) == 2:
        ok = ok + 1
    return ok
