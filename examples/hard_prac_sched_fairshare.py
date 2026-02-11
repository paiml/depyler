"""Fair Share scheduler simulation.

Divides CPU time among groups (users/departments) according to shares.
Tracks actual vs entitled usage to maintain fairness.
"""


def fs_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def fs_add_group(group_ids: list[int], shares: list[int],
                 usage: list[int], count_arr: list[int],
                 gid: int, share: int) -> int:
    """Add a group with given share weight. Returns index."""
    idx: int = count_arr[0]
    group_ids[idx] = gid
    shares[idx] = share
    usage[idx] = 0
    count_arr[0] = idx + 1
    return idx


def fs_total_shares(shares: list[int], count: int) -> int:
    """Sum of all shares."""
    total: int = 0
    i: int = 0
    while i < count:
        s: int = shares[i]
        total = total + s
        i = i + 1
    return total


def fs_entitled_pct(shares: list[int], idx: int, count: int) -> int:
    """Entitled percentage * 100 for group at idx."""
    total: int = fs_total_shares(shares, count)
    if total == 0:
        return 0
    s: int = shares[idx]
    return (s * 10000) // total


def fs_actual_pct(usage: list[int], idx: int, count: int) -> int:
    """Actual usage percentage * 100."""
    total: int = 0
    i: int = 0
    while i < count:
        u: int = usage[i]
        total = total + u
        i = i + 1
    if total == 0:
        return 0
    u_val: int = usage[idx]
    return (u_val * 10000) // total


def fs_most_underserved(shares: list[int], usage: list[int], count: int) -> int:
    """Find group most below its fair share. Returns index."""
    best: int = 0
    best_deficit: int = 0 - 2147483647
    total_shares: int = fs_total_shares(shares, count)
    total_usage: int = 0
    i: int = 0
    while i < count:
        u: int = usage[i]
        total_usage = total_usage + u
        i = i + 1
    if total_usage == 0:
        return 0
    j: int = 0
    while j < count:
        entitled: int = (shares[j] * total_usage) // total_shares
        actual: int = usage[j]
        deficit: int = entitled - actual
        if deficit > best_deficit:
            best_deficit = deficit
            best = j
        j = j + 1
    return best


def fs_schedule_tick(shares: list[int], usage: list[int], count: int) -> int:
    """Schedule one tick to most underserved group. Returns group index."""
    idx: int = fs_most_underserved(shares, usage, count)
    usage[idx] = usage[idx] + 1
    return idx


def fs_simulate(shares: list[int], usage: list[int], count: int, ticks: int) -> int:
    """Simulate fair share for given ticks. Returns 0."""
    t: int = 0
    while t < ticks:
        fs_schedule_tick(shares, usage, count)
        t = t + 1
    return 0


def test_module() -> int:
    """Test fair share scheduler."""
    passed: int = 0
    cap: int = 5
    gids: list[int] = fs_init_zeros(cap)
    shares: list[int] = fs_init_zeros(cap)
    usage: list[int] = fs_init_zeros(cap)
    cnt: list[int] = [0]

    # Test 1: add groups
    fs_add_group(gids, shares, usage, cnt, 1, 50)
    fs_add_group(gids, shares, usage, cnt, 2, 30)
    fs_add_group(gids, shares, usage, cnt, 3, 20)
    if cnt[0] == 3:
        passed = passed + 1

    # Test 2: total shares
    total: int = fs_total_shares(shares, cnt[0])
    if total == 100:
        passed = passed + 1

    # Test 3: entitled percentages
    e0: int = fs_entitled_pct(shares, 0, cnt[0])
    if e0 == 5000:
        passed = passed + 1

    # Test 4: simulate and verify proportional allocation
    fs_simulate(shares, usage, cnt[0], 100)
    u0: int = usage[0]
    u1: int = usage[1]
    u2: int = usage[2]
    if u0 > u1:
        if u1 > u2:
            passed = passed + 1

    # Test 5: total usage equals ticks
    total_u: int = u0 + u1 + u2
    if total_u == 100:
        passed = passed + 1

    return passed
