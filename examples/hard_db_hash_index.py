from typing import List, Tuple

def hash_key(key: int, num_buckets: int) -> int:
    h: int = ((key * 0x5BD1E995) ^ (key >> 13)) & 0xFFFFFFFF
    return h % num_buckets

def create_index(num_buckets: int) -> List[List[int]]:
    buckets: List[List[int]] = []
    for i in range(num_buckets):
        buckets.append([])
    return buckets

def index_insert(buckets: List[List[int]], key: int, value: int) -> List[List[int]]:
    idx: int = hash_key(key, len(buckets))
    new_buckets: List[List[int]] = []
    for i in range(len(buckets)):
        nb: List[int] = []
        for v in buckets[i]:
            nb.append(v)
        if i == idx:
            nb.append(key)
            nb.append(value)
        new_buckets.append(nb)
    return new_buckets

def index_lookup(buckets: List[List[int]], key: int) -> int:
    idx: int = hash_key(key, len(buckets))
    bucket: List[int] = buckets[idx]
    i: int = 0
    while i + 1 < len(bucket):
        if bucket[i] == key:
            return bucket[i + 1]
        i = i + 2
    return -1

def load_factor(buckets: List[List[int]]) -> float:
    total: int = 0
    for b in buckets:
        total = total + len(b) // 2
    return float(total) / float(len(buckets))

def rehash(buckets: List[List[int]], new_size: int) -> List[List[int]]:
    new_buckets: List[List[int]] = create_index(new_size)
    for b in buckets:
        i: int = 0
        while i + 1 < len(b):
            new_buckets = index_insert(new_buckets, b[i], b[i + 1])
            i = i + 2
    return new_buckets
