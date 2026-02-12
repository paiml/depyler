from typing import List, Tuple

def memtable_insert(table: List[Tuple[int, int]], key: int, value: int) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    inserted: bool = False
    for entry in table:
        if not inserted and key <= entry[0]:
            result.append((key, value))
            inserted = True
        if entry[0] != key:
            result.append(entry)
    if not inserted:
        result.append((key, value))
    return result

def memtable_lookup(table: List[Tuple[int, int]], key: int) -> int:
    for entry in table:
        if entry[0] == key:
            return entry[1]
    return -1

def flush_memtable(table: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
    return table

def merge_sorted(a: List[Tuple[int, int]], b: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    i: int = 0
    j: int = 0
    while i < len(a) and j < len(b):
        if a[i][0] < b[j][0]:
            result.append(a[i])
            i = i + 1
        elif a[i][0] > b[j][0]:
            result.append(b[j])
            j = j + 1
        else:
            result.append(b[j])
            i = i + 1
            j = j + 1
    while i < len(a):
        result.append(a[i])
        i = i + 1
    while j < len(b):
        result.append(b[j])
        j = j + 1
    return result

def compact_levels(levels: List[List[Tuple[int, int]]]) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for level in levels:
        result = merge_sorted(result, level)
    return result
