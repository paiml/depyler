from typing import List, Tuple

def avl_height(heights: List[int], idx: int) -> int:
    if idx < 0 or idx >= len(heights):
        return 0
    return heights[idx]

def avl_balance(heights: List[int], left: int, right: int) -> int:
    return avl_height(heights, left) - avl_height(heights, right)

def avl_update_height(heights: List[int], idx: int, left: int, right: int) -> List[int]:
    r: List[int] = []
    for h in heights:
        r.append(h)
    lh: int = avl_height(heights, left)
    rh: int = avl_height(heights, right)
    if lh > rh:
        r[idx] = lh + 1
    else:
        r[idx] = rh + 1
    return r

def avl_search(keys: List[int], lefts: List[int], rights: List[int], root: int, target: int) -> int:
    current: int = root
    while current >= 0 and current < len(keys):
        if target == keys[current]:
            return current
        elif target < keys[current]:
            current = lefts[current]
        else:
            current = rights[current]
    return -1

def avl_is_balanced(heights: List[int], lefts: List[int], rights: List[int], root: int) -> bool:
    if root < 0 or root >= len(heights):
        return True
    bf: int = avl_balance(heights, lefts[root], rights[root])
    if bf < -1 or bf > 1:
        return False
    return avl_is_balanced(heights, lefts, rights, lefts[root]) and avl_is_balanced(heights, lefts, rights, rights[root])

def avl_inorder(keys: List[int], lefts: List[int], rights: List[int], root: int) -> List[int]:
    if root < 0 or root >= len(keys):
        return []
    result: List[int] = []
    left_vals: List[int] = avl_inorder(keys, lefts, rights, lefts[root])
    for v in left_vals:
        result.append(v)
    result.append(keys[root])
    right_vals: List[int] = avl_inorder(keys, lefts, rights, rights[root])
    for v in right_vals:
        result.append(v)
    return result
