from typing import List, Tuple

def peg_literal(tokens: List[int], pos: int, expected: int) -> Tuple[bool, int]:
    if pos < len(tokens) and tokens[pos] == expected:
        return (True, pos + 1)
    return (False, pos)

def peg_sequence(results: List[Tuple[bool, int]]) -> Tuple[bool, int]:
    for r in results:
        if not r[0]:
            return (False, r[1])
    if len(results) > 0:
        return results[len(results) - 1]
    return (True, 0)

def peg_choice(tokens: List[int], pos: int, alts: List[int]) -> Tuple[bool, int]:
    for alt in alts:
        result: Tuple[bool, int] = peg_literal(tokens, pos, alt)
        if result[0]:
            return result
    return (False, pos)

def peg_star(tokens: List[int], pos: int, expected: int) -> Tuple[int, int]:
    count: int = 0
    cp: int = pos
    while cp < len(tokens) and tokens[cp] == expected:
        cp = cp + 1
        count = count + 1
    return (count, cp)

def peg_plus(tokens: List[int], pos: int, expected: int) -> Tuple[bool, int]:
    result: Tuple[int, int] = peg_star(tokens, pos, expected)
    if result[0] > 0:
        return (True, result[1])
    return (False, pos)

def peg_not(tokens: List[int], pos: int, expected: int) -> Tuple[bool, int]:
    result: Tuple[bool, int] = peg_literal(tokens, pos, expected)
    if result[0]:
        return (False, pos)
    return (True, pos)

def peg_parse(tokens: List[int], grammar: List[List[int]]) -> bool:
    pos: int = 0
    for rule in grammar:
        for expected in rule:
            if pos < len(tokens) and tokens[pos] == expected:
                pos = pos + 1
            else:
                return False
    return pos == len(tokens)
