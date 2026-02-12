"""Text processing: Regex-like pattern primitives.

Tests: character class matching, wildcard matching, repetition counting,
string extraction, template substitution.
"""

from typing import Dict, List, Tuple


def wildcard_match(text: str, pattern: str) -> bool:
    """Simple wildcard matching: ? matches any char, * matches any sequence."""
    nt: int = len(text)
    np: int = len(pattern)
    ti: int = 0
    pi: int = 0
    star_pi: int = -1
    star_ti: int = -1
    while ti < nt:
        if pi < np and (pattern[pi] == "?" or pattern[pi] == text[ti]):
            ti += 1
            pi += 1
        elif pi < np and pattern[pi] == "*":
            star_pi = pi
            star_ti = ti
            pi += 1
        elif star_pi >= 0:
            pi = star_pi + 1
            star_ti += 1
            ti = star_ti
        else:
            return False
    while pi < np and pattern[pi] == "*":
        pi += 1
    return pi == np


def count_digit_runs(text: str) -> int:
    """Count consecutive runs of digit characters."""
    count: int = 0
    in_run: bool = False
    i: int = 0
    while i < len(text):
        if text[i] >= "0" and text[i] <= "9":
            if not in_run:
                count += 1
                in_run = True
        else:
            in_run = False
        i += 1
    return count


def extract_digits(text: str) -> str:
    """Extract only digit characters from text."""
    result: List[str] = []
    i: int = 0
    while i < len(text):
        if text[i] >= "0" and text[i] <= "9":
            result.append(text[i])
        i += 1
    return "".join(result)


def extract_alpha(text: str) -> str:
    """Extract only alphabetic characters from text."""
    result: List[str] = []
    i: int = 0
    while i < len(text):
        lower: bool = text[i] >= "a" and text[i] <= "z"
        upper: bool = text[i] >= "A" and text[i] <= "Z"
        if lower or upper:
            result.append(text[i])
        i += 1
    return "".join(result)


def simple_template(tmpl: str, vals: Dict[str, str]) -> str:
    """Simple template substitution: replace {name} with vals[name]."""
    result: List[str] = []
    i: int = 0
    n: int = len(tmpl)
    while i < n:
        if tmpl[i] == "{":
            name_parts: List[str] = []
            i += 1
            while i < n and tmpl[i] != "}":
                name_parts.append(tmpl[i])
                i += 1
            if i < n:
                i += 1
            name_str: str = "".join(name_parts)
            if name_str in vals:
                result.append(vals[name_str])
            else:
                result.append("{")
                result.append(name_str)
                result.append("}")
        else:
            result.append(tmpl[i])
            i += 1
    return "".join(result)


def count_uppercase(text: str) -> int:
    """Count uppercase characters."""
    count: int = 0
    i: int = 0
    while i < len(text):
        if text[i] >= "A" and text[i] <= "Z":
            count += 1
        i += 1
    return count


def test_pattern() -> bool:
    """Test pattern matching."""
    ok: bool = True
    if not wildcard_match("hello", "h*o"):
        ok = False
    if not wildcard_match("hello", "h?llo"):
        ok = False
    if wildcard_match("hello", "h?o"):
        ok = False
    digits: str = extract_digits("abc123def456")
    if digits != "123456":
        ok = False
    alpha: str = extract_alpha("abc123def")
    if alpha != "abcdef":
        ok = False
    return ok
