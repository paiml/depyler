from typing import List, Tuple

def emit_push(value: int) -> List[int]:
    return [1, value]

def emit_add() -> List[int]:
    return [2, 0]

def emit_sub() -> List[int]:
    return [3, 0]

def emit_mul() -> List[int]:
    return [4, 0]

def emit_div() -> List[int]:
    return [5, 0]

def emit_load(slot: int) -> List[int]:
    return [6, slot]

def emit_store(slot: int) -> List[int]:
    return [7, slot]

def emit_jump(target: int) -> List[int]:
    return [8, target]

def emit_jump_if_zero(target: int) -> List[int]:
    return [9, target]

def compile_expr(tokens: List[int]) -> List[List[int]]:
    bytecode: List[List[int]] = []
    for t in tokens:
        if t >= 0 and t <= 999:
            bytecode.append(emit_push(t))
        elif t == 1000:
            bytecode.append(emit_add())
        elif t == 1001:
            bytecode.append(emit_sub())
        elif t == 1002:
            bytecode.append(emit_mul())
        elif t == 1003:
            bytecode.append(emit_div())
    return bytecode

def bytecode_size(program: List[List[int]]) -> int:
    total: int = 0
    for instr in program:
        total = total + len(instr)
    return total

def disassemble(program: List[List[int]]) -> List[int]:
    opcodes: List[int] = []
    for instr in program:
        opcodes.append(instr[0])
    return opcodes
