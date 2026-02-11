"""
Example demonstrating patterns that trigger performance warnings.
Rewritten to use supported transpilation patterns only.
"""

from typing import List


def string_concat_in_loop(items: List[str]) -> str:
    """String concatenation in loop."""
    result: str = ""
    for item in items:
        result = result + item
    return result


def nested_loop_sum(matrix: List[int], rows: int, cols: int) -> int:
    """Sum elements of a flattened matrix (rows x cols)."""
    total: int = 0
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            idx: int = r * cols + c
            if idx < len(matrix):
                total = total + matrix[idx]
            c = c + 1
        r = r + 1
    return total


def repeated_sum(data: List[int]) -> int:
    """Sum computed repeatedly (simulates expensive recomputation)."""
    result: int = 0
    for item in data:
        data_sum: int = 0
        for x in data:
            data_sum = data_sum + x
        result = result + item * data_sum
    return result


def large_sum_in_loop(n: int) -> int:
    """Creating sums in loops."""
    results: int = 0
    i: int = 0
    while i < n:
        temp_sum: int = 0
        j: int = 0
        while j < 100:
            temp_sum = temp_sum + j
            j = j + 1
        results = results + temp_sum
        i = i + 1
    return results


def linear_search_count(items: List[int], targets: List[int]) -> int:
    """Count how many targets are found in items."""
    found: int = 0
    for target in targets:
        i: int = 0
        while i < len(items):
            if items[i] == target:
                found = found + 1
                i = len(items)  # break
            else:
                i = i + 1
    return found


def power_sum(values: List[int]) -> int:
    """Sum of cubed values."""
    result: int = 0
    for x in values:
        cube: int = x * x * x
        result = result + cube
    return result


def process_item_stub(idx: int, item: int) -> int:
    """Stub for item processing."""
    return idx + item


def range_len_sum(items: List[int]) -> int:
    """Using index-based access pattern."""
    total: int = 0
    i: int = 0
    while i < len(items):
        total = total + process_item_stub(i, items[i])
        i = i + 1
    return total


def aggregate_row_sum(matrix: List[int], rows: int, cols: int) -> int:
    """Computing row sums in nested loop."""
    result: int = 0
    r: int = 0
    while r < rows:
        row_sum: int = 0
        c: int = 0
        while c < cols:
            idx: int = r * cols + c
            if idx < len(matrix):
                row_sum = row_sum + matrix[idx]
            c = c + 1
        c = 0
        while c < cols:
            idx2: int = r * cols + c
            if idx2 < len(matrix):
                result = result + matrix[idx2] * row_sum
            c = c + 1
        r = r + 1
    return result


def test_module() -> int:
    """Run all tests."""
    passed: int = 0

    # Test string_concat_in_loop
    words: List[str] = ["hello", " ", "world"]
    if string_concat_in_loop(words) == "hello world":
        passed = passed + 1

    # Test nested_loop_sum (2x3 matrix flattened)
    flat_matrix: List[int] = [1, 2, 3, 4, 5, 6]
    if nested_loop_sum(flat_matrix, 2, 3) == 21:
        passed = passed + 1

    # Test repeated_sum
    small: List[int] = [1, 2, 3]
    rep: int = repeated_sum(small)
    if rep == 36:
        passed = passed + 1

    # Test large_sum_in_loop
    ls: int = large_sum_in_loop(2)
    if ls == 9900:
        passed = passed + 1

    # Test linear_search_count
    items: List[int] = [10, 20, 30, 40, 50]
    targets: List[int] = [20, 40, 60]
    if linear_search_count(items, targets) == 2:
        passed = passed + 1

    # Test power_sum
    if power_sum([1, 2, 3]) == 36:
        passed = passed + 1

    # Test range_len_sum
    if range_len_sum([10, 20, 30]) == 63:
        passed = passed + 1

    return passed
