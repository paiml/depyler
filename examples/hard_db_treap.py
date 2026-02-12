from typing import List, Tuple

def treap_search(keys: List[int], lefts: List[int], rights: List[int], root: int, target: int) -> int:
    current: int = root
    while current >= 0 and current < len(keys):
        if target == keys[current]:
            return current
        elif target < keys[current]:
            current = lefts[current]
        else:
            current = rights[current]
    return -1

def treap_priority(seed: int, key: int) -> int:
    h: int = (seed ^ (key * 0x5BD1E995)) & 0xFFFFFFFF
    h = ((h >> 16) ^ h) * 0x45D9F3B
    return h & 0xFFFFFFFF

def treap_inorder(keys: List[int], lefts: List[int], rights: List[int], root: int) -> List[int]:
    if root < 0 or root >= len(keys):
        return []
    result: List[int] = []
    stack: List[int] = []
    current: int = root
    while current >= 0 or len(stack) > 0:
        while current >= 0 and current < len(keys):
            stack.append(current)
            current = lefts[current]
        if len(stack) > 0:
            current = stack[len(stack) - 1]
            stack = stack[0:len(stack) - 1]
            result.append(keys[current])
            current = rights[current]
        else:
            current = -1
    return result

def treap_size(keys: List[int], lefts: List[int], rights: List[int], root: int) -> int:
    if root < 0 or root >= len(keys):
        return 0
    count: int = 0
    stack: List[int] = [root]
    while len(stack) > 0:
        n: int = stack[len(stack) - 1]
        stack = stack[0:len(stack) - 1]
        if n >= 0 and n < len(keys):
            count = count + 1
            if lefts[n] >= 0:
                stack.append(lefts[n])
            if rights[n] >= 0:
                stack.append(rights[n])
    return count

def treap_depth(keys: List[int], lefts: List[int], rights: List[int], root: int) -> int:
    if root < 0 or root >= len(keys):
        return 0
    max_d: int = 0
    stack: List[Tuple[int, int]] = [(root, 1)]
    while len(stack) > 0:
        item: Tuple[int, int] = stack[len(stack) - 1]
        stack = stack[0:len(stack) - 1]
        n: int = item[0]
        d: int = item[1]
        if d > max_d:
            max_d = d
        if n >= 0 and n < len(keys):
            if lefts[n] >= 0:
                stack.append((lefts[n], d + 1))
            if rights[n] >= 0:
                stack.append((rights[n], d + 1))
    return max_d
