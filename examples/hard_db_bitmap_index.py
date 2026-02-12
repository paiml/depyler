from typing import List, Tuple

def create_bitmap(size: int) -> List[int]:
    return [0] * size

def set_bit(bitmap: List[int], pos: int) -> List[int]:
    result: List[int] = []
    for b in bitmap:
        result.append(b)
    if pos < len(result):
        result[pos] = 1
    return result

def clear_bit(bitmap: List[int], pos: int) -> List[int]:
    result: List[int] = []
    for b in bitmap:
        result.append(b)
    if pos < len(result):
        result[pos] = 0
    return result

def bitmap_and(a: List[int], b: List[int]) -> List[int]:
    result: List[int] = []
    for i in range(len(a)):
        result.append(a[i] & b[i])
    return result

def bitmap_or(a: List[int], b: List[int]) -> List[int]:
    result: List[int] = []
    for i in range(len(a)):
        result.append(a[i] | b[i])
    return result

def bitmap_not(a: List[int]) -> List[int]:
    result: List[int] = []
    for b in a:
        if b == 0:
            result.append(1)
        else:
            result.append(0)
    return result

def count_set(bitmap: List[int]) -> int:
    count: int = 0
    for b in bitmap:
        count = count + b
    return count

def bitmap_positions(bitmap: List[int]) -> List[int]:
    positions: List[int] = []
    for i in range(len(bitmap)):
        if bitmap[i] == 1:
            positions.append(i)
    return positions
