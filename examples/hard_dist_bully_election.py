from typing import List, Tuple

def bully_election_init(n: int) -> List[int]:
    state: List[int] = []
    for i in range(n):
        state.append(0)
    return state

def bully_election_step(state: List[int], node: int, value: int) -> List[int]:
    result: List[int] = []
    for s in state:
        result.append(s)
    if node < len(result):
        result[node] = value
    return result

def bully_election_check(state: List[int], condition: int) -> bool:
    for s in state:
        if s == condition:
            return True
    return False

def bully_election_aggregate(state: List[int]) -> int:
    total: int = 0
    for s in state:
        total = total + s
    return total

def bully_election_broadcast(state: List[int], value: int) -> List[int]:
    result: List[int] = []
    for s in state:
        result.append(value)
    return result

def bully_election_elect(state: List[int]) -> int:
    best: int = 0
    for i in range(1, len(state)):
        if state[i] > state[best]:
            best = i
    return best

def bully_election_converged(state: List[int]) -> bool:
    if len(state) == 0:
        return True
    first: int = state[0]
    for s in state:
        if s != first:
            return False
    return True
