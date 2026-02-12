from typing import List, Tuple

def earley_predict(items: List[Tuple[int, int, int]], rules_flat: List[int], num_rules: int, rule_len: int, pos: int) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for item in items:
        result.append(item)
    for item in items:
        ri: int = item[0]
        dot: int = item[1]
        rule_start: int = ri * rule_len
        actual_len: int = rules_flat[rule_start + rule_len - 1]
        if dot < actual_len - 1:
            ns: int = rules_flat[rule_start + dot + 1]
            if ns < 100:
                for i in range(num_rules):
                    rs: int = i * rule_len
                    if rules_flat[rs] == ns:
                        result.append((i, 0, pos))
    return result

def earley_scan(items: List[Tuple[int, int, int]], rules_flat: List[int], rule_len: int, token: int) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for item in items:
        ri: int = item[0]
        dot: int = item[1]
        rule_start: int = ri * rule_len
        actual_len: int = rules_flat[rule_start + rule_len - 1]
        if dot < actual_len - 1:
            ns: int = rules_flat[rule_start + dot + 1]
            if ns == token:
                result.append((ri, dot + 1, item[2]))
    return result

def earley_init(rules_flat: List[int], num_rules: int, rule_len: int, start: int) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for i in range(num_rules):
        rs: int = i * rule_len
        if rules_flat[rs] == start:
            result.append((i, 0, 0))
    return result

def count_items_flat(items: List[Tuple[int, int, int]]) -> int:
    return len(items)

def has_complete(items: List[Tuple[int, int, int]], rules_flat: List[int], rule_len: int, start: int) -> bool:
    for item in items:
        ri: int = item[0]
        dot: int = item[1]
        rs: int = ri * rule_len
        actual_len: int = rules_flat[rs + rule_len - 1]
        if dot == actual_len - 1 and rules_flat[rs] == start and item[2] == 0:
            return True
    return False
