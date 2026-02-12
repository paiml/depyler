from typing import List, Tuple, Dict

def packrat_match(rules: Dict[int, List[int]], memo: Dict[int, Dict[int, Tuple[bool, int]]], rule_id: int, tokens: List[int], pos: int) -> Tuple[bool, int]:
    if rule_id in memo and pos in memo[rule_id]:
        return memo[rule_id][pos]
    if rule_id not in rules:
        result: Tuple[bool, int] = (False, pos)
    else:
        rule: List[int] = rules[rule_id]
        result = try_match_seq(rules, memo, rule, tokens, pos)
    if rule_id not in memo:
        memo[rule_id] = {}
    memo[rule_id][pos] = result
    return result

def try_match_seq(rules: Dict[int, List[int]], memo: Dict[int, Dict[int, Tuple[bool, int]]], seq: List[int], tokens: List[int], pos: int) -> Tuple[bool, int]:
    cp: int = pos
    for sym in seq:
        if sym >= 100:
            if cp < len(tokens) and tokens[cp] == sym:
                cp = cp + 1
            else:
                return (False, pos)
        else:
            r: Tuple[bool, int] = packrat_match(rules, memo, sym, tokens, cp)
            if not r[0]:
                return (False, pos)
            cp = r[1]
    return (True, cp)

def packrat_parse(rules: Dict[int, List[int]], tokens: List[int], start: int) -> bool:
    memo: Dict[int, Dict[int, Tuple[bool, int]]] = {}
    result: Tuple[bool, int] = packrat_match(rules, memo, start, tokens, 0)
    return result[0] and result[1] == len(tokens)

def memo_hits(memo: Dict[int, Dict[int, Tuple[bool, int]]]) -> int:
    count: int = 0
    for rule_id in memo:
        count = count + len(memo[rule_id])
    return count

def clear_memo() -> Dict[int, Dict[int, Tuple[bool, int]]]:
    return {}
