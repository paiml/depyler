"""Text processing: String transformation and encoding.

Tests: case conversion, ROT13, string reversal, character substitution,
padding, trimming operations.
"""

from typing import Dict, List, Tuple


def to_upper(s: str) -> str:
    """Convert string to uppercase."""
    result: List[str] = []
    for ch in s:
        if ch >= "a" and ch <= "z":
            code: int = ord(ch) - ord("a") + ord("A")
            result.append(chr(code))
        else:
            result.append(ch)
    return "".join(result)


def to_lower(s: str) -> str:
    """Convert string to lowercase."""
    result: List[str] = []
    for ch in s:
        if ch >= "A" and ch <= "Z":
            code: int = ord(ch) - ord("A") + ord("a")
            result.append(chr(code))
        else:
            result.append(ch)
    return "".join(result)


def rot13(s: str) -> str:
    """Apply ROT13 cipher."""
    result: List[str] = []
    for ch in s:
        if ch >= "a" and ch <= "z":
            code: int = (ord(ch) - ord("a") + 13) % 26 + ord("a")
            result.append(chr(code))
        elif ch >= "A" and ch <= "Z":
            code2: int = (ord(ch) - ord("A") + 13) % 26 + ord("A")
            result.append(chr(code2))
        else:
            result.append(ch)
    return "".join(result)


def reverse_string(s: str) -> str:
    """Reverse a string."""
    chars: List[str] = []
    for ch in s:
        chars.append(ch)
    chars.reverse()
    return "".join(chars)


def pad_left(s: str, width: int, pad_ch: str) -> str:
    """Pad string on the left to given width."""
    result: List[str] = []
    padding: int = width - len(s)
    i: int = 0
    while i < padding:
        result.append(pad_ch)
        i += 1
    for ch in s:
        result.append(ch)
    return "".join(result)


def pad_right(s: str, width: int, pad_ch: str) -> str:
    """Pad string on the right to given width."""
    result: List[str] = []
    for ch in s:
        result.append(ch)
    padding: int = width - len(s)
    i: int = 0
    while i < padding:
        result.append(pad_ch)
        i += 1
    return "".join(result)


def strip_spaces(s: str) -> str:
    """Remove leading and trailing spaces."""
    start: int = 0
    while start < len(s) and s[start] == " ":
        start += 1
    end: int = len(s)
    while end > start and s[end - 1] == " ":
        end -= 1
    result: List[str] = []
    i: int = start
    while i < end:
        result.append(s[i])
        i += 1
    return "".join(result)


def camel_to_snake(s: str) -> str:
    """Convert camelCase to snake_case."""
    result: List[str] = []
    for ch in s:
        if ch >= "A" and ch <= "Z":
            if len(result) > 0:
                result.append("_")
            code: int = ord(ch) - ord("A") + ord("a")
            result.append(chr(code))
        else:
            result.append(ch)
    return "".join(result)


def test_transforms() -> bool:
    """Test string transformations."""
    ok: bool = True
    if to_upper("hello") != "HELLO":
        ok = False
    if to_lower("HELLO") != "hello":
        ok = False
    if rot13(rot13("hello")) != "hello":
        ok = False
    if reverse_string("abc") != "cba":
        ok = False
    if strip_spaces("  hi  ") != "hi":
        ok = False
    if camel_to_snake("camelCase") != "camel_case":
        ok = False
    return ok
