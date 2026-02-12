from typing import List, Tuple

def bplus_leaf_search(keys: List[int], values: List[int], target: int) -> int:
    for i in range(len(keys)):
        if keys[i] == target:
            return values[i]
    return -1

def bplus_find_child(keys: List[int], target: int) -> int:
    i: int = 0
    while i < len(keys) and target >= keys[i]:
        i = i + 1
    return i

def bplus_leaf_insert(keys: List[int], values: List[int], key: int, value: int) -> Tuple[List[int], List[int]]:
    nk: List[int] = []
    nv: List[int] = []
    inserted: bool = False
    for i in range(len(keys)):
        if not inserted and key <= keys[i]:
            nk.append(key)
            nv.append(value)
            inserted = True
        nk.append(keys[i])
        nv.append(values[i])
    if not inserted:
        nk.append(key)
        nv.append(value)
    return (nk, nv)

def bplus_split_leaf(keys: List[int], values: List[int]) -> Tuple[List[int], List[int], List[int], List[int], int]:
    mid: int = len(keys) // 2
    lk: List[int] = keys[0:mid]
    lv: List[int] = values[0:mid]
    rk: List[int] = keys[mid:]
    rv: List[int] = values[mid:]
    return (lk, lv, rk, rv, rk[0])

def bplus_range_scan(keys: List[int], values: List[int], low: int, high: int) -> List[int]:
    result: List[int] = []
    for i in range(len(keys)):
        if keys[i] >= low and keys[i] <= high:
            result.append(values[i])
    return result
