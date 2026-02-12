from typing import List, Tuple

def is_keyword(tok: int) -> bool:
    return tok >= 100 and tok < 200

def is_identifier(tok: int) -> bool:
    return tok >= 200 and tok < 300

def parse_select_list(tokens: List[int], pos: int) -> Tuple[List[int], int]:
    cols: List[int] = []
    while pos < len(tokens) and is_identifier(tokens[pos]):
        cols.append(tokens[pos])
        pos = pos + 1
        if pos < len(tokens) and tokens[pos] == 44:
            pos = pos + 1
    return (cols, pos)

def parse_from_clause(tokens: List[int], pos: int) -> Tuple[int, int]:
    if pos < len(tokens) and tokens[pos] == 101:
        pos = pos + 1
        if pos < len(tokens):
            return (tokens[pos], pos + 1)
    return (-1, pos)

def parse_where_clause(tokens: List[int], pos: int) -> Tuple[List[int], int]:
    conditions: List[int] = []
    if pos < len(tokens) and tokens[pos] == 102:
        pos = pos + 1
        while pos < len(tokens):
            conditions.append(tokens[pos])
            pos = pos + 1
    return (conditions, pos)

def parse_query(tokens: List[int]) -> Tuple[List[int], int, List[int]]:
    pos: int = 0
    if pos < len(tokens) and tokens[pos] == 100:
        pos = pos + 1
    cols: Tuple[List[int], int] = parse_select_list(tokens, pos)
    pos = cols[1]
    table: Tuple[int, int] = parse_from_clause(tokens, pos)
    pos = table[1]
    where: Tuple[List[int], int] = parse_where_clause(tokens, pos)
    return (cols[0], table[0], where[0])
