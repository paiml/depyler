"""Two pointer technique patterns.

Tests: container with most water, three sum to zero, remove duplicates,
trapping rain water, sorted squares.
"""


def container_with_most_water(heights: list[int]) -> int:
    """Maximum water container area using two pointers."""
    n: int = len(heights)
    if n < 2:
        return 0
    left: int = 0
    right: int = n - 1
    max_area: int = 0
    while left < right:
        width: int = right - left
        h: int = heights[left]
        if heights[right] < h:
            h = heights[right]
        area: int = width * h
        if area > max_area:
            max_area = area
        if heights[left] < heights[right]:
            left = left + 1
        else:
            right = right - 1
    return max_area


def three_sum_zero(nums: list[int]) -> list[list[int]]:
    """Find all unique triplets that sum to zero."""
    n: int = len(nums)
    arr: list[int] = []
    i: int = 0
    while i < n:
        arr.append(nums[i])
        i = i + 1
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if arr[j] < arr[i]:
                tmp: int = arr[i]
                arr[i] = arr[j]
                arr[j] = tmp
            j = j + 1
        i = i + 1
    result: list[list[int]] = []
    i = 0
    while i < n - 2:
        if i > 0 and arr[i] == arr[i - 1]:
            i = i + 1
            continue
        left: int = i + 1
        right: int = n - 1
        while left < right:
            total: int = arr[i] + arr[left] + arr[right]
            if total == 0:
                result.append([arr[i], arr[left], arr[right]])
                left = left + 1
                while left < right and arr[left] == arr[left - 1]:
                    left = left + 1
                right = right - 1
                while left < right and arr[right] == arr[right + 1]:
                    right = right - 1
            elif total < 0:
                left = left + 1
            else:
                right = right - 1
        i = i + 1
    return result


def remove_duplicates_sorted(arr: list[int]) -> list[int]:
    """Remove duplicates from sorted array, return new array."""
    n: int = len(arr)
    if n == 0:
        return []
    result: list[int] = [arr[0]]
    i: int = 1
    while i < n:
        if arr[i] != arr[i - 1]:
            result.append(arr[i])
        i = i + 1
    return result


def trap_rain_water(heights: list[int]) -> int:
    """Trapping rain water using two pointer approach."""
    n: int = len(heights)
    if n < 3:
        return 0
    left: int = 0
    right: int = n - 1
    left_max: int = heights[0]
    right_max: int = heights[n - 1]
    water: int = 0
    while left < right:
        if heights[left] < heights[right]:
            left = left + 1
            if heights[left] > left_max:
                left_max = heights[left]
            else:
                water = water + left_max - heights[left]
        else:
            right = right - 1
            if heights[right] > right_max:
                right_max = heights[right]
            else:
                water = water + right_max - heights[right]
    return water


def sorted_squares(arr: list[int]) -> list[int]:
    """Squares of sorted array, returned in sorted order."""
    n: int = len(arr)
    result: list[int] = [0] * n
    left: int = 0
    right: int = n - 1
    pos: int = n - 1
    while left <= right:
        left_sq: int = arr[left] * arr[left]
        right_sq: int = arr[right] * arr[right]
        if left_sq >= right_sq:
            result[pos] = left_sq
            left = left + 1
        else:
            result[pos] = right_sq
            right = right - 1
        pos = pos - 1
    return result


def test_module() -> bool:
    """Test all two pointer functions."""
    ok: bool = True

    if container_with_most_water([1, 8, 6, 2, 5, 4, 8, 3, 7]) != 49:
        ok = False

    trips: list[list[int]] = three_sum_zero([-1, 0, 1, 2, -1, -4])
    if len(trips) != 2:
        ok = False

    dedup: list[int] = remove_duplicates_sorted([1, 1, 2, 3, 3, 3, 4])
    if dedup != [1, 2, 3, 4]:
        ok = False

    if trap_rain_water([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]) != 6:
        ok = False

    sq: list[int] = sorted_squares([-4, -1, 0, 3, 10])
    if sq != [0, 1, 9, 16, 100]:
        ok = False

    return ok
