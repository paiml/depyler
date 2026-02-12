from typing import List, Tuple

def dfa_transition(table: List[List[int]], state: int, char_val: int) -> int:
    if state < 0 or state >= len(table):
        return -1
    if char_val < 0 or char_val >= len(table[state]):
        return -1
    return table[state][char_val]

def dfa_match(table: List[List[int]], accept: List[int], input_str: List[int]) -> bool:
    state: int = 0
    for ch in input_str:
        state = dfa_transition(table, state, ch)
        if state < 0:
            return False
    for a in accept:
        if state == a:
            return True
    return False

def minimize_partition(table: List[List[int]], accept: List[int], num_states: int) -> List[int]:
    partition: List[int] = []
    for i in range(num_states):
        is_accept: bool = False
        for a in accept:
            if i == a:
                is_accept = True
        if is_accept:
            partition.append(1)
        else:
            partition.append(0)
    return partition

def build_dfa_table(num_states: int, num_chars: int) -> List[List[int]]:
    table: List[List[int]] = []
    for i in range(num_states):
        row: List[int] = []
        for j in range(num_chars):
            row.append(-1)
        table.append(row)
    return table

def set_transition(table: List[List[int]], fr: int, ch: int, to: int) -> List[List[int]]:
    new_t: List[List[int]] = []
    for row in table:
        nr: List[int] = []
        for v in row:
            nr.append(v)
        new_t.append(nr)
    new_t[fr][ch] = to
    return new_t

def count_transitions(table: List[List[int]]) -> int:
    count: int = 0
    for row in table:
        for v in row:
            if v >= 0:
                count = count + 1
    return count
