from typing import List, Tuple

def intersect(doms: List[int], b1: int, b2: int) -> int:
    finger1: int = b1
    finger2: int = b2
    while finger1 != finger2:
        while finger1 < finger2:
            finger1 = doms[finger1]
        while finger2 < finger1:
            finger2 = doms[finger2]
    return finger1

def compute_dominators(preds: List[List[int]], n: int) -> List[int]:
    doms: List[int] = [-1] * n
    doms[0] = 0
    changed: bool = True
    while changed:
        changed = False
        for b in range(1, n):
            new_idom: int = -1
            for p in preds[b]:
                if doms[p] != -1:
                    if new_idom == -1:
                        new_idom = p
                    else:
                        new_idom = intersect(doms, new_idom, p)
            if new_idom != doms[b]:
                doms[b] = new_idom
                changed = True
    return doms

def dominance_frontier(doms: List[int], preds: List[List[int]], n: int) -> List[List[int]]:
    df: List[List[int]] = []
    for i in range(n):
        df.append([])
    for b in range(n):
        if len(preds[b]) >= 2:
            for p in preds[b]:
                runner: int = p
                while runner != doms[b]:
                    found: bool = False
                    for d in df[runner]:
                        if d == b:
                            found = True
                    if not found:
                        df[runner].append(b)
                    runner = doms[runner]
    return df

def is_dominator(doms: List[int], a: int, b: int) -> bool:
    current: int = b
    while current != 0:
        if current == a:
            return True
        current = doms[current]
    return a == 0

def dom_tree_depth(doms: List[int], node: int) -> int:
    depth: int = 0
    current: int = node
    while current != 0:
        current = doms[current]
        depth = depth + 1
    return depth
