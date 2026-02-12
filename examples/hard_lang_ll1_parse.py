from typing import List, Tuple

def ll1_parse_step(stack: List[int], rules_flat: List[int], rule_len: int, num_rules: int, token: int) -> List[int]:
    if len(stack) == 0:
        return stack
    top: int = stack[len(stack) - 1]
    new_stack: List[int] = stack[0:len(stack) - 1]
    if top >= 100:
        return new_stack
    for r in range(num_rules):
        rs: int = r * rule_len
        actual_len: int = rules_flat[rs + rule_len - 1]
        if rules_flat[rs] == top and actual_len > 1 and rules_flat[rs + 1] == token:
            rev: List[int] = []
            for j in range(1, actual_len):
                rev.append(rules_flat[rs + j])
            i: int = len(rev) - 1
            while i >= 0:
                new_stack.append(rev[i])
                i = i - 1
            return new_stack
    return new_stack

def ll1_parse(rules_flat: List[int], rule_len: int, num_rules: int, tokens: List[int], start: int) -> bool:
    stack: List[int] = [start]
    pos: int = 0
    while len(stack) > 0 and pos <= len(tokens):
        top: int = stack[len(stack) - 1]
        new_stack: List[int] = stack[0:len(stack) - 1]
        if top >= 100:
            if pos < len(tokens) and tokens[pos] == top:
                pos = pos + 1
                stack = new_stack
            else:
                return False
        else:
            if pos >= len(tokens):
                return False
            matched: bool = False
            for r in range(num_rules):
                rs: int = r * rule_len
                actual_len: int = rules_flat[rs + rule_len - 1]
                rhead: int = rules_flat[rs]
                rfirst: int = 0
                if actual_len > 1:
                    rfirst = rules_flat[rs + 1]
                if rhead == top and actual_len > 1 and rfirst == tokens[pos]:
                    rev: List[int] = []
                    for j in range(1, actual_len):
                        rev.append(rules_flat[rs + j])
                    i: int = len(rev) - 1
                    while i >= 0:
                        new_stack.append(rev[i])
                        i = i - 1
                    matched = True
                    stack = new_stack
                    break
            if not matched:
                return False
    return pos == len(tokens)

def is_ll1(rules_flat: List[int], rule_len: int, num_rules: int) -> bool:
    for i in range(num_rules):
        for j in range(i + 1, num_rules):
            rs_i: int = i * rule_len
            rs_j: int = j * rule_len
            len_i: int = rules_flat[rs_i + rule_len - 1]
            len_j: int = rules_flat[rs_j + rule_len - 1]
            if rules_flat[rs_i] == rules_flat[rs_j]:
                if len_i > 1 and len_j > 1:
                    if rules_flat[rs_i + 1] == rules_flat[rs_j + 1]:
                        return False
    return True

def count_nonterminals(rules_flat: List[int], rule_len: int, num_rules: int) -> int:
    seen: List[int] = []
    for r in range(num_rules):
        rs: int = r * rule_len
        sym: int = rules_flat[rs]
        found: bool = False
        for s in seen:
            if s == sym:
                found = True
        if not found:
            seen.append(sym)
    return len(seen)

def parse_trace(tokens: List[int], start: int) -> List[int]:
    trace: List[int] = [start]
    for t in tokens:
        trace.append(t)
    return trace
