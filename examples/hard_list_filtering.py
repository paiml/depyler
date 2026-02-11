# List operations with typed functions for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def filter_positive(nums: list[int]) -> list[int]:
    """Filter only positive numbers from a list."""
    result: list[int] = []
    for n in nums:
        if n > 0:
            result.append(n)
    return result


def map_double(nums: list[int]) -> list[int]:
    """Double each element in a list."""
    result: list[int] = []
    for n in nums:
        result.append(n * 2)
    return result


def reduce_sum(nums: list[int]) -> int:
    """Sum all elements in a list."""
    total: int = 0
    for n in nums:
        total = total + n
    return total


def filter_evens(nums: list[int]) -> list[int]:
    """Filter only even numbers from a list."""
    result: list[int] = []
    for n in nums:
        if n % 2 == 0:
            result.append(n)
    return result


def map_square(nums: list[int]) -> list[int]:
    """Square each element in a list."""
    result: list[int] = []
    for n in nums:
        result.append(n * n)
    return result


def test_module() -> int:
    """Test all list filtering functions."""
    assert filter_positive([-1, 2, -3, 4, 0]) == [2, 4]
    assert filter_positive([1, 2, 3]) == [1, 2, 3]
    assert filter_positive([]) == []
    assert map_double([1, 2, 3]) == [2, 4, 6]
    assert map_double([0, -1]) == [0, -2]
    assert reduce_sum([1, 2, 3, 4, 5]) == 15
    assert reduce_sum([]) == 0
    assert filter_evens([1, 2, 3, 4, 5, 6]) == [2, 4, 6]
    assert map_square([1, 2, 3, 4]) == [1, 4, 9, 16]
    return 0


if __name__ == "__main__":
    test_module()
