from typing import List, Tuple, Dict

def is_constant(instr: Tuple[int, int, int]) -> bool:
    return instr[0] == 0

def fold_binary(op: int, a: int, b: int) -> int:
    if op == 1:
        return a + b
    if op == 2:
        return a - b
    if op == 3:
        return a * b
    if op == 4 and b != 0:
        return a // b
    if op == 5:
        return a & b
    if op == 6:
        return a | b
    if op == 7:
        return a ^ b
    return 0

def const_propagate(instrs: List[Tuple[int, int, int]]) -> List[Tuple[int, int, int]]:
    constants: Dict[int, int] = {}
    result: List[Tuple[int, int, int]] = []
    for instr in instrs:
        if instr[0] == 0:
            constants[instr[1]] = instr[2]
            result.append(instr)
        elif instr[2] in constants:
            new_val: int = constants[instr[2]]
            result.append((0, instr[1], new_val))
            constants[instr[1]] = new_val
        else:
            result.append(instr)
    return result

def fold_constants(instrs: List[Tuple[int, int, int]]) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    constants: Dict[int, int] = {}
    for instr in instrs:
        if instr[0] == 0:
            constants[instr[1]] = instr[2]
            result.append(instr)
        elif instr[1] in constants and instr[2] in constants:
            folded: int = fold_binary(instr[0], constants[instr[1]], constants[instr[2]])
            result.append((0, instr[1], folded))
            constants[instr[1]] = folded
        else:
            result.append(instr)
    return result

def count_constants(instrs: List[Tuple[int, int, int]]) -> int:
    count: int = 0
    for instr in instrs:
        if instr[0] == 0:
            count = count + 1
    return count
