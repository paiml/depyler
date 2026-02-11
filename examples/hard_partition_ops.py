"""Array partition operations.

Tests: partition point, balanced partition, min diff partition, even-odd count.
"""


def partition_point(arr: list[int], pivot: int) -> int:
    """Count elements less than or equal to pivot (partition point)."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] <= pivot:
            count = count + 1
        i = i + 1
    return count


def min_partition_diff(arr: list[int]) -> int:
    """Minimum difference between two subset sums."""
    n: int = len(arr)
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    half: int = total // 2
    dp: list[bool] = [False] * (half + 1)
    dp[0] = True
    i = 0
    while i < n:
        j: int = half
        while j >= arr[i]:
            if dp[j - arr[i]]:
                dp[j] = True
            j = j - 1
        i = i + 1
    best: int = 0
    k: int = half
    while k >= 0:
        if dp[k]:
            best = k
            k = -1
        else:
            k = k - 1
    return total - 2 * best


def count_even_odd(arr: list[int]) -> list[int]:
    """Count even and odd numbers."""
    even: int = 0
    odd: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] % 2 == 0:
            even = even + 1
        else:
            odd = odd + 1
        i = i + 1
    return [even, odd]


def sum_of_parts(arr: list[int]) -> int:
    """Sum of prefix sums (sum of all left partitions)."""
    n: int = len(arr)
    total: int = 0
    running: int = 0
    i: int = 0
    while i < n:
        running = running + arr[i]
        total = total + running
        i = i + 1
    return total


def test_module() -> int:
    """Test partition operations."""
    ok: int = 0
    if partition_point([3, 1, 4, 1, 5], 3) == 3:
        ok = ok + 1
    if min_partition_diff([1, 6, 11, 5]) == 1:
        ok = ok + 1
    if min_partition_diff([1, 2, 3]) == 0:
        ok = ok + 1
    eo: list[int] = count_even_odd([1, 2, 3, 4, 5])
    if eo[0] == 2:
        ok = ok + 1
    if eo[1] == 3:
        ok = ok + 1
    if sum_of_parts([1, 2, 3]) == 10:
        ok = ok + 1
    if sum_of_parts([5]) == 5:
        ok = ok + 1
    return ok
