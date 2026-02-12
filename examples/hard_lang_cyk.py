from typing import List, Tuple

def cyk_parse(rules: List[Tuple[int, int, int]], unit_rules: List[Tuple[int, int]], tokens: List[int], start: int) -> bool:
    n: int = len(tokens)
    if n == 0:
        return False
    table: List[List[List[int]]] = []
    for i in range(n):
        row: List[List[int]] = []
        for j in range(n):
            row.append([])
        table.append(row)
    for i in range(n):
        for rule in unit_rules:
            if rule[1] == tokens[i]:
                table[i][i].append(rule[0])
    length: int = 2
    while length <= n:
        for i in range(n - length + 1):
            j: int = i + length - 1
            for k in range(i, j):
                for rule in rules:
                    b_found: bool = False
                    for sym in table[i][k]:
                        if sym == rule[1]:
                            b_found = True
                    c_found: bool = False
                    for sym2 in table[k + 1][j]:
                        if sym2 == rule[2]:
                            c_found = True
                    if b_found and c_found:
                        already: bool = False
                        for existing in table[i][j]:
                            if existing == rule[0]:
                                already = True
                        if not already:
                            table[i][j].append(rule[0])
        length = length + 1
    for sym3 in table[0][n - 1]:
        if sym3 == start:
            return True
    return False

def count_parses(table: List[List[List[int]]], start: int, n: int) -> int:
    count: int = 0
    if n > 0:
        for sym in table[0][n - 1]:
            if sym == start:
                count = count + 1
    return count

def is_cnf(rules: List[Tuple[int, int, int]], unit_rules: List[Tuple[int, int]]) -> bool:
    for rule in rules:
        if rule[1] >= 100 or rule[2] >= 100:
            return False
    for ur in unit_rules:
        if ur[1] < 100:
            return False
    return True
