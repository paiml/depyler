from typing import List, Tuple

def gc_create(n: int) -> List[int]:
    return [0] * n

def gc_increment(counter: List[int], node_id: int) -> List[int]:
    result: List[int] = []
    for c in counter:
        result.append(c)
    result[node_id] = result[node_id] + 1
    return result

def gc_value(counter: List[int]) -> int:
    total: int = 0
    for c in counter:
        total = total + c
    return total

def gc_merge(a: List[int], b: List[int]) -> List[int]:
    result: List[int] = []
    for i in range(len(a)):
        if a[i] > b[i]:
            result.append(a[i])
        else:
            result.append(b[i])
    return result

def pn_value(pos: List[int], neg: List[int]) -> int:
    p: int = 0
    for v in pos:
        p = p + v
    n: int = 0
    for v in neg:
        n = n + v
    return p - n
