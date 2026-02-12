from typing import List, Tuple

def sort_by_key(table: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for t in table:
        result.append(t)
    n: int = len(result)
    for i in range(n):
        for j in range(i + 1, n):
            if result[j][0] < result[i][0]:
                temp: Tuple[int, int] = result[i]
                result[i] = result[j]
                result[j] = temp
    return result

def merge_join(left: List[Tuple[int, int]], right: List[Tuple[int, int]]) -> List[Tuple[int, int, int, int]]:
    sl: List[Tuple[int, int]] = sort_by_key(left)
    sr: List[Tuple[int, int]] = sort_by_key(right)
    result: List[Tuple[int, int, int, int]] = []
    i: int = 0
    j: int = 0
    while i < len(sl) and j < len(sr):
        if sl[i][0] < sr[j][0]:
            i = i + 1
        elif sl[i][0] > sr[j][0]:
            j = j + 1
        else:
            k: int = j
            while k < len(sr) and sr[k][0] == sl[i][0]:
                result.append((sl[i][0], sl[i][1], sr[k][0], sr[k][1]))
                k = k + 1
            i = i + 1
    return result

def is_sorted(table: List[Tuple[int, int]]) -> bool:
    for i in range(1, len(table)):
        if table[i][0] < table[i - 1][0]:
            return False
    return True

def merge_cost(left_size: int, right_size: int) -> float:
    return float(left_size) + float(right_size)

def output_size_estimate(left_size: int, right_size: int, selectivity: float) -> int:
    return int(float(left_size) * float(right_size) * selectivity)
