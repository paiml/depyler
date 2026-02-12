"""Text processing: CSV and delimiter-based parsing.

Tests: field counting, key-value parsing, string operations,
delimiter detection, string repetition.
"""

from typing import Dict, List, Tuple


def csv_field_count(line: str) -> int:
    """Count fields in a simple CSV line."""
    if len(line) == 0:
        return 0
    count: int = 1
    i: int = 0
    while i < len(line):
        if line[i] == ",":
            count += 1
        i += 1
    return count


def count_semicolons(line: str) -> int:
    """Count semicolons in a string."""
    count: int = 0
    i: int = 0
    while i < len(line):
        if line[i] == ";":
            count += 1
        i += 1
    return count


def find_char_pos(s: str, target_code: int) -> int:
    """Find position of character by its ASCII code."""
    i: int = 0
    while i < len(s):
        if ord(s[i]) == target_code:
            return i
        i += 1
    return -1


def substring_before(s: str, pos: int) -> str:
    """Get substring before position."""
    result: List[str] = []
    i: int = 0
    while i < pos and i < len(s):
        result.append(s[i])
        i += 1
    return "".join(result)


def substring_after(s: str, pos: int) -> str:
    """Get substring after position."""
    result: List[str] = []
    i: int = pos + 1
    while i < len(s):
        result.append(s[i])
        i += 1
    return "".join(result)


def repeat_string(s: str, count: int) -> str:
    """Repeat a string count times."""
    result: List[str] = []
    i: int = 0
    while i < count:
        result.append(s)
        i += 1
    return "".join(result)


def join_strings(parts: List[str], delim: str) -> str:
    """Join list of strings with delimiter."""
    if len(parts) == 0:
        return ""
    result: List[str] = []
    i: int = 0
    while i < len(parts):
        if i > 0:
            result.append(delim)
        result.append(parts[i])
        i += 1
    return "".join(result)


def test_csv() -> bool:
    """Test CSV parsing functions."""
    ok: bool = True
    fc: int = csv_field_count("a,b,c,d")
    if fc != 4:
        ok = False
    eq_pos: int = find_char_pos("name=value", ord("="))
    if eq_pos != 4:
        ok = False
    before: str = substring_before("hello=world", 5)
    if before != "hello":
        ok = False
    after: str = substring_after("hello=world", 5)
    if after != "world":
        ok = False
    rep: str = repeat_string("ab", 3)
    if rep != "ababab":
        ok = False
    return ok
