"""Job sequencing with deadlines: maximize profit."""


def sort_jobs_by_profit_desc(profits: list[int], deadlines: list[int]) -> list[int]:
    """Return indices sorted by profit descending."""
    length: int = len(profits)
    indices: list[int] = []
    idx: int = 0
    while idx < length:
        indices.append(idx)
        idx = idx + 1
    i: int = 0
    while i < length:
        j: int = i + 1
        while j < length:
            if profits[indices[j]] > profits[indices[i]]:
                temp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = temp
            j = j + 1
        i = i + 1
    return indices


def max_profit_jobs(profits: list[int], deadlines: list[int]) -> int:
    """Return maximum total profit from job sequencing."""
    length: int = len(profits)
    if length == 0:
        return 0
    max_deadline: int = 0
    idx: int = 0
    while idx < length:
        if deadlines[idx] > max_deadline:
            max_deadline = deadlines[idx]
        idx = idx + 1
    slots: list[int] = []
    si: int = 0
    while si < max_deadline:
        slots.append(-1)
        si = si + 1
    order: list[int] = sort_jobs_by_profit_desc(profits, deadlines)
    total_profit: int = 0
    oi: int = 0
    while oi < length:
        job_idx: int = order[oi]
        slot: int = deadlines[job_idx] - 1
        while slot >= 0:
            if slots[slot] == -1:
                slots[slot] = job_idx
                total_profit = total_profit + profits[job_idx]
                slot = -1
            else:
                slot = slot - 1
        oi = oi + 1
    return total_profit


def count_scheduled_jobs(profits: list[int], deadlines: list[int]) -> int:
    """Count how many jobs can be scheduled."""
    length: int = len(profits)
    if length == 0:
        return 0
    max_deadline: int = 0
    idx: int = 0
    while idx < length:
        if deadlines[idx] > max_deadline:
            max_deadline = deadlines[idx]
        idx = idx + 1
    slots: list[int] = []
    si: int = 0
    while si < max_deadline:
        slots.append(-1)
        si = si + 1
    order: list[int] = sort_jobs_by_profit_desc(profits, deadlines)
    count: int = 0
    oi: int = 0
    while oi < length:
        job_idx: int = order[oi]
        slot: int = deadlines[job_idx] - 1
        while slot >= 0:
            if slots[slot] == -1:
                slots[slot] = job_idx
                count = count + 1
                slot = -1
            else:
                slot = slot - 1
        oi = oi + 1
    return count


def test_module() -> int:
    passed: int = 0

    profits: list[int] = [20, 15, 10, 5, 1]
    deadlines: list[int] = [2, 2, 1, 3, 3]

    if max_profit_jobs(profits, deadlines) == 40:
        passed = passed + 1

    if count_scheduled_jobs(profits, deadlines) == 3:
        passed = passed + 1

    if max_profit_jobs([], []) == 0:
        passed = passed + 1

    p2: list[int] = [100]
    d2: list[int] = [1]
    if max_profit_jobs(p2, d2) == 100:
        passed = passed + 1
    if count_scheduled_jobs(p2, d2) == 1:
        passed = passed + 1

    p3: list[int] = [10, 20, 30]
    d3: list[int] = [1, 1, 1]
    if max_profit_jobs(p3, d3) == 30:
        passed = passed + 1

    return passed
