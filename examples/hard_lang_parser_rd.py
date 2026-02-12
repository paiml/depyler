from typing import List, Tuple

def parse_expr(tokens: List[int], pos: int) -> Tuple[int, int]:
    left: int = 0
    np: int = pos
    result: Tuple[int, int] = parse_term(tokens, np)
    left = result[0]
    np = result[1]
    while np < len(tokens) and (tokens[np] == 43 or tokens[np] == 45):
        op: int = tokens[np]
        np = np + 1
        right_result: Tuple[int, int] = parse_term(tokens, np)
        right: int = right_result[0]
        np = right_result[1]
        if op == 43:
            left = left + right
        else:
            left = left - right
    return (left, np)

def parse_term(tokens: List[int], pos: int) -> Tuple[int, int]:
    left: int = 0
    np: int = pos
    result: Tuple[int, int] = parse_factor(tokens, np)
    left = result[0]
    np = result[1]
    while np < len(tokens) and (tokens[np] == 42 or tokens[np] == 47):
        op: int = tokens[np]
        np = np + 1
        right_result: Tuple[int, int] = parse_factor(tokens, np)
        right: int = right_result[0]
        np = right_result[1]
        if op == 42:
            left = left * right
        else:
            if right != 0:
                left = left // right
    return (left, np)

def parse_factor(tokens: List[int], pos: int) -> Tuple[int, int]:
    if pos < len(tokens) and tokens[pos] == 40:
        result: Tuple[int, int] = parse_expr(tokens, pos + 1)
        np: int = result[1]
        if np < len(tokens) and tokens[np] == 41:
            np = np + 1
        return (result[0], np)
    if pos < len(tokens):
        return (tokens[pos], pos + 1)
    return (0, pos)

def evaluate(tokens: List[int]) -> int:
    result: Tuple[int, int] = parse_expr(tokens, 0)
    return result[0]

def count_parens(tokens: List[int]) -> Tuple[int, int]:
    opens: int = 0
    closes: int = 0
    for t in tokens:
        if t == 40:
            opens = opens + 1
        elif t == 41:
            closes = closes + 1
    return (opens, closes)
