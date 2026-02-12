from typing import List, Tuple

def detect_loop(adj: List[int], offsets: List[int], num_nodes: int, entry: int) -> List[int]:
    visited: List[int] = [0] * num_nodes
    result: List[int] = []
    stack: List[int] = [entry]
    while len(stack) > 0:
        node: int = stack[len(stack) - 1]
        stack = stack[0:len(stack) - 1]
        if node >= 0 and node < num_nodes and visited[node] == 0:
            visited[node] = 1
            result.append(node)
            start: int = offsets[node]
            end: int = offsets[node + 1]
            i: int = start
            while i < end:
                stack.append(adj[i])
                i = i + 1
    return result

def is_invariant(instr: Tuple[int, int, int], loop_defs: List[int]) -> bool:
    for d in loop_defs:
        if d == instr[2]:
            return False
    return True

def hoist_invariants(instrs: List[Tuple[int, int, int]], loop_start: int, loop_end: int) -> List[Tuple[int, int, int]]:
    loop_defs: List[int] = []
    for i in range(loop_start, loop_end):
        loop_defs.append(instrs[i][1])
    hoisted: List[Tuple[int, int, int]] = []
    remaining: List[Tuple[int, int, int]] = []
    for i in range(len(instrs)):
        if i >= loop_start and i < loop_end:
            is_inv: bool = True
            for d in loop_defs:
                if d == instrs[i][2]:
                    is_inv = False
            if is_inv:
                hoisted.append(instrs[i])
            else:
                remaining.append(instrs[i])
        else:
            remaining.append(instrs[i])
    result: List[Tuple[int, int, int]] = []
    for h in hoisted:
        result.append(h)
    for r in remaining:
        result.append(r)
    return result

def unroll(body: List[int], factor: int) -> List[int]:
    result: List[int] = []
    for i in range(factor):
        for b in body:
            result.append(b)
    return result

def trip_count(start: int, end: int, step: int) -> int:
    if step <= 0:
        return 0
    return (end - start + step - 1) // step
