from typing import List, Tuple

def char_class(c: int) -> int:
    if c >= 48 and c <= 57:
        return 1
    if (c >= 65 and c <= 90) or (c >= 97 and c <= 122) or c == 95:
        return 2
    if c == 32 or c == 9 or c == 10:
        return 0
    return 3

def scan_string(src: List[int], pos: int, quote: int) -> Tuple[List[int], int]:
    chars: List[int] = []
    pos = pos + 1
    while pos < len(src) and src[pos] != quote:
        if src[pos] == 92 and pos + 1 < len(src):
            pos = pos + 1
        chars.append(src[pos])
        pos = pos + 1
    if pos < len(src):
        pos = pos + 1
    return (chars, pos)

def scan_number(src: List[int], pos: int) -> Tuple[int, int]:
    val: int = 0
    while pos < len(src) and src[pos] >= 48 and src[pos] <= 57:
        val = val * 10 + src[pos] - 48
        pos = pos + 1
    return (val, pos)

def scan_word(src: List[int], pos: int) -> Tuple[int, int]:
    start: int = pos
    while pos < len(src) and char_class(src[pos]) >= 1 and char_class(src[pos]) <= 2:
        pos = pos + 1
    return (pos - start, pos)

def tokenize_full(src: List[int]) -> List[Tuple[int, int]]:
    tokens: List[Tuple[int, int]] = []
    pos: int = 0
    while pos < len(src):
        cc: int = char_class(src[pos])
        if cc == 0:
            pos = pos + 1
        elif cc == 1:
            r: Tuple[int, int] = scan_number(src, pos)
            tokens.append((1, r[0]))
            pos = r[1]
        elif cc == 2:
            r2: Tuple[int, int] = scan_word(src, pos)
            tokens.append((2, r2[0]))
            pos = r2[1]
        elif src[pos] == 34 or src[pos] == 39:
            r3: Tuple[List[int], int] = scan_string(src, pos, src[pos])
            tokens.append((3, len(r3[0])))
            pos = r3[1]
        else:
            tokens.append((4, src[pos]))
            pos = pos + 1
    return tokens

def filter_tokens(tokens: List[Tuple[int, int]], keep_type: int) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for t in tokens:
        if t[0] == keep_type:
            result.append(t)
    return result
