from typing import List, Tuple

def peephole_add_zero(instrs: List[Tuple[int, int, int]]) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for instr in instrs:
        if instr[0] == 1 and instr[2] == 0:
            continue
        result.append(instr)
    return result

def peephole_mul_one(instrs: List[Tuple[int, int, int]]) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for instr in instrs:
        if instr[0] == 3 and instr[2] == 1:
            continue
        result.append(instr)
    return result

def peephole_double_neg(instrs: List[Tuple[int, int, int]]) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    i: int = 0
    while i < len(instrs):
        if i + 1 < len(instrs) and instrs[i][0] == 9 and instrs[i + 1][0] == 9:
            if instrs[i][1] == instrs[i + 1][2]:
                i = i + 2
                continue
        result.append(instrs[i])
        i = i + 1
    return result

def strength_reduce(instrs: List[Tuple[int, int, int]]) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for instr in instrs:
        if instr[0] == 3 and instr[2] == 2:
            result.append((1, instr[1], instr[1]))
        elif instr[0] == 4 and instr[2] == 2:
            result.append((10, instr[1], 1))
        else:
            result.append(instr)
    return result

def apply_all_peepholes(instrs: List[Tuple[int, int, int]]) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = instrs
    result = peephole_add_zero(result)
    result = peephole_mul_one(result)
    result = peephole_double_neg(result)
    result = strength_reduce(result)
    return result

def count_optimized(before: int, after: int) -> int:
    return before - after
