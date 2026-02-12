from typing import List, Tuple

def define_symbol(table: List[Tuple[int, int, int]], name: int, sym_type: int, scope: int) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for t in table:
        result.append(t)
    result.append((name, sym_type, scope))
    return result

def lookup_symbol(table: List[Tuple[int, int, int]], name: int, scope: int) -> int:
    i: int = len(table) - 1
    while i >= 0:
        if table[i][0] == name and table[i][2] <= scope:
            return table[i][1]
        i = i - 1
    return -1

def enter_scope(current: int) -> int:
    return current + 1

def exit_scope(current: int) -> int:
    if current > 0:
        return current - 1
    return 0

def all_in_scope(table: List[Tuple[int, int, int]], scope: int) -> List[int]:
    result: List[int] = []
    for t in table:
        if t[2] == scope:
            result.append(t[0])
    return result

def count_symbols(table: List[Tuple[int, int, int]]) -> int:
    return len(table)
