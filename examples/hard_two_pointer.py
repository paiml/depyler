"""Two pointer technique problems on sorted arrays."""


def two_sum_sorted(arr: list[int], target: int) -> list[int]:
    """Find two indices in sorted array that sum to target. Returns [i,j] or [-1,-1]."""
    lo: int = 0
    hi: int = len(arr) - 1
    while lo < hi:
        s: int = arr[lo] + arr[hi]
        if s == target:
            result: list[int] = [lo, hi]
            return result
        if s < target:
            lo = lo + 1
        else:
            hi = hi - 1
    result: list[int] = [-1, -1]
    return result


def remove_duplicates(arr: list[int]) -> int:
    """Remove duplicates in-place from sorted array. Returns new length."""
    n: int = len(arr)
    if n == 0:
        return 0
    write_pos: int = 1
    read_pos: int = 1
    while read_pos < n:
        if arr[read_pos] != arr[write_pos - 1]:
            arr[write_pos] = arr[read_pos]
            write_pos = write_pos + 1
        read_pos = read_pos + 1
    return write_pos


def container_water(heights: list[int]) -> int:
    """Max water between two lines (container with most water)."""
    lo: int = 0
    hi: int = len(heights) - 1
    best: int = 0
    while lo < hi:
        width: int = hi - lo
        h: int = heights[lo]
        if heights[hi] < h:
            h = heights[hi]
        area: int = width * h
        if area > best:
            best = area
        if heights[lo] < heights[hi]:
            lo = lo + 1
        else:
            hi = hi - 1
    return best


def sort_colors(arr: list[int]) -> list[int]:
    """Dutch national flag: sort array of 0s, 1s, 2s in-place."""
    lo: int = 0
    mid: int = 0
    hi: int = len(arr) - 1
    while mid <= hi:
        if arr[mid] == 0:
            tmp: int = arr[lo]
            arr[lo] = arr[mid]
            arr[mid] = tmp
            lo = lo + 1
            mid = mid + 1
        elif arr[mid] == 1:
            mid = mid + 1
        else:
            tmp2: int = arr[mid]
            arr[mid] = arr[hi]
            arr[hi] = tmp2
            hi = hi - 1
    return arr


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [2, 7, 11, 15]
    pair: list[int] = two_sum_sorted(arr1, 9)
    if pair[0] == 0 and pair[1] == 1:
        passed = passed + 1

    nopair: list[int] = two_sum_sorted(arr1, 3)
    if nopair[0] == -1:
        passed = passed + 1

    arr2: list[int] = [1, 1, 2, 2, 3]
    new_len: int = remove_duplicates(arr2)
    if new_len == 3:
        passed = passed + 1

    heights: list[int] = [1, 8, 6, 2, 5, 4, 8, 3, 7]
    if container_water(heights) == 49:
        passed = passed + 1

    colors: list[int] = [2, 0, 2, 1, 1, 0]
    sorted_c: list[int] = sort_colors(colors)
    if sorted_c[0] == 0 and sorted_c[1] == 0 and sorted_c[2] == 1:
        passed = passed + 1

    arr3: list[int] = [1, 3, 5, 7, 9]
    pair2: list[int] = two_sum_sorted(arr3, 12)
    if pair2[0] == 1 and pair2[1] == 4:
        passed = passed + 1

    h2: list[int] = [1, 1]
    if container_water(h2) == 1:
        passed = passed + 1

    return passed
