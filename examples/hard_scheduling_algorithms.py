"""Job and interval scheduling algorithm patterns.

Tests: interval scheduling maximization, weighted job scheduling,
task deadline scheduling, round-robin simulation, and meeting rooms.
"""


def max_non_overlapping(starts: list[int], ends: list[int]) -> int:
    """Maximum number of non-overlapping intervals (greedy by end time)."""
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


def min_meeting_rooms(starts: list[int], ends: list[int]) -> int:
    """Minimum number of meeting rooms required."""
    n: int = len(starts)
    events: list[list[int]] = []
    i: int = 0
    while i < n:
        events.append([starts[i], 1])
        events.append([ends[i], -1])
        i = i + 1
    ne: int = len(events)
    i = 0
    while i < ne:
        j: int = i + 1
        while j < ne:
            swap: bool = False
            if events[j][0] < events[i][0]:
                swap = True
            elif events[j][0] == events[i][0] and events[j][1] < events[i][1]:
                swap = True
            if swap:
                tmp: list[int] = events[i]
                events[i] = events[j]
                events[j] = tmp
            j = j + 1
        i = i + 1
    max_rooms: int = 0
    current: int = 0
    i = 0
    while i < ne:
        current = current + events[i][1]
        if current > max_rooms:
            max_rooms = current
        i = i + 1
    return max_rooms


def job_scheduling_deadline(deadlines: list[int], profits: list[int]) -> int:
    """Schedule jobs with deadlines for maximum profit (one job per time slot)."""
    n: int = len(deadlines)
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
            if profits[indices[j]] > profits[indices[i]]:
                tmp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = tmp
            j = j + 1
        i = i + 1
    max_deadline: int = 0
    i = 0
    while i < n:
        if deadlines[i] > max_deadline:
            max_deadline = deadlines[i]
        i = i + 1
    slots: list[int] = [-1] * (max_deadline + 1)
    total_profit: int = 0
    i = 0
    while i < n:
        idx: int = indices[i]
        slot: int = deadlines[idx]
        while slot > 0:
            if slots[slot] == -1:
                slots[slot] = idx
                total_profit = total_profit + profits[idx]
                slot = 0
            else:
                slot = slot - 1
        i = i + 1
    return total_profit


def round_robin_schedule(burst_times: list[int], quantum: int) -> list[int]:
    """Round-robin scheduling. Returns completion times for each process."""
    n: int = len(burst_times)
    remaining: list[int] = []
    completion: list[int] = [0] * n
    i: int = 0
    while i < n:
        remaining.append(burst_times[i])
        i = i + 1
    time: int = 0
    done: int = 0
    while done < n:
        progress: bool = False
        i = 0
        while i < n:
            if remaining[i] > 0:
                progress = True
                if remaining[i] <= quantum:
                    time = time + remaining[i]
                    remaining[i] = 0
                    completion[i] = time
                    done = done + 1
                else:
                    time = time + quantum
                    remaining[i] = remaining[i] - quantum
            i = i + 1
        if not progress:
            break
    return completion


def test_module() -> bool:
    """Test all scheduling algorithm functions."""
    ok: bool = True

    if max_non_overlapping([1, 2, 3, 0], [3, 4, 5, 6]) != 2:
        ok = False

    if min_meeting_rooms([0, 5, 15], [30, 10, 20]) != 2:
        ok = False
    if min_meeting_rooms([1, 2, 3], [4, 5, 6]) != 3:
        ok = False

    if job_scheduling_deadline([2, 1, 2, 1, 3], [100, 19, 27, 25, 15]) != 142:
        ok = False

    rr: list[int] = round_robin_schedule([10, 5, 8], 3)
    if rr[1] != 14:
        ok = False

    return ok
