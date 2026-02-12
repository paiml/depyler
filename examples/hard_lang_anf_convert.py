from typing import List, Tuple

def anf_let(name: int, value: int, body: List[int]) -> List[int]:
    result: List[int] = [0, name, value]
    for b in body:
        result.append(b)
    return result

def anf_atom(value: int) -> List[int]:
    return [1, value]

def anf_binop(op: int, left: int, right: int) -> List[int]:
    return [2, op, left, right]

def flatten_expr(expr: List[int], counter: int) -> Tuple[List[List[int]], int, int]:
    if expr[0] == 1:
        return ([], expr[1], counter)
    if expr[0] == 2:
        lets: List[List[int]] = []
        tmp: int = counter
        counter = counter + 1
        lets.append([0, tmp, expr[1]])
        return (lets, tmp, counter)
    return ([], 0, counter)

def normalize(exprs: List[List[int]]) -> List[List[int]]:
    result: List[List[int]] = []
    counter: int = 100
    for expr in exprs:
        flat: Tuple[List[List[int]], int, int] = flatten_expr(expr, counter)
        for let in flat[0]:
            result.append(let)
        result.append(anf_atom(flat[1]))
        counter = flat[2]
    return result

def is_anf(instrs: List[List[int]]) -> bool:
    for instr in instrs:
        if instr[0] == 2:
            if instr[2] >= 100 or instr[3] >= 100:
                return False
    return True

def count_lets(instrs: List[List[int]]) -> int:
    count: int = 0
    for instr in instrs:
        if instr[0] == 0:
            count = count + 1
    return count
