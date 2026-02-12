from typing import List, Tuple

def match_char(pattern: int, ch: int) -> bool:
    if pattern == 46:
        return True
    return pattern == ch

def match_here(text: List[int], start_pos: int, pattern: List[int], start_pat: int) -> Tuple[bool, int]:
    pos: int = start_pos
    pp: int = start_pat
    while pp < len(pattern):
        if pos >= len(text):
            return (False, pos)
        p: int = pattern[pp]
        if p == 46 or p == text[pos]:
            pos = pos + 1
            pp = pp + 1
        else:
            return (False, pos)
    return (True, pos)

def regex_search(text: List[int], pattern: List[int]) -> int:
    for i in range(len(text)):
        matched: bool = True
        for j in range(len(pattern)):
            if i + j >= len(text):
                matched = False
                break
            if pattern[j] != 46 and pattern[j] != text[i + j]:
                matched = False
                break
        if matched:
            return i
    return -1

def regex_count(text: List[int], pattern: List[int]) -> int:
    count: int = 0
    for i in range(len(text)):
        matched: bool = True
        for j in range(len(pattern)):
            if i + j >= len(text):
                matched = False
                break
            if pattern[j] != 46 and pattern[j] != text[i + j]:
                matched = False
                break
        if matched:
            count = count + 1
    return count

def pattern_length(pattern: List[int]) -> int:
    return len(pattern)

def is_wildcard(c: int) -> bool:
    return c == 46
