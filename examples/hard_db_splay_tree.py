from typing import List, Tuple

def splay_search(keys: List[int], lefts: List[int], rights: List[int], root: int, target: int) -> int:
    current: int = root
    while current >= 0 and current < len(keys):
        if target == keys[current]:
            return current
        elif target < keys[current]:
            current = lefts[current]
        else:
            current = rights[current]
    return -1

def zig(keys: List[int], lefts: List[int], rights: List[int], x: int) -> Tuple[List[int], List[int]]:
    nl: List[int] = []
    nr: List[int] = []
    for l in lefts:
        nl.append(l)
    for r in rights:
        nr.append(r)
    y: int = nl[x]
    if y >= 0:
        nl[x] = nr[y]
        nr[y] = x
    return (nl, nr)

def zag(keys: List[int], lefts: List[int], rights: List[int], x: int) -> Tuple[List[int], List[int]]:
    nl: List[int] = []
    nr: List[int] = []
    for l in lefts:
        nl.append(l)
    for r in rights:
        nr.append(r)
    y: int = nr[x]
    if y >= 0:
        nr[x] = nl[y]
        nl[y] = x
    return (nl, nr)

def tree_size(lefts: List[int], rights: List[int], root: int) -> int:
    if root < 0 or root >= len(lefts):
        return 0
    return 1 + tree_size(lefts, rights, lefts[root]) + tree_size(lefts, rights, rights[root])

def tree_depth(lefts: List[int], rights: List[int], root: int) -> int:
    if root < 0 or root >= len(lefts):
        return 0
    ld: int = tree_depth(lefts, rights, lefts[root])
    rd: int = tree_depth(lefts, rights, rights[root])
    if ld > rd:
        return ld + 1
    return rd + 1
