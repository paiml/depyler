from typing import List, Tuple

def insert_tuple(store: List[int], row_width: int, data: List[int]) -> List[int]:
    result: List[int] = []
    for s in store:
        result.append(s)
    for i in range(row_width):
        if i < len(data):
            result.append(data[i])
        else:
            result.append(0)
    return result

def delete_at(store: List[int], row_width: int, idx: int) -> List[int]:
    result: List[int] = []
    num_rows: int = len(store) // row_width
    for i in range(num_rows):
        if i != idx:
            for j in range(row_width):
                result.append(store[i * row_width + j])
    return result

def scan_col(store: List[int], row_width: int, col: int, value: int) -> List[int]:
    result: List[int] = []
    num_rows: int = len(store) // row_width
    for i in range(num_rows):
        if col < row_width and store[i * row_width + col] == value:
            for j in range(row_width):
                result.append(store[i * row_width + j])
    return result

def project_col(store: List[int], row_width: int, col: int) -> List[int]:
    result: List[int] = []
    num_rows: int = len(store) // row_width
    for i in range(num_rows):
        if col < row_width:
            result.append(store[i * row_width + col])
    return result

def store_size(store: List[int], row_width: int) -> int:
    if row_width == 0:
        return 0
    return len(store) // row_width
