from typing import List, Tuple

def lift_lambda(func_id: int, free_vars: List[int], body: List[int]) -> Tuple[List[int], List[int]]:
    new_params: List[int] = []
    for v in free_vars:
        new_params.append(v)
    new_body: List[int] = []
    for b in body:
        found: bool = False
        for idx in range(len(free_vars)):
            if b == free_vars[idx]:
                new_body.append(1000 + idx)
                found = True
        if not found:
            new_body.append(b)
    return (new_params, new_body)

def find_nested_lambdas(code: List[int], lambda_marker: int) -> List[int]:
    positions: List[int] = []
    for i in range(len(code)):
        if code[i] == lambda_marker:
            positions.append(i)
    return positions

def lift_all(funcs: List[Tuple[int, List[int], List[int]]]) -> List[Tuple[int, List[int], List[int]]]:
    lifted: List[Tuple[int, List[int], List[int]]] = []
    for func in funcs:
        fid: int = func[0]
        params: List[int] = func[1]
        body: List[int] = func[2]
        free: List[int] = []
        for b in body:
            if b > 100:
                is_param: bool = False
                for p in params:
                    if p == b:
                        is_param = True
                if not is_param:
                    free.append(b)
        result: Tuple[List[int], List[int]] = lift_lambda(fid, free, body)
        all_params: List[int] = []
        for p in params:
            all_params.append(p)
        for np in result[0]:
            all_params.append(np)
        lifted.append((fid, all_params, result[1]))
    return lifted

def count_lifted(original: int, lifted: int) -> int:
    return lifted - original

def total_params(funcs: List[Tuple[int, List[int], List[int]]]) -> int:
    total: int = 0
    for f in funcs:
        total = total + len(f[1])
    return total
