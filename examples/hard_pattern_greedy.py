"""Greedy algorithms: activity selection, coin change greedy, jump game.

Tests: activity_select, greedy_coin, can_jump, min_platforms, fractional_knapsack.
"""


def activity_select(starts: list[int], ends: list[int]) -> int:
    """Select maximum non-overlapping activities (sorted by end time)."""
    n: int = len(starts)
    if n == 0:
        return 0
    count: int = 1
    last_end: int = ends[0]
    i: int = 1
    while i < n:
        if starts[i] >= last_end:
            count = count + 1
            last_end = ends[i]
        i = i + 1
    return count


def greedy_coin_change(coins: list[int], amount: int) -> int:
    """Greedy coin change (works for canonical coin systems). Returns coin count."""
    remaining: int = amount
    count: int = 0
    i: int = len(coins) - 1
    while i >= 0:
        c: int = coins[i]
        while remaining >= c:
            remaining = remaining - c
            count = count + 1
        i = i - 1
    if remaining > 0:
        return -1
    return count


def can_jump(nums: list[int]) -> int:
    """Jump game: can reach last index? Returns 1 or 0."""
    n: int = len(nums)
    if n <= 1:
        return 1
    max_reach: int = 0
    i: int = 0
    while i < n:
        if i > max_reach:
            return 0
        new_reach: int = i + nums[i]
        if new_reach > max_reach:
            max_reach = new_reach
        if max_reach >= n - 1:
            return 1
        i = i + 1
    return 0


def min_jumps(nums: list[int]) -> int:
    """Minimum jumps to reach end. Returns -1 if impossible."""
    n: int = len(nums)
    if n <= 1:
        return 0
    jumps: int = 0
    current_end: int = 0
    farthest: int = 0
    i: int = 0
    while i < n - 1:
        new_reach: int = i + nums[i]
        if new_reach > farthest:
            farthest = new_reach
        if i == current_end:
            jumps = jumps + 1
            current_end = farthest
            if current_end >= n - 1:
                return jumps
        i = i + 1
    return -1


def min_platforms(arrivals: list[int], departures: list[int]) -> int:
    """Minimum platforms needed at a station."""
    n: int = len(arrivals)
    events: list[int] = []
    i: int = 0
    while i < n:
        events.append(arrivals[i] * 2)
        events.append(departures[i] * 2 + 1)
        i = i + 1
    ne: int = len(events)
    ei: int = 0
    while ei < ne - 1:
        ej: int = ei + 1
        while ej < ne:
            if events[ej] < events[ei]:
                tmp: int = events[ei]
                events[ei] = events[ej]
                events[ej] = tmp
            ej = ej + 1
        ei = ei + 1
    current: int = 0
    max_plat: int = 0
    k: int = 0
    while k < ne:
        ev: int = events[k]
        if ev % 2 == 0:
            current = current + 1
        else:
            current = current - 1
        if current > max_plat:
            max_plat = current
        k = k + 1
    return max_plat


def fractional_knapsack_approx(weights: list[int], values: list[int], capacity: int) -> int:
    """Approximate fractional knapsack using integer ratio sorting. Returns integer value."""
    n: int = len(weights)
    ratios: list[int] = []
    order: list[int] = []
    i: int = 0
    while i < n:
        ratios.append(values[i] * 1000 // weights[i])
        order.append(i)
        i = i + 1
    oi: int = 0
    while oi < n - 1:
        oj: int = oi + 1
        while oj < n:
            if ratios[order[oj]] > ratios[order[oi]]:
                tmp: int = order[oi]
                order[oi] = order[oj]
                order[oj] = tmp
            oj = oj + 1
        oi = oi + 1
    remaining: int = capacity
    total_value: int = 0
    k: int = 0
    while k < n:
        idx: int = order[k]
        w: int = weights[idx]
        v: int = values[idx]
        if w <= remaining:
            total_value = total_value + v
            remaining = remaining - w
        else:
            total_value = total_value + v * remaining // w
            remaining = 0
        k = k + 1
    return total_value


def test_module() -> int:
    """Test greedy algorithms."""
    passed: int = 0

    s: list[int] = [1, 3, 0, 5, 8, 5]
    e: list[int] = [2, 4, 6, 7, 9, 9]
    if activity_select(s, e) == 4:
        passed = passed + 1

    if greedy_coin_change([1, 5, 10, 25], 41) == 4:
        passed = passed + 1

    if can_jump([2, 3, 1, 1, 4]) == 1:
        passed = passed + 1

    if can_jump([3, 2, 1, 0, 4]) == 0:
        passed = passed + 1

    if min_jumps([2, 3, 1, 1, 4]) == 2:
        passed = passed + 1

    arr: list[int] = [900, 940, 950, 1100, 1500, 1800]
    dep: list[int] = [910, 1200, 1120, 1130, 1900, 2000]
    if min_platforms(arr, dep) == 3:
        passed = passed + 1

    return passed
