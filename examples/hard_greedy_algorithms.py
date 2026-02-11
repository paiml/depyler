"""Greedy algorithm patterns.

Tests: activity selection, coin change greedy, jump game,
minimum platforms, and Huffman-like merge cost.
"""


def activity_selection(starts: list[int], ends: list[int]) -> int:
    """Maximum number of non-overlapping activities (sorted by end time)."""
    n: int = len(starts)
    if n == 0:
        return 0
    indices: list[int] = []
    i: int = 0
    while i < n:
        indices.append(i)
        i = i + 1
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if ends[indices[j]] < ends[indices[i]]:
                tmp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = tmp
            j = j + 1
        i = i + 1
    count: int = 1
    last_end: int = ends[indices[0]]
    k: int = 1
    while k < n:
        idx: int = indices[k]
        if starts[idx] >= last_end:
            count = count + 1
            last_end = ends[idx]
        k = k + 1
    return count


def greedy_coin_change(coins: list[int], amount: int) -> int:
    """Greedy coin change (works for canonical coin systems). Returns coin count or -1."""
    sorted_coins: list[int] = []
    i: int = 0
    while i < len(coins):
        sorted_coins.append(coins[i])
        i = i + 1
    i = 0
    while i < len(sorted_coins):
        j: int = i + 1
        while j < len(sorted_coins):
            if sorted_coins[j] > sorted_coins[i]:
                tmp: int = sorted_coins[i]
                sorted_coins[i] = sorted_coins[j]
                sorted_coins[j] = tmp
            j = j + 1
        i = i + 1
    remaining: int = amount
    count: int = 0
    ci: int = 0
    while ci < len(sorted_coins) and remaining > 0:
        while remaining >= sorted_coins[ci]:
            remaining = remaining - sorted_coins[ci]
            count = count + 1
        ci = ci + 1
    if remaining == 0:
        return count
    return -1


def can_jump(nums: list[int]) -> bool:
    """Jump game: can you reach the last index?"""
    n: int = len(nums)
    if n <= 1:
        return True
    max_reach: int = 0
    i: int = 0
    while i < n:
        if i > max_reach:
            return False
        candidate: int = i + nums[i]
        if candidate > max_reach:
            max_reach = candidate
        if max_reach >= n - 1:
            return True
        i = i + 1
    return max_reach >= n - 1


def min_platforms(arrivals: list[int], departures: list[int]) -> int:
    """Minimum platforms needed at a station."""
    n: int = len(arrivals)
    arr: list[int] = []
    dep: list[int] = []
    i: int = 0
    while i < n:
        arr.append(arrivals[i])
        dep.append(departures[i])
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
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if dep[j] < dep[i]:
                tmp: int = dep[i]
                dep[i] = dep[j]
                dep[j] = tmp
            j = j + 1
        i = i + 1
    platforms: int = 1
    max_platforms: int = 1
    ai: int = 1
    di: int = 0
    while ai < n and di < n:
        if arr[ai] <= dep[di]:
            platforms = platforms + 1
            ai = ai + 1
        else:
            platforms = platforms - 1
            di = di + 1
        if platforms > max_platforms:
            max_platforms = platforms
    return max_platforms


def merge_frequency_cost(freqs: list[int]) -> int:
    """Simulate Huffman-like merging: always merge two smallest, return total cost."""
    heap: list[int] = []
    i: int = 0
    while i < len(freqs):
        heap.append(freqs[i])
        i = i + 1
    total: int = 0
    while len(heap) > 1:
        min1_idx: int = 0
        j: int = 1
        while j < len(heap):
            if heap[j] < heap[min1_idx]:
                min1_idx = j
            j = j + 1
        min1: int = heap[min1_idx]
        new_heap: list[int] = []
        k: int = 0
        while k < len(heap):
            if k != min1_idx:
                new_heap.append(heap[k])
            k = k + 1
        heap = new_heap
        min2_idx: int = 0
        j = 1
        while j < len(heap):
            if heap[j] < heap[min2_idx]:
                min2_idx = j
            j = j + 1
        min2: int = heap[min2_idx]
        new_heap2: list[int] = []
        k = 0
        while k < len(heap):
            if k != min2_idx:
                new_heap2.append(heap[k])
            k = k + 1
        heap = new_heap2
        merged: int = min1 + min2
        total = total + merged
        heap.append(merged)
    return total


def test_module() -> bool:
    """Test all greedy algorithm functions."""
    ok: bool = True

    if activity_selection([1, 3, 0, 5, 8, 5], [2, 4, 6, 7, 9, 9]) != 4:
        ok = False

    if greedy_coin_change([1, 5, 10, 25], 41) != 4:
        ok = False
    if greedy_coin_change([1, 5, 10], 0) != 0:
        ok = False

    if not can_jump([2, 3, 1, 1, 4]):
        ok = False
    if can_jump([3, 2, 1, 0, 4]):
        ok = False

    if min_platforms([900, 940, 950, 1100, 1500, 1800],
                     [910, 1200, 1120, 1130, 1900, 2000]) != 3:
        ok = False

    if merge_frequency_cost([5, 10, 20, 30]) != 105:
        ok = False

    return ok
