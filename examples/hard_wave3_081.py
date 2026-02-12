"""Systems: Round-robin scheduler simulation.
Tests: queue management, time slice allocation, process cycling.
"""
from typing import Dict, List, Tuple

def round_robin_schedule(bursts: List[int], quantum: int) -> List[int]:
    """Simulate round-robin scheduling, return completion times."""
    n: int = len(bursts)
    remaining: List[int] = []
    completion: List[int] = []
    for b in bursts:
        remaining.append(b)
        completion.append(0)
    time: int = 0
    done: int = 0
    while done < n:
        i: int = 0
        while i < n:
            if remaining[i] > 0:
                if remaining[i] <= quantum:
                    time = time + remaining[i]
                    remaining[i] = 0
                    completion[i] = time
                    done += 1
                else:
                    time = time + quantum
                    remaining[i] = remaining[i] - quantum
            i += 1
    return completion

def average_turnaround(bursts: List[int], quantum: int) -> float:
    completions: List[int] = round_robin_schedule(bursts, quantum)
    total: int = 0
    for c in completions:
        total = total + c
    return float(total) / float(len(completions))

def fcfs_schedule(bursts: List[int]) -> List[int]:
    """First-come first-served scheduling."""
    completion: List[int] = []
    time: int = 0
    for b in bursts:
        time = time + b
        completion.append(time)
    return completion

def sjf_schedule(bursts: List[int]) -> List[int]:
    """Shortest job first (non-preemptive) scheduling."""
    n: int = len(bursts)
    indices: List[int] = []
    i: int = 0
    while i < n:
        indices.append(i)
        i += 1
    j: int = 0
    while j < n:
        k: int = 0
        lim: int = n - j - 1
        while k < lim:
            if bursts[indices[k]] > bursts[indices[k + 1]]:
                temp: int = indices[k]
                indices[k] = indices[k + 1]
                indices[k + 1] = temp
            k += 1
        j += 1
    completion: List[int] = []
    i = 0
    while i < n:
        completion.append(0)
        i += 1
    time: int = 0
    for idx in indices:
        time = time + bursts[idx]
        completion[idx] = time
    return completion

def test_scheduler() -> bool:
    ok: bool = True
    rr: List[int] = round_robin_schedule([10, 5, 8], 3)
    if len(rr) != 3:
        ok = False
    fcfs: List[int] = fcfs_schedule([3, 5, 2])
    if fcfs[2] != 10:
        ok = False
    return ok
