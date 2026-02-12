from typing import List, Tuple

def build_cfg(instrs: List[Tuple[int, int]]) -> List[int]:
    blocks: List[int] = [0, -1, -1]
    current: int = 0
    for instr in instrs:
        if instr[0] == 1:
            new_id: int = len(blocks) // 3
            blocks.append(new_id)
            blocks.append(-1)
            blocks.append(-1)
            if blocks[current * 3 + 1] == -1:
                blocks[current * 3 + 1] = instr[1]
            else:
                blocks[current * 3 + 2] = new_id
            current = new_id
        elif instr[0] == 2:
            if blocks[current * 3 + 1] == -1:
                blocks[current * 3 + 1] = instr[1]
            new_id2: int = len(blocks) // 3
            blocks.append(new_id2)
            blocks.append(-1)
            blocks.append(-1)
            current = new_id2
    return blocks

def predecessors(blocks: List[int], target: int) -> List[int]:
    preds: List[int] = []
    num_blocks: int = len(blocks) // 3
    for i in range(num_blocks):
        if blocks[i * 3 + 1] == target or blocks[i * 3 + 2] == target:
            preds.append(blocks[i * 3])
    return preds

def is_reachable(blocks: List[int], start: int, target: int) -> bool:
    num_blocks: int = len(blocks) // 3
    visited: List[int] = [0] * num_blocks
    stack: List[int] = [start]
    while len(stack) > 0:
        curr: int = stack[len(stack) - 1]
        stack = stack[0:len(stack) - 1]
        if curr == target:
            return True
        if curr >= 0 and curr < num_blocks and visited[curr] == 0:
            visited[curr] = 1
            s1: int = blocks[curr * 3 + 1]
            s2: int = blocks[curr * 3 + 2]
            if s1 >= 0:
                stack.append(s1)
            if s2 >= 0:
                stack.append(s2)
    return False

def count_blocks(blocks: List[int]) -> int:
    return len(blocks) // 3

def count_edges(blocks: List[int]) -> int:
    edges: int = 0
    num_blocks: int = len(blocks) // 3
    for i in range(num_blocks):
        if blocks[i * 3 + 1] >= 0:
            edges = edges + 1
        if blocks[i * 3 + 2] >= 0:
            edges = edges + 1
    return edges
