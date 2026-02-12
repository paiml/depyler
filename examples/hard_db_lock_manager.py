from typing import List, Tuple

def acquire_lock(locks: List[Tuple[int, int]], resource: int, txn_id: int) -> Tuple[List[Tuple[int, int]], bool]:
    for lock in locks:
        if lock[0] == resource:
            if lock[1] == txn_id:
                result: List[Tuple[int, int]] = []
                for l in locks:
                    result.append(l)
                return (result, True)
            result2: List[Tuple[int, int]] = []
            for l in locks:
                result2.append(l)
            return (result2, False)
    result3: List[Tuple[int, int]] = []
    for lock in locks:
        result3.append(lock)
    result3.append((resource, txn_id))
    return (result3, True)

def release_lock(locks: List[Tuple[int, int]], resource: int, txn_id: int) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for lock in locks:
        if lock[0] == resource and lock[1] == txn_id:
            continue
        result.append(lock)
    return result

def release_all(locks: List[Tuple[int, int]], txn_id: int) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for lock in locks:
        if lock[1] != txn_id:
            result.append(lock)
    return result

def has_lock(locks: List[Tuple[int, int]], resource: int, txn_id: int) -> bool:
    for lock in locks:
        if lock[0] == resource and lock[1] == txn_id:
            return True
    return False

def count_locks(locks: List[Tuple[int, int]], txn_id: int) -> int:
    count: int = 0
    for lock in locks:
        if lock[1] == txn_id:
            count = count + 1
    return count
