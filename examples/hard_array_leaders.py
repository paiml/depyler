"""Leaders in array and equilibrium point detection."""


def find_leaders(arr: list[int]) -> list[int]:
    """Find all leaders: elements greater than all to their right."""
    n: int = len(arr)
    if n == 0:
        result: list[int] = []
        return result
    leaders: list[int] = []
    last_idx: int = n - 1
    max_from_right: int = arr[last_idx]
    leaders.append(arr[last_idx])
    i: int = n - 2
    while i >= 0:
        if arr[i] > max_from_right:
            leaders.append(arr[i])
            max_from_right = arr[i]
        i = i - 1
    reversed_leaders: list[int] = []
    j: int = len(leaders) - 1
    while j >= 0:
        reversed_leaders.append(leaders[j])
        j = j - 1
    return reversed_leaders


def equilibrium_point(arr: list[int]) -> int:
    """Find index where left sum equals right sum. Returns -1 if none."""
    n: int = len(arr)
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    left_sum: int = 0
    j: int = 0
    while j < n:
        right_sum: int = total - left_sum - arr[j]
        if left_sum == right_sum:
            return j
        left_sum = left_sum + arr[j]
        j = j + 1
    return -1


def count_inversions(arr: list[int]) -> int:
    """Count inversions: pairs (i,j) where i<j and arr[i]>arr[j]. O(n^2)."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if arr[i] > arr[j]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def max_subarray_sum(arr: list[int]) -> int:
    """Kadane's algorithm for maximum subarray sum."""
    n: int = len(arr)
    if n == 0:
        return 0
    best: int = arr[0]
    current: int = arr[0]
    i: int = 1
    while i < n:
        if current + arr[i] > arr[i]:
            current = current + arr[i]
        else:
            current = arr[i]
        if current > best:
            best = current
        i = i + 1
    return best


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [16, 17, 4, 3, 5, 2]
    leaders: list[int] = find_leaders(arr1)
    if leaders[0] == 17 and leaders[1] == 5 and leaders[2] == 2:
        passed = passed + 1

    arr2: list[int] = [1, 3, 5, 2, 2]
    if equilibrium_point(arr2) == 2:
        passed = passed + 1

    arr3: list[int] = [2, 4, 1, 3, 5]
    if count_inversions(arr3) == 3:
        passed = passed + 1

    arr4: list[int] = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
    if max_subarray_sum(arr4) == 6:
        passed = passed + 1

    arr5: list[int] = [1, 2, 3]
    if equilibrium_point(arr5) == -1:
        passed = passed + 1

    arr6: list[int] = [5, 4, 3, 2, 1]
    if count_inversions(arr6) == 10:
        passed = passed + 1

    arr7: list[int] = [7]
    leaders2: list[int] = find_leaders(arr7)
    if len(leaders2) == 1 and leaders2[0] == 7:
        passed = passed + 1

    return passed
