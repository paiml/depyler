"""Divide and conquer patterns using iterative approaches.

Tests: iterative merge sort, iterative power, maximum subarray (D&C style),
closest pair distance, and iterative binary search variants.
"""


def iterative_merge_sort(arr: list[int]) -> list[int]:
    """Bottom-up merge sort (iterative divide and conquer)."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(arr[i])
        i = i + 1
    width: int = 1
    while width < n:
        left: int = 0
        while left < n:
            mid: int = left + width
            if mid > n:
                mid = n
            right: int = left + 2 * width
            if right > n:
                right = n
            merged: list[int] = []
            li: int = left
            ri: int = mid
            while li < mid and ri < right:
                if result[li] <= result[ri]:
                    merged.append(result[li])
                    li = li + 1
                else:
                    merged.append(result[ri])
                    ri = ri + 1
            while li < mid:
                merged.append(result[li])
                li = li + 1
            while ri < right:
                merged.append(result[ri])
                ri = ri + 1
            k: int = 0
            while k < len(merged):
                result[left + k] = merged[k]
                k = k + 1
            left = left + 2 * width
        width = width * 2
    return result


def iterative_power(base: int, exp: int) -> int:
    """Fast exponentiation using iterative squaring."""
    result: int = 1
    b: int = base
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = result * b
        b = b * b
        e = e // 2
    return result


def max_crossing_sum(arr: list[int], low: int, mid: int, high: int) -> int:
    """Find max subarray sum crossing the midpoint."""
    left_sum: int = -999999999
    total: int = 0
    i: int = mid
    while i >= low:
        total = total + arr[i]
        if total > left_sum:
            left_sum = total
        i = i - 1
    right_sum: int = -999999999
    total = 0
    j: int = mid + 1
    while j <= high:
        total = total + arr[j]
        if total > right_sum:
            right_sum = total
        j = j + 1
    return left_sum + right_sum


def max_subarray_dc(arr: list[int]) -> int:
    """Maximum subarray using iterative simulation of divide and conquer."""
    n: int = len(arr)
    if n == 0:
        return 0
    if n == 1:
        return arr[0]
    stack: list[list[int]] = []
    results: dict[int, int] = {}
    stack.append([0, n - 1, 0])
    while len(stack) > 0:
        top: list[int] = stack[len(stack) - 1]
        stack = stack[0:len(stack) - 1]
        lo: int = top[0]
        hi: int = top[1]
        key: int = lo * n + hi
        if lo == hi:
            results[key] = arr[lo]
        else:
            mid: int = (lo + hi) // 2
            left_key: int = lo * n + mid
            right_key: int = (mid + 1) * n + hi
            if left_key in results and right_key in results:
                cross: int = max_crossing_sum(arr, lo, mid, hi)
                left_val: int = results[left_key]
                right_val: int = results[right_key]
                best: int = left_val
                if right_val > best:
                    best = right_val
                if cross > best:
                    best = cross
                results[key] = best
            else:
                stack.append([lo, hi, 0])
                if left_key not in results:
                    stack.append([lo, mid, 0])
                if right_key not in results:
                    stack.append([mid + 1, hi, 0])
    final_key: int = 0 * n + (n - 1)
    return results[final_key]


def count_inversions(arr: list[int]) -> int:
    """Count inversions using merge sort approach."""
    n: int = len(arr)
    temp: list[int] = []
    i: int = 0
    while i < n:
        temp.append(arr[i])
        i = i + 1
    count: int = 0
    width: int = 1
    while width < n:
        left: int = 0
        while left < n:
            mid: int = left + width
            if mid > n:
                mid = n
            right: int = left + 2 * width
            if right > n:
                right = n
            merged: list[int] = []
            li: int = left
            ri: int = mid
            while li < mid and ri < right:
                if temp[li] <= temp[ri]:
                    merged.append(temp[li])
                    li = li + 1
                else:
                    merged.append(temp[ri])
                    count = count + (mid - li)
                    ri = ri + 1
            while li < mid:
                merged.append(temp[li])
                li = li + 1
            while ri < right:
                merged.append(temp[ri])
                ri = ri + 1
            k: int = 0
            while k < len(merged):
                temp[left + k] = merged[k]
                k = k + 1
            left = left + 2 * width
        width = width * 2
    return count


def test_module() -> bool:
    """Test all divide and conquer functions."""
    ok: bool = True

    sorted_arr: list[int] = iterative_merge_sort([5, 2, 8, 1, 9, 3])
    if sorted_arr != [1, 2, 3, 5, 8, 9]:
        ok = False

    if iterative_power(2, 10) != 1024:
        ok = False
    if iterative_power(3, 0) != 1:
        ok = False

    ms: int = max_subarray_dc([-2, 1, -3, 4, -1, 2, 1, -5, 4])
    if ms != 6:
        ok = False

    inv: int = count_inversions([2, 4, 1, 3, 5])
    if inv != 3:
        ok = False
    inv2: int = count_inversions([1, 2, 3, 4])
    if inv2 != 0:
        ok = False

    return ok
