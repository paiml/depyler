from typing import List, Tuple

def btree_search(keys: List[int], children: List[int], target: int) -> int:
    i: int = 0
    while i < len(keys) and target > keys[i]:
        i = i + 1
    if i < len(keys) and keys[i] == target:
        return i
    if i < len(children):
        return children[i]
    return -1

def btree_insert_sorted(keys: List[int], value: int) -> List[int]:
    result: List[int] = []
    inserted: bool = False
    for k in keys:
        if not inserted and value <= k:
            result.append(value)
            inserted = True
        result.append(k)
    if not inserted:
        result.append(value)
    return result

def btree_split(keys: List[int], order: int) -> Tuple[List[int], int, List[int]]:
    mid: int = len(keys) // 2
    left: List[int] = []
    right: List[int] = []
    for i in range(mid):
        left.append(keys[i])
    for i in range(mid + 1, len(keys)):
        right.append(keys[i])
    return (left, keys[mid], right)

def btree_merge(a: List[int], b: List[int], sep: int) -> List[int]:
    result: List[int] = []
    for v in a:
        result.append(v)
    result.append(sep)
    for v in b:
        result.append(v)
    return result

def btree_height(num_keys: int, order: int) -> int:
    if num_keys <= 0 or order <= 1:
        return 0
    height: int = 0
    capacity: int = 1
    while capacity < num_keys:
        capacity = capacity * order
        height = height + 1
    return height
