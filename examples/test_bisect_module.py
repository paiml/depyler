"""
Comprehensive test of Python bisect module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's bisect module
(binary search and sorted list insertion) to Rust equivalents.

Expected Rust mappings:
- bisect.bisect_left() -> binary search finding leftmost position
- bisect.bisect_right() -> binary search finding rightmost position
- bisect.insort_left() -> insert maintaining sort order (left)
- bisect.insort_right() -> insert maintaining sort order (right)

Note: Manual implementations provided for learning.
"""

import bisect
from typing import List


def binary_search_left(arr: List[int], target: int) -> int:
    """Binary search finding leftmost position"""
    left: int = 0
    right: int = len(arr)

    while left < right:
        mid: int = (left + right) // 2

        if arr[mid] < target:
            left = mid + 1
        else:
            right = mid

    return left


def binary_search_right(arr: List[int], target: int) -> int:
    """Binary search finding rightmost position"""
    left: int = 0
    right: int = len(arr)

    while left < right:
        mid: int = (left + right) // 2

        if arr[mid] <= target:
            left = mid + 1
        else:
            right = mid

    return left


def test_bisect_left() -> int:
    """Test finding insertion point (leftmost)"""
    data: List[int] = [1, 3, 3, 3, 5, 7, 9]
    target: int = 3

    # Find leftmost position where target would be inserted
    position: int = binary_search_left(data, target)

    return position


def test_bisect_right() -> int:
    """Test finding insertion point (rightmost)"""
    data: List[int] = [1, 3, 3, 3, 5, 7, 9]
    target: int = 3

    # Find rightmost position where target would be inserted
    position: int = binary_search_right(data, target)

    return position


def test_bisect_not_found_left() -> int:
    """Test bisect_left with value not in list"""
    data: List[int] = [1, 3, 5, 7, 9]
    target: int = 4

    position: int = binary_search_left(data, target)

    return position


def test_bisect_not_found_right() -> int:
    """Test bisect_right with value not in list"""
    data: List[int] = [1, 3, 5, 7, 9]
    target: int = 4

    position: int = binary_search_right(data, target)

    return position


def insort_left(arr: List[int], value: int) -> List[int]:
    """Insert value maintaining sort order (leftmost position)"""
    position: int = binary_search_left(arr, value)

    # Insert at position
    new_arr: List[int] = []
    for i in range(len(arr)):
        if i == position:
            new_arr.append(value)
        new_arr.append(arr[i])

    # Handle case where insert is at end
    if position == len(arr):
        new_arr.append(value)

    return new_arr


def insort_right(arr: List[int], value: int) -> List[int]:
    """Insert value maintaining sort order (rightmost position)"""
    position: int = binary_search_right(arr, value)

    # Insert at position
    new_arr: List[int] = []
    for i in range(len(arr)):
        if i == position:
            new_arr.append(value)
        new_arr.append(arr[i])

    # Handle case where insert is at end
    if position == len(arr):
        new_arr.append(value)

    return new_arr


def test_insort_left() -> List[int]:
    """Test inserting with insort_left"""
    data: List[int] = [1, 3, 5, 7, 9]
    value: int = 4

    result: List[int] = insort_left(data, value)

    return result


def test_insort_right() -> List[int]:
    """Test inserting with insort_right"""
    data: List[int] = [1, 3, 5, 7, 9]
    value: int = 4

    result: List[int] = insort_right(data, value)

    return result


def test_insort_duplicate_left() -> List[int]:
    """Test inserting duplicate with insort_left"""
    data: List[int] = [1, 3, 3, 3, 5]
    value: int = 3

    result: List[int] = insort_left(data, value)

    return result


def test_insort_duplicate_right() -> List[int]:
    """Test inserting duplicate with insort_right"""
    data: List[int] = [1, 3, 3, 3, 5]
    value: int = 3

    result: List[int] = insort_right(data, value)

    return result


def binary_search_contains(arr: List[int], target: int) -> bool:
    """Check if value exists in sorted array"""
    position: int = binary_search_left(arr, target)

    if position < len(arr) and arr[position] == target:
        return True
    else:
        return False


def count_occurrences_sorted(arr: List[int], target: int) -> int:
    """Count occurrences of value in sorted array"""
    left: int = binary_search_left(arr, target)
    right: int = binary_search_right(arr, target)

    count: int = right - left

    return count


def find_range(arr: List[int], target: int) -> tuple:
    """Find start and end indices of target in sorted array"""
    start: int = binary_search_left(arr, target)
    end: int = binary_search_right(arr, target)

    # Check if target exists
    if start < len(arr) and arr[start] == target:
        return (start, end - 1)
    else:
        return (-1, -1)


def find_closest_value(arr: List[int], target: int) -> int:
    """Find closest value to target in sorted array"""
    if len(arr) == 0:
        return -1

    position: int = binary_search_left(arr, target)

    # Check boundaries
    if position == 0:
        return arr[0]
    if position == len(arr):
        return arr[len(arr) - 1]

    # Compare with neighbors
    before: int = arr[position - 1]
    after: int = arr[position]

    before_dist: int = abs(target - before)
    after_dist: int = abs(target - after)

    if before_dist <= after_dist:
        return before
    else:
        return after


def merge_sorted_arrays(arr1: List[int], arr2: List[int]) -> List[int]:
    """Merge two sorted arrays"""
    result: List[int] = []
    i: int = 0
    j: int = 0

    while i < len(arr1) and j < len(arr2):
        if arr1[i] <= arr2[j]:
            result.append(arr1[i])
            i = i + 1
        else:
            result.append(arr2[j])
            j = j + 1

    # Add remaining elements
    while i < len(arr1):
        result.append(arr1[i])
        i = i + 1

    while j < len(arr2):
        result.append(arr2[j])
        j = j + 1

    return result


def maintain_sorted_list(operations: List[tuple]) -> List[int]:
    """Maintain sorted list through multiple insertions"""
    sorted_list: List[int] = []

    for op in operations:
        operation_type: str = op[0]
        value: int = op[1]

        if operation_type == "insert":
            sorted_list = insort_right(sorted_list, value)

    return sorted_list


def find_insertion_point(arr: List[int], value: int) -> int:
    """Find where to insert value to maintain order"""
    position: int = binary_search_left(arr, value)
    return position


def test_bisect_edge_cases() -> tuple:
    """Test bisect with edge cases"""
    # Empty list
    empty: List[int] = []
    empty_pos: int = binary_search_left(empty, 5)

    # Single element
    single: List[int] = [5]
    single_before: int = binary_search_left(single, 3)
    single_after: int = binary_search_left(single, 7)
    single_equal: int = binary_search_left(single, 5)

    return (empty_pos, single_before, single_after, single_equal)


def find_floor_ceiling(arr: List[int], target: int) -> tuple:
    """Find floor and ceiling values for target"""
    position: int = binary_search_left(arr, target)

    floor_val: int = -1
    ceiling_val: int = -1

    # Floor is largest value <= target
    if position > 0:
        floor_val = arr[position - 1]

    # Ceiling is smallest value >= target
    if position < len(arr):
        ceiling_val = arr[position]

    return (floor_val, ceiling_val)


def test_all_bisect_features() -> None:
    """Run all bisect module tests"""
    # Basic bisect tests
    sorted_data: List[int] = [1, 3, 3, 3, 5, 7, 9]

    left_pos: int = test_bisect_left()
    right_pos: int = test_bisect_right()

    not_found_left: int = test_bisect_not_found_left()
    not_found_right: int = test_bisect_not_found_right()

    # Insort tests
    insorted_left: List[int] = test_insort_left()
    insorted_right: List[int] = test_insort_right()

    dup_left: List[int] = test_insort_duplicate_left()
    dup_right: List[int] = test_insort_duplicate_right()

    # Search operations
    contains: bool = binary_search_contains([1, 3, 5, 7, 9], 5)
    not_contains: bool = binary_search_contains([1, 3, 5, 7, 9], 4)

    # Count occurrences
    count: int = count_occurrences_sorted([1, 3, 3, 3, 5], 3)

    # Find range
    data_range: tuple = find_range([1, 3, 3, 3, 5], 3)

    # Find closest
    closest: int = find_closest_value([1, 3, 5, 7, 9], 4)

    # Merge sorted
    arr1: List[int] = [1, 3, 5]
    arr2: List[int] = [2, 4, 6]
    merged: List[int] = merge_sorted_arrays(arr1, arr2)

    # Maintain sorted list
    operations: List[tuple] = [("insert", 5), ("insert", 2), ("insert", 8), ("insert", 1)]
    maintained: List[int] = maintain_sorted_list(operations)

    # Find insertion point
    insert_pos: int = find_insertion_point([1, 3, 5, 7], 4)

    # Edge cases
    edge_results: tuple = test_bisect_edge_cases()

    # Floor and ceiling
    floor_ceil: tuple = find_floor_ceiling([1, 3, 5, 7, 9], 4)

    print("All bisect module tests completed successfully")
