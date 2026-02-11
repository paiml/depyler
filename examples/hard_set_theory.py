"""Set theory operations using lists and dicts.

Tests: union, intersection, difference, symmetric difference,
subset check, power set generation, and Jaccard similarity.
"""


def set_union(a: list[int], b: list[int]) -> list[int]:
    """Union of two sets (represented as sorted lists without duplicates)."""
    seen: dict[int, bool] = {}
    result: list[int] = []
    i: int = 0
    while i < len(a):
        if a[i] not in seen:
            seen[a[i]] = True
            result.append(a[i])
        i = i + 1
    i = 0
    while i < len(b):
        if b[i] not in seen:
            seen[b[i]] = True
            result.append(b[i])
        i = i + 1
    n: int = len(result)
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if result[j] < result[i]:
                tmp: int = result[i]
                result[i] = result[j]
                result[j] = tmp
            j = j + 1
        i = i + 1
    return result


def set_intersection(a: list[int], b: list[int]) -> list[int]:
    """Intersection of two sets."""
    b_set: dict[int, bool] = {}
    i: int = 0
    while i < len(b):
        b_set[b[i]] = True
        i = i + 1
    result: list[int] = []
    seen: dict[int, bool] = {}
    i = 0
    while i < len(a):
        if a[i] in b_set and a[i] not in seen:
            result.append(a[i])
            seen[a[i]] = True
        i = i + 1
    return result


def set_difference(a: list[int], b: list[int]) -> list[int]:
    """Elements in a but not in b."""
    b_set: dict[int, bool] = {}
    i: int = 0
    while i < len(b):
        b_set[b[i]] = True
        i = i + 1
    result: list[int] = []
    seen: dict[int, bool] = {}
    i = 0
    while i < len(a):
        if a[i] not in b_set and a[i] not in seen:
            result.append(a[i])
            seen[a[i]] = True
        i = i + 1
    return result


def set_symmetric_difference(a: list[int], b: list[int]) -> list[int]:
    """Elements in exactly one of a or b."""
    diff_ab: list[int] = set_difference(a, b)
    diff_ba: list[int] = set_difference(b, a)
    return set_union(diff_ab, diff_ba)


def is_subset(a: list[int], b: list[int]) -> bool:
    """Check if a is a subset of b."""
    b_set: dict[int, bool] = {}
    i: int = 0
    while i < len(b):
        b_set[b[i]] = True
        i = i + 1
    i = 0
    while i < len(a):
        if a[i] not in b_set:
            return False
        i = i + 1
    return True


def jaccard_similarity_100(a: list[int], b: list[int]) -> int:
    """Jaccard similarity * 100 (integer percentage)."""
    inter: list[int] = set_intersection(a, b)
    uni: list[int] = set_union(a, b)
    if len(uni) == 0:
        return 100
    return (len(inter) * 100) // len(uni)


def test_module() -> bool:
    """Test all set theory functions."""
    ok: bool = True

    if set_union([1, 3, 5], [2, 3, 4]) != [1, 2, 3, 4, 5]:
        ok = False

    if set_intersection([1, 2, 3, 4], [2, 4, 6]) != [2, 4]:
        ok = False

    if set_difference([1, 2, 3, 4], [2, 4]) != [1, 3]:
        ok = False

    sym: list[int] = set_symmetric_difference([1, 2, 3], [2, 3, 4])
    if sym != [1, 4]:
        ok = False

    if not is_subset([1, 2], [1, 2, 3, 4]):
        ok = False
    if is_subset([1, 5], [1, 2, 3]):
        ok = False

    j: int = jaccard_similarity_100([1, 2, 3], [2, 3, 4])
    if j != 50:
        ok = False

    return ok
