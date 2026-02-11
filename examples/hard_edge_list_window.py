"""Sliding window patterns: max/min in window, running statistics."""


def max_in_window(arr: list[int], win_size: int) -> list[int]:
    """Find maximum value in each sliding window of given size."""
    result: list[int] = []
    n: int = len(arr)
    if n == 0 or win_size <= 0:
        return result
    i: int = 0
    while i <= n - win_size:
        max_val: int = arr[i]
        j: int = i + 1
        while j < i + win_size:
            if arr[j] > max_val:
                max_val = arr[j]
            j = j + 1
        result.append(max_val)
        i = i + 1
    return result


def min_in_window(arr: list[int], win_size: int) -> list[int]:
    """Find minimum value in each sliding window of given size."""
    result: list[int] = []
    n: int = len(arr)
    if n == 0 or win_size <= 0:
        return result
    i: int = 0
    while i <= n - win_size:
        min_val: int = arr[i]
        j: int = i + 1
        while j < i + win_size:
            if arr[j] < min_val:
                min_val = arr[j]
            j = j + 1
        result.append(min_val)
        i = i + 1
    return result


def sum_in_window(arr: list[int], win_size: int) -> list[int]:
    """Compute sum in each sliding window."""
    result: list[int] = []
    n: int = len(arr)
    if n == 0 or win_size <= 0:
        return result
    window_sum: int = 0
    i: int = 0
    while i < win_size and i < n:
        window_sum = window_sum + arr[i]
        i = i + 1
    if i == win_size:
        result.append(window_sum)
    i = win_size
    while i < n:
        window_sum = window_sum + arr[i] - arr[i - win_size]
        result.append(window_sum)
        i = i + 1
    return result


def max_subarray_of_size_k(arr: list[int], k_size: int) -> int:
    """Find maximum sum subarray of size k."""
    n: int = len(arr)
    if n == 0 or k_size <= 0 or k_size > n:
        return 0
    window_sum: int = 0
    i: int = 0
    while i < k_size:
        window_sum = window_sum + arr[i]
        i = i + 1
    best: int = window_sum
    i = k_size
    while i < n:
        window_sum = window_sum + arr[i] - arr[i - k_size]
        if window_sum > best:
            best = window_sum
        i = i + 1
    return best


def longest_subarray_with_sum(arr: list[int], target: int) -> int:
    """Find length of longest subarray with exact sum (non-negative values)."""
    n: int = len(arr)
    best: int = 0
    start: int = 0
    current_sum: int = 0
    end: int = 0
    while end < n:
        current_sum = current_sum + arr[end]
        while current_sum > target and start <= end:
            current_sum = current_sum - arr[start]
            start = start + 1
        if current_sum == target:
            window_len: int = end - start + 1
            if window_len > best:
                best = window_len
        end = end + 1
    return best


def count_distinct_in_window(arr: list[int], win_size: int) -> list[int]:
    """Count distinct elements in each sliding window."""
    result: list[int] = []
    n: int = len(arr)
    if n == 0 or win_size <= 0:
        return result
    i: int = 0
    while i <= n - win_size:
        seen: dict[int, int] = {}
        j: int = i
        while j < i + win_size:
            val: int = arr[j]
            seen[val] = 1
            j = j + 1
        count: int = 0
        for sk in seen:
            count = count + 1
        result.append(count)
        i = i + 1
    return result


def test_module() -> int:
    """Test all sliding window functions."""
    passed: int = 0
    r1: list[int] = max_in_window([1, 3, 2, 5, 1, 4], 3)
    if r1 == [3, 5, 5, 5]:
        passed = passed + 1
    r2: list[int] = min_in_window([1, 3, 2, 5, 1, 4], 3)
    if r2 == [1, 2, 1, 1]:
        passed = passed + 1
    r3: list[int] = sum_in_window([1, 2, 3, 4, 5], 3)
    if r3 == [6, 9, 12]:
        passed = passed + 1
    r4: int = max_subarray_of_size_k([2, 1, 5, 1, 3, 2], 3)
    if r4 == 9:
        passed = passed + 1
    r5: int = longest_subarray_with_sum([1, 2, 3, 1, 1, 1, 1], 3)
    if r5 == 3:
        passed = passed + 1
    r6: list[int] = count_distinct_in_window([1, 2, 1, 3, 2], 3)
    if r6 == [2, 3, 3]:
        passed = passed + 1
    r7: list[int] = max_in_window([], 3)
    if len(r7) == 0:
        passed = passed + 1
    r8: list[int] = sum_in_window([10], 1)
    if r8 == [10]:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
