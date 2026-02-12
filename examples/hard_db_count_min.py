from typing import List, Tuple

def cm_hash(key: int, seed: int, width: int) -> int:
    return ((key * seed) ^ (key >> 16)) % width

def cm_create(depth: int, width: int) -> List[List[int]]:
    table: List[List[int]] = []
    for i in range(depth):
        row: List[int] = [0] * width
        table.append(row)
    return table

def cm_add(table: List[List[int]], key: int, seeds: List[int]) -> List[List[int]]:
    r: List[List[int]] = []
    for row in table:
        nr: List[int] = []
        for v in row:
            nr.append(v)
        r.append(nr)
    for i in range(len(seeds)):
        idx: int = cm_hash(key, seeds[i], len(r[i]))
        r[i][idx] = r[i][idx] + 1
    return r

def cm_query(table: List[List[int]], key: int, seeds: List[int]) -> int:
    min_val: int = 999999999
    for i in range(len(seeds)):
        idx: int = cm_hash(key, seeds[i], len(table[i]))
        if table[i][idx] < min_val:
            min_val = table[i][idx]
    return min_val

def cm_total(table: List[List[int]]) -> int:
    total: int = 0
    if len(table) > 0:
        for v in table[0]:
            total = total + v
    return total
