"""Pattern recognition in sequences.

Tests: arithmetic sequence detection, geometric detection, plateau finding.
"""


def is_arithmetic_val(arr: list[int]) -> int:
    """Check if array forms an arithmetic sequence. Returns 1 if yes, 0 if no."""
    n: int = len(arr)
    if n <= 2:
        return 1
    diff: int = arr[1] - arr[0]
    i: int = 2
    while i < n:
        if arr[i] - arr[i - 1] != diff:
            return 0
        i = i + 1
    return 1


def find_common_difference(arr: list[int]) -> int:
    """Find common difference of arithmetic sequence (0 if not arithmetic)."""
    if len(arr) < 2:
        return 0
    diff: int = arr[1] - arr[0]
    i: int = 2
    while i < len(arr):
        if arr[i] - arr[i - 1] != diff:
            return 0
        i = i + 1
    return diff


def longest_plateau(arr: list[int]) -> int:
    """Find length of longest plateau (consecutive equal elements)."""
    if len(arr) == 0:
        return 0
    best: int = 1
    current: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] == arr[i - 1]:
            current = current + 1
            if current > best:
                best = current
        else:
            current = 1
        i = i + 1
    return best


def count_peaks(arr: list[int]) -> int:
    """Count local peaks (strictly greater than both neighbors)."""
    if len(arr) < 3:
        return 0
    count: int = 0
    i: int = 1
    while i < len(arr) - 1:
        if arr[i] > arr[i - 1] and arr[i] > arr[i + 1]:
            count = count + 1
        i = i + 1
    return count


def longest_alternating(arr: list[int]) -> int:
    """Find length of longest alternating subsequence."""
    n: int = len(arr)
    if n <= 1:
        return n
    length: int = 1
    i: int = 1
    prev_up: int = -1
    while i < n:
        if arr[i] > arr[i - 1]:
            if prev_up != 1:
                length = length + 1
                prev_up = 1
        elif arr[i] < arr[i - 1]:
            if prev_up != 0:
                length = length + 1
                prev_up = 0
        i = i + 1
    return length


def test_module() -> None:
    assert is_arithmetic_val([2, 4, 6, 8]) == 1
    assert is_arithmetic_val([5, 5, 5]) == 1
    assert is_arithmetic_val([1, 2, 4]) == 0
    assert find_common_difference([3, 6, 9, 12]) == 3
    assert find_common_difference([1, 2, 4]) == 0
    assert longest_plateau([1, 2, 2, 2, 3, 3]) == 3
    assert longest_plateau([1, 2, 3]) == 1
    assert longest_plateau([]) == 0
    assert count_peaks([1, 3, 2, 4, 1]) == 2
    assert count_peaks([1, 2, 3]) == 0
    assert longest_alternating([1, 5, 4, 9, 3]) == 5
    assert longest_alternating([1, 2, 3, 4]) == 2
