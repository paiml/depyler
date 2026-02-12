from typing import List, Tuple

def prepare(proposal_num: int, promises: List[int]) -> bool:
    count: int = 0
    for p in promises:
        if p >= proposal_num:
            count = count + 1
    return count > len(promises) // 2

def accept(proposal_num: int, value: int, acceptors: List[int]) -> List[int]:
    result: List[int] = []
    for a in acceptors:
        if a <= proposal_num:
            result.append(1)
        else:
            result.append(0)
    return result

def is_majority(votes: List[int]) -> bool:
    yes: int = 0
    for v in votes:
        if v == 1:
            yes = yes + 1
    return yes > len(votes) // 2

def highest_promise(promises: List[Tuple[int, int]]) -> Tuple[int, int]:
    best_num: int = -1
    best_val: int = -1
    for p in promises:
        if p[0] > best_num:
            best_num = p[0]
            best_val = p[1]
    return (best_num, best_val)

def paxos_round(proposer: int, value: int, acceptor_states: List[int], n: int) -> Tuple[bool, int]:
    votes: List[int] = accept(proposer, value, acceptor_states)
    if is_majority(votes):
        return (True, value)
    return (False, -1)
