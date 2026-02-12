from typing import List, Tuple

def chord_hash(key: int, m: int) -> int:
    return ((key * 0x5BD1E995) ^ (key >> 13)) % (1 << m)

def in_range(val: int, start: int, end: int, ring_size: int) -> bool:
    if start < end:
        return val > start and val <= end
    return val > start or val <= end

def find_successor(finger_table: List[int], node_id: int, key: int, ring_size: int) -> int:
    for f in finger_table:
        if in_range(key, node_id, f, ring_size):
            return f
    if len(finger_table) > 0:
        return finger_table[0]
    return node_id

def build_finger_table(node_id: int, nodes: List[int], m: int) -> List[int]:
    ring_size: int = 1 << m
    table: List[int] = []
    for i in range(m):
        start: int = (node_id + (1 << i)) % ring_size
        best: int = nodes[0]
        for n in nodes:
            if in_range(n, start - 1, start + ring_size // 2, ring_size):
                best = n
                break
        table.append(best)
    return table

def chord_lookup(tables: List[List[int]], node_ids: List[int], start: int, key: int, ring_size: int) -> int:
    current: int = start
    for step in range(len(node_ids)):
        idx: int = -1
        for i in range(len(node_ids)):
            if node_ids[i] == current:
                idx = i
        if idx < 0:
            return current
        succ: int = find_successor(tables[idx], current, key, ring_size)
        if succ == current:
            return current
        current = succ
    return current
