from typing import List, Tuple

def prepare(participants: List[int], votes: List[int]) -> bool:
    for v in votes:
        if v == 0:
            return False
    return True

def commit_phase(participants: List[int], prepared: bool) -> List[int]:
    result: List[int] = []
    for p in participants:
        if prepared:
            result.append(1)
        else:
            result.append(0)
    return result

def two_phase_commit(participants: List[int], votes: List[int]) -> Tuple[bool, List[int]]:
    can_commit: bool = prepare(participants, votes)
    outcomes: List[int] = commit_phase(participants, can_commit)
    return (can_commit, outcomes)

def log_decision(txn_id: int, decision: bool, participants: List[int]) -> List[int]:
    log: List[int] = [txn_id]
    if decision:
        log.append(1)
    else:
        log.append(0)
    for p in participants:
        log.append(p)
    return log

def recovery(log_entries: List[List[int]]) -> List[Tuple[int, bool]]:
    decisions: List[Tuple[int, bool]] = []
    for entry in log_entries:
        txn_id: int = entry[0]
        committed: bool = entry[1] == 1
        decisions.append((txn_id, committed))
    return decisions
