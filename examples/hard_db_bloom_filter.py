from typing import List, Tuple

def bf_create(size: int) -> List[int]:
    return [0] * size

def bf_add(bf: List[int], key: int) -> List[int]:
    r: List[int] = []
    for b in bf:
        r.append(b)
    h1: int = ((key * 2654435761) >> 16) % len(r)
    h2: int = ((key * 0x85EBCA6B) >> 13) % len(r)
    r[h1] = 1
    r[h2] = 1
    return r

def bf_check(bf: List[int], key: int) -> bool:
    h1: int = ((key * 2654435761) >> 16) % len(bf)
    h2: int = ((key * 0x85EBCA6B) >> 13) % len(bf)
    return bf[h1] == 1 and bf[h2] == 1

def bf_fp_rate_scaled(bf: List[int]) -> int:
    s: int = 0
    for b in bf:
        s = s + b
    r_scaled: int = (s * 10000) // len(bf)
    return (r_scaled * r_scaled) // 10000

def bf_union(a: List[int], b: List[int]) -> List[int]:
    r: List[int] = []
    for i in range(len(a)):
        if a[i] == 1 or b[i] == 1:
            r.append(1)
        else:
            r.append(0)
    return r
