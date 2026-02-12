from typing import List, Tuple

def create_version(key: int, value: int, txn_id: int) -> Tuple[int, int, int, int]:
    return (key, value, txn_id, 0)

def add_version(versions: List[Tuple[int, int, int, int]], key: int, value: int, txn_id: int) -> List[Tuple[int, int, int, int]]:
    result: List[Tuple[int, int, int, int]] = []
    for v in versions:
        if v[0] == key and v[3] == 0:
            result.append((v[0], v[1], v[2], txn_id))
        else:
            result.append(v)
    result.append(create_version(key, value, txn_id))
    return result

def read_version(versions: List[Tuple[int, int, int, int]], key: int, txn_id: int) -> int:
    best: int = -1
    best_txn: int = -1
    for v in versions:
        if v[0] == key and v[2] <= txn_id and (v[3] == 0 or v[3] > txn_id):
            if v[2] > best_txn:
                best = v[1]
                best_txn = v[2]
    return best

def gc_versions(versions: List[Tuple[int, int, int, int]], oldest_txn: int) -> List[Tuple[int, int, int, int]]:
    result: List[Tuple[int, int, int, int]] = []
    for v in versions:
        if v[3] == 0 or v[3] >= oldest_txn:
            result.append(v)
    return result

def count_versions(versions: List[Tuple[int, int, int, int]], key: int) -> int:
    count: int = 0
    for v in versions:
        if v[0] == key:
            count = count + 1
    return count
