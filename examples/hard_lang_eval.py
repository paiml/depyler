from typing import List, Tuple, Dict

def eval_literal(value: int) -> int:
    return value

def eval_add(a: int, b: int) -> int:
    return a + b

def eval_sub(a: int, b: int) -> int:
    return a - b

def eval_mul(a: int, b: int) -> int:
    return a * b

def eval_div(a: int, b: int) -> int:
    if b == 0:
        return 0
    return a // b

def eval_program(instrs: List[Tuple[int, int, int]], regs: List[int]) -> List[int]:
    result: List[int] = []
    for r in regs:
        result.append(r)
    for instr in instrs:
        op: int = instr[0]
        a: int = instr[1]
        b: int = instr[2]
        if op == 1:
            result[a] = b
        elif op == 2:
            result[a] = eval_add(result[a], result[b])
        elif op == 3:
            result[a] = eval_sub(result[a], result[b])
        elif op == 4:
            result[a] = eval_mul(result[a], result[b])
        elif op == 5:
            result[a] = eval_div(result[a], result[b])
    return result

def eval_stack(instrs: List[int], values: List[int]) -> List[int]:
    stack: List[int] = []
    vi: int = 0
    for op in instrs:
        if op == 0:
            if vi < len(values):
                stack.append(values[vi])
                vi = vi + 1
        elif op == 1 and len(stack) >= 2:
            b: int = stack[len(stack) - 1]
            a: int = stack[len(stack) - 2]
            stack = stack[0:len(stack) - 2]
            stack.append(a + b)
        elif op == 2 and len(stack) >= 2:
            b2: int = stack[len(stack) - 1]
            a2: int = stack[len(stack) - 2]
            stack = stack[0:len(stack) - 2]
            stack.append(a2 * b2)
    return stack

def trace_eval(instrs: List[Tuple[int, int, int]], regs: List[int]) -> List[List[int]]:
    trace: List[List[int]] = []
    state: List[int] = []
    for r in regs:
        state.append(r)
    for instr in instrs:
        snap: List[int] = []
        for s in state:
            snap.append(s)
        trace.append(snap)
        if instr[0] == 1:
            state[instr[1]] = instr[2]
        elif instr[0] == 2:
            state[instr[1]] = state[instr[1]] + state[instr[2]]
    return trace
