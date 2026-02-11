"""Lottery scheduler simulation.

Probabilistic scheduling where each task holds tickets.
A deterministic PRNG selects a winning ticket each quantum.
"""


def lot_lcg(seed: int) -> int:
    """Linear congruential generator for reproducible randomness."""
    result: int = (seed * 1103515245 + 12345) % 2147483648
    return result


def lot_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def lot_add_task(task_ids: list[int], tickets: list[int],
                 count_arr: list[int], tid: int, ticket_count: int) -> int:
    """Add task with tickets. Returns index."""
    idx: int = count_arr[0]
    task_ids[idx] = tid
    tickets[idx] = ticket_count
    count_arr[0] = idx + 1
    return idx


def lot_total_tickets(tickets: list[int], count: int) -> int:
    """Sum of all tickets."""
    total: int = 0
    i: int = 0
    while i < count:
        t: int = tickets[i]
        total = total + t
        i = i + 1
    return total


def lot_draw_winner(tickets: list[int], task_ids: list[int],
                    count: int, seed: int) -> int:
    """Draw a winning ticket. Returns task_id of winner."""
    total: int = lot_total_tickets(tickets, count)
    if total == 0:
        return 0 - 1
    new_seed: int = lot_lcg(seed)
    winning: int = new_seed % total
    cumulative: int = 0
    i: int = 0
    while i < count:
        t: int = tickets[i]
        cumulative = cumulative + t
        if winning < cumulative:
            result: int = task_ids[i]
            return result
        i = i + 1
    last: int = task_ids[count - 1]
    return last


def lot_simulate(tickets: list[int], task_ids: list[int],
                 wins: list[int], count: int, rounds: int, seed: int) -> int:
    """Simulate lottery for given rounds. wins[] tracks wins per task. Returns final seed."""
    s: int = seed
    r: int = 0
    while r < rounds:
        s = lot_lcg(s)
        total: int = lot_total_tickets(tickets, count)
        if total > 0:
            winning: int = s % total
            cumulative: int = 0
            i: int = 0
            while i < count:
                t: int = tickets[i]
                cumulative = cumulative + t
                if winning < cumulative:
                    wins[i] = wins[i] + 1
                    i = count
                i = i + 1
        r = r + 1
    return s


def lot_transfer_tickets(tickets: list[int], from_idx: int, to_idx: int, amount: int) -> int:
    """Transfer tickets between tasks. Returns 1 on success."""
    current: int = tickets[from_idx]
    if current < amount:
        return 0
    tickets[from_idx] = current - amount
    tickets[to_idx] = tickets[to_idx] + amount
    return 1


def test_module() -> int:
    """Test lottery scheduler."""
    passed: int = 0
    cap: int = 5
    tids: list[int] = lot_init_zeros(cap)
    tickets: list[int] = lot_init_zeros(cap)
    cnt: list[int] = [0]

    # Test 1: add tasks
    lot_add_task(tids, tickets, cnt, 100, 10)
    lot_add_task(tids, tickets, cnt, 200, 20)
    lot_add_task(tids, tickets, cnt, 300, 30)
    if cnt[0] == 3:
        passed = passed + 1

    # Test 2: total tickets
    total: int = lot_total_tickets(tickets, cnt[0])
    if total == 60:
        passed = passed + 1

    # Test 3: drawing produces a valid winner
    winner: int = lot_draw_winner(tickets, tids, cnt[0], 42)
    if winner == 100:
        passed = passed + 1
    if winner == 200:
        passed = passed + 1
    if winner == 300:
        passed = passed + 1

    # Test 4: simulation gives all tasks some wins
    wins: list[int] = lot_init_zeros(cap)
    lot_simulate(tickets, tids, wins, cnt[0], 1000, 42)
    w0: int = wins[0]
    w1: int = wins[1]
    w2: int = wins[2]
    if w0 > 0:
        if w1 > 0:
            if w2 > 0:
                passed = passed + 1

    # Test 5: ticket transfer
    ok: int = lot_transfer_tickets(tickets, 2, 0, 5)
    if ok == 1:
        new_total: int = lot_total_tickets(tickets, cnt[0])
        if new_total == 60:
            passed = passed + 1

    return passed
