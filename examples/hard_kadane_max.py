"""Kadane's algorithm for maximum subarray sum."""


def kadane_max_sum(arr: list[int]) -> int:
    """Find the maximum subarray sum using Kadane's algorithm."""
    if len(arr) == 0:
        return 0
    max_ending: int = arr[0]
    max_so_far: int = arr[0]
    i: int = 1
    length: int = len(arr)
    while i < length:
        val: int = arr[i]
        candidate: int = max_ending + val
        if val > candidate:
            max_ending = val
        else:
            max_ending = candidate
        if max_ending > max_so_far:
            max_so_far = max_ending
        i = i + 1
    return max_so_far


def kadane_with_indices(arr: list[int]) -> list[int]:
    """Return [max_sum, start_index, end_index] of max subarray."""
    if len(arr) == 0:
        return [0, 0, 0]
    max_ending: int = arr[0]
    max_so_far: int = arr[0]
    start: int = 0
    end: int = 0
    temp_start: int = 0
    i: int = 1
    length: int = len(arr)
    while i < length:
        val: int = arr[i]
        candidate: int = max_ending + val
        if val > candidate:
            max_ending = val
            temp_start = i
        else:
            max_ending = candidate
        if max_ending > max_so_far:
            max_so_far = max_ending
            start = temp_start
            end = i
        i = i + 1
    result: list[int] = [max_so_far, start, end]
    return result


def max_circular_subarray(arr: list[int]) -> int:
    """Find max subarray sum in a circular array."""
    if len(arr) == 0:
        return 0
    normal_max: int = kadane_max_sum(arr)
    total: int = 0
    negated: list[int] = []
    i: int = 0
    length: int = len(arr)
    while i < length:
        total = total + arr[i]
        negated.append(-arr[i])
        i = i + 1
    neg_max: int = kadane_max_sum(negated)
    circular_max: int = total + neg_max
    if circular_max == 0:
        return normal_max
    if normal_max > circular_max:
        return normal_max
    return circular_max


def test_module() -> int:
    """Test Kadane's algorithm variants."""
    passed: int = 0

    r1: int = kadane_max_sum([1, -2, 3, 4, -1])
    if r1 == 7:
        passed = passed + 1

    r2: int = kadane_max_sum([-1, -2, -3])
    if r2 == -1:
        passed = passed + 1

    r3: int = kadane_max_sum([5])
    if r3 == 5:
        passed = passed + 1

    info: list[int] = kadane_with_indices([1, -2, 3, 4, -1])
    if info[0] == 7:
        passed = passed + 1

    if info[1] == 2 and info[2] == 3:
        passed = passed + 1

    r6: int = max_circular_subarray([5, -3, 5])
    if r6 == 10:
        passed = passed + 1

    r7: int = kadane_max_sum([])
    if r7 == 0:
        passed = passed + 1

    r8: int = kadane_max_sum([2, 3, -1, 4])
    if r8 == 8:
        passed = passed + 1

    return passed
