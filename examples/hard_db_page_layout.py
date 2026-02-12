from typing import List, Tuple

def create_page(size: int) -> List[int]:
    return [0] * size

def write_tuple(page: List[int], offset: int, data: List[int]) -> List[int]:
    result: List[int] = []
    for p in page:
        result.append(p)
    for i in range(len(data)):
        if offset + i < len(result):
            result[offset + i] = data[i]
    return result

def read_tuple(page: List[int], offset: int, length: int) -> List[int]:
    result: List[int] = []
    for i in range(length):
        if offset + i < len(page):
            result.append(page[offset + i])
    return result

def free_space(page: List[int]) -> int:
    count: int = 0
    for p in page:
        if p == 0:
            count = count + 1
    return count

def compact_page(page: List[int]) -> List[int]:
    data: List[int] = []
    empty: List[int] = []
    for p in page:
        if p != 0:
            data.append(p)
        else:
            empty.append(0)
    result: List[int] = []
    for d in data:
        result.append(d)
    for e in empty:
        result.append(e)
    return result
