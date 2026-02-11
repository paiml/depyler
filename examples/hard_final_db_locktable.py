"""Lock table for transaction concurrency control.

Simulates shared (S) and exclusive (X) lock management.
Tracks lock holders and implements deadlock detection via wait-for graph.
"""


def create_lock_table(num_resources: int) -> list[int]:
    """Create lock table. lock_modes[i]: 0=none, 1=shared, 2=exclusive."""
    modes: list[int] = []
    i: int = 0
    while i < num_resources:
        modes.append(0)
        i = i + 1
    return modes


def create_holders(num_resources: int, max_holders: int) -> list[int]:
    """Flat array: holders[res * max_holders + j] = txn_id or -1."""
    arr: list[int] = []
    i: int = 0
    while i < num_resources * max_holders:
        arr.append(0 - 1)
        i = i + 1
    return arr


def acquire_shared(lock_modes: list[int], holders: list[int], resource: int, txn_id: int, max_h: int) -> int:
    """Try to acquire shared lock. Returns 1 if granted, 0 if blocked."""
    mode: int = lock_modes[resource]
    if mode == 2:
        return 0
    lock_modes[resource] = 1
    j: int = 0
    while j < max_h:
        hv: int = holders[resource * max_h + j]
        if hv < 0:
            holders[resource * max_h + j] = txn_id
            return 1
        j = j + 1
    return 0


def acquire_exclusive(lock_modes: list[int], holders: list[int], resource: int, txn_id: int, max_h: int) -> int:
    """Try to acquire exclusive lock. Returns 1 if granted, 0 if blocked."""
    mode: int = lock_modes[resource]
    if mode != 0:
        if mode == 1:
            hv0: int = holders[resource * max_h]
            if hv0 == txn_id:
                has_others: int = 0
                j2: int = 1
                while j2 < max_h:
                    hcheck: int = holders[resource * max_h + j2]
                    if hcheck >= 0:
                        has_others = 1
                    j2 = j2 + 1
                if has_others == 0:
                    lock_modes[resource] = 2
                    return 1
            return 0
        return 0
    lock_modes[resource] = 2
    holders[resource * max_h] = txn_id
    return 1


def release_lock(lock_modes: list[int], holders: list[int], resource: int, txn_id: int, max_h: int) -> int:
    """Release all locks held by txn_id on resource. Returns 1 if released."""
    released: int = 0
    j: int = 0
    while j < max_h:
        hv: int = holders[resource * max_h + j]
        if hv == txn_id:
            holders[resource * max_h + j] = 0 - 1
            released = 1
        j = j + 1
    if released == 1:
        has_any: int = 0
        k: int = 0
        while k < max_h:
            hv2: int = holders[resource * max_h + k]
            if hv2 >= 0:
                has_any = 1
            k = k + 1
        if has_any == 0:
            lock_modes[resource] = 0
    return released


def count_locks_held(lock_modes: list[int]) -> int:
    """Count resources with any lock held."""
    cnt: int = 0
    i: int = 0
    while i < len(lock_modes):
        mv: int = lock_modes[i]
        if mv > 0:
            cnt = cnt + 1
        i = i + 1
    return cnt


def detect_cycle(waits_for: list[int], num_txns: int) -> int:
    """Detect cycle in wait-for graph. waits_for[i] = txn that i waits for, -1 if none.

    Returns 1 if cycle detected. Uses tortoise-and-hare approach.
    """
    i: int = 0
    while i < num_txns:
        slow: int = i
        fast: int = i
        step: int = 0
        while step < num_txns + 1:
            sw: int = waits_for[slow]
            if sw < 0:
                step = num_txns + 1
            else:
                slow = sw
                fw1: int = waits_for[fast]
                if fw1 < 0:
                    step = num_txns + 1
                else:
                    fw2: int = waits_for[fw1]
                    if fw2 < 0:
                        step = num_txns + 1
                    else:
                        fast = fw2
                        if slow == fast:
                            return 1
            step = step + 1
        i = i + 1
    return 0


def test_module() -> int:
    """Test lock table."""
    ok: int = 0
    nr: int = 3
    mh: int = 4
    modes: list[int] = create_lock_table(nr)
    holders: list[int] = create_holders(nr, mh)
    s1: int = acquire_shared(modes, holders, 0, 1, mh)
    if s1 == 1:
        ok = ok + 1
    s2: int = acquire_shared(modes, holders, 0, 2, mh)
    if s2 == 1:
        ok = ok + 1
    x1: int = acquire_exclusive(modes, holders, 1, 1, mh)
    if x1 == 1:
        ok = ok + 1
    x2_fail: int = acquire_exclusive(modes, holders, 1, 2, mh)
    if x2_fail == 0:
        ok = ok + 1
    wf: list[int] = [1, 2, 0]
    cyc: int = detect_cycle(wf, 3)
    if cyc == 1:
        ok = ok + 1
    return ok
