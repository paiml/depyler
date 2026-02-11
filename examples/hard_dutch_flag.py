"""Dutch national flag partitioning (three-way partition)."""


def swap_elements(arr: list[int], i: int, j: int) -> list[int]:
    """Swap two elements in array and return modified array."""
    temp: int = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
    return arr


def dutch_flag_partition(arr: list[int], pivot: int) -> list[int]:
    """Partition array into elements < pivot, == pivot, > pivot."""
    low: int = 0
    mid: int = 0
    last: int = len(arr) - 1
    while mid <= last:
        if arr[mid] < pivot:
            arr = swap_elements(arr, low, mid)
            low = low + 1
            mid = mid + 1
        elif arr[mid] == pivot:
            mid = mid + 1
        else:
            arr = swap_elements(arr, mid, last)
            last = last - 1
    return arr


def count_partitions(arr: list[int], pivot: int) -> list[int]:
    """Count elements less than, equal to, and greater than pivot."""
    less: int = 0
    equal: int = 0
    greater: int = 0
    i: int = 0
    length: int = len(arr)
    while i < length:
        if arr[i] < pivot:
            less = less + 1
        elif arr[i] == pivot:
            equal = equal + 1
        else:
            greater = greater + 1
        i = i + 1
    result: list[int] = [less, equal, greater]
    return result


def is_partitioned(arr: list[int], pivot: int) -> int:
    """Check if array is partitioned around pivot. Returns 1 if yes, 0 if no."""
    phase: int = 0
    i: int = 0
    length: int = len(arr)
    while i < length:
        if phase == 0:
            if arr[i] == pivot:
                phase = 1
            elif arr[i] > pivot:
                phase = 2
        elif phase == 1:
            if arr[i] < pivot:
                return 0
            elif arr[i] > pivot:
                phase = 2
        else:
            if arr[i] <= pivot:
                return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test Dutch flag partitioning."""
    passed: int = 0

    arr1: list[int] = [2, 0, 1, 2, 0, 1, 1]
    result1: list[int] = dutch_flag_partition(arr1, 1)
    if is_partitioned(result1, 1) == 1:
        passed = passed + 1

    counts: list[int] = count_partitions([1, 2, 3, 1, 2, 3], 2)
    if counts[0] == 2:
        passed = passed + 1

    if counts[1] == 2:
        passed = passed + 1

    if counts[2] == 2:
        passed = passed + 1

    arr2: list[int] = [3, 3, 3]
    result2: list[int] = dutch_flag_partition(arr2, 3)
    if result2[0] == 3 and result2[1] == 3 and result2[2] == 3:
        passed = passed + 1

    if is_partitioned([1, 1, 2, 2, 3, 3], 2) == 1:
        passed = passed + 1

    if is_partitioned([3, 1, 2], 2) == 0:
        passed = passed + 1

    swapped: list[int] = swap_elements([10, 20, 30], 0, 2)
    if swapped[0] == 30 and swapped[2] == 10:
        passed = passed + 1

    return passed
