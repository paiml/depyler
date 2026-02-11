"""Jump game variants.

Tests: can reach end, minimum jumps, maximum reach.
"""


def can_reach_end(jumps: list[int]) -> int:
    """Check if can reach last index. jumps[i] = max jump from i. Returns 1 if reachable."""
    n: int = len(jumps)
    if n <= 1:
        return 1
    max_reach: int = 0
    i: int = 0
    while i < n:
        if i > max_reach:
            return 0
        new_reach: int = i + jumps[i]
        if new_reach > max_reach:
            max_reach = new_reach
        if max_reach >= n - 1:
            return 1
        i = i + 1
    return 0


def min_jumps_to_end(jumps: list[int]) -> int:
    """Minimum number of jumps to reach end. Returns -1 if impossible."""
    n: int = len(jumps)
    if n <= 1:
        return 0
    if jumps[0] == 0:
        return -1
    max_reach: int = jumps[0]
    step_reach: int = jumps[0]
    count: int = 1
    i: int = 1
    while i < n:
        if i == n - 1:
            return count
        new_reach: int = i + jumps[i]
        if new_reach > max_reach:
            max_reach = new_reach
        step_reach = step_reach - 1
        if step_reach == 0:
            count = count + 1
            if i >= max_reach:
                return -1
            step_reach = max_reach - i
        i = i + 1
    return count


def max_reachable_index(jumps: list[int]) -> int:
    """Find the maximum index reachable from index 0."""
    n: int = len(jumps)
    max_reach: int = 0
    i: int = 0
    while i <= max_reach:
        if i >= n:
            break
        new_reach: int = i + jumps[i]
        if new_reach > max_reach:
            max_reach = new_reach
        i = i + 1
    if max_reach >= n:
        max_reach = n - 1
    return max_reach


def test_module() -> int:
    """Test jump game operations."""
    ok: int = 0
    if can_reach_end([2, 3, 1, 1, 4]) == 1:
        ok = ok + 1
    if can_reach_end([3, 2, 1, 0, 4]) == 0:
        ok = ok + 1
    if min_jumps_to_end([2, 3, 1, 1, 4]) == 2:
        ok = ok + 1
    if max_reachable_index([2, 3, 1, 1, 4]) == 4:
        ok = ok + 1
    if max_reachable_index([1, 0, 0, 0]) == 1:
        ok = ok + 1
    return ok
