from typing import List, Tuple

def fingerprint(key: int) -> int:
    return ((key * 0x5BD1E995) >> 24) & 0xFF

def bucket1(key: int, size: int) -> int:
    return key % size

def bucket2(key: int, fp: int, size: int) -> int:
    return (key ^ (fp * 0x5BD1E995)) % size

def cf_insert(table: List[int], key: int, size: int) -> Tuple[List[int], bool]:
    fp: int = fingerprint(key)
    b1: int = bucket1(key, size)
    b2: int = bucket2(key, fp, size)
    r: List[int] = []
    for t in table:
        r.append(t)
    if r[b1] == 0:
        r[b1] = fp
        return (r, True)
    if r[b2] == 0:
        r[b2] = fp
        return (r, True)
    return (r, False)

def cf_lookup(table: List[int], key: int, size: int) -> bool:
    fp: int = fingerprint(key)
    b1: int = bucket1(key, size)
    b2: int = bucket2(key, fp, size)
    return table[b1] == fp or table[b2] == fp

def cf_delete(table: List[int], key: int, size: int) -> List[int]:
    fp: int = fingerprint(key)
    b1: int = bucket1(key, size)
    r: List[int] = []
    for t in table:
        r.append(t)
    if r[b1] == fp:
        r[b1] = 0
    return r
