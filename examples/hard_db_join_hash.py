from typing import List, Tuple

def hash_join_build(table: List[Tuple[int, int]], num_buckets: int) -> List[List[Tuple[int, int]]]:
    buckets: List[List[Tuple[int, int]]] = []
    for i in range(num_buckets):
        buckets.append([])
    for row in table:
        idx: int = row[0] % num_buckets
        buckets[idx].append(row)
    return buckets

def hash_join_probe(build_ht: List[List[Tuple[int, int]]], probe: List[Tuple[int, int]], num_buckets: int) -> List[Tuple[int, int, int, int]]:
    result: List[Tuple[int, int, int, int]] = []
    for p in probe:
        idx: int = p[1] % num_buckets
        for b in build_ht[idx]:
            if b[0] == p[1]:
                result.append((p[0], p[1], b[0], b[1]))
    return result

def hash_join(left: List[Tuple[int, int]], right: List[Tuple[int, int]], num_buckets: int) -> List[Tuple[int, int, int, int]]:
    ht: List[List[Tuple[int, int]]] = hash_join_build(left, num_buckets)
    return hash_join_probe(ht, right, num_buckets)

def partition(table: List[Tuple[int, int]], num_parts: int) -> List[List[Tuple[int, int]]]:
    parts: List[List[Tuple[int, int]]] = []
    for i in range(num_parts):
        parts.append([])
    for row in table:
        idx: int = row[0] % num_parts
        parts[idx].append(row)
    return parts

def optimal_buckets(rows: int) -> int:
    b: int = rows // 10
    if b < 1:
        b = 1
    return b
