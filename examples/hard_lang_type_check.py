from typing import List, Tuple, Dict

def type_of_literal(value: int) -> int:
    if value == 0 or value == 1:
        return 3
    return 1

def check_binop(left_type: int, right_type: int, op: int) -> int:
    if left_type == 1 and right_type == 1:
        if op >= 1 and op <= 4:
            return 1
        if op == 5 or op == 6:
            return 3
    if left_type == 2 and right_type == 2:
        if op >= 1 and op <= 4:
            return 2
        if op == 5 or op == 6:
            return 3
    if left_type == 3 and right_type == 3:
        if op == 7 or op == 8:
            return 3
    return 0

def check_unary(operand_type: int, op: int) -> int:
    if op == 9 and operand_type == 1:
        return 1
    if op == 9 and operand_type == 2:
        return 2
    if op == 10 and operand_type == 3:
        return 3
    return 0

def type_check_expr(types: List[int], ops: List[int]) -> List[int]:
    results: List[int] = []
    for i in range(len(ops)):
        if i + 1 < len(types):
            result: int = check_binop(types[i], types[i + 1], ops[i])
            results.append(result)
    return results

def has_type_error(results: List[int]) -> bool:
    for r in results:
        if r == 0:
            return True
    return False

def infer_variable_type(assignments: List[Tuple[int, int]]) -> Dict[int, int]:
    types: Dict[int, int] = {}
    for assign in assignments:
        var_id: int = assign[0]
        val_type: int = assign[1]
        types[var_id] = val_type
    return types
