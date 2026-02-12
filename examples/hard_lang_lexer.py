from typing import List, Tuple

def is_digit(c: int) -> bool:
    return c >= 48 and c <= 57

def is_alpha(c: int) -> bool:
    return (c >= 65 and c <= 90) or (c >= 97 and c <= 122) or c == 95

def is_whitespace(c: int) -> bool:
    return c == 32 or c == 9 or c == 10 or c == 13

def lex_number(src: List[int], pos: int) -> Tuple[int, int]:
    value: int = 0
    while pos < len(src) and is_digit(src[pos]):
        value = value * 10 + (src[pos] - 48)
        pos = pos + 1
    return (value, pos)

def lex_identifier(src: List[int], pos: int) -> Tuple[List[int], int]:
    chars: List[int] = []
    while pos < len(src) and (is_alpha(src[pos]) or is_digit(src[pos])):
        chars.append(src[pos])
        pos = pos + 1
    return (chars, pos)

def tokenize(src: List[int]) -> List[Tuple[int, int]]:
    tokens: List[Tuple[int, int]] = []
    pos: int = 0
    while pos < len(src):
        if is_whitespace(src[pos]):
            pos = pos + 1
        elif is_digit(src[pos]):
            result: Tuple[int, int] = lex_number(src, pos)
            tokens.append((1, result[0]))
            pos = result[1]
        elif is_alpha(src[pos]):
            id_result: Tuple[List[int], int] = lex_identifier(src, pos)
            tokens.append((2, len(id_result[0])))
            pos = id_result[1]
        elif src[pos] == 43:
            tokens.append((3, 43))
            pos = pos + 1
        elif src[pos] == 45:
            tokens.append((3, 45))
            pos = pos + 1
        elif src[pos] == 42:
            tokens.append((3, 42))
            pos = pos + 1
        elif src[pos] == 47:
            tokens.append((3, 47))
            pos = pos + 1
        elif src[pos] == 40:
            tokens.append((4, 40))
            pos = pos + 1
        elif src[pos] == 41:
            tokens.append((4, 41))
            pos = pos + 1
        else:
            tokens.append((0, src[pos]))
            pos = pos + 1
    return tokens

def count_tokens_by_type(tokens: List[Tuple[int, int]], tok_type: int) -> int:
    count: int = 0
    for t in tokens:
        if t[0] == tok_type:
            count = count + 1
    return count
