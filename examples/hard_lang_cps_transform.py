from typing import List, Tuple

def cps_literal(value: int, cont: int) -> List[int]:
    return [0, value, cont]

def cps_add(a: int, b: int, cont: int) -> List[int]:
    return [1, a, b, cont]

def cps_call(func: int, arg: int, cont: int) -> List[int]:
    return [2, func, arg, cont]

def cps_if(cond: int, then_cont: int, else_cont: int) -> List[int]:
    return [3, cond, then_cont, else_cont]

def transform_expr(expr: List[int], next_cont: int) -> List[List[int]]:
    result: List[List[int]] = []
    if expr[0] == 0:
        result.append(cps_literal(expr[1], next_cont))
    elif expr[0] == 1:
        result.append(cps_add(expr[1], expr[2], next_cont))
    elif expr[0] == 2:
        result.append(cps_call(expr[1], expr[2], next_cont))
    return result

def chain_transforms(exprs: List[List[int]]) -> List[List[int]]:
    result: List[List[int]] = []
    cont: int = len(exprs)
    i: int = len(exprs) - 1
    while i >= 0:
        transformed: List[List[int]] = transform_expr(exprs[i], cont)
        for t in transformed:
            result.append(t)
        cont = cont - 1
        i = i - 1
    return result

def count_continuations(cps: List[List[int]]) -> int:
    conts: List[int] = []
    for c in cps:
        if len(c) > 2:
            last: int = c[len(c) - 1]
            found: bool = False
            for existing in conts:
                if existing == last:
                    found = True
            if not found:
                conts.append(last)
    return len(conts)
