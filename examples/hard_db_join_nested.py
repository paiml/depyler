from typing import List, Tuple

def nested_loop_join(left: List[Tuple[int, int]], right: List[Tuple[int, int]]) -> List[Tuple[int, int, int, int]]:
    result: List[Tuple[int, int, int, int]] = []
    for l in left:
        for r in right:
            if l[1] == r[0]:
                result.append((l[0], l[1], r[0], r[1]))
    return result

def block_nested_join(left: List[Tuple[int, int]], right: List[Tuple[int, int]], block_size: int) -> List[Tuple[int, int, int, int]]:
    result: List[Tuple[int, int, int, int]] = []
    i: int = 0
    while i < len(left):
        end: int = i + block_size
        if end > len(left):
            end = len(left)
        for j in range(i, end):
            for r in right:
                if left[j][1] == r[0]:
                    result.append((left[j][0], left[j][1], r[0], r[1]))
        i = end
    return result

def semi_join(left: List[Tuple[int, int]], right: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    right_keys: List[int] = []
    for r in right:
        right_keys.append(r[0])
    for l in left:
        for rk in right_keys:
            if l[1] == rk:
                result.append(l)
                break
    return result

def anti_join(left: List[Tuple[int, int]], right: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for l in left:
        found: bool = False
        for r in right:
            if l[1] == r[0]:
                found = True
        if not found:
            result.append(l)
    return result

def join_cost(left_size: int, right_size: int) -> int:
    return left_size * right_size
