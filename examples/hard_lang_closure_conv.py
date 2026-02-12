from typing import List, Tuple

def make_closure(func_id: int, free_vars: List[int]) -> List[int]:
    closure: List[int] = [func_id]
    for v in free_vars:
        closure.append(v)
    return closure

def apply_closure(closure: List[int], args: List[int]) -> List[int]:
    result: List[int] = []
    for c in closure:
        result.append(c)
    for a in args:
        result.append(a)
    return result

def extract_free_vars(body: List[int], params: List[int]) -> List[int]:
    free: List[int] = []
    for v in body:
        is_param: bool = False
        for p in params:
            if p == v:
                is_param = True
        if not is_param and v > 0:
            found: bool = False
            for f in free:
                if f == v:
                    found = True
            if not found:
                free.append(v)
    return free

def closure_convert(funcs: List[Tuple[int, List[int], List[int]]]) -> List[List[int]]:
    closures: List[List[int]] = []
    for func in funcs:
        fid: int = func[0]
        params: List[int] = func[1]
        body: List[int] = func[2]
        free: List[int] = extract_free_vars(body, params)
        closures.append(make_closure(fid, free))
    return closures

def count_free_vars(closures: List[List[int]]) -> List[int]:
    counts: List[int] = []
    for c in closures:
        counts.append(len(c) - 1)
    return counts

def total_closure_size(closures: List[List[int]]) -> int:
    total: int = 0
    for c in closures:
        total = total + len(c)
    return total
