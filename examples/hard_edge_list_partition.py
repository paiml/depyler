"""List partitioning: Dutch national flag, even/odd split, pivot-based."""


def partition_by_pivot(arr: list[int], pivot: int) -> list[int]:
    """Partition list into elements < pivot, then >= pivot."""
    lo_part: list[int] = []
    hi_part: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] < pivot:
            lo_part.append(arr[i])
        else:
            hi_part.append(arr[i])
        i = i + 1
    result: list[int] = []
    j: int = 0
    while j < len(lo_part):
        result.append(lo_part[j])
        j = j + 1
    j = 0
    while j < len(hi_part):
        result.append(hi_part[j])
        j = j + 1
    return result


def dutch_flag_sort(arr: list[int]) -> list[int]:
    """Sort array containing only 0, 1, 2 using single pass."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    lo: int = 0
    mid: int = 0
    hi: int = len(result) - 1
    while mid <= hi:
        if result[mid] == 0:
            tmp: int = result[lo]
            result[lo] = result[mid]
            result[mid] = tmp
            lo = lo + 1
            mid = mid + 1
        elif result[mid] == 1:
            mid = mid + 1
        else:
            tmp2: int = result[mid]
            result[mid] = result[hi]
            result[hi] = tmp2
            hi = hi - 1
    return result


def partition_even_odd(arr: list[int]) -> list[int]:
    """Put all even numbers before odd numbers."""
    evens: list[int] = []
    odds: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] % 2 == 0:
            evens.append(arr[i])
        else:
            odds.append(arr[i])
        i = i + 1
    result: list[int] = []
    j: int = 0
    while j < len(evens):
        result.append(evens[j])
        j = j + 1
    j = 0
    while j < len(odds):
        result.append(odds[j])
        j = j + 1
    return result


def partition_negative_positive(arr: list[int]) -> list[int]:
    """Put all negative numbers before non-negative."""
    neg: list[int] = []
    pos: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] < 0:
            neg.append(arr[i])
        else:
            pos.append(arr[i])
        i = i + 1
    result: list[int] = []
    j: int = 0
    while j < len(neg):
        result.append(neg[j])
        j = j + 1
    j = 0
    while j < len(pos):
        result.append(pos[j])
        j = j + 1
    return result


def three_way_split(arr: list[int], lo_val: int, hi_val: int) -> list[int]:
    """Split into < lo_val, [lo_val..hi_val], > hi_val."""
    below: list[int] = []
    between: list[int] = []
    above: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] < lo_val:
            below.append(arr[i])
        elif arr[i] > hi_val:
            above.append(arr[i])
        else:
            between.append(arr[i])
        i = i + 1
    result: list[int] = []
    j: int = 0
    while j < len(below):
        result.append(below[j])
        j = j + 1
    j = 0
    while j < len(between):
        result.append(between[j])
        j = j + 1
    j = 0
    while j < len(above):
        result.append(above[j])
        j = j + 1
    return result


def stable_partition(arr: list[int], threshold: int) -> list[int]:
    """Stable partition: elements <= threshold first, rest after."""
    lo_part: list[int] = []
    hi_part: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] <= threshold:
            lo_part.append(arr[i])
        else:
            hi_part.append(arr[i])
        i = i + 1
    result: list[int] = []
    j: int = 0
    while j < len(lo_part):
        result.append(lo_part[j])
        j = j + 1
    j = 0
    while j < len(hi_part):
        result.append(hi_part[j])
        j = j + 1
    return result


def test_module() -> int:
    """Test all partition functions."""
    passed: int = 0
    r1: list[int] = partition_by_pivot([5, 1, 8, 3, 7, 2], 5)
    ok1: int = 1
    i: int = 0
    while i < 3:
        if r1[i] >= 5:
            ok1 = 0
        i = i + 1
    if ok1 == 1:
        passed = passed + 1
    r2: list[int] = dutch_flag_sort([2, 0, 1, 2, 0, 1])
    if r2 == [0, 0, 1, 1, 2, 2]:
        passed = passed + 1
    r3: list[int] = dutch_flag_sort([])
    if len(r3) == 0:
        passed = passed + 1
    r4: list[int] = partition_even_odd([1, 2, 3, 4, 5, 6])
    if r4[0] == 2:
        passed = passed + 1
    r5: list[int] = partition_negative_positive([3, -1, 4, -5, 0])
    if r5[0] == -1:
        passed = passed + 1
    r6: list[int] = three_way_split([1, 5, 3, 8, 2, 7, 4], 3, 5)
    if r6[0] < 3:
        passed = passed + 1
    r7: list[int] = stable_partition([5, 1, 3, 7, 2, 4], 3)
    if r7 == [1, 3, 2, 5, 7, 4]:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
