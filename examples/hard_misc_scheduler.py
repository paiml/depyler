def round_robin(tasks: list[int], quantum: int) -> list[int]:
    n: int = len(tasks)
    remaining: list[int] = []
    i: int = 0
    while i < n:
        remaining.append(tasks[i])
        i = i + 1
    order: list[int] = []
    done: int = 0
    while done == 0:
        all_done: int = 1
        j: int = 0
        while j < n:
            r: int = remaining[j]
            if r > 0:
                all_done = 0
                if r <= quantum:
                    order.append(j)
                    remaining[j] = 0
                else:
                    order.append(j)
                    remaining[j] = r - quantum
            j = j + 1
        if all_done == 1:
            done = 1
    return order

def priority_schedule(tasks: list[int], priorities: list[int]) -> list[int]:
    n: int = len(tasks)
    done: list[int] = []
    i: int = 0
    while i < n:
        done.append(0)
        i = i + 1
    order: list[int] = []
    scheduled: int = 0
    while scheduled < n:
        best: int = 0 - 1
        best_pri: int = 0 - 1
        j: int = 0
        while j < n:
            dj: int = done[j]
            if dj == 0:
                pj: int = priorities[j]
                if best == (0 - 1) or pj > best_pri:
                    best = j
                    best_pri = pj
            j = j + 1
        if best >= 0:
            order.append(best)
            done[best] = 1
            scheduled = scheduled + 1
    return order

def shortest_job_first(tasks: list[int]) -> list[int]:
    n: int = len(tasks)
    indices: list[int] = []
    i: int = 0
    while i < n:
        indices.append(i)
        i = i + 1
    j: int = 0
    while j < n - 1:
        k: int = j + 1
        while k < n:
            ij: int = indices[j]
            ik: int = indices[k]
            tj: int = tasks[ij]
            tk: int = tasks[ik]
            if tj > tk:
                indices[j] = ik
                indices[k] = ij
            k = k + 1
        j = j + 1
    return indices

def avg_wait_time(tasks: list[int], order: list[int]) -> float:
    n: int = len(order)
    total_wait: float = 0.0
    current_time: float = 0.0
    i: int = 0
    while i < n:
        idx: int = order[i]
        total_wait = total_wait + current_time
        current_time = current_time + tasks[idx] * 1.0
        i = i + 1
    return total_wait / (n * 1.0)

def test_module() -> int:
    passed: int = 0
    t: list[int] = [3, 1, 2]
    rr: list[int] = round_robin(t, 2)
    nr: int = len(rr)
    if nr >= 3:
        passed = passed + 1
    ps: list[int] = priority_schedule(t, [1, 3, 2])
    ps0: int = ps[0]
    if ps0 == 1:
        passed = passed + 1
    sjf: list[int] = shortest_job_first(t)
    sjf0: int = sjf[0]
    if sjf0 == 1:
        passed = passed + 1
    awt: float = avg_wait_time(t, sjf)
    if awt >= 0.0:
        passed = passed + 1
    rr0: int = rr[0]
    if rr0 == 0:
        passed = passed + 1
    return passed
