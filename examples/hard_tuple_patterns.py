"""Hard tuple patterns that stress-test tuple transpilation.

Tests: tuple creation, mixed-type tuples, unpacking, dictionary keys,
lexicographic comparison, named tuple-like patterns, nested tuples,
slicing, function arguments, sorting, set elements, min/max.
"""

from typing import Tuple, List, Dict, Set


def create_triple(x: int, y: int, z: int) -> Tuple[int, int, int]:
    """Create and return a 3-element integer tuple."""
    return (x, y, z)


def access_triple(t: Tuple[int, int, int]) -> int:
    """Sum all elements of a 3-element tuple via indexing."""
    return t[0] + t[1] + t[2]


def mixed_type_tuple(name: str, age: int, active: bool) -> Tuple[str, int, bool]:
    """Create a mixed-type tuple of str, int, bool."""
    return (name, age, active)


def extract_mixed(record: Tuple[str, int, bool]) -> str:
    """Extract and format fields from a mixed tuple."""
    name: str = record[0]
    age: int = record[1]
    active: bool = record[2]
    status: str = "active" if active else "inactive"
    return name + ":" + str(age) + ":" + status


def unpack_assignment(pair: Tuple[int, int]) -> int:
    """Unpack a tuple in assignment and compute product."""
    a, b = pair
    return a * b


def unpack_triple(t: Tuple[int, int, int]) -> int:
    """Unpack a 3-tuple and return weighted sum."""
    x, y, z = t
    return x * 1 + y * 10 + z * 100


def enumerate_sum(items: List[int]) -> int:
    """Use enumerate to sum index*value products."""
    total: int = 0
    for i, val in enumerate(items):
        total += i * val
    return total


def zip_dot_product(xs: List[int], ys: List[int]) -> int:
    """Compute dot product using zip to unpack pairs."""
    total: int = 0
    for x, y in zip(xs, ys):
        total += x * y
    return total


def tuple_dict_keys(keys: List[Tuple[int, int]], values: List[str]) -> Dict[Tuple[int, int], str]:
    """Build a dictionary using tuples as keys."""
    result: Dict[Tuple[int, int], str] = {}
    for i in range(len(keys)):
        result[keys[i]] = values[i]
    return result


def lookup_tuple_key(grid: Dict[Tuple[int, int], str], row: int, col: int) -> str:
    """Look up a value by tuple key in a dictionary."""
    key: Tuple[int, int] = (row, col)
    if key in grid:
        return grid[key]
    return "not found"


def compare_tuples(a: Tuple[int, int], b: Tuple[int, int]) -> int:
    """Lexicographic tuple comparison returning -1, 0, or 1."""
    if a < b:
        return -1
    elif a > b:
        return 1
    else:
        return 0


def make_point(x: int, y: int) -> Tuple[int, int]:
    """Named-tuple-like pattern: create a 2D point as a tuple."""
    return (x, y)


def manhattan_distance(p1: Tuple[int, int], p2: Tuple[int, int]) -> int:
    """Compute Manhattan distance between two point-tuples."""
    dx: int = p1[0] - p2[0]
    dy: int = p1[1] - p2[1]
    if dx < 0:
        dx = -dx
    if dy < 0:
        dy = -dy
    return dx + dy


def divmod_result(a: int, b: int) -> Tuple[int, int]:
    """Return quotient and remainder as a tuple."""
    quotient: int = a // b
    remainder: int = a % b
    return (quotient, remainder)


def min_max_list(nums: List[int]) -> Tuple[int, int]:
    """Return the min and max of a list as a tuple."""
    lo: int = nums[0]
    hi: int = nums[0]
    for n in nums:
        if n < lo:
            lo = n
        if n > hi:
            hi = n
    return (lo, hi)


def tuple_with_list(data: List[int], flag: bool) -> Tuple[List[int], bool]:
    """Create a tuple containing a list and a bool."""
    filtered: List[int] = []
    for x in data:
        if x > 0:
            filtered.append(x)
    return (filtered, flag)


def count_positive_from_compound(compound: Tuple[List[int], bool]) -> int:
    """Extract list from compound tuple and count elements."""
    items: List[int] = compound[0]
    return len(items)


def nested_tuple_access(outer: Tuple[Tuple[int, int], Tuple[int, int]]) -> int:
    """Access elements from a tuple of tuples and sum all."""
    return outer[0][0] + outer[0][1] + outer[1][0] + outer[1][1]


def build_nested_tuple(a: int, b: int, c: int, d: int) -> Tuple[Tuple[int, int], Tuple[int, int]]:
    """Build a nested tuple (pair of pairs)."""
    return ((a, b), (c, d))


def tuple_slice_first_two(t: Tuple[int, int, int, int]) -> Tuple[int, int]:
    """Slice first two elements from a 4-tuple."""
    return (t[0], t[1])


def tuple_slice_last_two(t: Tuple[int, int, int, int]) -> Tuple[int, int]:
    """Slice last two elements from a 4-tuple."""
    return (t[2], t[3])


def sort_tuples_by_second(pairs: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
    """Sort a list of tuples by their second element (selection sort)."""
    result: List[Tuple[int, int]] = list(pairs)
    n: int = len(result)
    for i in range(n):
        min_idx: int = i
        for j in range(i + 1, n):
            if result[j][1] < result[min_idx][1]:
                min_idx = j
        if min_idx != i:
            temp: Tuple[int, int] = result[i]
            result[i] = result[min_idx]
            result[min_idx] = temp
    return result


def tuple_set_membership(elements: List[Tuple[int, int]]) -> int:
    """Add tuples to a set and return the count of unique tuples."""
    seen: Set[Tuple[int, int]] = set()
    for t in elements:
        seen.add(t)
    return len(seen)


def min_tuple(a: Tuple[int, int], b: Tuple[int, int]) -> Tuple[int, int]:
    """Return the lexicographically smaller tuple."""
    if a <= b:
        return a
    return b


def max_tuple(a: Tuple[int, int], b: Tuple[int, int]) -> Tuple[int, int]:
    """Return the lexicographically larger tuple."""
    if a >= b:
        return a
    return b


def find_min_max_tuples(pairs: List[Tuple[int, int]]) -> Tuple[Tuple[int, int], Tuple[int, int]]:
    """Find the lexicographic min and max from a list of tuples."""
    lo: Tuple[int, int] = pairs[0]
    hi: Tuple[int, int] = pairs[0]
    for p in pairs:
        if p < lo:
            lo = p
        if p > hi:
            hi = p
    return (lo, hi)


def test_all() -> bool:
    """Comprehensive test exercising all tuple patterns."""
    # 1. Tuple creation and access
    triple: Tuple[int, int, int] = create_triple(1, 2, 3)
    assert access_triple(triple) == 6

    # 2. Mixed-type tuples
    record: Tuple[str, int, bool] = mixed_type_tuple("Alice", 30, True)
    assert extract_mixed(record) == "Alice:30:active"
    record2: Tuple[str, int, bool] = mixed_type_tuple("Bob", 25, False)
    assert extract_mixed(record2) == "Bob:25:inactive"

    # 3. Tuple unpacking in assignments
    assert unpack_assignment((3, 7)) == 21
    assert unpack_triple((5, 3, 2)) == 235

    # 4. Tuple unpacking in for loops (enumerate, zip)
    assert enumerate_sum([10, 20, 30]) == 0 * 10 + 1 * 20 + 2 * 30  # 80
    assert zip_dot_product([1, 2, 3], [4, 5, 6]) == 32

    # 5. Tuple as dictionary keys
    keys: List[Tuple[int, int]] = [(0, 0), (0, 1), (1, 0)]
    vals: List[str] = ["origin", "right", "down"]
    grid: Dict[Tuple[int, int], str] = tuple_dict_keys(keys, vals)
    assert lookup_tuple_key(grid, 0, 0) == "origin"
    assert lookup_tuple_key(grid, 0, 1) == "right"
    assert lookup_tuple_key(grid, 5, 5) == "not found"

    # 6. Tuple comparison (lexicographic)
    assert compare_tuples((1, 2), (1, 3)) == -1
    assert compare_tuples((2, 0), (1, 9)) == 1
    assert compare_tuples((3, 3), (3, 3)) == 0

    # 7. Named tuple-like patterns (point)
    p1: Tuple[int, int] = make_point(0, 0)
    p2: Tuple[int, int] = make_point(3, 4)
    assert manhattan_distance(p1, p2) == 7

    # 8. Return multiple values as tuple
    q, r = divmod_result(17, 5)
    assert q == 3
    assert r == 2
    lo, hi = min_max_list([3, 1, 4, 1, 5, 9, 2, 6])
    assert lo == 1
    assert hi == 9

    # 9. Tuple with compound types (List, bool)
    compound: Tuple[List[int], bool] = tuple_with_list([-1, 2, -3, 4, 5], True)
    assert count_positive_from_compound(compound) == 3
    assert compound[1] == True

    # 10. Nested tuple access
    nested: Tuple[Tuple[int, int], Tuple[int, int]] = build_nested_tuple(1, 2, 3, 4)
    assert nested_tuple_access(nested) == 10

    # 11. Tuple slicing patterns
    four: Tuple[int, int, int, int] = (10, 20, 30, 40)
    first_two: Tuple[int, int] = tuple_slice_first_two(four)
    last_two: Tuple[int, int] = tuple_slice_last_two(four)
    assert first_two == (10, 20)
    assert last_two == (30, 40)

    # 12. Tuple in function arguments (already tested above, add explicit check)
    assert access_triple((100, 200, 300)) == 600

    # 13. Tuple sorting by element
    unsorted: List[Tuple[int, int]] = [(3, 1), (1, 3), (2, 2)]
    sorted_pairs: List[Tuple[int, int]] = sort_tuples_by_second(unsorted)
    assert sorted_pairs[0] == (3, 1)
    assert sorted_pairs[1] == (2, 2)
    assert sorted_pairs[2] == (1, 3)

    # 14. Tuple as set elements
    with_dups: List[Tuple[int, int]] = [(1, 2), (3, 4), (1, 2), (5, 6), (3, 4)]
    assert tuple_set_membership(with_dups) == 3

    # 15. Min/max with tuple comparisons
    assert min_tuple((1, 5), (1, 3)) == (1, 3)
    assert max_tuple((1, 5), (1, 3)) == (1, 5)
    assert min_tuple((2, 0), (1, 9)) == (1, 9)
    assert max_tuple((2, 0), (1, 9)) == (2, 0)

    lo_t, hi_t = find_min_max_tuples([(3, 1), (1, 5), (2, 2), (1, 3)])
    assert lo_t == (1, 3)
    assert hi_t == (3, 1)

    return True


if __name__ == "__main__":
    result: bool = test_all()
    if result:
        print("All tuple pattern tests passed!")
