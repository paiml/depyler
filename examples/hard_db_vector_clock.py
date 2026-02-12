from typing import List, Tuple

def vc_create(n: int) -> List[int]:
    return [0] * n

def vc_increment(clock: List[int], node_id: int) -> List[int]:
    result: List[int] = []
    for c in clock:
        result.append(c)
    result[node_id] = result[node_id] + 1
    return result

def vc_merge(a: List[int], b: List[int]) -> List[int]:
    result: List[int] = []
    for i in range(len(a)):
        if a[i] > b[i]:
            result.append(a[i])
        else:
            result.append(b[i])
    return result

def vc_happens_before(a: List[int], b: List[int]) -> bool:
    at_least_one_less: bool = False
    for i in range(len(a)):
        if a[i] > b[i]:
            return False
        if a[i] < b[i]:
            at_least_one_less = True
    return at_least_one_less

def vc_concurrent(a: List[int], b: List[int]) -> bool:
    return not vc_happens_before(a, b) and not vc_happens_before(b, a) and a != b

def vc_send(clock: List[int], node_id: int) -> List[int]:
    return vc_increment(clock, node_id)

def vc_receive(local: List[int], remote: List[int], node_id: int) -> List[int]:
    merged: List[int] = vc_merge(local, remote)
    return vc_increment(merged, node_id)
