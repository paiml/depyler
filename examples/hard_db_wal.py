from typing import List, Tuple

def create_entry(txn_id: int, op: int, rec_id: int, value: int) -> Tuple[int, int, int, int]:
    return (txn_id, op, rec_id, value)

def append_log(log: List[int], txn_id: int, op: int, rec_id: int, value: int) -> List[int]:
    result: List[int] = []
    for e in log:
        result.append(e)
    result.append(txn_id)
    result.append(op)
    result.append(rec_id)
    result.append(value)
    return result

def replay_log(log: List[int]) -> List[Tuple[int, int]]:
    state: List[Tuple[int, int]] = []
    num_entries: int = len(log) // 4
    for e in range(num_entries):
        op: int = log[e * 4 + 1]
        rec_id: int = 0
        rec_id = log[e * 4 + 2]
        val: int = 0
        val = log[e * 4 + 3]
        if op == 1:
            new_state: List[Tuple[int, int]] = []
            found: bool = False
            for s in state:
                s0: int = s[0]
                if s0 == rec_id:
                    new_state.append((rec_id, val))
                    found = True
                else:
                    new_state.append(s)
            if not found:
                new_state.append((rec_id, val))
            state = new_state
    return state

def checkpoint(log: List[int], min_txn_id: int) -> List[int]:
    result: List[int] = []
    num_entries: int = len(log) // 4
    for e in range(num_entries):
        if log[e * 4] >= min_txn_id:
            result.append(log[e * 4])
            result.append(log[e * 4 + 1])
            result.append(log[e * 4 + 2])
            result.append(log[e * 4 + 3])
    return result

def log_size(log: List[int]) -> int:
    return len(log) // 4
