from typing import List, Tuple

def round_robin_init(n: int) -> List[int]:
    state: List[int] = []
    for i in range(n):
        state.append(0)
    return state

def round_robin_step(state: List[int], node: int, value: int) -> List[int]:
    result: List[int] = []
    for s in state:
        result.append(s)
    if node < len(result):
        result[node] = value
    return result

def round_robin_check(state: List[int], condition: int) -> bool:
    for s in state:
        if s == condition:
            return True
    return False

def round_robin_aggregate(state: List[int]) -> int:
    total: int = 0
    for s in state:
        total = total + s
    return total

def round_robin_broadcast(state: List[int], value: int) -> List[int]:
    result: List[int] = []
    for s in state:
        result.append(value)
    return result

def round_robin_elect(state: List[int]) -> int:
    best: int = 0
    for i in range(1, len(state)):
        if state[i] > state[best]:
            best = i
    return best

def round_robin_converged(state: List[int]) -> bool:
    if len(state) == 0:
        return True
    first: int = state[0]
    for s in state:
        if s != first:
            return False
    return True
