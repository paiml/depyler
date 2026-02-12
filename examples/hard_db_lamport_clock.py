from typing import List, Tuple

def lc_create() -> int:
    return 0

def lc_tick(clock: int) -> int:
    return clock + 1

def lc_send(clock: int) -> Tuple[int, int]:
    new_clock: int = clock + 1
    return (new_clock, new_clock)

def lc_receive(local: int, msg_ts: int) -> int:
    if local > msg_ts:
        return local + 1
    return msg_ts + 1

def lc_order_events(timestamps: List[int]) -> List[int]:
    indices: List[int] = []
    for i in range(len(timestamps)):
        indices.append(i)
    n: int = len(indices)
    for i in range(n):
        for j in range(i + 1, n):
            if timestamps[indices[j]] < timestamps[indices[i]]:
                temp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = temp
    return indices

def simulate_events(num_nodes: int, num_events: int) -> List[int]:
    clocks: List[int] = [0] * num_nodes
    events: List[int] = []
    for i in range(num_events):
        node: int = i % num_nodes
        clocks[node] = lc_tick(clocks[node])
        events.append(clocks[node])
    return events

def max_clock(clocks: List[int]) -> int:
    m: int = 0
    for c in clocks:
        if c > m:
            m = c
    return m
