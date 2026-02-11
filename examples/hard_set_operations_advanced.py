"""Hard set operation patterns for transpiler stress-testing.

Tests: set union, intersection, difference, symmetric difference,
set comprehension-like patterns, subset/superset checks, set-based
deduplication, and set operations implemented via dict-based sets.
"""


def set_union(a: list[int], b: list[int]) -> list[int]:
    """Compute the union of two lists treated as sets, returning sorted unique elements."""
    seen: dict[int, bool] = {}
    for x in a:
        seen[x] = True
    for x in b:
        seen[x] = True
    result: list[int] = []
    for key in seen:
        result.append(key)
    result.sort()
    return result


def set_intersection(a: list[int], b: list[int]) -> list[int]:
    """Compute the intersection of two lists treated as sets."""
    set_a: dict[int, bool] = {}
    for x in a:
        set_a[x] = True
    result: list[int] = []
    added: dict[int, bool] = {}
    for x in b:
        if x in set_a and x not in added:
            result.append(x)
            added[x] = True
    result.sort()
    return result


def set_difference(a: list[int], b: list[int]) -> list[int]:
    """Compute elements in a but not in b (set difference)."""
    set_b: dict[int, bool] = {}
    for x in b:
        set_b[x] = True
    result: list[int] = []
    added: dict[int, bool] = {}
    for x in a:
        if x not in set_b and x not in added:
            result.append(x)
            added[x] = True
    result.sort()
    return result


def set_symmetric_difference(a: list[int], b: list[int]) -> list[int]:
    """Compute the symmetric difference: elements in either but not both."""
    set_a: dict[int, bool] = {}
    set_b: dict[int, bool] = {}
    for x in a:
        set_a[x] = True
    for x in b:
        set_b[x] = True
    result: list[int] = []
    added: dict[int, bool] = {}
    for x in a:
        if x not in set_b and x not in added:
            result.append(x)
            added[x] = True
    for x in b:
        if x not in set_a and x not in added:
            result.append(x)
            added[x] = True
    result.sort()
    return result


def is_subset(a: list[int], b: list[int]) -> bool:
    """Check if all elements of a are contained in b."""
    set_b: dict[int, bool] = {}
    for x in b:
        set_b[x] = True
    for x in a:
        if x not in set_b:
            return False
    return True


def is_superset(a: list[int], b: list[int]) -> bool:
    """Check if a contains all elements of b."""
    return is_subset(b, a)


def are_disjoint(a: list[int], b: list[int]) -> bool:
    """Check if two lists treated as sets have no common elements."""
    set_a: dict[int, bool] = {}
    for x in a:
        set_a[x] = True
    for x in b:
        if x in set_a:
            return False
    return True


def unique_elements(data: list[int]) -> list[int]:
    """Remove duplicates from a list preserving first-occurrence order."""
    seen: dict[int, bool] = {}
    result: list[int] = []
    for x in data:
        if x not in seen:
            seen[x] = True
            result.append(x)
    return result


def count_unique(data: list[int]) -> int:
    """Count the number of unique elements in a list."""
    seen: dict[int, bool] = {}
    for x in data:
        seen[x] = True
    count: int = 0
    for _ in seen:
        count += 1
    return count


def set_power_set_count(n: int) -> int:
    """Compute the size of the power set of an n-element set (2^n)."""
    result: int = 1
    for _ in range(n):
        result *= 2
    return result


def jaccard_similarity_int(a: list[int], b: list[int]) -> float:
    """Compute Jaccard similarity between two lists treated as sets.

    J(A, B) = |A intersect B| / |A union B|. Returns 0.0 if both empty.
    """
    inter: list[int] = set_intersection(a, b)
    uni: list[int] = set_union(a, b)
    if len(uni) == 0:
        return 0.0
    return float(len(inter)) / float(len(uni))


def set_partition_even_odd(data: list[int]) -> list[list[int]]:
    """Partition unique elements into even and odd sets.

    Returns [even_set, odd_set] where each is sorted and deduplicated.
    """
    evens: dict[int, bool] = {}
    odds: dict[int, bool] = {}
    for x in data:
        if x % 2 == 0:
            evens[x] = True
        else:
            odds[x] = True
    even_list: list[int] = []
    for key in evens:
        even_list.append(key)
    even_list.sort()
    odd_list: list[int] = []
    for key in odds:
        odd_list.append(key)
    odd_list.sort()
    return [even_list, odd_list]


def multi_set_intersection(sets: list[list[int]]) -> list[int]:
    """Compute the intersection of multiple lists treated as sets."""
    if len(sets) == 0:
        return []
    result: list[int] = unique_elements(sets[0])
    for i in range(1, len(sets)):
        result = set_intersection(result, sets[i])
    return result


def multi_set_union(sets: list[list[int]]) -> list[int]:
    """Compute the union of multiple lists treated as sets."""
    if len(sets) == 0:
        return []
    result: list[int] = unique_elements(sets[0])
    for i in range(1, len(sets)):
        result = set_union(result, sets[i])
    return result


def string_set_union(a: list[str], b: list[str]) -> list[str]:
    """Compute union of two string lists treated as sets."""
    seen: dict[str, bool] = {}
    for x in a:
        seen[x] = True
    for x in b:
        seen[x] = True
    result: list[str] = []
    for key in seen:
        result.append(key)
    result.sort()
    return result


def string_set_intersection(a: list[str], b: list[str]) -> list[str]:
    """Compute intersection of two string lists treated as sets."""
    set_a: dict[str, bool] = {}
    for x in a:
        set_a[x] = True
    result: list[str] = []
    added: dict[str, bool] = {}
    for x in b:
        if x in set_a and x not in added:
            result.append(x)
            added[x] = True
    result.sort()
    return result


def test_all() -> bool:
    """Comprehensive test exercising all set operation functions."""
    # Test set_union
    u: list[int] = set_union([1, 2, 3], [3, 4, 5])
    assert u == [1, 2, 3, 4, 5]
    assert set_union([], [1, 2]) == [1, 2]
    assert set_union([1, 2], []) == [1, 2]

    # Test set_intersection
    inter: list[int] = set_intersection([1, 2, 3, 4], [3, 4, 5, 6])
    assert inter == [3, 4]
    assert set_intersection([1, 2], [3, 4]) == []

    # Test set_difference
    diff: list[int] = set_difference([1, 2, 3, 4], [3, 4, 5])
    assert diff == [1, 2]
    assert set_difference([1, 2], [1, 2]) == []

    # Test set_symmetric_difference
    sym: list[int] = set_symmetric_difference([1, 2, 3], [2, 3, 4])
    assert sym == [1, 4]
    assert set_symmetric_difference([1, 2], [1, 2]) == []
    assert set_symmetric_difference([1], [2]) == [1, 2]

    # Test is_subset
    assert is_subset([1, 2], [1, 2, 3, 4]) == True
    assert is_subset([1, 5], [1, 2, 3]) == False
    assert is_subset([], [1, 2]) == True

    # Test is_superset
    assert is_superset([1, 2, 3, 4], [1, 2]) == True
    assert is_superset([1, 2], [1, 2, 3]) == False

    # Test are_disjoint
    assert are_disjoint([1, 2], [3, 4]) == True
    assert are_disjoint([1, 2, 3], [3, 4]) == False

    # Test unique_elements
    uniq: list[int] = unique_elements([3, 1, 2, 1, 3, 4, 2])
    assert uniq == [3, 1, 2, 4]

    # Test count_unique
    assert count_unique([1, 2, 2, 3, 3, 3]) == 3
    assert count_unique([]) == 0

    # Test set_power_set_count
    assert set_power_set_count(0) == 1
    assert set_power_set_count(3) == 8
    assert set_power_set_count(5) == 32

    # Test jaccard_similarity_int
    j1: float = jaccard_similarity_int([1, 2, 3], [2, 3, 4])
    # inter={2,3}, union={1,2,3,4}, J=2/4=0.5
    assert j1 > 0.49 and j1 < 0.51
    j2: float = jaccard_similarity_int([1, 2], [3, 4])
    assert j2 < 0.01
    j3: float = jaccard_similarity_int([1, 2], [1, 2])
    assert j3 > 0.99

    # Test set_partition_even_odd
    parts: list[list[int]] = set_partition_even_odd([1, 2, 3, 4, 5, 6, 2, 4])
    assert parts[0] == [2, 4, 6]
    assert parts[1] == [1, 3, 5]

    # Test multi_set_intersection
    multi_inter: list[int] = multi_set_intersection([[1, 2, 3], [2, 3, 4], [3, 4, 5]])
    assert multi_inter == [3]
    assert multi_set_intersection([]) == []

    # Test multi_set_union
    multi_uni: list[int] = multi_set_union([[1, 2], [3, 4], [2, 5]])
    assert multi_uni == [1, 2, 3, 4, 5]

    # Test string_set_union
    su: list[str] = string_set_union(["apple", "banana"], ["banana", "cherry"])
    assert len(su) == 3

    # Test string_set_intersection
    si: list[str] = string_set_intersection(["apple", "banana", "cherry"], ["banana", "date"])
    assert si == ["banana"]

    return True


def main() -> None:
    """Run all tests and report results."""
    result: bool = test_all()
    if result:
        print("All set operation tests passed!")


if __name__ == "__main__":
    main()
