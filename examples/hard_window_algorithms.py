"""Sliding window algorithm patterns.

Tests: maximum sum window, minimum window containing target sum,
longest substring without repeats, window average, and max in each window.
"""


def max_sum_window(arr: list[int], k: int) -> int:
    """Maximum sum of any contiguous subarray of size k."""
    n: int = len(arr)
    if k > n or k <= 0:
        return 0
    window_sum: int = 0
    i: int = 0
    while i < k:
        window_sum = window_sum + arr[i]
        i = i + 1
    max_sum: int = window_sum
    i = k
    while i < n:
        window_sum = window_sum + arr[i] - arr[i - k]
        if window_sum > max_sum:
            max_sum = window_sum
        i = i + 1
    return max_sum


def min_subarray_len(arr: list[int], target: int) -> int:
    """Minimum length subarray with sum >= target. Returns 0 if impossible."""
    n: int = len(arr)
    if n == 0:
        return 0
    min_len: int = n + 1
    left: int = 0
    current_sum: int = 0
    right: int = 0
    while right < n:
        current_sum = current_sum + arr[right]
        while current_sum >= target:
            window_len: int = right - left + 1
            if window_len < min_len:
                min_len = window_len
            current_sum = current_sum - arr[left]
            left = left + 1
        right = right + 1
    if min_len > n:
        return 0
    return min_len


def longest_unique_substring_len(s: str) -> int:
    """Length of longest substring without repeating characters."""
    n: int = len(s)
    if n == 0:
        return 0
    last_seen: dict[str, int] = {}
    max_len: int = 0
    left: int = 0
    right: int = 0
    while right < n:
        c: str = s[right]
        if c in last_seen:
            pos: int = last_seen[c]
            if pos >= left:
                left = pos + 1
        last_seen[c] = right
        window_len: int = right - left + 1
        if window_len > max_len:
            max_len = window_len
        right = right + 1
    return max_len


def window_averages(arr: list[int], k: int) -> list[int]:
    """Compute average (integer division) of each window of size k."""
    n: int = len(arr)
    if k > n or k <= 0:
        return []
    result: list[int] = []
    window_sum: int = 0
    i: int = 0
    while i < k:
        window_sum = window_sum + arr[i]
        i = i + 1
    result.append(window_sum // k)
    i = k
    while i < n:
        window_sum = window_sum + arr[i] - arr[i - k]
        result.append(window_sum // k)
        i = i + 1
    return result


def count_subarrays_with_sum(arr: list[int], target: int) -> int:
    """Count subarrays with exact sum equal to target using prefix sum + dict."""
    count: int = 0
    prefix: int = 0
    prefix_counts: dict[int, int] = {0: 1}
    i: int = 0
    while i < len(arr):
        prefix = prefix + arr[i]
        complement: int = prefix - target
        if complement in prefix_counts:
            count = count + prefix_counts[complement]
        if prefix in prefix_counts:
            prefix_counts[prefix] = prefix_counts[prefix] + 1
        else:
            prefix_counts[prefix] = 1
        i = i + 1
    return count


def test_module() -> bool:
    """Test all window algorithm functions."""
    ok: bool = True

    if max_sum_window([2, 1, 5, 1, 3, 2], 3) != 9:
        ok = False

    if min_subarray_len([2, 3, 1, 2, 4, 3], 7) != 2:
        ok = False
    if min_subarray_len([1, 1, 1], 100) != 0:
        ok = False

    if longest_unique_substring_len("abcabcbb") != 3:
        ok = False
    if longest_unique_substring_len("bbbbb") != 1:
        ok = False

    avgs: list[int] = window_averages([1, 3, 2, 6, -1, 4, 1, 8, 2], 5)
    if len(avgs) != 5:
        ok = False
    if avgs[0] != 2:
        ok = False

    if count_subarrays_with_sum([1, 1, 1], 2) != 2:
        ok = False
    if count_subarrays_with_sum([1, 2, 3], 3) != 2:
        ok = False

    return ok
