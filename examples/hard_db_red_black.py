from typing import List, Tuple

def rb_create_node(key: int, color: int, left: int, right: int, parent: int) -> List[int]:
    return [key, color, left, right, parent]

def rb_is_red(nodes: List[List[int]], idx: int) -> bool:
    if idx < 0 or idx >= len(nodes):
        return False
    return nodes[idx][1] == 0

def rb_rotate_left(nodes: List[List[int]], x: int) -> List[List[int]]:
    result: List[List[int]] = []
    for n in nodes:
        result.append([n[0], n[1], n[2], n[3], n[4]])
    y: int = result[x][3]
    if y >= 0 and y < len(result):
        result[x][3] = result[y][2]
        result[y][2] = x
    return result

def rb_rotate_right(nodes: List[List[int]], x: int) -> List[List[int]]:
    result: List[List[int]] = []
    for n in nodes:
        result.append([n[0], n[1], n[2], n[3], n[4]])
    y: int = result[x][2]
    if y >= 0 and y < len(result):
        result[x][2] = result[y][3]
        result[y][3] = x
    return result

def rb_search(nodes: List[List[int]], root: int, key: int) -> int:
    current: int = root
    while current >= 0 and current < len(nodes):
        if key == nodes[current][0]:
            return current
        elif key < nodes[current][0]:
            current = nodes[current][2]
        else:
            current = nodes[current][3]
    return -1

def rb_black_height(nodes: List[List[int]], root: int) -> int:
    if root < 0 or root >= len(nodes):
        return 0
    height: int = 0
    current: int = root
    while current >= 0 and current < len(nodes):
        if nodes[current][1] == 1:
            height = height + 1
        current = nodes[current][2]
    return height
