from typing import List, Tuple

def cont_identity(value: int) -> int:
    return value

def cont_add(value: int, addend: int) -> int:
    return value + addend

def cont_mul(value: int, factor: int) -> int:
    return value * factor

def apply_cont_chain(value: int, ops: List[Tuple[int, int]]) -> int:
    result: int = value
    for op in ops:
        if op[0] == 0:
            result = cont_identity(result)
        elif op[0] == 1:
            result = cont_add(result, op[1])
        elif op[0] == 2:
            result = cont_mul(result, op[1])
    return result

def build_cont_chain(operations: List[int], values: List[int]) -> List[Tuple[int, int]]:
    chain: List[Tuple[int, int]] = []
    for i in range(len(operations)):
        val: int = 0
        if i < len(values):
            val = values[i]
        chain.append((operations[i], val))
    return chain

def compose_conts(a: List[Tuple[int, int]], b: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for op in a:
        result.append(op)
    for op in b:
        result.append(op)
    return result

def factorial_cps(n: int) -> int:
    result: int = 1
    for i in range(1, n + 1):
        result = result * i
    return result

def fibonacci_cps(n: int) -> int:
    if n <= 1:
        return n
    a: int = 0
    b: int = 1
    for i in range(2, n + 1):
        temp: int = a + b
        a = b
        b = temp
    return b
