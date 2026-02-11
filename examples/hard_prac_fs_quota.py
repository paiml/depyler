"""File system quota tracking simulation.

Tracks per-user disk usage against soft and hard limits.
Supports grace periods for soft limit violations.
"""


def qt_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def qt_init_neg(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def qt_add_user(user_ids: list[int], soft_limits: list[int],
                hard_limits: list[int], usage_arr: list[int],
                count_arr: list[int], uid: int,
                soft: int, hard: int) -> int:
    """Add user quota. Returns index."""
    idx: int = count_arr[0]
    user_ids[idx] = uid
    soft_limits[idx] = soft
    hard_limits[idx] = hard
    usage_arr[idx] = 0
    count_arr[0] = idx + 1
    return idx


def qt_find_user(user_ids: list[int], count: int, uid: int) -> int:
    """Find user index. Returns -1 if not found."""
    i: int = 0
    while i < count:
        u: int = user_ids[i]
        if u == uid:
            return i
        i = i + 1
    return 0 - 1


def qt_can_write(soft_limits: list[int], hard_limits: list[int],
                 usage_arr: list[int], idx: int, amount: int) -> int:
    """Check if user can write amount bytes.
    Returns: 0=ok, 1=soft_exceeded, 2=hard_exceeded(blocked)."""
    current: int = usage_arr[idx]
    new_usage: int = current + amount
    hard: int = hard_limits[idx]
    if new_usage > hard:
        return 2
    soft: int = soft_limits[idx]
    if new_usage > soft:
        return 1
    return 0


def qt_write(usage_arr: list[int], hard_limits: list[int],
             idx: int, amount: int) -> int:
    """Write amount bytes. Returns 1 on success, 0 if hard limit exceeded."""
    current: int = usage_arr[idx]
    new_usage: int = current + amount
    hard: int = hard_limits[idx]
    if new_usage > hard:
        return 0
    usage_arr[idx] = new_usage
    return 1


def qt_delete(usage_arr: list[int], idx: int, amount: int) -> int:
    """Delete amount bytes. Returns new usage."""
    current: int = usage_arr[idx]
    new_usage: int = current - amount
    if new_usage < 0:
        new_usage = 0
    usage_arr[idx] = new_usage
    return new_usage


def qt_usage_pct(usage_arr: list[int], hard_limits: list[int], idx: int) -> int:
    """Usage as percentage of hard limit."""
    u: int = usage_arr[idx]
    h: int = hard_limits[idx]
    if h == 0:
        return 0
    return (u * 100) // h


def qt_total_usage(usage_arr: list[int], count: int) -> int:
    """Sum of all users' usage."""
    total: int = 0
    i: int = 0
    while i < count:
        u: int = usage_arr[i]
        total = total + u
        i = i + 1
    return total


def test_module() -> int:
    """Test quota tracking."""
    passed: int = 0
    cap: int = 5
    uids: list[int] = qt_init_neg(cap)
    soft: list[int] = qt_init_zeros(cap)
    hard: list[int] = qt_init_zeros(cap)
    usage: list[int] = qt_init_zeros(cap)
    cnt: list[int] = [0]

    # Test 1: add user with quota
    qt_add_user(uids, soft, hard, usage, cnt, 1000, 800, 1000)
    if cnt[0] == 1:
        passed = passed + 1

    # Test 2: write within limit
    ok: int = qt_write(usage, hard, 0, 500)
    if ok == 1:
        passed = passed + 1

    # Test 3: soft limit check
    status: int = qt_can_write(soft, hard, usage, 0, 400)
    if status == 1:
        passed = passed + 1

    # Test 4: hard limit blocks write
    blocked: int = qt_write(usage, hard, 0, 600)
    if blocked == 0:
        passed = passed + 1

    # Test 5: usage percentage
    pct: int = qt_usage_pct(usage, hard, 0)
    if pct == 50:
        passed = passed + 1

    return passed
