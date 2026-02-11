"""Hard list comprehension and functional patterns for transpiler stress-testing.

Tests: list comprehensions with filters, nested comprehensions, map/filter
equivalents using explicit loops, chained transformations, zip-based
operations, enumerate patterns, and reduction operations.
"""


def squares_of_evens(nums: list[int]) -> list[int]:
    """Compute squares of even numbers from a list, using filter+map pattern."""
    result: list[int] = []
    for x in nums:
        if x % 2 == 0:
            result.append(x * x)
    return result


def cubes_of_odds(nums: list[int]) -> list[int]:
    """Compute cubes of odd numbers from a list."""
    result: list[int] = []
    for x in nums:
        if x % 2 != 0:
            result.append(x * x * x)
    return result


def flatten_2d(matrix: list[list[int]]) -> list[int]:
    """Flatten a 2D list into a 1D list (nested comprehension equivalent)."""
    result: list[int] = []
    for row in matrix:
        for val in row:
            result.append(val)
    return result


def transpose_matrix(matrix: list[list[int]]) -> list[list[int]]:
    """Transpose a 2D matrix using nested iteration."""
    if len(matrix) == 0:
        return []
    rows: int = len(matrix)
    cols: int = len(matrix[0])
    result: list[list[int]] = []
    for c in range(cols):
        new_row: list[int] = []
        for r in range(rows):
            new_row.append(matrix[r][c])
        result.append(new_row)
    return result


def cartesian_product(a: list[int], b: list[int]) -> list[list[int]]:
    """Compute the cartesian product of two lists as list of pairs."""
    result: list[list[int]] = []
    for x in a:
        for y in b:
            result.append([x, y])
    return result


def filter_and_transform(nums: list[int], threshold: int) -> list[int]:
    """Filter values above threshold and double them."""
    result: list[int] = []
    for x in nums:
        if x > threshold:
            result.append(x * 2)
    return result


def enumerate_with_index(items: list[str]) -> list[str]:
    """Create indexed string representations of list items."""
    result: list[str] = []
    for i in range(len(items)):
        entry: str = str(i) + ":" + items[i]
        result.append(entry)
    return result


def zip_sum_pairs(a: list[int], b: list[int]) -> list[int]:
    """Sum corresponding elements from two lists of equal length."""
    length: int = len(a)
    if len(b) < length:
        length = len(b)
    result: list[int] = []
    for i in range(length):
        result.append(a[i] + b[i])
    return result


def zip_multiply_pairs(a: list[int], b: list[int]) -> list[int]:
    """Multiply corresponding elements from two lists."""
    length: int = len(a)
    if len(b) < length:
        length = len(b)
    result: list[int] = []
    for i in range(length):
        result.append(a[i] * b[i])
    return result


def chain_lists(lists: list[list[int]]) -> list[int]:
    """Chain multiple lists into one (like itertools.chain)."""
    result: list[int] = []
    for lst in lists:
        for item in lst:
            result.append(item)
    return result


def reduce_sum(nums: list[int]) -> int:
    """Reduce a list by summing all elements."""
    total: int = 0
    for x in nums:
        total += x
    return total


def reduce_product(nums: list[int]) -> int:
    """Reduce a list by multiplying all elements."""
    if len(nums) == 0:
        return 0
    product: int = 1
    for x in nums:
        product *= x
    return product


def reduce_max(nums: list[int]) -> int:
    """Find the maximum element in a list."""
    if len(nums) == 0:
        return 0
    best: int = nums[0]
    for x in nums:
        if x > best:
            best = x
    return best


def reduce_min(nums: list[int]) -> int:
    """Find the minimum element in a list."""
    if len(nums) == 0:
        return 0
    best: int = nums[0]
    for x in nums:
        if x < best:
            best = x
    return best


def prefix_sums(nums: list[int]) -> list[int]:
    """Compute prefix sums (cumulative sum) of a list."""
    result: list[int] = []
    running: int = 0
    for x in nums:
        running += x
        result.append(running)
    return result


def take_while_positive(nums: list[int]) -> list[int]:
    """Take elements from the front while they are positive."""
    result: list[int] = []
    for x in nums:
        if x <= 0:
            break
        result.append(x)
    return result


def drop_while_negative(nums: list[int]) -> list[int]:
    """Drop elements from the front while they are negative."""
    result: list[int] = []
    dropping: bool = True
    for x in nums:
        if dropping and x < 0:
            continue
        dropping = False
        result.append(x)
    return result


def group_consecutive(nums: list[int]) -> list[list[int]]:
    """Group consecutive equal elements together."""
    if len(nums) == 0:
        return []
    groups: list[list[int]] = []
    current_group: list[int] = [nums[0]]
    for i in range(1, len(nums)):
        if nums[i] == nums[i - 1]:
            current_group.append(nums[i])
        else:
            groups.append(current_group)
            current_group = [nums[i]]
    groups.append(current_group)
    return groups


def interleave(a: list[int], b: list[int]) -> list[int]:
    """Interleave two lists element by element."""
    result: list[int] = []
    length: int = len(a)
    if len(b) < length:
        length = len(b)
    for i in range(length):
        result.append(a[i])
        result.append(b[i])
    # Append remaining elements
    for i in range(length, len(a)):
        result.append(a[i])
    for i in range(length, len(b)):
        result.append(b[i])
    return result


def test_all() -> bool:
    """Comprehensive test exercising all list comprehension functions."""
    # Test squares_of_evens
    assert squares_of_evens([1, 2, 3, 4, 5, 6]) == [4, 16, 36]
    assert squares_of_evens([1, 3, 5]) == []
    assert squares_of_evens([]) == []

    # Test cubes_of_odds
    assert cubes_of_odds([1, 2, 3, 4, 5]) == [1, 27, 125]
    assert cubes_of_odds([2, 4, 6]) == []

    # Test flatten_2d
    assert flatten_2d([[1, 2], [3, 4], [5]]) == [1, 2, 3, 4, 5]
    assert flatten_2d([]) == []
    assert flatten_2d([[], [1], []]) == [1]

    # Test transpose_matrix
    t: list[list[int]] = transpose_matrix([[1, 2, 3], [4, 5, 6]])
    assert t == [[1, 4], [2, 5], [3, 6]]
    assert transpose_matrix([]) == []

    # Test cartesian_product
    cp: list[list[int]] = cartesian_product([1, 2], [3, 4])
    assert len(cp) == 4
    assert cp[0] == [1, 3]
    assert cp[1] == [1, 4]
    assert cp[2] == [2, 3]
    assert cp[3] == [2, 4]

    # Test filter_and_transform
    assert filter_and_transform([1, 5, 3, 8, 2, 7], 4) == [10, 16, 14]
    assert filter_and_transform([1, 2, 3], 10) == []

    # Test enumerate_with_index
    indexed: list[str] = enumerate_with_index(["a", "b", "c"])
    assert indexed[0] == "0:a"
    assert indexed[1] == "1:b"
    assert indexed[2] == "2:c"

    # Test zip_sum_pairs
    assert zip_sum_pairs([1, 2, 3], [4, 5, 6]) == [5, 7, 9]
    assert zip_sum_pairs([1, 2], [3]) == [4]

    # Test zip_multiply_pairs
    assert zip_multiply_pairs([2, 3, 4], [5, 6, 7]) == [10, 18, 28]

    # Test chain_lists
    assert chain_lists([[1, 2], [3], [4, 5, 6]]) == [1, 2, 3, 4, 5, 6]
    assert chain_lists([]) == []

    # Test reduce_sum
    assert reduce_sum([1, 2, 3, 4, 5]) == 15
    assert reduce_sum([]) == 0

    # Test reduce_product
    assert reduce_product([1, 2, 3, 4]) == 24
    assert reduce_product([]) == 0
    assert reduce_product([5]) == 5

    # Test reduce_max
    assert reduce_max([3, 1, 4, 1, 5, 9]) == 9
    assert reduce_max([-5, -3, -1]) == -1

    # Test reduce_min
    assert reduce_min([3, 1, 4, 1, 5, 9]) == 1
    assert reduce_min([-5, -3, -1]) == -5

    # Test prefix_sums
    assert prefix_sums([1, 2, 3, 4]) == [1, 3, 6, 10]
    assert prefix_sums([]) == []
    assert prefix_sums([5]) == [5]

    # Test take_while_positive
    assert take_while_positive([3, 5, 7, -1, 4, 6]) == [3, 5, 7]
    assert take_while_positive([-1, 2, 3]) == []
    assert take_while_positive([]) == []

    # Test drop_while_negative
    assert drop_while_negative([-3, -2, -1, 0, 1, 2]) == [0, 1, 2]
    assert drop_while_negative([1, 2, 3]) == [1, 2, 3]
    assert drop_while_negative([]) == []

    # Test group_consecutive
    groups: list[list[int]] = group_consecutive([1, 1, 2, 2, 2, 3, 1, 1])
    assert len(groups) == 4
    assert groups[0] == [1, 1]
    assert groups[1] == [2, 2, 2]
    assert groups[2] == [3]
    assert groups[3] == [1, 1]
    assert group_consecutive([]) == []

    # Test interleave
    assert interleave([1, 3, 5], [2, 4, 6]) == [1, 2, 3, 4, 5, 6]
    assert interleave([1, 2], [3, 4, 5, 6]) == [1, 3, 2, 4, 5, 6]
    assert interleave([1, 2, 3], []) == [1, 2, 3]

    return True


def main() -> None:
    """Run all tests and report results."""
    result: bool = test_all()
    if result:
        print("All list comprehension tests passed!")


if __name__ == "__main__":
    main()
