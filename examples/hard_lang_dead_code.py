from typing import List, Tuple

def find_used_vars(instrs: List[Tuple[int, int, int]]) -> List[int]:
    used: List[int] = []
    for instr in instrs:
        src: int = instr[2]
        found: bool = False
        for u in used:
            if u == src:
                found = True
        if not found:
            used.append(src)
    return used

def eliminate_dead(instrs: List[Tuple[int, int, int]], used: List[int]) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for instr in instrs:
        dst: int = instr[1]
        is_used: bool = False
        for u in used:
            if u == dst:
                is_used = True
        if is_used or instr[0] >= 10:
            result.append(instr)
    return result

def iterative_dce(instrs: List[Tuple[int, int, int]]) -> List[Tuple[int, int, int]]:
    current: List[Tuple[int, int, int]] = instrs
    changed: bool = True
    while changed:
        used: List[int] = find_used_vars(current)
        new_instrs: List[Tuple[int, int, int]] = eliminate_dead(current, used)
        changed = len(new_instrs) < len(current)
        current = new_instrs
    return current

def unreachable_blocks(succs: List[List[int]], entry: int, total: int) -> List[int]:
    visited: List[int] = [0] * total
    worklist: List[int] = [entry]
    while len(worklist) > 0:
        b: int = worklist[len(worklist) - 1]
        worklist = worklist[0:len(worklist) - 1]
        if visited[b] == 0:
            visited[b] = 1
            for s in succs[b]:
                worklist.append(s)
    unreachable: List[int] = []
    for i in range(total):
        if visited[i] == 0:
            unreachable.append(i)
    return unreachable

def count_eliminated(before: int, after: int) -> int:
    return before - after
