from typing import List, Tuple

def hash_node_ch(node_id: int) -> int:
    return ((node_id * 0x5BD1E995) ^ (node_id >> 13)) & 0xFFFFFFFF

def hash_key_ch(key: int) -> int:
    return ((key * 0x85EBCA6B) ^ (key >> 16)) & 0xFFFFFFFF

def build_ring(nodes: List[int], vnodes: int) -> List[Tuple[int, int]]:
    ring: List[Tuple[int, int]] = []
    for node in nodes:
        for v in range(vnodes):
            h: int = hash_node_ch(node * 1000 + v)
            ring.append((h, node))
    n: int = len(ring)
    for i in range(n):
        for j in range(i + 1, n):
            if ring[j][0] < ring[i][0]:
                temp: Tuple[int, int] = ring[i]
                ring[i] = ring[j]
                ring[j] = temp
    return ring

def find_node(ring: List[Tuple[int, int]], key: int) -> int:
    h: int = hash_key_ch(key)
    for entry in ring:
        if entry[0] >= h:
            return entry[1]
    if len(ring) > 0:
        return ring[0][1]
    return -1

def add_node(ring: List[Tuple[int, int]], node: int, vnodes: int) -> List[Tuple[int, int]]:
    new_ring: List[Tuple[int, int]] = []
    for e in ring:
        new_ring.append(e)
    for v in range(vnodes):
        h: int = hash_node_ch(node * 1000 + v)
        new_ring.append((h, node))
    n: int = len(new_ring)
    for i in range(n):
        for j in range(i + 1, n):
            if new_ring[j][0] < new_ring[i][0]:
                temp: Tuple[int, int] = new_ring[i]
                new_ring[i] = new_ring[j]
                new_ring[j] = temp
    return new_ring

def remove_node(ring: List[Tuple[int, int]], node: int) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for e in ring:
        if e[1] != node:
            result.append(e)
    return result
