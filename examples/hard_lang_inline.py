from typing import List, Tuple

def should_inline(func_size: int, call_count: int, threshold: int) -> bool:
    if func_size <= threshold:
        return True
    if call_count == 1:
        return True
    return func_size * call_count < threshold * 3

def inline_at(caller: List[int], callee: List[int], site: int) -> List[int]:
    result: List[int] = []
    for i in range(site):
        result.append(caller[i])
    for c in callee:
        result.append(c)
    for i in range(site + 1, len(caller)):
        result.append(caller[i])
    return result

def count_calls(code: List[int], call_op: int) -> int:
    count: int = 0
    for c in code:
        if c == call_op:
            count = count + 1
    return count

def inline_benefit(func_size: int, call_overhead: int) -> int:
    return call_overhead - 1

def code_size(functions: List[List[int]]) -> int:
    total: int = 0
    for f in functions:
        total = total + len(f)
    return total
