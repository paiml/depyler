from typing import List, Tuple
import math

def hll_hash(key: int) -> int:
    h: int = key * 0x5BD1E995
    h = h ^ (h >> 13)
    h = (h * 0xC2B2AE35) & 0xFFFFFFFF
    return h

def leading_zeros(h: int) -> int:
    if h == 0:
        return 32
    count: int = 0
    while (h & 0x80000000) == 0:
        count = count + 1
        h = (h << 1) & 0xFFFFFFFF
    return count

def hll_add(registers: List[int], key: int, p: int) -> List[int]:
    h: int = hll_hash(key)
    idx: int = h & ((1 << p) - 1)
    w: int = h >> p
    lz: int = leading_zeros(w) + 1
    r: List[int] = []
    for v in registers:
        r.append(v)
    if lz > r[idx]:
        r[idx] = lz
    return r

def hll_count(registers: List[int]) -> float:
    m: int = len(registers)
    alpha: float = 0.7213 / (1.0 + 1.079 / float(m))
    sm: float = 0.0
    for v in registers:
        sm = sm + math.pow(2.0, float(0 - v))
    return alpha * float(m * m) / sm

def hll_merge(a: List[int], b: List[int]) -> List[int]:
    r: List[int] = []
    for i in range(len(a)):
        if a[i] > b[i]:
            r.append(a[i])
        else:
            r.append(b[i])
    return r
