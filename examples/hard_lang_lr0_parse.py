from typing import List, Tuple

def lr0_action(state: int, token: int, table: List[List[int]]) -> int:
    if state < len(table) and token < len(table[state]):
        return table[state][token]
    return 0

def lr0_goto(state: int, sym: int, table: List[List[int]]) -> int:
    if state < len(table) and sym < len(table[state]):
        return table[state][sym]
    return -1

def lr0_parse(action_table: List[List[int]], goto_table: List[List[int]], rule_lens: List[int], rule_lhs: List[int], tokens: List[int]) -> bool:
    stack: List[int] = [0]
    pos: int = 0
    max_steps: int = len(tokens) * 10
    steps: int = 0
    while pos <= len(tokens) and steps < max_steps:
        steps = steps + 1
        state: int = stack[len(stack) - 1]
        tok: int = 0
        if pos < len(tokens):
            tok = tokens[pos]
        action: int = lr0_action(state, tok, action_table)
        if action > 0:
            stack.append(action)
            pos = pos + 1
        elif action < -1:
            ri: int = (0 - action) - 2
            if ri >= len(rule_lens):
                return False
            rl: int = rule_lens[ri]
            for i in range(rl):
                if len(stack) > 1:
                    stack = stack[0:len(stack) - 1]
            ns: int = lr0_goto(stack[len(stack) - 1], rule_lhs[ri], goto_table)
            if ns < 0:
                return False
            stack.append(ns)
        elif action == -1:
            return True
        else:
            return False
    return False

def build_table(states: int, syms: int) -> List[List[int]]:
    table: List[List[int]] = []
    for i in range(states):
        row: List[int] = [0] * syms
        table.append(row)
    return table

def set_entry(table: List[List[int]], s: int, t: int, v: int) -> List[List[int]]:
    r: List[List[int]] = []
    for row in table:
        nr: List[int] = []
        for val in row:
            nr.append(val)
        r.append(nr)
    r[s][t] = v
    return r
