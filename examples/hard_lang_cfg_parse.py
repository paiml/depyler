from typing import List, Tuple

def first_set(rules: List[List[int]], symbol: int) -> List[int]:
    result: List[int] = []
    for rule in rules:
        if rule[0] == symbol and len(rule) > 1:
            first: int = rule[1]
            found: bool = False
            for r in result:
                if r == first:
                    found = True
            if not found:
                result.append(first)
    return result

def follow_set(rules: List[List[int]], symbol: int) -> List[int]:
    result: List[int] = []
    for rule in rules:
        for i in range(1, len(rule)):
            if rule[i] == symbol and i + 1 < len(rule):
                nxt: int = rule[i + 1]
                found: bool = False
                for r in result:
                    if r == nxt:
                        found = True
                if not found:
                    result.append(nxt)
    return result

def is_terminal(symbol: int) -> bool:
    return symbol >= 100

def parse_rule(rule: List[int], tokens: List[int], pos: int) -> Tuple[bool, int]:
    np: int = pos
    for i in range(1, len(rule)):
        if is_terminal(rule[i]):
            if np < len(tokens) and tokens[np] == rule[i]:
                np = np + 1
            else:
                return (False, pos)
        else:
            return (True, np)
    return (True, np)

def validate_grammar(rules: List[List[int]]) -> bool:
    for rule in rules:
        if len(rule) < 2:
            return False
        if is_terminal(rule[0]):
            return False
    return True

def count_rules_for(rules: List[List[int]], symbol: int) -> int:
    count: int = 0
    for rule in rules:
        if rule[0] == symbol:
            count = count + 1
    return count
